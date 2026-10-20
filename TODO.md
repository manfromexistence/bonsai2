# TODO.md — Open Work for the `G:\Dx` Hub

**Snapshot taken 2026-09-26 against the tree, not restated from memory.** Every item
below carries the evidence that proves it is open and a `Done when` check another agent can
run without asking anyone.

**Scope:** hub-level work, plus the execution track this repository also owns
(`tpu/**` is tracked here — see *The execution track* below). Other product work lives
in each product repo's own ledger — see §6 for what the hub knows about those and what
it deliberately does not claim.

---

## How to use this file

- Take items in priority order: **P0 → P1 → P2**. `BLOCKED` items are not skipped, they
  are parked with the blocker named and re-checked.
- Cite the ID (`HUB-01`, `MF-01`, …) in the commit subject and in the `CHANGELOG.md` line.
- On completion: **remove the item here and add a `CHANGELOG.md` entry in the same commit**
  (`AGENTS.md` §5). Never leave both records.
- Do not file an item without evidence. Do not resolve an item without running its check.
- Nothing here authorises a delete. Where an item's remedy is removal, it stays a proposal
  until approved (`AGENTS.md` §2.2).

---

## Scoreboard

| Priority | Open | What it guards |
| --- | --- | --- |
| **P0** | 4 | durability — work that exists only on this machine, and routing that is dead |
| **P1** | 12 | correctness of the hub's own claims: dangling citations, broken rules, blocked channels |
| **P2** | 9 | hygiene and long-tail distribution |
| **BLOCKED** | 4 | held by hardware/software not present on this machine |
| **TPU track** | 6 | 50 tok/s for one stream on an 8k window, and the context ceiling that bounds it |
| | **35** | total open |

**Pass of 2026-10-20 (third): the total moves 38 → 35 and no score moves.** Three items
left the ledger because the **route** was withdrawn, not because the work was done:
`GPU-01`, `GPU-03` and `GPU-04` were GPU-route items, and the track is now TPU-only
(`AGENTS.md` section 2, rule 7). The decision and what it cost are in `CHANGELOG.md`; the
facts those items produced stay in `tpu/DO-NOT-RETEST.md` (section 14: the GGUFs carry no
MTP head, the base model does). The 50 tok/s target is re-scoped to this TPU, and its
route is `TPU-03` (multi-token verification) plus `TPU-07` (the ternary unpack, ~9x off the
HBM roofline).

**Pass of 2026-10-20 (second): the total moves 38 → 37 and no score moves.** Two items
left the ledger, both because the work was done, not because the ledger was tidied:
`GPU-02` is **closed** — the MTP-head question is now answered from the file's own
header, which no further session can improve on — and `TPU-01` is **withdrawn** — its
done-when was "≤ 100 ms/step at 256 launches" and the measurement says 181.3 ms, so the
launch-overhead theory it rested on is refuted and the correction is in
`CHANGELOG.md` rather than quietly dropped. `GPU-04` was filed in their place: the
graft that replaces the MTP route. The earlier pass added the nine items; this one
pruned two of them and added one.

**Hub documentation-governance: 19 % → 78 %** (19.45 → 77.6 exact). Derived, not felt — the
rubric and the arithmetic are in the **Score rubric** section at the end of this file, and
`README.md` publishes the same number.

> **Arithmetic correction, 2026-09-26.** This ledger published **22 % → 74 %**, and both figures
> were wrong at the sum, not at the scoring. `85·20 + 80·18 + 85·18 + 80·12 + 70·15 + 65·12 +
> 60·5` is **7760**, not the 7400 the file asserted (**77.6 %**, not 74); the earlier reading's
> products are **1945**, not the 2235 asserted (**19.45 %**, not 22). No weight and no
> dimension score was touched — the seven inputs are exactly as scored when the rubric was
> written. **Both errors ran in the same direction, understating the hub**, which is the
> opposite of the failure mode `AGENTS.md` §4 worries about; that is worth naming because it
> means the incentive here is honesty about the arithmetic, not a rescue. The lesson is the
> one `HUB-04` already files: a hand-added number in a doc is an unverified claim, and the fix
> is a script, not a re-read.

**Pass of 2026-09-26 (late): the total stays 29.** Four records were repaired — `CHANGELOG.md`
created (it had been cited by `README.md`, `AGENTS.md` §5 and this file while absent from
disk); `DOC-01`'s `Done when` rejoined after its second half was found orphaned below this
file's license footer; `AGENTS.md`'s commit-message bullet rejoined after the **same** defect;
and `README.md`'s duplicated `## Quick start` heading removed — and `AGENTS.md` §8's
free-space figure was re-measured (5.2 GB → 12.73 GB). **None of these closes an item**:
`docs/daemon.md` still does not exist, so `DOC-01` stands, and the change-history dimension
was already being paid 80 against a missing file — writing that file makes the published score
honest rather than raising it. **The 74 → 78 movement above is entirely the corrected sum and
entirely not a reward for this pass.**

---

## P0 — durability and dead routing

### HUB-01 · Commit and push the untracked hub corpus

The hub's entire knowledge base exists only on this machine.

