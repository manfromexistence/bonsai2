//! Software rasterization: span fills, integer blending, damage tracking.
//!
//! # What was wrong, and what is right
//!
//! The first version of this module blended **every** pixel of **every**
//! paint through the full source-over formula:
//!
//! ```text
//! dr = f32::from(data[i]) / 255.0;              // convert
//! out = (color.r * a + dr * inv) * 255.0        // float multiply-add
//!        .round().clamp(0.0, 255.0) as u8;      // round, clamp, convert back
//! ```
//!
//! Four conversions and a divide **per channel, per pixel**, with a branch in
//! the middle, on a hot path that touched ~2 million pixels a frame. On the
//! Ryzen 5 5600G that measured **27.2 ms per frame** — about 58 clock cycles
//! per pixel touched, on a core that can retire six macro-ops a cycle.
//!
//! Two things fix almost all of it:
//!
//! 1. **An opaque paint is a copy, not a blend.** Source-over with `a == 1`
//!    degenerates to `out = src`. So opaque spans become a pattern fill
//!    (`slice::fill`), which the compiler lowers to wide stores. No
//!    conversion, no divide, no branch.
//! 2. **A rounded rectangle is mostly a rectangle.** The interior of a rounded
//!    rect is at least `radius` away from any edge, so those pixels are
//!    fully covered and need no distance function at all. Only the two
//!    horizontal bands containing the corners, plus the two partial pixels at
//!    each end of a row, actually need the SDF.
//!
//! What is left is stored as **`u32` pixels** (`0xAARRGGBB`, premultiplied) so
//! that:
//!
//! * a span fill is one `u32` store per pixel with no channel shuffling,
//! * the translucent blend is **integer** and branchless (the exact
//!   divide-by-255 identity `div255`), so LLVM can vectorise it without a
//!   single `unsafe` block in this crate,
//! * the bytes handed to GDI are already in the `BGRA` order a 32-bit
//!   `BI_RGB` DIB wants — the Win32 host blits with **zero conversion**.
//!
//! # Damage tracking
//!
//! [`Rasterizer::paint_all`] takes an optional damage rectangle. When one is
//! given, the framebuffer is **not** cleared: only the damaged region is
//! repainted, and every paint is clipped to it. This is how a UI engine keeps
//! a 60 Hz animation off the critical path — when one 180x120 card moves, the
//! work is one card's worth of pixels, not the whole frame.
//!
//! The correctness assumption is that the scene has an **opaque backdrop
//! covering the viewport**: the damaged region is reset by repainting that
//! backdrop, and everything on top of it is repainted in the same pass. A
//! scene with transparency at the edges of the damage region will show stale
//! pixels.

use crate::layout::Rect;
use crate::text::GlyphBitmap;
use std::sync::Arc;

/// A packed pixel: `0xAARRGGBB`, premultiplied.
///
/// In memory (little-endian) the byte order is `B, G, R, A` — which is
/// exactly what a 32-bit `BI_RGB` DIB expects, so the Win32 host can blit the
/// framebuffer without touching a byte.
pub type Pixel = u32;

/// A linear RGBA colour with non-premultiplied alpha in 0..=1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

