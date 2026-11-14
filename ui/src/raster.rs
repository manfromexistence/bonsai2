//! Software rasterization with span-occlusion culling.
//!
//! # The claim under test
//!
//! "Painter's-algorithm overdraw is the reason UI rendering leaves performance
//! on the table; a front-to-back early exit writes every pixel once."
//!
//! That is *half* right, and this module implements the half that is real:
//!
//! * **Real:** in a typical application chrome, the root background, the
//!   sidebar, the content pane and the modal backdrop all cover the same
//!   pixels. Culling the fully hidden ones before they reach the pixel
//!   pipeline is a genuine, measurable saving. [`OcclusionMask`] does that at
//!   16-pixel block granularity using a `u128` per row, so a whole 2048-pixel
//!   row fits in 16 words and the test is one AND per block.
//! * **Not real:** most *real* overdraw is translucent (shadows, glass, blur),
//!   and translucent pixels cannot be culled without changing the image. A
//!   front-to-back "write each pixel exactly once" pipeline only works if every
//!   layer is opaque — which no real UI is.
//!
//! So the honest claim is: **opaque overdraw is culled; translucent overdraw is
//! not, and cannot be.** The backdrop-blur case additionally requires
//! neighbouring pixels, so it is a barrier that no single-pass early exit can
//! cross. See PLAN.md.

use crate::layout::Rect;
use crate::text::GlyphBitmap;
use std::sync::Arc;

/// A linear RGBA colour. Alpha is 0..=1 and *not* premultiplied.
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
}

/// A premultiplied RGBA8 frame buffer.
#[derive(Clone, Debug)]
pub struct FrameBuffer {
    /// Width in pixels.
    pub width: usize,
    /// Height in pixels.
    pub height: usize,
    /// RGBA8 data, premultiplied, row-major.
    pub data: Vec<u8>,
}

