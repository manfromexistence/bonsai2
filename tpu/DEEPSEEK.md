# DeepSeek-V4.1-Flash-1017 — Local / Low-Resource Running Report

**Topic:** running a 763B-parameter model on everyday hardware (12 GB GPU / 64 GB RAM) via
GSQ-RCO quantization and the Strata inference engine.
**Compiled:** 2026-10-24 (web research snapshot).
**Status tier:** repo-managed documentation of externally reported (runtime/hosted) facts.
Every number below names its source.

---

## 1. Executive summary

DeepSeek-V4.1-Flash-1017 (released 2026-10-17, MIT-licensed weights on Hugging Face)
is a 763B-parameter sparse MoE (384 routed experts per layer, 6 active per token) with a
552B backbone + 196B Engram tables and a 1M-token context. On 2026-10-24, ISTA-DASLab
released **GSQ-RCO** quantizations of it (IQ1_M 1.75 bpw, IQ2_XXS 2.00 bpw, IQ2_XS 2.25 bpw)
that run on a **single 12 GB GPU** via the **Strata** inference engine, with the Engram
tables memory-mapped on an SSD. Quality stays at 98.7–100.0% of the base model on
AIME25 / GPQA-Diamond / LiveCodeBench v6. Early community measurements: ~2.9–8.2 tok/s
on modest single/multi-GPU boxes; ~4.7 tok/s on an RTX 5070 12 GB + 64 GB RAM.

Key sources: Hugging Face model card `deepseek-ai/DeepSeek-V4.1-Flash-1017` (official),
`ISTA-DASLab/DeepSeek-V4.1-Flash-1017-GSQ-RCO-GGUF` (quantization card),
`Niko1221/Strata` docs/DEEPSEEK41.md (engine + measured speeds),
X post `@AGTPinsights` status 2113890971847213216 (compiled summary),
`ai-pulse-lab.com` signal (Chinese-language community report),
`dataxad.com` blog (DeepInfer engine), `local-ai-zone.github.io` deep dive,
`unsloth.ai` docs (Unsloth GGUFs).

---

## 2. Base model — DeepSeek-V4.1-Flash-1017

Source: Hugging Face `deepseek-ai/DeepSeek-V4.1-Flash-1017` model card; DeepSeek official news page.

| Property | Value |
| --- | --- |
| Release date | 2026-10-17 |
| API model name | `deepseek-flash` (legacy `deepseek-v4-flash` still routes) |
| Total parameters | 763B (552B backbone MoE + 196B Engram) |
| Architecture | Causal Encoder-Decoder (CED): 20-layer encoder + 20-layer decoder |
| Active params / token | 8B input (prefill), 16B output (decode) |
| Routed experts | 384 per layer, 6 active per token |
| Context window | 1,048,576 tokens |
| Modalities | text + image in, text out |
| License | MIT (weights + repo) |
| Recommended sampling | temperature 1.0, top_p 1.0, max_tokens >= 256K |
| Engram tables | 196.6B params, 202.8 GB at MXFP8 |

Official benchmark deltas vs. the September V4.1-Flash build (source: HF model card):

| Benchmark | V4.1-Flash-1017 | V4.1-Flash |
| --- | --- | --- |
| HLE w/o tools | 42.6 (44.0 text-only) | 36.8 (39.1) |
| Terminal-Bench 4.0 | 42.3 | 31.2 |
| DeepSWE v1.1 | 77.0 | 74.2 |
| ProgramBench | 29.1 | 20.3 |
| NL2Repo-Bench | 68.2 | 65.4 |
| Automation-Bench v1.0.6 | 60.5 | 54.8 |
| Agents' Last Exam | 35.4 | 31.8 |
| Apex Bench v1.1 | 72.1 | 64.6 |
| Chartography (w/ tools) | 83.0 | 78.9 |
| BabyVision (w/ tools) | 92.5 | 89.6 |
| ZeroBench-main (w/ tools) | 51.0 | 49.0 |

Against the competitive set on the same card, the 1017 build leads GLM-5.3 and K3 on every
row where comparisons exist, wins 7 of 8 comparable rows vs. GPT-5.6 Sol (loses tool-less
HLE), and beats Claude Opus 5 on DeepSWE (77.0 vs 74.0), Automation-Bench, Apex Bench
and Agents' Last Exam while Opus 5 leads on HLE, Terminal-Bench and ProgramBench.

