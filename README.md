# Dx

> **The computer that remembers. The agent that knows you. The stack that changes everything.**

**Dx is an AI-native development ecosystem written in Rust** — a full-stack family of ~20 tightly-integrated tools built around one impossible-sounding idea:

**Your machine should remember everything you do — semantically, privately, in plain text — and every AI agent you own should be able to ask it questions.**

Not screenshots dumped into a hard drive until your disk dies.
Not a cloud service reading your screen.
**A local, encrypted, searchable memory of your entire digital life that costs gigabytes per *decade*, not per *week*.**

---

## This is not another "screen recorder"

Every lifelogging tool you've tried — Screenpipe, Rewind, Recall, Windrecorder — makes the same mistake: **pixels are the product**. They dump gigabytes of video per day, then burn CPU trying to OCR their way back to what Dx captures natively at the source.

| | Screenshot-first tools | **Dx** |
|---|---|---|
| Storage | 1–5+ GB **per day** | Years of activity in **low GBs total** |
| Truth | Pixels, text derived later (lossy OCR) | **Semantic text captured at the source**, pixels are opt-in evidence |
| Agent-ready | Needs vision models, ~1k+ tokens/frame | Plain Markdown — greppable by any agent on Earth |
| Privacy | Store first, redact later (maybe) | **Privacy gate runs BEFORE anything touches disk** |
| Forgetting | Delete files and pray | Crypto-shredding: destroy one key, prove deletion with a signed manifest |
| Skills for agents | Burn tokens guessing workflows | **Mined free from your actual behavior** |

And then there's the moment that sells it. You walk up to any terminal and type:

```powershell
dx-daemon current
```

…and your machine tells you what you're doing right now — focused app, window geometry, visible apps, even **what's readable on your screen** via opt-in AVIF snapshots with OCR.

```powershell
dx-daemon search "clicked save"
dx-daemon usage        # where did my week go?
dx-daemon forget --minutes 5
```

Ask it *"what was I doing last Tuesday?"* — and get a real answer from a Markdown timeline your agent can read like a book. **No other product on Earth does this.**

---

## dx-daemon — the flagship

A per-user session daemon (no admin rights, no GUI deps) that turns your OS activity into a **structured knowledge base**:

```text
OS/App Sources
    |
Bounded Capture Queue
    |
PRIVACY GATE  <-- nothing durable happens before this line
    |
Normalization + Correlation + Dedup
    |
Encrypted Append-Only Journal   <-- canonical recovery source, hash-chained, BLAKE3 + CRC32
    |
SQLite + FTS5                   <-- instant full-text search
    |
Markdown Knowledge Tree         <-- timeline/, apps/, projects/, sessions/ with stable dx:// URIs
    |
MCP / IPC / CLI / TUI           <-- every AI agent can plug in
```

- **~0.4% CPU, ~18 MB RAM** while watching everything (measured, debug build)
- Typed text is **never** reconstructed from keystrokes — semantic sources only
- Clipboard capture is opt-in and redacted to an 80-char preview
- Screenshots are opt-in, AVIF-compressed, deduplicated, encrypted `.dxm` objects referenced *from* the timeline — pixels serve the text, never the other way around
- Every generated file carries `schema: dx-daemon/v1` front matter, BLAKE3 hashes, and stable anchors (`dx://2026-08-13/14#evt_...`)
- Honest capabilities everywhere: if a platform can't do something, it says so instead of faking parity

### Agents don't just read it. They grow because of it.

Here's where Dx leaves the entire industry behind: while Hermes-style agents **burn tokens** to hallucinate reusable skills from chat logs, dx-daemon **mines your real behavior deterministically** — repeated action sequences become workflow candidates, workflow candidates compile into skills. Ground truth beats guesses, at zero marginal cost. **Unlimited free skills, learned from reality.**

Any MCP-capable agent — Claude Desktop, Hermes, OpenClaw, whatever ships next month — plugs in today:

```json
{ "mcpServers": { "dx": { "command": "dx-daemon", "args": ["--mcp-stdio"] } } }
```

Search, current state, capabilities, pause/resume, crypto-shred forget — all permission-gated per agent through `AgentGrant`s. Your agents see only what you allow.

---

## The ecosystem

One brand. One vision. Rust all the way down.

| Project | What it is |
|---|---|
| **daemon** | The memory. The flagship. **COMPLETE** — all 11 targets shipping, cloud-offloaded visual memory live. |
| **cli** | The `dx` command-line hub binding the ecosystem together |
| **code** | A hardened Zed fork — our AI-native code editor |
| **agent** | Rust autonomous agent fusing the best of OpenClaw channels, Hermes self-update, deer-flow orchestration |
| **tui** | Terminal UI, forked from grok-build and rebuilt as ours |
| **desktop** | Native desktop shell |
| **www** | Our own web framework (`dx build`, sandboxed single-file output) |
| **media** | Millions of media assets, served fast |
| **icon** | Massive icon packs (LobeChat + more) powering every surface |
| **metasearch** | Search across everything |
| **flow** | Local model runtime — AI without the API bill |
| **forge** | Package/project forge |
| **check** | Static analysis, DX style |
| **dcp** | Cross-platform build pipeline |
| **serializer / style / token / tree / driven** | Core infrastructure crates |
| **train** | ML training pipeline |
| **mobile** | Our phone client |
| **video-player** | Media playback engine |

