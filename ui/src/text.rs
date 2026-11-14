//! Atlas-free text: outlines, banded evaluation, and cumulative advances.
//!
//! # The claim under test
//!
//! "A glyph atlas is mandatory because evaluating quadratic Béziers per pixel is
//! slower than a bitmap blit, and unhinted outlines look muddy at 11–13 px."
//!
//! Both halves are partly true and both halves have a known answer:
//!
//! * **Band decomposition** (Eric Lengyel's *Slug*) cuts a glyph outline into
//!   horizontal bands so that a scanline only tests the curves that can
//!   possibly cross it. That turns "per-pixel curve evaluation" into "per-pixel
//!   segment evaluation inside one band", which is what makes analytic text
//!   viable at all. [`GlyphOutline::flatten`] builds those bands.
//! * **Small-size muddiness** is a *hinting* problem, not an atlas problem.
//!   The fix is phase snapping plus stem darkening, which is *not* implemented
//!   here yet — see TODO.md. What *is* implemented is the part that does not
//!   need hinting: exact horizontal coverage per scanline, so the glyph is
//!   anti-aliased correctly rather than sampled.
//!
//! # What you lose
//!
//! Without a cache, every frame re-evaluates the outline. That is fine for
//! static text and expensive for a 15 000-character code editor. The honest
//! design is therefore *hybrid*: analytic evaluation for scaled/animated text,
//! and a small tile-local cache for static text — which is exactly what the
//! ecosystem has converged on independently.

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

impl Vec2 {
    /// Constructs a point.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// A contour point. `on_curve` distinguishes TrueType corner points from
/// control points.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    /// X coordinate in font units.
    pub x: f32,
    /// Y coordinate in font units.
    pub y: f32,
    /// Whether this is an on-curve (corner) point.
    pub on_curve: bool,
}

/// A closed contour of a glyph outline.
#[derive(Clone, Debug, Default)]
pub struct Contour {
    /// The contour points, in order.
    pub points: Vec<Point>,
}

/// A glyph outline in font units (em = 1000 by convention).
#[derive(Clone, Debug, Default)]
pub struct GlyphOutline {
    /// Contours, in draw order.
    pub contours: Vec<Contour>,
    /// Advance width in font units.
    pub advance: f32,
}

/// A flattened line segment.
#[derive(Clone, Copy, Debug)]
pub struct Line {
    /// Start point.
    pub a: Vec2,
    /// End point.
    pub b: Vec2,
}

/// A horizontal band of a flattened glyph.
#[derive(Clone, Debug)]
pub struct Band {
    /// Band top in font units.
    pub y0: f32,
    /// Band bottom in font units.
    pub y1: f32,
    /// Indices into [`FlatGlyph::segments`] that can cross this band.
    pub segments: Vec<u32>,
}

/// A glyph flattened into line segments plus a band index.
#[derive(Clone, Debug)]
pub struct FlatGlyph {
    /// Flattened segments.
    pub segments: Vec<Line>,
    /// Horizontal bands, ascending in `y`.
    pub bands: Vec<Band>,
    /// Bounding box `(min_x, min_y, max_x, max_y)` in font units.
    pub bounds: (f32, f32, f32, f32),
}

/// A rasterized glyph: an 8-bit coverage bitmap.
#[derive(Clone, Debug, Default)]
pub struct GlyphBitmap {
    /// Bitmap width in pixels.
    pub width: usize,
    /// Bitmap height in pixels.
    pub height: usize,
    /// Coverage, row-major, `width * height` bytes.
    pub alpha: Vec<u8>,
}

impl GlyphBitmap {
    /// Coverage at a pixel, or 0 if out of bounds.
    #[must_use]
    pub fn at(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            0
        } else {
            self.alpha[y * self.width + x]
        }
    }

    /// Sum of all coverage values, i.e. the "ink area" in pixel units.
    #[must_use]
    pub fn ink_area(&self) -> f64 {
        self.alpha.iter().map(|&a| f64::from(a)).sum::<f64>() / 255.0
    }
}

/// Number of segments used to flatten one quadratic Bézier.
const QUAD_SUBDIVISIONS: usize = 8;

