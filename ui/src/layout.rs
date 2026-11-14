//! Flex layout with an **analytic** water-filling distribution.
//!
//! # The claim under test
//!
//! "Flexbox `flex-grow` / `flex-shrink` with `min-*` / `max-*` clamps requires a
//! sequential *freeze-and-redistribute* loop, which is impossible to express
//! without warp divergence on a GPU."
//!
//! That is true of the CSS reference implementation, but it is a property of the
//! *algorithm*, not of the problem. The CSS loop is solving a **water-filling
//! (simplex projection)** problem: find the level `λ` such that
//!
//! ```text
//! sum_i clamp(base_i + λ * factor_i, min_i, max_i) == available
//! ```
//!
//! which is continuous and piecewise-linear in `λ`, hence solvable exactly by
//! sorting the breakpoints (the `λ` values at which an item saturates at its
//! `min` or `max`) and doing one sweep. That is [`solve_grow`] and
//! [`solve_shrink`] here, and [`solve_grow_reference`] /
//! [`solve_shrink_reference`] implement the CSS freeze loop so the two can be
//! differentially tested against each other (see `tests/flex_solver.rs`).
//!
//! # What is actually modelled
//!
//! A deliberately small subset of CSS Flexbox, chosen so that every claim
//! about the algorithm can be tested:
//!
//! * `flex-direction: row | column`
//! * `flex-grow`, `flex-shrink`, `flex-basis`
//! * `min-width` / `min-height`, `max-width` / `max-height`
//! * `width` / `height` overrides
//! * `padding`, `gap`
//! * `justify-content`: start / end / center / space-between / space-around
//! * `align-items`: stretch / start / center / end
//! * text wrapping against the final main-axis size
//!
//! Grid, `float`, `position`, writing modes, and margin collapsing are **not**
//! implemented. Do not claim they are.

use crate::tree::{NodeId, NodeTree};

/// Positive infinity, used to mean "unconstrained" on one axis.
pub const INFINITY: f32 = f32::INFINITY;

/// A width/height pair.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Size {
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

impl Size {
    /// The zero size.
    pub const ZERO: Size = Size { w: 0.0, h: 0.0 };
    /// Constructs a size.
    #[must_use]
    pub const fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
    /// The extent along the main axis for a given direction.
    #[must_use]
    pub fn main(self, dir: FlexDirection) -> f32 {
        match dir {
            FlexDirection::Row => self.w,
            FlexDirection::Column => self.h,
        }
    }
    /// The extent along the cross axis for a given direction.
    #[must_use]
    pub fn cross(self, dir: FlexDirection) -> f32 {
        match dir {
            FlexDirection::Row => self.h,
            FlexDirection::Column => self.w,
        }
    }
}

/// An axis-aligned rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

impl Rect {
    /// Constructs a rectangle.
    #[must_use]
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
    /// The extent along the main axis for a given direction.
    #[must_use]
    pub fn main(self, dir: FlexDirection) -> f32 {
        match dir {
            FlexDirection::Row => self.w,
            FlexDirection::Column => self.h,
        }
    }
    /// The extent along the cross axis for a given direction.
    #[must_use]
    pub fn cross(self, dir: FlexDirection) -> f32 {
        match dir {
            FlexDirection::Row => self.h,
            FlexDirection::Column => self.w,
        }
    }
}

/// Padding / inset on all four edges.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct EdgeInsets {
    /// Left inset.
    pub left: f32,
    /// Top inset.
    pub top: f32,
    /// Right inset.
    pub right: f32,
    /// Bottom inset.
    pub bottom: f32,
}

impl EdgeInsets {
    /// Uniform inset.
    #[must_use]
    pub const fn all(v: f32) -> Self {
        Self {
            left: v,
            top: v,
            right: v,
            bottom: v,
        }
    }
    /// Sum of the horizontal insets.
    #[must_use]
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }
    /// Sum of the vertical insets.
    #[must_use]
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

