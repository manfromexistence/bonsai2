# GAPS.md - dxbot (Tauri) against T3 Code (Electron), deep pass 2026-10-31

**What this file is.** A deliberately unkind comparison between `G:\Dx\bot`
(dxbot, Tauri desktop) and `G:\Dx\t3code` (T3 Code: Electron desktop, web,
server and mobile), written after syncing the T3 Code clone to upstream. It
answers one question: *what does T3 Code do better, and how would each gap be
closed?* It is not an inventory of dxbot; the "what we lead on" section exists
so the document is not a one-sided advert.

**Claim tiers.** Every row is **repo-managed**: a path in one of the two trees,
named so it can be opened. Nothing was built, launched or benchmarked for this
document. No row is `executed`, `runtime` or `release-qualified`.

**Provenance.**

| Fact | Value |
| --- | --- |
| T3 Code clone | `G:\Dx\t3code`, remote `pingdotgg/t3code` |
| T3 Code synced to | `6fc9144a` (`origin/main`, 2026-10-31; 31 commits pulled) |
| Local T3 Code change | `pnpm-lock.yaml` stashed before the fast-forward (`stash@{0}`); `TAURI.md` untracked and untouched |
| dxbot measured at | `be20e485` on `main` (= `bot/main`) |
| T3 Code licence | MIT, (c) 2026 T3 Tools Inc. |
| dxbot licence | AGPL-3.0 |

---

## 0. The brutal one-paragraph answer

T3 Code is the better **agent cockpit**. Its terminal is Ghostty's VT engine,
its right panel is a composable surface system (Browser, Terminal, Files,
Diff, Pull request, Linked pull requests, Device), its browser preview has a
CDP relay, passkeys, recording, element picking and annotations, its projects
have branches, worktrees and scripts, and its Windows story is WSL plus a dev
container that ship. dxbot is the larger product - own agent engine, voice,
dictation, grammar, memory, plan, dashboard, 14-tab right pane - but several
of its T3-shaped surfaces are **frames around a running T3 Code sidecar, not
ports**: `T3_NATIVE_ROUTES` is empty. The honest scoreboard is "wider product,
shallower cockpit, and a framed comparison surface that must not be counted as
native capability".

---

## 1. The two products, side by side

| | T3 Code | dxbot |
| --- | --- | --- |
| Shape | Agent harness control surface for *other* CLIs | Local-first AI workspace with its own Rust harness |
| Desktop shell | Electron | Tauri 2 |
| Clients | Web, Electron desktop, iOS and Android | Tauri desktop; Android scaffold; no iOS project |
| Backend | `apps/server` Node CLI, outlives every client | `server/` Rust crate, port 3773 |
| Terminal engine | `libghostty-vt` (C ABI, WASM + native) | xterm.js 6 + host-op PTY |
| Right panel | 7 surface types, inline or sheet, per-surface runtime ids | 14 tabs, one column width, desktop/mobile sheet |
| Ported T3 surfaces in dxbot | - | `/t3code/*` routes frame the upstream app; **0 native ports** |
| Provider layer | 12 provider/effect packages | 6 drivers |
| Source control | 9 packages + PR surfaces | 4 SCM clients + a GitHub section, no PR tab |
| Remote reach | Clerk + Cloudflare relay + push | LAN / tailnet / SSH |
| Install channels | npx, scripts, winget, scoop, brew, deb, AUR, Releases | GitHub Releases only |

---

## 2. Gaps, ranked by what a user feels

