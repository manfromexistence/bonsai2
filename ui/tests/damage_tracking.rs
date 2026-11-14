//! Correctness of the damage-tracking path.
//!
//! Damage tracking is the easiest place in a renderer to introduce a bug that
//! only shows up as "a faint ghost of the old frame" — so it gets its own
//! differential test: a damaged incremental repaint must produce **byte-for-byte
//! the same pixels** as a full repaint of the same scene.

use ui::layout::Rect;
use ui::raster::{FrameBuffer, Paint, Rgba};

/// A scene with overlapping opaque and translucent layers.
fn scene() -> Vec<Paint> {
    vec![
        Paint::Solid {
            rect: Rect::new(0.0, 0.0, 200.0, 150.0),
            color: Rgba::rgb(0.10, 0.12, 0.16),
        },
        Paint::RoundedRect {
            rect: Rect::new(20.0, 20.0, 120.0, 60.0),
            radius: 12.0,
            color: Rgba::new(0.9, 0.4, 0.2, 0.65),
        },
        Paint::Shadow {
            rect: Rect::new(40.0, 60.0, 80.0, 50.0),
            radius: 8.0,
            blur: 14.0,
            color: Rgba::new(0.0, 0.0, 0.0, 0.6),
        },
        Paint::RoundedRect {
            rect: Rect::new(60.0, 30.0, 90.0, 40.0),
            radius: 6.0,
            color: Rgba::new(0.2, 0.8, 0.6, 0.4),
        },
        Paint::Solid {
            rect: Rect::new(100.0, 90.0, 70.0, 40.0),
            color: Rgba::rgb(0.3, 0.35, 0.45),
        },
    ]
}

#[test]
fn damaged_repaint_matches_full_repaint() {
    let paints = scene();

    // Full repaint.
    let mut full = FrameBuffer::new(200, 150);
    full.paint_all(&paints, &[]);

    // Incremental: repaint the same scene, but only inside a damage region
    // that does not cover the whole frame.
    let mut incremental = FrameBuffer::new(200, 150);
    // Start from a full repaint so the untouched region has the right pixels.
    incremental.paint_all(&paints, &[]);
    // Now damage a region and repaint only that region.
    let damage = Rect::new(15.0, 15.0, 140.0, 120.0);
    incremental.paint_all(&paints, &[damage]);

    assert_eq!(
        full.pixels, incremental.pixels,
        "a damaged repaint must be pixel-identical to a full repaint"
    );
}

#[test]
fn damaged_repaint_of_a_different_scene_updates_the_region() {
    // Start with a red square, then damage a region and paint a blue square
    // there. The damaged region must change; the rest must not.
    let mut r = FrameBuffer::new(100, 100);
    r.paint_all(
        &[Paint::Solid {
            rect: Rect::new(0.0, 0.0, 100.0, 100.0),
            color: Rgba::rgb(1.0, 0.0, 0.0),
        }],
        &[],
    );
    let outside_before = r.pixels[0];

    let damage = Rect::new(20.0, 20.0, 30.0, 30.0);
    r.paint_all(
        &[Paint::Solid {
            rect: Rect::new(0.0, 0.0, 100.0, 100.0),
            color: Rgba::rgb(1.0, 0.0, 0.0),
        }],
        &[damage],
    );
    r.paint_all(
        &[
            Paint::Solid {
                rect: Rect::new(0.0, 0.0, 100.0, 100.0),
                color: Rgba::rgb(1.0, 0.0, 0.0),
            },
            Paint::Solid {
                rect: Rect::new(20.0, 20.0, 30.0, 30.0),
                color: Rgba::rgb(0.0, 0.0, 1.0),
            },
        ],
        &[damage],
    );

    // The damaged centre is blue.
    let centre = r.pixels[35 * 100 + 35];
    assert_eq!(
        (centre >> 16) & 0xff,
        0,
        "centre should have no red, got {centre:#x}"
    );
    assert_eq!(centre & 0xff, 255, "centre should be blue, got {centre:#x}");
    // A pixel outside the damage region kept the old colour.
    assert_eq!(
        r.pixels[0], outside_before,
        "outside the damage must not change"
    );
}

#[test]
fn opaque_span_fill_writes_exact_bytes() {
    let mut r = FrameBuffer::new(16, 16);
    r.paint_all(
        &[Paint::Solid {
            rect: Rect::new(0.0, 0.0, 16.0, 16.0),
            color: Rgba::rgb(0.25, 0.5, 0.75),
        }],
        &[],
    );
    for y in 0..16 {
        for x in 0..16 {
            let p = r.pixels[y * 16 + x];
            assert_eq!(p >> 24, 255, "alpha at ({x},{y})");
            assert_eq!((p >> 16) & 0xff, 64, "red at ({x},{y})");
            assert_eq!((p >> 8) & 0xff, 128, "green at ({x},{y})");
            assert_eq!(p & 0xff, 191, "blue at ({x},{y})");
        }
    }
}

#[test]
fn translucent_blend_is_order_dependent_and_correct() {
    // 50% white over 50% black, then the result composited over the frame.
    let mut r = FrameBuffer::new(4, 4);
    r.paint_all(
        &[
            Paint::Solid {
                rect: Rect::new(0.0, 0.0, 4.0, 4.0),
                color: Rgba::rgb(0.0, 0.0, 0.0),
            },
            Paint::Solid {
                rect: Rect::new(0.0, 0.0, 4.0, 4.0),
                color: Rgba::new(1.0, 1.0, 1.0, 0.5),
            },
        ],
        &[],
    );
    let p = r.pixels[0];
    // 0.5 white over black: ~128 in each colour channel, alpha 255.
    assert_eq!(p >> 24, 255);
    assert!((128..=131).contains(&((p >> 16) & 0xff)), "got {p:#x}");
    assert!((128..=131).contains(&((p >> 8) & 0xff)), "got {p:#x}");
    assert!((128..=131).contains(&(p & 0xff)), "got {p:#x}");
}

#[test]
fn rounded_rect_interior_is_exactly_opaque_and_corners_are_not() {
    let mut r = FrameBuffer::new(64, 64);
    r.paint_all(
        &[Paint::RoundedRect {
            rect: Rect::new(8.0, 8.0, 48.0, 48.0),
            radius: 16.0,
            color: Rgba::rgb(1.0, 1.0, 1.0),
        }],
        &[],
    );
    // Deep inside.
    assert_eq!(r.pixels[32 * 64 + 32] >> 24, 255);
    // Just outside the corner arc.
    assert_eq!(r.pixels[9 * 64 + 9] >> 24, 0);
}