- **Evidence (re-measured 2026-10-20):** `git ls-files` returns **137** files, where
  it returned 5 at the 2026-09-26 snapshot — the 22-folder `tpu/` execution track and
  the two ledgers are tracked now. The hub's own corpus is not:
  `git ls-files --others --exclude-standard` returns **33** — the 23 files under `docs/`
  (~180 KB: the register, 21 per-tool pages and the space review), `DX.md` (19.8 KB),
  eight other root files (`AGENTS.md`, `CAP.md`, `COLAB.md`, `DEPRECATED_README.md`,
  `DX_TOOLS_AI_GAP.md`, `JULES_BUILD_PROMPT.md`, `MINICPM5-200MB-PLAN.md`, `PLAN.md`)
  and one scratch file (`tpu/14-bonsai-transposed/kernel.py.bak`).
  `git ls-files docs` returns 0 and `git ls-files DX.md` returns nothing, so the
  `Done when` is not met and this item stays open.
- **Why it matters:** `docs/README.md:83` calls the register "the internal canonical map"
  and the stated purpose of `docs/` is that DX "has operating memory outside chat"
  (`docs/README.md:143`). Operating memory that is not committed is chat with extra steps.
  `AGENTS.md` §2.5 records that this exact failure mode already bit once
  (`OPEN-WORK-2026-09-19.md:95`).
- **Done when:** `git ls-files docs | Measure-Object -Line` reports 23,
  `git ls-files DX.md` returns a line, `git status --porcelain` is empty, and
  `git status -sb` shows no ahead marker after `git push`.

### HUB-02 · Land the three dirty hub files

- **Evidence (re-measured 2026-10-20):** `git status --porcelain` shows **three**
  modified files — `.gitignore` (+1), `README.md` (+32 −5) and `dx` (+3 −1) — and they
  are not the three this item names. The diffs: `.gitignore` gains `!/CAP.md`;
  `README.md` renames the `typesafe` product to `decisions` and adds a
  *"Bonsai 2 27B on a TPU (measured)"* section; `dx` points `desktop` at `code` and adds
  `metasearch`.
- **`PROMPTS.md` is not a dirty tracked file at all.** It is on disk and **ignored** by
  `.gitignore`, so it does not appear in `git status` — a different fact from the one
  recorded here, and one that needs its own decision (commit it, or keep it ignored).
- **Complication to resolve while you are in there:** the `dx` edit is on the file
  §MF-01 documents as broken, and it now also repoints `desktop` from `desktop` to
  `code`, so the declared path no longer names a folder called `desktop`. Read the diff
  before landing it; do not land a regression on a path just because it was already
  open. The README's `typesafe` → `decisions` rename must land together with the folder
  rename, or the two disagree.
- **Done when:** `git diff --stat HEAD` is empty and the tree is pushed.

### MF-01 · Repair the seven dangling paths in the `dx` manifest

The routing manifest is the file every tool and doc resolves names against, and seven of
its entries do not resolve.

- **Evidence:** `Test-Path` per declared value, run 2026-09-26 — `build` False, `py` False,
  `py/package-manager` False, `agent` False, `inspirations` False, `.dx/cache` False,
  `cli/.cargo-home` False. The `agent` entry was already flagged as dangling in
  `docs/space-review-2026-09-23.md:48`; `cli/.cargo-home` appears to have been swept into
  `cli/trash/.cargo-home` (`docs/space-review-2026-09-23.md:35`).
- **Choose one of three answers per path — do not leave a fourth state:**
  1. **restore** the path (re-clone from `essence-dx/<name>`, which is recoverable for the
     21 clean pushed repos in `DEPRICATED_10-20-2026/`),
  2. **repoint** to where the tree actually is (e.g. `dx_agents` → its live location),
  3. **delete** the declaration and record the removal in `CHANGELOG.md`.
- **Done when:** a script walks every key in the manifest's `paths` block, `Test-Path`
  returns True for each, and the check is re-runnable from `scripts/` (see `HUB-04`).

### DOC-01 · The flagship has no achievement page

`docs/` has 21 tool pages and none of them is `daemon`.

- **Evidence:** `Get-ChildItem docs` lists agent, build, check, cli, code, dcp, driven,
  extensions, flow, forge, i18n, icon, js, media, metasearch, native, providers, py,
  serializer, style, www — no `daemon.md`. Meanwhile `README.md:95` calls **daemon** "The
  memory. The flagship. **COMPLETE**", and `README.md:46` gives it a whole architecture
  section. The hub's loudest public claim is the one page the register does not back.
- **Also blocks:** the register's own rule — "Keep one page per folder" (`docs/README.md:70`).
- **Done when:** `docs/daemon.md` exists, follows the shape of `docs/cli.md`, states each
  claim at its tier (`AGENTS.md` §4), and is linked from the `docs/README.md` page index.

---

## P1 — the hub's claims do not match the hub's tree

### HUB-03 · `docs/policies/` is cited as policy and does not exist

- **Evidence:** `docs/README.md:143` lists the `docs` folder's achievement as holding
  "source-control, CI, artifact, archive, backup, restore, and maintainability policy" and
  cites `docs/policies/*.md` plus `docs/policies/maintainability-baseline.json`.
  `Test-Path docs\policies` → **False**. The allow-list already expects it
  (`.gitignore:23`).
- **Done when:** the five named policy files exist and every one is reachable from
  `docs/README.md`, or the citation is corrected to say what actually exists.