/// Main axis direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexDirection {
    /// Horizontal main axis.
    #[default]
    Row,
    /// Vertical main axis.
    Column,
}

/// Main-axis distribution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Justify {
    /// Pack at the start.
    #[default]
    Start,
    /// Pack at the end.
    End,
    /// Centre the group.
    Center,
    /// Put the free space between items.
    SpaceBetween,
    /// Put half the free space before, half between, half after.
    SpaceAround,
}

/// Cross-axis alignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    /// Stretch items to fill the cross axis.
    #[default]
    Stretch,
    /// Align to the cross start.
    Start,
    /// Centre on the cross axis.
    Center,
    /// Align to the cross end.
    End,
}

/// Per-node layout properties.
#[derive(Clone, Debug)]
pub struct LayoutProps {
    /// Skip this node entirely.
    pub display_none: bool,
    /// Flex grow factor.
    pub grow: f32,
    /// Flex shrink factor.
    pub shrink: f32,
    /// Flex basis; `None` means "measure the content".
    pub basis: Option<f32>,
    /// Minimum width.
    pub min_w: f32,
    /// Minimum height.
    pub min_h: f32,
    /// Maximum width; `None` means unbounded.
    pub max_w: Option<f32>,
    /// Maximum height; `None` means unbounded.
    pub max_h: Option<f32>,
    /// Explicit width.
    pub width: Option<f32>,
    /// Explicit height.
    pub height: Option<f32>,
    /// Padding.
    pub padding: EdgeInsets,
    /// Gap between children on the main axis.
    pub gap: f32,
    /// Main axis direction.
    pub direction: FlexDirection,
    /// Main-axis distribution.
    pub justify: Justify,
    /// Cross-axis alignment.
    pub align: Align,
    /// Line height multiplier for text.
    pub line_height: f32,
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            display_none: false,
            grow: 0.0,
            shrink: 1.0,
            basis: None,
            min_w: 0.0,
            min_h: 0.0,
            max_w: None,
            max_h: None,
            width: None,
            height: None,
            padding: EdgeInsets::default(),
            gap: 0.0,
            direction: FlexDirection::Row,
            justify: Justify::Start,
            align: Align::Stretch,
            line_height: 1.2,
        }
    }
}

/// Clamp to `[lo, hi]`, where a non-finite `hi` means "unbounded".
#[inline]
fn clamp_f(v: f32, lo: f32, hi: f32) -> f32 {
    if v < lo {
        lo
    } else if hi.is_finite() && v > hi {
        hi
    } else {
        v
    }
}

/// Analytic distribution of extra main-axis space across flex items.
///
/// Solves `sum_i clamp(base_i + λ·grow_i, min_i, max_i) == base_sum + free` for
/// `λ` by sorting the saturation breakpoints. One sort, one sweep, no
/// iteration over redistribution rounds.
///
/// `base` must already be clamped into `[min_i, max_i]` (the CSS "hypothetical
/// main size").
///
/// Returns the new main sizes. If the items cannot absorb all of `free` (every
/// item hit its `max`), the leftover stays unallocated for `justify-content` to
/// place.
#[must_use]
pub fn solve_grow(
    base: &[f32],
    grow: &[f32],
    min: &[f32],
    max: &[f32],
    free: f32,
) -> Vec<f32> {
    let n = base.len();
    // Hypothetical main size: the base clamped into [min, max].
    let bases: Vec<f32> = (0..n).map(|i| clamp_f(base[i], min[i], max[i])).collect();
    let mut out = bases.clone();
    if n == 0 || free <= 0.0 {
        return out;
    }
    let sum_grow: f32 = grow.iter().sum();
    if sum_grow <= 0.0 {
        return out;
    }

    // Saturation breakpoints: the λ at which item i reaches its maximum.
    //
    // An item that already sits at its maximum has τ = 0 and saturates
    // immediately. Freezing it removes its (useless) grow factor from the
    // pool — without this the level λ is computed against grow factors that
    // can never absorb anything, and the free space is not conserved.
    let mut thresholds: Vec<(f32, usize)> = Vec::new();
    for i in 0..n {
        if grow[i] > 0.0 && max[i].is_finite() {
            let tau = (max[i] - bases[i]) / grow[i];
            if tau >= 0.0 {
                thresholds.push((tau, i));
            }
        }
    }
    thresholds.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut frozen = vec![false; n];
    let mut cum_extra = 0.0f32;
    let mut cum_grow = 0.0f32;
    let mut k = 0usize;
    while k < thresholds.len() {
        let denom = sum_grow - cum_grow;
        if denom <= 0.0 {
            break;
        }
        let lambda = (free - cum_extra) / denom;
        let (tau, i) = thresholds[k];
        if tau < lambda {
            frozen[i] = true;
            cum_extra += (max[i] - bases[i]).max(0.0);
            cum_grow += grow[i];
            k += 1;
        } else {
            break;
        }
    }
    let denom = sum_grow - cum_grow;
    let lambda = if denom > 0.0 {
        (free - cum_extra) / denom
    } else {
        0.0
    };

    for i in 0..n {
        out[i] = if frozen[i] {
            max[i]
        } else {
            clamp_f(bases[i] + lambda * grow[i], min[i], max[i])
        };
    }
    out
}