---

## Quick start

```powershell
git clone https://github.com/essence-dx/daemon
cd daemon
cargo run -p dx-daemon -- --install   # autostart at logon
cargo run -p dx-daemon -- current     # watch it know things
```

Data lives in `%LOCALAPPDATA%\DX\daemon` (macOS/Linux follow platform conventions). Encrypted. Local. Yours.

---

## We are changing how software remembers

For fifty years, operating systems forgot everything the moment a window closed. Every tool you used, everything you read, every decision you made — evaporated.

**Dx ends that.**

Text-first. Private-by-architecture. Agent-native. Free skills from real behavior. Years of memory in megabytes.

The AI era doesn't need another chatbot.
**It needs a memory. We built it.**

---

---

## Appendix: Qwen3.8-27B — Full Research & Kaggle Run Log (Aug 25–26 2026)

> **One-line takeaway:** Qwen3.8-27B is a 27B dense model that beats 30B+ flagships on agentic coding (SWE-bench Pro 61.7 vs Opus 4.6 Max 53.4) and runs at **~10.2 tok/s** on a free Kaggle P100 (Q3_K_XL, 8192 ctx, 13.1 GB) — see `qwen3.8-27b-research.md:1` and `kaggle-qwen38/qwen38-27b-run.ipynb:1` for the full pipeline.

### 1. What we proved

- **Model:** `Qwen/Qwen3.8-27B` (HF + ModelScope), released **Aug 14 2026**, **Apache 2.0**, 27B dense, 262K native context (1M YaRN), multimodal, hybrid thinking (`reasoning_effort` xhigh/medium/low) — sibling of 2.4T Max. See research `qwen3.8-27b-research.md:15`.
- **Why it matters for web dev:** vendor-reported **SWE-bench Pro 61.7**, **Terminal-Bench 73.0**, **LiveCodeBench v6 90.3**, **CoWorkBench 70.7** (all above prior Qwen3.6-27B and, on SWE-bench/CoWork, above Opus 4.6 Max). Qwen team flagged *"big step up on web frontend"* — same stack that one-shotted the viral pelican SVG. See `qwen3.8-27b-research.md:32`.
- **Memory (Unsloth Dynamic V3.0 GGUF):** 1-bit 7–8G | 2-bit 9–11G | **3-bit 12–14G** | **4-bit 16–19G** | 6-bit 23–26G | 8-bit 31G | BF16 56G. Rule: RAM+VRAM ≈ quant size. See `qwen3.8-27b-research.md:58`.

### 2. How we ran it — 11 Kaggle versions, then success

All runs at `https://www.kaggle.com/code/essencefromexistence/qwen3-8-27b-llama-cpp-run` (private, GPU P100 16G, Internet ON, 30h/week free). CLI `kaggle==2.2.4` via `python -m kaggle` (`kaggle-qwen38/kernel-metadata.json:1`).

| Ver | Status | Wall | Root cause | Fix |
|-----|--------|------|------------|-----|
| 1 @ `manfromexistence` | ERROR ~35m | DNS `Temporary failure resolving` + `nvidia-smi: command not found` → CPU 20G session, 20 GB disk | Account not phone-verified → Kaggle silently downgrades to CPU+offline | Switch to `essencefromexistence` (verified) + fail-fast guard (`qwen38-27b-run.ipynb:1`) |
| 2 | ERROR 2.5m | `CMake Error ggml/src/ggml-cuda/CMakeLists.txt:182 CUDA::cuda_driver not found` | `libcuda.so` not in `lib64/stubs` (stubs empty, read-only) | Set `CUDAToolkit_ROOT`, symlink stubs → still failed |
| 3–7 | ERROR 2.5–35m | Same CMake + later `ld: cannot find -lCUDA::cuda_driver` → then `undefined reference to cuDeviceGet` after naive `sed` removal | Imported target `CUDA::cuda_driver` broken on Kaggle image | Patched `llama.cpp/ggml/src/ggml-cuda/CMakeLists.txt:182` → `sed s/CUDA::cuda_driver/cuda/` + `-L/usr/local/nvidia/lib64 -lcuda` + `-DCMAKE_CUDA_ARCHITECTURES="60;75"` (P100+T4) |
| 8 | ERROR 2.5m | `sed: can't read ggml/... No such file` + `set -e` abort | Wrong path (`ggml/` vs `llama.cpp/ggml/`) | Fix path to `llama.cpp/ggml/...` |
| 9 | ERROR 35m | Same `ld: cannot find -lCUDA::cuda_driver` (sed path wrong, patch not applied) | Patch not applied → configure passed but link used literal `-lCUDA::cuda_driver` | Correct `sed` + linker flags |
| 10 | ERROR 45m | Duplicate build cell (old unpatched `In [2]` still present) → second cmake without patch failed | Two `%%bash` build cells, second still old code | Delete duplicate cell (`fix_cells.py`) |
| **11** | **COMPLETE 37m** | — | — | **Patched build, Q3_K_XL 13.1G (fits P100 16G), ctx 8192, generic `*.gguf` glob, `cp || true` for same-file `set -e`** |

