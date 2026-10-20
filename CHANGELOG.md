# CHANGELOG.md — What Changed in the `G:\Dx` Hub

Dated, receipted, per release. One event has one record: when an item in [`TODO.md`](TODO.md)
closes, it is deleted there and a line is added here **in the same commit**
(`AGENTS.md` §5). Every entry names the `file:line` or the command output that earns it.

**Claim tiers** (`AGENTS.md` §4) apply to every line below. *repo-managed* means committed
and readable. *executed* means run here with a named command. *runtime / hosted* means true
on another host. *release-qualified* means shipping verified end to end on a real device.
Nothing in this file is release-qualified.

---

## Unreleased

### 2026-10-20 — `docs:` the 50 tok/s route: MTP speculation on the GPU, launch fusion on the TPU

**The target is 50 tok/s for a single stream on an 8k context window. The research says
the mechanism is multi-token-prediction speculation, and that the cheapest proof of it
on this machine is a GPU run, not a TPU one.**

- **The measurement that reframes the problem:** **180.6 ms/step at batch 1**
  (`tpu/14-bonsai-transposed/receipt-14-b1-f16-tc1024-fused.json`) against
  **284.7 ms/step at batch 80**
  (`tpu/14-bonsai-transposed/receipt-14-b80-f16-tc1024.json`). Eighty sequences cost
  1.58× one sequence: on this track the step is ~85 % per-projection launch overhead
  across ~400 projections, not bandwidth. The traffic floor is 6.37 GiB ÷ 709 GB/s =
  **9.0 ms** (`tpu/13-cost-sweep/README.md:28`), so 50 tok/s (20 ms/step) is 2.2× the
  floor and needs that fixed cost amortised over several tokens, not tuning.
- **The route with a number attached:** `tpu/mtp-test/kernel.py:3-4` records
  **50.44 tok/s at 48.3 % acceptance** for the same 27B dense architecture with
  llama.cpp's `--spec-type draft-mtp --spec-draft-n-max 8`, and
  `tpu/spec-draft/kernel.py:8-9` records **25.5 tok/s** for it with no draft at all.
  The draft is the checkpoint's own MTP head, so nothing new is downloaded.
- **Two traps that travel with the flag:** it was renamed on 2026-05-13
  (`--spec-type mtp` → `--spec-type draft-mtp`) and the old spelling is ignored
  *silently*; and the MTP head never saw the chat template, so acceptance on chat is
  ~83 % against ~90 % on pretraining text.
- **Filed as open work, not as a claim:** `TODO.md` `GPU-01`…`GPU-03` and
  `TPU-01`…`TPU-06`, each with its evidence and its `Done when`.
- **What this entry does not claim:** no 50 tok/s run of Bonsai 2 27B on an 8k window
  has been performed on this machine. 50.44 tok/s is the reference checkpoint's number,
  measured elsewhere, and the open question is whether the PTQ1_0 quantization kept
  the MTP head at all (`GPU-02`).

### 2026-10-19 — `feat:` tpu nb14 — the 64-layer step is one jitted program: 4.62 → 5.54 tok/s per stream at 8k

- **What changed** (`tpu/14-bonsai-transposed/kernel.py`): `full_step` runs all 64 layers
  inside one `jax.jit`; the per-layer weight pytree is passed as an **argument** (`W_t`)
  instead of being closed over; the 61-chunk vocab loop became one jitted program
  (`_logits_raw(h, out_raw)`); a persistent compile cache (`/content/jaxcache`) and a
  compile watchdog were added.
