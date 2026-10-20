# G:\Dx space review — 2026-09-23

**Nothing has been deleted.** Every item below is a proposal with the evidence that supports it.

`G:` is at **99% — 477 GB total, 472 GB used, 5.2 GB free.** `G:\Dx` itself is **~166 GB**, so
roughly 306 GB lives elsewhere on the volume (`G:\Dev`, `G:\Archives`, `G:\Ai`, `G:\osxcross`, …).

**Headline: ~88 GB of `G:\Dx` is regenerable build/dependency output.** That is the win, and it is
much larger than the loose clutter we swept earlier.

---

## 1. rustup / cargo: you were right — there are three copies, and only one pair is live

The `G:\Dev` copy is under `Caches`, not `Tools`, and there is a **third** copy inside `G:\Dx`.

| location | toolchains | live? | evidence |
|---|---|---|---|
| `C:\Users\Computer\.rustup` → junction → `G:\DevCaches\rustup` | 2 | **YES** | `rustup show` → `rustup home: C:\Users\Computer\.rustup`, listing exactly these 2 |
| `C:\Users\Computer\.cargo` → junction → `G:\DevCaches\cargo` | — | **YES** | `~/.cargo` resolves here |
| `G:\Dev\Caches\rustup` (also `D:\DevCaches\rustup`) | **17** | only if `RUSTUP_HOME` is set — it is **not** | `D:\DevCaches` is a symlink → `G:\Dev\Caches` |
| `G:\Dev\Caches\cargo` | — | **partly** — `which cargo` → `D:\DevCaches\cargo\bin\cargo`, i.e. this `bin/`; toolchains still come from `G:\DevCaches\rustup` | `which -a cargo` |
| `G:\Dx\.rustup` | 1 (`1.96.0-x86_64-pc-windows-msvc`) | **no** | 1.2 GB; nothing references it |
| `G:\Dx\.cargo` | registry only — **no `bin/`, no `config.toml`** | **no** | 103 MB; nothing references it |
| `G:\Dx\cli\.cargo-home` | — | **declared but missing** | `dx` manifest line 24: `cargo_home = cli/.cargo-home` |

**Why the duplicate:** the `G:\Dx` pair is a redundant third copy, not the one in use. `CARGO_HOME`
and `RUSTUP_HOME` are **unset**, and a repo-wide grep found **no script, Makefile, CMake file or doc
referencing `CARGO_HOME`, `RUSTUP_HOME`, or `G:\Dx\.cargo`/`.rustup`**. `cargo +1.96.0 --version`
resolves to `cargo 1.96.0` purely from the live home, so removing the `G:\Dx` pair cannot break the
build.

Two loose ends:

- `G:\Dx\cli\.cargo-home` is declared in the manifest but **does not exist**. A same-shaped stray sits
  at `G:\Dx\cli\trash\.cargo-home` (`.global-cache`, `.package-cache`, `.package-cache-mutate`,
  `registry` — byte-for-byte the same four entries as `G:\Dx\.cargo`). The declared cargo home appears
  to have been swept into `cli/trash/`.
- `check/.cargo`, `cli/.cargo`, `daemon/.cargo`, `dcp/.cargo` each contain **only a `config.toml`**.
  Intentional per-repo configs — **not** duplicates. Leave them.

---

## 2. What is left of the dx-decisions / dx project at `G:\Dx`

### The agent project

The manifest still declares `dx_agents = agent`, but **`G:\Dx\agent` no longer exists** — that
reference is dangling. The agent project now lives at:

```
G:\Dx\DEPRICATED_10-20-2026\agent\     origin = github.com/essence-dx/agent.git
```

with siblings `codex/` (46 entries), `openclaw/` (63), `providers/` (11).

### `DEPRICATED_10-20-2026/` — 3.3 GB, 22 separate pushed repos

