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
| [`raster`](src/raster.rs) | "Painter's-algorithm overdraw is the bottleneck" | **Half true.** Fully hidden **opaque** paints are culled before they touch the pixel pipeline, using a `u128` span mask per row (16 px granularity, one AND per block). Translucent paints are **never** culled — see §5.3. The renderer also does **span fills** (an opaque paint is a copy, not a blend), **span-split rounded rects** (the interior needs no distance function), **integer branchless blending**, and **damage-region tracking**. Together: 27.3 ms -> 0.43 ms per frame, measured. |
| [`blur`](src/blur.rs) | "Backdrop blur needs ping-pong targets, so nesting is capped" | **Holds as a critique; the fix works.** A box filter of any radius is O(1) per pixel from a summed-area table, so there are no render targets, no VRAM growth and no nesting cap. The *neighbouring-pixel dependency* remains and cannot be removed. |
| [`spring`](src/spring.rs) | "Animations must be ticked by the CPU" | **Mostly false.** A damped harmonic oscillator has a closed-form solution, so any consumer can evaluate it at an arbitrary timestamp. `Spring::settle_time` returns exactly when the animation will finish, which is what lets a scheduler *stop* instead of spin at 240 Hz. Verified against numerical integration. |
| [`scheduler`](src/scheduler.rs) | "The GPU is always faster than the CPU for UI" | **False as stated.** Waking a GPU compute pipeline costs tens of microseconds of submission latency; re-rasterising a small dirty rect on the CPU costs single-digit microseconds. The router makes this a per-frame decision instead of an architectural bet. |
| [`gpu`](src/gpu.rs) + [`shaders/`](shaders) | The GPU can consume these structures | **repo-managed only.** `repr(C)` layouts with compile-time size assertions, and WGSL that mirrors the CPU algorithms. **Not compiled.** |

---

## 4. Measured numbers

Reproduce with:

```
cd G:\Dx\ui
cargo test                                  # 25 tests
cargo run --release --example bench         # the tables below
```

Measured on the development machine, single-threaded, release build.

### 4.1 The optimisation that mattered

The first version of this crate blended **every pixel of every paint** through
the full source-over formula - four `f32::from(u8)` conversions, four divides
by 255, four float multiply-adds and four `round().clamp().as u8` **per
channel, per pixel**, with a branch in the middle. On ~2 million pixel-touches
a frame that cost **58 clock cycles per pixel touched** on a core that retires
six macro-ops a cycle.

Three changes fixed it, and **none of them was SIMD intrinsics**:

1. **An opaque paint is a copy, not a blend.** Source-over with `a == 1`
   degenerates to `out = src`, so opaque spans became a pattern fill. That
   alone took the 1080p fill from **72.9 Mpx/s to 2 192 Mpx/s**.
2. **A rounded rectangle is mostly a rectangle.** Its interior is at least
   `radius` away from any edge, so those pixels need no distance function.
   Only the corner bands and the two partial pixels per row do.
3. **Damage tracking.** A *region list* (not a bounding box) of what actually
   changed, so an animating card repaints ~10% of the frame instead of all of
   it.

The pixel format moved to `u32` (`0xAARRGGBB`, premultiplied) along the way.
That removed the per-channel unpacking, made the translucent blend a
branchless integer expression using the exact divide-by-255 identity
(`div255(x) = (x + 128 + ((x + 128) >> 8)) >> 8`), and - because the byte
order in memory is already `B, G, R, A` - let the Win32 host blit the
framebuffer into a `BI_RGB` DIB with **zero conversion**.

### 4.2 Offline micro-benchmarks

| Measurement | Before | After | Change |
| --- | --- | --- | --- |
| Layout, 1 000 nodes | 0.123 ms | 0.118 ms | - |
| Layout, 100 000 nodes | 16.4 ms | 15.2 ms | - |
| Flex water-filling solve (32 children) | 493 ns | 426 ns | - |
| Atlas-free glyph raster (16 px em) | 10.0 Mpx/s | 10.7 Mpx/s | - |
| **Opaque fill, 1920x1080** | **72.9 Mpx/s** (27.7 ms) | **2 192 Mpx/s** (0.95 ms) | **30x** |
| Span-occlusion culling (64 stacked rects) | 86.8% of touches culled | 86.8% | - |
| Box blur, 1920x1080, radius 8 | 37.5 Mpx/s | 41.9 Mpx/s | - |

**Is the fill memory-bound or compute-bound?** Two working-set sizes:

| Buffer | Throughput | Write bandwidth |
| --- | --- | --- |
| 256x256 (256 KB, L2-resident) | 5 390 Mpx/s | 21.6 GB/s |
| 1920x1080 (8.3 MB, L3-resident) | 2 215 Mpx/s | 8.9 GB/s |

