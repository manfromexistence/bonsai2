# DX

`G:\Dx` is the hub for the DX ecosystem: one CLI (`dx`) plus a family of
Rust-first tools and products that each live in their own repository beside
it. This repository carries the hub's operating memory, routing manifest, and
public story. It is not a monorepo. Every child folder below is an independent
git repository with its own `origin`, and work on a product belongs in that
product's repo on that repo's branch.

The previous hub README is preserved as `DEPRECATED_README.md`. Read
`AGENTS.md` before touching anything here; it is the contract for how this
hub is worked on safely.

## Start here

```powershell
dx install            # bootstrap: download the CLI, extend PATH, install tools
dx tool list --json   # registry tools with install state and versions
dx --help             # the full public command surface
```

## DX tools

A tool is a CLI binary driven through `dx <tool>` and published by
`TOOL_REGISTRY` in `cli/src/update_registry.rs` (repo-managed). The registry
currently holds 21 entries: the `cli` entry (published as `dx.exe`) plus 20
companion tools.

| Tool | Folder | Repository | Binary | Command |
| --- | --- | --- | --- | --- |
| cli | `cli` | `essence-dx/cli` | `dx` | the CLI itself |
| check | `check` | `essence-dx/check` | `dx-check` | `dx check` |
| dcp | `dcp` | `essence-dx/dcp` | `dx-dcp` | `dx dcp` |
| driven | `driven` | `essence-dx/driven` | `dx-driven` | `dx driven` |
| forkness | `forkness` | `essence-dx/forkness` | `dx-forkness` | `dx forkness` |
| forge | `forge` | `essence-dx/forge` | `dx-forge` | `dx forge`, `dx add` |
| grimoire | `grimoire` | `essence-dx/grimoire` | `dx-grimoire` | `dx grimoire` |
| hackin | `hackin` | `essence-dx/hackin` | `dx-hackin` | none yet |
| i18n | `i18n` | `essence-dx/i18n` | `dx-i18n` | `dx i18n` |
| icon | `icon` | `essence-dx/icon` | `dx-icon` | `dx icon` |
| media | `media` | `essence-dx/media` | `dx-media` | `dx media` |
| metasearch | `metasearch` | `essence-dx/metasearch` | `dx-metasearch` | `dx metasearch` |
| os | `os` | `essence-dx/os` | `dx-os` | `dx os` |
| script | `script` | `essence-dx/script` | `dx-script` | `dx script` |
| serializer | `serializer` | `essence-dx/serializer` | `dx-serializer` | `dx serializer` |
| skilltune | `skilltune` | `essence-dx/dx-skilltune` | `dx-skilltune` | `dx skilltune` |
| style | `style` | `essence-dx/style` | `dx-style` | `dx style` |
| token | `token` | `essence-dx/token` | `dx-token` | `dx token` |
| tree | `tree` | `essence-dx/tree` | `dx-tree` | `dx tree` |
| www | `www` | `essence-dx/www` | `dx-www` | `dx www`, `dx new`, `dx dev`, `dx run`, `dx graph`, `dx deploy` |
| daemon | `daemon` | `essence-dx/daemon` | `dx-daemon` | none yet |
| video-player | `video-player` | `essence-dx/video-player` | `dx-video-player` | none yet |

`daemon` and `video-player` are DX tools with published release binaries in
`tools/bin/` (verified by directory listing on 2026-10-09) that have no `dx` route
wired yet. `forkness` and `grimoire` are DX tools with a `dx` route wired and
no release artifact yet — see the honest command-surface status below.
`hackin` is the Rust port of the upstream Strix AI pentesting engine: the
workspace is repo-managed and green, and **it runs** - the agent loop, the
provider, the budget, the session store, the turn guard, the request log, the
**75 knowledge packs**, the toolbox, a real Docker sandbox, a live multi-agent
graph, the MCP client, the web research client and the upstream's operating
doctrine are ported and wired together, verified end to end against a local
mock of the OpenAI API. The repository is **`essence-dx/hackin`, private**, and
its Python citations are **checked by a script** rather than trusted - the
check found **24** stale ones, now fixed. But **the MCP stdio transport, the
subscription sign-in, the dedupe judge, the TUI, the viewer and the cloud CLI
are not ported**, no release binary has been published, and **its CI cannot
run**: GitHub allocates no runner to the organisation's private repositories
(`TODO.md` `BLK-05`) - see `docs/hackin.md` and `hackin/TODO.md`.
Everything else in the table has a live `dx <tool>` command.

