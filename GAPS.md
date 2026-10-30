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
CDP relay, passkeys, recording, element picking and annotations, agents answer
with sandboxed HTML pages verified by a headless browser, a live simulator can
sit beside a thread, its projects have branches, worktrees and scripts, and
its Windows story is WSL plus a dev container that ship. dxbot is the larger
product - own agent engine, voice, dictation, grammar, memory, plan,
dashboard, 14-tab right pane - but several of its T3-shaped surfaces are
**frames around a running T3 Code sidecar, not ports**: `T3_NATIVE_ROUTES` is
empty. The honest scoreboard is "wider product, shallower cockpit, and a
framed comparison surface that must not be counted as native capability".

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
| G26 | Visual replies (agent-published HTML pages) | HIGH |
| G27 | Device panel (live simulator/emulator) | HIGH |
| G28 | Outside agents and MCP interoperability | HIGH |
| G29 | Multi-environment peers and delegation | MEDIUM-HIGH |
| G30 | Browser profile import and default browser | MEDIUM-HIGH |
| G31 | Question answers with file attachments | MEDIUM |
| G32 | Settings inheritance and project-scoped UI | MEDIUM |
| G33 | Composer depth | MEDIUM-HIGH |
| G34 | Rendering, citations and selection | MEDIUM |
| G35 | Diff and review performance | MEDIUM |
| G36 | Provider-specific model options and history | MEDIUM |
| G37 | Onboarding, migration and project bootstrap | MEDIUM |
| G38 | Keyboard focus model and accessibility | MEDIUM |
| G39 | Appearance/theme distribution (we partly lead) | MEDIUM |

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

### G26 - Visual replies: agents answer with a sandboxed page (HIGH)

T3 Code lets an agent publish a self-contained HTML page above its written
reply: charts, tables, diagrams, collages, mockups. The page follows the
current theme, runs scripts in a sandbox, supports expand and zoom, embeds
local images so it survives the source files moving, and is deleted with the
thread. Before publishing, the agent verifies the page with screenshots from a
headless Chrome that T3 Code keeps for itself (about 120 MB, downloaded once):

- `apps/server/src/htmlRender/HtmlRender.ts`, `headlessChrome.ts`,
  `publicProxy.ts`.
- `apps/web/src/components/guest/GuestPage.tsx`, `GuestHtmlRender.tsx`.
- `docs/user/html-renders.md`.

dxbot has HTML renderers inside the showcase/router and Whirl, but no
agent-authored visual-reply surface, no per-thread page lifecycle, and no
headless-browser verification pass. Search for `htmlRender` finds only
showcase code.

Closing: a `visual` message part plus a sandboxed renderer in the transcript,
and a headless Chromium service for verification. The message-part schema
already supports typed parts.

### G27 - Device panel: live simulator/emulator beside a thread (HIGH)

T3 Code runs an iOS Simulator or Android Emulator in the right panel, exposes
it to the agent through `device_*` tools and an `agent-device` CLI, and lets
the user tap the same device while watching:

- `apps/server/src/device/` (`DeviceService`, `DeviceHubProxy`,
  `AgentDeviceShim`, `AgentDeviceTarget`, `DeviceActions`, `DeviceHost`).
- Right-panel `Device` tab (`RightPanelTabs.tsx:367`).
- `docs/user/devices.md`.

dxbot has browser device presets (`features/browser/core/devices.ts`) and an
Android build emulator helper (`features/distributed/core/emulator.ts`), but no
live device surface, no device tool namespace, and no device-sharing between
agent and user.

### G28 - Outside agents and MCP interoperability (HIGH)

T3 Code's environment exposes an MCP server so agents it did not start -
Claude Code or Codex in a terminal, ChatGPT, MCP-capable bots - can read
projects and threads, start and message threads, and list providers/models.
Each external agent signs in once and is scoped:

- `apps/server/src/mcp/`, `apps/server/src/mcpApps/`,
  `apps/server/src/peer/PeerMcpClient.ts`.
- `packages/contracts/src/threadMetadataMcp.ts`.
- `docs/user/outside-agents.md`.

dxbot has an MCP **client** (`src/features/acp/core/client.ts`,
`mcps-surface.tsx`), but no MCP server exposing dxbot's own threads, no MCP
apps host, and therefore no outside-agent interoperability.

### G29 - Multi-environment peers and delegation (MEDIUM-HIGH)

T3 Code models environments as peers that can forward work and delegate
remotely: `apps/server/src/peer/PeerForwarding.ts`, `RemoteDelegation.ts`,
`PeerLinks.ts`, `PeerLinkRequests.ts`, plus `apps/server/src/environment/`.
A thread can live on one machine while the client is on another.

