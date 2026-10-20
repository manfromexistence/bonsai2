# DX_HARNESS.md — the dx-harness: what it is, where it is, what is left

> **What this file is.** The status page for the DX agent harness: what the
> harness is, which tree owns it, what has been built, what is proven, and
> what is still open. Every claim names its tier — **repo-managed** (the code
> exists), **executed** (something ran and produced the number), or
> **not yet claimed**.
>
> **What this file is not.** Not a plan and not a promise. It is a record.
>
> Companion files: `AGENTS.md` (how to work in this hub), `TODO.md` (the hub
> ledger), `CHANGELOG.md` (what shipped), `docs/README.md` (the register).

---

## 1. The short version

The **dx-harness** is the agent engine: the thing that plans work, calls
tools, reads and writes files, runs commands, and holds a conversation. It is
the **grok-build** harness (upstream `xai-org/grok-build`, Apache-2.0) with
the DX additions on top.

It currently lives in **three** trees. That is the problem this page exists
to make legible.

| Tree | What it is | Built? | Tests run? |
|---|---|---|---|
| `G:\Dx\tui` | **the upstream source.** A hard fork of grok-build, 172 members, 92 `crates/codegen`. Merges upstream regularly. | `cargo check -p xai-grok-pager-bin` passed 2026-10-20 (1m59s, 0 errors) **after** a 19-hour cold build | never on this HEAD |
| `G:\Dx\bot\crates\` | **the working copy.** Bot's vendored closure — 103 crates, plus fixes tui does not have (§2.3). | **never** — no `target/` dir has ever existed here | never |
| `G:\Dx\harness` | **the test copy**, mirroring bot. 125 members, own git repo, 3 commits. | **green** — `dx-harness.exe` builds and runs | **yes** — resolver 9/9, corpus 13/13 offline |

**`G:\Dx\harness` is the only one that can be tested.** The other two cannot
be tested quickly: building tui took 19 hours and died to a disk-full error,
and bot's copy has never been compiled at all.

---

## 2. What was actually found before building anything

### 2.1 The five DX features are already in bot's copy

This was the single biggest surprise, and it changed the plan. Measured
2026-10-21 by SHA-256 comparison of every file:

| # | Feature | tui file | In `bot/crates`? | Identical to tui? | Committed in bot? |
|---|---|---|---|---|---|
| 1 | Tolerant tool resolver | `crates/codegen/xai-grok-tools/src/tolerant.rs` | yes | **yes** | yes (`0030d4e6`) |
| 2 | `heal_note` plumbing | `crates/codegen/xai-grok-tools/src/registry/types.rs` | yes | **yes** | yes |
| 3 | `get_tool_details` in all 7 toolsets | `crates/codegen/xai-grok-agent/src/config.rs` | yes | **yes** | yes |
| 4 | Arg healing + ≤30-token notes | `crates/codegen/xai-grok-shell/src/session/helpers/tool_input_parsing.rs` | yes | **yes** | yes |
| 5 | Compact tool catalog (4,946→4,533 B) | `crates/codegen/xai-grok-agent/templates/dx_serializer_compact_tools.md` | yes | **yes** | yes |

Plus `suppress_tool_card` (hidden meta tools) at
`xai-grok-shell/src/session/acp_session_impl/tool_calls.rs:2773,3042,3384`.

**Whole-tree diff:** 2,590 `.rs` files in both trees → **35 differ, and none
of them is §0 work.** They are unrelated upstream churn
(`xai-fast-worktree/src/copy/cow.rs`, `xai-grok-agent/src/prompt/user_message.rs`,
`xai-grok-sampler/src/client.rs`, and 32 test files). Bot has **zero** tui-only
files under `xai-grok-shell/src`.

So the work was **not** "copy five things into bot". It was already done.

### 2.2 What is genuinely missing

| Gap | Measurement |
|---|---|
| **46 of tui's 47 `common/token/*` crates are absent from bot** | bot has only `common/token/route`. The 47 total **0.9 MB** of source. |
| **Six of those token crates have zero tests** | `tool-router` (246 lines), `token-budget` (282), `prefix-cache` (299), `core` (370) — all **0 `#[test]`**. `prompt-compress` has 2. `compaction` has 0. |
| **No golden corpus exists in any tree** | a repo-wide search for `*golden*` / `*corpus*` outside `target/` returns 3 hits, none of them a tool-call corpus. This is the one thing blocking the §0a measurement. |
| **The small-first prompt policy does not exist** | only one hand-configured case, `SessionRecapConfig` |
| **`bot/crates` has never been compiled** | no `target/` dir, in bot or in `bot/src-tauri` |

### 2.3 Bot's copy is the better base — and it was the wrong call to ignore it

**This was corrected on 2026-10-21, and the correction matters.**

The harness tree was first copied from **tui**. Bot's vendored copy is
better, in three concrete ways:

1. **The `ReasoningEffort::Max` fix.** Tui maps `Max` to
   `async_openai::ReasoningEffort::Max` and needs a `[patch.crates-io]`
   pointing async-openai at a **git fork** that has that variant — a
   dependency a fresh clone cannot resolve. Bot maps `Max` and `Xhigh` both
   onto `Xhigh` **in code**, with a documented rationale, and needs **no
   patch at all**.

   ```rust
   // `Xhigh` and `Max` both exceed `High`; the dependency's top level is
   // `Xhigh`, so both map onto it.
   Self::Xhigh | Self::Max => crate::rs::ReasoningEffort::Xhigh,
   ```

2. **Windows-vs-unix test portability fixes in 9 files that tui does not
   have** — `xai-grok-config` (fs_atomic ×2), `xai-grok-hooks`
   (runner/command), `xai-grok-lifecycle` (×3), `xai-grok-sandbox`
   (example), `xai-grok-tools` (bash/mod), `xai-grok-workspace`
   (workspace_server). **This machine is Windows.** Bot's `TODO.md` §2 lists
   this work as done; tui never got it.

3. **A live `#[test]`** in `xai-grok-sampler/src/client.rs` that tui lacks.

**The two trees have diverged in BOTH directions.** Measured file by file
2026-10-21: **2,576 identical, 9 newer in bot, 5 newer in tui.**

So `G:\Dx\harness` mirrors **bot's** `crates/`, and
`scripts/sync_from_bot.sh` is the re-sync. The **six token crates** are the
exception — they are in tui and **not** in bot, so the sync re-adds them
from tui.

### 2.4 What is correctly absent from bot, and stays absent

| Not copied | Why |
|---|---|
| `xai-grok-pager` + the 5 other pager crates | the ratatui/crossterm TUI — a pure ACP *client*. Bot replaces it with a GUI; the harness replaces it with a CLI. |
| `xai-ratatui-inline`, `xai-ratatui-textarea` | terminal rendering |
| `xai-grok-mermaid`, `xai-grok-markdown`, `xai-grok-markdown-core` | rendering, TUI-side |
| `common/dx` | the dx UI crate. Its `push_tool_block` skip for `ToolDetails` is a GUI concern. |

**Consequence:** the `/model` picker (`model_catalog.rs`) and the session tab
strip (`views/session_tabs.rs`) live in **`xai-grok-pager`**. They are **TUI
features, not harness features.** They do not belong in the harness. The
harness supplies the engine underneath both.

---

## 3. `G:\Dx\harness` — what exists

Created 2026-10-21. Its own git repo, **no commits yet** (12 untracked
files). 0.07 GB of source.

```
G:\Dx\harness\
├── Cargo.toml              generated - do not hand-edit
├── Cargo.lock
├── rust-toolchain.toml     1.96.0, same pin as tui
├── cli/dx-harness-cli/     the headless CLI (new)
├── crates/                 the engine (125 members)
│   ├── codegen/            72
│   ├── common/             15 + token/ 7
│   ├── prod-mc/            1
│   └── build/              1
├── scripts/
│   ├── gen_workspace.py    generates Cargo.toml from tui
│   ├── sync_from_tui.sh    re-syncs the closure from tui
│   └── dev.sh              the build/test recipes
├── PROVENANCE.md  README.md  TODO.md  CHANGELOG.md  AGENTS.md  .gitignore
```

### 3.1 The copy

125 workspace members: **bot's 103 crates**, plus the **six token crates**
from tui (bot does not have them).

**20 TUI/GUI crates deliberately absent**, and **12 upstream workspace
dependencies dropped** for the same reason. Every drop is listed in
`G:\Dx\harness\PROVENANCE.md` §4.3.

**The six token crates are new to this copy** (bot does not have them):
`dx-token-core`, `tool-router`, `prefix-cache`, `prompt-compress`,
`token-budget`, `compaction`.

**Six out-of-tree relative paths rewritten** — `../../../prod/mc/…` →
`../../prod-mc/…` in the six crates that need the types crate.

**The workspace root is generated, not hand-written.**
`scripts/gen_workspace.py` takes **bot's** manifest as the source of the
workspace settings, and tui's only for the workspace dependencies bot's
pruned manifest omits. It:

1. copies `[workspace.package]` and `[workspace.lints.clippy]` verbatim;
2. keeps every `[workspace.dependencies]` entry whose `path` exists here;
3. rewrites the path prefixes — **source-aware**, because tui's paths are
   `crates/…` and bot's are `codegen/…` (bot's workspace root *is* its
   `crates/` directory);
4. **takes bot's version pins where the two disagree** — tui pins
   `rusqlite = "0.37"`, bot pins `"0.39"`, and bot's crates need 0.39;
5. copies `[patch.crates-io]` **only if the source has one** — bot's code
   does not need it, so the harness's manifest has none;
6. adds the entries neither manifest declares: `prod-mc-cli-chat-proxy-types`
   and the six token crates;
7. emits `[workspace.members]` from the directories actually present.

**The sync is a mirror, not an overlay.** `scripts/sync_from_bot.sh` removes
crates the source dropped, because bot's manifest is pruned and would not
declare the leftovers.

### 3.2 The CLI — `dx-harness`

One binary, thirteen subcommands. **No GUI, no TUI, no mouse.**

| Command | What it proves | Costs AI? |
|---|---|---|
| `features` | the command list | no |
| `model "<prompt>"` | a real model answers, end to end | **yes** |
| `tool-route "<request>"` | smart tool selection + the token delta | no |
| `token "<text>"` | prompt compression, measured | no |
| `prefix-cache "<prefix>"` | the stable-prefix key | no |
| `budget "<model>"` | the token budget for a model | no |
| `compact "<text>"` | conversation compaction | no |
| `resolver` | the tolerant resolver, against a real tool catalog | no |
| `corpus-offline` | the resolver over the corpus's near-misses | no |
| `corpus` | the §0a measurement: a real model, the real resolver | **yes** |
| `small-first "<task>"` | the cheapest-model policy | **yes** (one word) |
| `rlm` | the recursive long-context runtime | no |
| `git` | worktree isolation for parallel agents | no |

**Only three commands spend AI money, and all three are optional.** The
token pipeline and the resolver must be provable *offline*, on every run, at
zero cost. That split is the point.

### 3.3 Model access — the route that actually works

**The trap.** The OpenCode Zen public lane refuses direct calls:

```
POST https://opencode.ai/zen/v1/chat/completions
-> HTTP 403 {"type":"error","error":{"type":"FreeTierError",
     "message":"OpenCode's free tier can only be used from within OpenCode."}}
```

**With `Bearer public` and with a valid `oc_sk_` key alike.** The free models
are not on that path.

**The route that works** is the "go" inference gateway — the URL the
models.dev catalog names for the `opencode-go` provider:

```
https://opencode.ai/inference/go/openai/v1/chat/completions
```

It needs **two** headers, both mandatory:

| Header | Value | If missing |
|---|---|---|
| `Authorization` | `Bearer <oc_sk_ key>` | 401/403 |
| `x-opencode-session` | a **fresh id per request** | `400 MissingSessionID` |

Reusing one session id makes the *second* request on it fail. Verified
2026-10-21 with `minimax-m3.1-flash-free` → **HTTP 200, `HARNESS_OK`**.

The CLI has three routes:

| Route | How | Reaches |
|---|---|---|
| `OpenCodeGo` (default) | direct HTTP to the go gateway | `minimax-m3.1-flash-free` and the other free models |
| `OpenCode` (fallback) | shells out to `opencode run --format json --auto` | `opencode-go/*` models |
| `Direct` | direct HTTP to any OpenAI-compatible endpoint | `tokenharbor`, `deepseek`, anything keyed |

**The curl-only path**, which needs no build at all:

```bash
bash scripts/model.sh "Reply with exactly: HARNESS_OK"
# HARNESS_OK
```

### 3.4 Credentials on this machine

| Fact | Value |
|---|---|
| The `oc_sk_` key | written to `C:\Users\Computer\.dx\opencode-key` (51 bytes, mode-restricted). Read from there, or from `$OPENCODE_API_KEY`. Never printed, never written by a tool. |
| OpenCode Console | **stored** (OAuth, `opencode auth export` → `integrationID: opencode`) |
| `TOKENHARBOR_API_KEY` | **set**, 73 chars |
| `DEEPSEEK_API_KEY` | **set**, 35 chars |
| `gmi` provider (MiniMax-M3, Qwen3.8-27B) | base URL + model list present in `opencode.json`, **but no key found** |
| `opencode` CLI | `G:\Dev\Tools\Scoop\apps\nodejs\26.4.0\opencode.ps1` |
| `opencode` Zen public catalog | **85 models**, `GET https://opencode.ai/zen/v1/models` with `Bearer public` → HTTP 200 |
| `opencode.db` | **16.62 GB** in the user profile — a large disk consumer unrelated to the harness |

### 3.5 Build speed — the levers

| Lever | State |
|---|---|
| `sccache` 0.17.0 at `G:\Dx\bin\sccache.exe` | **installed and never configured.** Now wired as `RUSTC_WRAPPER`. **1,255 cache hits** over the session. |
| `CARGO_INCREMENTAL` | **must be unset** when sccache is the wrapper — sccache refuses: *"incremental compilation is prohibited."* The first build attempt died on exactly that. |
| `CARGO_TARGET_DIR` | unset — every project builds into its own `target/` |
| `-j 2` | the only safe job count. `-j 12` dies with exit code 1 and **zero cargo output**; `-j 4` gives `rustc-LLVM ERROR: out of memory`. Measured on this machine (`tui/justfile:61-67`). |
| `rust-toolchain.toml` | **added** — pins 1.96.0 like tui. Without it the build used `stable` and the two trees compiled the same code with different compilers. |
| **Rebuild time** | **7.5 s** after the cold build. |

### 3.6 The `engine` feature is off by default

The full engine (`xai-grok-shell`) pulls in `xai-codebase-graph` → `mimalloc`
→ a **C build**, and this machine's environment has zig's `INCLUDE`/`LIB`
pointing at the MSVC compiler, which breaks `cc-rs`:

```
c1xx: fatal error C1083: Cannot open source file: '/WX-'
```

So the default build links only the token crates and `xai-grok-tools` — no C
compiler needed. The engine is behind `--features engine`.

---

## 4. The §0a measurement — the golden corpus

This is the one deliverable that makes `G:\Dx\tui\TODO.md` §0a measurable,
and it exists in **no other tree**. §0a was blocked on exactly this:

> *Tolerant execution model: prefer execution with safe fallbacks over strict
> `invalid_arguments → retry`; validation failures that cannot be healed keep
> the current graceful error. **Measure:** single-pass success up, silent-wrong
> ≈ 0, token delta vs full-schema baseline (golden corpus × local/cloud
> models).*

### 4.1 What was built

`G:\Dx\harness\corpus\golden.json` — **13 entries**. Each declares a real
tool call, what the harness *should* do with it, and a `near_miss` to feed
the resolver.

Two runners, and the split is the point:

| Command | Cost | What it measures |
|---|---|---|
| `dx-harness corpus-offline` | **free** | every `near_miss` fed to the **real** resolver, checked against the expected tool. Deterministic. |
| `dx-harness corpus` | one model call per entry | the model picks a tool itself; the real resolver runs over its answer |

The live runner does **not** tell the model the answer — it gives the
catalog and asks the model to choose. A corpus that names the tool measures
obedience, not capability.

### 4.2 The offline result — 13/13, free, deterministic

```
Read-File            -> read_file          ok
GrepSearch           -> grep_search        ok
RunInTerminal        -> run_in_terminal    ok
WriteFile            -> write_file         ok
ListDir              -> list_dir           ok
grep                 -> grep_search        ok
read-file            -> read_file          ok
READFILE             -> read_file          ok
Run-In-Terminal      -> run_in_terminal    ok
list_dir             -> list_dir           ok
deploy_to_production -> (not found)        ok
send_email           -> (not found)        ok
teleport             -> (not found)        ok

checked: 13   passed: 13   failed: 0
```

**The offline probe caught a wrong expectation in the corpus itself.**
`run_terminal` does **not** resolve to `run_in_terminal`: normalising gives
`runterminal` and `runinterminal`, which is not a substring either way. The
resolver fixes **separators and case, not missing words**. The corpus now
says so, and the case uses `Run-In-Terminal` instead.

### 4.3 The live result — and it is not good enough yet

`minimax-m3.1-flash-free`, 2026-10-21:

| | |
|---|---|
| entries | 13 |
| exact | 9 |
| healed | 2 |
| refused | 2 |
| **silent wrong** | **3 (23.1%)** |
| single pass | 84.6% |

**Do not read 84.6% as a good score.** Three of thirteen answers were wrong
in a way the harness could not detect:

| Request | The model reached for | Why it is silent-wrong |
|---|---|---|
| `send_email` | `send_feedback` | **`send_feedback` exists.** The resolver resolved it as a unique substring, and the turn "succeeded" while doing the wrong thing. |
| `teleport` | `run_in_terminal` | same shape — a real tool, the wrong one |
| one entry | *(no tool call)* | the model emitted nothing parseable; refused |

**This is the failure mode the corpus exists to catch, and it is not fixed.**
The resolver's substring fallback is too eager when the request is for a
tool that does not exist. The fix belongs upstream in
`xai-grok-tools/src/tolerant.rs` — the substring fallback needs a stronger
uniqueness test, or a "the model asked for something outside the catalog"
signal that refuses rather than guesses.

### 4.4 The tool-routing number, measured

`dx-harness tool-route "run the test suite and check for failures"` over a
50-tool catalog:

```
Routed 15/50 tools: 5000 -> 1500 schema tokens (70.0% saved).
```

**70% of the schema tokens dropped.** The crate's own doc header claims
"50-90% on schema tokens" — 70% is inside that band, measured on a
synthetic catalog.

---

## 5. Verification status — the honest ledger

| Claim | Tier | Evidence |
|---|---|---|
| The harness workspace resolves | **executed** | `cargo metadata --no-deps` → EXIT 0, 125 members |
| **The CLI builds and runs** | **executed** | `cargo build -p dx-harness-cli --release -j 2` → EXIT 0; `dx-harness.exe` runs |
| **A rebuild takes 7.5 s** | **executed** | second build after the cold one |
| **1,255 sccache cache hits** | **executed** | `sccache --show-stats`, 2026-10-21 |
| **`minimax-m3.1-flash-free` answers, direct** | **executed** | HTTP 200 `HARNESS_OK` on the go gateway |
| **The Zen lane refuses direct calls** | **executed** | HTTP 403 `FreeTierError`, with `Bearer public` **and** with a valid `oc_sk_` key |
| **The resolver heals near-misses** | **executed** | 9/9 in `dx-harness resolver`; 13/13 in `corpus-offline` |
| **The corpus measured the live model** | **executed** | 13 entries, `minimax-m3.1-flash-free`, 3 silent-wrongs |
| **Tool routing saves 70% of schema tokens** | **executed** | 5000 → 1500 tokens over a 50-tool catalog |
| The 5 DX features are in bot's copy, byte-identical | **repo-managed** | SHA-256 on every file |
| 35 of 2,590 shared `.rs` files differ, none §0 work | **repo-managed** | file-by-file hash diff |
| 46 of 47 token crates absent from bot | **repo-managed** | directory listing |
| 6 token crates have **0 tests** | **repo-managed** | `#[test]` count per file |
| No golden corpus exists in tui or bot | **repo-managed** | it exists only in `G:\Dx\harness` |
| `bot/crates` has never been compiled | **repo-managed** | no `target/` dir exists |
| **The full engine links** | **not yet claimed** | `mimalloc`'s C build fails on this machine's MSVC env |
| The §0a silent-wrong rate is acceptable | **not yet claimed** | **3/13 — it is not** |
| WASM for the website / VS Code extension | **not yet claimed** | §9 |

---

## 6. Disk — the constraint that shaped everything

`G:` was at **7.3 GB free** at the worst point on 2026-10-21, then recovered
to **78.2 GB** when other work finished. The lesson from 2026-10-20 stands:
a cold build of this closure is tens of gigabytes, and the volume was full.

| Event | Measurement |
|---|---|
| `G:\Dx\tui\target` before `cargo clean` | **27.78 GB** |
| `cargo clean` result | `Removed 45927 files, 27.8GiB total` |
| `G:` free after that clean | 26.35 GB (94.5% used) |
| `G:` free during the harness build (worst) | **7.3 GB** — `forkness` +4.24 GB and `hackin` +2.51 GB grew *while the build ran* |
| `G:` free now | **78.2 GB** |
| `G:\Dx\harness\target` | **deleted externally** during the build; the source tree is intact at 0.07 GB |

**Standing rule:** never delete `target/` to make room — report it
(`G:\Dx\AGENTS.md` §2.2). And check free space before any heavy build.

---

## 7. The blocker — fixed, and what it taught

The harness CLI did not compile. One error, two sites:

```
error[E0599]: no variant, associated function, or constant named `Max`
              found for enum `async_openai::types::responses::ReasoningEffort`
  --> crates/codegen/xai-grok-sampling-types/src/types.rs:783:54
  --> crates/codegen/xai-grok-sampling-types/src/types.rs:796:41
```

**Root cause — a dropped `[patch.crates-io]`.** `G:\Dx\tui\Cargo.toml:3-4` has:

```toml
[patch.crates-io]
async-openai = { git = "https://github.com/our-forks/async-openai.git",
                 rev = "95b52ebdedf42143083cf3d6f0e0be7c84e9c808" }
```

The fork has `ReasoningEffort::Max`. The registry's `async-openai` 0.33.1
does not. `scripts/gen_workspace.py` copied `[workspace.package]` and
`[workspace.lints.clippy]` but **not** `[patch.crates-io]`, so cargo
resolved the registry version.

**The first fix was to carry the patch across.** That was also wrong — it
treated the symptom. The right fix is bot's: **map `Max` onto `Xhigh` in
the code** and drop the dependency on a git fork entirely. Bot had already
done that.

| Tree | `async-openai` source | Has the patch? | Needs it? |
|---|---|---|---|
| `G:\Dx\tui` | the git fork (`our-forks/async-openai` @ `95b52ebd`) | yes | **yes** |
| `G:\Dx\bot\crates` | registry 0.33.1 | no | **no** — its code maps `Max` onto `Xhigh` |
| `G:\Dx\harness` (now) | registry 0.33.1 | no | **no** — it inherits bot's fix |

The generator now copies `[patch.crates-io]` **only if the source has one**,
and the harness's manifest has none. **The real lesson:** tui's approach
makes a fresh clone need a git fork it cannot resolve; bot's does not. The
harness should be on bot's side of that line.

> Note `bot/TODO.md` §2 records a fix for a *different* skew of the same crate
> ("`ReasoningEffort::Max` vs `async-openai 0.33.1`"). That entry is about the
> **registry** version; bot's own code change is the actual answer.

### The second blocker — `mimalloc` and a broken C toolchain

`xai-grok-shell` → `xai-codebase-graph` → `mimalloc` → `libmimalloc-sys`, a
**C build**. It failed:

```
c1xx: fatal error C1083: Cannot open source file: '/WX-'
```

The cause is the environment, not the code: `INCLUDE` and `LIB` point at
**zig**'s libc (`G:\Dev\Tools\Scoop\apps\zig\0.16.0\lib\...`) while the
compiler is MSVC's `cl.exe`. The two disagree about what `/WX-` means.

**The fix, for now:** the full engine is behind an off-by-default `engine`
feature. The default build links the token crates and `xai-grok-tools` — no C
compiler needed. **The proper fix is a clean MSVC environment** (correct
`INCLUDE`/`LIB`, or a `cc` that speaks to the right toolchain), and that is
still open.

---

## 8. Known bugs to fix on the way

| # | Where | Problem |
|---|---|---|
| 1 | `harness/scripts/gen_workspace.py` | **FIXED.** It dropped `[patch.crates-io]` → the `ReasoningEffort::Max` blocker. It now copies the patch **only if the source has one**, and takes the source's version pins. Bot's source has no patch and does not need one. |
| 1b | -- | *(was "bot/crates has the same bug". **That was wrong.** Bot has no patch because bot's code maps `Max` onto `Xhigh` itself. Bot was never broken - the harness was, because it was copied from tui.)* |
| 2 | `bot/src-tauri/Cargo.toml:274` | `xai-grok-markdown-core = { path = "../../tui/crates/codegen/xai-grok-markdown-core" }` — the **only** dep that escapes bot's vendored copy. Everything else points at `../crates/codegen/…`, and `xai-grok-markdown-core` **is** vendored in bot. Bot's build silently depends on tui's tree. |
| 3 | `bot/src-tauri/Cargo.toml:324-326` | `[patch.crates-io]` → `libsodium-sys` / `libxdo-sys` at `../../../Rustdesk/libs/…`. Cargo reads `[patch]` **regardless of features**, so any build needs `G:\Rustdesk` present. |
| 4 | `bot/src-tauri/Cargo.toml:113` | `dx-harness` is not in `default`. Nothing constructs `DxHarness`. `bot/HARNESS.md` §0 says so outright. |
| 5 | `bot/HARNESS.md` §0 | The duplicate `zeroclaw-*` crates under `crates/common/agent/` are labelled 1.0.0 but are an **older, trimmed** snapshot of the 0.8.5 copies. They coexist only because `dx-harness` is off; enabling it links two incompatible `zeroclaw-config` types. |
| 6 | `bot/TODO.md:86-89` | *"Use **Bash**, not PowerShell — PowerShell's ConPTY sandbox is broken on this machine."* True on 2026-10-11; **not true on 2026-10-21** — `pwsh` 7.6.2 runs fine and `just --list` resolves. Needs re-measuring before it is trusted as a constraint. |
| 7 | the machine's environment | `INCLUDE` and `LIB` point at **zig**'s libc while the compiler is MSVC's `cl.exe`. Any C build in the graph breaks with `c1xx: fatal error C1083: Cannot open source file: '/WX-'`. This is what blocks `mimalloc`, and therefore the full engine. |
| 8 | `xai-grok-tools/src/tolerant.rs` | The substring fallback resolves `send_feedback` when the model asked for `send_email`. **3 of 13 corpus entries were silent-wrong because of this.** The fix belongs upstream. |
| 9 | `tui` | tui lacks bot's 9 Windows-portability fixes and bot's `ReasoningEffort::Max` fix. **Tui is the weaker tree for this machine.** Those fixes should be ported back into tui, or tui should be re-synced from bot. |

---

## 9. Where the harness goes afterwards

This is the argument for doing it as a **library with a CLI**, not an app.

| Target | How the harness reaches it | Constraint |
|---|---|---|
| `G:\Dx\bot` (Tauri GUI) | in-process, over `xai_acp_lib::acp_channels()` — the seam `bot/src-tauri/src/dx_harness.rs` already implements | `!Send` types cannot cross `tokio::spawn`; needs a `LocalSet` |
| `G:\Dx\tui` (TUI) | in-process, ACP client — the TUI is already a pure ACP client | the TUI keeps its own pager crates; only the engine is shared |
| `G:\Dx\code` (code editor) | Rust (`crates/acp_thread`, `crates/acp_tools`, `crates/agent`, …) — links natively | same `!Send` caveat |
| `G:\Dx\extensions` (VS Code) | **TypeScript** (5,150 `.ts` files, `@cline/packages`) — needs a **WASM** build | see below |
| Website | WASM, same as extensions | see below |
| Android / iOS | the sandbox crate is already platform-aware; mobile needs the JNI/ObjC seam bot's grammar work already solved | |

**The honest constraint:** WASM is **not** a copy problem, it is a
*dependency* problem. `git2`, `reqwest`+TLS, `tokio`+threads and `rusqlite`
all need replacing or making optional. That is a large piece of work with its
own decision — it should **not** be smuggled into this one. Realistic order:
native Rust targets first (bot, tui, code), then WASM as a separate track.

---

## 10. Decisions taken, and the ones still open

### Taken

| Decision | Why |
|---|---|
| `G:\Dx\harness` is its **own git repo**, not a folder in the hub | the hub is deny-by-default (`.gitignore` line 4 is `/*`), and every sibling product is its own repo |
| **The engine comes from bot, not tui** | bot's copy has the `ReasoningEffort::Max` fix in code (no git fork needed) and 9 Windows-portability fixes. The trees diverged both ways; bot's extra fixes are the ones this machine needs. §2.3 |
| **bot keeps its vendored copy** | a path dependency on `G:\Dx\harness` makes bot unbuildable without the whole hub — the "fresh clone cannot be built" problem `bot/HARNESS.md` §0 already complains about |
| **Six token crates, not 47** | 46 are modality compressors (video/pdf/ocr/audio) not in scope; they total 0.9 MB but most have zero tests, so copying them copies untested scaffolding |
| **The harness excludes every TUI/GUI crate** | the `/model` picker and the tab strip are TUI features, not engine features |

### Still open

1. **Does the harness get its own `origin` (`essence-dx/harness`)?** The repo
   has **3 commits and no remote**. It is not durable yet.
2. **Should the harness be published to crates.io** so bot and tui can depend
   on a version instead of a path? That is the clean end-state, and it is
   what makes a fresh clone work.
3. **Which provider is "my OpenCode key"?** The `oc_sk_` key works on the
   go gateway and reaches `minimax-m3.1-flash-free` and the other free
   models. The `gmi` provider (MiniMax-M3, Qwen3.8-27B) still has **no
   visible credential**.
4. **Should bot's 9 Windows-portability fixes and its `Max` fix be ported
   back into tui?** Tui is the weaker tree for this machine. Either tui is
   re-synced from bot, or the fixes are ported upstream.
5. **Should the silent-wrong fix go upstream?** The resolver's substring
   fallback is in `xai-grok-tools/src/tolerant.rs`. The source is now bot,
   so the fix lands in bot first.

---

## 11. What to do next, in order

1. **Fix the silent-wrongs.** 3 of 13 corpus entries resolved to a *real but
   wrong* tool — `send_feedback` for `send_email`, `run_in_terminal` for
   `teleport`. The fix is upstream in `xai-grok-tools/src/tolerant.rs`: the
   substring fallback needs a stronger uniqueness test, or a "the model
   asked for something outside the catalog" signal that refuses rather than
   guesses. **This is the §0a work, and it is not done.**
2. **Fix `bot/crates/Cargo.toml`** — add the same `[patch.crates-io]` the
   harness now carries. It is the identical bug, and bot cannot compile
   without it.
3. **Build `bot/crates`** once, properly, and record the receipt. It has
   never been compiled.
4. **Fix the C toolchain** — `INCLUDE`/`LIB` point at zig while the compiler
   is MSVC. Until that is sorted, the full engine cannot link, and
   `--features engine` stays off.
5. **Write real tests** for `tool-router`, `prefix-cache`, `token-budget` and
   `core` — the four with zero.
6. **Commit and push the harness repo.** It has **2 commits and no remote**.
7. **Copy the finished work back** into `tui`, then into `bot`.

### The fastest loop, right now

```bash
# free, no model, no network
cd G:/Dx/harness && ./target/release/dx-harness.exe corpus-offline corpus/golden.json
cd G:/Dx/bot    && bash scripts/harness-test.sh resolver

# one real model call
cd G:/Dx/harness && ./target/release/dx-harness.exe model "Reply with exactly: HARNESS_OK"

# no build at all
cd G:/Dx/harness && bash scripts/model.sh "Reply with exactly: HARNESS_OK"
```

Rebuilding after an edit: **7.5 s**.

---

MIT OR Apache-2.0
updated: 2026-10-21 00:00 UTC
