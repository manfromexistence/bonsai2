// Flex layout: the analytic water-filling distribution.
//
// This is the GPU mirror of `ui::layout::solve_grow` / `solve_shrink`.
//
// The insight: CSS's freeze-and-redistribute loop is not a fundamental
// requirement, it is an iterative way of solving a water-filling (simplex
// projection) problem. The problem has a closed-form solution: sort the
// saturation breakpoints and sweep. Because a container's children fit in a
// single warp in the overwhelming majority of real UIs, the sort can be a
// workgroup-local odd-even transposition sort with no global memory traffic.
//
// STATUS: repo-managed source text. NOT compiled by naga/tint in this
// repository.

const MAX_CHILDREN : u32 = 64u;

struct ChildSizes {
    // Hypothetical main size (already clamped into [min, max]).
    base : array<f32, 64>,
    // Flex grow factor.
    grow : array<f32, 64>,
    // Minimum main size.
    min_size : array<f32, 64>,
    // Maximum main size (a large sentinel means unbounded).
    max_size : array<f32, 64>,
    // Resolved main size, written back.
    out : array<f32, 64>,
};

var<workgroup> w_base : array<f32, 64>;
var<workgroup> w_grow : array<f32, 64>;
var<workgroup> w_min : array<f32, 64>;
var<workgroup> w_max : array<f32, 64>;
var<workgroup> w_tau : array<f32, 64>;
var<workgroup> w_resolved : array<f32, 64>;

// Odd-even transposition sort of the saturation thresholds.
// `w_tau[i]` holds the threshold of child `i`; we sort the (tau, index) pairs
// by carrying the index in a parallel array.
var<workgroup> w_order : array<u32, 64>;

@compute @workgroup_size(64)
fn solve_flex_grow(
    @builtin(local_invocation_id) lid : vec3<u32>,
    @builtin(storage_buffer, read) sizes : ChildSizes,
    @builtin(storage_buffer, read_write) results : ChildSizes,
) {
    let i = lid.x;
    let n = u32(results.count);
    if (i >= n) {
        return;
    }

    w_base[i] = sizes.base[i];
    w_grow[i] = sizes.grow[i];
    w_min[i] = sizes.min_size[i];
    w_max[i] = sizes.max_size[i];

    // Threshold at which this child saturates at its maximum.
    var tau : f32 = 3.4e38;
    if (w_grow[i] > 0.0 && w_max[i] < 3.4e38) {
        tau = (w_max[i] - w_base[i]) / w_grow[i];
    }
    w_tau[i] = tau;
    w_order[i] = i;
    workgroupBarrier();

    // Odd-even transposition sort, uniform trip count.
    let iters = (n + 1u) / 2u;
    for (var t : u32 = 0u; t < iters; t = t + 1u) {
        let left = t * 2u + (i & 1u);
        let right = left + 1u;
        if (right < n) {
            let a = w_tau[left];
            let b = w_tau[right];
            if (a > b) {
                w_tau[left] = b;
                w_tau[right] = a;
                let oa = w_order[left];
                let ob = w_order[right];
                w_order[left] = ob;
                w_order[right] = oa;
            }
        }
        workgroupBarrier();
    }

    // Sweep: freeze every child whose threshold is below the current level.
    var lambda : f32 = 0.0;
    var frozen_extra : f32 = 0.0;
    var remaining_grow : f32 = 0.0;
    for (var k : u32 = 0u; k < n; k = k + 1u) {
        remaining_grow = remaining_grow + w_grow[k];
    }

    var k : u32 = 0u;
    loop {
        if (k >= n) {
            break;
        }
        if (remaining_grow <= 0.0) {
            break;
        }
        let cand = (sizes.free_space - frozen_extra) / remaining_grow;
        let child = w_order[k];
        if (w_tau[k] < cand) {
            // This child saturates at its maximum.
            frozen_extra = frozen_extra + max(w_max[child] - w_base[child], 0.0);
            remaining_grow = remaining_grow - w_grow[child];
            k = k + 1u;
        } else {
            lambda = cand;
            break;
        }
    }
    if (remaining_grow > 0.0) {
        lambda = (sizes.free_space - frozen_extra) / remaining_grow;
    }
    workgroupBarrier();

    // Write back: frozen children take their maximum, the rest take the
    // proportional share.
    let frozen = w_tau[i] < lambda && w_grow[i] > 0.0;
    if (frozen) {
        w_resolved[i] = w_max[i];
    } else {
        w_resolved[i] = clamp_to_bounds(w_base[i] + lambda * w_grow[i], w_min[i], w_max[i]);
    }
    workgroupBarrier();

    if (i < n) {
        results.out[i] = w_resolved[i];
    }
}