impl FrameBuffer {
    /// Allocates a transparent frame buffer.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height * 4],
        }
    }

    /// Fills the whole buffer with a colour.
    pub fn clear(&mut self, color: Rgba) {
        let r = (color.r * color.a * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (color.g * color.a * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (color.b * color.a * 255.0).round().clamp(0.0, 255.0) as u8;
        let a = (color.a * 255.0).round().clamp(0.0, 255.0) as u8;
        for px in self.data.chunks_exact_mut(4) {
            px[0] = r;
            px[1] = g;
            px[2] = b;
            px[3] = a;
        }
    }

    /// Source-over blend of one pixel.
    pub fn blend(&mut self, x: usize, y: usize, color: Rgba, coverage: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let a = color.a * coverage;
        if a <= 0.0 {
            return;
        }
        let i = (y * self.width + x) * 4;
        let dr = f32::from(self.data[i]) / 255.0;
        let dg = f32::from(self.data[i + 1]) / 255.0;
        let db = f32::from(self.data[i + 2]) / 255.0;
        let da = f32::from(self.data[i + 3]) / 255.0;
        let inv = 1.0 - a;
        self.data[i] = ((color.r * a + dr * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
        self.data[i + 1] = ((color.g * a + dg * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
        self.data[i + 2] = ((color.b * a + db * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
        self.data[i + 3] = ((a + da * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
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
    pub fn mark_opaque_rect(&mut self, rect: Rect) {
        let x0 = rect.x.max(0.0).floor() as usize;
        let x1 = (rect.x + rect.w).min(self.width as f32).ceil().max(0.0) as usize;
        let y0 = rect.y.max(0.0).floor() as usize;
        let y1 = (rect.y + rect.h).min(self.height as f32).ceil().max(0.0) as usize;
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let b0 = x0.div_ceil(OCCLUSION_BLOCK);
        let b1 = (x1 - 1) / OCCLUSION_BLOCK;
        for y in y0..y1 {
            let row = y * self.words_per_row;
            for b in b0..=b1 {
                if b / OCCLUSION_BLOCK * OCCLUSION_BLOCK + OCCLUSION_BLOCK <= x1
                    && b * OCCLUSION_BLOCK >= x0
                {
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
    /// Returns `false` whenever a block only partially overlaps `rect`, because
    /// partial coverage cannot be proven opaque at this granularity.
    #[must_use]
    pub fn is_rect_fully_occluded(&self, rect: Rect) -> bool {
        let x0 = rect.x.max(0.0).floor() as usize;
        let x1 = (rect.x + rect.w).min(self.width as f32).ceil().max(0.0) as usize;
        let y0 = rect.y.max(0.0).floor() as usize;
        let y1 = (rect.y + rect.h).min(self.height as f32).ceil().max(0.0) as usize;
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

/// A software rasterizer.
#[derive(Clone, Debug)]
pub struct Rasterizer {
    /// Frame width in pixels.
    pub width: usize,
    /// Frame height in pixels.
    pub height: usize,
    /// Premultiplied RGBA8 pixels.
    pub data: Vec<u8>,
}

impl Rasterizer {
    /// Allocates a transparent rasterizer.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height * 4],
        }
    }

    /// Converts into a frame buffer.
    #[must_use]
    pub fn into_frame_buffer(self) -> FrameBuffer {
        FrameBuffer {
            width: self.width,
            height: self.height,
            data: self.data,
        }
    }

    /// Paints a list of items back-to-front, culling opaque items that are
    /// completely hidden by a later opaque item.
    pub fn paint_all(&mut self, paints: &[Paint]) -> RasterStats {
        let mut stats = RasterStats {
            paints: paints.len(),
            ..RasterStats::default()
        };
        stats.pixels_total = (self.width as u64) * (self.height as u64);

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
            if mask.is_rect_fully_occluded(b) {
                culled[i] = true;
                stats.culled += 1;
                stats.pixels_skipped += rect_pixels(b);
                continue;
            }
            mask.mark_opaque_rect(b);
        }

        // Back-to-front painting.
        for (i, paint) in paints.iter().enumerate() {
            if culled[i] {
                continue;
            }
            stats.pixels_painted += self.draw(paint);
        }
        stats
    }

    /// Draws one item, returning the number of pixels touched.
    fn draw(&mut self, paint: &Paint) -> u64 {
        match paint {
            Paint::Solid { rect, color } => {
                let mut n = 0;
                let x0 = rect.x.max(0.0).floor() as usize;
                let x1 = (rect.x + rect.w).min(self.width as f32).ceil().max(0.0) as usize;
                let y0 = rect.y.max(0.0).floor() as usize;
                let y1 = (rect.y + rect.h).min(self.height as f32).ceil().max(0.0) as usize;
                for y in y0..y1 {
                    for x in x0..x1 {
                        self.blend(x, y, *color, 1.0);
                        n += 1;
                    }
                }
                n
            }
            Paint::RoundedRect {
                rect,
                radius,
                color,
            } => {
                let mut n = 0;
                let x0 = rect.x.max(0.0).floor() as usize;
                let x1 = (rect.x + rect.w).min(self.width as f32).ceil().max(0.0) as usize;
                let y0 = rect.y.max(0.0).floor() as usize;
                let y1 = (rect.y + rect.h).min(self.height as f32).ceil().max(0.0) as usize;
                for y in y0..y1 {
                    for x in x0..x1 {
                        let px = x as f32 + 0.5;
                        let py = y as f32 + 0.5;
                        let d = sd_round_rect(px, py, *rect, *radius);
                        let coverage = (0.5 - d).clamp(0.0, 1.0);
                        if coverage > 0.0 {
                            self.blend(x, y, *color, coverage);
                            n += 1;
                        }
                    }
                }
                n
            }
            Paint::Shadow {
                rect,
                radius,
                blur,
                color,
            } => {
                let mut n = 0;
                let b = paint.bounds().unwrap_or(*rect);
                let x0 = b.x.max(0.0).floor() as usize;
                let x1 = (b.x + b.w).min(self.width as f32).ceil().max(0.0) as usize;
                let y0 = b.y.max(0.0).floor() as usize;
                let y1 = (b.y + b.h).min(self.height as f32).ceil().max(0.0) as usize;
                for y in y0..y1 {
                    for x in x0..x1 {
                        let px = x as f32 + 0.5;
                        let py = y as f32 + 0.5;
                        let d = sd_round_rect(px, py, *rect, *radius);
                        let coverage = shadow_alpha(d, *blur);
                        if coverage > 0.0 {
                            self.blend(x, y, *color, coverage);
                            n += 1;
                        }
                    }
                }
                n
            }
            Paint::Glyph {
                bitmap,
                origin,
                color,
            } => {
                let mut n = 0;
                let ox = origin.0.round() as i64;
                let oy = origin.1.round() as i64;
                for gy in 0..bitmap.height {
                    for gx in 0..bitmap.width {
                        let a = bitmap.at(gx, gy);
                        if a == 0 {
                            continue;
                        }
                        let x = ox + gx as i64;
                        let y = oy + gy as i64;
                        if x < 0 || y < 0 {
                            continue;
                        }
                        self.blend(x as usize, y as usize, *color, f32::from(a) / 255.0);
                        n += 1;
                    }
                }
                n
            }
        }
    }

    fn blend(&mut self, x: usize, y: usize, color: Rgba, coverage: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let a = color.a * coverage;
        if a <= 0.0 {
            return;
        }
        let i = (y * self.width + x) * 4;
        let dr = f32::from(self.data[i]) / 255.0;
        let dg = f32::from(self.data[i + 1]) / 255.0;
        let db = f32::from(self.data[i + 2]) / 255.0;
        let da = f32::from(self.data[i + 3]) / 255.0;
        let inv = 1.0 - a;
        self.data[i] = ((color.r * a + dr * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
        self.data[i + 1] = ((color.g * a + dg * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
        self.data[i + 2] = ((color.b * a + db * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
        self.data[i + 3] = ((a + da * inv) * 255.0).round().clamp(0.0, 255.0) as u8;
    }
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
#[must_use]
pub fn shadow_alpha(sdf: f32, blur: f32) -> f32 {
    if blur <= 0.0 {
        return if sdf <= 0.0 { 1.0 } else { 0.0 };
    }
    (0.5 * (1.0 - erf_approx(sdf / blur))).clamp(0.0, 1.0)
}