/// Analytic distribution of a main-axis deficit across flex items.
///
/// `scaled_shrink` is the CSS *scaled flex shrink factor*
/// (`flex-shrink × inner flex base size`).
#[must_use]
pub fn solve_shrink(
    base: &[f32],
    scaled_shrink: &[f32],
    min: &[f32],
    max: &[f32],
    deficit: f32,
) -> Vec<f32> {
    let n = base.len();
    let bases: Vec<f32> = (0..n).map(|i| clamp_f(base[i], min[i], max[i])).collect();
    let mut out = bases.clone();
    if n == 0 || deficit <= 0.0 {
        return out;
    }
    let sum_s: f32 = scaled_shrink.iter().sum();
    if sum_s <= 0.0 {
        return out;
    }

    // Saturation breakpoints: the λ at which item i reaches its minimum.
    // An item already at its minimum has σ = 0 and saturates immediately,
    // which removes its (useless) shrink factor from the pool.
    let mut thresholds: Vec<(f32, usize)> = Vec::new();
    for i in 0..n {
        if scaled_shrink[i] > 0.0 && bases[i] >= min[i] {
            let sigma = (bases[i] - min[i]) / scaled_shrink[i];
            if sigma >= 0.0 {
                thresholds.push((sigma, i));
            }
        }
    }
    thresholds.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut frozen = vec![false; n];
    let mut cum_reduction = 0.0f32;
    let mut cum_s = 0.0f32;
    let mut k = 0usize;
    while k < thresholds.len() {
        let denom = sum_s - cum_s;
        if denom <= 0.0 {
            break;
        }
        let lambda = (deficit - cum_reduction) / denom;
        let (sigma, i) = thresholds[k];
        if sigma < lambda {
            frozen[i] = true;
            cum_reduction += (bases[i] - min[i]).max(0.0);
            cum_s += scaled_shrink[i];
            k += 1;
        } else {
            break;
        }
    }
    let denom = sum_s - cum_s;
    let lambda = if denom > 0.0 {
        (deficit - cum_reduction) / denom
    } else {
        0.0
    };

    for i in 0..n {
        out[i] = if frozen[i] {
            min[i]
        } else {
            clamp_f(bases[i] - lambda * scaled_shrink[i], min[i], max[i])
        };
    }
    out
}

