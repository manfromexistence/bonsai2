# T3CODE-MERGE-PLAN.md - what dxbot has, what T3 Code has, and what can be copied

**Purpose.** A merge plan, not an implementation. It inventories both trees,
separates T3 Code code that can be copied from code that must be rewritten for
Tauri/Rust, and gives a wave order with exact source paths and destination
paths. It is the companion to `GAPS.md` (the 65-gap measurement).

**Provenance.**

| Fact | Value |
| --- | --- |
| T3 Code source | `G:\Dx\t3code`, `6fc9144a` (2026-10-31) |
| T3 Code licence | MIT, (c) 2026 T3 Tools Inc. |
| dxbot source | `G:\Dx\bot`, `be20e485` |
| dxbot licence | AGPL-3.0 |
| Rule | MIT code may be merged into AGPL; keep the copyright and permission notice, and record the upstream commit in a PROVENANCE file beside the copy |

**The one-line answer.** The **UI** is largely copyable; the **runtime** is not.
T3 Code's visual layer is React + Base UI + Tailwind + zustand + Effect, and
dxbot already has React 19, Base UI, Tailwind 4, zustand, Tiptap, dnd-kit,
pdf.js and lucide. T3 Code's server, Electron main process, provider packages
and mobile native modules are Node/Electron/Effect or Kotlin/Swift and must be
rewritten for Tauri/Rust - copying them would rebuild the exact problem Tauri
was chosen to avoid.

---

## 1. What dxbot already has

Measured from the tree, not from memory.

| Area | dxbot owner | State |
| --- | --- | --- |
| Desktop shell | Tauri 2 (`src-tauri/`) | Windows + macOS builds; tray, pill, toast windows |
| Chat shell | `src/features/dx/components/chat/chat-shell.tsx` | streaming, queue, approvals, usage, checkpoints, 14 right-pane tabs |
| Composer | `chat-input.tsx` + Plate (`plate-composer.tsx`) | context chips, slash commands, skills, recall, stash, attachments |
| Transcript | `beui-transcript.tsx`, `message-list.tsx`, `message-parts.tsx` | markdown, citations, selection toolbar, diff parts, media |
| Right pane | `right-pane.tsx` (14 tabs) | search, files, terminal, browser, review, preview, environment, dashboard, plan, memory, usage, settings |
| Terminal | `terminal-workspace.tsx`, `terminal-view.tsx`, `features/terminal/core/*` | xterm 6, split panes, host-op PTY, session reuse |
| Browser | `features/browser/*` | browser tab + per-origin policy |
| Files | `files-panel.tsx` | tree, search, editor hooks |
| Diff/review | `review-part-view.tsx`, `diff-review.tsx`, `github-prs-section.tsx` | turn diffs, review parts, GitHub PR section |
| Settings | `features/settings/registry.ts` | 7 dialog tabs + 25 route tabs |
| Voice | `features/pill`, `src-tauri/src/{recorder,transcribe,local_transcription,tts}.rs` | wake word, local ASR, TTS, dictation |
| Grammar | `grammar.rs`, `grammar_overlay*.rs` | Harper engine + system-wide overlay |
| Harness | `crates/`, `server/` | own agent engine, checkpointing, durable jobs, settlement |
| Remote | `src-tauri/src/remote/`, `host_tailscale.rs`, `host_ssh.rs` | LAN, tailnet, SSH |

**What we do not have, and T3 Code does:** the list is `GAPS.md` (65 items).
The fusion-ready subset is the UI and the client-side logic; the rest is host
work.

---

## 2. The hard constraint: the UI is not standalone

Measured coupling in `apps/web/src`:

| Dependency | Files importing it |
| --- | --- |
| `@t3tools/contracts` | 940 |
| `@t3tools/client-runtime` | 510 |
| `effect` / `effect/*` | 424 |
| `@effect/atom-react` | 177 |
| `@tanstack/react-router` | 130 |
| `@pierre/diffs` / `@pierre/trees` | 42 |
| `@legendapp/list` | 16 |
| `@clerk/*` | 16 |

