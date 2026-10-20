# AGENTS.md — Operating Manual for the `G:\Dx` Hub

**Read this before you touch anything in this workspace.** `G:\Dx` is not a normal
repository, and the instincts every coding agent carries in will damage it. This file
is the contract. When your habits and this file disagree, this file wins.

**Audience:** every AI agent (and human) working at `G:\Dx` — the hub.
**Applies to:** the hub repository only. Product folders own their own code and their
own instructions; when you work inside one, that folder's docs govern that folder.

---

## 1. What this workspace actually is

`G:\Dx` is a **hub**, not a monorepo. It holds the operating memory, the routing
manifest, and the public story for a family of ~20 separate products.

| Property | Reality | Evidence |
| --- | --- | --- |
| Repo shape | Hub with a deny-everything ignore file | `.gitignore` line 4 is `/*`; only an allow-list lets a file exist in the index |
| Tracked files (before 2026-09-26) | 5 — `.gitignore`, `PROMPTS.md`, `README.md`, `build.ps1`, `dx` | `git ls-files` |
| Child products | 19 independent git repos with their own `origin` | `git -C <folder> remote get-url origin` → `https://github.com/essence-dx/<name>.git` |
| Routing | A TOML-shaped workspace manifest maps short names to folders | `dx` at repo root (`workspace`/`paths`/`tools` blocks) |
| Install contract | Which binary comes from which repo | `TOOL_REGISTRY` in `cli/src/update_registry.rs` (see `DX.md:16`) |
| Achievement memory | One page per folder, validation-based | `docs/README.md`, `docs/<tool>.md` |
| Last hub commit | `fb2f08f`, 2026-09-09 | `git log -1` |

**Folders that are *not* repos and must not be treated as source:**
`bin/`, `docs/`, `native/`, `test/`, `zzz/`, `.workbuddy-ai/`,
`DEPRICATED_10-20-2026/` (44 entries on 2026-10-20: 31 git repos and 13 plain
folders, including `llama.cpp`, `superpowers`, `ZZZ-bot-trash`, and a
`root-junk/` folder of 10 loose root files parked on 2026-10-20 — see
`docs/space-review-2026-09-23.md:57` for the original 22-repo measurement).

---

## 2. The six hard rules

1. **Never absorb a child repo into the hub.** `git add cli/`, `git add docs/../code`,
   or `git add -f` on a product folder is a corruption event. Nested `.git` directories
   would be silently swallowed into one hub tree and the product would lose its history.
   The `/*` rule in `.gitignore` is your safety net — do not defeat it.
2. **Never delete anything to make a problem go away.** Not with `Remove-Item`, not with
   `rmdir`, not by moving it into `trash/` either. Space is the recurring temptation here:
   `G:` sits at **99% full** and ~88 GB of `G:\Dx` is regenerable build output
   (`docs/space-review-2026-09-23.md:5`). The house standard is that review's own opening
   line — *"Nothing has been deleted. Every item below is a proposal with the evidence
   that supports it."* Write the proposal, name the size and the evidence, and wait.
   **Exception:** an explicit user approval after that written proposal makes deletion
   permitted; record the exact approved target list and exclusions before acting.
3. **New root file → new allow-list line, same commit.** Because of `/*`, a file you
   create at the root is invisible to `git add` until `.gitignore` names it. `AGENTS.md`,
   `TODO.md`, `CHANGELOG.md`, `CURRENT_STATUS.md`, `PLAN.md` are already allow-listed;
   anything else you add at root gets its `!/name` line in the same commit, or it does not
   exist. Verify with `git check-ignore -v <path>`.
4. **Fix things in the repo that owns the file.** A defect inside `cli/`, `tui/`,
   `daemon/` or any other product is committed there, on that repo's branch, and pushed
   there. Hub commits carry hub artifacts only: this file, `README.md`, `TODO.md`,
   `CHANGELOG.md`, `docs/**`, `dx`, `build.ps1`, `PROMPTS.md`.
5. **A commit is not durable until it is pushed.** The hub has been left unpushed before,
   and a ledger entry claimed work that no commit contained
   (`OPEN-WORK-2026-09-19.md:95`). After committing hub work, run `git status -sb` and
   resolve any ahead marker.
6. **Do not trust the `dx` manifest blindly.** At least 10 declared paths are dangling
   today (§7). Confirm with `Test-Path` before acting on any path you read from it.