/// CSS reference algorithm: iterative freeze-and-redistribute for growth.
///
/// Kept only as an oracle for the differential test.
#[must_use]
pub fn solve_grow_reference(
    base: &[f32],
    grow: &[f32],
    min: &[f32],
    max: &[f32],
    free: f32,
) -> Vec<f32> {
    let n = base.len();
    let mut size: Vec<f32> = (0..n).map(|i| clamp_f(base[i], min[i], max[i])).collect();
    if free <= 0.0 {
        // A negative free space is a deficit, handled by `solve_shrink`.
        return size;
    }
    let mut frozen = vec![false; n];
    let mut remaining = free;
    loop {
        let total: f32 = (0..n).filter(|&i| !frozen[i]).map(|i| grow[i]).sum();
        if total <= 0.0 {
            break;
        }
        let targets: Vec<f32> = (0..n)
            .map(|i| {
                if frozen[i] {
                    size[i]
                } else {
                    base[i] + remaining * grow[i] / total
                }
            })
            .collect();
        let mut violated = false;
        for i in 0..n {
            if frozen[i] {
                continue;
            }
            if targets[i] > max[i] {
                frozen[i] = true;
                size[i] = max[i];
                remaining -= (size[i] - base[i]).max(0.0);
                violated = true;
            } else if targets[i] < min[i] {
                frozen[i] = true;
                size[i] = min[i];
                remaining -= (size[i] - base[i]).max(0.0);
                violated = true;
            } else {
                size[i] = targets[i];
            }
        }
        if !violated {
            break;
        }
    }
    size
}

/// CSS reference algorithm: iterative freeze-and-redistribute for shrinking.
#[must_use]
pub fn solve_shrink_reference(
    base: &[f32],
    scaled_shrink: &[f32],
    min: &[f32],
    max: &[f32],
    deficit: f32,
) -> Vec<f32> {
    let n = base.len();
    let mut size: Vec<f32> = (0..n).map(|i| clamp_f(base[i], min[i], max[i])).collect();
    let mut frozen = vec![false; n];
    let mut remaining = deficit;
    loop {
        let total: f32 = (0..n).filter(|&i| !frozen[i]).map(|i| scaled_shrink[i]).sum();
        if total <= 0.0 {
            break;
        }
        let targets: Vec<f32> = (0..n)
            .map(|i| {
                if frozen[i] {
                    size[i]
                } else {
                    base[i] - remaining * scaled_shrink[i] / total
                }
            })
            .collect();
        let mut violated = false;
        for i in 0..n {
            if frozen[i] {
                continue;
            }
            if targets[i] < min[i] {
                frozen[i] = true;
                size[i] = min[i];
                remaining -= (base[i] - size[i]).max(0.0);
                violated = true;
            } else if targets[i] > max[i] {
                frozen[i] = true;
                size[i] = max[i];
                remaining -= (base[i] - size[i]).max(0.0);
                violated = true;
            } else {
                size[i] = targets[i];
            }
        }
        if !violated {
            break;
        }
    }
    size
}

/// Statistics from one layout pass.
#[derive(Clone, Copy, Debug, Default)]
pub struct LayoutStats {
    /// Nodes visited.
    pub nodes: usize,
    /// Number of flex containers whose distribution was solved analytically.
    pub solved_containers: usize,
}

/// Lays out the whole tree inside `viewport`.
///
/// The root node is sized to the viewport. Returns statistics.
pub fn layout(tree: &mut NodeTree, viewport: Size) -> LayoutStats {
    let root = tree.root();
    let mut stats = LayoutStats::default();
    arrange(tree, root, Rect::new(0.0, 0.0, viewport.w, viewport.h), &mut stats, 0);
    stats.nodes = tree.len();
    stats
}