### Install, update, and discovery

`dx install` is the one-command bootstrap: it downloads the CLI into
`%LOCALAPPDATA%\dx\bin`, adds that directory to the user `PATH`, then launches
the installed CLI to install the 18 companion tools. `dx tool
list|install|update|doctor|repair` manages them per tool or `--all`, always
with `--dry-run` on mutating commands and `dx.tool.*.v1` JSON contracts.

Release assets follow one naming convention:
`dx-<tool>-<rust-target-triple>[.exe]`. On Windows one `dx-<tool>` file per
tool lands in `%LOCALAPPDATA%\dx\bin`. The `tools/bin/` folder at the hub root holds
the locally staged per-triple builds (Windows x64/arm64, Linux x64/aarch64
plus musl, macOS x64/arm64, Android, and armv7 entries as of 2026-10-09).

### Honest command-surface status

- Added: `dx skilltune` runs the real `dx-skilltune` binary from the
  `skilltune/` Rust workspace (ReflACT skill-document training: `train`,
  `gate`, `rank`, `apply`, `prompts`, `config`). It works once
  `dx-skilltune` is installed or built locally.
- Added: `dx bot` and `dx decisions` are registered and routed, but fail
  closed with an install hint until those workspaces publish a release
  binary. The bot workspace currently builds its Tauri app as `dxbot`, which
  does not match the `dx-bot-<triple>` asset convention; the decisions
  workspace is C++ and Python tooling with no `dx-decisions` binary yet.
- Added: `dx grimoire` and `dx forkness` are registered and routed, and both
  fail closed. `essence-dx/grimoire` has no GitHub release
  (`gh release view` reports `release not found`) and `dx-grimoire` is not on
  the npm registry, so `dx tool install grimoire` fails with `API 404`.
  `essence-dx/forkness` **is** published — private, `main` tracking
  `origin/main` — but it has no release yet, so `dx tool install forkness`
  fails the same way until the house `dx-forkness-<triple>` assets exist.
  Both routes still find a locally built binary when one exists.
- Removed: `dx build`, `dx js`, `dx py`, and `dx agents` (plus the hidden
  `dx bundle` alias). They routed at the `build/`, `js/`, `py/`, and `agent/`
  trees, all of which are absent from this workspace, so every one of those
  commands failed closed at its workspace guard. `dx <script.js|ts>` and
  `dx <script.py>` direct script dispatch is unchanged.

## DX products

A product is a larger app or surface rather than a `dx <tool>` binary. Some
still expose a CLI route.

| Product | Folder | Repository | What it is | CLI route |
| --- | --- | --- | --- | --- |
| code | `code` | `essence-dx/code` | dx-code: GUI desktop code editor | `dx desktop sync-web` |
| tui | `tui` | `essence-dx/tui` | dx-tui terminal UI | none (hidden `dx tui` launches `cli/tui`) |
| bot | `bot` | `essencefromexistence/flow` | dx app for mobile and desktop | `dx bot` (fails closed until release) |
| decisions | `decisions` | `essence-dx/dx-decisions` | dx decision-model system (default STT: Phonon-2.5, 156 MB, 25 langs; default VLM: VisionPsy-Nano-460M-Flash, 256 MB Q4_0, 64 visual tokens, ~10-25x faster TTFT) | `dx decisions` (fails closed until release) |
| extensions | `extensions` | `essence-dx/extensions` | DX Extensions — the DX fork of the Cline agent stack; `apps/dx-vscode` ships as the DX Command Center | none |
| website | `website` | `essence-dx/website` | dx official website | `dx site` (hidden) |
| codebase | `codebase` | `essence-dx/codebase` | The code knowledge graph for coding agents. A **fork** of `DeusData/codebase-memory-mcp` (MIT), upstream kept as the `upstream` remote; the DX layer is the decision record in `codebase/TECH.md` and the ledger in `codebase/TODO.md` | none yet — no `dx codebase` route and no release (`codebase/TODO.md` `CB-01`, `CB-02`) |

`codebase` is a **fork, not a DX-authored tool**. The indexer, the store, the
Cypher engine, the 162 vendored grammars and CBM Lens are upstream's work under
MIT; the `essence-dx` organisation name is why the slug carries no second `dx`
prefix. The repository is public and forks `DeusData/codebase-memory-mcp` on
GitHub, so "Sync fork" and upstream pull requests both work.

