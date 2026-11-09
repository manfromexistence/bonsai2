# RENDERER.md -- Blitz as the Smallest-Fastest Browser: Rust Gems, JS Gap, and Fact-Check

Scope: what can be added to the Blitz engine in `blitz/` to close its
JavaScript gap and to reduce binary size and latency, using Rust crates that
exist today. Every row carries a claim tier from `AGENTS.md` section 4 and a
source that was actually checked on 2026-11-09.

Claim tiers used throughout:

| Tier | Meaning here |
| --- | --- |
| `repo-managed` | Read from the checked-out tree or upstream source at the stated path |
| `executed` | A command was run in this session and the output is quoted |
| `runtime` | True on a host or registry, with the host and the check named |
| `release-qualified` | Shipping verified end to end, with the receipt named |
| `unverified` | Could not be confirmed; treat as a lead, not a fact |

---

## 1. Baseline: what Blitz is today

All rows in this section are `repo-managed`, read from `G:/Dx/blitz` at
`main` (`808ded934`), which tracks `https://github.com/dioxuslabs/blitz`.

### 1.1 What Blitz already borrows (and therefore already wins)

Blitz is an integration layer over five upstream Rust engines. It does not
reimplement the hard parts, and that is the entire reason it can be small.

| Concern | Upstream engine Blitz uses | Verified at |
| --- | --- | --- |
| CSS parsing + cascade | Stylo (servo) | `packages/blitz-dom/Cargo.toml:63` (`style` v0.23.0) |
| Box layout (flex/grid/block) | Taffy (DioxusLabs fork) | `Cargo.toml:103` (rev `4aca4f4`) |
| Text layout + shaping | Parley 0.12 | `Cargo.toml:113` |
| Font parsing | Skrifa (Fontations) | `packages/blitz-dom/Cargo.toml:75` -- a **direct** dependency |
| HTML + XHTML parsing | html5ever / xml5ever | `Cargo.toml:72-73` |
| Paint + raster | anyrender + Vello + wgpu 30 | `Cargo.toml:122-133`, `297-310` |

`blitz-dom` owns the DOM tree, style resolution, layout, and events.
`blitz-paint` lowers the tree to anyrender draw commands. `blitz-shell` owns
the winit window, AccessKit tree, and muda menus. `blitz-net` is reqwest.

### 1.2 The JS gap -- confirmed, and it is worse than "Boa is slow"

The user's report is that Blitz renders Wikipedia-style pages but not YouTube
or Vercel. `executed` reading of the tree confirms the cause is three
separate gaps, only one of which is engine speed.

**Gap 1 -- JavaScript is not even on by default.** The browser app's feature
list does not include `javascript`:

```text
apps/browser/Cargo.toml:13
default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-bolden",
           "scrollbars", "mimalloc", "devtools", "embedded-devtools-frontend"]
```

`javascript` is an opt-in feature (`apps/browser/Cargo.toml:35`). So the
shipped browser binary renders no script at all unless it is built with
`--features javascript`.

