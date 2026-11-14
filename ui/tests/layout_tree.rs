//! Tree mutation, layout integration, springs and the frame router.

#![allow(clippy::field_reassign_with_default)]

use ui::layout::{self, Align, EdgeInsets, FlexDirection, Justify, LayoutProps, Size};
use ui::scheduler::{FrameCost, Route, Scheduler};
use ui::spring::Spring;
use ui::tree::NodeTree;

#[test]
fn tree_mutation_is_o1_and_preserves_order() {
    let mut tree = NodeTree::new();
    let root = tree.root();
    let a = tree.alloc(LayoutProps::default());
    let b = tree.alloc(LayoutProps::default());
    let c = tree.alloc(LayoutProps::default());
    assert!(tree.insert_child(root, a, None));
    assert!(tree.insert_child(root, b, None));
    assert!(tree.insert_child(root, c, None));
    assert_eq!(tree.sibling_list(root), vec![a, b, c]);

    // Insert in the middle.
    let d = tree.alloc(LayoutProps::default());
    assert!(tree.insert_child(root, d, Some(1)));
    assert_eq!(tree.sibling_list(root), vec![a, d, b, c]);

    // Remove a middle node; the rest keeps its order.
    tree.remove(d);
    assert_eq!(tree.sibling_list(root), vec![a, b, c]);

    // The freed slot is reused.
    let e = tree.alloc(LayoutProps::default());
    assert_eq!(e, d);
}

#[test]
fn tree_rejects_cycles_and_double_attach() {
    let mut tree = NodeTree::new();
    let root = tree.root();
    let a = tree.alloc(LayoutProps::default());
    let b = tree.alloc(LayoutProps::default());
    assert!(tree.insert_child(root, a, None));
    assert!(tree.insert_child(a, b, None));
    // b is already attached.
    assert!(!tree.insert_child(root, b, None));
    // a cannot become a child of its own descendant.
    assert!(!tree.insert_child(b, a, None));
}

#[test]
fn snapshot_pads_with_identity_gaps() {
    let mut tree = NodeTree::new();
    let root = tree.root();
    for _ in 0..70 {
        let child = tree.alloc(LayoutProps::default());
        assert!(tree.insert_child(root, child, None));
    }
    let snap = tree.snapshot();
    assert_eq!(snap.node_count, 71);
    // One gap after every 32 elements.
    assert_eq!(snap.slot_count, 71 + 71 / 32);
    let gaps = snap.is_gap.iter().filter(|g| **g).count();
    assert_eq!(gaps, 71 / 32);
    // Gap slots carry the identity element.
    for (i, is_gap) in snap.is_gap.iter().enumerate() {
        if *is_gap {
            assert_eq!(snap.parent[i], u32::MAX);
            assert_eq!(snap.w[i], 0.0);
        }
    }
}

#[test]
fn layout_distributes_grow_space() {
    let mut tree = NodeTree::new();
    let root = tree.root();
    let mut root_props = LayoutProps::default();
    root_props.direction = FlexDirection::Row;
    root_props.gap = 0.0;
    root_props.width = Some(300.0);
    root_props.height = Some(100.0);
    if let Some(n) = tree.node_mut(root) {
        n.props = root_props;
    }

    let mut widths = Vec::new();
    for grow in [1.0f32, 2.0, 1.0] {
        let mut p = LayoutProps::default();
        p.grow = grow;
        p.basis = Some(20.0);
        let child = tree.alloc(p);
        assert!(tree.insert_child(root, child, None));
        widths.push(child);
    }
    layout::layout(&mut tree, Size::new(300.0, 100.0));

    let w0 = tree.node(widths[0]).unwrap().w;
    let w1 = tree.node(widths[1]).unwrap().w;
    let w2 = tree.node(widths[2]).unwrap().w;
    // 300 - 3*20 = 240 to distribute 1:2:1 => 60, 120, 60.
    assert!((w0 - 80.0).abs() < 1e-3, "w0={w0}");
    assert!((w1 - 140.0).abs() < 1e-3, "w1={w1}");
    assert!((w2 - 80.0).abs() < 1e-3, "w2={w2}");
}