dxbot has remote transports (`host_tailscale.rs`, `host_ssh.rs`,
`src-tauri/src/remote/`) but no peer link, forwarding or delegation contract; a
"remote" is a transport to one machine, not a set of collaborating
environments.

### G30 - Browser profiles import and default browser (MEDIUM-HIGH)

T3 Code imports cookies from another browser into a preview profile (OS keyring
unlock, one-time copy), and on macOS the app can be the system default browser,
opening links into a browser surface:

- `docs/user/browser-import.md`, `docs/user/default-browser.md`.
- Desktop `preview/BrowserSession.ts`, `WebviewPreferences.ts`, profile model in
  `packages/contracts/src/browserProfile.ts` / `browserImport.ts`.

dxbot has per-origin policy (`src/features/browser/core/profiles.ts`) but no
cookie import and no default-browser registration.

### G31 - Question answers with file attachments (MEDIUM)

T3 Code lets a user attach files or paste images into an agent question's
answer field, uploads them to the environment running the thread (including
remote environments), gives the agent paths, keeps per-question drafts, and
cleans attachments up when a thread is reverted or deleted
(`docs/user/question-attachments.md`, `apps/server/src/attachmentStore.ts`,
`attachmentPaths.ts`).

dxbot's question view (`src/features/dx/components/chat/question-part-view.tsx`)
tells the user to "attach files in the composer" - it does not own attachments
per question, and there is no server-side upload/cleanup contract for them.

### G32 - Settings inheritance and project-scoped overrides UI (MEDIUM)

T3 Code stores most settings on the server, lets a project override a defined
subset of keys, and every settings page has an "Applying settings for ..."
selector for project and environment (`docs/user/project-settings.md`,
`packages/contracts/src/settings.ts`).

dxbot has ported the layering core (`src/features/settings/settings-layers.ts`)
but its own header says the page wiring is still owed; the theme import,
server-published themes and WSL choice are also listed as not done. So the
model exists and no user can reach it.

### G33 - Composer depth (MEDIUM-HIGH)

T3 Code's composer is a product of its own:

- Tiptap rich mode plus a plain mode, with Markdown canonicalisation
  (`docs/internals/composer-editors.md`).
- 120,000-character limit, and a paste of 32 KiB or more becomes a text-file
  attachment instead of filling context (`docs/user/composer.md`).
- Context chips for files, pull requests, terminal excerpts, environments and
  threads; prompt history; a stash; slash-command search; code-block language
  picker; drag-to-mention; context undo; pending approval actions; pending
  user-input panel; plan follow-up banner; provider-state row; prompt-length
  validation.

dxbot has a Plate composer, prompt recall (`prompt-recall.ts`), draft stash
(`parkDraft` / `restoreStashed`), context chips (`context-chips.ts`), slash
commands and skills. Missing pieces: plain/rich toggle, hard length cap,
paste-to-attachment, code-block language picker, pending-approval actions, and
a per-question user-input panel.

### G34 - Rendering, citations and selection (MEDIUM)

T3 Code renders markdown under explicit permission rules
(`ChatMarkdown.permissions.browser.test.tsx`), citations with a comment editor
and source attribution (`AssistantCitationChip.tsx`,
`AssistantCitationCommentEditor.tsx`, `AssistantCitationSource.tsx`,
`CitationList`), a selection toolbar, generated-image markdown, media
components, and per-message actions.

dxbot has `ChatMarkdown`-equivalent rendering, `assistant-citation-chip.tsx`,
`assistant-selection-toolbar.tsx` and `response-media.tsx`. The gap is the
citation **comment editor** and the permissions test lane, not the basic chip.

### G35 - Diff and review performance (MEDIUM)

T3 Code runs diffs off the main thread and renders a changed-files tree:
`DiffWorkerPoolProvider.tsx`, `DiffPanel.tsx`, `DiffPanelShell.tsx`,
`ChangedFilesTree.tsx`, `DiffFilePathCopyButton.tsx`, pull-request file
revisions (`docs/internals/pull-request-file-revisions.md`).

dxbot has diff parts and a review surface, but no diff worker pool and no
changed-files tree equivalent; large diffs block the renderer. A search for
`new Worker` finds only showcase and vendor code.

### G36 - Provider-specific model options and history (MEDIUM)

T3 Code parses per-provider model options and imports existing CLI history:
`apps/server/src/claudeModelOptions.ts`, `codexModelOptions.ts`,
`claudeHistoryWorker.ts`, `claude-history-worker.ts`, plus provider packages
with their own contracts. The picker therefore reflects what each CLI actually
accepts, and old Claude sessions can be imported.

dxbot has a generic model catalogue (`models-config.ts`, `model-discovery.ts`)
and six drivers, but no per-provider option parser and no CLI-history import.

### G37 - Onboarding, migration and project bootstrap (MEDIUM)

