# Ternary LFM3-1.7B - measured, and stopped

Status page for the ternary-quantization attempt on `LiquidAI/LFM3-1.7B`. The work is
**paused on purpose**: the ~430 MB target was not reached, and the next move is to fine-tune a
small model before returning to ternary.

**Scope:** Kaggle, 2x T4, llama.cpp `a7af149`. Hub doc; it certifies nothing outside these
receipts.

---

## 1. The one-line result

| tier | claim | evidence |
| --- | --- | --- |
| executed | ternary LFM3-1.7B collapses at 2.1 bpw with llama.cpp PTQ | kernel `lfm3-1-7b-ternary-ladder`, KLD 10.4 |
| executed | the Hadamard fold is implemented correctly and still does not rescue it | kernel `lfm3-hadamard-ternary`, no-op 0.14% + KLD 10.54 |
| executed | IQ2_S at 640 MiB is the usable floor today | ladder receipt, KLD 0.663 |
| repo-managed | kernels and proofs live in `G:\Ai\kaggle\lfm3-*/` | `kernel.py`, `verify_fold.py` |

---

## 2. Measured ladder

Reference: F16 GGUF 3306 MiB, perplexity 21.3708 over 60 chunks of wikitext-2.

| arm | MiB | bpw | KLD | same top-1 | gate |
| --- | ---: | ---: | ---: | ---: | --- |
| `q4km_imatrix` | 1048.7 | 5.09 | **0.087** | **85.9%** | pass |
| `iq2m_imatrix` | 640.1 | 3.11 | 0.663 | 62.8% | fail (0.60 line) |
| `q2_0_rtn` | 607.7 | 2.95 | 9.834 | 0.013% | destroyed |
| `q2_0_rot` | 607.7 | 2.95 | 9.173 | 0.052% | destroyed |
| `tq2_0_default` | 574.9 | 2.79 | 9.849 | 0.000% | destroyed |
| `tq2_0_full` | 434.3 | 2.11 | 10.405 | 0.026% | destroyed |
| `tq1_0_pinned` | 430.8 | 2.09 | 10.538 | 0.020% | destroyed |
| `q2_0_folded` | ~500 | 2.09 | 10.541 | 0.020% | destroyed |

The cliff is between 3.11 bpw (KLD 0.66) and 2.11 bpw (KLD 10.4) - a step, not a slope.

---

## 3. What was ruled out, with the reason

| attempt | outcome | why |
| --- | --- | --- |
| PrismML `PTQ1_0`/`PQ2_0` | not usable | their fork has no `lfm3` arch; a packed file could never be scored |
| plain `TQ1_0`/`TQ2_0`/`Q2_0` | KLD ~10 | `quant_weights` is discarded (`ggml-quants.c:2416,2423`) - no error compensation |
| pinning `attn_gate` + `shortconv` at high precision | no effect | KLD 10.405 -> 10.538; the damage is in attention/FFN matrices |
| Hadamard fold (R2-style v/o only) | -6.7% KLD | 9.834 -> 9.173, still destroyed |
| full blockwise Hadamard + runtime transform | KLD 10.54 | fold verified as a no-op (0.14%), so the loss is the ternary assignment |
| community Q1_0 quant (431 MB) | unusable | incoherent output, and slower than Q4 |

---

## 4. Why the fold run matters

The Hadamard contract (`prism.hadamard.*`) exists in upstream llama.cpp but both the converter
and the loader refuse `lfm3`. Both were patched, the model rebuilt, 128 tensors folded, and
the result validated in two stages:

1. **fold only, F16** - perplexity 23.5224 vs 23.489 reference, delta **+0.14%**. A correct
   fold is a no-op, so this isolates a convention bug from a quantization result.
2. **ternary on top** - KLD **10.541**, i.e. no improvement over unfolded RTN.

That two-stage order is the reusable part: it converts "the transform should help" into a
measurement, and it is why the negative result can be trusted.

---

## 5. Cost, measured

| stage | time |
| --- | --- |
| CUDA build (once, now cached as a dataset) | 28.8 min |
| rebuild with the Hadamard patch | 31 min |
| convert + F16 reference + imatrix | ~5 min |
| one ternary arm quantized | 20-26 s |
| evaluation, 7 arms, 2 GPUs | 69.9 min |

Quantization is ~10% of a run. Build 249 MiB of binaries are cached as dataset
`essencefromexistence/llamacpp-cuda-a7af149a`, so a new arm costs ~15 min.

---

## 6. Where the work lives

| artifact | path or id |
| --- | --- |
| full record, failures and next steps | `G:\Ai\kaggle\lfm3-hadamard-ternary\RESULTS.md` |
| ladder kernel | `G:\Ai\kaggle\lfm3-1.7b-ternary\` |
| Hadamard kernel + fold proof | `G:\Ai\kaggle\lfm3-hadamard-ternary\` |
| live-log technique | `G:\Dx\KAGGLE_LIVE_LOGS.md` |

---

## 7. Honest limits

This page does not claim ternary is impossible, and it does not claim 640 MiB is "good" - that
arm misses a gate borrowed from Llama-3-8B. It claims exactly two things: at 2.1 bpw this model
is destroyed under llama.cpp PTQ, and the Hadamard implementation used here was verified before
being blamed.

The remaining untested lever inside the same toolchain is `sign_mode=explicit` (random signs,
one run). Anything below ~600 MiB that keeps quality otherwise needs QAT.

---

MIT OR Apache-2.0
updated: 2026-10-29 00:00 UTC