| # | Gap | Severity |
| --- | --- | --- |
| G1 | `/t3code/*` surfaces are frames, not ports | CRITICAL TO HONESTY |
| G2 | Terminal fidelity and multi-client sharing | HIGH |
| G3 | WSL ships; our hypervisor guest has never booted | HIGH |
| G4 | Right panel surface system | HIGH |
| G5 | Project, branch and worktree workflow | HIGH |
| G6 | Background threads, notifications, badges | HIGH |
| G7 | Thread lifecycle in the sidebar | MEDIUM-HIGH |
| G8 | Command palette breadth | MEDIUM-HIGH |
| G9 | Guest links, prompt links, sharing | MEDIUM-HIGH |
| G10 | Portable handoffs and thread lineage | MEDIUM-HIGH |
| G11 | SnapShot annotated capture | MEDIUM-HIGH |
| G12 | Diagnostics and observability | MEDIUM |
| G13 | Update orchestration | MEDIUM |
| G14 | Secrets providers | MEDIUM |
| G15 | MCP server and MCP apps | MEDIUM |
| G16 | Pull request workspace | MEDIUM-HIGH |
| G17 | Triage workflow | MEDIUM |
| G18 | Activity log | MEDIUM |
| G19 | Mobile experience and widgets | MEDIUM-HIGH |
| G20 | Background service and CLI | MEDIUM |
| G21 | Keybindings as a product | MEDIUM |
| G22 | Contracts, tests, docs, browser tests | MEDIUM (compounding) |
| G23 | Distribution and release engineering | MEDIUM |
| G24 | Agent orchestration depth | HIGH (architectural) |
| G25 | Environment model | MEDIUM |

### G1 - The `/t3code/*` surfaces are frames, not ports (CRITICAL TO HONESTY)

dxbot exposes 26 T3 routes under `src/app/t3code/**`, including Pull
requests, Scheduled tasks, Diagnostics, Keybindings, Snap and Source control.
Every one of them renders `T3CodePage`, which **starts the T3 Code sidecar and
frames its upstream URL**:

- `src/features/t3code/components/t3code-page.tsx` - "This is not a port of
  t3code's UI ... When the routes are ported into this app's Next build, this
  component is what gets replaced."
- `src/features/t3code/native/index.ts` - `T3_NATIVE_ROUTES` is **empty on
  purpose**; nothing is ported.
- `src-tauri/src/t3code_host.rs` - supervises the sidecar (`DX_T3CODE_ROOT`,
  `T3CODE_RELATIVE = ../inspirations/t3code-build`).

**Consequence.** "We have the scheduled tasks page" currently means "we can
show T3 Code's scheduled tasks page inside dxbot". It must not be counted as a
dxbot capability in any ledger, and the framing must be labelled in the UI.

**Closing it.** Port surfaces one at a time into the existing tab registry and
delete them from `T3_ROUTES` as they land. The empty registry is the right
mechanism; it just has to stop being empty.

### G2 - Terminal fidelity and multi-client sharing (HIGH)

**They have.**

- Ghostty's VT engine, not an xterm compatibility layer:
  `apps/web/src/terminal/ghostty/README.md` - `libghostty-vt` C ABI compiled to
  WASM; Canvas 2D renderer; IME, selection, scrolling, links, cursor in
  `surface.ts`; a vendored Nerd Font for prompt glyphs.
- Server-owned PTYs that outlive the renderer and can be shared or
  reconnected: `docs/internals/terminal-runtime.md`.
- Bounded retention: 5,000 lines / 8 MiB UTF-8 server, 512 KiB client, with
  query/response traffic stripped from replayed history.
- Native mobile terminal views in Kotlin and Swift:
  `apps/mobile/modules/t3-terminal/android/.../TerminalCanvasView.kt`,
  `apps/mobile/modules/t3-terminal/ios/T3TerminalView.swift`.

**We have.** Real PTY host ops, session reuse across reloads, split panes and
maximise without remount: `src/features/terminal/core/transport.ts`,
`src/features/dx/components/chat/terminal-workspace.tsx`; renderer is xterm 6.

**Gap.** A good xterm in a good layout vs a VT engine with shared scrollback
and native mobile surfaces. Closing: Ghostty adapter behind
`terminal-view.tsx`; server-side scrollback owner for two-client attach.

### G3 - "VM": WSL ships, our hypervisor guest has never booted (HIGH)

T3 Code has **no embedded hypervisor**:

- **WSL runtime**: Windows builds embed a same-arch Linux CLI archive and run
  agents/projects inside a chosen distro (`docs/user/install.md`, install
  section on WSL; `docs/operations/release.md` `--wsl-runtime`;
  `apps/desktop/src/wsl/`).
- **Dev container**: Ubuntu 24.04, Node 26, pnpm, Rust, `vp`, `gh`
  (`docs/internals/devcontainer.md`).