/// Measures the border-box size a node wants, given an available border-box.
///
/// Returns the **border-box** size (content + padding).
fn measure(tree: &NodeTree, id: NodeId, avail: Size, depth: usize) -> Size {
    if depth > 512 {
        return Size::ZERO;
    }
    let Some(node) = tree.node(id) else {
        return Size::ZERO;
    };
    let props = node.props.clone();
    if props.display_none {
        return Size::ZERO;
    }
    let pad = props.padding;
    let inner_w = avail.w - pad.horizontal();
    let inner_h = avail.h - pad.vertical();

    let children = tree.sibling_list(id);
    if children.is_empty() {
        let content = if let Some(text) = &node.text {
            let w = if inner_w.is_finite() {
                inner_w.max(0.0)
            } else {
                text.max_content_width()
            };
            let lines = if inner_w.is_finite() {
                text.line_ranges(inner_w.max(0.0)).len().max(1)
            } else {
                1
            };
            Size::new(w, lines as f32 * text.line_height)
        } else {
            Size::ZERO
        };
        return Size::new(content.w + pad.horizontal(), content.h + pad.vertical());
    }

    let content = match props.direction {
        FlexDirection::Row => {
            let mut main = 0.0f32;
            let mut cross = 0.0f32;
            for (i, &ch) in children.iter().enumerate() {
                let cp = tree.node(ch).map(|n| n.props.clone()).unwrap_or_default();
                let m = measure(tree, ch, Size::new(INFINITY, inner_h), depth + 1);
                main += cp.basis.unwrap_or(m.w);
                cross = cross.max(m.h);
                if i + 1 < children.len() {
                    main += props.gap;
                }
            }
            Size::new(main, cross)
        }
        FlexDirection::Column => {
            let mut main = 0.0f32;
            let mut cross = 0.0f32;
            for (i, &ch) in children.iter().enumerate() {
                let cp = tree.node(ch).map(|n| n.props.clone()).unwrap_or_default();
                let m = measure(tree, ch, Size::new(inner_w, INFINITY), depth + 1);
                main += cp.basis.unwrap_or(m.h);
                cross = cross.max(m.w);
                if i + 1 < children.len() {
                    main += props.gap;
                }
            }
            Size::new(cross, main)
        }
    };
    Size::new(content.w + pad.horizontal(), content.h + pad.vertical())
}