impl Rgba {
    /// Fully transparent.
    pub const TRANSPARENT: Rgba = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    /// Constructs a colour.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    /// Constructs an opaque colour.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

/// Packs a colour into a premultiplied pixel.
#[inline]
fn pack(c: Rgba) -> Pixel {
    let a = c.a.clamp(0.0, 1.0);
    let r = (c.r.clamp(0.0, 1.0) * a * 255.0 + 0.5) as u32;
    let g = (c.g.clamp(0.0, 1.0) * a * 255.0 + 0.5) as u32;
    let b = (c.b.clamp(0.0, 1.0) * a * 255.0 + 0.5) as u32;
    let alpha = (a * 255.0 + 0.5) as u32;
    (alpha << 24) | (r << 16) | (g << 8) | b
}

/// Exact division by 255 for `x` in `0..=65025`.
///
/// `div255(x) == x / 255` exactly, using the shift-add identity. Integer, so
/// the blend loop has no float in it and vectorises cleanly.
#[inline]
const fn div255(x: u32) -> u32 {
    let t = x + 128;
    (t + (t >> 8)) >> 8
}

/// Branchless premultiplied source-over of a constant source over a span.
///
/// Every channel is `s + d * (255 - sa) / 255`, evaluated in integers.
#[inline]
fn blend_span(dst: &mut [u32], src: Pixel) {
    let sa = src >> 24;
    let inv = 255 - sa;
    let sb = src & 0xff;
    let sg = (src >> 8) & 0xff;
    let sr = (src >> 16) & 0xff;

    for d in dst.iter_mut() {
        let db = *d & 0xff;
        let dg = (*d >> 8) & 0xff;
        let dr = (*d >> 16) & 0xff;
        let da = *d >> 24;
        let ob = (sb + div255(db * inv)).min(255);
        let og = (sg + div255(dg * inv)).min(255);
        let orr = (sr + div255(dr * inv)).min(255);
        let oa = (sa + div255(da * inv)).min(255);
        *d = (oa << 24) | (orr << 16) | (og << 8) | ob;
    }
}

/// A drawable item.
#[derive(Clone, Debug)]
pub enum Paint {
    /// An axis-aligned rectangle.
    Solid {
        /// Target rectangle.
        rect: Rect,
        /// Fill colour.
        color: Rgba,
    },
    /// A rounded rectangle, anti-aliased with a signed distance field.
    RoundedRect {
        /// Target rectangle.
        rect: Rect,
        /// Corner radius in pixels.
        radius: f32,
        /// Fill colour.
        color: Rgba,
    },
    /// A soft drop shadow, approximated with an erf falloff.
    Shadow {
        /// The shape casting the shadow.
        rect: Rect,
        /// Corner radius.
        radius: f32,
        /// Blur radius in pixels.
        blur: f32,
        /// Shadow colour.
        color: Rgba,
    },
    /// A rasterized glyph.
    Glyph {
        /// Coverage bitmap.
        bitmap: Arc<GlyphBitmap>,
        /// Top-left corner in pixels.
        origin: (f32, f32),
        /// Colour multiplied by the coverage.
        color: Rgba,
    },
}

impl Paint {
    /// The area this paint touches, if any.
    #[must_use]
    pub fn bounds(&self) -> Option<Rect> {
        match self {
            Paint::Solid { rect, .. } | Paint::RoundedRect { rect, .. } => Some(*rect),
            Paint::Shadow { rect, blur, .. } => Some(Rect::new(
                rect.x - blur,
                rect.y - blur,
                rect.w + blur * 2.0,
                rect.h + blur * 2.0,
            )),
            Paint::Glyph { bitmap, origin, .. } => Some(Rect::new(
                origin.0,
                origin.1,
                bitmap.width as f32,
                bitmap.height as f32,
            )),
        }
    }

    /// Whether this paint fully replaces the pixels under it.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        match self {
            Paint::Solid { color, .. } | Paint::RoundedRect { color, .. } => color.a >= 1.0,
            Paint::Shadow { .. } | Paint::Glyph { .. } => false,
        }
    }

    /// The kind of paint, for accounting.
    #[must_use]
    pub fn kind(&self) -> PaintKind {
        match self {
            Paint::Solid { .. } => PaintKind::Solid,
            Paint::RoundedRect { .. } => PaintKind::Rounded,
            Paint::Shadow { .. } => PaintKind::Shadow,
            Paint::Glyph { .. } => PaintKind::Glyph,
        }
    }
}

/// The category of a paint, for per-kind pixel accounting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaintKind {
    /// Axis-aligned rectangle.
    Solid,
    /// Rounded rectangle.
    Rounded,
    /// Soft shadow.
    Shadow,
    /// Glyph coverage blit.
    Glyph,
}

/// A premultiplied RGBA8 frame buffer.
#[derive(Clone, Debug)]
pub struct FrameBuffer {
    /// Width in pixels.
    pub width: usize,
    /// Height in pixels.
    pub height: usize,
    /// Packed pixels, premultiplied, row-major.
    pub pixels: Vec<Pixel>,
}

impl FrameBuffer {
    /// Allocates a transparent frame buffer.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }

    /// Fills the whole buffer with a colour.
    pub fn clear(&mut self, color: Rgba) {
        self.pixels.fill(pack(color));
    }

