# DX_HARNESS.md — the plan for `G:\Dx\harness`

> **What this file is.** A plan, written to be argued with. It proposes a new
> `G:\Dx\harness` folder: a small, headless, fast-building Rust workspace that
> holds the dx-harness **engine** with no GUI and no TUI, where every feature
> can be tested with a CLI command instead of a mouse.
>
> **What this file is not.** Not a decision, and not a schedule. Nothing here has
> been built. Every "done" claim in this document is **repo-managed** — the code
> exists in some tree — and nothing is **executed** unless the table says so.

Companion files: `AGENTS.md` (how to work in this hub), `TODO.md` (the hub
ledger), `CHANGELOG.md` (what shipped), `docs/README.md` (the register).

---

## 1. The problem this solves

Today the harness engine lives in two places that both cost too much to test:

| Where | What it is | Cost to test |
|---|---|---|
| `G:\Dx\tui` | 172-member workspace, 92 `crates/codegen` crates | `cargo check -p xai-grok-pager-bin` needs a cold build; the tree's `target/` reached **27.78 GB** before `cargo clean` on 2026-10-20 |
| `G:\Dx\bot\crates\` | 103-crate vendored copy of the harness closure | Smaller, but **has never been compiled on this machine** — `bot/crates/target/` and `bot/src-tauri/target/` do not exist |

The 2026-10-20 session proved the cost concretely: a single cold
`cargo check` on tui ran 19 hours and died to a **disk-full** error, not a
compile error. Then `cargo test` died the same way. `G:` is now at
**~10 GB free**.

Meanwhile the features we actually want are **not testable as shipped**:

- `dx-harness` is **off by default** (`bot/src-tauri/Cargo.toml:113`;
  `default = ["win32-shell", "grammarcheck", "zeroclaw-channels"]`).
- `bot/src-tauri/src/dx_harness.rs` is gated `#![cfg(feature = "dx-harness")]`,
  and **nothing constructs `DxHarness`** — `bot/HARNESS.md` §0 says so plainly.
- The GUI cannot be driven in this environment, and neither can the TUI.

**The thesis:** a headless harness with a CLI surface can be tested by an agent
without a GUI, without a TUI, and without paying AI money for every check.

---

## 2. What already exists — do not rebuild it

