//! `ui` — a hybrid CPU/GPU UI rendering core.
//!
//! This crate is the *algorithmic* half of a proposed GPU-resident UI engine.
//! It implements, in portable Rust with **zero external dependencies**, the
//! pieces that the design conversations kept claiming were impossible or
//! world-first:
//!
//! * [`tree`] — a flat, parent-vector UI tree with O(1) structural mutation and
//!   a gap-padded Euler-tour snapshot that a GPU prefix scan can walk without
//!   branching.
//! * [`layout`] — flex layout solved with an **analytic water-filling**
//!   distribution (sort-and-sweep, no iteration, no warp divergence) plus
//!   cumulative-advance text wrapping via binary search.
//! * [`text`] — atlas-free glyph coverage: TrueType-style quadratic outlines
//!   flattened into horizontal **bands**, evaluated by scanline with exact
//!   horizontal coverage (no CPU glyph bitmap atlas required).
//! * [`raster`] — SDF primitives with a `u128` span-occlusion culler that
//!   removes fully hidden primitives before they touch the pixel pipeline.
//! * [`blur`] — box blur via a summed-area table: O(1) per pixel for *any*
//!   radius, no temporary render targets, no nesting limit.
//! * [`spring`] — closed-form damped-harmonic-oscillator animation, so a
//!   spring can be evaluated at an arbitrary timestamp without ticking.
//! * [`scheduler`] — the hardware-adaptive frame router that decides between a
//!   CPU micro-patch and a GPU macro-frame.
//! * [`gpu`] — `repr(C)` buffer layouts that mirror the WGSL in `shaders/`, plus
//!   the shader source itself.
//!
//! # What this crate does *not* claim
//!
//! Nothing here is claimed to be "the fastest possible renderer". See
//! `README.md` for the measured numbers and `PLAN.md` for the honest gap
//! analysis against GPUI, Rune and Vello. Every claim in those documents is
//! tiered: *repo-managed* (code exists here), *executed* (this crate was run),
//! or *runtime* (a number measured on real hardware).

#![doc(html_no_source)]

pub mod blur;
pub mod gpu;
pub mod layout;
pub mod raster;
pub mod scheduler;
pub mod spring;
pub mod text;
pub mod tree;

pub use layout::{Align, EdgeInsets, FlexDirection, Justify, LayoutProps, Rect, Size};
pub use raster::{FrameBuffer, Paint, RasterStats, Rasterizer, Rgba};
pub use scheduler::{FrameCost, Route, Scheduler, Telemetry};
pub use spring::Spring;
pub use text::{GlyphBitmap, GlyphOutline, TextRun};
pub use tree::{EulerSnapshot, GAP_PERIOD, NONE, Node, NodeId, NodeTree};

/// Crate version, mirrored so the GPU shaders can assert they were built
/// against the same layout revision.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