    /// The alpha of one pixel, 0..=255.
    #[must_use]
    pub fn alpha_at(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            0
        } else {
            (self.pixels[y * self.width + x] >> 24) as u8
        }
    }

    /// The red channel of one pixel, 0..=255.
    #[must_use]
    pub fn red_at(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            0
        } else {
            ((self.pixels[y * self.width + x] >> 16) & 0xff) as u8
        }
    }

    /// Paints into this frame buffer, delegating to a [`Rasterizer`].
    pub fn paint_all(&mut self, paints: &[Paint], damage: &[Rect]) -> RasterStats {
        let raster = Rasterizer::new(self.width, self.height);
        raster.paint_all(&mut self.pixels, paints, damage)
    }
}

/// Granularity of the occlusion mask, in pixels.
pub const OCCLUSION_BLOCK: usize = 16;

/// A per-row bitmask of opaque 16-pixel blocks.
#[derive(Clone, Debug)]
pub struct OcclusionMask {
    width: usize,
    height: usize,
    words_per_row: usize,
    bits: Vec<u128>,
}

impl OcclusionMask {
    /// Allocates an empty mask.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let blocks = width.div_ceil(OCCLUSION_BLOCK).max(1);
        let words_per_row = blocks.div_ceil(128).max(1);
        Self {
            width,
            height,
            words_per_row,
            bits: vec![0; words_per_row * height],
        }
    }

    /// Marks every fully covered 16-pixel block of `rect` as opaque.
    ///
    /// `clip` bounds the work: with damage tracking only a small region
    /// changes, and marking the whole frame would cost more than the
    /// repaint it is meant to save.
    pub fn mark_opaque_rect(&mut self, rect: Rect, clip: Rect) {
        let x0 = rect.x.max(clip.x).max(0.0).floor() as usize;
        let x1 = (rect.x + rect.w)
            .min(clip.x + clip.w)
            .min(self.width as f32)
            .ceil()
            .max(0.0) as usize;
        let y0 = rect.y.max(clip.y).max(0.0).floor() as usize;
        let y1 = (rect.y + rect.h)
            .min(clip.y + clip.h)
            .min(self.height as f32)
            .ceil()
            .max(0.0) as usize;
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let b0 = x0.div_ceil(OCCLUSION_BLOCK);
        let b1 = (x1 - 1) / OCCLUSION_BLOCK;
        for y in y0..y1 {
            let row = y * self.words_per_row;
            for b in b0..=b1 {
                if b * OCCLUSION_BLOCK + OCCLUSION_BLOCK <= x1 && b * OCCLUSION_BLOCK >= x0 {
                    self.bits[row + b / 128] |= 1u128 << (b % 128);
                }
            }
        }
    }

    /// Whether the block containing pixel column `block` is marked opaque in
    /// `row`.
    #[must_use]
    pub fn block_is_opaque(&self, row: usize, block: usize) -> bool {
        if row >= self.height {
            return false;
        }
        self.bits[row * self.words_per_row + block / 128] & (1u128 << (block % 128)) != 0
    }

    /// Conservative test: is every pixel of `rect` covered by an opaque block?
    ///
    /// Only the part of `rect` inside `clip` is considered.
    #[must_use]
    pub fn is_rect_fully_occluded(&self, rect: Rect, clip: Rect) -> bool {
        let x0 = rect.x.max(clip.x).max(0.0).floor() as usize;
        let x1 = (rect.x + rect.w)
            .min(clip.x + clip.w)
            .min(self.width as f32)
            .ceil()
            .max(0.0) as usize;
        let y0 = rect.y.max(clip.y).max(0.0).floor() as usize;
        let y1 = (rect.y + rect.h)
            .min(clip.y + clip.h)
            .min(self.height as f32)
            .ceil()
            .max(0.0) as usize;
        if x1 <= x0 || y1 <= y0 {
            return true;
        }
        let b0 = x0 / OCCLUSION_BLOCK;
        let b1 = (x1 - 1) / OCCLUSION_BLOCK;
        for y in y0..y1 {
            for b in b0..=b1 {
                if !self.block_is_opaque(y, b) {
                    return false;
                }
            }
        }
        true
    }
}

