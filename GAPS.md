# GAPS.md - dxbot (Tauri) against T3 Code (Electron), measured 2026-10-31

**What this file is.** A fresh, deliberately unkind comparison between
`G:\Dx\bot` (dxbot, Tauri desktop) and `G:\Dx\t3code` (T3 Code, Electron
desktop + web + server + mobile), written after syncing the T3 Code clone to
upstream. It answers one question only: *what does T3 Code do better, and what
would it take to close each gap?* It is not a feature inventory of dxbot; the
"what we have that they do not" section exists so the document is not a
one-sided advert.

**Claim tiers.** Every row here is **repo-managed**: a path in one of the two
trees, named so it can be opened. Nothing was built, launched or benchmarked
for this document. No row is `executed`, `runtime` or `release-qualified`.

**Provenance.**

| Fact | Value |
| --- | --- |
| T3 Code clone | `G:\Dx\t3code`, remote `pingdotgg/t3code` |
| T3 Code synced to | `6fc9144a` (`origin/main`, 2026-10-31; 31 commits pulled) |
| Local T3 Code change | `pnpm-lock.yaml` was stashed before the fast-forward (`git stash list`, message "local pnpm-lock before sync"); `TAURI.md` is untracked and was left alone |
| dxbot measured at | `be20e485` on `main` (= `bot/main`) |
| T3 Code licence | MIT, (c) 2026 T3 Tools Inc. |
| dxbot licence | AGPL-3.0 |

---

## 0. The brutal one-paragraph answer