---

## 3. GSQ-RCO quantization (ISTA-DASLab)

Source: Hugging Face `ISTA-DASLab/DeepSeek-V4.1-Flash-1017-GSQ-RCO-GGUF` model card.

- **GSQ** (Gumbel-Softmax Quantization, arXiv 2604.18556) — post-training scalar
  quantization that jointly learns per-coordinate grid assignments and per-group scales
  via Gumbel-Softmax relaxation. Closes most of the scalar-vs-vector gap at 2–3 bits
  while staying deployable as standard GGUF.
- **RCO** (Riemannian Constrained Optimization, arXiv 2605.00649) — assigns one of K
  quantization types to each of N tensors under an exact total size budget, reformulated
  as a smooth Riemannian manifold in logit space; gradient-based on task loss, no
  constraint-specific hyperparameter tuning.
- Authors: Dadgarnia, Tabesh, Nikdan, Helcig, Kurtic, Kleinegger, Alistarh (DASLab,
  IST Austria). Michael Helcig posted the release on 2026-10-24.

Builds (backbone + shared 111 GB Engram file, IQ4_NL, 4.50 bpw):

| Build | bpw | Backbone | Engram | Total | Notes |
| --- | --- | --- | --- | --- | --- |
| IQ1_M | 1.75 | 120.72 GB | 110.60 GB | 231.32 GB | Smallest |
| IQ2_XXS | 2.00 | 138.24 GB | 110.60 GB | 248.84 GB | Recommended; closest to base |
| IQ2_XS | 2.25 | 155.23 GB | 110.60 GB | 265.83 GB | Largest |

Quality vs. base model (reasoning effort 50, AIME25 / GPQA-Diamond / LiveCodeBench v6):

| Variant | bpw | AIME25 | GPQA-D | LCB v6 | Task avg | Recovery |
| --- | --- | --- | --- | --- | --- | --- |
| Base (FP8/FP4) | n/a | 100.00 | 90.91 | 89.71 | 93.54 | 100.0% |
| GSQ-RCO IQ1_M | 1.75 | 100.00 | 88.38 | 88.57 | 92.32 | 98.7% |
| GSQ-RCO IQ2_XXS | 2.00 | 100.00 | 90.91 | 89.71 | 93.54 | 100.0% |

- Dense tensors (attention, shared experts, embeddings, head) choose among Q5_K / Q6_K / Q8_0;
  routed experts choose among IQ1_S … IQ3_S.
- As of 2026-10-23, llama.cpp does **not** yet support DeepSeek-V4.1; these GGUFs run in
  Strata. (Strata docs also list llama.cpp support as a future path.)
- Reproducibility: each build ships `tensor-allocation/*.rco-allocation.txt` listing the
  quant type per tensor (1008 tensors) plus target/measured bpw.

---

## 4. Strata inference engine (local running)

Source: `Niko1221/Strata` docs/DEEPSEEK41.md; stratallm.org; X post `@coldniko` (Niko Veit).