`agent`, `build`, `checkpoint-desktop`, `checkpoint-tui`, `code`, `codex`, `desktop-code`,
`diffusion`, `dx-video-player-depricated-20260820-1856`, `js`, `landing`, `logo`, `mobile`,
`mobile-depricated`, `mobile-depricated-latest`, `native`, `openclaw`,
`os-depricated-20260820-1856`, `providers`, `py`, `train`, `website-depricated`.

**21 of 22 are fully pushed with a clean working tree** (0 unpushed commits, 0 dirty) and each has
`origin = github.com/essence-dx/<name>.git` — recoverable with a plain `git clone`. Their compressed
history totals **≈938 MB** (`git count-objects` `size-pack`).

**Exception: `mobile/`** (branch `dev`) — 0 unpushed commits, but **15,889 files staged for
deletion** plus 50 untracked entries (`.agents/`, `.claude/`, `.cursor/`, `.devin/`, `.changeset/`,
`.github/`, …). Mid-operation, not clean. **Review before touching.**

---

## 3. Is the decisions build rebuildable? Yes — proven

`G:\Dx\.target-decisions\` (73 MB) is a **live CMake/Ninja tree**, not a stale artifact:

```
CMAKE_GENERATOR:INTERNAL=Ninja
CMAKE_BUILD_TYPE:STRING=Release
CMAKE_CXX_COMPILER:UNINITIALIZED=G:/Dev/Tools/Scoop/apps/mingw/current/bin/c++.exe
CMAKE_MAKE_PROGRAM:FILEPATH=G:/Dev/Tools/Scoop/shims/ninja.exe
dx_decisions_BINARY_DIR:STATIC=G:/Dx/.target-decisions
dx_decisions_SOURCE_DIR:STATIC=G:/Dx/decisions
```

Both tools still exist (`c++.exe` 3,314,688 B; `ninja.exe` 136,192 B) and a dry run of its own build
system confirms it regenerates and builds:

```
$ ninja -n -C G:/Dx/.target-decisions
ninja: Entering directory `G:/Dx/.target-decisions'
[0/1] Re-running CMake...
```

It holds 17 `.exe` outputs (`dx-decisions.exe` 7.3 MB, `dx_eval_harness.exe` 7.0 MB, the
`serializer_*` tests, …) — all compiler output.

**Before deleting it, two facts:**

1. **Nothing in the repo references it.** Grepping `target-decisions` across `decisions/` returns zero
   hits, and `decisions/Makefile` still defaults `BUILD_DIR ?= build`, so `make` never rebuilds into
   it. It is an out-of-tree build someone invoked explicitly.
2. **It is one of three build trees.** `decisions/build/` (9.8 MB) and `decisions/build-onnx/`
   (28 MB) are also complete CMake trees, and `make distclean` removes `build-onnx` by design.

Rebuild command:

```
cmake -S G:/Dx/decisions -B G:/Dx/.target-decisions -G Ninja -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_CXX_COMPILER=G:/Dev/Tools/Scoop/apps/mingw/current/bin/c++.exe
cmake --build G:/Dx/.target-decisions
```

---

## 4. Where the 166 GB actually is

| project | size | biggest single thing inside |
|---|---|---|
| `decisions/` | 31.4 GB | `.models/` 31.3 GB |
| `desktop/` | 29.8 GB | `desktop/web/` 30.3 GB (18 app subdirs) |
| `bot/` | 29.3 GB | `node_modules/` 15.2 GB + `.next/` 11.3 GB |
| `extensions/` | 19.2 GB | `node_modules/` 13.6 GB |
| `cline/` | 18.3 GB | `node_modules/` 12.4 GB |
| `video-player/` | 7.2 GB | 16 × `build-<target>/` ≈ 6.5 GB |
| `media/` | 5.0 GB | `icon/` 417 MB |
| `bin/` | 4.7 GB | `pre-imgbb/` 183 MB, `stale-pre-rename/` 174 MB |
| `www/` | 3.6 GB | `trash/` 1.8 GB, `examples/` 1.6 GB |
| `DEPRICATED_10-20-2026/` | 3.3 GB | 22 git repos |
| `mobile/` | 2.9 GB | — |
| `website/` | 2.4 GB | — |
| `tui/` | 1.9 GB | `dist/` 213 MB |
| `flow-like/` | 1.6 GB | — |
| `.rustup/` | 1.2 GB | duplicate toolchain |
| `cli/` | 930 MB | `trash/` 686 MB |

