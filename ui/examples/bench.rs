//! Measured micro-benchmarks.
//!
//! Run with: `cargo run --release --example bench`
//!
//! Every number printed by this binary is measured on the machine that ran
//! it, single-threaded. They are the receipts behind the tables in README.md.

#![allow(clippy::field_reassign_with_default)]

use std::time::Instant;
use ui::blur::box_blur_rgba8;
use ui::layout::{self, FlexDirection, LayoutProps, Size};
use ui::raster::{Paint, Rasterizer, Rgba};
use ui::text::{GlyphOutline, Vec2};
use ui::tree::NodeTree;

fn build_tree(tree: &mut NodeTree, target: usize) {
    let root = tree.root();
    let mut frontier = vec![root];
    let mut count = 1usize;
    while count < target && !frontier.is_empty() {
        let mut next = Vec::new();
        for parent in frontier {
            for _ in 0..16 {
                if count >= target {
                    break;
                }
                let mut props = LayoutProps::default();
                props.direction = FlexDirection::Column;
                props.basis = Some(16.0);
                props.gap = 1.0;
                let child = tree.alloc(props);
                tree.insert_child(parent, child, None);
                next.push(child);
                count += 1;
            }
        }
        frontier = next;
    }
}

fn main() {
    println!("ui {} — measured micro-benchmarks", ui::VERSION);
    println!("single-threaded, release build\n");

    // --- layout -----------------------------------------------------------
    println!("layout (tree build + full pass, 1920x1080 viewport)");
    for n in [1_000usize, 10_000, 100_000] {
        let mut tree = NodeTree::new();
        build_tree(&mut tree, n);
        let t = Instant::now();
        layout::layout(&mut tree, Size::new(1920.0, 1080.0));
        let el = t.elapsed();
        let nodes = tree.len();
        println!(
            "  {nodes:>7} nodes   {:>9.3} ms   {:>10.0} nodes/s",
            el.as_secs_f64() * 1e3,
            nodes as f64 / el.as_secs_f64()
        );
    }

    // --- flex solver ------------------------------------------------------
    println!("\nflex water-filling solver (32 children, 100k iterations)");
    let base = vec![20.0f32; 32];
    let grow = vec![1.0f32; 32];
    let min = vec![0.0f32; 32];
    let max = vec![60.0f32; 32];
    let t = Instant::now();
    let iters = 100_000;
    let mut acc = 0.0f32;
    for _ in 0..iters {
        let out = layout::solve_grow(&base, &grow, &min, &max, 400.0);
        acc += out[0];
    }
    let el = t.elapsed();
    println!(
        "  {:>9.1} ns/solve   (checksum {acc:.1})",
        el.as_secs_f64() * 1e9 / iters as f64
    );

    // --- glyph rasterisation ---------------------------------------------
    println!("\natlas-free glyph rasterisation (16 px em, 8 sub-scanlines)");
    let glyph = GlyphOutline::circle(Vec2::new(500.0, 500.0), 400.0, 96);
    let iters = 20_000;
    let t = Instant::now();
    let mut px = 0u64;
    for _ in 0..iters {
        let bmp = glyph.rasterize(16.0, 8);
        px += (bmp.width * bmp.height) as u64;
    }
    let el = t.elapsed();
    println!(
        "  {:>9.2} Mpx/s   ({:.1} px/glyph)",
        px as f64 / el.as_secs_f64() / 1e6,
        px as f64 / iters as f64
    );

    // --- solid fill -------------------------------------------------------
    println!("\nopaque rectangle fill (1920x1080)");
    let mut r = Rasterizer::new(1920, 1080);
    let iters = 200;
    let t = Instant::now();
    for _ in 0..iters {
        r.paint_all(
            &[Paint::Solid {
                rect: ui::layout::Rect::new(0.0, 0.0, 1920.0, 1080.0),
                color: Rgba::rgb(0.2, 0.3, 0.4),
            }],
            &[],
        );
    }
    let el = t.elapsed();
    let total = (1920.0 * 1080.0) * iters as f64;
    println!(
        "  {:>9.2} Mpx/s   ({:.2} ms/frame)",
        total / el.as_secs_f64() / 1e6,
        el.as_secs_f64() * 1e3 / iters as f64
    );

    // --- fill: memory-bound or compute-bound? -------------------------------
    println!("\nopaque fill at two working-set sizes (is the fill memory-bound?)");
    for (w, h, label) in [
        (256usize, 256usize, "256x256  (256 KB, L2-resident)"),
        (1920, 1080, "1920x1080 (8.3 MB, L3-resident)"),
    ] {
        let mut r = Rasterizer::new(w, h);
        let iters = if w == 256 { 2000 } else { 200 };
        let t = Instant::now();
        for _ in 0..iters {
            r.paint_all(
                &[Paint::Solid {
                    rect: ui::layout::Rect::new(0.0, 0.0, w as f32, h as f32),
                    color: Rgba::rgb(0.2, 0.3, 0.4),
                }],
                &[],
            );
        }
        let el = t.elapsed();
        let total = (w * h) as f64 * iters as f64;
        println!(
            "  {label:<32} {:>8.2} Mpx/s   {:>7.2} GB/s written",
            total / el.as_secs_f64() / 1e6,
            total * 4.0 / el.as_secs_f64() / 1e9
        );
    }

    // --- occlusion culling ------------------------------------------------
    println!("\nspan-occlusion culling (64 stacked opaque rects, 1920x1080)");
    // Painter's order: the first paint is the backmost. The front-most is a
    // full-screen sheet, so everything behind it is genuinely hidden.
    let mut paints = Vec::new();
    for i in 0..64 {
        paints.push(Paint::Solid {
            rect: ui::layout::Rect::new(0.0, 0.0, 1920.0, 1920.0 - i as f32 * 16.0),
            color: Rgba::rgb(0.1, 0.2, 0.3),
        });
    }
    let mut r = Rasterizer::new(1920, 1080);
    let stats = r.paint_all(&paints, &[]);
    println!(
        "  painted {} of {} paints; {:.1}% of pixel-touches culled",
        stats.paints - stats.culled,
        stats.paints,
        100.0 * stats.culled_fraction()
    );

    // --- blur -------------------------------------------------------------
    println!("\nbox blur via summed-area table (1920x1080, radius 8)");
    let mut data = vec![0u8; 1920 * 1080 * 4];
    for (i, px) in data.chunks_exact_mut(4).enumerate() {
        let v = ((i % 1920) / 8) as u8;
        px[0] = v;
        px[1] = v;
        px[2] = v;
        px[3] = 255;
    }
    let iters = 20;
    let t = Instant::now();
    for _ in 0..iters {
        box_blur_rgba8(&mut data, 1920, 1080, 8);
    }
    let el = t.elapsed();
    let total = (1920.0 * 1080.0) * iters as f64;
    println!(
        "  {:>9.2} Mpx/s   ({:.2} ms/frame)",
        total / el.as_secs_f64() / 1e6,
        el.as_secs_f64() * 1e3 / iters as f64
    );
}