/// Statistics from a rasterization pass.
#[derive(Clone, Copy, Debug, Default)]
pub struct RasterStats {
    /// Total paints submitted.
    pub paints: usize,
    /// Paints removed by the occlusion culler.
    pub culled: usize,
    /// Pixel-touches performed by the paints that were actually drawn.
    pub pixels_painted: u64,
    /// Pixel-touches skipped because the paint was fully occluded.
    pub pixels_skipped: u64,
    /// Pixels in the frame.
    pub pixels_total: u64,
    /// Pixel-touches per paint kind: `[solid, rounded, shadow, glyph]`.
    pub touches_by_kind: [u64; 4],
    /// Microseconds per paint kind, same indexing, only populated when
    /// [`Rasterizer::profile`] is set. The sum is a little more than the
    /// raster stage time because each timestamp costs ~25 ns.
    pub us_by_kind: [u64; 4],
}

impl RasterStats {
    /// Fraction of pixel-touches that the occlusion culler removed.
    #[must_use]
    pub fn culled_fraction(&self) -> f64 {
        let total = self.pixels_painted + self.pixels_skipped;
        if total == 0 {
            0.0
        } else {
            self.pixels_skipped as f64 / total as f64
        }
    }
}

/// A software rasterizer: geometry plus the paint algorithms.
///
/// It does **not** own a framebuffer. The caller supplies the pixel slice on
/// every call, which is what makes zero-copy rendering possible: a Win32 host
/// can hand over the `CreateDIBSection` bits and the rasterizer writes into
/// them directly, removing the engine-to-DIB row copy entirely. For an owned
/// buffer, use [`FrameBuffer`].
///
/// Because the caller owns the buffer, damage tracking keeps working the same
/// way: the caller reuses the same slice across frames and passes the damage
/// regions.
#[derive(Clone, Debug)]
pub struct Rasterizer {
    /// Frame width in pixels.
    pub width: usize,
    /// Frame height in pixels.
    pub height: usize,
    /// When true, [`RasterStats::us_by_kind`] is populated. Measuring a
    /// timestamp per paint costs roughly 25 ns each, which is measurable on a
    /// 430 us frame, so it is opt-in rather than always-on.
    pub profile: bool,
}