#[allow(clippy::too_many_arguments)]
fn arrange(
    tree: &mut NodeTree,
    id: NodeId,
    rect: Rect,
    stats: &mut LayoutStats,
    depth: usize,
) {
    if depth > 512 {
        return;
    }
    let Some(node) = tree.node(id) else {
        return;
    };
    let props = node.props.clone();
    if props.display_none {
        if let Some(n) = tree.node_mut(id) {
            n.x = rect.x;
            n.y = rect.y;
            n.w = 0.0;
            n.h = 0.0;
        }
        return;
    }
    if let Some(n) = tree.node_mut(id) {
        n.x = rect.x;
        n.y = rect.y;
        n.w = rect.w;
        n.h = rect.h;
    }
    let pad = props.padding;
    let inner = Rect::new(
        rect.x + pad.left,
        rect.y + pad.top,
        (rect.w - pad.horizontal()).max(0.0),
        (rect.h - pad.vertical()).max(0.0),
    );
    let children = tree.sibling_list(id);
    if children.is_empty() {
        if let Some(text) = tree.node(id).and_then(|n| n.text.clone()) {
            let lines = text.line_ranges(inner.w).len().max(1);
            let h = lines as f32 * text.line_height + pad.vertical();
            if let Some(n) = tree.node_mut(id) {
                n.h = h;
            }
        }
        return;
    }

    let n = children.len();
    let dir = props.direction;
    let mut base = Vec::with_capacity(n);
    let mut cross = Vec::with_capacity(n);
    let mut grow = Vec::with_capacity(n);
    let mut shrink = Vec::with_capacity(n);
    let mut mins = Vec::with_capacity(n);
    let mut maxs = Vec::with_capacity(n);

    for &ch in &children {
        let cp = tree.node(ch).map(|x| x.props.clone()).unwrap_or_default();
        let m = measure(
            tree,
            ch,
            match dir {
                FlexDirection::Row => Size::new(INFINITY, inner.h),
                FlexDirection::Column => Size::new(inner.w, INFINITY),
            },
            depth + 1,
        );
        let definite = match dir {
            FlexDirection::Row => cp.width,
            FlexDirection::Column => cp.height,
        };
        let raw = definite.or(cp.basis).unwrap_or_else(|| m.main(dir));
        let (lo, hi) = match dir {
            FlexDirection::Row => (cp.min_w, cp.max_w),
            FlexDirection::Column => (cp.min_h, cp.max_h),
        };
        let b = clamp_f(raw, lo, hi.unwrap_or(f32::INFINITY));
        base.push(b);
        cross.push(m.cross(dir));
        grow.push(if definite.is_some() { 0.0 } else { cp.grow.max(0.0) });
        shrink.push(if definite.is_some() {
            0.0
        } else {
            cp.shrink.max(0.0)
        });
        mins.push(lo);
        maxs.push(hi.unwrap_or(f32::INFINITY));
    }

    let gap_total = props.gap * (n as f32 - 1.0).max(0.0);
    let container_main = inner.main(dir);
    let total_base: f32 = base.iter().sum();
    let free = container_main - total_base - gap_total;

    let sizes: Vec<f32> = if free > 0.0 && grow.iter().any(|&g| g > 0.0) {
        stats.solved_containers += 1;
        solve_grow(&base, &grow, &mins, &maxs, free)
    } else if free < 0.0 && shrink.iter().any(|&s| s > 0.0) {
        stats.solved_containers += 1;
        let scaled: Vec<f32> = shrink.iter().zip(&base).map(|(&s, &b)| s * b).collect();
        solve_shrink(&base, &scaled, &mins, &maxs, -free)
    } else {
        base.clone()
    };

    let used: f32 = sizes.iter().sum();
    let leftover = (container_main - used - gap_total).max(0.0);
    let (offset, extra_gap) = match props.justify {
        Justify::Start => (0.0, 0.0),
        Justify::End => (leftover, 0.0),
        Justify::Center => (leftover * 0.5, 0.0),
        Justify::SpaceBetween => {
            if n > 1 {
                (0.0, leftover / (n as f32 - 1.0))
            } else {
                (0.0, 0.0)
            }
        }
        Justify::SpaceAround => {
            let per = leftover / n as f32;
            (per * 0.5, per)
        }
    };

    let cross_container = inner.cross(dir);
    let main_origin = match dir {
        FlexDirection::Row => inner.x,
        FlexDirection::Column => inner.y,
    };
    let mut cursor = main_origin + offset;
    for (i, &ch) in children.iter().enumerate() {
        let cp = tree.node(ch).map(|x| x.props.clone()).unwrap_or_default();
        let mut main_size = sizes[i];
        let cross_size;
        let mut cross_off = 0.0f32;

        match dir {
            FlexDirection::Row => {
                if let Some(w) = cp.width {
                    main_size = w;
                }
                match props.align {
                    Align::Stretch => cross_size = cp.height.unwrap_or(cross_container),
                    Align::Start => cross_size = cp.height.unwrap_or(cross[i]),
                    Align::Center => {
                        cross_size = cp.height.unwrap_or(cross[i]);
                        cross_off = (cross_container - cross_size) * 0.5;
                    }
                    Align::End => {
                        cross_size = cp.height.unwrap_or(cross[i]);
                        cross_off = cross_container - cross_size;
                    }
                }
            }
            FlexDirection::Column => {
                if let Some(h) = cp.height {
                    main_size = h;
                }
                match props.align {
                    Align::Stretch => cross_size = cp.width.unwrap_or(cross_container),
                    Align::Start | Align::Center | Align::End => {
                        cross_size = cp.width.unwrap_or(cross[i]);
                        match props.align {
                            Align::Center => cross_off = (cross_container - cross_size) * 0.5,
                            Align::End => cross_off = cross_container - cross_size,
                            _ => {}
                        }
                    }
                }
            }
        }
        cross_off = cross_off.max(0.0);

        let child_rect = match dir {
            FlexDirection::Row => Rect::new(cursor, inner.y + cross_off, main_size, cross_size),
            FlexDirection::Column => Rect::new(inner.x + cross_off, cursor, cross_size, main_size),
        };
        arrange(tree, ch, child_rect, stats, depth + 1);
        cursor += main_size + props.gap + extra_gap;
    }
}
