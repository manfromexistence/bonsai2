//! Text wrapping, glyph coverage, and rasterization behaviour.

use ui::raster::{FrameBuffer, Paint, Rgba};
use ui::text::{GlyphOutline, TextRun, Vec2};

#[test]
fn greedy_wrap_matches_reference() {
    // Words of width 10, 20, 30, 40 separated by spaces of width 5.
    let mut advances = Vec::new();
    let mut acc = 0.0f32;
    let widths = [10.0f32, 20.0, 30.0, 40.0];
    for w in widths {
        acc += w;
        advances.push(acc);
        acc += 5.0; // trailing space, not part of any word
        advances.push(acc);
    }
    let run = TextRun {
        advances,
        break_points: vec![1, 3, 5, 7],
        line_height: 16.0,
    };

    // Reference: greedy wrap.
    let reference = |width: f32| -> Vec<(usize, usize)> {
        let mut lines = Vec::new();
        let mut start = 0usize;
        loop {
            let mut last_fit = None;
            for &b in &run.break_points {
                if b <= start {
                    continue;
                }
                let base = if start == 0 {
                    0.0
                } else {
                    run.advances[start - 1]
                };
                let w = run.advances[b - 1] - base;
                if w <= width {
                    last_fit = Some(b);
                } else {
                    break;
                }
            }
            let e = last_fit.unwrap_or(run.advances.len());
            lines.push((start, e));
            if e >= run.advances.len() {
                break;
            }
            start = e;
        }
        lines
    };

    for width in [15.0f32, 25.0, 35.0, 45.0, 55.0, 200.0] {
        let got = run.line_ranges(width);
        let want = reference(width);
        assert_eq!(got, want, "wrap mismatch at width {width}");
    }
}

#[test]
fn circle_glyph_area_is_close_to_analytic() {
    let glyph = GlyphOutline::circle(Vec2::new(500.0, 500.0), 400.0, 128);
    let px = 100.0; // 100 px per em => radius 40 px
    let bitmap = glyph.rasterize(px, 8);
    let analytic = std::f64::consts::PI * 40.0 * 40.0;
    let measured = bitmap.ink_area();
    let rel = (measured - analytic).abs() / analytic;
    assert!(
        rel < 0.02,
        "circle area off by {rel:.3}: measured={measured} analytic={analytic}"
    );
}

#[test]
fn glyph_coverage_is_inside_or_outside() {
    let glyph = GlyphOutline::circle(Vec2::new(500.0, 500.0), 400.0, 128);
    assert_eq!(glyph.coverage_at(500.0, 500.0), 1.0);
    assert_eq!(glyph.coverage_at(0.0, 0.0), 0.0);
    assert_eq!(glyph.coverage_at(950.0, 500.0), 0.0);
}

#[test]
fn rounded_rect_is_antialiased() {
    let mut r = FrameBuffer::new(32, 32);
    let stats = r.paint_all(
        &[Paint::RoundedRect {
            rect: ui::layout::Rect::new(4.0, 4.0, 24.0, 24.0),
            radius: 8.0,
            color: Rgba::rgb(1.0, 1.0, 1.0),
        }],
        &[],
    );
    assert_eq!(stats.paints, 1);
    assert_eq!(stats.culled, 0);
    // The interior pixel is fully covered, the corner is not.
    let center = r.pixels[16 * 32 + 16] >> 24;
    let corner = r.pixels[5 * 32 + 5] >> 24;
    assert_eq!(center, 255, "interior should be opaque");
    assert!(corner < 255, "rounded corner should be partially covered");
}

#[test]
fn fully_occluded_opaque_paint_is_culled() {
    let mut r = FrameBuffer::new(256, 256);
    let stats = r.paint_all(
        &[
            // Back: a small opaque square in the middle, entirely hidden.
            Paint::Solid {
                rect: ui::layout::Rect::new(64.0, 64.0, 64.0, 64.0),
                color: Rgba::rgb(1.0, 0.0, 0.0),
            },
            // Front: an opaque sheet covering the whole frame.
            Paint::Solid {
                rect: ui::layout::Rect::new(0.0, 0.0, 256.0, 256.0),
                color: Rgba::rgb(0.0, 1.0, 0.0),
            },
        ],
        &[],
    );
    assert_eq!(stats.paints, 2);
    assert_eq!(stats.culled, 1, "the hidden square should be culled");
}

#[test]
fn translucent_paint_is_never_culled() {
    let mut r = FrameBuffer::new(256, 256);
    let stats = r.paint_all(
        &[
            Paint::Solid {
                rect: ui::layout::Rect::new(0.0, 0.0, 256.0, 256.0),
                color: Rgba::new(0.0, 0.0, 0.0, 0.5),
            },
            Paint::Solid {
                rect: ui::layout::Rect::new(0.0, 0.0, 256.0, 256.0),
                color: Rgba::new(1.0, 1.0, 1.0, 0.5),
            },
        ],
        &[],
    );
    assert_eq!(stats.culled, 0);
}

#[test]
fn blur_preserves_total_energy() {
    let w = 64usize;
    let h = 64usize;
    let mut data = vec![0u8; w * h * 4];
    // One bright 8x8 block in the middle.
    for y in 28..36 {
        for x in 28..36 {
            let i = (y * w + x) * 4;
            data[i] = 255;
            data[i + 3] = 255;
        }
    }
    let before: u64 = data.chunks_exact(4).map(|p| u64::from(p[3])).sum();
    ui::blur::box_blur_rgba8(&mut data, w, h, 6);
    let after: u64 = data.chunks_exact(4).map(|p| u64::from(p[3])).sum();
    let ratio = after as f64 / before as f64;
    assert!(
        (ratio - 1.0).abs() < 0.02,
        "box blur should conserve total alpha, ratio={ratio}"
    );
}