impl Rasterizer {
    /// Creates a rasterizer for a frame of the given size.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            profile: false,
        }
    }

    /// Paints a list of items back-to-front, culling opaque items that are
    /// completely hidden by a later opaque item.
    ///
    /// `damage` is a **list** of regions, not a single rectangle. An empty
    /// list means "repaint everything" and clears the framebuffer first; a
    /// non-empty list repaints only those regions and **keeps** the rest of
    /// the framebuffer, which is what makes an incremental animation cheap.
    ///
    /// The regions must be **disjoint**. A paint is drawn once per region it
    /// intersects, so overlapping regions would double-blend translucent
    /// paint. A single bounding box around several disjoint regions is not a
    /// valid damage list -- that is the classic way to accidentally
    /// reintroduce a full-frame repaint.
    pub fn paint_all(
        &self,
        pixels: &mut [Pixel],
        paints: &[Paint],
        damage: &[Rect],
    ) -> RasterStats {
        let mut stats = RasterStats {
            paints: paints.len(),
            pixels_total: (self.width as u64) * (self.height as u64),
            ..RasterStats::default()
        };

        debug_assert!(
            damage_is_disjoint(damage),
            "damage regions must be disjoint: a paint is drawn once per region \
             it intersects, so overlapping regions double-blend translucent paint"
        );

        let full_frame = Rect::new(0.0, 0.0, self.width as f32, self.height as f32);
        let clips: Vec<Rect> = if damage.is_empty() {
            pixels.fill(0);
            vec![full_frame]
        } else {
            damage
                .iter()
                .copied()
                .filter(|r| r.w > 0.0 && r.h > 0.0)
                .collect()
        };
        if clips.is_empty() {
            return stats;
        }
        // The occlusion mask works on one rectangle, so it uses the bounding
        // box of the damage. That is only used to skip fully hidden paints,
        // so an over-approximation is safe.
        let mask_rect = clips
            .iter()
            .fold(None::<Rect>, |acc, r| {
                Some(match acc {
                    None => *r,
                    Some(a) => Rect::new(
                        a.x.min(r.x),
                        a.y.min(r.y),
                        (a.x + a.w).max(r.x + r.w) - a.x.min(r.x),
                        (a.y + a.h).max(r.y + r.h) - a.y.min(r.y),
                    ),
                })
            })
            .unwrap_or(full_frame);
        let clip = mask_rect;

        // Front-to-back prepass: work out which pixels are guaranteed opaque,
        // and which paints are therefore completely hidden.
        //
        // The mask only contains paints that are *in front of* the one being
        // tested, so a paint is never culled by its own coverage.
        let mut mask = OcclusionMask::new(self.width, self.height);
        let mut culled = vec![false; paints.len()];
        for (i, paint) in paints.iter().enumerate().rev() {
            if !paint.is_opaque() {
                continue;
            }
            let Some(b) = paint.bounds() else {
                continue;
            };
            if mask.is_rect_fully_occluded(b, clip) {
                culled[i] = true;
                stats.culled += 1;
                stats.pixels_skipped += rect_pixels(b);
                continue;
            }
            mask.mark_opaque_rect(b, clip);
        }

        // Back-to-front painting.
        for (i, paint) in paints.iter().enumerate() {
            if culled[i] {
                continue;
            }
            let kind = paint.kind() as usize;
            for clip in &clips {
                let t0 = self.profile.then(std::time::Instant::now);
                let touched = match paint {
                    Paint::Solid { rect, color } => self.draw_solid(pixels, *rect, *color, *clip),
                    Paint::RoundedRect {
                        rect,
                        radius,
                        color,
                    } => self.draw_rounded(pixels, *rect, *radius, *color, *clip),
                    Paint::Shadow {
                        rect,
                        radius,
                        blur,
                        color,
                    } => self.draw_shadow(pixels, *rect, *radius, *blur, *color, *clip),
                    Paint::Glyph {
                        bitmap,
                        origin,
                        color,
                    } => self.draw_glyph(pixels, bitmap, *origin, *color, *clip),
                };
                if let (true, Some(t0)) = (self.profile, t0) {
                    stats.us_by_kind[kind] += t0.elapsed().as_micros() as u64;
                }
                stats.pixels_painted += touched;
                stats.touches_by_kind[kind] += touched;
            }
        }
        stats
    }

    /// Fills a rectangle.
    fn draw_solid(&self, pixels: &mut [Pixel], rect: Rect, color: Rgba, clip: Rect) -> u64 {
        let src = pack(color);
        let opaque = color.a >= 1.0;
        let Some((x0, x1, y0, y1)) = intersect(rect, clip, self.width, self.height) else {
            return 0;
        };
        let mut n = 0u64;
        for y in y0..y1 {
            let row = y * self.width;
            let span = &mut pixels[row + x0..row + x1];
            if opaque {
                for p in span.iter_mut() {
                    *p = src;
                }
            } else {
                blend_span(span, src);
            }
            n += (x1 - x0) as u64;
        }
        n
    }

    /// Fills a rounded rectangle.
    ///
    /// The interior is split out of the SDF work: only the rows that contain a
    /// corner, and the two partial pixels at the end of every row, need the
    /// distance function.
    fn draw_rounded(
        &self,
        pixels: &mut [Pixel],
        rect: Rect,
        radius: f32,
        color: Rgba,
        clip: Rect,
    ) -> u64 {
        let src = pack(color);
        let opaque = color.a >= 1.0;
        let hw = rect.w * 0.5;
        let hh = rect.h * 0.5;
        let r = radius.clamp(0.0, hw.min(hh));
        let cx = rect.x + hw;
        let cy = rect.y + hh;

        let Some((cx0, cx1, cy0, cy1)) = intersect(rect, clip, self.width, self.height) else {
            return 0;
        };

        let mut n = 0u64;
        for y in cy0..cy1 {
            let sy = y as f32 + 0.5;
            let dy = (sy - cy).abs() - (hh - r);
            if dy > r {
                continue; // outside the shape entirely
            }
            let halfw = if dy <= 0.0 {
                hw
            } else {
                hw - r + (r * r - dy * dy).max(0.0).sqrt()
            };
            let sx0 = cx - halfw;
            let sx1 = cx + halfw;

            // Pixels entirely inside the shape: [x, x+1] within [sx0, sx1].
            let full_lo = sx0.ceil().max(cx0 as f32) as usize;
            let full_hi = ((sx1 - 1.0).floor() as usize + 1).min(cx1);
            if full_lo < full_hi {
                let row = y * self.width;
                let span = &mut pixels[row + full_lo..row + full_hi];
                if opaque {
                    for p in span.iter_mut() {
                        *p = src;
                    }
                } else {
                    blend_span(span, src);
                }
                n += (full_hi - full_lo) as u64;
            }

            // The partial pixels at each end of the row.
            let edge_lo = sx0.floor().max(cx0 as f32) as usize;
            let edge_hi = (sx1.ceil() as usize).min(cx1);
            for x in edge_lo..full_lo.min(edge_hi) {
                self.blend_edge(pixels, y, x, rect, r, color, opaque);
                n += 1;
            }
            for x in full_hi.max(edge_lo)..edge_hi {
                self.blend_edge(pixels, y, x, rect, r, color, opaque);
                n += 1;
            }
        }
        n
    }

    /// Anti-aliased edge pixel of a rounded rectangle.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn blend_edge(
        &self,
        pixels: &mut [Pixel],
        y: usize,
        x: usize,
        rect: Rect,
        radius: f32,
        color: Rgba,
        opaque: bool,
    ) {
        let px = x as f32 + 0.5;
        let py = y as f32 + 0.5;
        let d = sd_round_rect(px, py, rect, radius);
        let coverage = (0.5 - d).clamp(0.0, 1.0);
        if coverage <= 0.0 {
            return;
        }
        if opaque && coverage >= 1.0 {
            pixels[y * self.width + x] = pack(color);
        } else {
            self.blend_pixel(pixels, y, x, color, coverage);
        }
    }

    /// Draws a soft shadow.
    ///
    /// The interior of the shadow is a constant-alpha span per row, so only
    /// the band near the shape's edge is evaluated per pixel.
    fn draw_shadow(
        &self,
        pixels: &mut [Pixel],
        rect: Rect,
        radius: f32,
        blur: f32,
        color: Rgba,
        clip: Rect,
    ) -> u64 {
        let hw = rect.w * 0.5;
        let hh = rect.h * 0.5;
        let r = radius.clamp(0.0, hw.min(hh));
        let cx = rect.x + hw;
        let cy = rect.y + hh;

        let outer = Rect::new(
            rect.x - blur,
            rect.y - blur,
            rect.w + blur * 2.0,
            rect.h + blur * 2.0,
        );
        let Some((cx0, cx1, cy0, cy1)) = intersect(outer, clip, self.width, self.height) else {
            return 0;
        };

        let mut n = 0u64;
        for y in cy0..cy1 {
            let sy = y as f32 + 0.5;
            let dy = (sy - cy).abs() - (hh - r);
            if dy > r + blur {
                continue;
            }
            // Half width of the shape on this row.
            let halfw = if dy <= 0.0 {
                hw
            } else {
                hw - r + (r * r - dy * dy).max(0.0).sqrt()
            };
            // The shadow extends `blur` beyond the shape.
            let ext = halfw + blur;
            let row = y * self.width;

            let sx0 = (cx - ext).max(cx0 as f32) as usize;
            let sx1 = ((cx + ext).ceil() as usize).min(cx1);
            if sx0 >= sx1 {
                continue;
            }

            // Inside the shape's straight-edge region the vertical distance
            // dominates, so the alpha is constant along the row: blend it as
            // one span. The rest is evaluated per pixel.
            let straight = (cx - hw + r + dy.min(0.0)).max(cx0 as f32);
            let straight_end = (cx + hw - r - dy.min(0.0)).min(cx1 as f32);
            let s_lo = sx0.max(straight.ceil() as usize);
            let s_hi = (straight_end.floor() as usize + 1).min(sx1);

            if s_lo < s_hi {
                let alpha = shadow_alpha(dy - r, blur);
                if alpha > 0.0 {
                    let src = pack(Rgba::new(color.r, color.g, color.b, color.a * alpha));
                    blend_span(&mut pixels[row + s_lo..row + s_hi], src);
                }
                n += (s_hi - s_lo) as u64;
            }

            // In the middle band the distance to the shape is purely
            // horizontal (`|px - cx| - hw`), so the side bands need no sqrt
            // and no full signed-distance evaluation. Only corner rows do.
            let middle_band = dy <= 0.0;
            for x in sx0..s_lo {
                let d = if middle_band {
                    (x as f32 + 0.5 - cx).abs() - hw
                } else {
                    sd_round_rect(x as f32 + 0.5, sy, rect, r)
                };
                let alpha = shadow_alpha(d, blur);
                if alpha > 0.0 {
                    self.blend_pixel(pixels, y, x, color, alpha);
                }
                n += 1;
            }
            for x in s_hi..sx1 {
                let d = if middle_band {
                    (x as f32 + 0.5 - cx).abs() - hw
                } else {
                    sd_round_rect(x as f32 + 0.5, sy, rect, r)
                };
                let alpha = shadow_alpha(d, blur);
                if alpha > 0.0 {
                    self.blend_pixel(pixels, y, x, color, alpha);
                }
                n += 1;
            }
        }
        n
    }

    /// Blits a glyph coverage bitmap.
    fn draw_glyph(
        &self,
        pixels: &mut [Pixel],
        bitmap: &GlyphBitmap,
        origin: (f32, f32),
        color: Rgba,
        clip: Rect,
    ) -> u64 {
        let ox = origin.0.round() as i64;
        let oy = origin.1.round() as i64;
        let bounds = Rect::new(
            origin.0,
            origin.1,
            bitmap.width as f32,
            bitmap.height as f32,
        );
        let Some((cx0, cx1, cy0, cy1)) = intersect(bounds, clip, self.width, self.height) else {
            return 0;
        };
        let mut n = 0u64;
        for y in cy0..cy1 {
            let gy = (y as i64 - oy) as usize;
            if gy >= bitmap.height {
                continue;
            }
            for x in cx0..cx1 {
                let gx = (x as i64 - ox) as usize;
                if gx >= bitmap.width {
                    continue;
                }
                let a = bitmap.at(gx, gy);
                if a == 0 {
                    continue;
                }
                self.blend_pixel(pixels, y, x, color, f32::from(a) / 255.0);
                n += 1;
            }
        }
        n
    }

    /// Blends one pixel with a fractional coverage.
    #[inline]
    fn blend_pixel(&self, pixels: &mut [Pixel], y: usize, x: usize, color: Rgba, coverage: f32) {
        let src = pack(Rgba::new(
            color.r,
            color.g,
            color.b,
            color.a * coverage.clamp(0.0, 1.0),
        ));
        let i = y * self.width + x;
        let dst = pixels[i];
        let sa = src >> 24;
        if sa == 255 {
            pixels[i] = src;
            return;
        }
        if sa == 0 {
            return;
        }
        let inv = 255 - sa;
        let db = dst & 0xff;
        let dg = (dst >> 8) & 0xff;
        let dr = (dst >> 16) & 0xff;
        let da = dst >> 24;
        let sb = src & 0xff;
        let sg = (src >> 8) & 0xff;
        let sr = (src >> 16) & 0xff;
        let ob = (sb + div255(db * inv)).min(255);
        let og = (sg + div255(dg * inv)).min(255);
        let orr = (sr + div255(dr * inv)).min(255);
        let oa = (sa + div255(da * inv)).min(255);
        pixels[i] = (oa << 24) | (orr << 16) | (og << 8) | ob;
    }
}

