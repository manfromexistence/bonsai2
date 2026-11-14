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

### [x] 1.0 Re-prioritise: the rasterizer was 99.6% of the frame — **DONE**

**Measured on native Windows** (README.md §4.3): a 27.3 ms frame broke down as
**20 µs layout / 23 µs paint-list / 27 200 µs raster**. The GPU-layout work
targeted 0.07% of the frame.

**Resolved.** The rasterizer was rewritten around three ideas, none of which
required SIMD intrinsics:

1. an opaque paint is a **copy**, not a blend (pattern fill),
2. a rounded rectangle is **mostly a rectangle** (span split; the interior
   needs no distance function),
3. **damage regions** — a *list* of what changed, not a bounding box.

Frame time went **27.3 ms -> 0.43 ms** and CPU **101% -> 7.8%** of one core.
See README.md §4.1.

What this changes in the plan:

1. **The rasterizer is no longer the bottleneck** — but it is still 88% of the
   frame, and it is now *memory-bound* (see §2.2), not compute-bound.
2. **Still do not start the GPU layout port before the rasterizer is
   respectable.** Layout is now ~5% of a 0.43 ms frame; a GPU layout win
   against a 20 µs CPU layout is a win against almost nothing.
3. Keep the analytic flex solver — it is correct, tested, and it removes the
   *theoretical* blocker — but stop treating it as the performance story.

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

The native Windows host (`examples/win32.rs`) already works around this by
caching the bitmap on the `Engine` — which is exactly the evidence that the
*library API* is missing the capability, not the callers. The fix belongs in
`text.rs`: a `PreparedGlyph` that owns the flattened segments and the band
index, produced once per outline change.

### [ ] 2.2 SIMD the rasterizer — mostly done, and the remainder is memory-bound

The opaque fill went from **72.9 Mpx/s to 2 192 Mpx/s** (1080p), and the
whole native frame from 27.3 ms to 0.43 ms, without any SIMD intrinsics
(README.md §4.1).

**But the remainder is not ALU-bound.** Two working-set sizes:

| Buffer | Throughput | Write bandwidth |
| --- | --- | --- |
| 256x256 (L2-resident) | 5 390 Mpx/s | 21.6 GB/s |
| 1920x1080 (L3-resident) | 2 215 Mpx/s | 8.9 GB/s |

The 1080p fill is **memory-bound**, and `-C target-cpu=native` (AVX2) buys
only **+11%** on it (2 109 -> 2 342 Mpx/s). The L2-resident case at 0.82
cycles/px is still issuing roughly one `u32` store per cycle, so the fill loop
itself is *also* not vectorising.

**Two proposed fixes were tested; one worked, one was rejected.**

* **Worked: damage-track the presentation path.** The Win32 host now copies
  only the damaged rows into the DIB and blits only the damaged regions, so
  the bytes handed to the OS per frame dropped from 8.3 MB to ~1.6 MB. That
  moved CPU from 7.8-18.8% to **3.1-7.8%** of one core while animating.
* **Rejected: L2 sub-rect tiling.** Splitting damage regions above 256x256
  into cache-resident tiles was implemented and benchmarked. It lost in both
  cases: the single-paint 1080p fill went **2 215 -> 1 710 Mpx/s** and eight
  translucent layers went **91.8 -> 69.0 Mpx/s**. The L2-resident 5 390
  Mpx/s figure is a benchmark artefact (the same 262 KB written 2 000 times
  keeps its lines hot), not a property a one-pass fill can inherit. The code
  was removed; README.md 4.2 records the numbers.

**Sub-timers settle the order.** The native frame is now split per paint kind
(README.md 4.3): opaque solid fills **24-41 us**, rounded rects **5-20 us**,
**shadow 136-231 us**, glyph ~0 us, on a ~500 us frame.

Consequences:

1. **Per-layer damage is dead as a priority.** The opaque backdrop fill is
   ~30 us of a ~500 us frame; any span-subtraction machinery that costs more
   than that to run loses. The hypothesis that "the backdrop and the moved
   widget are both repainted" is the bottleneck was tested and the numbers
   say otherwise. (Mechanism A - overlapping damage rects - is closed by
   construction: the damage list is built disjoint and `paint_all`
   `debug_assert`s it.)
2. **The shadow is the target, and its easy half is done.** The erf falloff
   is now a 256-entry lookup table (the profile depends only on `sdf/blur`),
   and the middle band uses the purely horizontal distance
   `|px - cx| - hw` instead of the `sqrt`-based signed distance. Shadow:
   207-281 -> 136-231 us. What remains is the translucent blend per pixel.
3. **The translucent blend is the last big item, and it needs SIMD.** The
   8-layer benchmark runs it at 91.8 Mpx/s, roughly 48 cycles per pixel,
   against an opaque fill at ~2 cycles. That needs `unsafe` + `core::arch`
   (or LLVM being shown an explicitly chunked loop), which is a decision
   about the unsafe policy, not an algorithm. Until then the library stays
   unsafe-free and the frame is ~0.5 ms.

**Cost:** item 3 is about a week and needs the unsafe policy decided. Items
remaining under item 1 are closed.

### [ ] 2.3 Re-test on discrete GPU hardware

**Why:** the only machine available for measurement has an **integrated**
Radeon (Ryzen 5 5600G) — no dedicated VRAM, shared system RAM, higher dispatch
overhead. Every GPU claim made from that machine is pessimistic and must be
re-measured on discrete hardware before it is believed. A discrete GPU with
its own VRAM changes the fragment-bandwidth picture completely, which is
exactly the axis the rasterizer bottleneck lives on.

**Acceptance:** the native Windows host and the benchmark both run on a
discrete GPU, and README.md §4 carries numbers from both machines.

### [ ] 2.4 Measure input-to-pixel latency, not just frame time

**Why:** frame time is not latency. The number a user feels is *input event →
first changed pixel*, which includes the OS event queue, the compositor, and
the display's own pipeline delay. None of the engine's internal measurements
touch that.

**Acceptance:** a latency harness that timestamps a synthetic mouse event and
the first pixel change, with the compositor's contribution separated out where
the platform allows it.

### [ ] 2.5 Remove the allocation from layout

`NodeTree::sibling_list` allocates a `Vec` per node per measure pass, and
`measure` is called once per child during arrange — so the pass is
`O(n · depth)` in work and `O(n)` in allocations. Use a flat children array
with a sibling count, or pre-order indexing.

### [ ] 2.6 Write the missing flexbox

Documented as absent in README.md §6, listed here so it is not forgotten:
grid, `position`, `float`, writing modes, margin collapsing, `flex-basis`
percentages against an indefinite container, `min-content`/`max-content`
intrinsic sizing as a first-class query, `aspect-ratio`.

### [ ] 2.7 Text shaping

`TextRun` takes pre-shaped advances. There is no HarfBuzz/Parley integration, no
bidi, no ligatures, no script/itemisation. Any real use needs it.

### [ ] 2.8 Backdrop blur as a real barrier

`blur.rs` proves the *allocation* and *nesting* problems go away with a summed
area table. It does not implement the *dependency* problem: a blurred region
needs its background rendered first. The GPU path needs an explicit two-pass
structure with a barrier, and the CPU path needs the dirty region expanded to
include the blur kernel's support.

### [ ] 2.9 Make the culler handle partial occlusion
`OcclusionMask` works at 16 px granularity, so a shape that is 90% hidden but
covers 4 pixels of a 16 px block is not culled at all. A per-scanline exact
coverage mask (or a 4 px granularity) would cull more and cost more memory.

### [ ] 2.10 Finish the academic prior-art sweep for GPU layout

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
That would also expose the subset gaps in §2.6.

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
