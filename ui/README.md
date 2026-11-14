# `ui` — hybrid CPU/GPU UI rendering core

**Location:** `G:\Dx\ui` (Rust crate `ui`, zero external dependencies)
**Status:** repo-managed + executed. Not a shipped renderer, not release-qualified.
**Date of this document:** 2026-11-14

---

## 1. What this is

A crate that implements the *algorithmic core* of a proposed GPU-resident UI
engine, in plain Rust, so the claims can be checked instead of argued about.

It exists because a long design conversation kept asserting four things:

1. Flexbox `flex-grow`/`flex-shrink` with `min`/`max` clamps needs a sequential
   freeze-and-redistribute loop, so it "cannot" run on a GPU.
2. Glyph atlases are mandatory, because evaluating quadratic Béziers per pixel
   is slower than a bitmap blit.
3. Painter's-algorithm overdraw is why UI rendering leaves 10x–50x on the table.
4. A GPU-resident engine would be "the fastest possible" and "world-first".

This crate implements the strongest version of each claim, measures it, and
reports where it survives and where it does not.

**Short answer up front:** claims 1 and 2 are *false as stated* — the flex
clamp problem has an exact closed-form solution (implemented and differentially
tested here), and banded outline evaluation is a real, published technique
(Slug / Windfoil). Claims 3 and 4 are *half true at best* — opaque overdraw is
culled (86.8% of pixel-touches in the benchmark below), but translucent overdraw
and backdrop blur cannot be, and "fastest possible" is not a property a
renderer can have on its own. Details in §5.

---

## 2. Status, by tier

The hub's claim vocabulary (`G:\Dx\AGENTS.md` §4) is used strictly here.

| Statement | Tier | Evidence |
| --- | --- | --- |
| The crate builds, tests and benchmarks | **repo-managed + executed** | `cargo build`, `cargo test` (20/20 pass), `cargo run --release --example bench` |
| The flex water-filling solver equals the CSS reference algorithm | **executed** | `tests/flex_solver.rs`, 10 000 randomised cases, tolerance 1e-3 relative |
| The occlusion culler removes 86.8% of pixel-touches in the benchmark scene | **executed** | `examples/bench.rs`, release build, output quoted in §4 |
| The WGSL in `shaders/` is correct | **repo-managed only** | Source text. **Not compiled by naga/tint or any driver in this repo.** No GPU dependency exists here on purpose. |
| The GPU versions of these algorithms are faster than the CPU versions | **not established** | No GPU run has been performed. See TODO.md §1. |
| This is "the fastest possible" UI renderer | **rejected** | See §5. The claim is not well-posed. |

---

## 3. What is implemented