- **Sandbox delegated per provider**, not owned (`docs/user/cursor.md`,
  `docs/orchestration-v2/provider-capability-system.md`).
- **Default browser** and Finder HTML files can open into a browser surface
  (`docs/user/default-browser.md`), which is a desktop-integration depth we do
  not have.

dxbot: `src-tauri/src/host_vm.rs` (1,863 lines) is a real WHP guest with
refusals and a `GuestExec` trait, but the measured status is 32/100 capability,
~5/100 verified-shipping, **zero boots** (bot `AGENTS.md`, goal 9). No WSL
runtime, no dev container.

Closing: WSL backend first; keep the WHP guest as a separate research track
until a boot is measured.

### G4 - Right panel surface system (HIGH)

`apps/web/src/components/RightPanelTabs.tsx:319-369` declares Browser,
Terminal, Files, Diff, Pull request, Linked pull requests and Device. Each
surface has a runtime tab id, rename, mute, reorder, can be opened per browser
profile, and the panel is inline or a sheet with per-frame resizing
(`ChatRightPanel.tsx`, `RightPanelSheet.tsx`). `PersistentThreadTerminals.tsx`
keeps a thread's terminals across navigation.

dxbot: `RIGHT_PANE_TABS` (`right-pane.tsx:59`) = menu, search, files, terminal,
browser, chat, review, preview, environment, dashboard, plan, memory, usage,
settings. Inline/sheet and resize already exist; missing surfaces are Pull
request, Linked pull requests and Device, plus per-surface identity.

### G5 - Project, branch and worktree workflow (HIGH)

T3 Code treats a project as an object with settings, a clone flow, a favicon,
an environment badge, branch selection and base-branch selection:

- `apps/web/src/components/BranchToolbar.tsx`,
  `BranchToolbarBranchSelector.tsx`, `BranchToolbarEnvModeSelector.tsx`.
- `WorktreeBaseBranchPicker.tsx`, `WorktreeBaseBranchPicker` in the project
  flow, and `PreviousWorktreeItemContent.tsx`.
- `ProjectScriptsControl.tsx` and `projectScriptEditor.tsx`: per-project
  scripts run into a terminal.
- `ProjectCloneToastCoordinator.tsx`, `ProjectEnvironmentBadge.tsx`.

dxbot has branch and worktree choice **at new-chat time**
(`src/features/dx/components/chat/new-chat-dialog.tsx`: `branchMode`,
`worktreeMode`, `listWorktrees`), plus the backend (`host_git.rs`,
`server/src/worktree_setup.rs`) and a Git surface
(`src/features/dx/components/agent/git-surface.tsx`). What is missing is the
**in-thread workflow**: no branch toolbar beside the composer, no explicit
base-branch picker when creating a worktree, and no per-project script editor.

### G6 - Background threads, notifications and badges (HIGH)

T3 Code:

- `BackgroundThreadAnnouncer.tsx` and `backgroundThreadAnnouncement.ts`.
- `ThreadNotificationCoordinator.tsx` + logic tests, badges per thread.
- `PersistentThreadTerminals.tsx`.
- Mobile push and Live Activities: `docs/user/mobile-notifications.md`,
  `apps/mobile/src/widgets/AgentActivity.tsx`.

dxbot: no background-thread announcer; notifications are in-app toasts, and
there is no push, badge or ongoing-activity surface (confirmed by a search for
`BackgroundThread`, `mobile-notifications`, APNs/FCM in `src`, `src-tauri/src`,
`server/src`).

### G7 - Thread lifecycle in the sidebar (MEDIUM-HIGH)

T3 Code: snooze (`Sidebar.snooze.ts`, `CustomSnoozeDialog.tsx`), pin, settle,
undo close (`SidebarThreadUndoNotice.tsx`), archived migration
(`LegacyThreadMigrationToast.ts`), thread hover card
(`ThreadHoverCard.tsx`), status indicators, reopen-closed-view shortcut,
history swipe navigation (`HistorySwipeNavigation.ts`), in-thread find bar.

