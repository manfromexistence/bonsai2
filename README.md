# Kaggle access and Bonsai 2 notebooks

This is the operating guide for any agent that needs to use Kaggle from this
machine. It covers the credentials in `G:\Ai`, the datasets the kernels mount,
the kernels under `G:\Dx\tpu`, and the measured numbers that must not be
re-derived or mis-quoted.

Secrets are referenced by path only. The token values are never written here.

---

## 1. Accounts and credentials

| Item | Value |
| --- | --- |
| Kaggle account | `essencefromexistence` (access-token auth) |
| CLI | `G:\Ai\.venv\Scripts\kaggle.exe` — 2.2.4 (2.2.5 available) |
| Config dir | `G:\Ai\.secrets\kaggle` |
| Token file | `G:\Ai\.secrets\kaggle\env.ps1` — sets `KAGGLE_API_TOKEN` |
| Disabled old token | `G:\Ai\.secrets\kaggle\disabled\old-api-token-env-20260608-223805.ps1` |
| Convenience copy | `G:\Dx\tpu\env.ps1` — same token, gitignored via `/tpu/env.ps1` |
| Auth check | `G:\Ai\scripts\kaggle-auth-check.ps1` |

The token file has this shape; the value is deliberately not reproduced:


```powershell
$env:KAGGLE_API_TOKEN = 'KGAT_<redacted>'
```

The CLI is **not on PATH**. Load the environment first:

```powershell
. G:\Ai\scripts\env.ps1
kaggle config view
kaggle kernels list --mine --page-size 50
```

`G:\Ai\scripts\env.ps1` also sets `KAGGLE_CONFIG_DIR`, `HF_HOME`, `TEMP`, and
the `G:`-only Python environment. `G:\Ai\README.md` still says the account is
`manfromexistence` and that auth is "not stored yet" — both are stale. The live
account is `essencefromexistence`, verified 2026-11-05.

**Never** commit `env.ps1`, paste the token into a kernel, notebook, log, issue,
or chat message. The hub ignore file already excludes `/tpu/env.ps1`; keep it
that way.

---

## 2. Kaggle datasets

| Dataset | Contents | Size |
| --- | --- | --- |
| `essencefromexistence/bonsai2-27b-ptq1` | `Ternary-Bonsai-2-27B-PTQ1_0.gguf` | 5,946,648,928 B (~5.95 GB) |
| `essencefromexistence/bonsai2-llamacpp-build` | Cached `llama.cpp/build/bin` binaries (CUDA `libggml-cuda.so.0.21.0`, `llama-cli`, `llama-server`, `llama-bench`) | ~82 MB |

Inspect them without downloading:

```powershell
. G:\Ai\scripts\env.ps1
kaggle datasets files essencefromexistence/bonsai2-27b-ptq1
kaggle datasets files essencefromexistence/bonsai2-llamacpp-build
```

`PQ2_0` (7.2 GB) is **not** staged as a Kaggle dataset. If a kernel needs it,
download `Ternary-Bonsai-2-27B-PQ2_0.gguf` from
`prism-ml/Ternary-Bonsai-2-27B-gguf` inside the kernel (internet is enabled).

---

## 3. Kaggle kernels under `G:\Dx\tpu`

Each folder owns `kernel.py` as the source of truth, plus `make_notebook.py`,
`notebook.ipynb`, and `kernel-metadata.json`. After editing `kernel.py`, run
`python make_notebook.py` and `python make_notebook.py --check` in that folder.

### 3.1 GPU kernels — `tpu/out-of-scope-gpu/`

These are **out of scope for the TPU track** (`AGENTS.md` section 2 rule 7,
`tpu/README.md` section 7). They are kept for the record. Do not run one
without the user's explicit go-ahead.

| Folder | Kaggle id | Status (2026-11-05) | What it does |
| --- | --- | --- | --- |
| `gpu-bonsai-4way-tuned` | `essencefromexistence/bonsai2-4way-tuned` | CANCEL_ACKNOWLEDGED | Four Bonsai 2 instances, two per T4, four cloudflared tunnels |
| `gpu-llamacpp-t4-baseline` | `essencefromexistence/bonsai2-27b-t4-bench-api` | ERROR | Builds the fork, benches PTQ1_0, serves an OpenAI-compatible API |
| `gpu-draft-model-speculation-test` | `essencefromexistence/bonsai2-spec-draft` | ERROR | Qwen3.5-0.8B Q4_0 as a speculative draft |
| `gpu-draft-model-two-t4-test` | `essencefromexistence/bonsai2-spec-2gpu` | ERROR | Draft test across both T4s |
| `gpu-mtp-head-availability-test` | `essencefromexistence/bonsai2-mtp-test` | **COMPLETE** | MTP availability + same-session baseline |
| `gpu-mtp-head-graft-test` | `essencefromexistence/bonsai2-mtp-graft` | ERROR | MTP head graft experiment |
| `gpu-deepseek-v4.1-flash-t4` | `essencefromexistence/deepseek-v41-flash-t4` | not run | Recorded as INFEASIBLE on Kaggle |

The only complete GPU receipt is `bonsai2-mtp-test`:

```text
mtp-receipt.json
  model            Ternary-Bonsai-2-27B-PTQ1_0.gguf (5946648928 B)
  accelerator      T4 (1x)
  context          8192
  baseline         26.2 tok/s generation / 72.7 tok/s prompt
  draft_mtp        exit 1 -- file has no MTP head
  file_has_mtp_head false
  measured_utc     2026-10-20T14:58:18Z
```

Fetch it with:

```powershell
. G:\Ai\scripts\env.ps1
kaggle kernels output essencefromexistence/bonsai2-mtp-test -p G:\Temp\UserTemp\opencode\bonsai-mtp-test-out
```

### 3.2 TPU kernel — `tpu/kaggle-qwen38-tpu-130toks/`

| Item | Value |
| --- | --- |
| Kaggle id | `rahim3/qwen3-8-27b-bf16-on-kaggle-tpu-130-tok-s-api` |
| Accelerator | `TpuV5E8` (TPU, not GPU) |
| Result | ~130 tok/s single stream, ~540 tok/s at 8 streams (per its metadata/readme) |
| Datasets | `rahim3/qwen3-8-27b-bf16`, `rahim3/qwen38-tpu-env-v5e8` |

Note the owner is `rahim3`, not `essencefromexistence`.

### 3.3 TPU lab — `tpu/kaggle-tpu-lab/`

A **separate git repo** (`ARahim3/kaggle-tpu-lab`) with run-all Kaggle TPU
notebooks for Qwen3.8-27B (~130 tok/s) and GLM-5.3-Flash (~165 tok/s), plus
`launch.py` (push / status / stop). It is not part of the hub index.

### 3.4 Colab TPU kernels — the live track

These are **Colab** TPU v5e-1 kernels, not Kaggle. They are the actual track:
`lane-aligned-ternary-kernel/`, `multi-token-verification/`, `dflash2-draft/`,
`projection-cost-breakdown/`, `pallas-matmul-findings/`,
`reference-forward-pass/`, `ternary-2bit-weight-format/`,
`colab-auth-timeout-patch/`. See `tpu/README.md` for the runner recipes.

### 3.5 Archive

`tpu/archive/` holds superseded Kaggle generations
(`llamacpp-{baseline,fast,cached,four-model-comparison,tunnel}-kernel`, etc.).
Each carries its own receipt. Do not treat them as the current setup.

---

## 4. Pushing and pulling a kernel

```powershell
. G:\Ai\scripts\env.ps1

cd G:\Dx\tpu\out-of-scope-gpu\gpu-bonsai-4way-tuned
python make_notebook.py --check
kaggle kernels push -p . --accelerator NvidiaTeslaT4x2

kaggle kernels status essencefromexistence/bonsai2-4way-tuned
kaggle quota                     # remaining GPU hours, resets Mondays
kaggle kernels output essencefromexistence/bonsai2-4way-tuned -p G:\Temp\UserTemp\opencode\bonsai-4way-out
```

Notes that have already caused failures:

- The `id` in `kernel-metadata.json` is the **resolved slug**. Kaggle slugifies
  dots on first push; pushing the dotted name creates a second kernel.
- `--accelerator` is forwarded verbatim. Accepted values include
  `NvidiaTeslaT4`, `NvidiaTeslaT4x2`, `NvidiaTeslaP100`, `TpuV5E8`.
- Kaggle gives **30 GPU-hours/week** and a **12 h** session cap (CPU/GPU;
  TPU is ~9 h). T4x2 burns quota at 2x wall-clock.
- `kaggle kernels output` downloads the whole `/kaggle/working`, which can
  include a ~90 MB build tree. It can take longer than the default timeout.

---

## 5. Ground truth — do not re-derive or mis-quote

| Fact | Value | Evidence |
| --- | --- | --- |
| Decode, 1x T4, PTQ1_0, c=8192 | 26.2 tok/s | `bonsai2-mtp-test` `mtp-receipt.json` |
| Prompt processing, same run | 72.7 tok/s | same receipt |
| 2x T4 tensor/row split | **Broken** for this packing; pin to one card | PrismML `KNOWN_ISSUES.md`, #213 |
| MTP on PTQ1_0 | No MTP head in the file | `file_has_mtp_head: false` |
| Working fork | `sudoingX/llama.cpp` branch `bonsai2` (commit `ff41412`, 2026-09-27) | `gpu-llamacpp-t4-baseline/kernel.py:83-88` |
| Newer upstream | `PrismML-Eng/llama.cpp` `prism` (2026-11-02) adds GDN / FWHT / PQ2 / PTQ1 CUDA fusions | repo commits |
| Bonsai 2 base model | Ternary quantization of `Qwen/Qwen3.8-27B` | HF model card |
| Qwen3.8-27B DeepSWE 1.1 | 42.2 (vendor-reported) | Qwen model card |

The 26.2 tok/s figure is a **1x T4** measurement, not a 2x T4 split. The
2x T4 label exists because Kaggle exposes two cards, but the split path is
unusable, so kernels pin `CUDA_VISIBLE_DEVICES` and use one card per instance.

`pinkmanwest/llama-q1-turing` is a real Turing patch, but it is **Q1_0 only**
(binary +1/-1, Bonsai-27B), not ternary `PTQ1_0`/`PQ2_0`. It does not apply to
Bonsai 2 27B as-is.

---

## 6. Rules for a new agent

1. Read `G:\Dx\AGENTS.md` first. This file only extends it for Kaggle.
2. Load credentials with `. G:\Ai\scripts\env.ps1`; never hard-code the token.
3. Do not run a GPU kernel on this track without the user's explicit go-ahead
   (`AGENTS.md` section 2 rule 7). A GPU number is not a TPU result.
4. Do not write a bare speed claim. Every number names its receipt or it is
   not a result (`AGENTS.md` section 4).
5. The hub ignore file is deny-everything. A new root file needs its
   `!/name` line in `.gitignore` in the same commit.
6. Keep product work in the product repo. `tpu/` is tracked in this hub;
   `tpu/kaggle-tpu-lab/` is its own repo.

---

MIT OR Apache-2.0
updated: 2026-11-05 00:00 UTC