So "copy `components/chat`" is not a copy that compiles by itself. The chat
containers read Effect atoms, the T3 runtime, the T3 contracts and TanStack
Router. There are two honest strategies:

### Strategy A - T3 surface island (fastest visual parity, a second runtime)

Copy the T3 web app as an isolated build (its `apps/web/src`, the packages it
imports) and mount it in a Tauri window or child WebView; adapt its RPC to the
dxbot Rust server. The screens look and behave exactly like T3 Code because
they are T3 Code. Cost: a second frontend runtime, a duplicated design system,
and a compatibility layer that must track upstream. It is the fastest demo,
not the cleanest merge.

### Strategy B - incremental native merge (recommended)

Copy, in this order: the UI kit, the pure logic modules, then surface by
surface. Each surface is adapted to dxbot stores and the dxbot host/server.
No second runtime; the T3 look arrives one screen at a time. Slower to first
screenshot, cheaper forever.

**Recommendation: B for the product, A only as a temporary preview if a
screenshot is needed this week.** The rest of this plan is B.

---

## 3. Copyability tiers

### Tier 0 - copy nearly verbatim

Pure presentational or pure logic; no Electron, no Node, no T3 runtime.

| Source | Files | Notes |
| --- | --- | --- |
| `apps/web/src/components/ui/**` | 55 | Base UI + Tailwind primitives; dxbot already has `@base-ui/react` |
| `apps/web/src/index.css` | 1 | theme tokens; scope under a `.t3-surface` root first |
| `apps/web/src/appearanceContrast.ts`, `appearanceFonts.ts`, `themePalette.ts`, `vscodeThemeImport.ts`, `openVsxThemes.ts` | 5 | theme parity, theme import |
| `apps/web/src/components/{MorphIcon,Icons,JetBrainsIcons,RelativeTime,AnimatedHeight,AnimatedWidth}.tsx` | 6 | small visual primitives |
| `apps/web/src/components/RightPanelSheet.tsx`, `RightPanelTabs.tsx` | 2 | panel chrome, props-driven |
| `apps/web/src/terminal/ghostty/**` | 16 | Ghostty VT adapter + WASM + Nerd Font (see terminal section) |
| `packages/contracts/src/**` | 103 (74 non-test) | effect Schema; add `effect` dependency or transcribe types |
| `packages/shared/src/{promptLinks,usageLimits,threadPullRequests,composerTrigger,...}.ts` | selective | pure helpers; see manifest |
| `apps/web/src/*.ts` pure modules (composer, markdown, diff, keybindings, theme, stores) | ~120 | each has co-located tests; copy test with code |

### Tier 1 - copy plus an adapter

Stateful containers. They compile once the adapter layer exists (section 5).

| Source | Files | Adapter needed |
| --- | --- | --- |
| `apps/web/src/components/chat/**` | 327 | Effect atoms -> zustand/react-query bridge; client-runtime -> dxbot store; router shims |
| `apps/web/src/components/diffs/**` + `DiffPanel*.tsx` | ~20 | `@pierre/diffs`, worker pool |
| `apps/web/src/components/files/**` | 101 | `@pierre/trees`, file host ops |
| `apps/web/src/components/pullRequest/**` | 106 | source-control RPC |
| `apps/web/src/components/preview/**` | 41 | Tauri child WebView + preview RPC |
| `apps/web/src/components/device/**` | 34 | Rust device service |
| `apps/web/src/components/usage/**` | 20 | usage RPC |
| `apps/web/src/components/sidebar/**` | 22 | thread state + router shims |
| `apps/web/src/components/{CommandPalette*,Sidebar.tsx,ChatView.tsx,...}` | ~40 | store + router |
| `apps/web/src/components/{guest,media,permissions,promptLinks}/**` | 23 | small RPC surfaces |
| `apps/web/src/hooks/**`, `apps/web/src/lib/**`, `apps/web/src/state/**` | 259 | selective; many are generic |