### HUB-04 · `scripts/` is cited as automation and does not exist

- **Evidence:** `docs/README.md:144` credits the hub with "read-only or manifest-first
  scripts for folder audits, maintainability checks, archive manifests, artifact inventory,
  and backup-exclusion testing" at `scripts/*.ps1`. `Test-Path scripts` → **False**, though
  `.gitignore:27` allow-lists it.
- **First script to write:** the `MF-01` manifest-path checker. It turns a hand-verified
  finding into a re-runnable check, which is what keeps this file honest next month.
- **Done when:** `scripts/` holds at least the path checker, each script is read-only or
  manifest-first, and each is documented at the top.

### HUB-05 · CI is described as deliberately paused, but nothing documents it

- **Evidence:** `docs/README.md:142` says the `.github` folder "Documents that Actions are
  paused and keeps workflow files disabled" and cites `.github/workflows/README.md` plus
  `docs/policies/ci-policy.md`. Neither path exists; `Test-Path .github` → **False**.
- **Why it matters:** a new agent with no way to learn that CI is intentionally off will
  either trust green-badge wording in a doc or spend a day re-wiring Actions.
- **Done when:** the state is written down in one place and linked from `AGENTS.md`, or the
  citation is removed.

### HUB-06 · Five root files are reserved by `.gitignore` and none exist

- **Evidence:** the allow-list reserves `!/LOG.md` (`.gitignore:6`), `!/CURRENT_STATUS.md`
  (`:15`), `!/DX_LANE_PASS_SYSTEM.md` (`:17`), `!/DX_MANAGER_MEMORY.md` (`:18`) and
  `!/PLAN.md` (`:19`). `Test-Path` returns **False** for all five.
- **`LOG.md` is answered, and the answer is a stale allow-list.** It was the hub's working
  ledger — 26 commits touch it (`git log --oneline -- LOG.md`) between 2026-07-12 and
  2026-07-18 — and it was **deliberately deleted** in `c2e2535`, "chore: remove obsolete
  gitattributes file and task logging documentation" (2026-07-21). So this is not a lost
  file; it is a reservation nobody cleaned up after a retirement. `TODO.md` and
  `CHANGELOG.md` are its successors.
- **A decision is required per name, not a default:** adopt it under the evidence discipline
  of this file, or delete its allow-list line. Reserved-but-absent is how a repo grows
  phantom files that other docs then cite — see `HUB-03`, `HUB-04`, `HUB-05`.
- **Recommendation:** do **not** recreate `CURRENT_STATUS.md` or `LOG.md`. Status now lives
  in `README.md` with a published rubric and open work lives here; a second ledger is the
  two-records-of-one-event defect `AGENTS.md` §5 forbids. `DX_MANAGER_MEMORY.md` and
  `DX_LANE_PASS_SYSTEM.md` look like real intent worth reviving as `docs/` pages instead.
- **Progress, 2026-09-26:** the `LOG.md` disposition half of the done-when is now **met** —
  it is stated in `CHANGELOG.md` under "the hub's change history is no longer a phantom file"
  (retired in `c2e2535`, succeeded by `TODO.md` + `CHANGELOG.md`, allow-list line left in
  place pending this item). The five per-name decisions are still open, so `HUB-06` stays
  open. The `CHANGELOG.md` prerequisite this item depended on no longer blocks it.
- **Done when:** each of the five is either committed with content or gone from
  `.gitignore`, and the `LOG.md` disposition is stated in `CHANGELOG.md`.

### HUB-07 · `templates/dx-starter/` is reserved and empty

- **Evidence:** `.gitignore:28` through `.gitignore:31` allow-list the folder and its
  `.dx/` contents in fine detail; `Test-Path templates` → **False**. Someone planned a
  starter kit and the plan outlived the work.
- **Done when:** the scaffold exists with a working `dx` manifest of its own, or the
  allow-list lines are removed.

### HUB-08 · The root `package.json` is a 20-byte stub

- **Evidence:** `Get-Content package.json` returns exactly `{"dependencies": {}}` — no
  `name`, no `version`, no `license`, no `private`. It is untracked (denied by `/*`) and not
  allow-listed.
- **Why it matters:** a `package.json` at a repo root makes Node tooling treat `G:\Dx` as a
  JavaScript package root, so `npm install` run from here resolves against the hub instead of
  the product folder that owns the dependency.
- **Done when:** it is either a real manifest (`name`, `version`,
  `license: "MIT OR Apache-2.0"`, `private: true`) or removed as a proposal with a reason.


### DOC-02 · One page per folder is broken in both directions

- **Evidence (measured 2026-09-26):** nine folders that exist on disk have no page —
  `tui`, `token`, `tree`, `decisions`, `bot`, `mobile`, `os`, `video-player`, and `daemon`
  (tracked separately as `DOC-01`). Five pages describe folders that are no longer at that
  path — `agent`, `build`, `js`, `providers`, `py`, all now parked under
  `DEPRICATED_10-20-2026/` per `docs/space-review-2026-09-23.md:59`.
- **One naming trap inside this item:** `docs/flow.md` has no `flow/` folder because flow's
  code sits in `bot/` — `git -C bot remote get-url origin` resolves to
  `https://github.com/essencefromexistence/flow.git`, a **different account** from the whole
  `essence-dx` family. Decide on purpose whether `bot` is a folder name or a stray clone of
  a differently-owned repo.