/// Whether a list of rectangles is pairwise disjoint.
///
/// Debug-only use: overlapping damage regions make translucent paints blend
/// twice, which is a correctness bug rather than a performance bug.
fn damage_is_disjoint(damage: &[Rect]) -> bool {
    for (i, a) in damage.iter().enumerate() {
        for b in damage.iter().skip(i + 1) {
            let ax1 = a.x + a.w;
            let ay1 = a.y + a.h;
            let bx1 = b.x + b.w;
            let by1 = b.y + b.h;
            let overlaps = a.x < bx1 && b.x < ax1 && a.y < by1 && b.y < ay1;
            if overlaps {
                return false;
            }
        }
    }
    true
}

/// Intersects a rectangle with a clip rectangle and the frame bounds.
///
/// Returns the inclusive-start, exclusive-end pixel range `(x0, x1, y0, y1)`.
fn intersect(
    rect: Rect,
    clip: Rect,
    width: usize,
    height: usize,
) -> Option<(usize, usize, usize, usize)> {
    let x0 = rect.x.max(clip.x).max(0.0).floor().max(0.0) as usize;
    let y0 = rect.y.max(clip.y).max(0.0).floor().max(0.0) as usize;
    let x1 = (rect.x + rect.w)
        .min(clip.x + clip.w)
        .min(width as f32)
        .ceil()
        .max(0.0) as usize;
    let y1 = (rect.y + rect.h)
        .min(clip.y + clip.h)
        .min(height as f32)
        .ceil()
        .max(0.0) as usize;
    if x1 <= x0 || y1 <= y0 {
        return None;
    }
    Some((x0, x1, y0, y1))
}

