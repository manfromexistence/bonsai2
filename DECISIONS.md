# DECISIONS.md — What we are doing, and where it stands

**Audience:** the next AI agent (or human) picking up this work.
**Scope:** the `decisions` product (`G:\Dx\decisions`, repo `essence-dx/dx-typesafe`)
and its browser surface `apps/www`. The hub itself (`G:\Dx`) owns only this file,
`README.md`, `TODO.md`, etc. — product work is committed in the child repo, never here.

**Status:** renames executed in the product repo; browser feasibility analysed but
not built; the `esportsfire` sync is **done** (2026-10-24, §4).

---

## 1. What the product is

`dx-decisions` is a local-first decision engine ("System One decision model"). One
engine, six capabilities: decision, speech in, vision in, video in, speech out,
trainability. Live stack is authoritative in
`G:\Dx\decisions\docs\MODEL_STACK.md` — read it before touching any model.

Shipped experts (renamed 2026-10-24, see §3):

| expert | what it is | licence |
|---|---|---|
| `dx-decisions-v1.0` | ettin-reranker-17m-v1, 16.86M params, 67.6 MB ONNX | Apache-2.0 |
| `dx-base-v1.0` | pure upstream base, training default | Apache-2.0 |
| `dx-ocr-v1.0` | PP-OCRv4, 3 ONNX graphs, ~16 MB | Apache-2.0 |
| `dx-vision-v1.0` | SmolVLM2-256M-Video-Instruct (captioner, 256M) | Apache-2.0 |
| `dx-asr-v1.0` | Phonon-2 (ASR) | (see stack page) |
| `dx-tts-v1.0` | kokoro-82M (TTS) | (see stack page) |
| `dx-reasoning-v1.0` | LFM2.5-230M-QAD-Q4_0, 229.7M, 149.1 MB GGUF, **fallback only** | **LFM-1.0** (vendor) |

The browser surface is `G:\Dx\decisions\apps/www` (Next.js 16, untracked in git:
`git status -sb` reports `?? apps/`). It runs models client-side with
`onnxruntime-web` (WASM). Today it ships text (decision), image (OCR), TTS (kokoro);
video and audio axes are `ready: false`.

---

## 2. What we did (2026-10-24)

### A. Renamed the two unnamed models

- **LFM2.5-230M-QAD-Q4_0 → `dx-reasoning-v1.0`** (reasoning fallback, Backend 2).
  Executed: GGUF file renamed on disk, `src/config.cpp:36` and
  `tools/llama_prefix_bench.cpp:89` paths updated, `configs/backend2_decoder.json`
  updated, `scripts/dev/media_relay.py` updated. g++ syntax check clean.
- **SmolVLM2-256M-Video-Instruct → `dx-vision-v1.0`** (captioner). Directory and
  its two GGUFs renamed; `media_relay.py` paths/filenames updated; py_compile clean.

### B. Updated all live docs

`docs/MODEL.md`, `docs/MODEL_STACK.md`, `docs/CLI.md`, `WASM.md`, plus
AGENTS.md, CAPABILITIES.md, CLAIMS.md, HIDDEN_GEMS.md, README.md, RESULTS.md,
STATUS.md, VERDICT.md, JEVBENCHES.md, BENCHES.md, docs/dx-decisions.md,
docs/HARDWARE.md, docs/MODEL_PICK.md. Upstream HF ids kept as provenance in
parens (naming policy in `docs/MODEL.md` §naming). Session logs, receipts,
CHANGELOG, eval JSON left untouched (historical).

### C. Wrote `G:\Dx\decisions\WASM.md`

Full browser-feasibility analysis. Key conclusions:

- **Both models run in llama.cpp natively and in the bot's wllama browser.**
  LFM2.5 is arch `lfm2`, present in wllama's pinned llama.cpp (commit `5aeac37`);
  SmolVLM2 is arch `llama` + `clip` mmproj, wllama v3 supports it via
  `loadModelFromUrl({ url, mmprojUrl })` and image content parts.
- Captioner route A (recommended): ONNX + WebGPU via `transformers.js`,
  ~189 MB (q4f16). Route B (GGUF+mmproj) ~235 MB.