This is the most important section, because it is the one that is easiest to
get wrong. **Most of the "5 new things" are already copied into
`G:\Dx\bot\crates\`, byte-identical to tui's current working tree, and
already committed.**

Measured 2026-10-21 by SHA-256 comparison of every file:

| # | Feature | tui file | In `bot/crates`? | Identical to tui? | Committed in bot? |
|---|---|---|---|---|---|
| 1 | Tolerant tool resolver | `crates/codegen/xai-grok-tools/src/tolerant.rs` | yes | **yes** | yes (`0030d4e6`) |
| 2 | `heal_note` plumbing | `crates/codegen/xai-grok-tools/src/registry/types.rs` | yes | **yes** | yes |
| 3 | `get_tool_details` in all 7 toolsets | `crates/codegen/xai-grok-agent/src/config.rs` | yes | **yes** | yes |
| 4 | Arg healing + ≤30-token notes | `crates/codegen/xai-grok-shell/src/session/helpers/tool_input_parsing.rs` | yes | **yes** | yes |
| 5 | Compact tool catalog (4,946→4,533 B) | `crates/codegen/xai-grok-agent/templates/dx_serializer_compact_tools.md` | yes | **yes** | yes |

Also already in bot: `suppress_tool_card` (hidden meta tools) at
`xai-grok-shell/src/session/acp_session_impl/tool_calls.rs:2773,3042,3384`.

**Whole-tree diff:** 2,590 `.rs` files exist in both trees. **35 differ, and
none of them is §0 work** — they are unrelated upstream churn
(`xai-fast-worktree/src/copy/cow.rs`, `xai-grok-agent/src/prompt/user_message.rs`,
`xai-grok-sampler/src/client.rs`, and 32 test files). Bot has **zero** tui-only
files under `xai-grok-shell/src`.

`bot/PROVENANCE.md` records the copy: 103 crates, source `G:\Dx\tui` @
`54891ed8`, Apache-2.0, three documented deviations.

### What is deliberately *not* in bot, and should stay out

| Not copied | Why |
|---|---|
| `xai-grok-pager`, `xai-grok-pager-bin`, `xai-grok-pager-render`, `xai-grok-pager-diff`, `xai-grok-pager-minimal`, `xai-grok-pager-pty-harness` | the ratatui/crossterm TUI — a pure ACP *client*. Bot replaces it with a GUI; the harness replaces it with a CLI. |
| `xai-ratatui-inline`, `xai-ratatui-textarea`, `xai-grok-mermaid`, `third_party/mermaid-to-svg` | TUI rendering |
| `common/dx` | the dx UI crate. Its `push_tool_block` skip for `ToolDetails` is a **GUI** concern and does not belong in a headless harness. |
| `xai-grok-markdown`, `xai-grok-markdown-core` | **except** `xai-grok-markdown-core`, which bot *does* need — see §7, this is a live bug. |

**Consequence for the `/model` picker and the session tab strip:** both live in
`xai-grok-pager` (`model_catalog.rs`, `views/session_tabs.rs`). They are
**TUI features, not harness features.** They do not belong in `G:\Dx\harness`.
They belong in tui, and in bot's GUI as its own surface. The harness supplies
the *engine* underneath both.

---

## 3. The proposed layout

```
G:\Dx\harness\                  <- new git repo, own origin
├── Cargo.toml                  <- workspace root, members = harness crates only
├── crates/
│   ├── codegen/                <- copied from tui, engine-only crates
│   ├── common/                 <- copied from tui, leaf crates
│   └── build/                  <- xai-proto-build (protobuf codegen helper)
├── cli/                        <- the NEW thing: small, fast, headless binaries
│   ├── dx-harness-cli/         <- one command per feature
│   └── ...
├── PROVENANCE.md               <- copy origin, revision, deviations (model on bot's)
├── TODO.md
├── CHANGELOG.md
└── AGENTS.md
```

### The `cli/` crate is the point

Every feature gets **one small binary** that exercises it against a real
provider and prints a receipt. No GUI, no TUI, no mouse. This is the pattern
`tui/crates/codegen/xai-rlm` already uses — 12 example binaries
(`simple_test.rs`, `recursive_reduce.rs`, `phase1_demo.rs`, …) in one crate.

Proposed commands, one per planned feature:

| Command | Exercises | Done when |
|---|---|---|
| `dx-harness-tool-route` | smart tool routing: which tools get sent this turn, and why | prints the selected tool set + token delta vs. full catalog |
| `dx-harness-resolver` | tolerant resolver + arg healing | prints healed calls, the ≤30-token note, and the single-pass success rate |
| `dx-harness-small-first` | small-first prompt: a cheap model for a cheap job | prints which model handled which sub-task and the cost |
| `dx-harness-token` | token-efficiency: prefix cache hit rate, prompt compression | prints before/after tokens from `dx-token.exe` |
| `dx-harness-rlm` | recursive long-context runtime | prints the reduction tree and the answer |
| `dx-harness-git` | git harness: worktree isolation for parallel agents | prints worktree create/merge round-trip |
| `dx-harness-turn` | one full agent turn, headless, over ACP channels | prints the turn end-to-end with no UI |

---

## 4. The five feature areas — what is real today

Measured, not assumed. "Real" means the code exists and reads as an
implementation, not that it has been executed.

### 4.1 Smart tool routing

| Where | State |
|---|---|
| `tui/crates/common/token/tool-router/` | **246 lines, 0 tests.** Dynamically selects relevant tools per turn instead of sending all 50+. Its own doc header says *"Evidence (TOKEN.md — REAL): 50 tools, send only 5 relevant = 90% schema token savings"* and warns *"keyword-based routing is fragile; future: use classifier"*. |
| `tui/crates/common/token/reasoning-router/`, `batch-router/`, `multimodal-router/` | same family, same shape: ~1 file, 0 tests |
| `tui/crates/common/token/core/` (`dx-token-core`) | 370 lines, 0 tests |
| bot's copy | **`tool-router` is not in bot** — bot has only `common/token/route` out of 47 token crates |

**This is the largest real gap.** 46 of tui's 47 `common/token/*` crates are
missing from bot. They total **0.9 MB** of source — cheap to copy — but 6 of
the largest have **zero tests**, so copying them copies untested scaffolding.

### 4.2 RLM (recursive long-context runtime)

| Where | State |
|---|---|
| `tui/crates/codegen/xai-rlm/` | 1,959 lines, 12 example binaries. Description: *"Embeddable recursive long-context runtime for question answering, summarization, and agent-context preparation."* **In bot already, byte-identical.** |
| `tui/crates/common/token/rlm/` | 17 files, 2,536 lines. **Not in bot.** |
| `tui/crates/codegen/xai-grok-tools/src/implementations/grok_build/rlm/` | tool-side RLM implementation |

### 4.3 Token efficiency

| Where | State |
|---|---|
| `tui/crates/common/token/` | **47 crates, 0.9 MB total.** Includes `prefix-cache` (299 lines), `prompt-compress` (231), `token-budget` (282), `semantic-cache`, `compaction`, `governor`, `live-token-prune`, `retrieval-budget`, `response-cache`, `whitespace-normalize`, `schema-minifier`, `output-truncator`, and 12 modality compressors (video/pdf/ocr/audio/vision). **Only 2 crates have tests** (`prompt-compress` 2, `semantic-cache` 1). |
| `tui/crates/common/xai-grok-token-optimization/` | the transforms crate. **In bot already.** |
| `tui/crates/codegen/xai-grok-agent/templates/dx_serializer_compact_tools.md` | the 28-tool compact catalog, 4,533 B ≈ 1,133 tokens. **In bot already.** |
| `G:\Dx\bin\dx-token.exe` | **already built and working.** `dx-token count --ai <file>` counts with the repo's own tokenizer. This is the measurement tool and it already exists — no new work needed. |

### 4.4 Small-first prompt

| Where | State |
|---|---|
| `tui/crates/codegen/xai-grok-shell/src/util/config/resolve/session_recap.rs` | **the precedent.** `SessionRecapConfig { model, reasoning_effort, user_message_max_chars, agent_reply_max_chars, transcript_max_chars, timeout_ms }` — a cheap model on a cheap job, at low effort, with no tools. |
| `tui/crates/codegen/xai-grok-shell/src/session/acp_session_tests/recap_display_only_tests.rs:797` | the test that proves it: `recap_request_uses_small_model_and_compact_transcript` |
| `tui/crates/common/token/reasoning-router/` | the routing half — exists, untested |

**The generalisation is the work.** `SessionRecapConfig` is one hand-configured
case. "Small-first" as a *policy* means: classify the sub-task, pick the
cheapest model that can do it, cap effort, strip tools, and record the
decision. That policy does not exist yet in any tree.

### 4.5 Git harness

| Where | State |
|---|---|
| `tui/crates/codegen/xai-fast-worktree/` | worktree isolation for parallel agents. **In bot already.** 3 of its files differ from tui's working tree — unrelated upstream churn (`copy/cow.rs`, `git/safety_tests/gate.rs`, `worktree/plan.rs`). |
| `tui/crates/codegen/xai-grok-workspace/` | the workspace daemon. **In bot already.** |

### 4.6 The tool layer (already done, needs proving)

The §0 work from tui — tolerant resolver, arg healing, in-context learning
loop, hidden meta tools, compact catalog — is **fully present in bot** (§2
above). It needs a *receipt*, not more code.

---

## 5. Provider access — "my OpenCode key, CLI only"

### What is available on this machine, measured

| Fact | Value |
|---|---|
| `opencode` CLI | `G:\Dev\Tools\Scoop\apps\nodejs\26.4.0\opencode.ps1` — supports `opencode run --prompt "<text>" --format json --auto --model provider/model` |
| opencode config | `%USERPROFILE%\.config\opencode\opencode.json` — providers `gmi` and `tokenharbor`; `opencode.jsonc` sets `model` and `small_model` to `opencode-go/muse-spark-1.3-contributor` |
| `gmi` | base `https://api.gmi-serving.com/v1`, models `MiniMaxAI/MiniMax-M3`, `MiniMaxAI/MiniMax-M2.7`, `Qwen/Qwen3.8-27B` |
| `tokenharbor` | base `https://tokenharbor.ai/v1`, models `deepseek-v4.1-flash:free`, `deepseek-v4.1-flash`, `th-orchestra` |
| `TOKENHARBOR_API_KEY` | **set**, 73 chars |
| `DEEPSEEK_API_KEY` | **set**, 35 chars |
| `GMI` key | **not found** as an env var and **not** in `opencode.json` — the `gmi` provider has a base URL and model list but no visible credential |
| `opencode` auth store | `%USERPROFILE%\.local\share\opencode\` — **no `auth.json`**. Credentials are in env vars and in the provider config. |
| `opencode.db` | 16.62 GB. **This is a large disk consumer in the user's home dir, unrelated to the harness.** |

### The design

Two access paths, both CLI-only:

1. **In-process, via the harness's own provider layer.** The harness talks to
   any OpenAI-compatible endpoint directly. `tokenharbor` and `gmi` are both
   `@ai-sdk/openai-compatible` — plain `/v1/chat/completions`. The harness
   already has the machinery: `xai-grok-shell/src/agent/community_models.rs`
   builds `ModelEntryConfig` entries with `base_url` + `api_key` +
   `extra_headers`, and the Zen free lane (`space-bunny-free`) is already
   wired through it. **In bot already, byte-identical.**
2. **Out-of-process, via `opencode run --format json`.** For anything that
   needs OpenCode's own routing, skills or subagents.

**Cost discipline.** The user's point is that AI spend should go to *real*
verification, not to running the test suite. The split:

| Spend AI on | Do not spend AI on |
|---|---|
| The **golden corpus** — 200–500 real tool calls run against a live mid model, measuring single-pass success, silent-wrong rate, and token delta | Pure-function tests (resolver, arg healing, compaction, prefix cache) — these must run offline, with a mock inference server |
| One end-to-end turn per feature, on a real provider | Every regression run |

The harness already has the mock: `xai-grok-test-support` provides
`MockInferenceServer` (used by the recap test above). It is **in bot already**.

### The golden corpus is the missing measurement

`tui/TODO.md` §0a is blocked on exactly this:

> *Tolerant execution model: prefer execution with safe fallbacks over strict
> `invalid_arguments → retry`; validation failures that cannot be healed keep
> the current graceful error. **Measure:** single-pass success up, silent-wrong
> ≈ 0, token delta vs full-schema baseline (**golden corpus** × local/cloud
> models).*

**No golden corpus exists in any tree.** A repo-wide search for `*golden*` and
`*corpus*` outside `target/` returns three hits, none of them a tool-call
corpus. **This is the one deliverable that makes §0a measurable, and it is
exactly the thing a headless harness CLI is for.**

---

## 6. Making builds fast — and not running out of disk

This is the part that killed the 2026-10-20 session. Treat it as a hard
constraint, not an optimisation.

### Current state

| Fact | Value |
|---|---|
| `G:` free | **~10 GB** (was 26.35 GB after `cargo clean`, now 10 GB — `hackin` +15.19 GB and `forkness` +9.33 GB grew since) |
| `G:\Dx\tui\target` after clean | 0 GB (was 27.78 GB) |
| `sccache` | **0.17.0 at `G:\Dx\bin\sccache.exe` — installed, never configured** (`RUSTC_WRAPPER` unset) |
| `CARGO_INCREMENTAL` | unset in this shell; tui's `justfile` sets it to `1` |
| `CARGO_TARGET_DIR` | unset — every project builds into its own `target/` |
| `G:\Dx\bot\crates\target` | **does not exist** |
| `G:\Dx\bot\src-tauri\target` | **does not exist** |

### The plan

| Lever | Effect |
|---|---|
| **One shared `CARGO_TARGET_DIR`** for `harness` + `bot/crates` | the 103-crate closure compiles **once**, and bot's harness copy becomes a warm cache instead of a second cold build |
| **`RUSTC_WRAPPER=G:\Dx\bin\sccache.exe`** | sccache is already on disk. It is the single biggest win for *repeated* builds, because harness development is iterative, not one-shot |
| **`CARGO_INCREMENTAL=1`** | matches tui's `justfile` |
| **A dedicated disk for `target/`** | `target/` is regenerable output. Point it at a volume with room rather than fighting `G:` |
| **Never a cold build at `-j 12` or `-j 4`** | tui's `justfile:61-67` records the measured failure: `-j 12` dies with exit code 1 and **zero cargo output**; `-j 4` gives `rustc-LLVM ERROR: out of memory` on `xai-grok-pager`. `-j 2` is the only safe path |
| **A `just`-style recipe set** | `just check`, `just test`, `just cli <name>` — one command, no arguments to remember |
| **`--offline` is not an option** | `tui/TODO.md:494` records that test deps are not all cached |

### The disk truth nobody wants to hear

`G:` is at ~10 GB free with `hackin`, `forkness`, `esportsfire` and `codebase`
all growing. **A second cold build of the 103-crate closure does not fit.** The
harness must therefore start by **reusing** what exists — sharing a target dir
with `bot/crates`, and getting `bot/crates` built **once**, properly, before
anything new is compiled.

`G:\Dx\bot\TODO.md` §2 claims the build was "GREEN on Windows, EXIT 0, 0
errors" on 2026-10-11. That receipt is real but the artifacts are gone (no
`target/`), and that ledger line is over a month old.

---

## 7. Known bugs to fix on the way

These are measured, and two of them are in files this plan touches.

| # | Where | Problem |
|---|---|---|
| 1 | `bot/src-tauri/Cargo.toml:274` | `xai-grok-markdown-core = { path = "../../tui/crates/codegen/xai-grok-markdown-core" }` — the **only** dep that escapes the vendored copy. Everything else points at `../crates/codegen/…`. `xai-grok-markdown-core` **is** vendored in bot. This line makes bot's build silently depend on tui's tree — the exact coupling a standalone harness is supposed to break. |
| 2 | `bot/src-tauri/Cargo.toml:324-326` | `[patch.crates-io]` → `libsodium-sys` and `libxdo-sys` at `../../../Rustdesk/libs/…`. Cargo reads `[patch]` **regardless of features**, so a default build needs `G:\Rustdesk` present. `bot/HARNESS.md` §0 confirms: *"resolution fails on any machine that is not this one."* |
| 3 | `bot/src-tauri/Cargo.toml:113` | `dx-harness` is not in `default`. Nothing constructs `DxHarness`. |
| 4 | `bot/HARNESS.md` §0 | The duplicate `zeroclaw-*` crates under `crates/common/agent/` are labelled 1.0.0 but are an **older, trimmed** snapshot of the 0.8.5 copies. They coexist only because `dx-harness` is off; enabling it links two incompatible `zeroclaw-config` types. |
| 5 | `bot/crates/Cargo.toml` | Auto-assembled from tui's root manifest, 103 members. Its header says *"Do not hand-edit the deps table; regenerate it from upstream."* Any new crate needs the generator, not a manual edit. |
| 6 | `bot/TODO.md:86-89` | *"Use **Bash**, not PowerShell — PowerShell's ConPTY sandbox is broken on this machine."* True on 2026-10-11; **not true today** — `pwsh` 7.6.2 runs fine and `just --list` resolves. Needs re-measuring before it is trusted as a constraint. |

---

## 8. The sequence

Each step names what it proves. Steps 0–2 are prerequisites, not features.

| # | Step | Done when |
|---|---|---|
| 0 | **Free disk, deliberately.** Report the space situation; propose, do not delete. | `G:` has headroom for one closure build |
| 1 | **Build `bot/crates` once**, cold, with `CARGO_INCREMENTAL=1` and a shared target dir | `cargo check --workspace --all-targets` exits 0, and the receipt is written down |
| 2 | **Build `bot/src-tauri` with `--features dx-harness`** | the link is proven, or the `zeroclaw-config` collision is named and fixed |
| 3 | Create `G:\Dx\harness` — a git repo with its own origin, `Cargo.toml`, `PROVENANCE.md`, `TODO.md`, `CHANGELOG.md`, `AGENTS.md` | `git -C G:\Dx\harness status -sb` shows a clean repo with a real remote |
| 4 | Copy the **harness-only** closure from tui (tui's engine crates + `common/token/*` + `build/`), *not* from bot's stale copy | `cargo metadata --no-deps` exits 0; the closure excludes every `pager`/`ratatui` crate |
| 5 | Add the `cli/` crate with the first command (`dx-harness-turn`) | one real agent turn completes headless, with output |
| 6 | Build the **golden corpus** runner | the corpus is committed, and the §0a measurement (single-pass success, silent-wrong, token delta) has a number behind it |
| 7 | Add the remaining commands, one per feature | each command has a receipt in `CHANGELOG.md` |
| 8 | Implement the features that are **not** yet written: small-first *policy*, tool-router *with tests*, RLM-in-bot, prefix-cache wiring | each has tests that fail without it |
| 9 | Only then: update `bot` (GUI) and `tui` (TUI) to consume the harness | the GUI and TUI stop carrying their own copies of the engine |

---

## 9. Where the harness goes afterwards

This is the argument for doing it as a **library with a CLI**, not as an app.

| Target | How the harness reaches it | Constraint |
|---|---|---|
| `G:\Dx\bot` (Tauri GUI) | in-process, over `xai_acp_lib::acp_channels()` — the seam `bot/src-tauri/src/dx_harness.rs` already implements | `!Send` types cannot cross `tokio::spawn`; needs a `LocalSet` |
| `G:\Dx\tui` (TUI) | in-process, ACP client — the TUI is already a pure ACP client | the TUI keeps its own pager crates; only the engine is shared |
| `G:\Dx\extensions` (VS Code) | **TypeScript** (5,150 `.ts` files, `@cline/packages`) — needs a **WASM** build of the harness | the harness must compile to `wasm32`; the `tokio` + `reqwest` + `git2` stack is the obstacle, not the design |
| `G:\Dx\code` (code editor) | Rust (`crates/acp_thread`, `crates/acp_tools`, `crates/agent`, …) — can link natively | same `!Send` caveat |
| Website | WASM, same as extensions | |
| Android / iOS | the harness's `xai-grok-sandbox` is already platform-aware; mobile needs the JNI/ObjC seam that `bot`'s grammar work already solved | |

**The honest constraint:** WASM for the website and the VS Code extension is
**not** a copy problem, it is a *dependency* problem. `git2`, `reqwest` with
TLS, `tokio` with threads, and `rusqlite` all need to be either replaced or
made optional. That is a large piece of work with its own plan — it should
**not** be smuggled into this one. The realistic order is: native Rust targets
first (bot, tui, code), then WASM as a separate track with its own decision.

---

## 10. Open questions — these need answers before step 3

1. **Should `G:\Dx\harness` be a new git repo with its own `origin`
   (`essence-dx/harness`), or a folder inside the hub?** The hub is deny-by-
   default (`.gitignore` line 4 is `/*`), so a child folder needs its own repo.
   A new repo is the cleaner answer and matches every sibling product.
2. **Should bot's vendored `crates/` be deleted in favour of a path dependency
   on `G:\Dx\harness`?** That is the clean end-state, but it makes bot
   unbuildable on any machine without the hub checked out — which is exactly
   the "fresh clone cannot be built" problem `bot/HARNESS.md` §0 already
   complains about. The alternative is to publish the harness to crates.io.
3. **Which is the source of truth — tui or the harness?** Today tui holds the
   engine and bot holds a copy. If the harness becomes the source, tui's
   `crates/codegen/*` become a *downstream* copy and every upstream merge gets
   harder. This is the single most consequential decision in this document.
4. **How many of the 47 `common/token` crates are actually wanted?** 46 are
   missing from bot, they total 0.9 MB, and most have zero tests. Copying all
   47 copies untested scaffolding. Copying 5 copies a policy, not a pile.
5. **What is the OpenCode key, and which provider?** `tokenharbor` and
   `deepseek` have keys in env. The `gmi` provider — the one offering
   MiniMax-M3 and Qwen3.8-27B — has **no visible credential**. If the harness
   is to use "my OpenCode key", this needs naming.

---

## 11. Claim ledger for this document

| Claim | Tier | Evidence |
|---|---|---|
| The 5 §0 features are in `bot/crates`, byte-identical to tui | **repo-managed** | SHA-256 comparison, 2026-10-21; bot commit `0030d4e6` |
| 35 of 2,590 shared `.rs` files differ, none §0 work | **repo-managed** | file-by-file hash diff |
| 46 of 47 `common/token` crates are absent from bot | **repo-managed** | directory listing |
| `tool-router` and 5 other token crates have **0 tests** | **repo-managed** | `#[test]` count per file |
| `dx-harness` is off by default and nothing constructs `DxHarness` | **repo-managed** | `bot/src-tauri/Cargo.toml:113`; `bot/HARNESS.md` §0 |
| `bot/crates` has never been compiled on this machine | **repo-managed** | `bot/crates/target` and `bot/src-tauri/target` do not exist |
| `G:` is at ~10 GB free | **executed** | `Get-PSDrive G`, 2026-10-21 |
| `sccache` 0.17.0 is installed but `RUSTC_WRAPPER` is unset | **executed** | `G:\Dx\bin\sccache.exe --version`; env inspection |
| `TOKENHARBOR_API_KEY` and `DEEPSEEK_API_KEY` are set; the `gmi` key is not | **executed** | env inspection, 2026-10-21 |
| The harness closure builds and its tests pass | **not yet claimed** | step 1 |
| The §0a golden-corpus measurement exists | **not yet claimed** | step 6 |
| WASM works for the website / VS Code extension | **not yet claimed** | §9 — a separate track |

---

MIT OR Apache-2.0
updated: 2026-10-21 00:00 UTC