/// Height of a band in font units.
const BAND_HEIGHT: f32 = 64.0;

impl GlyphOutline {
    /// Builds a filled outline approximating a circle, for tests and demos.
    #[must_use]
    pub fn circle(center: Vec2, radius: f32, steps: usize) -> Self {
        let mut points = Vec::with_capacity(steps);
        for i in 0..steps {
            let t = std::f32::consts::TAU * (i as f32) / (steps as f32);
            points.push(Point {
                x: center.x + radius * t.cos(),
                y: center.y + radius * t.sin(),
                on_curve: true,
            });
        }
        Self {
            contours: vec![Contour { points }],
            advance: center.x * 2.0 + radius * 2.0,
        }
    }

    /// Bounding box in font units, or `None` for an empty outline.
    #[must_use]
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for c in &self.contours {
            for p in &c.points {
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                max_x = max_x.max(p.x);
                max_y = max_y.max(p.y);
            }
        }
        if min_x.is_finite() {
            Some((min_x, min_y, max_x, max_y))
        } else {
            None
        }
    }

    /// Flattens the outline into line segments and builds the band index.
    #[must_use]
    pub fn flatten(&self) -> FlatGlyph {
        let mut segments = Vec::new();
        for contour in &self.contours {
            let pts = expand_implied_points(&contour.points);
            let n = pts.len();
            if n < 2 {
                continue;
            }
            for i in 0..n {
                let a = pts[i];
                let b = pts[(i + 1) % n];
                let c = pts[(i + 2) % n];
                let p0 = Vec2::new(a.x, a.y);
                if b.on_curve {
                    segments.push(Line {
                        a: p0,
                        b: Vec2::new(b.x, b.y),
                    });
                } else {
                    let p1 = Vec2::new(b.x, b.y);
                    let p2 = Vec2::new(c.x, c.y);
                    let mut prev = p0;
                    for s in 1..=QUAD_SUBDIVISIONS {
                        let t = s as f32 / QUAD_SUBDIVISIONS as f32;
                        let mt = 1.0 - t;
                        let x = mt * mt * p0.x + 2.0 * mt * t * p1.x + t * t * p2.x;
                        let y = mt * mt * p0.y + 2.0 * mt * t * p1.y + t * t * p2.y;
                        let next = Vec2::new(x, y);
                        segments.push(Line { a: prev, b: next });
                        prev = next;
                    }
                }
            }
        }

        let bounds = self.bounds().unwrap_or((0.0, 0.0, 0.0, 0.0));
        let mut bands = Vec::new();
        if segments.is_empty() {
            return FlatGlyph {
                segments,
                bands,
                bounds,
            };
        }
        let y0 = bounds.1;
        let y1 = bounds.3;
        let n_bands = (((y1 - y0) / BAND_HEIGHT).ceil() as usize).max(1);
        for bi in 0..n_bands {
            let by0 = y0 + bi as f32 * BAND_HEIGHT;
            let by1 = by0 + BAND_HEIGHT;
            let segs: Vec<u32> = segments
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    let sy0 = s.a.y.min(s.b.y);
                    let sy1 = s.a.y.max(s.b.y);
                    sy1 >= by0 && sy0 <= by1
                })
                .map(|(i, _)| i as u32)
                .collect();
            bands.push(Band {
                y0: by0,
                y1: by1,
                segments: segs,
            });
        }

        FlatGlyph {
            segments,
            bands,
            bounds,
        }
    }

    /// Point-in-glyph test using the non-zero winding rule.
    ///
    /// Returns `1.0` inside, `0.0` outside.
    #[must_use]
    pub fn coverage_at(&self, x: f32, y: f32) -> f32 {
        let flat = self.flatten();
        if winding_at(&flat, x, y) != 0 {
            1.0
        } else {
            0.0
        }
    }

    /// Rasterizes the glyph at `px_size` pixels per em.
    ///
    /// `samples_per_row` is the number of vertical sub-scanlines per pixel row;
    /// horizontal coverage within a sub-scanline is computed exactly, so the
    /// only approximation is the vertical one.
    #[must_use]
    pub fn rasterize(&self, px_size: f32, samples_per_row: u32) -> GlyphBitmap {
        let flat = self.flatten();
        let (min_x, min_y, max_x, max_y) = flat.bounds;
        if !min_x.is_finite() || px_size <= 0.0 {
            return GlyphBitmap::default();
        }
        let scale = px_size / 1000.0;
        let width = (((max_x - min_x) * scale).ceil() as usize).max(1);
        let height = (((max_y - min_y) * scale).ceil() as usize).max(1);
        let samples = samples_per_row.max(1);
        let inv_samples = 1.0 / samples as f32;

        let mut acc = vec![0f32; width * height];
        for py in 0..height {
            for s in 0..samples {
                let sy = (py as f32 + (s as f32 + 0.5) * inv_samples) / scale + min_y;
                let spans = scanline_spans(&flat, sy);
                if spans.is_empty() {
                    continue;
                }
                for (sx0, sx1) in spans {
                    // Convert the span from font units to pixel space.
                    let px0 = (sx0 - min_x) * scale;
                    let px1 = (sx1 - min_x) * scale;
                    if px1 <= 0.0 || px0 >= width as f32 {
                        continue;
                    }
                    let px0 = px0.max(0.0);
                    let px1 = px1.min(width as f32);
                    let first = px0.floor() as usize;
                    let last = (px1.ceil() as usize).min(width);
                    for px in first..last {
                        let l = (px as f32).max(px0);
                        let r = ((px + 1) as f32).min(px1);
                        if r > l {
                            acc[py * width + px] += (r - l) * inv_samples;
                        }
                    }
                }
            }
        }

        GlyphBitmap {
            width,
            height,
            alpha: acc
                .into_iter()
                .map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
                .collect(),
        }
    }
}