dxbot: shelves and snooze exist (`src/features/dx/components/browser/thread-shelves.tsx`),
bookmarks exist, but there is no thread undo notice, no hover card, no
reopen-closed-view command, and no thread-level find bar (the only find bar in
the tree is the spreadsheet showcase).

### G8 - Command palette (MEDIUM-HIGH)

T3 Code's palette is a product: projects, threads, themes, add-project,
triage, copy prompt link - split into `CommandPalette*.tsx` with logic tests
(`CommandPalette.logic.ts`, `CommandPaletteThemeItems.tsx`,
`CommandPaletteProjectItems.tsx`, `CommandPaletteAddProject.tsx`).

dxbot has a "Search and Command Palette" dialog
(`src/features/dx/components/sidebar.tsx:917`) with suggestions and recent
tabs, plus a file Quick Open. What it does not have is T3 Code's palette
breadth: projects, all threads, themes, add-project, triage, and prompt-link
actions in one list.

### G9 - Sharing: guest links, prompt links, public read-only follow (MEDIUM-HIGH)

- Guest links let a teammate follow one thread in a browser with no account,
  and their messages queue until the owner sends or dismisses them
  (`docs/user/guest-links.md`; beta behind Labs + T3 Connect).
- Prompt links open a draft prompt, never execute, and cannot change mode,
  branch or worktree (`docs/user/prompt-links.md`); sent messages expose a
  "Copy prompt link" button.
- ChatGPT sharing control exists in the composer
  (`ChatGptSharingControl.tsx`).

dxbot: no guest links, no prompt links, no share-link surface. A search for
`guest link`, `prompt link` and `ChatGptSharing` finds nothing.

### G10 - Portable handoffs and thread lineage (MEDIUM-HIGH)

T3 Code transfers context when switching providers, after a portable restart,
or on a fork the provider cannot resume (`docs/user/portable-handoffs.md`):
short conversations intact; long ones favor recent turns, the original request
and command outcomes; Codex gets native history where supported; the rest get
attributed context; reasoning/tool state/attachments are not copied.

Its orchestration model is a run graph (`AppThread -> Run[] -> ExecutionNode`)
with thread fork/lineage and worktree handoff (`t3_worktree_handoff`).

dxbot: thread + turn only; no fork, lineage, handoff or provider-switch
context transfer (`server/src/orchestration/command.rs` has no `Fork`; no
`worktree` in `server/src/rpc.rs`). This is the deepest architectural gap.

### G11 - SnapShot: annotated screen capture (MEDIUM-HIGH)

T3 Code ships a full capture flow in the desktop shell:
`apps/desktop/src/snapShot/DesktopSnapShot.ts`, `captureConfigKdl.ts`,
`captureConfigEdit.ts`, `CaptureShortcutConfig.ts`, `ActiveWindow.ts`, plus a
settings route (`apps/web/src/routes/_chat.settings.snap-shot.tsx`) and
contracts (`DesktopSnapShot`, `SnapShotShortcut`, modifier pairs). It is not a
screenshot button; it is a configurable capture + annotation surface.

dxbot has OCR features and a notes/annotation stack, but no equivalent
capture-shortcut/annotation product. A search for `snapShot` finds only the
framed T3 route.

### G12 - Diagnostics and observability (MEDIUM)

T3 Code: `SlowRpcRequestToastCoordinator.ts`, `HostFreezeToastCoordinator.ts`,
`ConnectionStatusDot.tsx`, `RenderErrorBoundary.tsx`, `os-jank.ts`,
`apps/server/src/diagnostics/`, `apps/server/src/observability/`,
`apps/server/src/telemetry/`, desktop `DesktopTelemetryPublisher.ts`,
diagnostics settings route.

dxbot: `src-tauri/src/analytics.rs` and logs, but no slow-RPC, host-freeze,
connection-quality or render-error surface, and no telemetry publisher to a
server.

### G13 - Update system (MEDIUM)

T3 Code: desktop update logic + toast (`desktopUpdate.logic.ts`,
`desktopUpdate.toast.tsx`), server update action (`ServerUpdateAction.tsx`),
provider update launch notifications
(`ProviderUpdateLaunchNotification.logic.ts`), "update everything"
(`useUpdateEverything.ts`, `UpdateEverythingDialog.tsx`), release notes,
`QuitHoldOverlay.tsx`, `QuitHold.ts`, `DesktopRemoteUpdates.ts`.

