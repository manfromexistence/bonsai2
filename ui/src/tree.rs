//! Flat, parent-vector UI tree.
//!
//! # Why two representations
//!
//! The GPU wants a **contiguous** array in Euler-tour order so that layout can
//! be solved with parallel prefix scans over a parent vector. The CPU wants
//! **O(1)** structural mutation, because a UI tree changes shape constantly
//! (a dropdown opens, a virtualised row scrolls out).
//!
//! Reconciling those two wants is the whole trick:
//!
//! 1. [`NodeTree`] is an arena with a free list. Inserting a child never moves
//!    another node's bytes, so mutation costs nothing and never dirties the
//!    bus. This is the *same* property a Packed Memory Array gives you, but
//!    without the memmove, and it is exact rather than amortised.
//! 2. [`EulerSnapshot`] is a flattened Euler tour produced **only** when the
//!    structure changed. It is padded with one identity gap slot every
//!    [`GAP_PERIOD`] elements so the GPU scan runs at full memory bandwidth
//!    without branching on holes.
//!
//! The cost model is therefore: *mutation* is O(1) and touches no GPU memory;
//! *a structural change* costs one O(n) re-flatten on the CPU plus one
//! upload. Everything else (animations, hover, scroll) never re-flattens.

use crate::layout::LayoutProps;
use crate::text::TextRun;

/// Handle to a node. `NONE` means "no node".
pub type NodeId = u32;

/// Sentinel for "no node".
pub const NONE: NodeId = u32::MAX;

/// One identity gap slot is inserted after every `GAP_PERIOD` logical nodes.
///
/// 32 is one warp on NVIDIA/AMD and one AVX-512 vector on x86, so the gap
/// period lines up with the widest hardware SIMD unit we might use.
pub const GAP_PERIOD: usize = 32;

/// A single node in the tree.
#[derive(Clone, Debug)]
pub struct Node {
    /// Parent node, or [`NONE`] for the root.
    pub parent: NodeId,
    /// First child in paint order.
    pub first_child: NodeId,
    /// Last child in paint order.
    pub last_child: NodeId,
    /// Previous sibling, or [`NONE`].
    pub prev_sibling: NodeId,
    /// Next sibling, or [`NONE`].
    pub next_sibling: NodeId,
    /// Depth from the root (root is 0).
    pub depth: u16,
    /// Number of nodes in this subtree, including this node.
    pub subtree_size: u32,
    /// Flex layout properties.
    pub props: LayoutProps,
    /// Optional text content.
    pub text: Option<TextRun>,
    /// Geometry, written by [`crate::layout::layout`].
    pub x: f32,
    /// Geometry, written by [`crate::layout::layout`].
    pub y: f32,
    /// Geometry, written by [`crate::layout::layout`].
    pub w: f32,
    /// Geometry, written by [`crate::layout::layout`].
    pub h: f32,
    /// Whether this slot is currently in use.
    pub alive: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: NONE,
            first_child: NONE,
            last_child: NONE,
            prev_sibling: NONE,
            next_sibling: NONE,
            depth: 0,
            subtree_size: 1,
            props: LayoutProps::default(),
            text: None,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            alive: true,
        }
    }
}

/// A UI tree: an arena of nodes with parent/sibling links.
///
/// The tree is *not* a DOM and does not pretend to be one. There is no
/// `insertBefore` on strings, no `querySelector`, no mutation events. It is a
/// layout tree: nodes have geometry, properties and optional text.
#[derive(Clone, Debug)]
pub struct NodeTree {
    nodes: Vec<Node>,
    free: Vec<NodeId>,
    root: NodeId,
    structure_dirty: bool,
}

impl Default for NodeTree {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeTree {
    /// Creates a tree with a single root node.
    #[must_use]
    pub fn new() -> Self {
        let nodes = vec![Node::default()];
        Self {
            nodes,
            free: Vec::new(),
            root: 0,
            structure_dirty: true,
        }
    }

    /// The root node id.
    #[must_use]
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Number of live nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len() - self.free.len()
    }