- **Also unindexed:** `docs/space-review-2026-09-23.md` (12.5 KB) is not in the "Current
  pages" index at `docs/README.md:44`, even though three items in this file cite it.
- **Done when:** `docs/README.md`'s index and `Get-ChildItem docs` agree with
  `Get-ChildItem -Directory` folder by folder, and the `bot`/`flow` account question is
  answered in writing.

### DOC-03 · The flagship's loudest claim has no receipt

- **Evidence:** `README.md:95` carried "**COMPLETE** — all 11 targets shipping, cloud-offloaded
  visual memory live" with no build log, test run, release list or `docs/daemon.md` behind
  it. The table row now points at this item instead of asserting it, but **the claim itself
  is still unresolved** — 11 targets is a checkable fact and nothing here checks it.
- **What a receipt looks like:** a `docs/daemon.md` page listing each target with the
  `cargo build --target <triple>` output that produced it, or a release page URL — tier
  `executed` or `release-qualified`, per `AGENTS.md` §4.
- **Done when:** either the receipt exists and the wording is restored with a link to it, or
  the wording is permanently reduced to what is evidenced.

### DIST-01 · Private releases block eleven distribution channels at once

- **Evidence:** `DX.md:313` — "most stores refuse private-repo URLs. Flipping
  `essence-dx/cli` (and later each tool repo) from private to public costs $0 and unblocks
  rows 1–11 at once." The free-first matrix at `DX.md:287` stands at **8 of 14 channels
  READY** (rows 1–8 ready, 9–11 TODO, 12–14 LATER), and none of the eight can actually
  publish without a public release URL and SHA-256.
- **This is the highest-leverage open item in the file:** one $0 action clears eleven rows.
- **Done when:** `cli` has a public release using the 6-target asset naming proven at
  `DX.md:38` (`dx-<tool>-<triple>[.exe]`), and one channel from rows 1–8 is confirmed live on
  its own store page — live, not manifest-ready.

### DIST-02 · AUR package (`dx-cli-bin`)

- **Evidence:** `DX.md:297` marks row 9 **TODO**, cost $0, blocker "public release URL +
  checksums in `PKGBUILD`". Blocked by `DIST-01`, not by work.
- **Done when:** `PKGBUILD` published and `makepkg` verified in a clean Arch container.

### DIST-03 · MacPorts, pkgx and mise shims

- **Evidence:** `DX.md:298` marks row 10 **TODO**, cost $0, "copy Homebrew metadata".
- **Done when:** each manager installs `dx` from a clean machine, or the row is cut with a
  stated reason.

### DIST-04 · COPR / OBS / PPA for dnf, apt and zypper

- **Evidence:** `DX.md:299` marks row 11 **TODO**, "$0 (free build services)", needing
  "public repo + spec/dsc files in CI" — which also depends on `HUB-05`, because CI is off.
- **Done when:** spec and dsc files exist in the repo and one build service publishes a
  package.


---

## P2 — hygiene and the long tail

### MF-02 · Folders on disk that the manifest never declares

- **Evidence:** present in `Get-ChildItem -Directory` but absent from the `paths` block of
  `dx`: `daemon`, `bot`, `token`, `tree`, `decisions`, `native`, `os`, `mobile`,
  `video-player`, `extensions`, `test`, `zzz`, `bin`.
- **Why it matters:** `daemon` is the flagship (`README.md:95`) and is unrouted, so any tool
  that resolves products through the manifest cannot see the most important one.
- **Done when:** every product folder is either declared or listed as intentionally
  undeclared, and `MF-01`'s checker enforces both directions.

### MF-04 · Two parked siblings still spell "deprecated" as "depricated"

- **Evidence:** inside `DEPRICATED_10-20-2026/` sit
  `dx-video-player-depricated-20260820-1856` and
  `os-depricated-20260820-1856`. The parent folder was renamed to
  `DEPRICATED_10-20-2026` on 2026-10-20; these two were deliberately left
  alone because renaming them is a proposal, not an edit (`AGENTS.md` §2.2).
- **Constraint:** each holds a separate git repo. Rename, do not delete.
- **Done when:** both are renamed to the `DEPRECATED` spelling, or the drift is
  accepted and documented so nobody "helpfully" renames them mid-session.

### DIST-05 · The three LATER distribution rows

- **Evidence:** `DX.md:300`–`DX.md:302` — Snap Store/Flathub (store review time, not money),
  GHCR container image (needs a small Dockerfile), crates.io (needs the private `check`
  dependency made public or vendored).
- **Note on crates.io:** it is the only row gated by *code*, not by a store — the private
  `essence-dx/check` git dependency is what makes `cargo install` fail today (`DX.md:277`).
- **Done when:** each row is shipped or explicitly descoped in `CHANGELOG.md`.

### HUB-09 · A junk file with an unreachable name sat at the repo root — CLOSED 2026-10-20

- **Evidence (when it was open):** a 663-byte root file whose name was
  `` + `This` (verified by codepoint: U+F02A U+F02A U+0054 U+0068 U+0069
  U+0073). Its content was shell error text —
  `lists: cannot open \`lists' (No such file or directory)` and eleven similar
  lines — i.e. the residue of a command whose arguments were split word by word
  and redirected into a file. It was documentation of nothing.