/// Expands implied on-curve midpoints between consecutive control points and
/// rotates the contour so that it starts on-curve.
fn expand_implied_points(points: &[Point]) -> Vec<Point> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut pts: Vec<Point> = points.to_vec();
    if !pts[0].on_curve {
        if let Some(pos) = pts.iter().rposition(|p| p.on_curve) {
            pts.rotate_left(pos);
        } else {
            // All points are control points: synthesise the implied corner.
            let first = pts[0];
            let last = pts[pts.len() - 1];
            let mid = Point {
                x: 0.5 * (first.x + last.x),
                y: 0.5 * (first.y + last.y),
                on_curve: true,
            };
            pts.push(mid);
            pts.rotate_left(0);
            // Start from the synthesised corner.
            let idx = pts.len() - 1;
            pts.rotate_left(idx);
        }
    }
    let n = pts.len();
    let mut out = Vec::with_capacity(n * 2);
    for i in 0..n {
        let cur = pts[i];
        let next = pts[(i + 1) % n];
        out.push(cur);
        if !cur.on_curve && !next.on_curve {
            out.push(Point {
                x: 0.5 * (cur.x + next.x),
                y: 0.5 * (cur.y + next.y),
                on_curve: true,
            });
        }
    }
    out
}

/// Winding number of the flattened glyph at a point.
fn winding_at(flat: &FlatGlyph, px: f32, py: f32) -> i32 {
    let mut winding = 0;
    for s in &flat.segments {
        let (a, b) = (s.a, s.b);
        if a.y <= py {
            if b.y > py && is_left(a, b, px, py) > 0.0 {
                winding += 1;
            }
        } else if b.y <= py && is_left(a, b, px, py) < 0.0 {
            winding -= 1;
        }
    }
    winding
}