- **Measured, same prompt, same window, same batch** (`max_t` 8192, batch 1, 64
  generated, prompt *"what is the capital of franch, india and bangaldesh and give me
  tps"*): **4.62 → 5.54 tok/s, 216.3 → 180.6 ms/step**, decode 13.85 s → 11.56 s.
  Receipts: `receipt-14-b1-f16-tc1024-franch.json` (before) and
  `receipt-14-b1-f16-tc1024-fused.json` (after). The first token is `271` in both and
  the 64 generated tokens are byte-identical, so the fusion changed the speed and not
  the model.
- **The trap that cost 2 h 12 m:** closing over the weight tiles made JAX capture
  **6.95 GB of constants** at lowering (*"A large amount of constants were captured
  during lowering (6.95GB total)"*) and the compile never finished — the kernel
  connection dropped at 15:23 with the job still running. Passing the weights as an
  argument cut the compile to **139.6 s**; with the compile cache, the rerun's prefill
  was **4.6 s**.
- **The gate, for the record:** the fused smoke run (prompt `Hello`) returned first
  token `11` `','` — notebook 08's expected value *for that prompt* —
  (`receipt-14-b1-f16-tc1024-fused-smoke.json`). An automated check that expected `271`
  for every prompt aborted that run as a false alarm: the gate value is
  prompt-dependent, and the check was wrong, not the kernel.

### 2026-10-19 — `docs:` the single-chat context ceiling is 45k, not 64k — and 64k needs a q4 KV cache that does not exist

- **Measured** (Colab TPU v5e-1, batch 1, fp32 KV): `max_t` 45056 runs at
  **2.86 tok/s, 350.1 ms/step, peak 14.01 GiB**
  (`tpu/14-bonsai-transposed/receipt-14-b1-f16-tc1024-45056.json`); `max_t` 49152 dies
  with `RESOURCE_EXHAUSTED … 'jit_attn_step': Attempting to reserve 2.44G … There are
  2.30G free` — **140 MB short**; `max_t` 65536 dies the same way wanting 3.26 G with
  **184.53 MB free**. All three are recorded together in
  `tpu/14-bonsai-transposed/receipt-14-mem-ceiling.json`.
- **Why:** fp32 KV is 128 KiB/token/sequence, so 64k is 8 GiB of KV on a 15.75 GiB
  budget that already holds 6.37 GiB of weights. q4 KV would make 64k 1 GiB and the
  window would fit at ~11.3 GiB — but the kernel has no q4 KV path and no blocked or
  paged attention, so that is kernel work, not a flag.
- **A claim withdrawn by the same measurement:** *"context is expensive, batch is
  not"* does not survive the fixed-batch comparison. The 638 ms (`max_t` 320) and
  662 ms (`max_t` 512) step times were measured at batches 48 and 32 — **smaller**
  than the 284.7 ms run at batch 80 — so batch cannot be the sole cause, and the
  context cost at fixed batch has never been measured. Filed as `TODO.md` `TPU-06`
  rather than left as a settled claim.

### 2026-10-20 — `feat:` tpu nb14 — projection fusion is numerically exact, and it refutes the launch-overhead theory

- **What changed** (`tpu/14-bonsai-transposed/kernel.py`): the three projection groups
  that share one input are now served by a single Pallas launch. `attn_qkv`+`attn_gate`
  (GDN layers), `attn_q`+`attn_k`+`attn_v` (attention layers) and `ffn_gate`+`ffn_up`
  (both layer kinds) all have IN=5120, and the tile layout is block-major in the
  out-column, so concatenating the member tiles along the row axis *is* the combined
  tile. The member tiles are released after combining, so device bytes are unchanged at
  6.37 GiB. `FUSE=0` restores the old path; `PREFETCH` was added as a knob for the
  DMA/compute overlap and is 0 by default.
- **Measured, same session, same prompt, same window** (`max_t` 8192, batch 1, 64
  generated): **FUSE=0 → 5.49 tok/s, 182.3 ms/step, 400 launches**;
  **FUSE=1 → 5.52 tok/s, 181.3 ms/step, 256 launches**. First token `271` in both and
  the 64 generated tokens are byte-identical, so the fusion is numerically exact.
  Receipts: `receipt-14-fuse0-b1-f16-tc1024.json`, `receipt-14-fuse1-b1-f16-tc1024.json`.
- **The refutation, which is the point:** removing **36 % of the kernel launches moved
  the step time by 0.5 %.** The per-step cost is therefore *not* launch overhead, and
  the plan built on that assumption (`TODO.md` `TPU-01`, "≤ 100 ms/step at 256
  launches") is withdrawn. What the step actually pays for is the per-weight-byte VPU
  work — the ternary unpack — which is the same for one sequence as for eighty
  (180.6 ms/step at batch 1 against 284.7 ms/step at batch 80). **Only a pass that
  processes several tokens against the same unpacked weights can amortise it**, which is
  exactly what multi-token verification does, and why `TPU-03` is now the only route to
  50 tok/s on this hardware.
- **One real gain, recorded so it is not lost:** the fused program compiles in
  **57.5 s** against **142.3 s** for the 400-launch program, because there are fewer
  distinct Pallas programs to lower. The fusion is kept for that reason and for its
  exactness; it is not kept for speed.

### 2026-10-20 — `docs:` Bonsai 2's GGUFs have no MTP head — the 50 tok/s route needs a graft or a draft model

- **Measured from the artifact, not inferred:** the GGUF headers of all three PrismML
  conversions were read with byte-range requests (40 MB each, no 5.9 GB download):
  `Ternary-Bonsai-2-27B-PTQ1_0.gguf`, `-PQ2_0.gguf` and `-F16.gguf` each report
  **851 tensors, 64 blocks, and zero tensors named `nextn`/`mtp`**, and none of them
  carries a `qwen35.nextn_block_count` key. The architecture is `qwen35` with
  `context_length 262144`, 24/4 heads, `full_attention_interval 4` (48 GDN + 16
  attention), and the `prism.hadamard.*` metadata that defines the ternary scheme.
- **Consequence:** `--spec-type draft-mtp` cannot work on this file. The 50.44 tok/s
  at 48.3 % acceptance that made MTP the obvious route was measured on
  **Qwen/Qwen3.8-27B**, which is Bonsai 2's base model and *does* ship an MTP head
  (the HF card records `base_model: Qwen/Qwen3.8-27B`).
- **What remains:** graft the base model's MTP head into the PTQ1_0 file, or use a
  separate draft model (the repo's `spec-draft` kernel uses Qwen3.5-0.8B Q4_0), or use
  the n-gram family, which needs no second model at all. All three are measured on
  Kaggle T4 at `-c 8192` now.
- **`TODO.md` `GPU-02` is closed by this entry**: the question it asked — does the
  PTQ1_0 GGUF carry an MTP head — is answered from the file itself, in a way no
  further session can improve on.

### 2026-10-20 — `docs:` a graft tool for the MTP head, and the TPU ceiling priced honestly

- **A graft tool is written and self-tested** (`tpu/mtp-graft/kernel.py`, with the
  tool inlined so the Kaggle kernel carries it): it copies the base model's
  `blk.64.*` nextn block into Bonsai 2's PTQ1_0 file and adds
  `qwen35.nextn_predict_layers`, so llama.cpp's `--spec-type draft-mtp` finds the
  head inside the target. The head is published as its own GGUF
  (`unsloth/Qwen3.8-27B-GGUF` -> `MTP/mtp-Qwen3.8-27B-Q4_0.gguf`, 1.28 GB, 18
  tensors, of which 15 are the nextn block and three are shared weights the target
  already has). The tool is verified by a round-trip test on synthetic GGUFs, and
  **that test caught two real bugs**: tensor offsets in a GGUF are relative to the
  data section, so they must not be shifted when the header grows, and F32 is four
  bytes per element, not one.
- **`PREFETCH=1` does not compile** with this kernel's grid: Pallas reports a pytree
  arity mismatch, because `num_scalar_prefetch` wants the prefetched dimension to be
  a scalar dim and the grid is `(TOUT, 1, BPC)` with the parallel dim first. The
  `PREFETCH=0` control is `tpu/14-bonsai-transposed/receipt-14-pf0-b1-f16-tc1024.json`
  (5.52 tok/s, 181.3 ms/step). The lever's ceiling was 7.6 % anyway.
- **The TPU ceiling is now priced honestly.** A multi-token verification pass costs
  ~200 ms, so 50 tok/s needs 9-10 accepted tokens per pass -- an MTP head gives 2-3
  and n-gram matching under 1. The mechanism's honest ceiling on this hardware is
  ~10-15 tok/s; 50 tok/s on a TPU needs the **ternary unpack** to be ~9x faster
  (the step is ~20x off the slice's 9.0 ms traffic floor, and the unpack is 82 % of
  a projection). Filed as `TODO.md` `TPU-07` rather than promised.

### 2026-09-26 — `docs:` the hub's change history is no longer a phantom file

**`CHANGELOG.md` now exists.** Until today it was referenced everywhere and present
nowhere:

| Asserted by | What it said | On disk |
| --- | --- | --- |
| `README.md:164` | "What changed, dated, receipted" — a link in the *Working here* table | **absent** |
| `AGENTS.md` §5 | assigns this file its one job, and the delete-from-`TODO.md`-plus-a-line-here rule | **absent** |
| `TODO.md:16` | "Cite the ID … in the `CHANGELOG.md` line" | **absent** |
| `TODO.md:154` (`HUB-06` done-when) | "the `LOG.md` disposition is stated in `CHANGELOG.md`" — **unachievable** while the target file does not exist | **absent** |
| `.gitignore:13` | `!/CHANGELOG.md` allow-listed, so `git add` would already accept it | reserved |

This is the same defect class the ledger already tracks for `docs/policies/` (`HUB-03`) and
`scripts/` (`HUB-04`): a reservation plus a citation is not a file.

- **Evidence:** `Get-ChildItem -File` at the hub root listed `.gitignore`, `AGENTS.md`,
  `PROMPTS.md`, `README.md`, `TODO.md` and no `CHANGELOG.md`; `git ls-files` returns 5 files
  and it is not one of them.
- **What did *not* change:** any dimension score. The rubric had been awarding 80/100 for
  change history (`TODO.md`) while the file was missing, so creating it moves **no** points —
  it makes a number that was already being published honest. No bump was taken for it.
  (The published total *did* move, 74 % → 78 %, for a reason entirely separate from this
  entry — see the arithmetic correction below. Conflating the two would be exactly the
  double-count this bullet is here to prevent.)

**`LOG.md` disposition, as `HUB-06`'s done-when requires:** `LOG.md` was the hub's working
ledger (26 commits, 2026-07-12 → 2026-07-18) and was deliberately deleted in `c2e2535`
("chore: remove obsolete gitattributes file and task logging documentation", 2026-07-21). It
is **retired, not lost**. Its successors are `TODO.md` (open work) and this file (what
changed). The `!/LOG.md` line at `.gitignore:5` stays until `HUB-06` records a per-name
decision for all five reserved files; recreating `LOG.md` or `CURRENT_STATUS.md` would
re-create the two-records-of-one-event defect `AGENTS.md` §5 forbids, so this file takes no
action on it.

### 2026-09-26 — `fix:` the hub's headline score never added up

**74 % was wrong; so was 22 %.** Both were published as arithmetic, and neither expression
evaluated to the total printed under it.

| Published | The products it listed | Actual sum | Actual % |
| --- | --- | --- | --- |
| `= 7400 / 100 =` **74 %** | `85·20 + 80·18 + 85·18 + 80·12 + 70·15 + 65·12 + 60·5` | **7760** | **77.6 %** |
| `= 2235 / 100 =` **22 %** | `40·20 + 0 + 0 + 15·12 + 35·15 + 20·12 + 40·5` | **1945** | **19.45 %** |

- **executed:** the products were summed by script rather than re-added by hand, which is the
  only reason this was caught at all — the same file had been re-read several times today
  without anyone noticing. The seven weights sum to 100, as the table claims; only the
  dot-product totals were wrong.
- **Nothing was re-scored.** Every weight and every dimension score is exactly as written; the
  change is `7400 → 7760` and `2235 → 1945`. Anyone comparing this file to an older copy
  should expect the table to be byte-identical and the totals to differ.
- **The direction is worth stating plainly, because it is not the usual one.** Both errors
  *understated* the hub. This is not `AGENTS.md` §4's failure mode — nobody inflated anything —
  and the correction therefore cannot be waved away as a cosmetic edit in the author's favour.
  It is still a defect: the section's whole promise was "the number is arithmetic, not a mood",
  and a hand-added number in a document is an unverified claim.
- **Consequence for the path-to-100 table:** the remaining gap is **22.4** points, but the seven
  itemised increments sum to **26.0**. They were priced against the wrong base and overlap
  (`HUB-04`'s checker sits inside `MF-01`'s done-when *and* `DOC-01`'s tiering; the P2/BLOCKED
  tail is mostly `DIST-01`…`DIST-05` counted twice). The table now shows the overshoot to
  **103.6 %** rather than quietly re-cutting numbers until it lands on 100 — the increments are
  the useful part, the cumulative column is arithmetic on estimates.
- **What this argues for:** `HUB-04` (an executable checker over these claims). Every number in
  these four files is hand-maintained and this is the one that was load-bearing enough to be
  printed on the front page.

### 2026-09-26 — `docs:` repaired two truncated records and one stale measurement

- **`DOC-01`'s `Done when` was cut off mid-sentence** at `TODO.md:98` ("… states each"), and
  the remainder of that sentence was sitting **after** the license footer at `TODO.md:413`
  ("claim at its tier (`AGENTS.md` §4), and is linked from the `docs/README.md` page index.").
  The two halves are now rejoined and the orphan removed. The check is one complete sentence
  again, which matters because `Done when` is the thing another agent runs.
  - **Not closed:** rejoining the sentence does not write `docs/daemon.md`. `DOC-01` is still
    open, and the scoreboard total stays **29**.
- **`AGENTS.md` had the same defect.** Its commit-message bullet ended at "… shows legacy
  noise -" (`AGENTS.md:154`) with the continuation — the list of legacy subjects — stranded
  below its own license footer. Rejoined, orphan removed. Two files losing the tail of one
  sentence to the end of the file is one mechanism, not two: text appended after the footer
  and never re-read.
- **`README.md` had `## Quick start` twice** (`README.md:177` and `README.md:179`), with the
  content under the second. One of the duplicates is gone, and the footer now reads
  2026-09-26 rather than the 2026-08-26 it had carried since before the last hub commit.
- **Free space was stale, not conservative.** `AGENTS.md` §8 said "~5.2 GB free (99 % used)"
  citing `docs/space-review-2026-09-23.md:5`. `Get-PSDrive G` on 2026-09-26 measures
  **12.73 GB free / 464.2 GB used**. §8 now carries the new number and the old one, and keeps
  the review's conclusions, which the drift does not touch. Recorded rather than silently
  corrected because the 2026-09-23 figure is quoted elsewhere.

### 2026-09-26 — `feat:` goal 8 — `dx-forge` is now a build-time dependency of `bot`

Product work, owned by the `bot` repo per `AGENTS.md` §2.4; recorded here because it adds a
**cross-repo edge inside the hub** that the hub's own safety rules need to know about.

`bot/src-tauri` now carries `dx-forge = { path = "../../forge", optional = true }` behind a
new `forge-media` feature, off by default — the same shape `bot` already uses for
`metasearch`, `daemon`, `tui` and `hbb_common`.

- **executed — `cargo tree` at `G:\Dx\bot\src-tauri`, default features:** `dx-forge` absent
  from the tree. A clone of `bot` alone still builds.
- **executed — `cargo tree --features forge-media`:**
  `dx-forge v1.0.0 (G:\Dx\forge)` → `dx-config v0.1.0 (G:\Dx\serializer\crates\dx-config)` →
  `dx-serializer v1.0.0 (G:\Dx\serializer)`. 405 packages resolve.
- **Consequence for `AGENTS.md` §2.2 and the space review:** `forge/` and `serializer/` are
  now referenced *by path* from `bot`. Parking, renaming, or moving either one — including as
  part of any `docs/space-review-2026-09-23.md` proposal — silently breaks `bot`'s
  `forge-media` build. Added to `AGENTS.md` §8 as an environment fact.
- **Not done, stated plainly:** `cargo check --features forge-media` has **not** been run, so
  the crate compiling inside `bot`'s graph is unproven; `s2n-quic` and `octocrab` enter the
  graph and are unbuilt here. No `forge.*` op is reachable from the UI yet. Goal 8 is
  **1 of 6 tasks partially landed**, not closed, and `bot/TODO.md` carries that.
- **A trap found while reading the crate, recorded because it is a data-loss-shaped one:**
  `dx_forge::Repository::discover` does **not** fail when no repository exists — it *creates
  one* (`forge/src/core/repository.rs:59-64` initialises `.dx/forge` in the nearest `.dx`
  directory, or at the start path). A read-only "what is my sync status" call built on
  `discover` would therefore **write repository state into whatever folder was named**. Any
  status path in `bot` must locate `.dx/forge` read-only instead of calling `discover`. No
  such non-mutating helper exists in the crate today.

---

## Before this file

There is no imported history, and none is invented. The hub's recorded history is `git log`
(oldest visible `bffd49d`, newest `fb2f08f` "feat: automated commit", 2026-09-09), plus the
archived per-wave ledgers inside the `bot` product repo. Entries above start where a
`file:line` receipt starts.

MIT OR Apache-2.0
updated: 2026-10-20 20:10 UTC