- **It was also a live hazard:** `Test-Path -LiteralPath '**This'` returned
  **False**, so agents could not reliably reference, ignore, or clean it.
- **Closed on 2026-10-20 by evidence, not by action:** a byte-level listing of
  every root file (name escaped to codepoints) shows **no U+F02A name and no
  663-byte file** at the hub root. The file was already gone before the
  2026-10-20 parking pass; this session moved `llama.cpp`, `ZZZ-bot-trash`, and
  ten named root files, and that file was not among them.
- **Done when:** the root listing no longer contains it — verified 2026-10-20.

### HUB-10 · 933 MB of loose video and PNG at the hub root — PARTLY CLOSED 2026-10-20

- **Evidence (when it was open):** `Get-ChildItem -File` over 1 MB —
  `Frieren Beyon Journey's End.mp4` 347.8 MB, `One Piece.mp4` 296.2 MB,
  `Spiderman Into The SpiderVerse.mp4` 285.6 MB, `image.png` 3.0 MB. Total
  **932.6 MB** on a volume with ~5.2 GB free.
- **They are not tracked** (denied by `/*`), so this is a disk-pressure item, not a repo
  hygiene item, and it belongs in the same proposal queue as `ENV-01`.
- **Closed for the three videos on 2026-10-20:** they were moved (not deleted)
  to `DEPRICATED_10-20-2026/root-junk/`, together with `big146.bin` (153 MB),
  `Social.md` (3.9 MB), `small.gz` (2 MB), `$o`, `db`, `db-wal`, and
  `TODO-ESSENCE.md`. The 1 GB of video no longer sits at the root.
- **Still open:** `image.png` (3.0 MB) is still at the root.
- **Done when:** `image.png` is in `media/` or an archive by choice, not by a build
  failing on a full disk.

### ENV-01 · The volume is full; the sweep is a proposal, not a task

- **Evidence:** `G:` at **99% — 477 GB total, 472 GB used, 5.2 GB free**; `G:\Dx` is ~166 GB
  of which **~88 GB is regenerable build/dependency output**
  (`docs/space-review-2026-09-23.md:5`).
- **Do not act on this item by deleting.** The review that produced the number deleted
  nothing on purpose (`docs/space-review-2026-09-23.md:3`). The deliverable here is an
  approved, itemised, size-backed sweep list — and `HUB-04`'s scripts make that list
  reproducible.
- **Done when:** an approved sweep manifest exists, sizes are re-measured after any agreed
  action, and free space is above 50 GB.

### ENV-02 · Dead duplicate Rust toolchains inside the hub

- **Evidence:** `G:\Dx\.rustup` (1.2 GB, one toolchain, nothing references it) and
  `G:\Dx\.cargo` (103 MB, registry only, no `bin/`, no `config.toml`) while the live homes
  are the junctions `~/.rustup` → `G:\DevCaches\rustup` and `~/.cargo`, and `RUSTUP_HOME` /
  `CARGO_HOME` are unset (`docs/space-review-2026-09-23.md:23`).
- **Leave alone:** `check/.cargo`, `cli/.cargo`, `daemon/.cargo`, `dcp/.cargo` hold only a
  per-repo `config.toml` and are intentional (`docs/space-review-2026-09-23.md:39`).
- **Done when:** a decision is recorded, and `AGENTS.md` §8 reflects whichever homes are live.

### ENV-03 · One parked repo is mid-operation and must not be touched

- **Evidence:** `DEPRICATED_10-20-2026/mobile/` (branch `dev`) has 0 unpushed commits but
  **15,889 files staged for deletion** plus 50 untracked entries; the review's own verdict is
  "Mid-operation, not clean. **Review before touching**" (`docs/space-review-2026-09-23.md:68`).
- **The other 21 of 22 are clean** and recoverable by plain `git clone`.
- **Done when:** the staged deletion is either committed deliberately or reset, and the
  folder joins the clean-21 claim.

### DOC-04 · The README appendix cites files that are not in the hub

- **Evidence:** the appendix at `README.md:144` onward cites `qwen3.8-27b-research.md`,
  `kaggle-qwen38/`, `qwen/README.md`, `qwen/run.ps1`, `qwen/status.ps1`, `qwen/stop.ps1` and
  `run_final.log`. `Test-Path` on `qwen`, `kaggle-qwen38` and `qwen3.8-27b-research.md` →
  **False**. Every `file:line` citation in those 84 lines is therefore unresolvable from this
  repo.
- **Separately, it is in the wrong place:** a Kaggle GPU log is not a product README, and
  `docs/README.md:70` already assigns one page per folder.
- **Done when:** the appendix moves to `docs/` as its own dated receipt page with the correct
  owning path named, or its citations are corrected to paths that exist.


---

## BLOCKED — held by something not present on this machine

These stay visible on purpose. A blocker that is not written down becomes a silent
"we never got to it".