T3 Code: a welcome wizard for new installs (`docs/user/welcome-wizard.md`), a
V1-to-V2 database copy and thread migration on first launch
(`docs/user/thread-migration.md`), a legacy-thread migration toast, project
clone flow, and `ProjectCloneToastCoordinator`.

dxbot has an onboarding screen and storage migrations in Rust, but no thread
schema migration from an older product, no legacy-migration surface, and no
project clone flow.

### G38 - Keyboard focus model and accessibility (MEDIUM)

T3 Code documents and implements focus ownership: the palette keeps focus and
returns it to the composer; workspace focus commands; number shortcuts while a
palette or model picker is open; model shortcuts inside Settings; terminal
startup not stealing the composer:

- `docs/user/keyboard-focus.md`, `docs/user/keybindings.md`.
- `KeybindingsConfigWarning.tsx`, `KeybindingsUpdateToast.tsx`.

dxbot has a keybindings contract and shortcut arrays, but no focus-ownership
model, no keybinding page (per its own header), and no accessibility test lane.

### G39 - Appearance and theme distribution (MEDIUM, where we partly lead)

T3 Code ships per-device appearance settings, light/dark theme pairing, command
palette theme switching, and a theme-picker shortcut
(`docs/user/appearance.md`, `CommandPaletteThemeItems.tsx`).

dxbot's theme editor is deeper (custom CSS/font/colour presets,
`src/features/settings/theme/*`, `/themes` route), so this is not a gap in
capability; it is a gap in **reach**: our theme choice is not available from
the command palette, and themes are not shared/published across devices.



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

## 6.1 Server surface: breadth, not just file size

`apps/server/src` carries **40 directories**; `server/src` carries **one**
(`orchestration/`) beside 25 files. That shape is the architecture gap in one
line - T3 Code's server is a product with domain owners, ours is a core with
owners still to grow.

| Server domain | T3 Code | dxbot nearest |
| --- | --- | --- |
| Terminal | `terminal/` (PTY, retention) | host op in Tauri, no server owner |
| Preview / HTML render | `preview/`, `htmlRender/` | browser pane in the client |
| Pull request / review | `pullRequest/`, `review/`, `sourceControl/` | integrations clients in `src/features/integrations` |
| Triage | `triage/` | none |
| Scheduled tasks / scheduling | `scheduledTasks/`, `scheduling/` | `durable_jobs.rs` (jobs, no scheduler product) |
| MCP / MCP apps | `mcp/`, `mcpApps/` | client only in `src/features/acp` |
| Peer / relay / cloud | `peer/`, `relay/`, `cloud/` | `remote/`, Tailscale/SSH transports |
| Device | `device/` | none |
| Resource telemetry | `resourceTelemetry/` | none |
| Secrets | `secrets/` + provider packages | `host_secret.rs` / keychain |
| Usage / usage limits | `usage/` | `runtime_usage.rs`, budgets |
| Workspace / project / git / vcs | `workspace/`, `project/`, `git/`, `vcs/` | `worktree_setup.rs`, `host_git.rs` |
| Attachments | `attachmentStore.ts`, `attachmentPaths.ts` | composer attachments, no server store |
| Diagnostics / observability / telemetry | `diagnostics/`, `observability/`, `telemetry/` | `analytics.rs` |
| Guest share | `guest/` | none |



## 7. Prioritised plan

**P0 - weeks, high user impact**

1. **Port one T3 surface for real** (G1) and remove it from `T3_ROUTES`; stop
   counting framed routes as capability.
2. **WSL runtime** (G3): spawn the Linux CLI archive, map terminal/fs/git host
   ops; this is the shipping answer to "agents in Linux on Windows".
3. **Pull request surface** (G16) from the existing clients and GitHub section.
4. **Ghostty VT adapter** (G2) behind `terminal-view.tsx`, xterm as fallback.

**P1 - product reach**

5. Browser preview host (CDP, passkeys, recording, picker, annotations) and
   profile import (G4, G30).
6. Guest links and prompt links (G9); small surfaces with large product effect.
7. Relay + identity before mobile (G19); then Android, then iOS.
8. Project/branch/worktree workflow (G5), command palette (G8), thread
   lifecycle (G7), composer depth (G33).
9. Visual replies (G26) and the device panel (G27) - both are visible,
   demoable wins.
10. Outside agents through an MCP server (G28).

**P2 - compounding architecture**

11. Run graph, fork/lineage, provider handoff (G10, G24); peer delegation
    (G29).
12. Shared contract package and browser-test lane (G22); settings inheritance
    UI (G32) and keybinding page (G38).
13. Diagnostics, updates, secrets, MCP apps (G12-G15); provider-specific model
    options (G36).
14. Distribution channels (G23) and onboarding/migration (G37).

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