What the fork adds is a **surface**: one typed tool (`dx_codebase`) whose schema
makes the agent draft a request type, review its own draft, and commit to it,
with a budget that **refuses** an over-budget answer rather than truncating it.
That layer is DX's own work and it is exercised by a built binary on this
machine, driven over real stdio.

The measured shape of it, from the built binary's own `tools/list`:

| Surface | Tools | Schema the agent receives |
| --- | --- | --- |
| upstream's full menu | 18 | 24,111 bytes |
| the DX surface | 1 | 3,182 bytes |

**86.8% less schema, and 2547 bytes carry the whole contract** - the four
required fields, the thirteen request types, the budget, and the per-type
required parameters. That is a *surface-size* measurement, not a session-level
token cost; the session measurement is `codebase/TODO.md` `CB-13` and has not
been run. The reason the surface exists at all is upstream's own figure: a
session with MCP attached costs **+66% tokens** against the same session
without it. The graph is upstream's; the surface is the fork's.

## Local models: OCR, vision, STT, TTS

The working models on this machine live in `G:\Dx\models`. They are not the
same set the `esportsfire` app pins in `esportsfire/models/manifest.json`: that
manifest still ships `dx-ocr-1.0` (PP-OCRv4) and `dx-vision-1.0`
(SmolVLM2-256M) as its cascade. The table below is the current local default,
what `decisions/docs/MODEL_STACK.md` (sections 3, 7, 8) locks, and what the
tests in this section actually loaded.

| Role | Default model | Files on disk | Size | Runtime |
| --- | --- | --- | ---: | --- |
| OCR | **PP-OCRv6 Nano** (`PP-OCRv6_tiny_det_onnx` + `PP-OCRv6_tiny_rec_onnx`) | `models/ocr/{det,rec}/inference.onnx` + `ocr/ppocrv6_keys.txt` | ~6.0 MB | RapidOCR over ONNX Runtime, CPU |
| Vision | **VisionPsy-Nano-460M-Flash** (`qvac/VisionPsy-Nano-460M-Flash`, Apache-2.0) | `models/vlm/` (`q4_0` + `mmproj` Q8; q4_k_m-imat also present) | ~348 MiB GGUF | llama.cpp with a VisionPsy-capable `mtmd` build |
| STT | **Phonon-2.5** (`FermionResearch/Phonon-2.5`) | `models/stt/Phonon-2.5/snapshots/*/phonon-2.5.fsq` | ~149 MB | Fermion, CPU |
| STT (hi) - locked | **whisper-base-hindi** (int8 CTranslate2) | `models/stt/whisper-base-hindi-ct2/` (fp32 snapshot beside it) | **76 MB** int8 / 585 MB fp32 | faster-whisper / CTranslate2 |
| TTS (en/ar/ja) | **Supertonic 3** (`Supertone/supertonic-3`) | `models/tts/Supertonic-3/onnx/*.onnx` + `voice_styles/` | ~383 MB | ONNX Runtime, CPU |
| TTS (bn) | **MMS-TTS Bangla** (`facebook/mms-tts-ben`) | `models/tts/MMS-TTS-Bangla/snapshots/*/model.safetensors` | ~139 MB | PyTorch (VITS), CPU |
| TTS (hi) - locked | **Piper hi_IN** (`pratham`, `priyamvada`, `rohan`) | `models/tts/piper-hi-*/` (ONNX + `tokens.txt` + `espeak-ng-data`) | **77 MB** per voice | sherpa-onnx VITS, CPU |

`G:\Dx\zzz\dx-vision-v1.0` (SmolVLM2-256M) is the **shipped fallback**, parked
outside the model store on 2026-10-30, not the preferred VLM: on the same UI
frame VisionPsy read all four text lines exactly in 6.93 s, while
`dx-vision-v1.0` described the page generically in 1.82 s and did not transcribe
the text. `PP-OCRv4` is retained only as the legacy fallback: it stays accurate
on UI chrome, but it is no longer the OCR default. The VisionPsy F32 snapshot
(1.89 GB) was removed on 2026-10-30; the GGUF stack is the only local copy and
the one the tests used.

### The five-language speech stack

Target languages are **English, Arabic, Japanese, Bangla, Hindi**; Chinese was
dropped. All five now have a working model in each direction; Hindi was the
last gap and is fixed (see below).