| ID | Blocked on | Consequence | Receipt |
| --- | --- | --- | --- |
| **BLK-01** | No Android SDK or emulator installed | No device/mobile verification possible | `OPEN-WORK-2026-09-19.md:46` (Q-C10 "needs an Android SDK/emulator (**none installed**)") |
| **BLK-02** | `sshd.exe` absent, nothing on `:22` | SSH-tunnel path is written and tested but never run live | `OPEN-WORK-2026-09-19.md:29` (Q-A16) |
| **BLK-03** | `src-tauri` cannot link (`ort-sys`) | Desktop spawn/supervise, WSL choice, OS notification host op cannot be compiled here | `OPEN-WORK-2026-09-19.md:73` |
| **BLK-04** | No `GH_TOKEN`/`GITHUB_TOKEN` in this shell; `essence-dx` is private | Full `cargo` builds 401 on the private `check` dependency; release assets unverifiable anonymously | `DX.md:277`, `DX.md:34` |

**Un-spelling note:** `BLK-04` also blocks `DIST-01`, and `DIST-01` blocks `DIST-02`,
`DIST-03` and `DIST-04`. Four of the five distribution items collapse into one credential
plus one repo visibility change.

---

### TPU-02 · Overlap the weight DMA with compute

- **Evidence:** MaxText's Pallas performance guide prescribes multi-buffering and a
  custom prefetch block order through a scalar prefetch grid to overlap HBM loads with
  compute; the distributed Pallas docs describe `pl.BlockSpec(memory_space=pl.ANY)` as
  the way to let the compiler place operands in HBM for overlap. Notebook 14's kernel
  sets `num_scalar_prefetch=0` and never uses `pl.ANY` — so this is **untested** on this
  track, not refuted.
- **The baseline it must beat, and the ceiling on this lever:** the weight DMA of one
  projection is 0.29 ms of its 3.83 ms and the unpack arithmetic is the rest
  (`tpu/13-cost-sweep/README.md:63-76`). So even a perfect overlap buys at most the
  **7.6 %** of the step, and the A/B behind the last `CHANGELOG.md` entry says the step
  is VPU-bound rather than DMA-exposed. This is a small, well-priced lever — worth one
  session, not worth hoping on.
- **The knob exists and has been tried:** `PREFETCH` in
  `tpu/14-bonsai-transposed/kernel.py` sets `num_scalar_prefetch` on the
  `PrefetchScalarGridSpec`. **`PREFETCH=1` does not compile with this grid** --
  Pallas reports *"Pytree for `in_specs` and `inputs` do not match"*, because the
  prefetch mechanism wants the prefetched dimension to be a *scalar* dim and this
  kernel's grid is `(TOUT, 1, BPC)` with the parallel dim first. The `PREFETCH=0`
  control is `receipt-14-pf0-b1-f16-tc1024.json` (5.52 tok/s, 181.3 ms/step). The
  item stays open only for a grid that satisfies the mechanism, and its 7.6 % ceiling
  says it is not worth one.
- **Done when:** a receipt records a lower ms/step than TPU-01's run with the output
  text unchanged, and the prefetch and memory-space settings are in the receipt.

### TPU-03 · Multi-token verification: the only route to 50 tok/s on the TPU

- **Why it is the only route (measured 2026-10-20):** cutting 36 % of the kernel
  launches moved the step time by 0.5 % (`receipt-14-fuse0-b1-f16-tc1024.json` against
  `receipt-14-fuse1-b1-f16-tc1024.json`: 182.3 ms against 181.3 ms). The step pays for
  the per-weight-byte VPU work — the ternary unpack — which does not shrink with batch
  (180.6 ms/step at batch 1, 284.7 ms/step at batch 80). A verification pass computes
  k tokens against the *same* unpacked weights, so it amortises that cost k ways; no
  other lever on this kernel does.
- **The arithmetic that has to come out right, corrected 2026-10-20:** a pass costs
  ~200 ms, so 50 tok/s needs **9-10 accepted tokens per pass** -- and no real draft
  delivers that. An MTP head accepts ~2-3 tokens per pass, n-gram matching under 1,
  and MTP acceptance falls with depth unless the head is fine-tuned (FastMTP, arXiv
  2509.18362: 80 % / 56 % / 36 % at k=1/2/3 for a vanilla head). **So the honest
  ceiling of this mechanism on the TPU is ~10-15 tok/s, and 50 tok/s there needs the
  unpack itself ~9x faster** (`TODO.md` `TPU-07`), not more speculation.
- **The hard part is named:** 48 of this model's 64 layers are gated-delta-net, so a
  k-token chain needs the GDN state updated k times in sequence against one state, not
  k independent states; the 16 attention layers can take k queries in one pass behind a
  chain mask. The projections are shared across the chain, and the Pallas out-tile is
  already `(B, TC)`, so batching k candidates through the existing kernel is free —
  the recurrence and the mask are the work.
- **Done when:** a receipt records **≥ 2 tokens accepted per forward pass** and
  **≥ 15 tok/s single-stream at `max_t` 8192**, with the acceptance rate measured rather
  than assumed, and the same text as the single-token run for the accepted prefix.

### TPU-04 · 64k context needs a q4 KV cache that does not exist yet