    /// Whether the tree contains no live nodes beyond the root.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() <= 1
    }

    /// Whether a structural change happened since the last snapshot.
    #[must_use]
    pub fn structure_dirty(&self) -> bool {
        self.structure_dirty
    }

    /// Borrows a live node.
    #[must_use]
    pub fn node(&self, id: NodeId) -> Option<&Node> {
        let n = self.nodes.get(id as usize)?;
        n.alive.then_some(n)
    }

    /// Mutably borrows a live node.
    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        let n = self.nodes.get_mut(id as usize)?;
        n.alive.then_some(n)
    }

    /// Allocates a detached node with the given properties.
    pub fn alloc(&mut self, props: LayoutProps) -> NodeId {
        let node = Node {
            props,
            ..Node::default()
        };
        if let Some(id) = self.free.pop() {
            self.nodes[id as usize] = node;
            id
        } else {
            self.nodes.push(node);
            (self.nodes.len() - 1) as NodeId
        }
    }

    /// Detaches a node from its parent without freeing it.
    pub fn detach(&mut self, id: NodeId) {
        let Some(node) = self.nodes.get(id as usize) else {
            return;
        };
        let (parent, prev, next) = (node.parent, node.prev_sibling, node.next_sibling);
        if parent == NONE {
            return;
        }
        if prev != NONE {
            if let Some(p) = self.nodes.get_mut(prev as usize) {
                p.next_sibling = next;
            }
        } else if let Some(p) = self.nodes.get_mut(parent as usize) {
            p.first_child = next;
        }
        if next != NONE {
            if let Some(n) = self.nodes.get_mut(next as usize) {
                n.prev_sibling = prev;
            }
        } else if let Some(p) = self.nodes.get_mut(parent as usize) {
            p.last_child = prev;
        }
        self.bump_subtree(parent, -1);
        if let Some(n) = self.nodes.get_mut(id as usize) {
            n.parent = NONE;
            n.prev_sibling = NONE;
            n.next_sibling = NONE;
        }
        self.structure_dirty = true;
    }

    /// Inserts `child` into `parent`'s child list.
    ///
    /// `index` is the position among the parent's existing children; `None`
    /// appends. Returns `false` when the operation would be invalid (unknown
    /// node, already attached child, or a cycle).
    pub fn insert_child(&mut self, parent: NodeId, child: NodeId, index: Option<usize>) -> bool {
        if self.node(parent).is_none() {
            return false;
        }
        let Some(c) = self.node(child) else {
            return false;
        };
        if c.parent != NONE {
            return false;
        }
        // Cycle guard: `child` must not be an ancestor of `parent`.
        if self.is_ancestor(child, parent) {
            return false;
        }
        let sibs = self.sibling_list(parent);
        let pos = index.unwrap_or(sibs.len()).min(sibs.len());
        let prev = if pos == 0 { NONE } else { sibs[pos - 1] };
        let next = sibs.get(pos).copied().unwrap_or(NONE);

        if prev != NONE {
            self.nodes[prev as usize].next_sibling = child;
        } else {
            self.nodes[parent as usize].first_child = child;
        }
        if next != NONE {
            self.nodes[next as usize].prev_sibling = child;
        } else {
            self.nodes[parent as usize].last_child = child;
        }
        let depth = self.nodes[parent as usize].depth;
        let subtree = self.nodes[parent as usize].subtree_size;
        let n = &mut self.nodes[child as usize];
        n.parent = parent;
        n.prev_sibling = prev;
        n.next_sibling = next;
        n.depth = depth + 1;
        n.subtree_size = 1;
        let _ = subtree;
        self.bump_subtree(parent, 1);
        self.structure_dirty = true;
        true
    }

    /// Removes a node and its whole subtree, returning the slots to the free
    /// list for reuse.
    pub fn remove(&mut self, id: NodeId) {
        if id == NONE || id == self.root {
            return;
        }
        let Some(node) = self.node(id) else {
            return;
        };
        let subtree = node.subtree_size as usize;
        self.detach(id);
        // Collect the subtree ids (pre-order) and free them.
        let mut stack = vec![id];
        let mut freed = 0usize;
        while let Some(cur) = stack.pop() {
            let kids = self.sibling_list(cur);
            stack.extend(kids);
            if let Some(n) = self.nodes.get_mut(cur as usize) {
                n.alive = false;
                n.text = None;
            }
            self.free.push(cur);
            freed += 1;
        }
        debug_assert_eq!(freed, subtree, "subtree_size bookkeeping drifted");
        self.structure_dirty = true;
    }

    /// Whether `ancestor` is `node` or one of its ancestors.
    #[must_use]
    pub fn is_ancestor(&self, ancestor: NodeId, node: NodeId) -> bool {
        let mut cur = node;
        let mut guard = 0usize;
        while cur != NONE && guard < self.nodes.len() + 1 {
            if cur == ancestor {
                return true;
            }
            guard += 1;
            cur = self.nodes[cur as usize].parent;
        }
        false
    }

    /// Children of `id` in paint order.
    #[must_use]
    pub fn sibling_list(&self, id: NodeId) -> Vec<NodeId> {
        let mut out = Vec::new();
        let mut cur = self.nodes.get(id as usize).map_or(NONE, |n| n.first_child);
        let mut guard = 0usize;
        while cur != NONE && guard <= self.nodes.len() {
            out.push(cur);
            cur = self.nodes[cur as usize].next_sibling;
            guard += 1;
        }
        out
    }

    /// Pre-order (paint-order) traversal of every live node.
    #[must_use]
    pub fn dfs_order(&self) -> Vec<NodeId> {
        let mut out = Vec::with_capacity(self.len());
        let mut stack = vec![self.root];
        while let Some(cur) = stack.pop() {
            if cur == NONE {
                continue;
            }
            out.push(cur);
            let kids = self.sibling_list(cur);
            for k in kids.into_iter().rev() {
                stack.push(k);
            }
        }
        out
    }

    fn bump_subtree(&mut self, mut id: NodeId, delta: i64) {
        let mut guard = 0usize;
        while id != NONE && guard <= self.nodes.len() {
            guard += 1;
            let Some(n) = self.nodes.get_mut(id as usize) else {
                return;
            };
            n.subtree_size = (n.subtree_size as i64 + delta).max(0) as u32;
            id = n.parent;
        }
    }

    /// Produces a gap-padded Euler-tour snapshot for the GPU.
    ///
    /// The snapshot is the only thing that ever crosses the bus. It is
    /// rebuilt only when the structure changed; animations and hover states
    /// mutate the geometry arrays in place.
    #[must_use]
    pub fn snapshot(&self) -> EulerSnapshot {
        let order = self.dfs_order();
        let n = order.len();

        // Logical index of each node in the pre-order traversal.
        let mut index_of = vec![NONE; self.nodes.len()];
        for (i, &id) in order.iter().enumerate() {
            index_of[id as usize] = i as u32;
        }

        // One gap slot after every GAP_PERIOD logical elements.
        let slot_count = n + n / GAP_PERIOD;
        let mut parent = vec![NONE; slot_count];
        let mut depth = vec![0u32; slot_count];
        let mut x = vec![0f32; slot_count];
        let mut y = vec![0f32; slot_count];
        let mut w = vec![0f32; slot_count];
        let mut h = vec![0f32; slot_count];
        let mut is_gap = vec![true; slot_count];

        for (i, &id) in order.iter().enumerate() {
            let slot = i + i / GAP_PERIOD;
            let node = &self.nodes[id as usize];
            parent[slot] = if node.parent == NONE {
                NONE
            } else {
                index_of[node.parent as usize]
            };
            depth[slot] = u32::from(node.depth);
            x[slot] = node.x;
            y[slot] = node.y;
            w[slot] = node.w;
            h[slot] = node.h;
            is_gap[slot] = false;
        }

        EulerSnapshot {
            node_count: n,
            slot_count,
            order,
            parent,
            depth,
            x,
            y,
            w,
            h,
            is_gap,
        }
    }
}