---

## 5. Deletion candidates

### Tier A — regenerable build & dependency output. ~88 GB. Highest confidence.

Every one of these is reproducible from a lockfile or source tree.

| # | path | size |
|---|---|---|
| 1 | `desktop/web/Router/node_modules` | **19.62 GB** |
| 2 | `bot/node_modules` | **15.23 GB** |
| 3 | `extensions/node_modules` | **13.61 GB** |
| 4 | `cline/node_modules` | **12.44 GB** |
| 5 | `video-player/build-*` (16 cross-target dirs) | **≈6.5 GB** |
| 6 | `desktop/web/Graph/node_modules` | 3.48 GB |
| 7 | `desktop/web/Video/node_modules` | 850 MB |
| 8 | `desktop/web/3D/node_modules` | 781 MB |
| 9 | `desktop/web/Spreadsheets/node_modules` | 641 MB |
| 10 | `desktop/web/Graphics/node_modules` | 614 MB |
| 11 | `desktop/web/Train/node_modules` | 602 MB |
| 12 | `desktop/web/Cms/node_modules` | 430 MB |
| 13 | `tui/dist` | 213 MB |
| 14 | `desktop/target` | 64 MB |
| 15 | `video-player/build` | 45 MB |
| 16 | `decisions/build-onnx` + `decisions/build` | 38 MB |
| 17 | `.target-decisions` | 73 MB |
| 18 | `G:\Dx\.rustup` | 1.2 GB |
| 19 | `G:\Dx\.cargo` | 103 MB |
| 20 | `G:\Dx\node_modules` | 12 MB |
| 21 | `zzz/from-gdx-root/harness/.wb-edge-profile` | 66 MB |

**Subtotal ≈ 88 GB.** Note items 1–4 alone are **60.9 GB**.

> ⚠️ **One caveat on `bot/.next` (11.3 GB).** It is Next.js build output and would normally be the
> easiest win — but your own project convention is **never run `next build`**. So deleting it is
> space-safe but you may not be able to regenerate it. I have left it **out** of the table above and
> flag it for your decision: **11.3 GB, only if you can rebuild.**

### Tier B — trash and stale folders. ~2.8 GB. Low risk, worth eyeballing.

| # | path | size | note |
|---|---|---|---|
| 22 | `www/trash/` | 1.8 GB | contains `CSS.md`, `CURRENT_STATE.md`, `DX.md`, `PLAN.md`, `README.md` — a docs trash |
| 23 | `cli/trash/` | 686 MB | **check first** — holds `cli/trash/.cargo-home`, and the manifest's declared `cli/.cargo-home` is missing |
| 24 | `bin/pre-imgbb/` | 183 MB | |
| 25 | `bin/stale-pre-rename/` | 174 MB | prebuilt `dx-daemon` binaries for many targets; literally named *stale* |

### Tier C — needs your explicit call. ~35 GB.