- **Evidence:** `tpu/14-bonsai-transposed/receipt-14-mem-ceiling.json` — at `max_t`
  65536 the attention step cannot even be loaded (`jit_attn_step` wants 3.26 G, 184.53 M
  free) and at 49152 it is 140 MB short; 45056 fits and runs at 2.86 tok/s
  (`receipt-14-b1-f16-tc1024-45056.json`). fp32 KV is 128 KiB/token/sequence, so 64k is
  8 GiB of KV on a 15.75 GiB budget that already holds 6.37 GiB of weights.
- **The fix and its price:** q4 KV is 16 KiB/token/sequence, so 64k becomes 1 GiB and
  the window fits at ~11.3 GiB — but the kernel has no q4 KV path and no blocked or
  paged attention, and both are new kernel work rather than a configuration change.
- **Done when:** a receipt records a 65536-token window running end to end with the
  model, and the q4 KV round-trip error is bounded in the receipt.

### TPU-05 · The Colab v5e1 pool is quota-limited, and the unlock is a re-auth

- **Evidence:** 25 consecutive `503 Service Unavailable` on `colab new --tpu v5e1`
  (2026-10-19 12:31–13:08), then `Session READY` on the first attempt after the
  account was re-authorised; a second round the same evening failed 3 times and
  succeeded on try 4. Meanwhile `colab new` with no accelerator assigned a CPU session
  immediately, and `--tpu v6e1` was refused with *"You may not have quota or
  entitlement"* — so the account is fine and the accelerator pool is the constraint.
- **Done when:** `tpu/DO-NOT-RETEST.md` carries the unlock procedure and the symptom, so
  the next agent does not spend 40 minutes rediscovering it.

### TPU-06 · The context cost at fixed batch has never been measured

- **Evidence:** the step-time curve in `tpu/14-bonsai-transposed/README.md` moved batch
  and context together — 284.7 ms (batch 80, `max_t` 64), 309 ms (batch 64, `max_t` 160),
  638 ms (batch 48, `max_t` 320), 662 ms (batch 32, `max_t` 512) — so **the curve cannot
  be attributed**: the two longest-context runs also used the *smaller* batches, which
  rules out batch as the sole cause and leaves context unquantified. A claim made in
  conversation on 2026-10-19 that those jumps were "batch-driven, not context" does not
  survive that comparison and is withdrawn.
- **Why it matters:** every long-context plan (TPU-03, TPU-04) is priced against the
  context cost, and the 8k-window target is only meaningful once that cost is known.
- **Done when:** one run at batch 1 with `max_t` ∈ {64, 1024, 8192, 45056} prints the
  ms/step curve at fixed batch, and the receipt states the per-1k-token cost.

---

### TPU-07 · The real TPU blocker is the ternary unpack, ~9x off the HBM roofline

- **Evidence:** the step reads 6.37 GiB of packed weights per token, and the slice's
  measured read bandwidth is 709 GB/s (`tpu/13-cost-sweep/README.md:28`), so the
  traffic floor is **9.0 ms/token -- 104 tok/s**. The measured step is **181 ms
  (5.5 tok/s)**, i.e. **~20x off that floor**, and the projection breakdown puts the
  unpack at **82 % of a projection** (3.13 ms of 3.83 ms) -- the rest is the weight
  DMA (0.29 ms) and the matmul.
- **What has been ruled out, by measurement:** HBM bandwidth (86 % of spec on this
  slice), program count (`TO` 128 -> 1024 is 1.18x, saturating at 512), kernel
  launches (36 % fewer launches moved the step 0.5 %, 2026-10-20), DMA count, the
  k-loop, and the unpack's instruction count (a 1.4x cheaper shift chain changed
  nothing -- the cost is data movement *inside the VPU*).
- **What is still open:** the decode is bound by VPU data movement, and the known
  spellings measure 1.393 ms per projection (int16 lanes, `concatenate`) against a
  0.29 ms DMA floor. A formulation that never materialises the unpacked weights --
  folding the per-dimension power pattern into the activation, so the kernel only
  has to widen 2-bit values -- is the unexplored direction, and the only one that can
  put the TPU in the same range as the GPU route.
- **Done when:** a receipt records a step time below 40 ms at batch 1 / `max_t` 8192
  with the first token still `271` and the text byte-identical to
  `receipt-14-fuse1-b1-f16-tc1024.json`.

## The execution track — this repo owns `tpu/`

This repository's origin is `manfromexistence/bonsai2.git` and it tracks `tpu/**` — 22
notebook folders, `git ls-files` for the ground truth — so the Bonsai execution track's
open work belongs in this ledger rather than in a file no agent would read.
`tpu/TPU-STATUS.md` records what the track achieved; `tpu/DO-NOT-RETEST.md` records the
facts that must not be re-measured. Neither is a ledger of open work, and neither is a
substitute for this section.

The target this section is written against: **50 tok/s for a single stream on an 8k
context window**, for this model, **on TPU**, with a receipt.

**Scope (2026-10-20): this track is TPU-only.** A GPU run is not a result of this
project and may not be filed here (`AGENTS.md` section 2, rule 7; `tpu/README.md`
section 6). The GPU-route items that were in this section -- the 50 tok/s GPU
route, n-gram stacking on top of MTP, and the MTP-head graft -- were withdrawn
on that decision; the facts they produced (Bonsai 2's GGUFs carry no MTP head;
the base model does) are kept in `tpu/DO-NOT-RETEST.md` section 14 because they
are facts about the artifact, and the route to 50 tok/s is now `TPU-03` plus
`TPU-07`.