| Language | STT model | TTS model |
| --- | --- | --- |
| English | Phonon-2.5 (also 24 other European languages) | Supertonic 3 |
| Arabic | Dolphin CTC int8 (fast path) | Supertonic 3 |
| Japanese | Dolphin CTC int8 / SenseVoice-Small INT8 | Supertonic 3 |
| Bangla | Dolphin CTC int8 / Whisper base int8 | MMS-TTS Bangla |
| Hindi | **whisper-base-hindi int8** | **Piper hi_IN** |

Moonshine streaming tiny-ar is the documented Arabic specialist, but it is not
cached on this machine and was not exercised here; Dolphin covers Arabic on
disk today. Approximate speech footprint: **STT ~752 MB + 76 MB**, **TTS
~421-528 MB + 77 MB**.

### Hindi, fixed and measured (2026-10-30)

The stock multilingual `whisper-base` int8 produced romanized text or Urdu for
Hindi, which is why Hindi was previously reported as a gap. A Hindi fine-tune
fixes it:

| Hindi STT (int8, CPU) | Size | Mean CER | Median CER | RTF |
| --- | ---: | ---: | ---: | ---: |
| stock whisper-base (multilingual) | 152 MB | **1.015** | - | 0.13 |
| **whisper-base-hindi** (`collabora/`) | **76 MB** | **0.135** | **0.032** | **0.22** |
| whisper-small-hindi (`collabora/`) | 240 MB | 0.114 | 0.028 | 0.43 |

Measured on five FLEURS `hi_in` test clips (8-9 s each, real speech, reference
transcripts). The mean is pulled up by one hard proper-noun sentence; the
median is 3.2% CER. The small model gains 2 points of CER for 3x the size and
2x the time, so **whisper-base-hindi is locked** for Hindi and small stays as
the accuracy option.

For TTS, every candidate produced the reference sentence back at CER 0.000:

| Hindi TTS | Size | Sample rate | RTF | Round-trip CER |
| --- | ---: | ---: | ---: | ---: |
| **Piper `pratham`** (male) | **77 MB** | 22.05 kHz | **0.05** | **0.000** |
| Piper `priyamvada` (female) | 77 MB | 22.05 kHz | 0.07 | 0.000 |
| Piper `rohan` (male) | 77 MB | 22.05 kHz | 0.06 | 0.000 |
| MMS-TTS Hindi | 291 MB | 16 kHz | 0.27 | 0.000 |
| Supertonic 3 (previous) | 383 MB | 44.1 kHz | 0.30 | 0.067 |

Piper wins on size, sample rate and speed, so it is **locked** for Hindi:
`pratham` (male) is the default, `priyamvada` is the female voice, and `rohan`
is the alternate male. The earlier "Hindi is the weakest Supertonic voice" note
was a measurement error: the multilingual STT was romanizing the audio, not the
voice failing.

Both pins, with sha256 for every weight file and the measured CER/RTF, are in
`decisions/configs/speech_hindi.json` (commit `03b125a` in the `decisions`
repo). Re-check a pin with:

```powershell
Get-FileHash G:\Dx\models\stt\whisper-base-hindi-ct2\model.bin -Algorithm SHA256
```

### What was actually run (2026-10-30)

All four modalities were exercised on this machine; the scripts, commands and
raw numbers are in `docs/model-tests-2026-10-30.md`.

| Modality | Result | Evidence |
| --- | --- | --- |
| OCR | **works** - PP-OCRv6 Nano read a rendered UI screenshot 4/4 lines exactly in 0.51 s | `docs/model-tests-2026-10-30.md`, `docs/model-tests/test_ocr.py` |
| Vision | **works** - VisionPsy read the same text exactly through GGUF (q4_0 + mmproj Q8) and through `transformers` fp32 (5.6 t/s) | same, `vision_gguf.log` |
| STT | **5 of 5 languages** - English (Phonon) and Arabic/Japanese/Bangla (Dolphin); Hindi now via whisper-base-hindi at 0.032 median CER | same, `test_stt_roundtrip.py`, `bench_stt_hindi.py` |
| TTS | **works in all 5** - Supertonic 3 for en/ar/ja at 0.27-0.36 RTF, Piper for hi at 0.05-0.07, MMS Bangla at 0.47; the earlier "Hindi is weak" reading was a broken STT, corrected 2026-10-30 | same, `test_tts.py`, `gen_tts_hindi.py` |

Two defects surfaced and are recorded rather than smoothed over:

- VisionPsy's `config.json` enables `torch.compile`, which fails here because
  MSVC `cl` is absent. It runs with `TORCHDYNAMO_DISABLE=1` / eager mode; the
  GGUF path avoids the issue entirely.
- Hindi STT was the one gap. It is fixed: `whisper-base-hindi` (76 MB int8)
  replaced the stock multilingual model, whose output was romanized.

## Vendored third-party workspaces

These are real checkouts of other people's projects. They are **not** DX tools
and carry no `dx <tool>` route. Keep them out of the tool table above.

| Folder | Upstream remote | What it is | DX relationship |
| --- | --- | --- | --- |

*(No vendored checkouts remain at the time of writing. The two that were
compared on 2026-10-21 - `colbymchenry/codegraph` and
`DeusData/codebase-memory-mcp` - were resolved into the `codebase` product
above: the second became the fork, the first was removed. The comparison and
the deletion record are in `codebase/TECH.md` and `codebase/CHANGELOG.md`.)*

Three root clones were parked on 2026-10-20 because DX already carries the
same content, or because nothing referenced them:

- **`llama.cpp`** (the `PrismML-Eng` Prism fork) is now at
  `DEPRICATED_10-20-2026/llama.cpp`. Its working tree was already gutted at
  that point: 129 files on disk against 3445 tracked files showing as deleted,
  and no `Cargo.toml` in the hub path-depends on it. The `tpu/` kernels clone
  their own `llama.cpp` inside their Kaggle/Colab run directory
  (`tpu/llamacpp-cached-kernel/kernel.py:220-230`), so parking it breaks nothing.
- **`superpowers`** is now at `DEPRICATED_10-20-2026/superpowers`. It was the
  upstream `obra/superpowers` clone at `bb92a77` (v7.0.0), which is the
  **exact commit** already vendored into DX at
  `driven/vendor/superpowers/` — 97 files tracked in the `driven` repo, at
  version 7.0.0 commit `bb92a77` per `driven/vendor/superpowers/NOTICE.md`.
  The root clone was a duplicate, and `driven` is the live copy.
- **`magnitude`** and **`treg`** are now at
  `DEPRICATED_10-20-2026/magnitude` and
  `DEPRICATED_10-20-2026/treg`. Both stayed intact as clean git clones with
  their own `origin` (`magnitudedev/magnitude` at `b2f79f0`,
  `superdesigndev/treg` at `e67d0d46`), so either can be re-cloned. Nothing in
  the hub referenced them: no `Cargo.toml` or `package.json` path-depends on
  either, and neither had a `dx` route.
  - `magnitude` is a Bun/TypeScript monorepo with nested Rust crates (1627
    `.rs` files) that compiles and tunes inference kernels on the host device.
    Its own published benchmarks claim up to 2x llama.cpp (92% faster decode
    on Metal, 19% on CUDA) — **their** numbers, not measured here. It is a
    GPU/CPU engine, so it cannot advance the TPU-only Bonsai track
    (`AGENTS.md` section 2, rule 7).
  - `treg` is the `tools-registry` Python package (v0.26.0) plus a React
    frontend: a registry and credential-injecting proxy that lets an agent
    call thousands of priced third-party API endpoints and vendor CLIs from
    one base URL and one token. Its licence is **Apache-2.0 with an added
    hosted-service restriction** — free to self-host for your own team, not
    free to run as a competing hosted service. It is Superdesign's product,
    the same house behind the already-vendored `superpowers`.

## Bonsai 2 27B on a TPU (measured)

The `tpu/` track runs **Bonsai 2 27B** (the ~6 GB PTQ1_0 ternary GGUF) end to
end on a **Colab TPU v5e-1** with every ternary projection served by a fused
Pallas kernel that unpacks the packed weights in place -- llama.cpp has no TPU
backend, so it is a from-scratch JAX forward pass. All figures are aggregate
decode throughput across the batch, each with a receipt in `tpu/`:

| Run, 2026-10-18 (`[exec]` on v5e-1) | Result | Receipt |
| --- | --- | --- |
| Batch 80, 32 tokens generated | **281.01 tok/s** | `tpu/lane-aligned-ternary-kernel/receipt-b80-f16-tc1024.json` |
| Batch 80, 19-token prompt, 64 generated | 268.3 tok/s | `tpu/lane-aligned-ternary-kernel/receipt-prompt-b80-f16-tc1024.json` |
| Batch 64, 128 generated | 207.1 tok/s | `tpu/lane-aligned-ternary-kernel/receipt-prompt-b64-f16-tc1024.json` |
| Baseline before the layout fix | 29.66 tok/s | `tpu/11-fully-fused-64-layer-model/receipt-dec-cat-int16-tc512.json` |

