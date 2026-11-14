# TODO — `ui`

**Status:** repo-managed + executed.
**Last updated:** 2026-11-14

Ordered by *how much the item would change the verdict in README.md §5*, not by
how easy it is. Each item states what it would prove or disprove.

---

## 0. Gate: nothing below matters until this is done

### [ ] 0.1 Compile the WGSL with `naga` and run it

**Why:** every GPU claim in this crate is currently unverified. The shaders are
source text; they have never been through a shader compiler, let alone a
driver. If `shaders/layout.wgsl` does not compile, the GPU story is a story.

**Acceptance:** `naga` parses all four `.wgsl` files without error; a compute
pass runs the water-filling solver and produces the same sizes as
`layout::solve_grow` for the randomised cases in `tests/flex_solver.rs`.

**Risk:** the current `tile_blend.wgsl` contains placeholder helpers
(`unpack_rect`, `premultiplied`, `blend_over` are stubs). It is a *design*
artefact, not an implementation.

**Cost:** days. Not weeks.

---

## 1. The verdict-changing items

### [ ] 1.1 Actually measure the GPU against the CPU

**Why:** the entire premise is "GPU layout is faster than CPU layout". Nothing
in this repo has tested that on any GPU. The CPU baseline is 493 ns per flex
solve (32 children) and 0.12 ms for a 1 000-node layout. If GPU submission
latency and the prefix-scan overhead exceed those numbers, the GPU layout path
is a loss and the whole plan changes.

**Acceptance:** same workload, same machine, both paths. Report CPU µs and GPU
µs per frame including submission and readback.

**Cost:** days, once 0.1 lands.

### [ ] 1.2 Write the benchmark that actually matters

The current benchmark measures the renderer in isolation. UI latency is
dominated by what you *do not* draw. A meaningful harness measures:

- time from input event to first changed pixel,
- fraction of the frame that was actually dirty,
- energy per idle minute (does the GPU stay asleep?).

**Cost:** days.

### [ ] 1.3 Implement small-size hinting, or admit the atlas is needed

**Why:** this is the one place where the atlas argument actually wins. Slug
and Windfoil give exact geometric coverage of *unhinted* outlines. At 11–13 px
a 0.7 px stem straddles two pixel centres and renders as two muddy ~35%
alpha pixels. Real engines solve this with phase snapping plus stem
darkening; without it, atlas-free text is visibly worse than a hinted atlas.

**Options:**

1. **Phase snapping + stem darkening** (the CoreText approach): snap the
   baseline and dominant stem axis to the nearest half-pixel; widen the
   coverage integral by `Δw = k·(1 − luminance)`. Keeps "no atlas".
2. **Hybrid**: analytic for large/animated text, a small tile-local L2 cache
   for static small text — which is what the ecosystem converged on anyway.
3. **Accept the atlas** for text below ~14 px and say so.

**Acceptance:** a side-by-side screenshot comparison at 11/12/13/14/18 px
against a hinted atlas, judged by a human, not by a metric.

**Cost:** option 1 is a week; option 2 is a few days; option 3 is an hour and
an honest README paragraph.

---

## 2. Correctness gaps in what exists

### [ ] 2.1 Cache the flattened outline

`text::GlyphOutline::rasterize` re-flattens the outline on every call. That is
why the glyph number in README.md §4 is 10 Mpx/s. Flatten once, cache the
segments and the band index, and re-flatten only when the outline changes.

### [ ] 2.2 SIMD the rasterizer

72.9 Mpx/s for a solid fill is scalar Rust. A 4x16 strip rasterizer with
`u8x16` blending should reach several hundred Mpx/s. Until then, every CPU/GPU
comparison is contaminated by the CPU being slow.

### [ ] 2.3 Remove the allocation from layout

`NodeTree::sibling_list` allocates a `Vec` per node per measure pass, and
`measure` is called once per child during arrange — so the pass is
`O(n · depth)` in work and `O(n)` in allocations. Use a flat children array
with a sibling count, or pre-order indexing.