### Tier 2 - rewrite for Tauri/Rust (do not copy)

| Source | Why |
| --- | --- |
| `apps/desktop/src/**` (Electron main, preload, IPC, window, menus, updates, permissions, snapShot, WSL, SSH, preview host) | Electron APIs; Tauri equivalents are Rust commands/plugins |
| `apps/server/src/**` (40 domains) | Node + Effect + PTY/browser/relay services; rewrite as Rust modules behind the existing `server/` or `src-tauri/` |
| `packages/provider-*/**` | Effect services and Node install/auth/update logic; port data and contracts, not code |
| `packages/source-control-*/**`, `secret-provider-*` | Node clients; Rust clients or existing `features/integrations` |
| `apps/mobile/**` native modules | Kotlin/Swift; use as reference for Tauri mobile plugins |
| `packages/effect-*`, `infra/`, `packaging/`, `third-party-licenses/spdx` | architecture/build; copy data only |

### Tier 3 - do not merge at all

Clerk hosted auth, Cloudflare relay infrastructure, hosted web app, app-store
release pipelines, vendor demo data. These are services and processes, not
files to merge.

---

## 4. Direct copy manifest (recommended first copies)

Destination convention: copied code lives under `src/features/t3ui/` (owned
and adapted), not `src/vendor/` (verbatim, never edited). Each top-level copy
gets a `PROVENANCE.md` naming `6fc9144a`, the licence and the adaptations.

### 4.1 UI kit and theme (Wave 1)

| From | To | Action |
| --- | --- | --- |
| `apps/web/src/components/ui/**` | `src/features/t3ui/ui/**` | copy; rewrite `~/components/ui` imports |
| `apps/web/src/index.css` | `src/features/t3ui/theme/t3.css` | copy; scope tokens under `.t3-surface` |
| `appearance*.ts`, `themePalette.ts`, `vscodeThemeImport.ts`, `openVsxThemes.ts` | `src/features/t3ui/theme/` | copy + tests |
| `components/MorphIcon.tsx`, `Icons.tsx`, `JetBrainsIcons.tsx`, `RelativeTime.tsx`, `AnimatedHeight.tsx`, `AnimatedWidth.tsx` | `src/features/t3ui/primitives/` | copy |

The kit needs `morphicons` and `@pierre/*` icons only where used; check each
file before adding a dependency.

### 4.2 Chat surface (Wave 2)

| From | To | Action |
| --- | --- | --- |
| `apps/web/src/components/chat/Composer*.tsx`, `composer*.ts` | `src/features/t3ui/composer/` | copy; replace T3 atoms with dxbot composer store |
| `components/chat/ComposerPromptEditorTiptap.tsx` | `src/features/t3ui/composer/` | keep Tiptap; dxbot already has `@tiptap/react` 3 |
| `components/ChatMarkdown.tsx`, `markdown-*.ts` | `src/features/t3ui/markdown/` | copy; permissions tests included |
| `components/chat/AssistantCitation*.tsx`, `AssistantSelectionToolbar.tsx` | `src/features/t3ui/citations/` | copy; map to dxbot message parts |
| `components/chat/Message*`, `TimelineOverlays`, `ChatHeader`, `ChatBanner*` | `src/features/t3ui/chat/` | copy leaves first; containers after store bridge |
| `components/chat/ChatCanvas.tsx`, `canvasStore.ts` | `src/features/t3ui/canvas/` | copy if canvas parity is wanted |

### 4.3 Diffs, files, pull requests (Wave 3)