---

## Owned elsewhere — what the hub knows but must not claim

The hub records these numbers because they have receipts on this machine. It does **not**
re-verify them, and none of them may be quoted as hub status.

| Ledger | Recorded state | Receipt |
| --- | --- | --- |
| `bot/` (flow) | 3 waves, **25 open items**; Wave Q 31 of 46 closed (67 %), AI Harness War 3 open, Wave R 8 of 15 (53 %). "**Nothing is certified for release.**" | `OPEN-WORK-2026-09-19.md:6` |
| Distribution matrix | 8 of 14 channels READY, 3 TODO, 3 LATER | `DX.md:287` |
| `daemon` | Advertised "all 11 targets shipping, cloud-offloaded visual memory live" — **no page, no receipt in `docs/`** | `README.md:95`, `DOC-01` |
| Kaggle Qwen run | `COMPLETE`, ~10.15 tok/s mean on P100 — **runtime/hosted claim**, source files not in this repo | `README.md:144`, `DOC-04` |

---

## Score rubric — the arithmetic behind the percentage

The number published in `README.md` is derived from this table, not felt. Score each
dimension 0–100 against its definition; the weights sum to 100.

| # | Dimension | Wt | Before | After | Why not higher |
| --- | --- | --- | --- | --- | --- |
| 1 | Public story and honest status (`README.md`) | 20 | 40 | 85 | four phantom projects (`DOC-03`) and an appendix citing absent files (`DOC-04`) still open |
| 2 | Agent operating instructions (`AGENTS.md`) | 18 | 0 | 80 | written, but nothing yet proves an agent can *verify* a claim — `HUB-04` |
| 3 | Open-work ledger (`TODO.md`) | 18 | 0 | 85 | no executable checker; items are hand-measured (`HUB-04`) |
| 4 | Change history (`CHANGELOG.md`) | 12 | 15 | 80 | has been paid since the first reading against a file that **did not exist on disk** until 2026-09-26; it is now backed, but the first entry is unreleased and no version tag exists |
| 5 | Achievement register (`docs/`) | 15 | 35 | 70 | flagship has no page (`DOC-01`); 9 folders page-less (`DOC-02`); corpus uncommitted (`HUB-01`) |
| 6 | Manifest and routing integrity (`dx`) | 12 | 20 | 65 | 7 dangling paths (`MF-01`), 13 undeclared folders (`MF-02`) |
| 7 | Version-control durability | 5 | 40 | 60 | 24 hub files exist on one machine (`HUB-01`); tree still dirty (`HUB-02`) |
| | **Weighted** | **100** | **19.45 %** | **77.6 %** | |

**Before:** `40·20 + 0·18 + 0·18 + 15·12 + 35·15 + 20·12 + 40·5`
`= 800 + 0 + 0 + 180 + 525 + 240 + 200 = 1945 / 100` = **19.45 %**
**After:** `85·20 + 80·18 + 85·18 + 80·12 + 70·15 + 65·12 + 60·5`
`= 1700 + 1440 + 1530 + 960 + 1050 + 780 + 300 = 7760 / 100` = **77.6 %**

Both lines were written out longhand on 2026-09-26 because the totals above them had been
asserted as `2235` and `7400` and neither is the sum of the products beside it.

### Path from 78 % to 100 % — a gap of 22.4 points, priced at 26.0

| Close these | Points | Lands at |
| --- | --- | --- |
| `DOC-01` + `DOC-02` — register covers every folder, every claim tiered | +4.5 | 82.1 % |
| `MF-01` + `MF-02` — every declared path resolves, every product declared, checker in `scripts/` | +4.2 | 86.3 % |
| `HUB-01` + `HUB-02` — corpus committed **and pushed** | +3.0 | 89.3 % |
| `DOC-03` + `DOC-04` + `HUB-03..08` — public claims match the tree, policies and scripts exist | +6.3 | 95.6 % |
| `HUB-04` — an agent can re-run the checks instead of trusting this file | +1.8 | 97.4 % |
| First tagged release; `CHANGELOG.md` moves off *Unreleased* | +1.4 | 98.8 % |
| P2 + BLOCKED cleared or explicitly descoped | +4.8 | **103.6 %** |
| | **26.0** | over 100 by **3.6** |

**The last row is the honest part of this table, so it is left standing rather than smoothed.**
These seven increments total **26.0** against a remaining gap of **22.4**, so they over-promise
by 3.6 points: they were priced against the incorrect 74 % base, and several of them pay for
overlapping work — `HUB-04`'s checker is inside `MF-01`'s done-when *and* inside
`DOC-01`'s tiering, and the P2/BLOCKED tail is largely the same distribution channels
`DIST-01`…`DIST-05` already count once. **Read the increments, not the cumulative column.**
The cumulative column is arithmetic on estimates; it is shown so the overshoot is visible.

**The expensive points are the last ones.** 78 % means the hub can now be handed to another
agent and understood — that its files exist, its ledger names what is open, and its headline
number survives being added up. It does **not** mean any product is release-qualified, and per
`AGENTS.md` §4 no wording in this repo should imply that it does.

---

MIT OR Apache-2.0
updated: 2026-10-20 20:45 UTC