The 9.5x came from storing the packed tiles so the out-columns sit on the
TPU's 128-lane axis (`x @ w` over `(32, rows)`) instead of the 2-16-wide
in-dim axis; the decoded weights are bit-identical. 281.01 tok/s is the best
receipted aggregate, ~2.8x the 100 tok/s goal; it is not a per-stream rate
(80 sequences share one v5e-1 core). Start at `tpu/README.md`; settled facts
and the do-not-retest list live in `tpu/DO-NOT-RETEST.md`.

**Two rates, not one.** The table above is aggregate: 80 conversations
advancing in lockstep. The number that matters for latency is one
conversation, and it was measured on 2026-10-19 (`[exec]` on v5e-1):

| Single stream (batch 1), 2026-10-19 | Result | Receipt |
| --- | --- | --- |
| 8k window, 64 generated, 64-layer step fused into one jit | **5.54 tok/s** (180.6 ms/step) | `tpu/lane-aligned-ternary-kernel/receipt-b1-f16-tc1024-fused.json` |
| the same run before the fusion | 4.62 tok/s (216.3 ms/step) | `tpu/lane-aligned-ternary-kernel/receipt-b1-f16-tc1024-franch.json` |
| 45k window -- the largest fp32 window that runs at all | 2.86 tok/s | `tpu/lane-aligned-ternary-kernel/receipt-b1-f16-tc1024-45056.json` |
| 49152 and 65536 windows | out of memory (140 MB and 3.1 GB short) | `tpu/lane-aligned-ternary-kernel/receipt-mem-ceiling.json` |

Why single-stream is slow: 80 sequences cost only 1.58x one sequence
(180.6 ms/step against 284.7 ms/step), so the step is ~85 % per-projection
launch overhead across ~400 projections, not bandwidth -- the weight traffic
floor is 9.0 ms. Reaching **50 tok/s for one stream** therefore needs
multi-token verification, not tuning; the plan, with evidence and a `Done
when` per item, is in `TODO.md` (`GPU-01`...`TPU-06`). The route that already
carries a number is MTP speculative decoding, measured at 50.44 tok/s on the
reference 27B dense checkpoint by the repo's own Kaggle T4 work
(`tpu/gpu-mtp-head-availability-test/kernel.py`).

## Client and adjacent projects (not DX)

These folders sit in `G:\Dx` but are **not** DX tools, DX products, or DX
releases. They are client work and adjacent projects. The hub does not own
their CI, their releases, or their roadmaps.

| Folder | Repository | Status |
| --- | --- | --- |
| `dhakaraiment` | `manfromexistence/dhakaraiment` | Client work. Dhaka Raiment garment-house marketing site and browser CMS, Next.js 16 / Bun / Tailwind v4, media on Catbox. It will be relocated out of `G:\Dx` by its owner; left in place and untouched until then. |
| `esportsfire` | `manfromexistence/esportsfire` | Client-adjacent product. Live Free Fire matches, ranked ladders, and open tournaments for the South Asian scene. Next.js + Tauri v2 + Bun. Not in the DX register. |
| `ikra` | *none* | Client-adjacent product. Global Islamic LMS with super-admin / teacher / student / guardian roles, Next.js + Bun + Tailwind v4 + shadcn/ui, on a Drizzle/libSQL database with Playwright browser tests. **This folder was the `lms/` scaffold until 2026-10-20, when it was renamed to `ikra`;** the earlier no-database `ikra` prototype is parked at `DEPRICATED_10-20-2026/ikra`. Not a git repo. Not in the DX register. |
| `bhcrd` | *none* | Client research scratch. Kaggle notebooks, GPU probes, and the BPS2025 Bangla handwritten character recognition dataset for a `hwcr` model. No DX linkage. |

## Hub infrastructure