/// A flattened, gap-padded Euler tour of a [`NodeTree`].
///
/// The arrays are structure-of-arrays so that they can be uploaded verbatim
/// to a GPU storage buffer and consumed by a compute shader without a
/// repack on the CPU. Gap slots carry the identity element of the layout
/// semiring (`parent = NONE`, zero size) so the prefix scan glides over them.
#[derive(Clone, Debug)]
pub struct EulerSnapshot {
    /// Number of real nodes.
    pub node_count: usize,
    /// Number of slots including gaps.
    pub slot_count: usize,
    /// Pre-order node ids, one per real node.
    pub order: Vec<NodeId>,
    /// Parent index (logical, into `order`) per slot.
    pub parent: Vec<u32>,
    /// Depth per slot.
    pub depth: Vec<u32>,
    /// `x` per slot.
    pub x: Vec<f32>,
    /// `y` per slot.
    pub y: Vec<f32>,
    /// `w` per slot.
    pub w: Vec<f32>,
    /// `h` per slot.
    pub h: Vec<f32>,
    /// Whether a slot is a gap.
    pub is_gap: Vec<bool>,
}

impl EulerSnapshot {
    /// Physical slot index of logical node `i`.
    #[must_use]
    pub fn slot_of(&self, logical: usize) -> usize {
        logical + logical / GAP_PERIOD
    }
}