7. **The Bonsai/TPU track is TPU-only. A GPU run is not a result of this project.**
   This project exists to run Bonsai 2 27B -- and other good models -- **on TPU
   silicon, at maximum speed**. A number produced on a GPU (Kaggle T4, any CUDA or
   ROCm host) answers a different question, does not move the TPU kernel, and may
   not be quoted as this project's throughput, filed in `TODO.md` as track work, or
   used to decide what the TPU work does next. **An agent may not start a GPU run
   on this track without the user's explicit go-ahead.** A GPU is otherwise allowed
   only as a *correctness oracle* (llama.cpp's text as a cross-check), labelled as
   such and never as a speed claim.
   The reason is measured, not ideological: llama.cpp has no TPU backend, so a GPU
   run cannot inform the TPU kernel, while the TPU's own bottleneck is the
   per-weight-byte ternary unpack (`tpu/DO-NOT-RETEST.md` section 10) -- a GPU
   number does not move it.

---

## 3. Source-of-truth map

| Question | Canonical answer lives in |
| --- | --- |
| Where does tool X live on disk? | `dx` manifest, `paths` block |
| Which repo publishes binary X, under what filename? | `TOOL_REGISTRY` in `cli/src/update_registry.rs` |
| What has folder X actually achieved? | `docs/<x>.md`, then `docs/README.md` register row |
| What is open / owed / blocked? | `TODO.md` (hub), the folder's own ledger (products) |
| What is this model's throughput? | `tpu/**` receipts -- **TPU only**; a GPU number is not a result of this project (AGENTS.md section 2, rule 7) |
| What changed most recently? | `CHANGELOG.md` (hub), `git log` (products) |
| Public story and status | `README.md` |
| Release-asset naming | `dx-<tool>-<rust-target-triple>[.exe]` over 6 targets — proven by `essence-dx/media` v1.0.0 (`DX.md:38`) |
| Install layout on Windows | `%LOCALAPPDATA%\dx\bin`, one `dx-<tool>` file per tool (`DX.md:43`) |


---

## 4. Claim tiers — the vocabulary that keeps this repo honest

`docs/README.md:68` already sets the documentation rules. This extends them into a
vocabulary every agent must use when writing status anywhere in the hub.

| Tier | You may say this when | Required qualifier |
| --- | --- | --- |
| **repo-managed** | The code/manifest/script is committed and readable | name the path |
| **executed** | You personally ran it and saw the result | name the command + the log/receipt file |
| **runtime / hosted** | True on a host, not in this tree (Kaggle, GitHub, a store) | name the host and the check |
| **release-qualified** | Shipping is verified end to end on a real device | a receipt that owns the number |

**Never write a bare superlative.** No "fastest", "blazing", "instant", "COMPLETE",
"production-ready" without the row above that earns it. `docs/README.md:73` is explicit:
do not repeat a benchmark number without naming the report or receipt that owns it, and
separate repository-managed status from runtime, hosted, or release status.

Where a claim is known to be shaky, keep the doubt visible. The register already does
this — "product-wide speed and default-on wording remain blocked" (`docs/README.md:135`).
Copy that reflex. Deleting uncertainty wording to make a page read better is a defect.

---

## 5. The status-file protocol

Each hub file has exactly one job. Do not blur them.

| File | Job | Never put here |
| --- | --- | --- |
| `README.md` | The public story + honest status, for a stranger | running commentary, session logs, open tasks |
| `AGENTS.md` | How to work safely here (this file) | open work items, history |
| `TODO.md` | What is open, with evidence + done-when | finished work, narrative, marketing |
| `CHANGELOG.md` | What changed, dated, per release | speculation, TODOs, unshipped intent |
| `docs/<tool>.md` | One folder's verified achievements | benchmark numbers with no owner |

Completion discipline: when an item in `TODO.md` closes, **delete it from `TODO.md` and
add a `CHANGELOG.md` entry in the same commit.** Two records of one event is how ledgers
rot. The hub has already been caught with a ledger entry, no commit, and no changelog line
all at once (`OPEN-WORK-2026-09-19.md:95`).

`TODO.md` rules of engagement:

- Every item gets a stable ID (`HUB-`, `MF-`, `DIST-`, `DOC-`, `ENV-`) and is referenced
  by ID in commits and changelog lines.
- Every item carries **Evidence** (a file:line proving it is open) and **Done when** (an
  observable check another agent can run). No evidence means it is a wish, not a task; do
  not file it.
- Items blocked by something not present on this machine stay in the file, marked
  **BLOCKED**, with the blocker named — they never silently disappear.

---

## 6. Editing conventions

- **Markdown**, ATX headings, no heading-level jumps, one `#` per file, tables over prose
  for anything enumerable, `backticked` paths and identifiers, fenced blocks tagged
  (`powershell`, `text`, `json`, `toml`).
- **PowerShell is the default shell** in examples — this is Windows (`win32`) and every
  operator here works from a terminal. Always pass `-LiteralPath`, never `-Path`, for hub
  root files. This is not a style rule: a 663-byte junk file with an unreachable name
  (two private-use-area codepoints, U+F02A, so no normally quoted path can open it) sat
  at the root until it was removed on or before 2026-10-20 — the root listing as of
  2026-10-20 no longer contains it, but assume any root name you did not list from a
  byte-level source can fight you.
- Wrap prose near 88 columns to match `docs/` and `DX.md`. Keep hub docs ASCII-safe.
- **Footer every hub doc** exactly as the house does it:

  ```text
  MIT OR Apache-2.0
  updated: 2026-09-26 00:00 UTC
  ```

- **License is always** `MIT OR Apache-2.0` (dual, never one alone).
- **Commit messages** follow the Conventional shape already in use: `docs:` `feat:`
  `fix:` `refactor:` `chore:` + imperative summary. `git log` also shows legacy noise —
  `feat: automated commit`, `warnings!!!`, `feat: thanks`. Do not imitate it.

---

## 7. Known-bad references — do not act on these without checking

The `dx` manifest declares paths that no longer resolve (verified 2026-09-26 with
`Test-Path`):

| Declared | Value | On disk | Consequence |
| --- | --- | --- | --- |
| `build` | `build` | **missing** | any `dx build` routing is dead |
| `py` | `py` | **missing** | Python workstream unreachable |
| `py_package_manager` | `py/package-manager` | **missing** | as above |
| `dx_agents` | `agent` | **missing** | agent project dangling — real tree is `DEPRICATED_10-20-2026/agent` (`docs/space-review-2026-09-23.md:48`) |
| `inspirations` | `inspirations` | **missing** | dead reference in every doc that cites it |
| `cache` | `.dx/cache` | **missing** | cache routing unresolvable |
| `cargo_home` | `cli/.cargo-home` | **missing** | swept into `cli/trash/.cargo-home` (`docs/space-review-2026-09-23.md:37`) |

Also note the parking lot was renamed from the misspelled `DEPRICATED_9-17-2026`
to `DEPRICATED_10-20-2026` on 2026-10-20, and that it contains two siblings
still spelled differently (`dx-video-player-depricated-20260820-1856`,
`os-depricated-20260820-1856`) — three spellings of the same word. Do not
"fix" either one in place: renaming a folder that holds 30 pushed repos and
13 plain folders is a proposal, not an edit (§2.2).

On-disk folders the manifest does **not** declare: `daemon`, `bot`, `token`, `tree`,
`typesafe`, `native`, `os`, `mobile`, `video-player`, `extensions`, `test`, `zzz`, `bin`.
`daemon` is the flagship (`README.md:95`) and appears nowhere in `paths` — treat manifest
silence as *undeclared*, never as *nonexistent*. `grimoire` and `forkness` **are**
declared in `paths` as of 2026-10-20.

---

## 8. Environment facts worth memorizing

| Fact | Value |
| --- | --- |
| Platform | `win32`, PowerShell; the IDE surface is a terminal |
| Volume `G:` | ~477 GB total. **~12.7 GB free / 464.2 GB used (97% used), re-measured 2026-09-26** with `Get-PSDrive G`. The 2026-09-23 review's "~5.2 GB free (99% used)" (`docs/space-review-2026-09-23.md:5`) is now stale by ~7 GB; the review's *conclusions* — nothing deleted, every item a proposal — are unaffected and still bind (§2.2) |
| `G:\Dx` size | ~166 GB, of which ~88 GB is regenerable build/dependency output |
| Rust homes | the live ones are the junctions `~/.rustup` and `~/.cargo`; `G:\Dx\.rustup` and `G:\Dx\.cargo` are **dead duplicates** (`docs/space-review-2026-09-23.md:23`) |
| Network | `essence-dx/*` is private; anonymous API calls 404, and full `cargo` builds can 401 on the private `check` dependency (`DX.md:277`) |
| Cost discipline | free-first: only Apple/Windows signing and store listings cost money (`DX.md:304`); the $0 unlock that clears 11 distribution channels is making releases public (`DX.md:313`) |

**Disk safety:** with ~12 GB free, a large `cargo build` can still fail on space rather than on
code (`bot/src-tauri/target` alone measured 7.13 GB on 2026-09-26). Check free space before
heavy builds, and never "free up room" by deleting `target/`, caches, or toolchains — report
instead (§2.2).

### 8.1 Sibling folders that are build inputs to another folder

Some folders here are not just neighbours: one repo's `Cargo.toml` names another by **path**.
Moving, parking, or renaming a folder in this table breaks a build in a *different* repo, and
the failure appears far from the change. This binds every space-review proposal (§2.2) —
"regenerable" and "unreferenced" are different claims, and a path dependency makes a folder
neither.

| Depends (repo) | On (folder, by path) | Gated by | Measured |
| --- | --- | --- | --- |
| `bot/src-tauri` | `../../metasearch/crates/*`, `../../daemon/crates/*` | `embedded-engines` | `bot/src-tauri/Cargo.toml:140-144` |
| `bot/src-tauri` | `../../tui/crates/codegen/xai-grok-*` | `tui-models`, `tui-md`, `tui-env`, `tui-secrets`, `tui-version` | `bot/src-tauri/Cargo.toml:162-180` |
| `bot/src-tauri` | **`../../forge`** (`dx-forge` 1.0.0) | `forge-media` (added 2026-09-26, off by default) | `cargo tree --features forge-media` |
| `forge` | `../serializer`, `../serializer/crates/dx-config` | **unconditional** — `forge` cannot build without `serializer` | `forge/Cargo.toml:13-14` |
| `bot/src-tauri` | `../../../Rustdesk/libs/*` — **outside the hub entirely**, and its `[patch.crates-io]` block is read by Cargo even for a default build | `remote-rustdesk` for the dep; the `[patch]` paths are **not** gated | `bot/src-tauri/Cargo.toml:192` and `:200-202` |

Two consequences worth stating plainly:

1. **`forge/` and `serializer/` are now load-bearing for `bot`.** `dx-forge` reaches `serializer`
   by path itself, so enabling `forge-media` requires both folders to sit beside `bot` exactly
   as `metasearch`, `daemon` and `tui` must.
2. **Feature-gated is not the same as harmless.** An optional path dep is absent from the
   default `cargo tree` — verified for `forge-media` — so it costs a default build nothing. A
   `[patch.crates-io]` entry is different: Cargo resolves those regardless of features, so an
   ungated patch pointing outside the hub is a default-build dependency on a folder that is not
   in this repo at all.


---

## 9. Before you claim anything is done

1. `git status -sb` — is the tree what you think it is, and is the hub ahead of origin?
2. `git check-ignore -v <new file>` — is the new hub file actually visible to git?
3. Did you touch a child repo? That work belongs in a commit **there**, and this hub
   commit must not contain it.
4. Every number you wrote has a `file:line` you can point at.
5. A `TODO.md` item closed means: removed there **and** a `CHANGELOG.md` line, same commit.
6. The footer timestamp on every hub doc you edited is updated.
7. Re-read your status wording and ask: *would I say this out loud to someone who just
   cloned the repo and hit an error?* If not, downgrade the claim to its tier (§4).

---

## 10. Onboarding — first five minutes

```powershell
cd G:\Dx
git status -sb                      # hub state; expect untracked docs/
git ls-files                        # the hub's entire tracked surface
Get-Content dx                      # routing manifest - read §7 before trusting it
Get-Content docs\README.md          # the achievement register and its rules
git -C daemon status -sb            # a child repo is where product work happens
```

Then pick work by ID from `TODO.md`, in priority order. If your task has no ID, file it
there with evidence first, then do it.

---

## 11. What this file deliberately does not do

It does not certify any product as release-ready, and it does not assert a completion
percentage for the ~20 child repositories. This machine has not run their suites, and the
hub does not own their CI. `README.md` therefore reports a **measured** documentation and
governance score for the hub, plus the child-repo numbers that already carry a receipt.
Anything broader would be exactly the bare superlative §4 forbids.

---

MIT OR Apache-2.0
updated: 2026-10-20 00:00 UTC
