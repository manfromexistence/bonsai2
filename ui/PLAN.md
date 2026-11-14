# PLAN — `ui`

**Status:** repo-managed + executed.
**Last updated:** 2026-11-14

This document has four parts:

1. [The architecture that survives contact with silicon](#1-the-architecture-that-survives-contact-with-silicon)
2. [Prior art: what is actually new](#2-prior-art-what-is-actually-new)
3. [Research ledger: every claim, checked](#3-research-ledger-every-claim-checked)
4. [Milestones and decision gates](#4-milestones-and-decision-gates)

---

## 1. The architecture that survives contact with silicon

The source conversation proposed a **100% GPU-resident** engine. The evidence
says that is the wrong target, and the strongest evidence is not a benchmark —
it is that the team that built the best pure-GPU 2D renderer in existence
**stopped shipping it**.

Linebender demoted the compute-centric Vello renderer into `research/` and
shipped `vello_cpu` + `vello_gpu` on a shared **Sparse Strips** representation
instead. `vello_gpu` can be configured to run *without* compute shaders
(WebGL2 + wgpu). The reason given publicly is the usual one: pure-GPU compute
has a fixed per-dispatch overhead, and that overhead is larger than the entire
job for the small dirty regions that dominate real interaction.

So the plan is not "GPU or CPU". It is:

```
                     ┌───────────────────────────────┐
   input event ────► │  router (scheduler.rs)        │
                     │  cost = dirtyPx·depth +       │
                     │         animatedCurves·64     │
                     └───┬───────────────────────┬───┘
                    cost │                       │ cost
                    < T  │                       │ >= T
                         ▼                       ▼
              ┌──────────────────┐    ┌────────────────────────┐
              │ CPU micro-patch  │    │ GPU macro-frame        │
              │ small dirty rect │    │ full tile dispatch     │
              │ SIMD, L1/L2      │    │ indirect dispatch      │
              └──────────────────┘    └────────────────────────┘
                         │                       │
                         └───────────┬───────────┘
                                     ▼
                          composited surface
```

**The four stages of the GPU path** (the parts that are actually new, mapped to
the modules that implement their CPU twins):

| Stage | What it does | CPU twin here | Novelty |
| --- | --- | --- | --- |
| 1. Input + springs | Reads the mapped input ring buffer, evaluates closed-form springs at the current timestamp, marks dirty nodes | `spring.rs` | Low — analytic springs are standard; the *closed form* is the useful part |
| 2. Layout | Solves the flex tree with the analytic water-filling distribution, one container per workgroup | `layout.rs::solve_grow` | **Medium-high** — no production engine does GPU flex layout; the *algorithm* is here and tested |
| 3. Tile binning + blend | Bins primitives into 16x16 tiles, blends front-to-back in registers, exits early on opaque pixels | `raster.rs` | **Low** — this is what WebRender, Ganesh and Vello already do |
| 4. Text | Evaluates banded quadratic outlines directly, no atlas | `text.rs` | **Medium** — the technique is published; the *shipped default* is not |

**What survives from the original plan:** stage 2 (the analytic solver) and
stage 4 (the banded evaluation, once hinting is solved). **What does not:**
the "every pixel written exactly once" promise, and the "0% CPU" promise.

**The load-bearing constraint** the original plan missed: a GPU frame has a
fixed wake-up cost. For a caret blink or a hover highlight, a CPU micro-patch
is faster end-to-end even though the CPU is nominally slower per pixel. The
router's threshold is therefore a *product* decision (what feels responsive),
not a hardware constant, and it should be tuned on the target machine.

---

## 2. Prior art: what is actually new

Pillars, as they were framed in the source conversation:

* **P1** — GPU compute layout via a parent-vector tree and parallel prefix scan
* **P2** — atlas-free GPU analytic/vector text
* **P3** — compute-shader tile binning, front-to-back blending, opaque early exit
* **P4** — GPU-driven frame loop with analytic time evaluation

| Project | P1 | P2 | P3 | P4 | Notes |
| --- | --- | --- | --- | --- | --- |
| Zed `gpui` | no | no (CPU atlas) | no (instanced SDF quads) | no | CPU Taffy layout, CPU glyph atlas |
| `anomalyco/rune` | no (CPU Taffy) | **partial** — Slug only for "big, rotating or scaling text"; otherwise CPU Parley + atlas | no (uber-quad SDF pass) | partial — analytic springs on a render thread | The closest existing engine to the proposal |
| Linebender `vello` | no (CPU Taffy/Parley) | no (CPU scene encoding) | **yes** — compute prefix-sum tiles, but **back-to-front**, no opaque early exit | no | The pure-compute renderer was retired to `research/` |
| `vello_gpu` (current) | no | no | partial — sparse strips, can run **without** compute shaders | no | The shipped split is CPU + GPU, deliberately |
| HarfBuzz upstream | no | no | no | no | **No GPU work exists upstream.** The `hb-gpu` in circulation is a third-party zero-star repo |
| `texel-org/windfoil-algorithm` | no | **yes** — winding-integral box filter | no | no | Slug vs Windfoil split: Slug wins when minified, Windfoil when magnified |
| `diffusionstudio/slug-webgpu` | no | **yes** — Slug in WebGPU | no | no | Third-party |
| WebRender / Ganesh / Chrome | no | no | **yes** — tile-based, occlusion-culled | partial (Chrome tiles the compositor) | P3 is standard GPU practice |
| `longbridge/nsh` + `gpui-kit` | no | no | no | no | Developer-velocity tooling: TSX, Tailwind, Shadcn-style, wasm build target |
| **`ui` (this repo)** | **algorithm implemented, CPU** | **algorithm implemented, CPU** | **opaque culling implemented, CPU** | **closed-form springs + router** | GPU twins exist as **uncompiled** WGSL |

### The verdict

* **P3 is not novel.** Tile-based GPU rasterization with occlusion culling is
  standard practice in browsers and Skia. It is good engineering; it is not a
  contribution.
* **P2 is not novel as a technique** (Slug 2021, Windfoil 2025-ish), but the
  *default text path* of every shipping engine still uses a CPU-rasterised
  atlas. Adopting it wholesale is still an open opportunity.
* **P1 is genuinely unexplored.** No *production* UI engine was found that
  computes flex layout on the GPU — GPUI and Rune both lay out on the CPU
  (Taffy). An academic sweep for parallel-layout literature was **not
  completed** in this session, so the precise claim is "no shipping engine,
  and no paper found in the searches that were run", not "nobody has ever
  done this". The usual reason given for the gap — "the clamp-freeze loop is
  sequential" — **turns out to be wrong**; it has an exact closed form,
  implemented and differentially tested in `layout.rs`. This is the most
  defensible piece of the original proposal.
* **P4 is half novel.** Analytic time evaluation is real and useful. Shader-spawned
  producers are not available in WebGPU.

**So: "first to conceive" — no. "First to ship the full combination" — plausibly
yes, and the crate contains the pieces that made the combination credible in
the first place.** The defensible claim is narrow: *the flex-clamp problem is
analytically solvable, and here is the proof and the differential test.*

---

## 3. Research ledger: every claim, checked

Tiers: **VERIFIED** (primary source read), **PARTLY TRUE**, **CONTRADICTED**,
**NOT FOUND**. Sources were read during the 2026-11-14 session.

### 3.1 The four "breakthroughs"

| Claim | Verdict | Source |
| --- | --- | --- |
| Eric Lengyel's Slug algorithm entered the public domain in March 2026 | **VERIFIED** (2026-03-17) via Rune's own `PLAN.md`; the register entry itself still needs primary confirmation | `github.com/anomalyco/rune/blob/dev/PLAN.md` |
| Slug was "recently merged into HarfBuzz as hb-gpu" | **CONTRADICTED** — there is no `harfbuzz/hb-gpu`; the real repo is `hartleyblakey/hb-gpu`, a third-party zero-star project | `github.com/harfbuzz/harfbuzz`, `github.com/hartleyblakey/hb-gpu` |
| Vello blends front-to-back with opaque early exit | **CONTRADICTED** — Vello blends back-to-front; the compute-centric renderer was demoted to `research/` | `github.com/linebender/vello`, `.../ARCHITECTURE.md` |
| `dispatchWorkgroupsIndirect` gives "95% of Work Graphs" | **OVERSTATED** — it gives dynamic workgroup counts, not shader-spawned producers or dynamic amplification | WGSL/WebGPU compute spec |
| Work graphs are absent from WebGPU | **VERIFIED** | WebGPU spec status |
| Packed Memory Arrays (gapped dynamic arrays) are a real data structure | **VERIFIED** (Braun et al.) — but see below | Braun,"Mehlhorn et al., packed memory arrays |
| Aaron Hsu's Co-dfns uses flat parent-vector trees on the GPU | **VERIFIED** for APL/parsing; **NOT FOUND** for CSS layout | `Co-dfns/co-dfns` |

### 3.2 The ecosystem claims

| Claim | Verdict |
| --- | --- |
| `anomalyco/rune` is real and is OpenCode's Rust UI engine | **VERIFIED** — `github.com/anomalyco/rune` |
| Rune uses CPU Taffy layout | **VERIFIED** — from its `PLAN.md` |
| Rune uses CPU Parley/HarfRust/skrifa/fontique with a GPUI-style two-tier line cache and a greyscale glyph atlas | **VERIFIED** — from its `PLAN.md` |
| Rune uses Slug only as a fallback for "big, rotating or scaling text and icons" | **VERIFIED** — from its `PLAN.md` |
| Rune has a CPU reference renderer for differential testing | **VERIFIED** |
| Rune's `AGENTS.md` claims "An idle window draws nothing" | **VERIFIED** (from the earlier reading of the repo) |
| `texel-org/windfoil-algorithm` is real | **VERIFIED** — winding-integral box filter by Matt DesLauriers; compares against Slug and splits by minified vs magnified |
| `diffusionstudio/slug-webgpu` is real | **VERIFIED** — third-party Slug-in-WebGPU |
| `longbridge/nsh` (Native Shell) is real: TSX, Tailwind, `--target gpui`, `--target web`, `createPlayer` | **VERIFIED** — `github.com/longbridge/nsh` |
| `longbridge/gpui-kit` (ex `gpui-component`), `gpui-base`, `gpui-shell` are real | **VERIFIED** |
| `zed-industries/embedded_gpui` is real | **VERIFIED** |
| `ppmpreetham/zopra` is real | **VERIFIED** — reactive wrapper with a `view!` RSX-style macro |
| GPUI has upstream wasm32 browser rendering | **NOT FOUND as a shipped target** — no upstream browser target was confirmed; treat browser GPUI as experimental |
| "rgx by zeron" | **NOT FOUND** — no such project could be located; treat as fabricated |

### 3.3 What this changes in the plan

1. **Do not cite Vello's tile binning as novel.** Cite it as prior art and
   differentiate on P1 and P2.
2. **Do not cite `hb-gpu` as HarfBuzz work.** It is a third-party repo.
3. **Do not promise single-write pixels.** It is impossible through
   translucent layers.
4. **Do promise the analytic flex solver**, with the differential test as the
   receipt.
5. **Do promise a hybrid router**, because the strongest argument for it is
   that the best pure-GPU renderer in existence was retired in favour of a
   CPU/GPU split.

---

## 4. Milestones and decision gates

Each milestone ends in a **gate**: a measurement that can kill the plan.

### M0 — Compile the shaders *(gate: they parse)*

Compile `shaders/*.wgsl` with `naga`. Implement `premultiplied`,
`unpack_rect`, `blend_over` properly, and a real half-float unpacker.

> **Gate:** all four files compile; the layout shader reproduces
> `solve_grow` on the randomised cases from `tests/flex_solver.rs`.
> **Kill criterion:** if the GPU solver cannot match the CPU solver on 10 000
> random cases, the shader is wrong and everything downstream is fiction.

### M1 — GPU layout, measured *(gate: it must beat the CPU)*

Port the layout stage to compute. Measure the same tree on both paths,
including submission and readback.

> **Gate:** on the target machine, GPU layout of a 1 000-node tree must beat
> 0.123 ms including all overhead.
> **Kill criterion:** if it does not, the GPU layout path is dead and the
> project should be re-scoped as a CPU engine with a better rasterizer.
> **Note:** a 1 000-node tree is the *realistic* case. Beating it is harder
> than beating a 100 000-node tree, and the 100 000-node number is the one
> that would make a good slide and a bad engine.

### M2 — Text: hinting, or the atlas *(gate: a human judges pixels)*

Implement phase snapping and stem darkening, or implement the hybrid cache, or
accept the atlas for small text.

> **Gate:** at 11/12/13/14 px, a human prefers the result over a hinted atlas.
> **Kill criterion:** if not, adopt the atlas for small text and keep the
> analytic path for scaled/animated text. Say so in the README.

### M3 — Tile rasterizer with the translucent barrier *(gate: correct pixels)*

Implement the opaque early-exit path, and implement the explicit two-pass
structure for backdrop blur.

> **Gate:** pixel-exact against the CPU reference rasterizer for scenes with
> shadows, translucency and blur.
> **Kill criterion:** any divergence means the barrier model is wrong.

### M4 — The router on real hardware *(gate: measured latency)*

Build the end-to-end harness: input event to first changed pixel, idle energy.

> **Gate:** on the target laptop, the router's threshold is derived from
> measurement, and the idle state draws nothing.
> **Kill criterion:** if the GPU path never wins in any realistic interaction,
> the router will always choose the CPU and the GPU work is a research
> artefact. That is still a legitimate outcome — but say it plainly.

**Partial result already in hand** (`examples/win32.rs`, README.md §4.1):

* The idle half of this gate is **met**: when the spring settles, the window
  renders **zero frames** and the process burns **0.00 s of CPU per second**.
  The router's "do nothing" path works.
* The measurement half is **not** met, and it surfaced something that reorders
  the plan: the frame is **27.3 ms of which 99.6% is rasterization** and
  **0.07% is layout**. The GPU layout path targets 20 µs. So the threshold
  cannot be derived until the rasterizer is respectable (§TODO 1.0).
* The test machine has **integrated** graphics only. Every GPU number taken
  from it is pessimistic and must be re-taken on discrete hardware.

### M5 — Differential test against Yoga/Taffy

Render the same tree with `ui`, `yoga` and `taffy`; compare geometry.

> **Gate:** agreement on the supported subset, and an explicit, published list
> of where they disagree and why.

---

## 5. The one-paragraph version

The original proposal was 70% right about the problems and 30% right about the
solutions. The problems are real — the CPU *is* in the way of flex layout, the
GPU *is* the right place for it, and the clamp-freeze loop *is* the thing that
made it look impossible. The solutions were mostly over-claimed: the clamp
problem has an exact closed form (now implemented and differentially tested),
tile binning and occlusion culling are prior art (WebRender, Ganesh, Vello),
atlas-free text needs hinting work that nobody has finished for small sizes,
"write every pixel once" is impossible through translucent layers, and the
purest test of the whole idea is that Linebender shipped a CPU/GPU split
instead of a pure-GPU renderer. The defensible, narrow, useful claim is: *the
GPU-layout blocker was solvable, here is the proof, and here is the engine
that uses it* — with a hybrid router, because the best pure-GPU 2D renderer in
existence was retired for exactly that reason.
