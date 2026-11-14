// Shared structures for the `ui` compute pipeline.
//
// STATUS: repo-managed source text. This file has NOT been compiled by naga,
// tint or a driver in this repository (the crate has no GPU dependency on
// purpose). Treat it as a reviewed design artefact.

struct GpuNode {
    parent : u32,
    depth : u32,
    flags : u32,
    reserved : u32,
    x : f32,
    y : f32,
    w : f32,
    h : f32,
};

struct GpuLayoutParams {
    node_count : u32,
    slot_count : u32,
    viewport_w : f32,
    viewport_h : f32,
    time : f32,
    gap_period : u32,
    _pad : vec2<f32>,
};

const NONE : u32 = 0xFFFFFFFFu;
const GAP_PERIOD : u32 = 32u;

// A gap slot carries the identity element of the layout semiring: no parent,
// zero extent. The prefix scan walks over it at full memory bandwidth without
// branching.
fn is_gap_slot(node : GpuNode) -> bool {
    return node.parent == NONE;
}

fn clamp_to_bounds(value : f32, lo : f32, hi : f32) -> f32 {
    return clamp(value, lo, hi);
}