| Path | Job |
| --- | --- |
| `dx` | The TOML-shaped routing manifest: `workspace`, `paths`, `tools`, `dx_www`, `media`, `tui` blocks. Several declared paths are dangling; confirm with `Test-Path` before acting on any of them. |
| `tools/bin/` | Staged local binaries and the per-triple release builds. `dx.exe` and `dx.cmd` are the normal Windows entrypoints. |
| `docs/` | The achievement register: one page per folder plus `docs/README.md` rules. |
| `.dx/` | Hub runtime state and receipts. |
| `DEPRICATED_10-20-2026/` | Parking lot for retired folders, repos, and root junk (49 entries on 2026-10-20: 34 git repos and 15 plain folders). Nothing in it is deleted; it is history kept out of the way. |
| `AGENTS.md` | The operating manual for this hub. Read it first. |
| `TODO.md`, `CHANGELOG.md`, `DX.md`, `PROMPTS.md` | Open work, dated change log, worker notes, prompts. |
| `build.ps1` | Hub build entrypoint. |
| `.gitignore` | Deny-everything (`/*`) with an allow-list. A new root file needs its `!/name` line or git cannot see it. |

## Parked folders

These folders used to sit at the hub root and were moved into the parking lot
(moves, not deletions) during the 2026-10-09 and 2026-10-20 cleanups:

| Was at root | Now parked as | Parked on |
| --- | --- | --- |
| `.vscode`, `.workbuddy-ai`, `benchmark`, `test`, `zzz`, `pi-test`, `channels`, `exa` | `DEPRICATED_10-20-2026\<same name>` | 2026-10-09 |
| `native` | `DEPRICATED_10-20-2026\native-2026-10-09` | 2026-10-09 |
| `mobile` | `DEPRICATED_10-20-2026\mobile-DEPRECATED-20261009-1616` | 2026-10-09 |
| `llama.cpp` | `DEPRICATED_10-20-2026\llama.cpp` | 2026-10-20 |
| `superpowers` | `DEPRICATED_10-20-2026\superpowers` | 2026-10-20 |
| `magnitude` | `DEPRICATED_10-20-2026\magnitude` | 2026-10-20 |
| `treg` | `DEPRICATED_10-20-2026\treg` | 2026-10-20 |
| `ikra` | `DEPRICATED_10-20-2026\ikra` | 2026-10-20 |
| `tunnel` | `DEPRICATED_10-20-2026\tunnel` | 2026-10-20 |
| `ZZZ-bot-trash` | `DEPRICATED_10-20-2026\ZZZ-bot-trash` | 2026-10-20 |
| `Frieren Beyon Journey's End.mp4`, `One Piece.mp4`, `Spiderman Into The SpiderVerse.mp4`, `big146.bin`, `Social.md`, `small.gz`, `$o`, `db`, `db-wal`, `TODO-ESSENCE.md`, `package.json`, `fx.cmd`, `runner.sh`, `mc5_two.py`, `mc5_three.py`, `dataset-metadata.json`, `bonsai2-27b-t4-bench-api.ipynb`, `dx-typesafe-golden-fit-probe.ipynb`, `image.png`, `gguf_prism_report.json` | `DEPRICATED_10-20-2026\root-junk\<same name>` | 2026-10-20 |

The parking lot was named `DEPRICATED_9-17-2026` and was renamed to
`DEPRICATED_10-20-2026` on 2026-10-20, when the second batch was added. The
older items keep their original 2026-10-09 park date in the table above.

`mobile` was superseded by `bot`, which is the dx app for both mobile and
desktop. `native` was the C++ renderer workspace that `dx native` and
`dx ecosystem status` read; with the folder parked those surfaces report the
path as missing until the config path is repointed or `dx native` is retired.

## Environment facts

Measured on 2026-10-09 with `Get-PSDrive G`: `G:` is a ~477 GB volume with
~23 GB free. The `essence-dx/*` repositories are private, so anonymous GitHub
API calls return 404 and a full `cargo` build can fail with 401 on the
private `check` dependency (`cli/DX.md`). The hub is Windows (`win32`) with
PowerShell as the shell.

## Source of truth

| Question | Answer lives in |
| --- | --- |
| Where does tool X live on disk? | the `dx` manifest, `paths` block |
| Which repo publishes binary X, under what filename? | `TOOL_REGISTRY` in `cli/src/update_registry.rs` |
| What has folder X achieved? | `docs/<x>.md`, then the `docs/README.md` register |
| What is open or blocked? | `TODO.md` (hub), the folder's own ledger (products) |
| What changed most recently? | `CHANGELOG.md` (hub), `git log` (products) |
| How do I work here without breaking it? | `AGENTS.md` |

MIT OR Apache-2.0
updated: 2026-10-30 00:00 UTC