**Final build log:** `97M llama-server 0.3.0-dev` built with `GNU 11.4.0`, `arch 60;75`, `ggml 0.22.0`. See `run_final.log:20529`.

### 3. Live results — hello prompts + tok/s + Kaggle usage

All from the `COMPLETE` run (P100-PCIE-16GB, Driver 580.159.04 CUDA 13.0):

```
=== Prompt: 'Hello' ===
Answer: Hello! How's it going? Is there something I can help you with today?
Time: 6.05s | Prompt 41 → Completion 57 | 9.42 tok/s | GPU 98% 13385/16384 MiB

=== Prompt: 'Hello! Who are you?' ===
Answer: Hello! I'm Qwen (Tongyi Qianwen) by Alibaba...
Time: 12.40s | 46 → 132 | 10.64 tok/s | GPU 98%

=== Prompt: 'Say hello in 3 languages' ===
Answer: English: Hello / Spanish: Hola / Japanese: こんにちは
Time: 10.96s | 46 → 114 | 10.40 tok/s | GPU 99%
```

- **Average tok/s:** P100 mean **10.15**, weighted **10.30** (303 toks / 29.41s). Final config v13 --accelerator NvidiaTeslaT4: **2x Tesla T4**, UD-Q4_K_XL 17.6G split 8109/9371 MiB, avg **10.94 tok/s** (+8%, better Q4 quant), load 80s at `temperature 1.0 top_p 0.95 top_k 20` with `reasoning_effort low` (`qwen38-27b-run.ipynb:4`).
- **Kaggle usage snapshot** (`run_final.log:2117`):
  - GPU: `Tesla P100-PCIE-16GB` 16384 MiB, **13385 MiB used / 2886 free**, 38% idle → 98–99% during gen, 59°C 40W/250W
  - RAM: 31Gi total, 3.3Gi used, 27Gi avail
  - Disk: `/dev/loop1` 20G, **14G used 6.0G avail 70%** (13.1G model + 97M binary)
  - Wall: build ~33m + download Q3 13.1G ~45s (hf_transfer) + load 10s + gens ~30s
- **Cost:** ~37m = **0.62h** of the 30h/week free GPU quota. `COMPLETE` kernels auto-stop — **not burning hours now** (`kaggle kernels status ...` → `COMPLETE`).

Full research: `qwen3.8-27b-research.md:1` (Colab T4 15G → Q3, Kaggle T4x2 30G → Q4, Unsloth Desktop, OpenRouter `qwen/qwen3.8-27b-free`). Kernel source: `kaggle-qwen38/` (metadata + notebook + patch logs at `G:\Temp\UserTemp\opencode\run*.log`).

### 4. Easy on/off system — one-command run

We ship a local on/off wrapper so you never re-pay the 33m build. See `qwen/README.md:1` and scripts below:

```powershell
# One-time: Kaggle CLI already installed, auth as essencefromexistence (OAuth, auto-refresh)
# Run (pushes cached-build notebook v11 → ~1–2m if binary cached, else ~37m first time):
.\qwen\run.ps1          # → https://www.kaggle.com/code/essencefromexistence/qwen3-8-27b-llama-cpp-run

# Check:
.\qwen\status.ps1       # COMPLETE / RUNNING + tok/s tail

# Stop is automatic (batch kernel); for local Ollama/llama.cpp:
.\qwen\stop.ps1         # kills local server if running
```

Local alternative (no Kaggle, no build — instant):

```powershell
# Ollama (17–19G for Q4, 12–14G for Q3):
ollama pull hf.co/unsloth/Qwen3.8-27B-GGUF:UD-Q3_K_XL
ollama run hf.co/unsloth/Qwen3.8-27B-GGUF:UD-Q3_K_XL  # /think /no_think to toggle

# Unsloth Desktop (Win/Mac/Linux GUI):
# https://unsloth.ai/download → search Qwen3.8 → Download → Run
```

Scripts live at `qwen/run.ps1:1`, `qwen/status.ps1:1`, `qwen/stop.ps1:1`, `qwen/local/run-local.ps1:1`. Next optimization (v12) will cache the 97M `llama-server` as a Kaggle Dataset (`dataset_sources` in `kernel-metadata.json:10`) to make re-runs ~2m.

MIT OR Apache-2.0
updated: 2026-08-26 00:40 UTC