T3 Code is the better **agent cockpit**: its terminal is a real VT engine
(Ghostty's, via `libghostty-vt` WebAssembly and native bindings), its right
panel is a composable surface system with Browser, Terminal, Files, Diff,
Pull request, Linked pull requests and Device tabs, its browser preview has a
CDP relay, passkeys, session recording, element picking and annotations, and
its Windows story runs agents inside WSL with a shipped Linux CLI archive.
dxbot has the larger product (its own agent engine, voice, dictation, grammar,
memory, plan, dashboard, 14-tab right pane) and a genuinely good terminal with
split panes, but T3 Code wins on **reach, terminal fidelity, preview depth,
source-control depth and multi-client sharing**. dxbot's "VM" is an embedded
Linux guest that has never booted; T3 Code's "VM" is WSL plus a dev container,
which ships today.

---

## 1. The two products, side by side

| | T3 Code | dxbot |
| --- | --- | --- |
| Shape | Agent harness control surface for *other* CLIs | Local-first AI workspace with its own Rust harness |
| Desktop shell | Electron | Tauri 2 |
| Clients | Web, Electron desktop, **iOS and Android** | Tauri desktop; Android scaffold; no iOS project |
| Backend | `apps/server` Node CLI, outlives every client | `server/` Rust crate, port 3773 |
| Terminal engine | `libghostty-vt` (C ABI, WASM in web, native on mobile) | xterm.js 6 + host-op PTY |
| Right panel | 7 tab types, inline or sheet, resize per frame | 14 tabs, one column width, desktop/mobile sheet |
| Provider layer | 12 packages (`provider-*`, `effect-*`) | 6 drivers under `src/features/providers` |
| Source control | 9 packages (GitHub, GitLab, Bitbucket, Azure DevOps, Forgejo, GitCafe, core, definitions, testing) | GitHub PR sections + GitLab/Bitbucket/Gitea/Azure DevOps clients under `src/features/integrations` |
| Remote reach | Clerk identity + Cloudflare relay + managed tunnels + push | LAN / tailnet / SSH only |
| Install channels | npx, install scripts, winget, scoop, brew, deb, AUR, GitHub Releases | GitHub Releases (macOS + Windows) |

---

## 2. Gaps, ranked by impact

Severity is about what a user feels, not how hard the fix is. Evidence is the
T3 Code path, then our nearest path.

### G1 - Terminal fidelity and sharing (HIGH)

**What they have.**

- The terminal is not xterm. It is the same `libghostty-vt` C ABI that Ghostty
  uses, compiled to WASM for the web and bound natively on mobile:
  `apps/web/src/terminal/ghostty/README.md` (renderer batches backgrounds and
  style runs into a Canvas 2D frame; IME, selection, scrolling, links and
  cursor blinking live in `surface.ts`). A Nerd Font is vendored for prompt
  glyphs.
- PTYs are **server-owned** and outlive the renderer. The environment server
  owns session lifetime and retained output; every client attaches through the
  environment connection, so a session can be reconnected to, or shared by two
  clients: `docs/internals/terminal-runtime.md`.
- Retention is bounded and specified: 5,000 lines and 8 MiB UTF-8 per terminal
  on the server, 512 KiB per client buffer, with query/response traffic stripped
  from replayed history so a restored scrollback cannot inject junk into a live
  shell (`docs/internals/terminal-runtime.md`).
- Mobile has a **native terminal view** for both platforms:
  `apps/mobile/modules/t3-terminal/android/src/main/java/expo/modules/t3terminal/TerminalCanvasView.kt`
  and `apps/mobile/modules/t3-terminal/ios/T3TerminalView.swift`.

**What we have.** A real PTY over host ops, session reuse across reloads, and
split panes with maximise that does not remount a terminal:
`src/features/terminal/core/transport.ts`, `src/features/dx/components/chat/terminal-workspace.tsx`.
The renderer is xterm.js (`@xterm/xterm ^6.0.0` in `package.json`), not a VT
engine; there is no native mobile terminal.

**The gap in one sentence.** Our terminal is a good xterm in a well-built
split layout; theirs is Ghostty's VT with server-shared scrollback and native
mobile surfaces.

**Closing it.** Two separable pieces. (a) A Ghostty VT adapter behind the
existing `terminal-view.tsx` surface: the WASM artefacts and the C ABI are
MIT/upstream, and our transport already speaks raw PTY bytes. (b) A
server-side scrollback owner so two clients can attach; today our output is
pulled per pane and the session is host-op scoped.

### G2 - The right panel is a surface system, ours is a tab list (HIGH)

**What they have.** `apps/web/src/components/RightPanelTabs.tsx:319-369`
declares the tab types: **Browser, Terminal, Files, Diff, Pull request, Linked
pull requests, Device**. Each is a *surface* with its own runtime tab id,
rename, mute, reorder, and add-browser-in-profile action; the panel renders
inline beside the chat or as a sheet on narrow layouts
(`apps/web/src/components/chat/ChatRightPanel.tsx`), and its width follows a
resize drag every frame.

**What we have.** Fourteen tabs in one list (`RIGHT_PANE_TABS` in
`src/features/dx/components/chat/right-pane.tsx:59`): menu, search, files,
terminal, browser, chat, review, preview, environment, dashboard, plan, memory,
usage, settings. It already has an inline/sheet split and a resize handle
(`right-pane.tsx:383-470`), so the shell is not the problem.

**The real gap.** Missing surfaces, not missing chrome: no **Pull request**
surface, no **Linked pull requests** surface, no **Device** surface (a live
mobile-device preview), and no per-surface runtime identity (rename, mute,
reorder, multiple browser surfaces in one profile). Our `review` tab is a
review/diff read-out, not a PR workspace.

**Closing it.** Add three surfaces to the existing tab registry:
`pr` (reuse `src/features/dx/components/agent/github-prs-section.tsx` plus the
integrations clients), `prLinked` (same data, link view), and `device` (needs
G5's mobile work first). Give browser surfaces a tab id per profile instead of
one browser tab.

### G3 - "VM": WSL and a dev container ship; our hypervisor guest has never booted (HIGH)

The premise needs correcting. T3 Code has **no embedded hypervisor**. What it
has:

- **WSL as a first-class runtime.** Windows builds embed a same-arch Linux CLI
  archive and run agents/projects inside a chosen WSL distro
  (`docs/user/install.md:114`, `docs/operations/release.md:494-515`,
  `apps/desktop/src/wsl/`).
- **A dev container** for the repo (Ubuntu 24.04, Node 26, pnpm, Rust, `vp`,
  `gh`) with named volumes: `docs/internals/devcontainer.md`.
- **Per-provider sandbox policy** handed to the CLI that owns it, rather than a
  sandbox of its own (`docs/user/cursor.md:62-66`,
  `docs/orchestration-v2/provider-capability-system.md:242`).

dxbot has the opposite shape: a real embedded Linux guest behind a `GuestExec`
trait in `src-tauri/src/host_vm.rs` (1,863 lines / 89 KB), with the WHP
partition probe and named refusals - but the measured status is **32 / 100
capability, ~5 / 100 verified-shipping, zero boots** (bot `AGENTS.md`, goal 9),
and there is no WSL runtime and no dev container.

**Verdict.** T3 Code's VM story is unglamorous and works; ours is ambitious and
has never run. If the goal is "agents in a Linux box on Windows", WSL is the
short path and is one release channel away; the WHP guest is a research track.

**Closing it.** P0: a WSL runtime backend (spawn the shipped Linux CLI archive,
map `terminal.*` and `fs.*` host ops to `wsl.exe`). Keep the WHP guest as a
separate track; do not claim it as the VM answer until a boot is measured.

### G4 - Browser preview depth (HIGH)

**What they have.** The desktop preview is a real browser host, not an iframe:
`apps/desktop/src/preview/` contains `CdpRelay` (agent drives the page over the
Chrome DevTools Protocol), `Passkeys` / `PasskeyBackend` (WebAuthn works),
`RecordingInput` (session recording), `PickPreload` (element picking feeds the
agent), `Annotation*`, `FaviconCapture`, `DesktopBrowserHost`, `Manager` and a
profile model. Settings expose browser profiles and import
(`packages/contracts/src/browserProfile.ts`, `browserImport.ts`).

**What we have.** A browser pane and a browser-view iframe path; the port
records the iframe limitation in our own gap history, and the per-origin policy
model exists (`src/features/browser/core/profiles.ts`). No CDP relay, no
passkeys, no recording, no element picker, no annotation layer.

**Closing it.** The Tauri equivalent is a child WebView plus a CDP bridge, not
an iframe. The existing browser tab and profile policy can host it; the work is
the host side, and it is the largest single UI gap after terminal.

### G5 - Reach: identity, relay, push, mobile (HIGH)

- **Identity:** Clerk across web/desktop/mobile vs our local `better-auth`.
- **Relay:** Cloudflare relay with DPoP bootstrap and webhook forwarding
  (`docs/internals/remote.md`) vs LAN/tailnet/SSH
  (`src-tauri/src/host_tailscale.rs`, `host_ssh.rs`, `src-tauri/src/remote/`).
- **Push:** APNs/FCM and Live Activities/widgets vs in-app toasts only.
- **Mobile:** shipped iOS and Android apps, including the native terminal
  (`apps/mobile/`), vs an Android scaffold in `src-tauri/gen/android` and no
  iOS project.
- **Install channels:** npx/scripts/winget/scoop/brew/deb/AUR/Releases vs
  Releases only.

This is the gap that makes T3 Code feel like a product and dxbot feel like a
desktop app.

### G6 - Provider ecosystem as packages (MEDIUM-HIGH)

T3 Code has `packages/provider-core`, `provider-acp`, `provider-acp-registry`,
`provider-cursor`, `provider-grok`, `provider-muse`, `provider-opencode`,
`provider-pi`, `provider-fx`, `provider-testing`, plus
`effect-codex-app-server` and `effect-acp`: twelve packages, each with its own
contracts and tests. dxbot has six drivers:
`src/features/providers/{claude,codex,cursor,grok,opencode,antigravity}-driver.ts`.

The difference is not count alone. Their provider layer is an installable
capability system (auth, install, update, model manifest, usage limits), and
the UI is generated from it (`docs/internals/adding-a-provider.md`). Ours is a
driver per CLI plus a hand-built picker.

### G7 - Source control and pull requests (MEDIUM-HIGH)

T3 Code ships nine source-control packages (GitHub, GitLab, Bitbucket, Azure
DevOps, Forgejo, GitCafe, core, definitions, testing) and a Pull request
surface with linked PRs and per-file revisions
(`packages/contracts/src/pullRequest.ts`, `threadPullRequest.ts`,
`docs/internals/pull-request-file-revisions.md`).

dxbot has real clients for GitHub, GitLab, Bitbucket, Gitea, Azure DevOps under
`src/features/integrations/core/` and a GitHub PR section
(`src/features/dx/components/agent/github-prs-section.tsx`), so the transport
layer is not the gap. The gap is the **surface**: no PR tab, no linked-PR
workflow from a thread, no file-revision view.

### G8 - Resource telemetry and diagnostics (MEDIUM)

T3 Code exposes per-agent CPU/memory to clients, including a Linux monitor for
the WSL backend (`docs/internals/resource-telemetry.md`,
`packages/contracts/src/resourceTelemetry.ts`). dxbot has wire names transcribed
and unserved; there is no producer.

### G9 - Scheduled tasks, updates and distribution polish (MEDIUM)

T3 Code has a scheduled-tasks settings route
(`apps/web/src/routes/_chat.settings.scheduled-tasks.tsx`), an "update
everything" flow (`apps/web/src/components/sidebar/useUpdateEverything.ts`),
server-update status in the composer, and eight install channels. dxbot has
durable jobs in the server (`server/src/durable_jobs.rs`) but no scheduled-task
surface of the same completeness, and one install channel.

### G10 - Contracts, docs and tests as a product (MEDIUM, compounding)

T3 Code: 107 contract modules under `packages/contracts/src`, shared by every
client, each with tests; 85 docs across user/internals/operations/
orchestration-v2; 5,444 test files. dxbot: five ported contract files, six docs
under `docs/`, and 582 TS test files plus Rust tests. Their contract package is
why web, desktop and mobile stay in step; ours is a port, and the port has
already drifted once (the orchestration contract split).

---

## 3. What dxbot has that T3 Code does not

Not a consolation prize - these are real, and two of them are product-defining:

| Capability | dxbot evidence | T3 Code |
| --- | --- | --- |
| Its own agent engine (no external CLI required) | `crates/`, in-process harness behind Tauri | none; requires one of the CLIs |
| Voice: wake word, local ASR, TTS, dictation modes | `src-tauri/src/recorder.rs`, `transcribe.rs`, `local_transcription.rs`, `src/features/pill` | voice input exists, no local ASR/TTS stack of this depth |
| Grammarly-style grammar with Harper, system-wide overlay | `src-tauri/src/grammar.rs`, `grammar_overlay*.rs`, 55/100 capability measured | not a feature |
| Pill/overlay window UX, toasts, tray, OS integration | `src-tauri/src/pill.rs`, `toast.rs`, `tray.rs` | Electron window chrome only |
| Scratchpad, canvas, research, memory, plan, dashboard, environment tabs | 14 right-pane tabs, `right-pane.tsx:59` | 7 tab types |
| Usage/token accounting surfaces | `token-usage.ts`, `token-usage-panel.tsx`, `context-usage.tsx` | usage exists, less granular |
| Local-first storage and offline path | Rust storage, local mirror | server + cloud identity first |

The honest summary: dxbot is a **workspace** with an agent in it; T3 Code is an
**agent cockpit** that is better at being a cockpit.

---

## 4. Right panel, tab by tab

| Surface | T3 Code | dxbot | Gap |
| --- | --- | --- | --- |
| Browser | yes, multi-surface per profile, CDP, annotations, passkeys | partial: one browser tab, iframe-level | HIGH (G4) |
| Terminal | yes, Ghostty VT, server-shared | yes, xterm + splits, host-op PTY | MEDIUM (G1) |
| Files | yes | yes | parity |
| Diff | yes | yes (`review`, diff parts) | parity |
| Pull request | yes | no tab; clients + a GitHub section exist | HIGH (G7) |
| Linked pull requests | yes | no | HIGH |
| Device (mobile preview) | yes | no | HIGH (G5) |
| Search | yes (route-level) | yes | parity |
| Usage / telemetry | usage + resource telemetry | usage yes; telemetry no | MEDIUM (G8) |
| Canvas / mini player | yes (`ChatCanvas.tsx`) | canvas exists elsewhere | parity-ish |
| Plan / memory / environment / dashboard | no equivalents as tabs | yes | we lead |

---

## 5. Terminal, feature by feature

| Property | T3 Code | dxbot |
| --- | --- | --- |
| VT engine | `libghostty-vt` C ABI (WASM + native) | xterm.js 6 |
| Renderer | Canvas 2D batches, React out of the loop | xterm canvas/DOM |
| PTY owner | environment server, shared by clients | host op, per pane |
| Reconnect / share | yes, attach from any client | session reuse on reload |
| Scrollback bound | 5,000 lines / 8 MiB server, 512 KiB client | 2 MB replay in the pane |
| Replay safety | strips query/response traffic | not specified in our tree |
| Splits / maximise | yes (client layouts) | yes, no remount on split |
| Mobile terminal | native Kotlin + Swift views | none |
| Theming | terminal appearance settings | theme pairs in `features/terminal/core/theme.ts` |

---

## 6. "VM" comparison, stated plainly

| | T3 Code | dxbot |
| --- | --- | --- |
| Windows Linux runtime | WSL distro, shipped Linux CLI archive | none |
| Dev environment | `.devcontainer` + Codespaces | none |
| Agent sandbox | delegated to the provider CLI | none of our own |
| Embedded hypervisor guest | none | `host_vm.rs`, WHP probe, zero boots |
| User-facing VM surface | WSL settings + terminal | not rendered; `vm-guest.ts` imported by nothing |
| Measured status | ships in installers | 32/100 capability, ~5/100 shipping |

---

## 7. Prioritised plan

**P0 - close in weeks, high user impact**

1. **WSL runtime** (G3): spawn the existing Linux CLI under `wsl.exe`; map
   `terminal.*`, `fs.*`, `git.*` host ops. This is the shortest path to "agents
   in Linux on Windows" and does not depend on the hypervisor work.
2. **Pull request surface** (G7): promote `github-prs-section.tsx` and the
   integrations clients into a `pr` tab with linked PRs and file revisions.
3. **Terminal adapter** (G1a): put Ghostty VT behind `terminal-view.tsx`; keep
   xterm as the fallback until parity is measured.

**P1 - product reach**

4. **Browser preview host** (G4): Tauri child WebView + CDP bridge; passkeys,
   element picking, annotations, profiles.
5. **Relay + identity** (G5): a hosted path to the machine before mobile.
6. **Mobile**: finish Android, then iOS; the native terminal module is the
   reference shape.

**P2 - compounding**

7. **Provider packages** (G6): turn the six drivers into capability packages
   with auth/install/usage contracts the picker reads.
8. **Telemetry** (G8) and **contracts** (G10): one contract package shared by
   desktop and server, generated types, drift test.
9. **Distribution** (G9): winget/scoop/brew, `npx`-style bootstrap.

---

## 8. Re-run the measurement

```powershell
# T3 Code side
git -C G:\Dx\t3code fetch origin --prune
git -C G:\Dx\t3code rev-list --left-right --count main...origin/main
Get-ChildItem G:\Dx\t3code\packages -Directory | Measure-Object
Get-ChildItem G:\Dx\t3code\apps

# terminal / panel / preview evidence
Get-Content G:\Dx\t3code\docs\internals\terminal-runtime.md
Select-String G:\Dx\t3code\apps\web\src\components\RightPanelTabs.tsx -Pattern 'label:'
Get-ChildItem G:\Dx\t3code\apps\desktop\src\preview

# dxbot side
Get-Content G:\Dx\bot\src\features\dx\components\chat\right-pane.tsx |
  Select-String 'RIGHT_PANE_TABS' -Context 0,20
Get-Content G:\Dx\bot\src\features\terminal\core\transport.ts
Get-ChildItem G:\Dx\bot\src\features\providers
Get-Item G:\Dx\bot\src-tauri\src\host_vm.rs
```

---

## 9. What this file does not claim

- It does not claim T3 Code's features work on this machine; they are read from
  the tree, not run.
- It does not claim dxbot's rows are release-qualified; most are
  repo-managed and some (the hypervisor guest) have never executed.
- It does not treat "has a package" as "ships a usable feature". Where a T3
  Code capability is contract-only, the row says so.

MIT OR Apache-2.0
updated: 2026-10-31 00:00 UTC