dxbot: `update_checker.rs` and the Tauri updater; no provider/server update
orchestration, no update-everything, no quit-hold coordination.

### G14 - Secrets and identity providers (MEDIUM)

T3 Code: `packages/secret-provider-core`,
`secret-provider-definitions`, `secret-provider-onepassword` (1Password
integration), plus environment auth (`docs/internals/environment-auth.md`) and
Clerk identity.

dxbot: OS keychain through `src-tauri/src/crypto.rs` and `host_secret.rs`;
no external secret-manager provider.

### G15 - MCP: server, apps and thread metadata (MEDIUM)

T3 Code: `apps/server/src/mcp/`, `apps/server/src/mcpApps/`,
`packages/contracts/src/threadMetadataMcp.ts` - agents can rename, settle and
link PRs themselves through MCP, and MCP apps render in the panel.

dxbot: MCP **client** support exists (`src/features/acp/core/client.ts`,
`src/features/dx/components/agent/mcps-surface.tsx`), but there is no MCP
server that exposes thread state, and no MCP app host.

### G16 - Pull request workspace (MEDIUM-HIGH)

T3 Code: `apps/server/src/pullRequest/`, `apps/server/src/review/`,
`PullRequestThreadDialog.tsx`, `PullRequestContextDetails.tsx`,
`packages/contracts/src/pullRequest.ts`, `threadPullRequest.ts`,
`docs/internals/pull-request-file-revisions.md`, nine source-control packages.

dxbot: clients for GitHub, GitLab, Bitbucket, Gitea and Azure DevOps
(`src/features/integrations/core/`), a GitHub PR section, and a framed PR
route. Missing: a PR tab, linked-PR workflow from a thread, per-file revision
view, and review comments.

### G17 - Triage workflow (MEDIUM)

T3 Code: `apps/server/src/triage/`, command-palette "Triage an issue", a new
thread with no project, agent reads logs/state/source for that version, asks
before fixing or posting (`docs/user/triage.md`).

dxbot: no triage entry point. Support surfaces exist in the CMS showcase, not
for dxbot itself.

### G18 - Activity log (MEDIUM)

T3 Code groups consecutive tool calls into an expandable summary with commands,
inputs, status, exit codes, live updating output, "Open diff", and
orchestrator summaries ("Ran 2 commands and sent messages to 3 threads")
(`docs/user/activity-log.md`).

dxbot renders tool parts (`message-parts.tsx`, `tool-network-table.tsx`) but has
no grouped activity summary or orchestrator activity semantics.

### G19 - Mobile experience and widgets (MEDIUM-HIGH)

T3 Code mobile: native terminal, notifications, Live Activities, home-screen
widgets (`apps/mobile/src/widgets/AgentActivity.tsx`,
`SubscriptionUsage*.tsx`), navigation, offline persistence, store releases
(`docs/operations/mobile-app-store-screenshots.md`).

dxbot: Android scaffold only (`src-tauri/gen/android`), no iOS project, no
widgets, no push.

### G20 - Background service and CLI (MEDIUM)

T3 Code: `t3 service install/status` on Linux/macOS, server CLI, headless
environment (`docs/user/background-service.md`, `apps/server/src/cli/`).

dxbot: `server/` binary and CLI module (`server/src/cli.rs`) but no user
service install and no documented headless mode.

### G21 - Keybindings as a product (MEDIUM)

T3 Code: keybinding contract (`packages/contracts/src/keybindings.ts`), a
settings route (`_chat.settings.keybindings.tsx`), warnings
(`KeybindingsConfigWarning.tsx`), update toast, keyboard focus doc.