### [ ] 2.4 Write the missing flexbox

Documented as absent in README.md §6, listed here so it is not forgotten:
grid, `position`, `float`, writing modes, margin collapsing, `flex-basis`
percentages against an indefinite container, `min-content`/`max-content`
intrinsic sizing as a first-class query, `aspect-ratio`.

### [ ] 2.5 Text shaping

`TextRun` takes pre-shaped advances. There is no HarfBuzz/Parley integration, no
bidi, no ligatures, no script/itemisation. Any real use needs it.

### [ ] 2.6 Backdrop blur as a real barrier

`blur.rs` proves the *allocation* and *nesting* problems go away with a summed
area table. It does not implement the *dependency* problem: a blurred region
needs its background rendered first. The GPU path needs an explicit two-pass
structure with a barrier, and the CPU path needs the dirty region expanded to
include the blur kernel's support.

### [ ] 2.7 Make the culler handle partial occlusion

`OcclusionMask` works at 16 px granularity, so a shape that is 90% hidden but
covers 4 pixels of a 16 px block is not culled at all. A per-scanline exact
coverage mask (or a 4 px granularity) would cull more and cost more memory.

### [ ] 2.8 Finish the academic prior-art sweep for GPU layout

**Why:** PLAN.md §2 claims no shipping engine computes flex layout on the GPU.
That claim is *verified for the shipping engines checked* (GPUI, Rune, Vello,
Xilem) but the **academic** sweep — SIGGRAPH, Eurographics, PACMPL HPG, PPoPP,
IEEE TVCG, CHI — was not completed. If someone published a parallel flexbox
solver, the novelty claim in PLAN.md §2 has to be rewritten.

**Acceptance:** a documented search across those venues, with hits and misses
recorded, and PLAN.md §2 updated to match whatever is found.

**Cost:** half a day.

---

## 3. Structural work

### [ ] 3.1 Decide the ownership model

`ui` currently lives inside the `G:\Dx` hub repository and is tracked by the
hub's `.gitignore` allow-list. Every other product in `G:\Dx` is an
independent repository with its own origin (`essence-dx/*`). If `ui` is going
to be a product rather than a hub artefact, it needs its own repository, its
own `AGENTS.md`, and its own register row in `G:\Dx\docs\README.md`.

### [ ] 3.2 Register the crate in the hub manifest

`G:\Dx\dx` has a `paths` block and `G:\Dx\cli\src\update_registry.rs` has a
`TOOL_REGISTRY`. `ui` is a library, not a distributed binary, so it may not
belong in the registry — but the hub's `docs/` register should say where it
lives and what it has achieved.

### [ ] 3.3 Differential-test against a real engine

The flex solver is differentially tested against a CSS reference *algorithm*
written here. That proves the solver is self-consistent, not that it matches
Chromium's layout. The real test is: render the same tree with `yoga`
(CSS flexbox in Rust), Taffy, and `ui`, and compare geometry pixel for pixel.
That would also expose the subset gaps in §2.4.

---

## 4. Explicitly not planned

These were proposed in the source conversation and are **not** going in, with
reasons, so they are not re-litigated:

| Proposal | Why not |
| --- | --- |
| 100% GPU-resident engine, 0% CPU during animation | Input still arrives from the OS. The CPU never leaves the loop, it just stops *ticking*. |
| "Every pixel written to VRAM exactly once" | Impossible through translucent layers and across backdrop-blur barriers. |
| Work graphs for the frame loop | Not in WebGPU; `dispatchWorkgroupsIndirect` is the portable subset and does not give shader-spawned producers. |
| Co-dfns / parent-vector trees as a *dependency* | The flat parent-vector representation is useful and is used here; the APL lineage is not. The interesting part was the analytic clamp solver, and that does not need it. |
| A `dx-` crate prefix | The crate is `ui`. |