#[test]
fn layout_honours_max_width_clamps() {
    let mut tree = NodeTree::new();
    let root = tree.root();
    let mut root_props = LayoutProps::default();
    root_props.direction = FlexDirection::Row;
    root_props.width = Some(300.0);
    root_props.height = Some(50.0);
    if let Some(n) = tree.node_mut(root) {
        n.props = root_props;
    }
    let mut p1 = LayoutProps::default();
    p1.grow = 1.0;
    p1.basis = Some(20.0);
    p1.max_w = Some(60.0);
    let a = tree.alloc(p1);
    assert!(tree.insert_child(root, a, None));

    let mut p2 = LayoutProps::default();
    p2.grow = 1.0;
    p2.basis = Some(20.0);
    let b = tree.alloc(p2);
    assert!(tree.insert_child(root, b, None));

    layout::layout(&mut tree, Size::new(300.0, 50.0));
    let wa = tree.node(a).unwrap().w;
    let wb = tree.node(b).unwrap().w;
    assert!(
        (wa - 60.0).abs() < 1e-3,
        "clamped child should be 60, got {wa}"
    );
    assert!(
        (wb - 240.0).abs() < 1e-3,
        "sibling absorbs the rest, got {wb}"
    );
}

#[test]
fn layout_applies_padding_gap_and_justify() {
    let mut tree = NodeTree::new();
    let root = tree.root();
    let mut root_props = LayoutProps::default();
    root_props.direction = FlexDirection::Row;
    root_props.padding = EdgeInsets::all(10.0);
    root_props.gap = 5.0;
    root_props.justify = Justify::End;
    root_props.align = Align::Start;
    root_props.width = Some(200.0);
    root_props.height = Some(100.0);
    if let Some(n) = tree.node_mut(root) {
        n.props = root_props;
    }
    let mut p = LayoutProps::default();
    p.width = Some(40.0);
    p.height = Some(20.0);
    let child = tree.alloc(p);
    assert!(tree.insert_child(root, child, None));

    layout::layout(&mut tree, Size::new(200.0, 100.0));
    let n = tree.node(child).unwrap();
    // Inner box is [10, 190]; justify=end puts the 40-wide child at x=150.
    assert!((n.x - 150.0).abs() < 1e-3, "x={}", n.x);
    assert!((n.y - 10.0).abs() < 1e-3, "y={}", n.y);
}

#[test]
fn spring_matches_numerical_integration() {
    let mut spring = Spring {
        value: 0.0,
        velocity: 0.0,
        target: 1.0,
        stiffness: 180.0,
        damping: 12.0,
        mass: 1.0,
        ..Spring::default()
    };
    // Closed-form single step of 0.2 s.
    let analytic = spring.step(0.2);

    // Euler integration with a tiny step, same initial conditions.
    let mut x = 0.0f32;
    let mut v = 0.0f32;
    let dt = 1e-4f32;
    for _ in 0..2000 {
        let a = (180.0 * (1.0 - x) - 12.0 * v) / 1.0;
        v += a * dt;
        x += v * dt;
    }
    let err = (analytic - x).abs();
    assert!(err < 2e-3, "analytic={analytic} numeric={x} err={err}");
}

#[test]
fn spring_settle_time_is_finite_and_accurate() {
    let mut spring = Spring {
        value: 0.0,
        velocity: 0.0,
        target: 1.0,
        stiffness: 180.0,
        damping: 20.0,
        mass: 1.0,
        ..Spring::default()
    };
    let t = spring.settle_time(0.001);
    assert!(t > 0.0 && t.is_finite(), "settle time = {t}");
    // Stepping past the predicted settle time must reach the rest threshold.
    let steps = (t / 0.001).ceil() as u32 + 10;
    for _ in 0..steps {
        spring.step(0.001);
    }
    assert!(
        spring.is_settled(),
        "value={} velocity={}",
        spring.value,
        spring.velocity
    );
}

#[test]
fn scheduler_routes_small_dirties_to_the_cpu() {
    let s = Scheduler::default();
    // A caret blink: a 2x16 region, one layer.
    assert_eq!(s.route(&FrameCost::area(2, 16, 1)), Route::CpuMicro);
    // A full-screen repaint at 1440p with 4 layers.
    assert_eq!(s.route(&FrameCost::area(2560, 1440, 4)), Route::GpuMacro);
}