- Video in browser is **not** a solved path (transformers.js issue #1450 open);
  use sampled frames.
- LFM Open License v1.0: commercial use gated at $10M revenue (§5); redistribution
  needs licence copy + NOTICE (§4a/§4d). Apache-2.0 models have no such condition.
- Cross-origin isolation trap (bot already paid for it): port `bot/src/lib/ai/wllama-build.ts`
  build selection first; headers second.
- Gates green: `check_doc_refs.py` (3616 resolve) and `check_doc_numbers.py` (56 agree).

### D. Did NOT do

- The `dx-vision` name collision was resolved as (b2) and **is** now applied in
  `esportsfire` (§4) — but it is still unconfirmed by the owner.
- No git commits were made in `decisions`.

---

## 3. The open decision: dx-vision name collision

`docs/MODEL.md:104` originally defined `dx-vision-v1.0` = PP-OCRv4. The owner asked
that the captioner (SmolVLM2) take that name. One name cannot mean two models.

- **(b1)** keep `dx-vision-v1.0` for OCR, give the captioner a new name
  (`dx-caption-v1.0` / `dx-vision-language-v1.0`).
- **(b2)** OCR becomes `dx-ocr-v1.0`, `dx-vision-v1.0` moves to the captioner.
  (Assumed; `WASM.md` and the docs use this.)

**Confirm which before the esportsfire sync, because that project's `models/manifest.json`
and code reference these names.**

---

## 4. `esportsfire` sync — done 2026-10-24

`esportsfire` had `models/{dx-decisions-1.0, dx-ocr-1.0, dx-reasoning-1.0,
dx-vision-1.0}` plus a `manifest.json` (generated 2026-10-18, 598 MiB total). Its
copies were **stale** — they predated the rename. All four items are now closed:

| was | is | verified |
|---|---|---|
| `dx-reasoning-1.0/LFM2.5-230M-QAD-Q4_0.gguf` | `dx-reasoning-1.0/dx-reasoning-v1.0.gguf` | 149,081,056 B, `e75f8326…` — same as `.models/decoder/dx-reasoning-1.0.gguf` |
| `dx-vision-1.0/SmolVLM2-…-Q4_K_M.gguf` | `dx-vision-1.0/dx-vision-v1.0-Q4_K_M.gguf` | 131,234,176 B, `1d86050b…` |
| `dx-vision-1.0/mmproj-SmolVLM2-…-f16.gguf` | `dx-vision-1.0/dx-vision-v1.0-mmproj-f16.gguf` | 190,033,440 B, `819d4b38…` |
| `manifest.json` sha256/bytes for renamed files | unchanged values, new `name`s; `total_bytes` 626,879,638 → **626,892,746** | `bun run models:check` → 11 files, 594.1 MiB, all match |

- **`dx-reasoning-1.0.gguf` was not the name chosen.** The source file in
  `.models/decoder/` is `dx-reasoning-1.0.gguf`, but the published expert is
  `dx-reasoning-v1.0` (`docs/MODEL.md` expert table) and the vision files are
  `dx-vision-v1.0-*`. The file inside the `dx-reasoning-1.0/` folder therefore
  carries the `-v1.0` form; `name_note` in the manifest says so, because a
  filename-blind re-copy from `.models/decoder/` would silently rename it back.
- **`model.onnx` was left alone, correctly.** `esportsfire` holds 67,634,313 B /
  `817e900f…` = the **base** checkpoint. `decisions/apps/www/public/models/model.onnx`
  is 67,634,071 B / `9eda56ee…` = the **merged** one. The 242 B delta is the whole
  tell; the manifest's `collision_note` now carries both numbers so the next sync
  does not "fix" it into the wrong model.
- **`ppocrv4_chars.json` was left alone too.** 59,388 B here vs 46,135 B in
  `decisions` — serialisation only. Both parse to the identical 6,625-entry list in
  the same order (checked with `json.load`, `a == b` → `True`), and
  `ppocr_keys.txt` is the manifest's declared 6,623 lines.
- **How `esportsfire` loads them:** entirely through `models/manifest.json` —
  `manifest::EMBEDDED_MANIFEST` is `include_str!` of the same file, so there is no
  second copy to keep in step. No TS/RS file hardcodes a GGUF name. The one place
  the rename actually broke was the vision stage's projector picker
  (`pipeline.rs`: `starts_with("mmproj")` → `contains("mmproj")`, case-insensitive);
  without it `SubprocessVision::spawn` refuses with "manifest pins no `mmproj` file"
  while the renamed projector sits in the folder.
- **Owed, not local:** `ModelFetcher.kt` fetches `${baseUrl}/${name}` with `name`
  from the manifest, so a device build now asks for the new filenames. Anything
  already uploaded under the old names needs re-uploading first.
- Committed in `esportsfire` (product repo), not here: `models/manifest.json`,
  `src-tauri/src/models/pipeline.rs`, `CHANGELOG.md`.

Run the decisions repo gates after any doc edit there:
`ctest --test-dir build` (21/22, one red by design), `.venv-vision\Scripts\python.exe
scripts\dev\check_doc_refs.py`, and `check_doc_numbers.py`.

---

## 5. Key facts to not re-derive

- Native speed: LFM2.5 tg_median **140.92 t/s** (six runs 125.52–151.42) on Ryzen 5
  5600G, from `eval/hidden_gem_gate10.json`. Browser speed is **unmeasured** — do not
  quote a number.
- Decision model: 0.881 noul / 0.759 route on the 273-row diverse set
  (`docs/dx-decisions.md` §7d); gate is RED on route (0.759 < 0.85) by design.
- Encoder is default, not the decoder. "Never generate" is the design.
- `dx_eval_harness_gate` is red by design (0.72 noul vs 0.85 threshold). Do not
  "fix" it.
- `G:` is ~97% full. Propose before deleting anything (hub AGENTS.md §2.2).
- `decisions/` and `esportsfire/` are separate git repos. Commit product work there,
  not in the hub. Hub commits carry hub artifacts only.

---

## 6. Files created/changed this session (hub)

- `G:\Dx\DECISIONS.md` (this file) — new; allow-listed in `.gitignore` as
  `!/DECISIONS.md`.
- `G:\Dx\.gitignore` — added `!/DECISIONS.md`.

## 7. What NOT to do

- Do not rename `dx-vision-v1.0` without resolving the collision in §3.
- Do not ship the f16 mmproj (190 MB) in a browser; use the Q8_0 (103.8 MB).
- Do not quote GPU/native numbers as browser numbers.
- Do not commit `apps/` or `.models/` (gitignored).
- Do not delete anything to free space. Propose, with size and evidence, then wait.

MIT OR Apache-2.0
updated: 2026-10-24 00:00 UTC