Strata is an open-source local inference engine (Niko Veit) that runs large MoE models on
consumer GPUs/RAM with OpenAI- and Anthropic-compatible APIs. For DeepSeek-V4.1-Flash it
uses a dedicated engine package `strata-dsv41` (ported from Project Maya's `maya-ds4`, MIT).

### 4.1 Hardware / memory layout (Strata docs)

| Component | Size | Where it lives |
| --- | --- | --- |
| Routed experts (15,360 of 18.8 MB) | 288.8 GB (UD-Q8) | as many as fit in VRAM; rest computed on CPU, read in place from files |
| Attention + dense weights | 16.8 GB | VRAM, one block of layers per card (~3.9 GB/card on 4 cards) |
| Engram (n-gram table) | 202.8 GB | SSD (memory-mapped); a few rows read per token; not counted in RAM |

Measured on 4x Radeon AI PRO R9700 (32 GB each) + 251 GB RAM: 6,197 experts (116.5 GB)
in VRAM, 9,163 (172.3 GB) on CPU (~that much free RAM to stay cached). `strata launch --plan`
and `setup.py --check` compute this for your cards/RAM before downloading.

### 4.2 Official 2-bit builds offered by Strata setup

Three ISTA-DASLab sets (shared 111 GB Engram + 8 GB DSpark drafter):

| Set | bpw | Backbone | Total disk | Notes |
| --- | --- | --- | --- | --- |
| IST-DAS-1.75bpw (IQ1_M) | 1.75 | 121 GB | 239 GB | smallest; weakest on prose |
| IST-DAS-2.00bpw (IQ2_XXS) | 2.00 | 138 GB | 257 GB | middle; recommended |
| IST-DAS-2.25bpw (IQ2_XS) | 2.25 | 155 GB | 274 GB | largest; best quality |

Quality vs. original (Strata docs, 4,995 positions): 2.00 → KL 0.170, PPL +15.5%
(code +8.6%, agent +5.3%); 2.25 → KL 0.107, PPL +7.3% (code +3.2%, agent +2.2%);
IQ1_M → PPL +21% (code +8.9%, agent +10.2%, multilingual +14%, docs +24%,
plain English prose +53%). ISTA-DASLab card says IQ1_M keeps 98.7% of base task average.

Measured speeds (Strata docs):
- RTX 5070 (12 GB) + 64 GB RAM + 3.6 GB/s NVMe: ~4.7 tok/s decode; 121 tok/s prefill
  (4K) → 231 tok/s (8K, layer-major path).
- RTX 3060 (12 GB) + 46 GB RAM: ~8–10 tok/s (code 9.9, agent 7.8; 149 tok/s at 8K);
  DSpark off there (`--dspark on` adds 2–13%).
- 4x R9700: ~48 tok/s on 2.00 bpw, ~80 tok/s on 2.25 bpw (code, with shipped
  hot-expert profile).
- Under ~40 GB RAM: untested and slow (setup says so, still installs).

Community-reported (X/AGTP summary):
- RTX 4060 Ti 16 GB + 64 GB RAM: 2.9 tok/s.
- 3x R9700 + 64 GB RAM: 2.8–8.2 tok/s.
- 4x GB10: ~121 tok/s decode (multi-GPU community build).
- Niko Veit showed it running on 64 GB RAM; his post passed 48K views.
- Strata does not yet officially support V100.

### 4.3 Install (Strata)

```
./setup.sh --family deepseek41 --model IST-DAS-2.00bpw   # 257 GB (resumable, SHA-256)
./setup.sh --family deepseek41 --model IST-DAS-2.25bpw   # 274 GB
./setup.sh --family deepseek41 --model IST-DAS-1.75bpw   # 239 GB
./setup.sh --family deepseek41 --model UD-Q8_K_XL        # Unsloth 508 GB set
./setup.sh --family deepseek41 --gguf-dir DIR            # files you already have
strata launch --model ist-das-2.00bpw --plan             # dry-run placement
strata launch --model-path DIR --model ist-das-2.00bpw   # start from existing files
```

Default context 8192 tokens. Needs `strata-dsv41` next to the engine (build with
`cmake -DSTRATA_BUILD_DSV41=ON`). Loading takes a few minutes (config
`engine_ready_s: 3600`).

API behavior: `/v1/chat/completions`, `/v1/messages`, streaming, stop, `/metrics`.
Thinking on by default (effort `high`=75); `reasoning_effort: "none"` or
`enable_thinking: false` disables. Effort words: low=50, medium=62, high=75, max=100,
or integer 1–100. Tool calls emitted as DSML, converted to OpenAI `tool_calls` /
Anthropic `tool_use` (including `tool_choice`, streamed args, calls inside thinking).
Defaults when none specified: temperature 1.0, top_p 1.0. Conversation slots preserved
across requests/restarts (~17 MB per 6.5K tokens), capped 2 GB, removed after a week.
Not in this version: concurrent requests (`--parallel` ignored with warning) and vision
(`--vision` ignored with warning).

Tuning: `strata launch --tune quick|thorough` runs a short tuner (`tools/dsv41/tune.py`)
measuring decode speed across DSpark on/off, shorter draft block, thread counts; keeps a
setting only for >5% gain; ~5–10 min. RAM store (`--ssd-cache`) sizes itself at start.

### 4.4 Unsloth path (alternative GGUFs)

Source: unsloth.ai docs/models/deepseek-v4.1-flash.

Unsloth ships its own Dynamic GGUFs (UD-IQ1_S … UD-Q8_K_XL) with Engram offloading,
claiming ~3x faster inference than other CPU/VRAM/ngram repos on top of llama.cpp.
Hardware table (RAM+VRAM or unified, Engram on SSD):

| Quant | Total | N-gram table | Rest |
| --- | --- | --- | --- |
| UD-IQ1_S | 233.0 GB | IQ4_NL / 110.6 GB | 122.4 GB |
| UD-IQ1_M | 250.4 GB | IQ4_NL / 110.6 GB | 139.8 GB |
| UD-IQ2_XXS | 300.5 GB | Q5_K / 135.2 GB | 165.3 GB |
| UD-Q2_K_XL | 334.8 GB | Q5_K / 135.2 GB | 199.6 GB |
| UD-IQ3_XXS | 394.7 GB | Q6_K / 161.3 GB | 233.5 GB |
| UD-IQ3_S | 423.6 GB | Q6_K / 161.3 GB | 262.3 GB |
| UD-Q3_K_XL | 485.1 GB | MXFP8 / 202.8 GB | 282.3 GB |
| UD-Q4_K_XL | 500.7 GB | MXFP8 / 202.8 GB | 297.9 GB |
| UD-Q8_K_XL | 508.4 GB | MXFP8 / 202.8 GB | 305.6 GB |

RAM requirements: 1-bit 128–145 GB, 2-bit 176–205 GB, 3-bit 256–290 GB,
4-bit 305 GB, 8-bit 315 GB. Includes DSpark speculative decoding (1.3–1.5x decode
speedup) with a Q8_0 drafter (8 GB) / BF16 drafter (8.5 GB). Engram precision is
swappable (IQ4_NL 110.6 / Q5_K 135.2 / Q6_K 161.3 / MXFP8 202.8 GB).

---

## 5. DeepInfer (DeepSeek's own engine, datacenter scale)

Source: dataxad.com blog "DeepSeek-V4.1-Flash-1017 and DeepInfer"; kingy.ai blog.

DeepInfer (released 2026-10-20, Apache 2.0) is DeepSeek's in-house inference engine,
supports exactly one model (DeepSeek-V4.1-Flash-1017). Began as a vLLM fork with ideas
from SGLang/TensorRT-LLM. Kernels: FlashMLA, DeepGEMM, DeepEP, TileKernels, DeepSelect,
custom TileLang. Features: attention data parallelism + balanced context parallelism,
MoE expert parallelism with EPLB, DSpark draft tokens with confidence-scheduled
verification, SWA bounded replay, async scheduler, batch-invariant inference +
deterministic sampling (NVIDIA only), structured output (regex/JSON schema/EBNF).

Measured (DeepSeek's own numbers, read off charts — approximate):
- Decode: ~26,000 output tok/s per GPU at ~100 tok/s per user, on 64 GPUs in one NVLink
  domain, 64-way expert parallelism, 131,072-token context, 384 routed + 64 redundant
  experts. Lighter load: ~180 tok/s at 128 req/GPU, ~250 at 64, ~325 at 8. Heavy:
  ~1.7M output tok/s cluster-wide at 256 req/GPU.
- Prefill: up to ~89,000 uncached input tok/s per GPU on 4 GPUs; 4,096-token batch in
  ~70 ms at ~60,000 tok/s; 16,384 tokens ~82,000 tok/s in ~200 ms; 65,536 tokens
  ~730 ms at ~89,000 tok/s.
- DSpark: τ = 3.25 tokens per full forward pass (3 draft tokens, 75% per-token
  acceptance).
- SWA bounded replay: replays only last ~140 tokens (128 window + margin) on
  prefill→decode transition; on 131K prompt eliminates almost all decoder prefill work.

Deployment limits today: NVIDIA SM100 (Blackwell B200/GB200) or Huawei Ascend 950 NPUs only;
offline (batch) inference only — missing KV cache system, prefill/decode disaggregation,
load balancing across DP replicas. Needs Linux, GCC 13+, Python 3.12, CUDA 13 or CANN 9.2.
Config: 384 routed experts (6 active), FP4 experts with UE8M0 scale, single KV head
512-dim, 128-token sliding window + compressed sparse attention with two-level indexer,
Engram n-gram lookups.

DeepSeek Harness (`dsh`) is the agent-level counterpart (MIT, launched Aug 2026);
v0.2.1-rc.2 added 1017 support + DeepSeek Search/Fetch APIs.

DwarfStar (antirez/ds4) is the single-machine C engine (Metal/CUDA/ROCm) for owned
hardware: Q2 341 GiB (152 GiB main weights), Q4 483 GiB (294 GiB main), 189 GiB Engram
tables always read from disk; 128 GB Mac/Spark with `--ssd-streaming`, or 2 machines
with tensor parallelism over RDMA.

---

## 6. Pricing / API economics (DeepSeek official + providers)

Source: local-ai-zone deep dive; openrouter.ai (per AGTP summary).

- Off-peak: $0.15/M input, $0.003/M cache read, $0.60/M output.
- Peak (01:00–04:00 and 06:00–10:00 UTC, Mon–Fri): $0.30 / $1.20 (2x).
- DeepSeek official reports ~91.5% cache hit rate → weighted avg input actually paid
  ~$0.017/M. Official endpoint: 156 tps, 1.38 s P50 latency, 100% uptime.
- Third parties (release-day snapshot): inference.net $0.10/$0.50 in/out; DeepInfra
  discounted $0.14/$0.42 (list $0.20/$0.60).
- Search & Fetch APIs: $0.75 per 1,000 calls per endpoint; free until 2026-10-31 16:00 UTC
  when called via server-side tools in DeepSeek Harness / Claude Code / Codex.

---

## 7. Running it locally — practical reality

- Full precision: ~510 GB native weights, ~614 GB GPU memory required (per AGTP summary).
- Unsloth planning estimates for 552B backbone: BF16 ~1,100 GB+, Q8 ~590 GB,
  Q4_K_M ~320 GB, Q2_K ~210 GB.
- Community quants: 122 quantized variants on HF; antirez `deepseek-v4.1-flash-gguf`
  (Q2 + Vision, 4.13 GB each, ~16 GB RAM) is an experimental artifact, not the full model.
- No Jinja chat template ships; use DeepSeek's `encoding/encoding.py` or `deepseek-recipe`
  (Rust + Python bindings) for correct prompt encoding. Getting the format wrong is the
  fastest way to reproduce a benchmark miss.
- OrcaRouter community builds: "abliterated" GGUFs claiming 91.5% fewer refusals and
  92% MMLU vs 91% original (per AGTP summary; treat as unverified community claim).
- InferX offers it free for a couple of days (per AGTP summary); most-used model on
  OpenRouter per InferX.

---

## 8. Key sources (URLs)

- Hugging Face base model: https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017
- ISTA-DASLab quant card: https://huggingface.co/ISTA-DASLab/DeepSeek-V4.1-Flash-1017-GSQ-RCO-GGUF
- pfeifferj runtime reconstruction: https://huggingface.co/pfeifferj/DeepSeek-V4.1-Flash-GSQ-RCO-GGUF
- Strata repo/docs: https://github.com/Niko1221/Strata (docs/DEEPSEEK41.md)
- Strata site: https://stratallm.org
- Unsloth guide: https://unsloth.ai/docs/models/deepseek-v4.1-flash
- DeepSeek official news: https://www.deepseek.com/en/news/deepseek-v4-1-flash/
- Dataxad DeepInfer blog: https://dataxad.com/blog/deepseek-v4-1-flash-1017-deepinfer
- Local AI Zone deep dive: https://local-ai-zone.github.io/blog/deepseek-v4-1-flash-1017-deep-dive.html
- AI Pulse Lab signal: https://ai-pulse-lab.com/signals/2026-10-24/...
- AGTP X summary: https://x.com/AGTPinsights/status/2113890971847213216
- Niko Veit X post: https://x.com/coldniko/status/2113835370869583983
- DeepInfer repo: https://github.com/deepseek-ai/DeepInfer
- DeepSeek Harness: https://github.com/deepseek-ai/deepseek-harness
- DwarfStar (ds4): https://github.com/antirez/ds4
- GSQ paper/code: arXiv 2604.18556 / github.com/IST-DASLab/GSQ
- RCO paper/code: arXiv 2605.00649 / github.com/IST-DASLab/RCO

---

## 9. Caveats / claim discipline

- All speed numbers are runtime/hosted claims from community or vendor sources, not
  reproduced on this machine. Treat as indicative.
- Benchmark numbers are DeepSeek's own (official model card); independent verification
  of the strongest claims (e.g. DeepSWE 77.0) was still catching up at publication.
- Community claims (OrcaRouter refusal/MMLU deltas, InferX "most-used" ranking, free
  trials) are unverified third-party statements.
- DeepInfer numbers are read off charts (approximate) and are DeepSeek's own.
- Strata docs state under ~40 GB RAM is untested and slow; V100 not officially supported;
  llama.cpp support not yet available as of 2026-10-23.
- This project's Bonsai/TPU track is TPU-only; GPU numbers here are not TPU results and
  must not be quoted as this project's throughput (AGENTS.md §2 rule 7).

## 10. Kaggle TPU feasibility (can we run it on the free TPU?)

Question: run DeepSeek-V4.1-Flash-1017 on Kaggle's free TPU (v5e-8) under the
~20 h/week quota / ~9 h session cap.

**Short answer: No — not practically, not today.** Three blockers, each independently
sufficient.

### 10.1 Memory: 128 GB HBM vs. 231–614 GB needed

- Kaggle TPU v5e-8 = 8 chips × 16 GB HBM = **128 GB HBM** (Kaggle docs; HF optimum-tpu).
- DeepSeek-V4.1-Flash-1017: 763B params. Native weights ~510 GB, ~614 GB GPU memory.
- Smallest available quant (ISTA-DASLab IQ1_M): **231.32 GB** total
  (120.72 GB backbone + 110.60 GB Engram). The backbone alone (120.72 GB) leaves
  ~7 GB of the 128 GB for KV cache / activations / runtime — effectively zero headroom.
  The 111 GB Engram must be paged from host RAM/disk, which on Kaggle's TPU VM is
  slow and starves decode.
- Even GLM-5.3-Flash (320B) on the same v5e-8 needs a custom 3-bit/int8 JAX engine and
  still only gets ~165 tok/s; DeepSeek is ~2.4× the parameters.

### 10.2 No TPU backend exists

- DeepInfer (DeepSeek's engine): NVIDIA SM100 (Blackwell) or Huawei Ascend 950 only.
  No TPU path.
- Strata: CUDA (NVIDIA) / CPU; no TPU.
- `kaggle-tpu-lab` (github.com/ARahim3/kaggle-tpu-lab) ships JAX/Pallas engines only for
  Qwen3.8-27B and GLM-5.3-Flash. There is no DeepSeek-V4.1-Flash TPU engine. Building one
  means writing a JAX engine for a 763B MoE with Engram lookups, compressed sparse
  attention (two-level indexer), DSpark speculative decoding and SWA bounded replay — a
  multi-month project, not a notebook.
- llama.cpp has no TPU backend either.

### 10.3 Quota and session shape

- Free quota ~20 TPU h/week (not 30), session cap ~9 h then reconnect (kaggle-tpu-lab docs).
- Loading a 231 GB model + compile cache consumes a large fraction of a session before
  any inference happens; reconnects lose state.

### What to do instead

- **Use the API**: DeepSeek official `deepseek-flash` at $0.30/M input (peak) /
  $0.60/M output (off-peak $0.15/$0.60), ~91.5% cache hit → ~$0.017/M effective input.
  Cheapest path to full quality.
- **Run locally on a single 12 GB GPU** with Strata (ISTA-DASLab quants): ~2.9–8.2 tok/s
  measured; ~4.7 tok/s on RTX 5070 12 GB + 64 GB RAM.
- **If TPU is the goal**: this is the project's own Bonsai/TPU track; DeepSeek-on-TPU is
  not available and would be a separate engine project. GPU/TPU numbers here are not
  results of this project's TPU work (AGENTS.md §2 rule 7).

### 10.4 Correction: Kaggle GPU (2x T4) — not TPU

The user's actual hardware is Kaggle's **2x T4 GPU** session (not TPU). Re-evaluating
DeepSeek-V4.1-Flash-1017 on that:

- **GPU compute:** Strata's DeepSeek engine ships a `cuda-cuda13` package (Windows + Linux).
  T4 is NVIDIA Turing (sm_75). Strata documents only RTX 3060 (Ampere, sm_86) and
  RTX 5070 (Blackwell) + R9700 (AMD); T4 (sm_75) support is **unverified** — setup
  says plainly if no package fits your card. CUDA 13 still targets sm_75, but the binary
  may not include it. Try it; if unsupported, setup stops before downloading.
- **Memory:** 2x T4 = 32 GB VRAM. Dense weights 16.8 GB split ~8.4 GB/card → fits with
  headroom; more routed experts stay in VRAM than on a single 12 GB card → faster than
  single-GPU. Memory is **not** the blocker here.
- **Disk (the real blocker):** smallest quant is 231 GB (IQ1_M). Kaggle dataset limit
  is ~20 GB (legacy) / ~100 GB (newer private quota). A 231 GB model cannot be attached
  as one dataset. Downloading 231 GB from HuggingFace inside a session takes hours and
  eats the ~30 h/week GPU quota. **This is what makes it impractical on Kaggle.**
- **Speed (if it ran):** T4 is slower than RTX 3060 (~8-10 tok/s single) → expect less;
  2 cards help experts → roughly 5-8 tok/s estimated. RTX 3060 figure is the reference.
- **Quota:** ~30 h/week GPU, session ~9-12 h; loading 231 GB + compile burns a big
  chunk per session.

**Verdict for 2x T4 on Kaggle:** memory OK, but disk/quota is the killer. Realistic
options: (a) download only IQ1_M and run across sessions with a persistent HF cache
(still huge); (b) host weights on a faster source; (c) just use the DeepSeek API
($0.30/$1.20 per M peak, ~$0.017/M effective input with cache) — cheapest path to
full quality; (d) run locally on a 12 GB GPU with Strata if you own one.

## 11. Cheapest self-hosting (monthly cost)

Three realistic paths for DeepSeek-V4.1-Flash-1017 (quantized via Strata or full precision).

### Path A — Strata quantized on a rented single GPU (cheapest monthly)

- Needs: 12 GB VRAM + 46–64 GB system RAM + 239 GB SSD + NVIDIA CUDA 13 (RTX 3060
  documented working; RTX 5070 faster).
- Cheapest rental: Vast.ai RTX 3060 listings from ~$0.03–0.07/hr, but a 64 GB RAM +
  240 GB disk instance is typically ~$0.10–0.20/hr.
- Monthly (24/7): roughly **$70–$150/month** (interruptible/spot lower; on-demand higher).
- Speed: ~8–10 tok/s (RTX 3060) to ~5 tok/s (RTX 5070) per Strata docs.
- Caveat: Strata's `cuda-cuda13` package; T4 (sm_75) unverified. RTX 3060 (Ampere)
  is the documented working card.

### Path B — Full model (4-bit INT4) on rented GPUs

- Needs: 306 GB VRAM (8× RTX A6000) per getdeploying.com.
- Cost: **~$3,514/month** (8× RTX A6000 on-demand). 8-bit: $10,426/mo; 16-bit: $46,022/mo.
- Break-even vs API: ~8B tokens/month at 4-bit. Below that, the API is cheaper.

### Path C — Own hardware (one-time, no monthly rental)

- 128 GB Mac (M-series) with SSD streaming via DwarfStar (antirez/ds4): Q2 ~341 GiB
  file, 152 GiB main weights + 189 GiB Engram on SSD. Needs 48+ GB RAM; Mac mini M4
  32 GB is too small. One-time cost ~$3,000–$6,000. No monthly rental.
- Or a desktop PC: RTX 3060 12 GB + 64 GB RAM + 256 GB NVMe SSD, ~$400–$800 used.

### API comparison (usually cheapest)

- DeepSeek official `deepseek-flash`: $0.30/M input (peak) / $1.20/M output;
  off-peak $0.15/$0.60; ~91.5% cache hit → ~$0.017/M effective input.
- Cheapest third party: inference.net $0.10/$0.50 per M.
- Self-hosting Strata only wins if you serve >~tens of millions of tokens/month
  and already own the hardware; rented GPU only wins at very high volume.

---

MIT OR Apache-2.0
updated: 2026-10-24 00:00 UTC