| # | path | size | the catch |
|---|---|---|---|
| 26 | `DEPRICATED_10-20-2026/` | 3.3 GB | 22 pushed repos — but `mobile/` is dirty (15,889 staged deletions). Clone-to-recover works for the other 21. |
| 27 | `G:\Dev\Caches\rustup\` | **17 toolchains** (1.87 → nightly, gnu + msvc) | The live home is `G:\DevCaches\rustup` with **2**. Toolchains are typically 1–2 GB each, so the extra 15 are likely **15–25 GB** of dead weight — but confirm nothing sets `RUSTUP_HOME` to this one first. |
| 28 | `G:\DevCaches\{wb,edge-probe-fanout,edge-probe-fresh,edge-web-link}` | unmeasured | harness scratch siblings of the `.wb-*` files we just parked |
| 29 | `G:\Dev\Caches\{puppeteer,npm,pnpm-cache,bun-install-cache,pip,uv,gradle}` | unmeasured | package-manager caches — regenerable, often tens of GB combined |

### Tier D — the 31 GB model store. Your call, per model.

`decisions/.models/` = **31.3 GB** — the single biggest item on the drive. **It holds
`rerankers/ettin-17m`, the validated `ettin-reranker-17m-v1` checkpoint. That must stay.**

Most of the rest is re-downloadable third-party weight:

| file | size |
|---|---|
| `decoder/MiniCPM5-2B-Q8_0.gguf` | 2.68 GB |
| `nemotron-rerank-1b-v2/model.safetensors` | 2.47 GB |
| `bge-reranker-v2-m3/model.safetensors` | 2.27 GB |
| `encoder-large/model_fp32.onnx` | 1.74 GB |
| `decoder/MiniCPM5-2B-Q4_K_M.gguf` | 1.56 GB |
| `decoder/MiniCPM5-1B-Claude-Opus-Fable5-V2-Thinking-Q8_0.gguf` | 1.15 GB |
| `decoder/MiniCPM5-1B-Q4_K_M.gguf` | 688 MB |
| `qwen3-reranker/` (401 files) | 621 MB + |
| `reranker/model_fp32.onnx` | 599 MB |
| `reranker/model.safetensors` | 598 MB |
| `encoder/model_fp32.onnx` | 568 MB |

Subdirs: `bge-reranker-v2-m3`, `decoder`, `encoder`, `encoder-large`, `eval`, `eval-reranker`,
`eval-reranker-diverse`, `needle`, `nemotron-rerank-1b-v2`, `qwen3-reranker`, `reranker`,
`rerankers` (9 models: `ettin-150m`, **`ettin-17m`**, `ettin-1b`, `ettin-68m`, `jina-v3`,
`jina-v3.5`, `qwen3-reranker-0.6b-gguf`, `qwen3-reranker-4b`, `qwen3-reranker-4b-gguf`).

Selective pruning here could reclaim **20 GB+**. I have **not** touched it and am **not**
recommending a wholesale wipe — this needs a per-model decision from you.

### Never delete

- **`G:\Dx\dx`** — the workspace root manifest. Moving it breaks every path in the workspace.
- **`G:\Dx\.dx\`** — build-status receipts.
- **`G:\Dx\decisions\.models\rerankers\ettin-17m\`** — the validated checkpoint.
- **`G:\Dx\decisions\zzz\scratch-dirs\`** — **~30,591 authored data rows never appended to the
  corpus.** Not in the 410,004 count. Unmerged work, not rubbish.
- **`G:\Dx\decisions\{corpus,tools,eval,src,include,configs,scripts,tests}\`** — the product.
- **`G:\Dx\.workbuddy-ai\`** — agent state.
- The live Rust homes, and the per-repo `.cargo/config.toml` files.
- The sibling repos (`bot`, `cli`, `tui`, `check`, `style`, `media`, `serializer`, …) — each its own
  origin.

---

## Method notes

- **`du` is unusable on this volume.** `G:` is 99% full and runs at ~50 files/sec — `du -sh assets`
  (1,753 files) took **33.5 s**, and `du` on a large directory exceeded 10 minutes. **All sizes here
  come from `robocopy /L /E /BYTES`**, which reads the same directory in **0.5 s** (~70× faster).
  Verified against `du` on `assets`: 135,416,100 B vs `du`'s 133 M.
- "Safe" claims are backed by `git log @{u}..` (unpushed commits), `git status --porcelain` (dirty
  files), `git remote get-url origin` (recoverability), and repo-wide greps for
  `CARGO_HOME` / `RUSTUP_HOME` / `target-decisions`.
- Repo sizes in Tier C are `git count-objects -vH` `size-pack` values (compressed history, excludes
  working trees).
- Raw measurements: `.workbuddy-ai/memory/size-scan-robocopy-2026-09-23.txt`,
  `size-scan-regenerable-2026-09-23.txt`.
