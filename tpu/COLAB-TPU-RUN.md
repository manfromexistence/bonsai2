# Running Bonsai 2 on a Colab TPU v5e-1 (verified working, 2026-10-27)

This page records a **fresh, end-to-end run of the real 64-layer model on a
Colab TPU v5e-1**, with the exact commands, the environment pin, and the
resulting tok/s. It is the durable copy of a run that succeeded on
2026-10-27. Tier: `[exec]` (run by the operator, receipts named below).

**Result:** single stream, 8k window, 64 generated tokens, first token `271`,
generated text byte-identical across arms:

| arm | tok/s | ms/step | receipt |
| --- | --- | --- | --- |
| shipped decode, FOLD=0, GQA on | **7.43** | 134.6 | `lane-aligned-ternary-kernel/receipt-fresh-fold0-b1-f16-tc1024.json` |
| decode fold, FOLD=2, GQA on (same model) | **7.67** | 130.3 | `lane-aligned-ternary-kernel/receipt-fresh-fold2-b1-f16-tc1024.json` |

Aggregate/batch-80 throughput (281.01 tok/s) is a different quantity and was
not re-run here; see `TPU-STATUS.md`.

---

## 0. Prerequisites (on the Windows client)

- `google-colab-cli` installed (`uv tool install google-colab-cli`).
- The 120 s auth-timeout patch applied into the CLI's own site-packages:
  ```
  $sp = "$env:APPDATA\uv\tools\google-colab-cli\Lib\site-packages"
  Copy-Item 'G:\Dx\tpu\colab-auth-timeout-patch\colab_timeout_patch.py' $sp -Force
  Copy-Item 'G:\Dx\tpu\colab-auth-timeout-patch\zz_colab_timeout.pth'   $sp -Force
  ```
- A Google account with TPU v5e-1 quota. `v6e1` is refused for entitlement on
  this account; v5e1 is the only TPU available.

## 1. Create the session

```
colab new -s dx-tps --tpu v5e1
```

The pool is frequently unavailable (`503 Service Unavailable` on the assign
endpoint, sometimes for hours). Retry until `Session READY`. A CPU-only
`colab new` (no `--tpu`) assigns immediately, which proves the account is fine.

## 2. Upload the two source files

```
colab upload 'G:\Dx\tpu\lane-aligned-ternary-kernel\kernel.py' '/content/kernel.py'
colab upload 'G:\Dx\tpu\reference-forward-pass\kernel.py'         '/content/04.py'
```

`kernel.py` is the real 64-layer model + fused Pallas kernel; `04.py` is the
fp32 oracle forward pass that `kernel.py` imports via `FWD_SRC`.

## 3. Pin the stack and fetch the weights

Run `G:\Temp\UserTemp\opencode\colab_run\setup_fast.py` (or an equivalent) with:

```
colab exec -s dx-tps -f <path>\setup_fast.py --timeout 2400
```

It must do, in this order:

1. `pip install 'jax==0.11.2' 'jaxlib==0.11.2'` then
   `pip install libtpu==0.0.50 -f https://storage.googleapis.com/jax-releases/libtpu_releases.html`
   (jax first, then libtpu; stock Colab jax 0.7.2 / libtpu 0.0.21.1 cannot
   compile a Pallas kernel at all).
2. Download the 5.54 GiB weights from Hugging Face (Kaggle 403s from Colab
   IPs) into `/content/gguf/Ternary-Bonsai-2-27B-PTQ1_0.gguf`:
   ```
   curl -L --retry 3 --retry-delay 5 -C - -o /content/gguf/Ternary-Bonsai-2-27B-PTQ1_0.gguf \
     https://huggingface.co/prism-ml/Ternary-Bonsai-2-27B-gguf/resolve/main/Ternary-Bonsai-2-27B-PTQ1_0.gguf
   ```

A recycled session VM reverts to the stock stack, so re-pin every session.

## 4. Run the model

`colab exec -f FILE` runs the file as Python in the remote kernel, so the model
run is a subprocess with its own environment. The driver sets these variables
and then runs `/content/kernel.py`:

```
XLA_PYTHON_CLIENT_MEM_FRACTION=0.95
FWD_SRC=/content/04.py
PROMPT="what is the capital of franch, india and bangaldesh and give me tps"
N_TOKENS=64
MAX_T=8192
RUNS=1:float16
TC=1024
FUSE=1
GQA=1
NOCONCAT=0
PREFETCH=0
KVDTYPE=float32
FOLD=0          # 0 = shipped decode, 2 = decode fold (same model)
CHUNK=1
CHUNKCHECK=0
OUTDIR=/content/run0
```

Command:

```
colab exec -s dx-tps -f <driver.py> --timeout 5400
```

Expect: tile build ~13 s, prefill (compile + 17 steps) ~30-40 s, then decode.
The receipt lands at `$OUTDIR/receipt-b1-f16-tc1024.json`.

## 5. Download the receipt

```
colab download '/content/run0/receipt-b1-f16-tc1024.json' 'G:\Dx\tpu\lane-aligned-ternary-kernel\receipt-fresh-fold0-b1-f16-tc1024.json'
```

## 6. Stop the session when done

```
colab stop -s dx-tps
```

Kaggle/Colab sessions die with the notebook, and the TPU pool is scarce; do not
leave a session idle.

## Traps that must not be re-learned

- **Thinking is on by default.** A bare prompt spends its whole budget in a
  think block and emits nothing (looks like a hang). The kernel sets
  `--reasoning-budget 0 --reasoning-format none`; do not remove them.
- **A Pallas core halt wedges the runtime across processes.** Only
  `colab restart-kernel` clears it; compile failures do not wedge, so
  compile-only variants can be batched into one process.
- **After any session re-create the GGUF is gone** (`/content/gguf/`); re-run
  setup before the next run.
- **Weights come from Hugging Face, not Kaggle** (Kaggle 403s from Colab IPs).
- **`FWD_SRC` must point at the uploaded oracle** (`/content/04.py`), or
  `kernel.py` cannot import the reference forward pass.
- **Do not close over weight tiles in JAX** (constants capture → 6.95 GB,
  compile never finishes). Pass the per-layer weight pytree as an argument.
- **The correctness gate is prompt-dependent**: `Hello` -> first token 11 `,`,
  the 17-token prompt -> 271. Do not expect one value for every prompt.

## Claim tier

All numbers above are `[exec]`: measured on Colab TPU v5e-1 on 2026-10-27 by
the operator, receipts named in the table. A CPU reference or a GPU run would
not be a result of this project (AGENTS.md section 2, rule 7).

MIT OR Apache-2.0
updated: 2026-10-27 00:00 UTC