| From | To | Action |
| --- | --- | --- |
| `components/diffs/**` | `src/features/t3ui/diffs/` | copy; add `@pierre/diffs` |
| `DiffPanel.tsx`, `DiffPanelShell.tsx`, `DiffWorkerPoolProvider.tsx`, `ChangedFilesTree.tsx` | `src/features/t3ui/diffs/` | copy; add worker pool |
| `components/files/**` | `src/features/t3ui/files/` | copy; add `@pierre/trees`; map file ops to `host_git`/`fs.*` |
| `components/pullRequest/**` | `src/features/t3ui/pullrequest/` | copy; map to existing integrations clients |
| `components/usage/**` | `src/features/t3ui/usage/` | copy; map to `token-usage.ts` |
| `components/sidebar/**` + `Sidebar*.ts(x)` | `src/features/t3ui/sidebar/` | copy; router shims |

### 4.4 Terminal (Wave 4)

| From | To | Action |
| --- | --- | --- |
| `apps/web/src/terminal/ghostty/**` | `src/features/t3ui/terminal/ghostty/` | copy adapter + `vendor/ghostty-vt.wasm` + `ghostty-write-pty.wasm` + font |
| `native/libghostty-vt/VERSION` | `src/features/t3ui/terminal/PROVENANCE.md` | record the pin |
| `apps/web/scripts/build-libghostty-wasm.sh` | `scripts/t3-ghostty/` | copy build script; rebuild is optional |

Keep dxbot's `terminal-workspace.tsx` splits; replace only `terminal-view.tsx`'s
renderer with the Ghostty surface. Transport stays the dxbot PTY host op.

### 4.5 Logic and stores (copy with tests, in parallel with the surface)

| From | To |
| --- | --- |
| `apps/web/src/composer-*.ts` | `src/features/t3ui/logic/composer/` |
| `apps/web/src/markdown-*.ts` | `src/features/t3ui/logic/markdown/` |
| `apps/web/src/diff*.ts`, `diffPanelStore.ts` | `src/features/t3ui/logic/diffs/` |
| `apps/web/src/keybindings.ts`, `shortcutModifierState.ts` | `src/features/t3ui/logic/keys/` |
| `apps/web/src/pendingUserInput.ts`, `questionAttachments.ts` | `src/features/t3ui/logic/questions/` |
| `apps/web/src/promptStashStore.ts`, `composerDraftStore.ts` | `src/features/t3ui/logic/drafts/` |
| `apps/web/src/rightPanelStore.ts`, `rightPanelLayout.ts` | `src/features/t3ui/logic/panel/` |
| `apps/web/src/threadNotifications.ts`, `reopenClosedView.ts`, `closedViewStore.ts` | `src/features/t3ui/logic/threads/` |
| `apps/web/src/projectScripts.ts`, `projectIdentity.ts` | `src/features/t3ui/logic/projects/` |
| `apps/web/src/theme*.ts`, `vscodeThemeImport.ts` | `src/features/t3ui/logic/theme/` |

### 4.6 Contracts (foundation for everything)

| From | To | Action |
| --- | --- | --- |
| `packages/contracts/src/**` | `src/features/t3ui/contracts/` | copy; add `effect` dependency; keep tests |
| `packages/shared/src/{promptLinks,usageLimits,threadPullRequests,composerTrigger,...}.ts` | `src/features/t3ui/shared/` | copy the pure helpers only |
| `packages/client-runtime/src/{handoff,load-balancing,linkRequest,changeRequestLinks,diffCount,filePreview,mediaActions,clock,composerThreadItems}.ts` | `src/features/t3ui/runtime/` | copy these pure modules first; the rest is state machinery |

Do **not** copy `packages/client-runtime` wholesale on day one: it depends on
`provider-*`, `source-control-*` and `three`.

---

## 5. The adapter layer to build