/// Number of pixels a rectangle covers.
fn rect_pixels(rect: Rect) -> u64 {
    let w = rect.w.max(0.0);
    let h = rect.h.max(0.0);
    (w * h) as u64
}

/// Signed distance to a rounded rectangle (negative inside).
#[must_use]
pub fn sd_round_rect(px: f32, py: f32, rect: Rect, radius: f32) -> f32 {
    let hw = rect.w * 0.5;
    let hh = rect.h * 0.5;
    let r = radius.clamp(0.0, hw.min(hh));
    let cx = rect.x + hw;
    let cy = rect.y + hh;
    let qx = (px - cx).abs() - (hw - r);
    let qy = (py - cy).abs() - (hh - r);
    let ax = qx.max(0.0);
    let ay = qy.max(0.0);
    (ax * ax + ay * ay).sqrt() + qx.max(qy).min(0.0) - r
}

/// Approximation of the error function (Abramowitz & Stegun 7.1.26).
///
/// Computed in `f64` because the published coefficients carry more precision
/// than `f32` can hold; the result is narrowed once, at the end.
fn erf_approx(x: f32) -> f32 {
    let x = f64::from(x);
    let s = x.signum();
    let a = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * a);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-a * a).exp();
    (s * y) as f32
}

/// Shadow coverage from a signed distance and a blur radius.
///
/// The profile depends only on `sdf / blur`, a one-dimensional curve, so it is
/// sampled from a lookup table instead of evaluating the error function per
/// pixel. The shadow alone was measuring 207-281 us of a 520 us frame with the
/// erf evaluated per pixel; the table is accurate to about 0.2% of range,
/// which is far below what a human can see in a soft shadow.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn shadow_alpha(sdf: f32, blur: f32) -> f32 {
    if blur <= 0.0 {
        return if sdf <= 0.0 { 1.0 } else { 0.0 };
    }
    let t = sdf / blur;
    if t <= SHADOW_T_MIN {
        return SHADOW_ALPHA_MAX;
    }
    if t >= SHADOW_T_MAX {
        return 0.0;
    }
    let lut = shadow_lut();
    let x = (t - SHADOW_T_MIN) / (SHADOW_T_MAX - SHADOW_T_MIN) * SHADOW_LUT_STEPS as f32;
    let i = (x as usize).min(SHADOW_LUT_STEPS - 1);
    let f = x - i as f32;
    (lut[i] + (lut[i + 1] - lut[i]) * f).clamp(0.0, 1.0)
}

