# HELP.md — Bonsai 2 27B on Colab TPU: everything tried, what worked, what is left

A working document for the execution track in `tpu/`. It is **not** a ledger: open work
lives in [`TODO.md`](TODO.md) (items `TPU-01`…`TPU-07`), what changed is in
[`CHANGELOG.md`](CHANGELOG.md), and the facts that must not be re-measured are in
[`tpu/DO-NOT-RETEST.md`](tpu/DO-NOT-RETEST.md). This file is the *map*: every method that
has been tried on this track, what each one actually bought, the arithmetic that bounds
the rest, and the research behind the 1M-context question.

**Scope rule (AGENTS.md §2 rule 7):** this track is TPU-only. A GPU number is not a
result of this project, and no GPU run may be started without the user's explicit
go-ahead. GPU kernels under `tpu/` are kept for the record and marked out of scope.

---

## 1. The model, the file, the machine

| Fact | Value | Where it came from |
| --- | --- | --- |
| model | Bonsai 2 27B, ternary **PTQ1_0** (1.75 bits/weight) | `prism-ml/Ternary-Bonsai-2-27B-gguf` |
| file | `Ternary-Bonsai-2-27B-PTQ1_0.gguf`, 5,946,648,928 B (5.54 GiB) | same |
| architecture | `qwen35` — 64 blocks, **48 gated-delta-net + 16 full attention** (`full_attention_interval 4`) | GGUF header |
| dims | n_embd 5120, n_ff 17408, 24 query heads / 4 kv heads, head dim 256 | GGUF header |
| GDN | 16 groups, state 128×128, rank 48, conv 4 | GGUF header |
| **native context** | **262,144** (`qwen35.context_length`) | GGUF header |
| RoPE | 64 dims, `freq_base 1e7`, IMROPE half-split pairs | GGUF header + `04-*.py` |
| ternary scheme | `prism.hadamard.*`: block 1024, normalized-Sylvester-Walsh-Hadamard, explicit signs (28,672 = 5120+6144+17408) | GGUF header |
| MTP head | **absent** — 851 tensors, none named `nextn`/`mtp`, no `qwen35.nextn_block_count`; the base `Qwen/Qwen3.8-27B` has one | headers read by byte-range, all three conversions (PTQ1_0, PQ2_0, F16) |
| machine | Colab **TPU v5e-1**, one core, **15.75 GiB HBM**, ~709 GB/s measured HBM read | `13-cost-sweep/README.md:28` |
| stack | `jax 0.11.2` + `libtpu 0.0.50` (stock Colab's jax 0.7.2 cannot compile Pallas) | `tpu/DO-NOT-RETEST.md` §1 |
| weights on device | 6.37 GiB (28→32 byte rows, +14 % padding) | `14-bonsai-transposed/README.md` |

---

## 2. Every method that has been tried

Ordered by how much it was worth. "Receipt" names the file that owns the number.

### 2.1 The wins

| # | Method | Effect | Receipt |
| --- | --- | --- | --- |
| 1 | **Lane-aligned tiles**: store the packed block as `(32, rows)` and compute `x @ w` so the out-columns sit on the 128 lanes (Pallas maps only the last two dims to the register file; Mosaic puts a matmul's N axis on the lanes) | **29.66 → 281.01 tok/s** (9.5×) at batch 80 | `14-bonsai-transposed/receipt-14-b80-f16-tc1024.json` |
| 2 | **Batch ladder** 32 → 80 with f16 state storage | 2.09× aggregate; stops at the **memory wall** (15.67 of 15.75 GiB) | same + `receipt-14-b32…`, `-b64…`, `-b48…` |
| 3 | **Decode spelling**: `concatenate` instead of `stack(axis=1).reshape` (a layout conversion), then int16 lanes | 1.33× then 1.20× on the decode | `13-cost-sweep/README.md` Q5 |
| 4 | **Out-tile TC 512 → 1024** | +5.5 % at batch 80 | `receipt-14-b80-f16-tc1024.json` |
| 5 | **Fused 64-layer step**: all 64 layers in one `jax.jit`, the 61-chunk vocab loop in a second | **4.62 → 5.54 tok/s** single-stream at 8k; compile 142 s → 57 s | `receipt-14-fuse1-b1-f16-tc1024.json` |
| 6 | **Persistent compile cache** (`/content/jaxcache`) + a compile watchdog | the rerun's prefill is 4.6 s instead of a fresh 140 s compile | same run's prefill line |
| 7 | **GGUF header read by byte-range** (40 MB instead of a 5.9 GB download) | the whole MTP question answered from the artifact in minutes | `tpu/DO-NOT-RETEST.md` §14 |

### 2.2 The measured no-ops (each one closed a hypothesis)

| Method | Effect | Why it is settled |
| --- | --- | --- |
| **Pallas-callable cache** (one callable per geometry) | 130.25 → 130.30 tok/s, **0.04 %** | rebuilding ~400 callables per step was never the cost |
| **Program count** (`TO` 128 → 1024 = 3200 → 400 programs) | 1.18×, saturating at 512 | program count is not the lever |
| **Projection fusion** (400 → 256 launches: qkv+gate, q+k+v, gate+up) | **182.3 → 181.3 ms, 0.5 %** | **launch overhead is not the cost** — this refuted the whole "per-launch overhead" theory and is why the plan moved to the unpack |
| **DMA overlap** (`PREFETCH=1`) | does not compile: Pallas wants a *scalar* prefetch dim; this grid is `(TOUT, 1, BPC)` | ceiling was 7.6 % anyway (DMA is 0.29 ms of a 3.83 ms projection) |
| **HBM bandwidth** | 709 GB/s read measured, 86 % of spec | not the constraint |
| **The k-loop** (8 in-blocks per step) | 15 % *slower* | refuted twice |
| **Cheaper unpack arithmetic** (1.4× fewer ops) | no change | the decode is data-movement bound *inside the VPU* |
| **Folding the power into the activation** | **impossible, by algebra** | the trit value is `floor(3·((byte·p) & 0xFF)/256) − 1`: the power enters through a modular multiply and a threshold, so the value is not `q · c_p` |

### 2.3 The context ceiling (measured, then closed)

| window | outcome | receipt |
| --- | --- | --- |
| `max_t` 83 | 152.3 ms/step, **6.57 tok/s** | `receipt-14-ctx64-b1-f16-tc1024.json` |
| `max_t` 1024 | 153.6 ms, 6.51 tok/s | `receipt-14-ctx1024-…` |
| `max_t` 8192 | 180.9 ms, 5.53 tok/s | `receipt-14-ctx8192-…` |
| `max_t` 45056 | runs at 2.86 tok/s with `FUSE=0` | `receipt-14-b1-f16-tc1024-45056.json` |
| `max_t` 49152 | **OOM** — `jit_attn_step` wants 2.44 GB, 2.30 GB free (140 MB short) | `receipt-14-mem-ceiling.json` |
| `max_t` 65536 | **OOM** — wants 3.26 GB, 184.53 MB free | same |

**The whole 8k window costs 19 %** at batch 1 (152.3 → 180.9 ms). The earlier 638/662 ms
step times were a **batch × context** interaction, not context: they were measured at
batches 48 and 32, while the 284.7 ms run was at batch 80. Context cost at fixed batch
was never measured until 2026-10-20, and it is small.

### 2.4 Queued experiments (in the kernel, priced by one session)

| Knob | What it changes | Hypothesis |
| --- | --- | --- |
| `NOCONCAT=1` | serves the fourteen power groups with fourteen matmuls, skipping the two concatenation passes over the unpacked weights (the largest single term in the projection's traffic, ~25× the tile bytes) | the step's traffic falls ~2.3× ⇒ 152 → ~70–90 ms |
| `GQA=1` | contracts the 24 query heads against the 4 kv heads directly instead of materialising `K[:, :, _KIDX, :]` (201 MB/layer/step at 8k, 3.2 GB/step) | the 8k window's 29 ms falls toward ~10 ms |
| `FUSE=0/1` | projection fusion (256 vs 400 launches) | exact but 0.5 %; kept because it compiles 2.5× faster |
| `PREFETCH=0..4` | `num_scalar_prefetch` on the grid spec | does not compile at 1+ on this grid |

All four arms of the 2×2 (shipped / `NOCONCAT` / `GQA` / both) run in one session;
receipts are `tpu/14-bonsai-transposed/receipt-14-{base,nc,gqa,both}-b1-f16-tc1024.json`.

### 2.5 Tooling built along the way

| Tool | What it does | Where |
| --- | --- | --- |
| `ensure_setup.py` + `setup_fast.sh` | idempotent setup: pin jax/libtpu, download the 5.5 GB GGUF (skip if present), verify | `G:\Temp\UserTemp\opencode\colab_run\` |
| `run14_ask.sh` | one run = restart kernel → `colab exec` with the env knobs → tee to `14-ask.log` | same |
| `gguf_graft.py` | grafts an MTP head into a GGUF (header rewrite + tensor append); **self-tested** on synthetic GGUFs | `tpu/mtp-graft/kernel.py` (inlined) |
| GGUF header scanner | reads a 5.9 GB file's metadata from 40 MB of range requests | `G:\Temp\UserTemp\opencode\gguf_one.py` |
| compile watchdog | a thread that prints every 60 s while the first fused step compiles | `14-bonsai-transposed/kernel.py` |

### 2.6 The GPU route — recorded, out of scope

Four Kaggle kernels (MTP-head test, draft-model + n-gram at 8k, 2×T4, MTP graft) were
built and then ruled out of scope; they are kept in `tpu/` marked out of scope, and all
four Kaggle objects were restored after the deletion mistake (see `CHANGELOG.md`,
2026-10-20). Facts that survive: the file has no MTP head, and the base model does.

---

## 3. Where the time actually goes (the arithmetic that bounds the rest)

Per token, on one v5e-1:

| term | size | note |
| --- | --- | --- |
| weight traffic | 6.37 GiB packed → **54 GB** of bf16 unpacked | the MXU must read it back: **~108 GB of VMEM traffic per step** |
| HBM read floor | 6.37 GiB ÷ 709 GB/s = **9.0 ms** | never reached; the unpack dominates |
| KV at 8k | 16 attn layers × 4 kv heads × 256 × 2 × 4 B = **131 KB/token** | 1.07 GB at 8k; **the 16 attention layers are the only ones that grow** |
| GDN state | 48 layers × (128×128×48) × 2 B (f16) = **75 MB, constant** (151 MB in f32) | independent of context length — the reason long context is cheap on this family |
| context cost at 8k | **+29 ms** | mostly the kv-head expansion (201 MB/layer/step) |
| step today | **152 ms** (short window) / 181 ms (8k) | ~2.5–3× above the unpack floor |

**Consequences, stated so they stop moving:**

- **50 tok/s = 20 ms/step is ~3–4× below the bf16-unpack floor** for a 27B dense model
  on one v5e-1. The per-stream ceiling is **~11–20 tok/s**; the hardware's real number is
  **281 tok/s aggregate**, because eighty streams share one unpack.
- **Speculative decoding does not amortise the weight read on this architecture**: 48 of
  the 64 layers are gated-delta-net, whose state recurrence is sequential in the chain, so
  a k-token verification pass re-reads the weights k times. Getting the amortisation
  needs the **chunked gated-delta-rule** form (so a chain verifies in one pass) plus a
  real draft — our file has no MTP head.
- Routes to 50+ per stream: chunked-GDN spec decode with a grafted or small-model draft;
  int8 weights *and* activations (≈2× less traffic, a precision change); more than one
  chip; or a smaller model.

---

## 4. The 1M-context question — what the research says, and what fits

**What the model family supports.** Qwen3-Next (aka "Qwen 3.5"), the family Bonsai 2
comes from, natively supports **262,144** tokens — exactly the number in our GGUF header —
and Qwen states it has *"validated the model's performance on context lengths of up to 1
million tokens using the YaRN method"*. The vLLM recipe for it is
`VLLM_ALLOW_LONG_MAX_MODEL_LEN=1 --max-model-len 1048576 --rope-scaling
'{"rope_type":"yarn","factor":4.0,...}'`. So 1M is a **RoPE-scaling** problem, not a
retraining problem, and the model is the right one for it.

**The memory arithmetic (ours, measured):**

| KV format | bytes/token | 1M tokens | fits in 15.75 GiB with 6.37 GiB of weights? |
| --- | --- | --- | --- |
| fp32 (today) | 131 KB | **131 GB** | no |
| fp16 | 64 KB | 65 GB | no |
| int8 | 32 KB | 33 GB | no |
| 4-bit | 16 KB | 16 GB | no — over the budget before activations |

**A dense 1M KV cannot fit on one v5e-1, at any KV precision.** That is arithmetic, not
an implementation gap. The production answers to the same problem (per the 2026 KV-cache
literature: LMCache, Mooncake, InfiniGen, Spheron's NVMe offloading) are all
*tiered/offloaded* KV, and they only work because the KV is read rarely relative to
compute — here it is read **every token**, so paging 65 GB per token is hopeless.

**Three designs that do fit, in order of honesty:**

1. **Sink + sliding window over the 16 attention layers** (the family's own design):
   keep the last *W* tokens of KV plus a few attention-sink tokens; let the **48 GDN
   layers' constant state carry the rest**. The model then genuinely processes 1M tokens
   (its GDN state is a summary of them) while the KV stays at `W × 131 KB` — e.g. W = 32k
   ⇒ 4.2 GB fp32, or W = 64k in fp16 ⇒ 4.2 GB. This is the *correct* long-context shape
   for this architecture, and the only one that reaches 1M on one chip.
2. **Quantised KV** for that window (`--cache-type-k q4_0` in llama.cpp; a q4 KV path in
   our kernel is item `TPU-04`) — 4× more window for the same bytes.
3. **Host-RAM paging** for windows up to ~100k, where the per-token read is still
   affordable — not for 1M.

**The other half of 1M: prefill.** Our kernel prefills **one token per step** (a
deliberate, receipted choice: it is what makes the full prompt real rather than
truncated). At 150 ms/token, a 1M-token prefill would take **41 hours**. A 1M context
therefore also needs **chunked prefill** — many positions per pass — which for this
architecture means the same **chunked gated-delta-rule** work as the spec-decode path, with
the 16 attention layers already able to take many query positions behind a mask. **One
build, two payoffs:** chunked GDN unlocks both multi-token verification and 1M prefill.

**What the model itself can carry:** the file declares 262,144 native. Going to 1M is
RoPE extrapolation (YaRN), which Qwen validated for this family; expect quality to
degrade relative to the native window (the "lost in the middle" effect, 40 % at scale per
the 2026 literature) — that is a quality question for a session with a 1M prompt, not
something the kernel can fix.

**A concrete order of work for 1M:**

| step | what | cost | done-when |
| --- | --- | --- | --- |
| 1 | fp16 KV storage (halves the KV) | small | a receipt at `max_t` ≥ 90k |
| 2 | `GQA=1` (kills the kv-head expansion) | queued | 8k context cost < 15 ms |
| 3 | q4 KV (item `TPU-04`) | a kernel | 4× the window for the same bytes |
| 4 | sink + sliding-window attention | a kernel | a 1M-token session runs end to end |
| 5 | chunked GDN + chunked prefill | a kernel | a 1M-token prefill finishes in minutes, not 41 hours |
| 6 | YaRN RoPE scaling to 1M | config + a quality check | a 1M prompt is answered sensibly |

Steps 4–6 are the difference between "1M tokens of KV" and "a 1M-token conversation".

---

---

## 4a. The two concrete builds the format research unlocked (2026-10-20)

Both were found by reading the artifact, not by inference.

### 4a.1 `PQ2_0`: the same model, a 2-bit weight format whose unpack is trivial

`prism-ml/Ternary-Bonsai-2-27B-gguf` also publishes `Ternary-Bonsai-2-27B-PQ2_0.gguf`
(7.25 GB). Its header is the **same model**: `qwen35`, 851 tensors, the identical
`prism.hadamard.*` metadata (block 1024, the same 401 folded weight names, the same
28,672 sign values) — only the *encoding* of the weights differs. The 402 weight
tensors are type 142 (`TQ2_0`) instead of type 143 (`PTQ1_0`).

The layout, from the PrismML fork's own `ggml-common.h` / `ggml-quants.c`:

```c
// 2-bit quantization, 16 blocks of 16 elements, 2.625 bits per weight
typedef struct {
    uint8_t scales[QK_K/16];   // 16 bytes
    uint8_t qs[QK_K/4];        // 64 bytes: 4 two-bit fields each
    ggml_half d;               // the scale
} block_tq2_0;                 // QK_K = 256 elements per super-block

// dequantize_row_tq2_0:
int8_t q = (x[i].qs[j + m] >> (l*2)) & 3;
*y++ = (float)(q - 1) * d;
```

**Why it matters on a TPU:** the PTQ1_0 value is
`floor(3 * ((byte * p) & 0xFF) / 256) - 1` — a modular multiply, a mask, a multiply and
a shift per element, and the power `p` is a property of the *row*, which is what forces
the group concatenation. The 2-bit form is a shift, a mask, a subtract and a scale, with
**no row-dependent power at all** — so the concatenation disappears structurally rather
than by the `NOCONCAT` trick, and the four 2-bit planes can be contracted as four clean
matmuls.

**Price:** the file is 1.4 GB bigger (6.37 → ~7.8 GB of tiles on device), which lowers
the fp32-KV context ceiling from ~45k to ~38k tokens.

**Verification plan (no TPU needed for the format part):** reimplement
`dequantize_row_tq2_0` in numpy, check it against the C reference on random blocks,
then port it into the Pallas kernel and check the model's own gate (first token `11` for
`Hello`, `271` for a 17-token prompt) and its text.

### 4a.2 The MTP head exists — in community GGUFs, for the exact base

Our own file has no MTP head (851 tensors, zero `nextn`/`mtp`, no
`qwen35.nextn_block_count` — all three PrismML conversions). The head is nevertheless
available, in files that keep the same `prism.hadamard.*` metadata:

| file | tensors | head | notes |
| --- | --- | --- | --- |
| `sudoingx/Ternary-Bonsai-2-27B-PTQ1_0-MTP-GGUF` (`-mtp.gguf`, `-mtp-lean.gguf`) | 866 (851 + 15) | `qwen35.nextn_predict_layers: 1`, the 15 `blk.64.*` tensors | PTQ1_0 base, head weights in Q6_K / Q8_0 |
| `BoldingBuilds/Ternary-Bonsai-2-27B-Abliterated-PQ2_0-MTP-GGUF` | 866 | same | **2-bit base + head** — the ideal target |
| `ProCreations/Ternary-Bonsai-2-27B-MTP`, `ManniX-ITA/…-MTP-GGUF`, `vinpix/…-Stock-MTP-GGUF` | 866 | same | community releases |

So the "graft" is really a **download**: copy the 15 `blk.64.*` tensors out of a
community file. Two facts to plan around:

- The head's weights are **plain llama.cpp quant types** (Q8_0 / Q6_K), *not* in the
  prism-hadamard set (the folded list is 401 names = the 64 layers). So they are not
  folded, and the TPU kernel would need either a Q8_0/Q6_K decode path or a host-side
  dequant to bf16 (the head is ~430 M parameters → 0.86 GB in bf16, which fits).
- The head is a nextn *block* (`blk.64.*`) plus `nextn.eh_proj` / `enorm` / `hnorm` /
  `shared_head_norm` — i.e. a full transformer block that proposes the next token from
  the previous one. Verifying a chain of k candidates therefore needs the **chunked
  gated-delta-rule** form for the 48 GDN layers, so the k positions are verified in one
  weight read instead of k sequential ones. That single build unlocks both
  multi-token speculation *and* chunked prefill (the 1M-context path).

### 4a.3 The one number the 45 tok/s estimate rests on: the VMEM bandwidth

The 45 tok/s figure (22–25 ms/step) assumes the unpack's internal traffic can be
moved in ~12 ms. Our own measurement implies about **1 TB/s** of effective VMEM
bandwidth: the step is 152 ms while moving ~159 GB of VMEM traffic (the unpack expands
6.37 GiB of packed weights to 54 GB of bf16, with two concatenation passes over it).
TPU v5e's VMEM is 128 MiB per TensorCore and "much higher bandwidth" than HBM, but the
exact figure is not published, and at 1 TB/s the int8 path lands nearer **20–30 tok/s**
than 45.

**This is measurable, and it is exactly what the queued 2×2 matrix measures:**
`NOCONCAT=1` removes ~40 % of the traffic, so the ms/step it saves *is* the effective
VMEM bandwidth. One session answers the question that decides whether 45 tok/s is real.

## 5. Open work

`TODO.md`, items `TPU-01`…`TPU-07` (the track's ledger; `GPU-*` items were withdrawn when
the track was ruled TPU-only). In short: `TPU-02` the DMA overlap (7.6 % ceiling),
`TPU-03` multi-token verification, `TPU-04` the q4 KV and the 64k window, `TPU-05` the
Colab pool quota and its re-auth unlock, `TPU-06` closed (the fixed-batch context curve),
`TPU-07` the ternary unpack — the real blocker, ~9× off the HBM roofline.

## 6. Running it

```bash
# one run = restart kernel, then exec with the knobs
bash run14_ask.sh '1:float16' 1024 64 1 'your prompt here' 8192
#   RUNS=BATCH:state_dtype   TC=out-tile columns   N_TOKENS   CACHE   PROMPT   MAX_T
# env knobs: FUSE=0|1  NOCONCAT=0|1  GQA=0|1  PREFETCH=0..4
```

Etiquette the track requires: **always run the real 64-layer model** (never a
single-tensor benchmark), print the correctness gate (first token `11` for `Hello`,
`271` for a 17-token prompt), and let the receipt own the number. `tpu/README.md` §6 and
`tpu/DO-NOT-RETEST.md` §0 have the full rules.

Session reality: the v5e1 pool is quota-limited and returns `503 Service Unavailable` for
long stretches; a **re-auth** (`mv ~/.config/colab-cli/token.json …bak` then `colab
usage`) has opened it twice the same day. Re-pin the stack after every re-create
(`pin_stack.py` + `restart-kernel`, ~2 min) — a recycled VM comes back with jax 0.7.2 and
cannot compile a Pallas kernel.

---

MIT OR Apache-2.0
updated: 2026-10-20 22:35 UTC