| T3 dependency | dxbot replacement | Files |
| --- | --- | --- |
| TanStack Router (`Link`, `useNavigate`, `useParams`, `useSearch`) | Next navigation + a `t3ui/router.tsx` shim exposing the same hooks | one shim file; 130 import sites |
| `@effect/atom-react` atoms | dxbot zustand stores + `useT3Atom` shim; or add `@effect/atom-react` and a runtime provider | one bridge per store |
| `@t3tools/client-runtime` environment/RPC | `src/lib/agent/host.ts` + `server/src/rpc.rs`; map environment/project/thread ids | transport module |
| `@clerk/*` | `better-auth` session + device pairing | auth adapter |
| Electron `window`, `clipboard`, `dialog`, `shell`, IPC | Tauri commands/plugins | `src-tauri/src/` commands |
| Electron preview host | Tauri child WebView + CDP bridge | new Rust module |
| Electron snapShot | Rust screen capture + annotation UI | new Rust module |
| Electron auto-update/permissions/menus | Tauri updater, `platform/macos`, tray | existing + new commands |
| T3 Node server (`terminal`, `preview`, `device`, `mcp`, `htmlRender`, `peer`, `relay`) | Rust modules in `server/` or `src-tauri/` | rewrite per `GAPS.md` G2/G26/G27/G28/G29/G42 |
| Ghostty terminal | keep Ghostty renderer; adapt PTY bytes from `terminal.*` host ops | `terminal-view.tsx` |
| T3 CSS variables | scope under `.t3-surface`; map to dxbot theme tokens | theme bridge |

---

## 6. Gap to copy-source map (short form)

| Gap | Copy these | Rewrite these |
| --- | --- | --- |
| G1 framed surfaces | all Tier 1 surfaces | remove `T3_NATIVE_ROUTES` entries as each lands |
| G2 terminal | `terminal/ghostty/**` | server-owned scrollback in Rust |
| G4 right panel | `RightPanelTabs`, `RightPanelSheet`, tab leaves | panel state adapter |
| G5 project/branch | `BranchToolbar*`, `WorktreeBaseBranchPicker`, `ProjectScriptsControl` | worktree RPC |
| G6 notifications | `BackgroundThreadAnnouncer`, `ThreadNotificationCoordinator` | push/APNs/FCM |
| G7 sidebar | `Sidebar*`, `ThreadUndoNotice`, `ThreadHoverCard`, `CustomSnoozeDialog` | thread state adapter |
| G8 palette | `CommandPalette*`, `commandPaletteStore` | command registry |
| G9 links | `components/promptLinks/**`, `components/guest/**` | guest share service in Rust |
| G11 snapShot | settings route UI only | capture/annotation in Rust |
| G12 diagnostics | `ConnectionStatusDot`, freeze/slow-RPC toasts, error boundary | diagnostics RPC |
| G16 PR | `components/pullRequest/**` | PR/source-control RPC |
| G26 visual replies | `components/guest/**` | `htmlRender` + headless Chrome in Rust |
| G27 device | `components/device/**` | device service in Rust |
| G33 composer | `Composer*`, `composer-*.ts` | store bridge |
| G34 rendering | `ChatMarkdown`, citation components | permission policy |
| G35 diffs | `components/diffs/**`, `DiffWorkerPoolProvider` | worker host |
| G47 files | `components/files/**` | file ops |
| G53 client runtime | pure modules listed above | connection transport |
| G60 source control | `sourceControlPresentation.tsx`, diff source picker | commit/restore RPC |
| G64 protocol | `packages/contracts/src/rpc.ts` as the spec | Rust version handshake |

---

## 7. Waves and rough effort

Estimates are agent-days for one agent with review, not a promise.