/// The profile is flat outside `[-3, 3]`: erf(-3) = -0.995, so alpha is
/// within 0.25% of full, and erf(3) is within 0.11% of empty.
const SHADOW_T_MIN: f32 = -3.0;
const SHADOW_T_MAX: f32 = 3.0;
const SHADOW_LUT_STEPS: usize = 256;
const SHADOW_ALPHA_MAX: f32 = 0.5 * (1.0 - SHADOW_ERF3);

// erf(3), used for the saturation value above. The trailing digits are the
// published double-precision value narrowed once, deliberately: rounding them
// off here would move the shadow's saturation point by a visible amount.
#[allow(clippy::excessive_precision)]
const SHADOW_ERF3: f32 = 0.9953222650189527;

/// The shadow profile, sampled once on first use.
fn shadow_lut() -> &'static [f32; SHADOW_LUT_STEPS + 1] {
    static LUT: std::sync::OnceLock<[f32; SHADOW_LUT_STEPS + 1]> = std::sync::OnceLock::new();
    LUT.get_or_init(|| {
        let mut lut = [0f32; SHADOW_LUT_STEPS + 1];
        for (i, slot) in lut.iter_mut().enumerate() {
            let t = SHADOW_T_MIN
                + (SHADOW_T_MAX - SHADOW_T_MIN) * (i as f32) / (SHADOW_LUT_STEPS as f32);
            *slot = (0.5 * (1.0 - erf_approx(t))).clamp(0.0, 1.0);
        }
        lut
    })
}