dxbot: `src/features/settings/keybindings.ts` contract only; no page and no
storage path (the bot's own gap history already records this).

### G22 - Contracts, tests, docs, browser tests (MEDIUM, compounding)

T3 Code, measured today: **74** non-test contract modules under
`packages/contracts/src`, **92** docs under `docs/`, **1,755** `*.test.ts(x)`
files across the repo including `*.browser.test.tsx`, 1,730 `.ts/.tsx` files in
`apps/web/src` and 1,238 in `apps/server/src`.

dxbot, measured today: five ported contract files, **13** docs under `docs/`,
**582** TS test files, 141 `.rs` files in `src-tauri/src` and 31 in
`server/src`. The framed `/t3code/*` routes are the clearest symptom: without a
shared contract package, a surface drifts into a frame.

### G23 - Distribution and release engineering (MEDIUM)

T3 Code: six desktop artifacts in parallel, macOS/Linux/Windows signing,
Windows ARM, jsign on Linux runners, verify runners, npx/scripts/winget/scoop/
brew/deb/AUR, WSL archive embedded in installers.

dxbot: `release.yml` matrix = macOS + Windows, GitHub Releases only.

### G24 - Agent orchestration depth (HIGH, architectural)

From T3 Code's contracts and docs: run graph, execution nodes, subagents,
thread fork/lineage, provider handoff, worktree handoff, thread search across
environments, per-agent resource telemetry, command receipts and durable
events. dxbot has event sourcing, checkpoints, settlement and durable jobs, but
a thread is one linear turn stream. Every feature that depends on "this thread
was forked from that one" or "this run has three nodes" is blocked on this.

### G25 - Environment model (MEDIUM)

T3 Code: projects belong to environments; environments are machines (local,
WSL, SSH) with their own theme, auth and telemetry; the UI has an environment
selector in the branch toolbar and an environment context chip.

dxbot: a right-pane "environment" tab and remote/tailnet/SSH backends, but no
project-to-machine binding and no environment-scoped settings.

---

## 3. What dxbot has that T3 Code does not

| Capability | dxbot evidence | T3 Code |
| --- | --- | --- |
| Own agent engine (no external CLI required) | `crates/`, in-process harness | none; requires a CLI |
| Voice: wake word, local ASR, TTS, dictation | `src-tauri/src/recorder.rs`, `transcribe.rs`, `local_transcription.rs`, `src/features/pill` | voice input, no local ASR/TTS stack |
| Grammar (Harper) with system-wide overlay | `grammar.rs`, `grammar_overlay*.rs`, 55/100 capability | not a feature |
| Pill/overlay UX, toasts, tray, OS integration | `pill.rs`, `toast.rs`, `tray.rs` | Electron chrome only |
| Scratchpad, canvas, research, memory, plan, dashboard, environment tabs | 14 right-pane tabs | 7 surface types |
| Token/usage accounting surfaces | `token-usage.ts`, `context-usage.tsx` | usage exists, less granular |
| Local-first storage and offline path | Rust storage + local mirror | server/cloud first |

---

## 4. Right panel, tab by tab

| Surface | T3 Code | dxbot | Gap |
| --- | --- | --- | --- |
| Browser | yes, multi-surface per profile, CDP, annotations, passkeys | partial: one tab, iframe-level | HIGH (G4, G5 preview) |
| Terminal | Ghostty VT, server-shared | xterm + splits, host-op PTY | MEDIUM (G2) |
| Files | yes | yes | parity |
| Diff | yes, worker pool, changed-files tree | yes (`review`, diff parts) | parity-ish |
| Pull request | yes, linked PRs, revisions | no tab; clients + GitHub section | HIGH (G16) |
| Linked pull requests | yes | no | HIGH |
| Device (mobile preview) | yes | no | HIGH (G19) |
| Search | global, route-level | quick-open + files | MEDIUM (G8) |
| Usage | usage + resource telemetry | usage yes; telemetry no | MEDIUM |
| Canvas / mini player | yes (`ChatCanvas.tsx`) | canvas elsewhere | parity-ish |
| Plan / memory / environment / dashboard | no tab equivalents | yes | we lead |

---

## 5. Terminal, feature by feature

| Property | T3 Code | dxbot |
| --- | --- | --- |
| VT engine | `libghostty-vt` C ABI (WASM + native) | xterm.js 6 |
| Renderer | Canvas 2D batches, React out of the loop | xterm canvas/DOM |
| PTY owner | environment server, shared by clients | host op, per pane |
| Reconnect/share | attach from any client | session reuse on reload |
| Scrollback bound | 5,000 lines / 8 MiB server, 512 KiB client | 2 MB replay per pane |
| Replay safety | strips query/response traffic | not specified in our tree |
| Splits/maximise | client layouts | yes, no remount |
| Thread-owned terminals | yes (`PersistentThreadTerminals`) | pane layout persisted, not thread-scoped |
| Mobile terminal | native Kotlin + Swift | none |
| Theming | terminal appearance settings | `features/terminal/core/theme.ts` |

---

## 6. "VM" comparison, stated plainly

| | T3 Code | dxbot |
| --- | --- | --- |
| Windows Linux runtime | WSL distro + shipped Linux CLI archive | none |
| Dev environment | `.devcontainer` + Codespaces | none |
| Agent sandbox | delegated to the provider CLI | none of our own |
| Embedded hypervisor guest | none | `host_vm.rs`, WHP probe, zero boots |
| User-facing VM surface | WSL settings + terminal | not rendered; `vm-guest.ts` imported by nothing |
| Measured status | ships in installers | 32/100 capability, ~5/100 shipping |

---

## 7. Prioritised plan

**P0 - weeks, high user impact**

1. **Port one T3 surface for real** (G1) and remove it from `T3_ROUTES`; stop
   counting framed routes as capability.
2. **WSL runtime** (G3): spawn the Linux CLI archive, map terminal/fs/git host
   ops; this is the shipping answer to "agents in Linux on Windows".
3. **Pull request surface** (G16) from the existing clients and GitHub section.
4. **Ghostty VT adapter** (G2) behind `terminal-view.tsx`, xterm as fallback.

**P1 - product reach**

5. Browser preview host (CDP, passkeys, recording, picker, annotations).
6. Guest links and prompt links (G9); they are small surfaces with large
   product effect.
7. Relay + identity before mobile (G19); then Android, then iOS.
8. Project/branch/worktree workflow (G5), command palette (G8), thread
   lifecycle (G7).

**P2 - compounding architecture**

9. Run graph, fork/lineage, provider handoff (G10, G24).
10. Shared contract package and browser-test lane (G22).
11. Diagnostics, updates, secrets, MCP server (G12-G15).
12. Distribution channels (G23).

---

## 8. Re-run the measurement

```powershell
# T3 Code side
git -C G:\Dx\t3code fetch origin --prune
git -C G:\Dx\t3code rev-list --left-right --count main...origin/main
Get-ChildItem G:\Dx\t3code\packages -Directory | Measure-Object
Get-Content G:\Dx\t3code\apps\web\src\terminal\ghostty\README.md
Select-String G:\Dx\t3code\apps\web\src\components\RightPanelTabs.tsx -Pattern 'label:'
Get-ChildItem G:\Dx\t3code\apps\desktop\src\preview
Get-ChildItem G:\Dx\t3code\apps\desktop\src\snapShot
Get-Content G:\Dx\t3code\docs\user\portable-handoffs.md
Get-Content G:\Dx\t3code\docs\user\guest-links.md

# dxbot side
Get-Content G:\Dx\bot\src\features\t3code\native\index.ts
Get-Content G:\Dx\bot\src-tauri\src\t3code_host.rs
Select-String G:\Dx\bot\src\features\dx\components\chat\right-pane.tsx -Pattern 'RIGHT_PANE_TABS' -Context 0,20
Get-Content G:\Dx\bot\src\features\terminal\core\transport.ts
Get-ChildItem G:\Dx\bot\src\features\providers
Get-Item G:\Dx\bot\src-tauri\src\host_vm.rs
```

---

## 9. What this file does not claim

- It does not claim T3 Code's features work on this machine; they are read
  from the tree, not run.
- It does not claim dxbot's rows are release-qualified; most are repo-managed
  and the hypervisor guest has never executed.
- It does not treat "has a package" or "has a framed route" as "ships a usable
  feature". Framed routes are called out separately, by name.

MIT OR Apache-2.0
updated: 2026-10-31 00:00 UTC
