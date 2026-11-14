//! GPU-facing buffer layouts and the WGSL that consumes them.
//!
//! The Rust structs here are the *contract* between the CPU layout solver and
//! the GPU compute passes. They are `repr(C)` with static size assertions, so a
//! layout change breaks the build rather than producing a silently corrupt
//! buffer.
//!
//! # Status
//!
//! **repo-managed.** The WGSL in `shaders/` is source text. It has *not* been
//! compiled by `naga`, `tint` or a driver in this repository — there is no
//! GPU dependency here on purpose. Treat the shaders as a reviewed design
//! artefact, not as a shipped renderer. TODO.md tracks getting them compiled.

use crate::tree::GAP_PERIOD;

/// One node, as the GPU sees it.
///
/// Layout matches the `GpuNode` struct in `shaders/common.wgsl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuNode {
    /// Index of the parent in the Euler tour, or `u32::MAX` for the root.
    pub parent: u32,
    /// Depth from the root.
    pub depth: u32,
    /// Bit flags (e.g. `display:none`, `is_text`).
    pub flags: u32,
    /// Reserved, keeps the struct 16-byte aligned.
    pub reserved: u32,
    /// `x` in pixels.
    pub x: f32,
    /// `y` in pixels.
    pub y: f32,
    /// Width in pixels.
    pub w: f32,
    /// Height in pixels.
    pub h: f32,
}

const _: () = assert!(core::mem::size_of::<GpuNode>() == 32);
const _: () = assert!(core::mem::align_of::<GpuNode>() == 4);

/// Uniform block for the layout pass.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuLayoutParams {
    /// Number of real nodes.
    pub node_count: u32,
    /// Number of slots including identity gaps.
    pub slot_count: u32,
    /// Viewport width.
    pub viewport_w: f32,
    /// Viewport height.
    pub viewport_h: f32,
    /// Current time in seconds, for analytic animation.
    pub time: f32,
    /// Gap period used when the snapshot was built.
    pub gap_period: u32,
    /// Padding to a 16-byte multiple.
    pub _pad: [f32; 2],
}

const _: () = assert!(core::mem::size_of::<GpuLayoutParams>() == 32);

/// A tile descriptor for the rasterization pass.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuTile {
    /// Tile column.
    pub tile_x: u32,
    /// Tile row.
    pub tile_y: u32,
    /// Number of primitives in this tile.
    pub primitive_count: u32,
    /// Offset of the first primitive in the tile's index list.
    pub primitive_offset: u32,
}

const _: () = assert!(core::mem::size_of::<GpuTile>() == 16);

/// The gap period the shaders assume. Kept in sync with [`crate::tree::GAP_PERIOD`].
pub const GPU_GAP_PERIOD: usize = GAP_PERIOD;

/// WGSL source for the layout pass.
pub const WGSL_LAYOUT: &str = include_str!("../shaders/layout.wgsl");
/// WGSL source for the tile rasterizer.
pub const WGSL_TILE_BLEND: &str = include_str!("../shaders/tile_blend.wgsl");
/// WGSL source for the glyph evaluator.
pub const WGSL_GLYPH: &str = include_str!("../shaders/glyph.wgsl");
/// WGSL source for shared structures.
pub const WGSL_COMMON: &str = include_str!("../shaders/common.wgsl");

/// Every shader, with a stable name.
pub const SHADERS: &[(&str, &str)] = &[
    ("common", WGSL_COMMON),
    ("layout", WGSL_LAYOUT),
    ("tile_blend", WGSL_TILE_BLEND),
    ("glyph", WGSL_GLYPH),
];