| Wave | Work | Output | Estimate |
| --- | --- | --- | --- |
| W0 | vendor dirs, PROVENANCE, deps (`effect`, `@pierre/diffs`, `@pierre/trees`, `@legendapp/list`, `morphicons`), router/atom/RPC shims, `.t3-surface` theme | copied kit compiles in a blank route | 5-8 |
| W1 | UI kit, theme, RightPanelTabs/Sheet, Sidebar shell, CommandPalette shell | T3-looking shell with dxbot data | 8-12 |
| W2 | Composer (Tiptap), ChatMarkdown, citations, message/timeline leaves | T3-looking transcript + composer | 12-18 |
| W3 | Diffs, worker pool, files, PR, usage surfaces | T3-looking review panels | 15-20 |
| W4 | Ghostty terminal adapter behind existing PTY | T3 terminal rendering | 5-8 |
| W5 | Preview service + device panel + visual replies (Rust host) | live browser/device/HTML replies | 20-30 |
| W6 | Provider breadth, protocol versioning, MCP server/OAuth, relay/peer | reach + interoperability | 30-50 |
| W7 | Mobile surface set and native modules | mobile parity | 30-50 |

**What not to attempt in one go:** copying `apps/server` or `apps/desktop` and
running Node under Tauri. That reintroduces the Electron/Node weight on mobile
that Tauri was chosen to remove.

---

## 8. Licensing and provenance rules

1. Every copied directory gets `PROVENANCE.md`: upstream repo, commit
   `6fc9144a`, licence MIT, source paths, destination paths, adaptations.
2. Keep the MIT copyright and permission text in the app's third-party notices.
3. Do not copy T3 Code's trademarks, logos or app-store assets without the
   same review; code licence and brand licence are different questions.
4. Do not copy `third-party-licenses/spdx` as ours; generate notices from the
   merged dependency graph instead.
5. Record every adapter in the same PROVENANCE file. A copied file with an
   unrecorded rewrite is a provenance defect.

---

## 9. Risks

| Risk | Mitigation |
| --- | --- |
| Copied containers do not compile without Effect/T3 runtime | Start with Tier 0/leaf components; add `effect` only when a surface needs it |
| Two design systems (dxbot `@dx/ui` + T3 UI) | Scope `.t3-surface`; migrate one surface at a time; delete the old surface when replaced |
| Upstream drift | Pin the commit; keep copied tests; re-sync on a schedule; never edit upstream copy without recording it |
| Framed routes mistaken for ports | Each wave deletes its `T3_ROUTES` entry; `T3_NATIVE_ROUTES` grows by real components |
| Router mismatch (TanStack vs Next) | One shim, not 130 edits |
| Effect weight in the client | Copy pure modules first; use zustand bridge for containers until Effect is justified |
| Licence/trademark | MIT code yes; brand assets and hosted services separately reviewed |
| Copying server code | Do not; rewrite in Rust per G2/G12/G26/G27/G28/G29/G42/G59/G64 |

---

## 10. Immediate next ten commits (suggested)

1. `chore(t3ui): add PROVENANCE and copy the UI kit + theme tokens`
2. `chore(t3ui): add router, atom and RPC shims; mount one blank route`
3. `feat(shell): replace sidebar chrome with T3 sidebar leaves`
4. `feat(panel): replace right-pane chrome with RightPanelTabs/Sheet`
5. `feat(command): add the T3 command palette shell over dxbot commands`
6. `feat(composer): copy Tiptap composer + pure composer logic`
7. `feat(markdown): copy ChatMarkdown + citation components`
8. `feat(diffs): copy DiffPanel and the worker pool; wire dxbot diffs`
9. `feat(terminal): Ghostty VT behind the existing PTY transport`
10. `feat(pr): copy the pull-request surface and wire the integrations clients`

Each commit deletes one framed `/t3code/*` route or leaves it explicitly
labelled as framed.

---

## 11. What this plan does not claim

- It does not claim any copied file compiles today; the dependency counts are
  measured, the adaptation is work.
- It does not claim T3 Code's server features come for free with the UI. The
  UI is the cheap half; the host services are the expensive half.
- It does not set a date. The effort column is a planning estimate.

MIT OR Apache-2.0
updated: 2026-10-31 00:00 UTC