| Module | Claim under test | Verdict |
| --- | --- | --- |
| [`tree`](src/tree.rs) | Tree mutation must not force a re-upload of the whole tree | **Holds.** Insert/remove is O(1) with a free list; no array memmove, no bus traffic. The Euler-tour snapshot is rebuilt only on structural change and is padded with one identity gap every 32 slots so a GPU prefix scan walks it branch-free. |
| [`layout`](src/layout.rs) | "Flex clamping needs a sequential loop, so GPU layout is impossible" | **False.** `solve_grow` / `solve_shrink` solve the freeze-and-redistribute problem in closed form: the CSS loop is a water-filling (simplex projection) problem, which is continuous and piecewise-linear in the level `λ`, hence solvable by sorting the saturation breakpoints and sweeping once. Differentially tested against the CSS freeze loop. |
| [`layout`](src/layout.rs) | "Text wrapping is a circular dependency that breaks prefix-scan layout" | **Mostly false.** A cumulative advance table plus a binary search per line finds the break points; the dependency is not circular because wrapping depends only on the *container width*, not on sibling geometry. Implemented and tested against a greedy reference. |
| [`text`](src/text.rs) | "A glyph atlas is mandatory" | **Half false.** Atlas-free evaluation is real: TrueType-style quadratic outlines are flattened into horizontal **bands** (the idea behind Eric Lengyel's *Slug*), and a scanline only tests the curves that can cross it, with exact horizontal coverage. What is *not* solved here is small-size hinting — see §6. |
| [`raster`](src/raster.rs) | "Painter's-algorithm overdraw is the bottleneck" | **Half true.** Fully hidden **opaque** paints are culled before they touch the pixel pipeline, using a `u128` span mask per row (16 px granularity, one AND per block). Translucent paints are **never** culled — see §5.3. |
| [`blur`](src/blur.rs) | "Backdrop blur needs ping-pong targets, so nesting is capped" | **Holds as a critique; the fix works.** A box filter of any radius is O(1) per pixel from a summed-area table, so there are no render targets, no VRAM growth and no nesting cap. The *neighbouring-pixel dependency* remains and cannot be removed. |
| [`spring`](src/spring.rs) | "Animations must be ticked by the CPU" | **Mostly false.** A damped harmonic oscillator has a closed-form solution, so any consumer can evaluate it at an arbitrary timestamp. `Spring::settle_time` returns exactly when the animation will finish, which is what lets a scheduler *stop* instead of spin at 240 Hz. Verified against numerical integration. |
| [`scheduler`](src/scheduler.rs) | "The GPU is always faster than the CPU for UI" | **False as stated.** Waking a GPU compute pipeline costs tens of microseconds of submission latency; re-rasterising a small dirty rect on the CPU costs single-digit microseconds. The router makes this a per-frame decision instead of an architectural bet. |
| [`gpu`](src/gpu.rs) + [`shaders/`](shaders) | The GPU can consume these structures | **repo-managed only.** `repr(C)` layouts with compile-time size assertions, and WGSL that mirrors the CPU algorithms. **Not compiled.** |

---

## 4. Measured numbers

Reproduce with:

```
cd G:\Dx\ui
cargo test                                  # 20 tests
cargo run --release --example bench         # the table below
```

Measured on the development machine, single-threaded, release build. These are
*this crate's* numbers on a *reference implementation* — not a competitive
result, and not a GPU result.

| Measurement | Result | Note |
| --- | --- | --- |
| Layout, 1 000 nodes | 0.123 ms | allocation-heavy reference implementation |
| Layout, 10 000 nodes | 1.407 ms | |
| Layout, 100 000 nodes | 16.4 ms | |
| Flex water-filling solve (32 children) | 493 ns | one sort + one sweep, no redistribution rounds |
| Atlas-free glyph raster (16 px em, 8 sub-scanlines) | 10.0 Mpx/s | re-flattens the outline every call; not cached |
| Opaque fill, 1920x1080 | 72.9 Mpx/s (28.4 ms/frame) | scalar Rust, no SIMD |
| Span-occlusion culling (64 stacked opaque rects) | 12 of 64 painted, **86.8% of pixel-touches culled** | |
| Box blur, 1920x1080, radius 8 | 37.5 Mpx/s (55.3 ms/frame) | O(1) per pixel regardless of radius |

**Read these honestly.** The CPU numbers are a *baseline*, not a tuned engine:
Taffy, Skia and Vello all beat a straightforward Rust layout pass, and this
crate's rasterizer has no SIMD. That is the point of the measurement. A typical
application UI is ~1 000 nodes, where layout costs ~0.12 ms — well under a
frame. **Layout is not the bottleneck that the GPU work was aimed at.**

---

## 5. The brutal check

### 5.1 Are the four "breakthroughs" real?

| # | Claim | Verdict |
| --- | --- | --- |
| 1 | GPU prefix-scan flex layout is impossible because of clamp freezing | **The obstruction is real; the conclusion is wrong.** The freeze loop is an implementation choice, not a property of the problem. Closed-form solution implemented and differentially tested. |
| 2 | An atlas is mandatory | **The performance objection is real; the conclusion is wrong.** Banded evaluation is the published answer (Slug, Windfoil). The unresolved part is *hinting*, not atlases. |
| 3 | Painter's overdraw is 10x–50x of wasted silicon | **Directionally true for opaque layers, false as a blanket claim.** Real UI overdraw is dominated by *translucent* layers (shadows, glass, blur), which cannot be culled at all. |
| 4 | A GPU-driven loop removes the CPU from the frame | **Half true.** Animation *evaluation* can leave the CPU (done here). *Event delivery* cannot: input arrives from the OS, and on Windows/macOS the compositor is not fully GPU-driven. |

### 5.2 Is "world-first" defensible?

**No — not as a conception, and only possibly as a shipment.**

*Not novel as a technique:* banded Bézier evaluation is published (Eric Lengyel,
*Slug*, JCGT 2021; Matt DesLauriers, `texel-org/windfoil-algorithm`);
tile-based GPU rasterization with occlusion culling is standard practice
(Firefox WebRender's `WebRenderTiles`, Skia Ganesh/Graphite, Chrome's
occlusion tracking); analytic springs and GPU-driven frame loops are
mainstream in 3D.

*Not novel as a product:* `anomalyco/rune` already ships Slug as a fallback for
"big, rotating or scaling text", uses `vello_gpu` sparse strips for complex
vectors, and has a CPU reference renderer for differential testing. It does
*not* compute layout on the GPU and does *not* use an atlas-free default text
path — so the full combination is genuinely unclaimed by a shipping engine.

*Provably wrong in the source conversation:* the claim that Slug "recently
merged into HarfBuzz as hb-gpu" is **false**. There is no `harfbuzz/hb-gpu`;
the real repository is `hartleyblakey/hb-gpu`, a third-party, zero-star project
that is not part of HarfBuzz upstream. The claim that Vello blends front-to-back
is **false**; Vello blends back-to-front. The claim that
`dispatchWorkgroupsIndirect` gives "95% of Work Graphs" is **overstated** —
it gives dynamic workgroup *counts*, not shader-spawned producers or dynamic
amplification. Work graphs are indeed **not** exposed by WebGPU.

*One claim that checks out:* Eric Lengyel dedicated the Slug patent to the
public domain on **2026-03-17**. This is recorded in `anomalyco/rune`'s own
`PLAN.md`. (Secondary source — the register entry itself should be confirmed
before shipping.)

### 5.3 What is genuinely unfixable in the original plan

The original plan asserted a single-pass front-to-back blend where "every pixel
on a 4K/8K screen is written to VRAM exactly once". That is **impossible**:

* A translucent layer's result depends on what is behind it, so a
  front-to-back walk cannot stop early through it.
* A backdrop blur additionally depends on *neighbouring* pixels, so it is a
  barrier no single-pass early exit can cross — the tile needs the background
  rendered and filtered first.

The honest version is: **opaque tiles write once; translucent tiles are a
barrier; blurred tiles are a second pass.** This crate implements the opaque
case and marks the barrier; it does not pretend to solve the barrier.

### 5.4 Does this make us "the fastest possible"?

No, and the question is not well-posed as asked. UI latency is dominated by
things a renderer cannot control:

1. **Not redrawing what did not change.** Dirty-rect reuse dominates every
   modern engine; a perfect rasterizer that redraws the whole frame every
   frame is slower than a mediocre one that does not.
2. **Compositor and present latency** (DWM, CoreAnimation), plus input latency
   through the OS event queue.
3. **Wake-up cost.** A GPU frame costs tens of microseconds before the first
   pixel moves. For a caret blink or a hover highlight, the CPU is faster.

What a renderer *can* do — avoid hidden work, avoid waking the GPU when
nothing moves, keep the dirty set small — is exactly what this crate
implements on the CPU side, and what the (uncompiled) shaders are meant to
implement on the GPU side.

---

## 6. What this crate is **not**

* **Not a renderer.** It produces a framebuffer, not a window. There is no
  windowing, no event loop, no GPU device, no display.
* **Not a text engine.** No shaping, no bidi, no ligatures, no font loading.
  `TextRun` takes already-shaped advances. The glyph outline type exists to
  make the coverage algorithm testable, not to be a production font stack.
* **Not a CSS engine.** A documented subset of flexbox: row/column, grow,
  shrink, basis, min/max, width/height, padding, gap, justify, align. No
  grid, no float, no position, no writing modes, no margin collapsing. See
  the module docs in [`layout`](src/layout.rs).
* **Not GPU-resident.** There is no GPU code path that runs. The WGSL is
  uncompiled source.
* **Not faster than the incumbents.** The CPU numbers in §4 are a baseline.
  Beating GPUI, Rune or Vello requires an apples-to-apples comparison against
  *their* implementations, which has not been done.

---

## 7. Repository layout

```
G:\Dx\ui\
  Cargo.toml            crate `ui`, zero dependencies
  README.md             this file
  TODO.md               open work, ordered, with honest difficulty
  PLAN.md               the design, the research ledger, and the decision gates
  src/
    lib.rs              crate root and re-exports
    tree.rs             flat parent-vector tree, free list, gap-padded Euler snapshot
    layout.rs           flex solver (analytic water-filling), wrapping, tree layout
    text.rs             banded glyph outlines, scanline coverage, cumulative advances
    raster.rs           SDF primitives, u128 span-occlusion culler, framebuffer
    blur.rs             summed-area-table box blur
    spring.rs           closed-form damped harmonic oscillator
    scheduler.rs        hardware-adaptive frame router
    gpu.rs              repr(C) GPU layouts + WGSL includes
  shaders/
    common.wgsl         shared structs
    layout.wgsl         GPU water-filling solver
    tile_blend.wgsl     front-to-back tile blending with early exit
    glyph.wgsl          banded glyph coverage
  tests/
    flex_solver.rs      analytic vs CSS reference, 10 000 randomised cases
    layout_tree.rs      tree mutation, layout integration, springs, routing
    text_and_raster.rs  wrapping, coverage, culling, blur
  examples/
    bench.rs            the measurements in §4
```

---

## 8. Build and run

```bash
cd G:\Dx\ui
cargo build --release
cargo test                       # 20 tests, all passing
cargo clippy --all-targets       # clean
cargo run --release --example bench
```

Requires Rust 1.85+ (edition 2024). No network access, no vendored
dependencies, no GPU.

---

## 9. Where to read next

* [`TODO.md`](TODO.md) — what is not done, ordered by how much it would change
  the verdict.
* [`PLAN.md`](PLAN.md) — the architecture that survives contact with silicon,
  the prior-art matrix, the source ledger, and the decision gates.