/// Signed cross product of the directed edge `a -> b` against the point.
/// Positive when the point lies to the left of the edge.
fn is_left(a: Vec2, b: Vec2, px: f32, py: f32) -> f32 {
    (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

/// Computes the covered x-spans of the flattened glyph on the scanline `y`.
///
/// Uses the non-zero winding rule.
#[must_use]
pub fn scanline_spans(flat: &FlatGlyph, y: f32) -> Vec<(f32, f32)> {
    let mut crossings: Vec<(f32, i32)> = Vec::new();
    let band = flat
        .bands
        .iter()
        .find(|b| y >= b.y0 && y <= b.y1)
        .or_else(|| flat.bands.last());
    let Some(band) = band else {
        return Vec::new();
    };
    for &si in &band.segments {
        let s = flat.segments[si as usize];
        let (a, b) = (s.a, s.b);
        if (a.y <= y) != (b.y <= y) {
            let t = (y - a.y) / (b.y - a.y);
            let x = a.x + t * (b.x - a.x);
            let dir = if b.y > a.y { 1 } else { -1 };
            crossings.push((x, dir));
        }
    }
    if crossings.is_empty() {
        return Vec::new();
    }
    crossings.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut spans = Vec::new();
    let mut winding = 0;
    let mut start = 0.0f32;
    for (x, dir) in crossings {
        if winding == 0 {
            start = x;
        }
        winding += dir;
        if winding == 0 && x > start {
            spans.push((start, x));
        }
    }
    spans
}

/// A shaped run of text: per-glyph cumulative advances plus break
/// opportunities.
#[derive(Clone, Debug, Default)]
pub struct TextRun {
    /// Cumulative advance width after each glyph. `advances[0]` is the advance
    /// of the first glyph; the width of the range `[a, b)` is
    /// `advances[b-1] - advances[a-1]`.
    pub advances: Vec<f32>,
    /// Indices `b` such that a line may break *before* glyph `b`.
    pub break_points: Vec<usize>,
    /// Line height in pixels.
    pub line_height: f32,
}

impl TextRun {
    /// Builds a run from per-glyph advances, treating every space as a break
    /// opportunity.
    #[must_use]
    pub fn from_glyphs(glyph_advances: &[f32], line_height: f32) -> Self {
        let mut advances = Vec::with_capacity(glyph_advances.len());
        let mut acc = 0.0f32;
        for &g in glyph_advances {
            acc += g;
            advances.push(acc);
        }
        let mut break_points = Vec::new();
        for (i, &g) in glyph_advances.iter().enumerate() {
            if g <= 0.0 || i + 1 == glyph_advances.len() {
                break_points.push(i + 1);
            }
        }
        Self {
            advances,
            break_points,
            line_height,
        }
    }

    /// Width of the whole run.
    #[must_use]
    pub fn max_content_width(&self) -> f32 {
        self.advances.last().copied().unwrap_or(0.0)
    }

    /// Width of the widest unbreakable chunk.
    #[must_use]
    pub fn min_content_width(&self) -> f32 {
        if self.advances.is_empty() {
            return 0.0;
        }
        let mut widest = 0.0f32;
        let mut prev = 0usize;
        for &b in &self.break_points {
            let w = self.advances[b - 1]
                - if prev == 0 {
                    0.0
                } else {
                    self.advances[prev - 1]
                };
            widest = widest.max(w);
            prev = b;
        }
        widest
    }

    /// Greedy line breaking, using a binary search over the cumulative advance
    /// table for each line.
    ///
    /// Returns `(start, end)` glyph index pairs, one per line.
    #[must_use]
    pub fn line_ranges(&self, width: f32) -> Vec<(usize, usize)> {
        let n = self.advances.len();
        let mut lines = Vec::new();
        if n == 0 {
            return lines;
        }
        let width = width.max(0.0);
        let mut start = 0usize;
        loop {
            let base = if start == 0 {
                0.0
            } else {
                self.advances[start - 1]
            };
            let limit = width + base;
            let first = self.break_points.partition_point(|&b| b <= start);
            // Last break point whose segment still fits.
            let mut lo = first;
            let mut hi = self.break_points.len();
            let mut best: Option<usize> = None;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if self.advances[self.break_points[mid] - 1] <= limit {
                    best = Some(mid);
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            let end = match best {
                Some(m) => self.break_points[m],
                None => n,
            };
            lines.push((start, end));
            if end >= n {
                break;
            }
            start = end;
        }
        lines
    }

    /// Number of lines the run occupies at the given width.
    #[must_use]
    pub fn line_count(&self, width: f32) -> usize {
        self.line_ranges(width)
            .len()
            .max(if self.advances.is_empty() { 0 } else { 1 })
    }
}