The 2.4x gap says the 1080p fill is **memory-bound**, not compute-bound. And
`-C target-cpu=native` (AVX2) buys only **+11%** on the fill (2 109 -> 2 342
Mpx/s), because the remaining cost is bytes written, not ALU. So the next
optimisation is **fewer bytes**, not wider registers - which is exactly what
damage tracking does.

### 4.3 Native Windows host - real window, real compositor

`examples/win32.rs` (raw Win32 FFI, no crates) opens a real window, renders a
synthetic application-chrome scene - sidebar, 18 cards, translucent rounded
rects, a drop shadow, an atlas-free glyph, a HUD - and reports its own CPU and
memory footprint once a second.

```
cargo run --release --example win32          # interactive: the card follows the mouse
cargo run --release --example win32 demo     # continuous animation, no mouse needed
```

**Test machine:** AMD Ryzen 5 5600G (6 cores / 12 threads, **integrated** Radeon
graphics - *no discrete GPU*), Windows 11 Pro build 26300, 23.3 GB RAM.

| | Before | After |
| --- | --- | --- |
| Frames rendered | 33-34 / s | 38-41 / s |
| Frame time | 27.3 ms | **425-533 us** (**~58x**) |
| Raster stage | 27 200 us | **356-470 us** (**~62x**) |
| Layout stage | 20 us | 20 us |
| Paint-list build | 23 us | 28-45 us |
| Pixel-touches per frame | ~2.03 M | **~190-218 k** (**~10x fewer**) |
| CPU, animating | **101% of one core** | **7.8-18.8% of one core** |
| CPU, idle | 0.00 s/s | 0.00 s/s (zero frames drawn) |
| Working set | 21.9 MB | 22.1 MB (peak 26.2 MB) |
| Private bytes | 10.5 MB | 10.5 MB |
| Threads / handles | 5 / 183 | 5 / 183 |

**Where the frame goes now:**

| Stage | Time | Share of frame |
| --- | --- | --- |
| Layout (~70 nodes) | 20-37 us | ~5% |
| Build the paint list | 28-45 us | ~7% |
| Rasterize + blend | 356-470 us | **~88%** |

Layout has gone from 0.07% of the frame to about 5% - because the frame
itself got 58x smaller. It is still not the thing worth optimising, but the
gap has closed enough that it is no longer irrelevant.

**The idle-gating is real, not asserted.** When the spring settles, the window
draws **zero frames** and the process consumes **0.00 s of CPU per second** -
the timer still ticks at 60 Hz but never invalidates a region.

### 4.4 Compared against the other apps on this OS

Six-second CPU sample across every process, taken while the engine was
animating:

| Application | Working set | CPU (% of one core) |
| --- | --- | --- |
| node | 1 061 MB | 77.6% |
| WorkBuddyAI - helper | 296 MB | 69.5% |
| WorkBuddyAI - IDE, main | 1 844 MB | 48.4% |
| WorkBuddyAI - helper | 109 MB | 13.8% |
| PowerShell - this sampler | 103 MB | 12.0% |
| **`ui` engine (win32.exe), animating** | **22.1 MB** | **7.8%** |
| `opencode` | 210 MB | 1.0% |
| `zen` / `opencode` / terminal | 70-600 MB | 0.3-3.9% each |

**Before this optimisation the same table read:** the engine was the **heaviest
CPU consumer on the machine** at 101% of one core - 4.6x the IDE's main
process - while also being the **lightest** in memory at 21.9 MB.

**The honest reading of that table now:**

* **Memory: still the lightest thing measured.** 22.1 MB working set against
  1 844 MB for the IDE - about **83x smaller**. It does not appear in the
  machine's top RAM consumers at all; the smallest of those is ~70 MB.
* **CPU: from heaviest to seventh.** 7.8% of one core while animating at 40
  fps, and 0.00% when idle.
* **What actually bought that:** not AVX2, not the GPU, not the layout solver.
  An opaque fast path, span splitting, and damage regions.

One caveat on this machine: the 5600G's GPU is **integrated**, so it has no
dedicated VRAM and shares system RAM. That makes it a *pessimistic* test bed
for the GPU-resident thesis - dispatch overhead is higher and there is no
fast-path memory. Any GPU conclusion must be re-tested on discrete hardware
before it is believed. See TODO.md 2.3.

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
    damage_tracking.rs  a damaged repaint must be pixel-identical to a full one
  examples/
    bench.rs            the measurements in §4
    win32.rs            native Win32 window: real compositor, real input,
                        self-reported CPU/RAM (§4.1, §4.2)
```

---

## 8. Build and run

```bash
cd G:\Dx\ui
cargo build --release
cargo test                       # 25 tests, all passing
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