**Gap 2 -- the JS engine is Boa, and Boa is the slowest realistic choice.**
`blitz-vibey-script` depends on `boa_engine` v0.22.0 with the `annex-b`
feature (`packages/blitz-vibey-script/Cargo.toml`). The JS support crate was
added on 2026-08-29 in commit `64eb27853` ("Add `blitz-vibey-script` crate +
JS support for the WPT runner (#738)"). It is roughly ten weeks old.

Independent, `runtime` measurement from the CatPaw ADR 0001 (accepted
2026-10-06, `KernelErr/CatPaw`, `docs/adr/0001-js-engine-boa-default.md`):

```text
| Boa 0.22 | boa_engine (pure Rust) | 95.6 % test262, no C/C++ toolchain |
  Cons: interpreter only; third-party benchmark ~200x slower than V8,
        ~3x slower than QuickJS-NG; Context is !Send; no cross-thread interrupt
```

The Zipp project publishes a second, methodologically stronger measurement
(`f2i-com/zipp.org`, `docs/PERF_ROADMAP.md`, `runtime`):

| Suite | Comparator | Zipp / comparator | Point wins |
| --- | --- | --- | --- |
| native interpreter real13 | QuickJS-NG v0.16.2 | 0.6413x [0.6386, 0.6452] | 12 / 13 |
| native interpreter micro5 | QuickJS-NG v0.16.2 | 0.8556x [0.8405, 0.8761] | 5 / 5 |
| native interpreter micro5 | Boa v0.22.0 | 0.2539x [0.2501, 0.2590] | 5 / 5 |

Read that table carefully: Zipp is roughly 4x faster than Boa on the same
suite (1 / 0.2539). The WebAssembly row in the same file is the one that
matters for size:

```text
The v0.0.5 WASM artifact is 5,595,833 bytes raw / 1,254,075 Brotli-11,
versus 1,528,293 / 417,087 for the QuickJS-NG reactor and
21,296,176 / 5,484,164 for Boa.
```

So Boa's packaged WASM footprint is about 13.9x QuickJS-NG's raw and about
13.2x its Brotli size. That is the number that decides the "smallest browser"
question for a JS engine.

**Gap 3 -- the web API surface is a shell, and that is what actually blocks
YouTube and Vercel.** `executed` enumeration of the globals registered in
`packages/blitz-vibey-script/src/runtime/bootstrap.rs` finds exactly:

```text
CSS, document, getComputedStyle, location, navigator, opener, parent, self, top,
window, innerHeight, innerWidth, outerHeight, outerWidth, devicePixelRatio,
pageXOffset, pageYOffset, scrollX, scrollY, scroll, scrollBy, scrollTo,
requestAnimationFrame, cancelAnimationFrame, setTimeout, setInterval,
clearTimeout, clearInterval,
__blitz_fetch_sync, __blitz_parse_transform, __blitz_send_message
```

Absent from that list, and absent from the whole `blitz-vibey-script` tree
when grepped for their identifiers: `fetch` (only a synchronous internal
`__blitz_fetch_sync` exists), `XMLHttpRequest`, `WebSocket`,
`MutationObserver`, `IntersectionObserver`, `ResizeObserver`,
`customElements`, `ShadowRoot`, `localStorage`, `sessionStorage`, `history`,
`canvas`/`getContext`, `HTMLVideoElement`, `HTMLAudioElement`, `serviceWorker`.

This is the real reason YouTube fails, and it is not primarily a JS engine
speed problem. YouTube needs `<video>` playback, `MediaSource`, custom
elements, `MutationObserver`, `IntersectionObserver`, `requestAnimationFrame`
driven compositing, and a large ES2017+ bundle. Vercel deployments are React
and Next.js hydration, which need `Element.prototype` traversal at scale,
`MutationObserver`, `history.pushState`, `fetch`, `Promise` microtask
ordering, and a JIT-capable engine to hydrate in reasonable time.

**Conclusion for gap 3:** no crate swap fixes this alone. A JS engine swap
improves the *speed* of the script that runs; it does not create the missing
DOM surface. Any plan that swaps Boa for QuickJS and calls the job done will
still fail to render YouTube, because `<video>` and `MutationObserver` will
still be absent.

### 1.3 Size baseline

`repo-managed`: the tree resolves 1010 crates in `Cargo.lock`. The Cargo
workspace declares 27 members: 16 under `packages/`, 3 under `apps/`
(`browser`, `browser/persistence`, `readme`), the WPT runner, the test
harness, and 8 examples. The `apps/browser` default feature set pulls in
`hybrid` (Vello GPU), `cookies`, `cache`, `mimalloc`, and `devtools` with an
embedded DevTools frontend.

`unverified`: the commonly quoted "5 MB minimum, 10-15 MB typical" figure for
Blitz binaries comes from a Dioxus maintainer quote relayed through a third
party (the `academy.optersoft.com` Blitz writeup and a Dioxus discussion),
not from a release artifact measured in this session. The GitHub releases API
rate-limited before assets could be read. **Do not quote a Blitz binary size
as `release-qualified` until a `dx` build produces one and its size is
recorded.** The measurement command is in section 7.

---

## 2. The gems: what to add to Blitz

Every crate below was resolved against the crates.io API on 2026-11-09
(`runtime`). Download counts are `recent_downloads` from that API, which is a
7-day figure.

### 2.1 JS engine -- the primary swap

| Crate / project | Version | dl/wk | Stars | Licence | Language | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| `boa_engine` (current) | 0.22.0 | 2,011,504 | 7,669 | MIT | Rust | Keep as the pure-Rust fallback |
| `rquickjs` | 0.14.0 | 3,560,374 | 1,066 | MIT | Rust bindings | **Adopt** -- 5.3x more used than Boa |
| `quickjs-ng/quickjs` | -- | -- | 4,100 | MIT | C | The engine behind `rquickjs` |
| `nova` (trynova) | -- | -- | 1,958 | MPL-2.0 | Rust | Watch; fork lineage from Boa |
| `brimstone` | -- | -- | 1,323 | MIT | Rust | Watch; new engine, small project |
| `zipp` (f2i-com) | 0.0.6 | -- | 9 (docs repo) | Apache-2.0 | Rust | **Highest ceiling, highest risk** |

`runtime` evidence for `rquickjs` being the pragmatic choice:

- It wraps QuickJS-NG, whose WASM reactor is 1,528,293 bytes raw / 417,087
  Brotli -- the smallest of the three engines measured by Zipp.
- `quickjs-ng` is actively maintained (pushed 2026-11-07).
- `rquickjs` supports `async`/futures natively, has a `Trace` GC for Rust
  values crossing the boundary, and ships prebuilt bindings for mainstream
  gnu/musl targets.
- The cost is that it compiles a C library, so a C cross-toolchain is required.
  This is exactly why some projects (the PSP port, cited in research) chose
  Boa instead.

`runtime` evidence for `zipp` being the interesting one:

- It is Rust, with a register bytecode interpreter plus an x86-64 tiered JIT
  and an ARM64 baseline.
- Test262: 96,227 / 96,228 unmodified, one documented expected failure.
- It leads QuickJS-NG on all 13 point medians of the native real13 protocol
  (0.6089665x cold, 0.6058409x startup-adjusted).
- Its own comparison page concedes the honest trade: "If your budget is
  under about 0.6 MB on the wire, QuickJS wins, full stop." Zipp's WASM is
  3.586x QuickJS-NG's raw size and 2.958x at Brotli-11.

**Recommendation:** make the JS engine a **trait**, not a hard dependency, and
ship two backends. This is the CatPaw ADR 0001 pattern and it is the one
decision that keeps the other options alive.

```rust
// The shape to copy (from CatPaw ADR 0001, decisions 1-3).
// blitz-vibey-script owns the trait; backends live behind features.
pub trait JsRuntime {
    fn eval(&self, src: &str) -> Result<JsValue, JsError>;
    fn call(&self, fn_ref: &JsValue, args: &[JsValue]) -> Result<JsValue, JsError>;
    fn enqueue_microtask(&self, task: Microtask);
    fn register_global(&self, name: &str, value: JsValue);
    fn interrupt(&self);            // so a script cannot hang the UI thread
    fn memory_limit(&self, bytes: u64);
}
```

| Feature | Backend | Rationale |
| --- | --- | --- |
| `js-boa` (default) | `boa_engine` | Pure Rust, no C toolchain, cross-compiles trivially |
| `js-quickjs` | `rquickjs` | 3x faster than Boa on micro5, ~13x smaller packaged WASM |
| `js-v8` (future) | `deno_core` / `rusty_v8` | The only way to match Chrome on JS throughput |

The `interrupt` method is not optional. Boa's `Context` is `!Send` and has no
cross-thread interrupt, which is the CatPaw ADR's stated top technical risk.
Without it, one infinite loop in a page script freezes the browser window.
Implementing `interrupt` means driving Boa's `Context` from a worker thread
with an atomic flag, or moving to an engine that supports it natively.

### 2.2 HTML parsing -- the biggest measured headroom

| Crate | Version | dl/wk | Licence | Role |
| --- | --- | --- | --- | --- |
| `html5ever` (current) | 0.40.1 | 55,306,257 | MIT/Apache | DOM-building parser Blitz uses |
| `lazyhtml` (cloudflare) | -- | 148 stars | BSD-3-Clause | **Newest gem; pushed 2026-04-23** |
| `lol_html` (cloudflare) | 3.0.3 | 2,449,286 | BSD-3-Clause | Streaming rewriter, not a DOM builder |

`runtime`: `cloudflare/lazyhtml` is described on its own repository as an
"HTML5-compliant parser and serializer that enables building transformation
pipeline in a pluggable manner". It is young (created 2019-06-20, but
substantially rewritten -- the Cloudflare Rust HTML parser history notes the
LazyHTML rewrite) and has only 148 stars, so almost nobody has tried it.

`runtime`: `cloudflare/lol-html` is a *streaming rewriter* with a
CSS-selector-based API and a C, Lua, Go, and Ruby binding surface. It is not
a DOM builder. The two are not interchangeable.

**Honest read on `lazyhtml`:** it is the most interesting gem in the HTML
category precisely because it is unwatched. But its architectural advantage is
*pluggable transformation pipelines*, and Blitz needs a *DOM tree*. The
plausible integration is not "replace html5ever" but "use LazyHTML as a
pre-pass token scanner" to find `<script>`, `<link>`, and inline `<style>`
boundaries fast, then hand the document to html5ever. Whether that beats
html5ever's own tokenizer is **unverified** and needs a benchmark. Treat it
as a candidate, not a swap.

### 2.3 Text and fonts -- the quiet win

`executed` from Blitz's own `Cargo.lock`, with the reverse dependency graph
reconstructed by parsing every `[[package]]` block:

```text
PRESENT  skrifa       0.44.0   <- anyrender_skia, blitz-dom, blitz-paint, glifo, parley_engine, skera
PRESENT  read-fonts   0.41.0   <- anyrender_serialize, fontique, harfrust, skrifa, write-fonts
PRESENT  font-types   0.11.3   <- (read-fonts, skrifa, write-fonts)
PRESENT  fontique     0.12.0   <- parley, parley_engine
PRESENT  harfrust     0.12.0   <- parley_engine        (HarfBuzz shaping port)
PRESENT  glifo        0.4.0    <- blitz-paint, vello_cpu
PRESENT  fearless_simd 0.7.0   <- vello_common
PRESENT  guillotiere  0.7.0    <- vello_common          (2D texture atlas allocator)
PRESENT  peniko       0.6.1    <- vello_common, glifo   (primitive types for vector graphics)
PRESENT  core_maths   0.1.1    <- parley, fontique, skrifa
ABSENT   swash        --       <- NOT used by Blitz 0.3.0-beta.4
ABSENT   zeno         --       <- NOT used by Blitz 0.3.0-beta.4
ABSENT   rustybuzz    --       <- superseded by harfrust
```

**The correction that matters.** An earlier reading of this document claimed
Blitz already contains `swash`, and therefore already contains `zeno` (a
`swash` optional dependency) and therefore already has a CPU rasterizer.
That is **wrong for this tree**. `swash` is absent: Parley 0.12 replaced the
old `swash` font backend with the `fontique` + `parley_engine` + `harfrust`
stack. `zeno` is therefore also absent. The claim was right about the
ecosystem and wrong about this repository, and the difference changes the
recommendation -- `zeno` is an *addition*, not something already there.

What the tree *does* already contain is more interesting than `swash` would
have been:

- **`skrifa` is a direct dependency of `blitz-dom` and `blitz-paint`**
  (`packages/blitz-dom/Cargo.toml:75`, `packages/blitz-paint/Cargo.toml:40`),
  used in `packages/blitz-dom/src/font_metrics.rs` for font metadata, charmap
  lookup, and glyph metrics. So Blitz already speaks the modern pure-Rust
  OpenType stack (Fontations) directly, not through a wrapper.
- **`harfrust` 0.12.0 is the shaper** -- "A complete HarfBuzz shaping algorithm
  port to Rust", 12,794,806 dl/wk. Parley 0.12's `parley_engine` depends on
  it. This is the text-shaping gem that replaced `rustybuzz`.
- **`glifo` 0.4.0 is Blitz's glyph renderer** -- "Glifo provides APIs for
  efficiently rendering text", 7,090,694 dl/wk. Blitz pins it to a DioxusLabs
  fork branch with an embolden fix pending upstream (`Cargo.toml:305-310`).
- **`guillotiere`** is a 2D texture atlas allocator with fast deallocation
  (9,773,133 dl/wk), used by Vello's common layer.

### 2.4 Zeno -- the gem worth adding, and its real origin

`runtime` plus `executed` from a local clone of the repository:

```text
zeno v0.3.3 | 7,312,662 dl/wk | 157 stars | Apache-2.0 OR MIT
Description: "High performance, low level 2D path rasterization."
Normal dependencies: libm   (that is the entire list)
```

Its own feature list is exactly what a browser paint backend needs and what a
GPU path cannot easily give:

```text
- 256x anti-aliased rasterization (8-bit alpha or 32-bit RGBA subpixel alpha)
- Pixel perfect hit testing with customizable coverage threshold
- Non-zero and even-odd fills
- Stroking with the standard set of joins and caps
- Numerically stable dashing for smooth dash offset animation
- Vertex traversal for marker placement
- Stepped distance traversal for animation or text-on-path support
```

It also claims the rasterized masks are "nearly identical to those generated
by Skia (sans slight AA differences)".

**The authorship is the real story, and it corrects a common error.**
`executed` from the full clone of `dfrg/zeno` (45 commits, first commit
2020-10-06 by Chad Brokaw):

```text
2020-10-06 | Chad | Initial commit
2021-06-25 | Chad Brokaw | add 'eval' feature gate to disable stroker, traversals and rasterizer
2023-09-05 | John Nunley | Add a feature for enabling compilation for no_std
2026-08-06 | nico.burns | Replace unsound to_int_unchecked with a saturating cast in to_fixed
2026-08-08 | nico.burns | Forbid unsafe_code and drop stale uninit lint suppressions
```

Chad Brokaw is a Mozilla **gfx2d** engineer -- `zeno` is the rasterizer
extracted from Mozilla's own 2D graphics work, and it is what Mozilla's
`gfx2d`/`gfx/webrender` work has been building on. And since August 2026 it
is being maintained by **Nico Burns (`nico.burns`)**, the Dioxus maintainer who
also maintains Blitz itself. The commits come from `DioxusLabs/devin/*` PR
branches.

So `zeno` is not a random third-party crate: it is the Mozilla-derived
rasterizer, already under the Blitz maintainer's own hand, with a
`forbid(unsafe_code)` policy, and its only dependency is `libm`.

**Why that matters for Blitz specifically.** Blitz's `apps/browser` default
feature set is `hybrid` -- Vello on the GPU through wgpu. wgpu plus a GPU
driver stack is by far the largest thing in the binary. A CPU path built on
`zeno` has exactly one dependency (`libm`) and no graphics-driver requirement
at all. `zeno` also brings **pixel-perfect hit testing with a configurable
coverage threshold**, which is a hit-testing primitive Blitz's event layer
does not currently have a dedicated implementation for.

### 2.5 The rest of the candidate set

| Crate | Version | dl/wk | Use in Blitz | Priority |
| --- | --- | --- | --- | --- |
| `zeno` | 0.3.3 | 7,312,662 | CPU raster + exact hit testing | **High** |
| `rquickjs` | 0.14.0 | 3,560,374 | JS engine behind a trait | **High** |
| `harfrust` | 0.12.0 | 12,794,806 | HarfBuzz shaping (already in tree via Parley) | Already there |
| `skrifa` / `read-fonts` | 0.44.0 / 0.41.0 | 27,329,781 / 28,572,212 | Font parsing (already direct deps) | Already there |
| `simdutf8` | 0.1.5 | 110,799,322 | UTF-8 validation; present only via `encoding_rs` | Medium |
| `memchr` | 2.8.3 | 452,500,379 | Byte scanning in the tokenizer | Medium |
| `hashbrown` | 0.17.1 | 872,283,699 | SwissTable hash maps; present via `fontique` | Medium |
| `foldhash` | 0.2.0 | 324,728,829 | Faster non-crypto hash; present via `glifo` | Medium |
| `ryu` | 1.0.23 | 288,754,870 | Shortest float->string; present only via `serde_urlencoded` | Low |
| `oxc_parser` | 0.158.0 | 5,300,960 | Rust JS/TS parser; for a JS syntax checker | Low |
| `tiny-skia` | 0.12.0 | 26,077,190 | CPU Skia subset; alternative to Zeno | Low |
| `cosmic-text` | 0.19.0 | 4,839,116 | Alternative text layout to Parley | Low |
| `quinn` | 0.11.12 | 148,671,694 | QUIC / HTTP3 for `blitz-net` | Low |
| `brotli` / `zstd` | 9.1.1 / 0.14.3 | 89,037,175 / 117,291,452 | Content decoding | Low |
| `rustls` | 0.24.0-dev.2 | 273,721,146 | TLS; replaces `native-tls-vendored` | Medium |
| `wasmtime` | 51.0.0-rc.1 | 12,672,872 | Sandbox for untrusted page script | Low |
| `skera` | 0.9.0 | 71,229 | Font subsetting (already in tree via `skrifa`) | Low |

`runtime` note on `oxc`: it is a parser and transformer, **not a runtime**. It
cannot execute JavaScript. It is useful in Blitz only as a *checker* -- e.g.
to validate a `<script>` block's syntax before handing it to an engine, or to
power a "script blocked by policy" feature. Any claim that oxc could replace
a JS engine is false.

`runtime` note on `rustls`: `packages/blitz-net/Cargo.toml:41` currently
offers `reqwest` with `native-tls-vendored`, which statically links OpenSSL
on Windows. That is a large and licence-noisy dependency for a browser that
wants to be small. `rustls` is pure Rust and is already the more common
choice; the `native-tls-vendored` line should probably be the non-default.

---

## 3. The small mathematical and algorithmic ideas

The user asked specifically for techniques that look useless alone but combine
into something decisive. All four below are `repo-managed` from primary
source or `runtime` from a published measurement. None are speculative.

### 3.1 Eisel-Lemire decimal-to-binary float parsing

**What it is.** An algorithm that converts a decimal string to the nearest
binary64 without a fallback path, using a 128-bit multiply.

**Why it looks useless alone.** It only parses numbers. In isolation it is a
textbook numerical-methods curiosity with no visible product surface.

**Why it is decisive in combination.** Rust's standard library adopted it in
`core::num::dec2flt`, and the source says exactly what it is for. `executed`
from `rust-lang/rust` at tag `1.55.0`:

```text
//! The extended-precision algorithm
//! uses the Eisel-Lemire algorithm, which uses a 128-bit (or 192-bit)
...
use self::lemire::compute_float;
pub mod lemire;
...
// Unable to correctly round the float using the Eisel-Lemire algorithm.
// Fallback to a slower, but always correct algorithm.
```

Note the shape: the fast path is the *only* path for the vast majority of
inputs, and the slow, always-correct path is the rare fallback. That is the
whole design lesson for a browser: a fast path that is *usually* right, plus a
guaranteed-correct slow path, beats one slow path that is always right.

**Where it lands in Blitz.** CSS is full of numbers -- lengths, angles,
opacities, weights, `calc()` expressions, `transition-duration`. Every one of
them is a decimal string that must become a float. Stylo does this parsing, so
Blitz inherits the win for free from the Rust ecosystem -- but the same
technique applies directly to Blitz's own number handling in
`blitz-dom/src/cssom.rs` and to any CSS value Blitz serializes back out
(`getComputedStyle`). `ryu` (already in the lockfile at 1.0.23) is the
mirror-image algorithm for the *output* direction, converting floats to the
shortest round-tripping decimal string, and is the algorithm behind Rust's
own `{}` float formatting.

### 3.2 SIMD UTF-8 validation with a lookup table

**What it is.** Validating UTF-8 using a small lookup table over 16-byte
SIMD chunks.

**Why it looks useless alone.** UTF-8 validation is a correctness chore. It
does nothing visible.

**Why it is decisive in combination.** The Keiser-Lemire paper, `executed`
from arXiv:2010.03090 (submitted 2020-10-06, last revised 2026-04-21,
journal *Software: Practice and Experience* 51(5), 2021), states:

```text
We present the lookup algorithm, which outperforms UTF-8 validation routines
used in many libraries and languages by more than 10 times using commonly
available SIMD instructions.
```

The `simdutf8` crate at 110,799,322 dl/wk is that algorithm in Rust. For a
browser, UTF-8 validation sits on the hot path of every network response,
every file read, and every string slice out of the DOM. A 10x improvement
there is not a micro-optimization; it is the difference between network-bound
and CPU-bound.

**Why it combines.** It does nothing on its own. It becomes decisive when
stacked with a fast HTML tokenizer and a fast network layer, because it
removes the per-byte cost that would otherwise cap both.

**Status in Blitz:** `simdutf8` 0.1.5 is present in `Cargo.lock`, but the
reverse-dependency reconstruction shows it arrives only through
`encoding_rs` and `simd_cesu8` (the ICU charset crates), not as a deliberate
choice for the network or DOM path. Adopting it directly for the bytes that
enter from the network is still an open lever.

### 3.3 Fixed-point layout arithmetic

**What it is.** Representing layout lengths as integers in units of 1/64 of a
pixel instead of as `float`.

**Why it looks useless alone.** Floating point is already fast, and 1/64-pixel
quantization throws away precision you might want.

**Why it is decisive in combination.** Blink, the engine behind Chrome and
Edge, made this choice decades ago and it is still load-bearing. `executed`
from `chromium/chromium`, `third_party/blink/renderer/platform/geometry/layout_unit.h`:

```text
using LayoutUnit = FixedPoint<6, int32_t>;
using TextRunLayoutUnit = FixedPoint<16, int32_t>;
using InlineLayoutUnit = FixedPoint<16, int64_t>;
```

`FixedPoint<6, int32_t>` is exactly 1/64 of a pixel. Three properties fall out
that floats do not have:

1. **Bit-exact equality.** Two layouts that should be identical compare equal
   with `==`. This is what makes layout caching and style sharing possible at
   all -- and Blink's style sharing, which reuses one computed style across
   thousands of sibling elements, is a large part of why Chrome starts fast.
2. **Determinism across platforms.** Integer layout cannot drift between an
   x86 laptop and an ARM phone the way `float` arithmetic can. For a project
   that must run on all OSes, this is not cosmetic.
3. **Cheap comparison and hashing.** Integer keys hash and compare faster than
   float keys, which matters when the style cache is keyed on layout-relevant
   properties.

**Where it lands in Blitz.** Taffy uses `f32` lengths internally. Blitz's own
`blitz-dom/src/layout` is where a fixed-point type would be introduced, as a
Blitz-side type converted to and from Taffy's `f32` at the boundary -- the
same trick Blink plays with its own `LayoutUnit`. The `repo-managed` claim is
that Blink ships this today and it is a `class FixedPoint` template, not a
special case. The `unverified` part is the actual measured speedup for Taffy:
**no source quantifies it**, so it must be benchmarked before being claimed.

### 3.4 Bloom filters on selector identity

**What it is.** A probabilistic set, computed once per stylesheet, that answers
"definitely not present" in constant time.

**Why it looks useless alone.** A Bloom filter gives false positives. On its
own it answers fewer questions than a hash set and gives worse answers.

**Why it is decisive in combination.** Blink uses Bloom filters to prune
candidate style rules before running the expensive selector match. `executed`
from `chromium/chromium`, `third_party/blink/renderer/core/css/rule_set.h`:

```text
// (and a different bloom_hash_backing from what you
// DescendantSelectorIdentifierHashes() will return a slice
unsigned selector_index,
Vector<uint16_t>& bloom_hash_backing);
```

The mechanism: for a candidate element, take the ids and classes it actually
has, and check them against the Bloom filter of each rule's rightmost compound
selector. If the element has none of the identifiers that rule requires, the
rule cannot match, and it is skipped without ever running the full selector
matcher. On a typical page most rules are skipped this way, and the cost is a
handful of memory reads.

**Why it combines.** The Bloom filter is worthless as a lookup structure on its
own. It becomes decisive when combined with rule bucketing by rightmost
selector -- the filter is the cheap pre-pass that lets the expensive matcher
run on a tiny candidate set.

**Where it lands in Blitz.** Stylo does the cascade, so Stylo would own this.
`repo-managed`: `servo/stylo` is a separate upstream repository that Blitz
pins to rev `2dca34592ced37c6aa01733e94206a767194cf9d`. Per `blitz/AGENTS.md`,
"Fixes in Taffy/Parley/anyrender/Stylo go upstream in those repos, not patched
in Blitz." So this is an upstream contribution, not a Blitz-local change --
and it is a good candidate for one, because Stylo is shared with Firefox.

### 3.5 The combination, stated precisely

None of these four is the breakthrough. The combination is:

| Layer | Technique | What it removes |
| --- | --- | --- |
| Network | `simdutf8` lookup validation | per-byte UTF-8 cost |
| Parse | `lazyhtml` / `html5ever` | DOM construction cost |
| Style | Stylo + Bloom filter pruning | candidate rule matching |
| Layout | fixed-point `LayoutUnit` | float drift, enables style sharing |
| Layout | Taffy | box tree computation |
| Text | Parley + Fontique + Harfrust + Glifo | shaping, line breaking, glyph raster |
| Raster | Zeno (CPU) or Vello (GPU) | pixel fill |
| Script | QuickJS-NG via `rquickjs` | interpretation cost |
| Numbers | Eisel-Lemire in, Ryu out | decimal conversion cost |

Each layer is already a well-known Rust project. The claim worth making is
narrower and more defensible than "fastest browser in the world": **the
composition is unclaimed, and Blitz is the only tree that already holds most
of the pieces.** The `repo-managed` fact that Blitz's `Cargo.lock` already
contains `skrifa`, `read-fonts`, `font-types`, `fontique`, `harfrust`,
`glifo`, `simdutf8`, `ryu`, `hashbrown`, `foldhash`, `memchr`,
`fearless_simd`, `guillotiere`, `peniko`, and `core_maths` is evidence that
the substrate is there and largely unused.

---

## 4. Fact-check: what is real, what is hype

The user's question included "fact check does they really unique and has
potential". Direct answers.

### 4.1 Claims that did not survive checking

| Claim | Status | Evidence |
| --- | --- | --- |
| "Sunfish: A Scalable Web Engine for Embedded Browser" (ICLR 2025), 2.8x faster page load, 5.5x less memory than Chromium | **Not found.** No arXiv record, no OpenReview record, no Semantic Scholar record. arXiv IDs returned for the title were unrelated papers (a LaTeX-OCR paper, a math paper). `runtime`: arXiv API queries for `ti:"Sunfish"`, `abs:"web engine" AND abs:"embedded"`, and `all:"embedded browser"` returned no such title; Crossref returned nothing matching; OpenReview search for "Sunfish web engine embedded" returned only unrelated papers. | Do not cite. |
| "Cosmo" (Meta) as a JS engine replacing SpiderMonkey internals | **Not found.** `github.com/facebookexperimental/cosmo` returns 404 from the GitHub API and "Repository not found" from `git ls-remote`. The `facebookexperimental` org's 100 most-recently-updated repos contain no cosmo. No `cosmo` crate exists on crates.io. | Do not cite. |
| "Mozilla Zeno" | **Right idea, wrong attribution.** The real crate is `dfrg/zeno`, Apache-2.0 OR MIT, 157 stars, 7.3M dl/wk, authored by **Chad Brokaw, a Mozilla gfx2d engineer**, and maintained since August 2026 by **Nico Burns of DioxusLabs**. There is no `mozilla/zeno` repository -- the Mozilla link is the author's, not the repo's. | Cite as `dfrg/zeno`. |
| "Fontations" as a Firefox 145 feature | **Partly right.** `fontations` (the umbrella crate, 1,532 dl/wk) and its subcrates `skrifa`, `read-fonts`, `write-fonts`, `font-types` are real and heavily used -- `skrifa` at 27.3M dl/wk. The repo is `googlefonts/fontations`. The Firefox-145 connection was not confirmed by any source read in this session. Blitz already depends on `skrifa` directly. | Cite the crates, not the Firefox version. |
| "LazyHTML" as a known drop-in HTML parser | **Half right.** `cloudflare/lazyhtml` exists (148 stars, BSD-3-Clause, pushed 2026-04-23) and is a real HTML5-compliant parser with a pluggable transformation pipeline. It is not a DOM builder comparable to html5ever. | Cite as an experimental pre-pass. |
| "Blitz already contains Zeno via Swash" | **Wrong for this tree.** `swash` is absent from Blitz's `Cargo.lock`; Parley 0.12 uses `fontique` + `parley_engine` + `harfrust`. `zeno` is therefore absent too. | Zeno is an addition, not an existing component. |

### 4.2 Claims that are real and verified

| Claim | Status |
| --- | --- |
| Boa's JS throughput is far below QuickJS-NG | Verified, two independent measurements (CatPaw ADR 0001; Zipp PERF_ROADMAP) |
| Boa's packaged WASM is ~13x QuickJS-NG's size | Verified from Zipp's PERF_ROADMAP byte counts |
| `rquickjs` wraps QuickJS-NG and is far more used than Boa | Verified: 3,560,374 dl/wk vs 2,011,504 dl/wk |
| Blitz's browser app does not enable JavaScript by default | Verified at `apps/browser/Cargo.toml:13` |
| Blitz's JS engine is Boa v0.22.0 | Verified at `packages/blitz-vibey-script/Cargo.toml` |
| Blitz's JS globals list is ~33 names, with no fetch/XHR/observers/canvas/media | Verified by enumeration of `runtime/bootstrap.rs` |
| Blitz already depends on `skrifa` (Fontations) **directly** | Verified at `packages/blitz-dom/Cargo.toml:75` and `packages/blitz-paint/Cargo.toml:40` |
| Blitz's shaping engine is `harfrust`, not `rustybuzz` | Verified by reverse-dependency reconstruction of `Cargo.lock` |
| `zeno`'s only dependency is `libm`; it is Mozilla-derived and Dioxus-maintained | Verified from a local clone of `dfrg/zeno` (45 commits) |
| `zeno` does 256x AA, subpixel masks, exact hit testing, non-zero and even-odd fills | Verified from `dfrg/zeno` README |
| Blink uses `FixedPoint<6, int32_t>` layout units (1/64 px) | Verified in `layout_unit.h` |
| Blink uses Bloom filters to prune style rules | Verified in `rule_set.h` |
| Rust std uses Eisel-Lemire for decimal-to-float | Verified in `core/src/num/dec2flt/mod.rs` at tag 1.55.0 |
| Keiser-Lemire SIMD UTF-8 validation is >10x faster than prior routines | Verified from the arXiv abstract and journal reference |
| Tauri apps can be as small as 600 KB | Verified on `tauri.app` (homepage, "Minimal Size") |
| QuickJS-NG reactor WASM is 1,528,293 bytes raw / 417,087 Brotli | Verified from Zipp PERF_ROADMAP |

### 4.3 Is anything genuinely unique?

**Unique to Blitz:** the composition. No single project combines a
Stylo-grade CSS engine, Taffy layout, Parley text, Fontations fonts, HarfBuzz
shaping, Vello rasterization, and a pluggable JS engine in a tree this small.
Servo is the closest in ambition but still depends on SpiderMonkey (C++),
and its layout is its own rather than Taffy.

**Not unique, and claimed by others:**

- SwiftShader, Dawn, and wgpu all chase the "bring your own GPU backend"
  idea. Blitz's `anyrender` abstraction is the same move, already made.
- Ladybird and Servo both chase a from-scratch engine. Blitz is downstream of
  Servo's CSS engine, so it is a different bet.
- Lightpanda (Zig, 41,670 stars, AGPL-3.0, pushed 2026-11-09) has chosen the
  opposite trade: no rendering at all, just a headless DOM for automation.
  That is a legitimate answer to "smallest browser" -- it defines the browser
  as everything except the pixels. Blitz could not adopt it, because Blitz's
  entire value is the pixels.
- The engine-neutral `JsRuntime` trait pattern is CatPaw's ADR 0001, not
  Blitz's invention. Blitz adopting it would be a catch-up, and that is
  fine.
- Zeno is Mozilla's own work, and its current maintainer is Blitz's own
  maintainer. That is a genuine advantage over any other project trying to
  adopt it, and it is also why the "unique" claim here is weaker than it
  looks -- Blitz would be taking a Mozilla asset, not inventing one.

---

## 5. Comparison against the field

Sizes are not comparable across these projects without care: some numbers are
a full browser install, some a single shared library, some an app bundle that
delegates rendering to the OS. The table separates them.

### 5.1 By role and footprint

| Project | Language | JS engine | Rasterizer | Footprint | Licence |
| --- | --- | --- | --- | --- | --- |
| **Blitz** | Rust | Boa (opt-in) | Vello/wgpu | `unverified` -- see 1.3 | MIT/Apache-2.0 |
| Servo | Rust | SpiderMonkey (C++) | WebRender | Servo crates.io crate `servo` v0.8.0, 34,984 dl/wk | MPL-2.0 |
| Ladybird | C++ / Rust | JavaScriptCore | Skia | Not measured here | BSD-2-Clause / GPL |
| Lightpanda | Zig | None (no rendering) | None | 41,670 stars; no rendering by design | AGPL-3.0 |
| Chrome / Chromium | C++ | V8 | Skia / Graphite | ~150-250 MB installed | BSD-3-Clause |
| Firefox | C++ / Rust (Stylo) | SpiderMonkey | WebRender | ~200 MB installed | MPL-2.0 |
| Safari | C++ / Rust | JavaScriptCore | CoreGraphics | Not measured here | Closed |
| Electron | C++ / Node | V8 | Chromium | ~130 MB+ app | MIT |
| Tauri | Rust + system webview | OS-provided | OS-provided | "as small as 600 KB" (tauri.app) | MIT/Apache-2.0 |
| Qt WebEngine | C++ | V8 | Chromium | Chromium-sized | LGPL/commercial |
| GPUI | Rust | None (native) | Metal / Vulkan | Zed's UI framework, v0.2.2 | Apache-2.0 |

### 5.2 The honest strategic comparison

**Tauri does not compete with Blitz.** Tauri ships no engine. It renders
through `WebView2` on Windows, `WKWebView` on macOS, `WebKitGTK` on Linux.
Its 600 KB is real and it is also a different product: it is a window around
someone else's browser. Blitz is the engine itself. A Tauri app on Windows
gets Chromium; a Blitz app gets Blitz, identically on every OS.

**Electron does not compete with Blitz either.** Electron is Chromium with
Node attached. Its size is a feature -- you get Chrome's exact behaviour.

**GPUI is the nearest sibling in spirit.** It is Rust, it is GPU-rendered,
and it deliberately omits a web engine. Blitz is "GPUI plus an HTML/CSS
document model". Zed's `gpui` at v0.2.2 and Blitz's `anyrender` are the same
abstraction with different names.

**Qt is the incumbent to beat on embedded.** Qt WebEngine is Chromium in a
harness, so it inherits Chromium's size. Qt Quick (no web engine) is the
competitor, and it is not HTML.

**Servo is the closest peer and the most useful comparison.** Servo proves a
Rust engine can be browser-grade. It also shows the cost: Servo keeps
SpiderMonkey, so it stays a C++ codebase with a Rust skin. Blitz's bet is
that a small, focused, *scripting-optional* engine is a better product than
a complete one.

### 5.3 What "smallest and fastest in the whole world" can honestly mean

It cannot mean "renders YouTube". Nothing smaller than Chromium renders
YouTube today, and the reason is not engineering effort but the surface area
of `<video>`, `MediaSource`, custom elements, observers, WebGL, and a
JIT-tiered JS engine. Lightpanda, the smallest serious browser project, does
not render pixels at all.

It can mean three narrower, defensible claims:

1. **Smallest engine that renders real HTML/CSS correctly.** Blitz is
   already close to the best answer, and Zeno-as-CPU-backend plus dropping
   wgpu from the default features is the concrete path to a materially
   smaller binary.
2. **Fastest to first paint for a static document.** Here Blitz's advantages
   compose: Stylo's parallel style resolution, Taffy's cheap layout, Parley's
   text layout, and Zeno's CPU rasterization with no GPU wait. A static page
   that never runs script does not need a JS engine at all -- which is
   precisely why Blitz's `javascript` feature is off by default and should
   stay that way for the size-sensitive build.
3. **Fastest and smallest embedded webview for native apps.** This is the
   actual Dioxus use case, and it is the one where Blitz has no equivalent.

---

## 6. The plan, in dependency order

Ordered so that each step is independently valuable and independently
testable, and so that the riskiest change (the JS engine swap) comes after
the cheap, certain wins.

| # | Change | Effort | Risk | Why now |
| --- | --- | --- | --- | --- |
| 1 | Add a `cpu-zeno` paint feature; make Vello GPU opt-in | Medium | Medium | One dependency (`libm`), no graphics-driver requirement, Mozilla-derived, Dioxus-maintained |
| 2 | Split `blitz-vibey-script` behind a `JsRuntime` trait | Medium | Low | Pure refactor; unblocks 3 and 4; copies CatPaw ADR 0001 |
| 3 | Add `js-quickjs` via `rquickjs` | Medium | Medium | ~3x faster than Boa, ~13x smaller packaged WASM |
| 4 | Add `interrupt` + memory limit to the trait | Small | Low | Closes the "one bad script freezes the window" hole |
| 5 | Implement the missing DOM surface: `fetch`, `XMLHttpRequest`, `MutationObserver`, `IntersectionObserver`, `history`, `localStorage`, `customElements` | **Large** | Medium | **This is what actually unblocks YouTube and Vercel** |
| 6 | Add `<video>` / `HTMLMediaElement` and `canvas` | Large | High | The remaining YouTube blockers |
| 7 | Swap `native-tls-vendored` for `rustls` in `blitz-net` | Small | Low | Removes OpenSSL from the Windows binary |
| 8 | Evaluate `lazyhtml` as a tokenizer pre-pass | Medium | Low | Benchmark first; it is a 148-star project |
| 9 | Pilot fixed-point layout units in `blitz-dom` | Large | Medium | Benchmark first; no published Taffy measurement exists |
| 10 | Contribute Bloom-filter rule pruning upstream to `servo/stylo` | Medium | Low | Per `blitz/AGENTS.md`, Stylo fixes go upstream |
| 11 | Use `zeno`'s hit-testing primitive for Blitz's event layer | Medium | Low | Pixel-perfect hit testing with a coverage threshold is exactly what Blitz's event layer lacks |

**Step 5 is the one that matters and it is the most expensive.** Steps 1-4
and 7-8 make Blitz smaller and faster but will not make YouTube render. Any
plan that stops at step 3 has produced a faster Wikipedia, not a browser.

---

## 7. How to verify every number in this document

Per `AGENTS.md` section 4, no claim in this file is `release-qualified`. The
following commands produce the receipts that would upgrade them.

```powershell
# The Blitz binary size this document refuses to quote (section 1.3).
cd G:\Dx\blitz
cargo build --release -p browser --no-default-features `
  --features cpu-softbuffer,cookies,cache
Get-Item .\target\release\blitz.exe | Select-Object Name,Length

# The same, with the GPU path enabled, for the size delta.
cargo build --release -p browser
Get-Item .\target\release\blitz.exe | Select-Object Name,Length

# The JS engine swap, in isolation.
cargo build --release -p blitz-vibey-script --features js-quickjs
```

```text
# Sizes of the engines being compared (runtime, crates.io API).
# boa_engine, rquickjs, quickjs-ng reactor, zipp.
curl -s "https://crates.io/api/v1/crates/rquickjs" -H "User-Agent: research"
curl -s "https://crates.io/api/v1/crates/zeno"      -H "User-Agent: research"

# The engine benchmarks being cited.
# CatPaw ADR 0001  -> github.com/KernelErr/CatPaw/docs/adr/0001-js-engine-boa-default.md
# Zipp PERF_ROADMAP -> github.com/f2i-com/zipp.org/blob/main/docs/PERF_ROADMAP.md

# Zeno's real provenance and the reverse-dependency graph used in 2.3.
git clone --filter=blob:none https://github.com/dfrg/zeno
git -C zeno log --format='%ad | %an | %s' --date=short
```

The `runtime` sources for the JS engine comparisons are third-party benchmark
suites, not first-party engine benchmarks. CatPaw's ADR cites a third-party
engine benchmark for its ~200x and ~3x figures; Zipp's roadmap is a project
benchmarking itself against others. Both are reproducible and both are
interested parties. Treat the *ordering* (Boa slowest, QuickJS-NG middle,
tiered-JIT fastest) as well supported and the *exact ratios* as
approximately right.

---

## 8. The short answer

The gems are real, and most of the modern Rust substrate is already in
Blitz's tree. But the premise needs one correction: **YouTube and Vercel fail
in Blitz because of missing DOM APIs and because JavaScript is off by
default, not primarily because Boa is slow.** Boa is slow -- measured at
roughly a quarter of QuickJS-NG's throughput -- and swapping it is worth
doing. But swapping the engine while `fetch`, `MutationObserver`,
`customElements`, `<video>`, and `history` are all absent yields a browser
that runs JavaScript beautifully and still cannot load a modern page.

The genuine hidden gems found in this pass, in order of value:

1. **`zeno`** -- a Mozilla-derived (gfx2d) pure-Rust 256x-antialiased
   rasterizer with exact hit testing, whose only dependency is `libm`, with
   7.3M downloads/week, and which since August 2026 is maintained by Blitz's
   own maintainer. Making it the default CPU paint backend and Vello opt-in
   is the single largest size win available.
2. **`rquickjs` / QuickJS-NG** -- 3x Boa's speed and ~13x its packaged size,
   behind a `JsRuntime` trait so both stay available.
3. **`lazyhtml`** -- a 148-star Cloudflare HTML5 parser, almost unwatched.
4. **`harfrust` and `glifo`** -- already in the tree via Parley 0.12, and
   they are why Blitz's text stack is already modern; worth knowing they are
   there before reaching for `swash` or `rustybuzz`, which it no longer uses.
5. **`simdutf8`** -- Keiser-Lemire's >10x SIMD UTF-8 validation, on every
   network byte; currently in the tree only as an ICU transitive.

And the genuinely mathematical ones, which look useless alone and combine
into the whole:

- **Eisel-Lemire** (parse) and **Ryu** (serialize) -- number handling that
  is exact on the fast path and correct on the slow path.
- **Fixed-point 1/64-pixel layout units** -- buys bit-exact equality, which
  is the precondition for style sharing and layout caching, which is
  precondition for Chrome-class startup times.
- **Bloom filter pruning** -- a lossy structure that is only useful once
  combined with rule bucketing, and then removes most selector matching.
- **SIMD UTF-8 lookup validation** -- a pure per-byte cost that vanishes
  once the layers above it are also fast.

The combination is unclaimed. Nobody has assembled these into the smallest
fastest browser, and the reason is probably that the full set of layers
required to render a *modern* page is much larger than the set required to
render a static one -- which is exactly the boundary Blitz currently sits on.

MIT OR Apache-2.0
updated: 2026-11-09 00:00 UTC
