# T3CODE-MERGE-PLAN.md - delta merge plan (revision 2)

**Why this revision exists.** Revision 1 proposed copying large parts of T3
Code's UI as if dxbot were empty. It is not. dxbot already has a chat shell, a
composer, a transcript, diffs, files, a terminal, a browser pane, a 14-tab
right panel, a command palette, settings, notifications-in-app, queue,
approvals, subagents and its own design system. This revision inventories
**what we already have first**, and then copies only what is genuinely absent.

**Provenance.**

| Fact | Value |
| --- | --- |
| T3 Code source | `G:\Dx\t3code`, `6fc9144a` (2026-10-31), MIT |
| dxbot source | `G:\Dx\bot`, `be20e485`, AGPL-3.0 |
| Companion | `GAPS.md` - 65 measured gaps, corrected where this revision found overclaims |

---

## 1. What dxbot already has (do not copy these)

Measured from the tree. "Parity" means the capability exists; it does not mean
pixel-identical UI.

| Capability | dxbot evidence | T3 equivalent | Verdict |
| --- | --- | --- | --- |
| Chat shell + streaming turns | `chat-shell.tsx`, `live-response.tsx`, `streaming-markdown-cache.ts` | `ChatView.tsx` | parity |
| Transcript rendering | `beui-transcript.tsx`, `message-list.tsx`, `message-parts.tsx`, `markdown.tsx` | `ChatMarkdown.tsx` | parity |
| Citations + selection | `assistant-citation-chip.tsx`, `assistant-selection-toolbar.tsx` | citation components | parity (no comment editor) |
| Composer | `plate-composer.tsx`, `chat-input.tsx`, `context-chips.ts`, `prompt-recall.ts`, `prompt-stash.ts` | Tiptap composer | parity |
| Queue | `prompt-queue-pane.tsx` (reorder, edit, delete, send-now) | queued-run tools | parity (no steer) |
| Approvals / questions | `question-part-view.tsx`, `elicitation-part-view.tsx`, `elicitation-form.ts` | pending approval/user input | parity |
| Subagents / fan-out | `agent-board.tsx`, `fan-out-desk.tsx`, `subagent-summary-row.tsx`, `kind: "subagent"` in `types/agent.ts` | run nodes / subagent threads | partial (in-turn only) |
| Right panel | `right-pane.tsx` - 14 tabs: menu, search, files, terminal, browser, chat, review, preview, environment, dashboard, plan, memory, usage, settings | 7 surfaces | parity+ (missing PR/device) |
| Terminal | `terminal-workspace.tsx`, `terminal-view.tsx`, xterm 6, split/maximise, PTY session reuse | Ghostty VT | parity (different engine) |
| Browser pane | `browser-view.tsx`, `browser/core/page-script.ts` - same-origin snapshot/action, navigate | CDP browser service | partial (no cross-origin/CDP) |
| Diffs / review | `diff-review.tsx`, `review-part-view.tsx`, `git-diff-panel.tsx`, `changed-files-card.tsx` | DiffPanel + worker pool | partial (no source picker/worker) |
| Files | `files-panel.tsx` (tree, editor, transfer), `binary-preview.tsx`, `document-read-view.tsx` | files + previews | partial (preview depth) |
| Preview tab | `preview-pane.tsx` (TipTap markdown read/edit) | visual HTML replies | partial (not agent HTML) |
| Usage / context | `token-usage.ts`, `token-usage-panel.tsx`, `context-usage.tsx`, `turn-stats.ts` | usage + telemetry | parity (usage) |
| Settings | `settings/registry.ts` - 7 dialog + 25 route tabs | 27 routes | parity in breadth |
| Command palette | `sidebar.tsx` "Search and Command Palette" + Quick Open | CommandPalette | partial (narrower actions) |
| Sidebar / threads | spaces, tabs, shelves, bookmarks, archive, unread (`thread-unread.ts`), attention memory (`attention.ts`) | sidebar + snooze/pin | partial (no hover/undo) |
| In-app notifications | attention memory, agent board, unread, OS-notification gate (`os-notification-gate.ts`, closed) | notification coordinator + push | partial (no OS/push) |
| Theme editor | `features/settings/theme/*`, `/themes` route | themes + command palette switch | dxbot leads |
| Voice / ASR / TTS | `recorder.rs`, `transcribe.rs`, `local_transcription.rs`, `tts.rs`, pill | voice input only | dxbot leads |
| Grammar | `grammar.rs`, `grammar_overlay*.rs` | none | dxbot leads |
| Harness | `crates/`, `server/` (checkpoints, jobs, settlement) | external CLIs | dxbot leads |
| PR basics | `github-prs-section.tsx`, `features/integrations/core/*` (GitHub, GitLab, Bitbucket, Gitea, Azure) | PR workspace | partial (no surface) |
| Canvas / notes / plan / memory / dashboard | right-pane tabs + features | some equivalents | dxbot leads in tabs |

**Rule that follows:** do not copy T3 Code's chat containers, composer,
markdown renderer, sidebar state, command registry, settings fields, theme
editor, terminal transport or browser pane. They would be second
implementations of working features.

---

## 2. What is genuinely missing (the delta)

Grouped by what a user feels. Full evidence is in `GAPS.md`.

### 2.1 Missing surfaces (copy T3 presentation, keep dxbot logic)

| Missing | dxbot state | Copy from |
| --- | --- | --- |
| Pull request workspace, linked PRs, per-file revisions, review comments | GitHub section + integrations clients; no tab | `apps/web/src/components/pullRequest/**`, `ThreadHoverCard` PR bits |
| Device panel (live iOS Simulator / Android Emulator) | none; only browser device presets | `apps/web/src/components/device/**` |
| Visual replies (agent-published sandboxed HTML) | preview tab renders markdown only | `apps/web/src/components/guest/**` |
| Diff source picker (turn / branch / commit), worker pool, changed-files tree | diff parts + review view, no picker/worker | `components/DiffPanel*.tsx`, `components/diffs/**`, `DiffWorkerPoolProvider`, `ChangedFilesTree` |
| File/document preview depth (PDF pages, audio, delimited tables, worker binary, browser document) | binary/document reads exist; previews shallow | selected leaves in `components/files/**` |
| Command palette breadth (projects, all threads, themes, triage, prompt links) | suggestions + recent tabs | `CommandPalette*`, small adapters |
| Sidebar polish (hover card, undo, reopen closed view, drag/reorder motion) | shelves/bookmarks/unread exist | `Sidebar*` leaves |
| T3 visual language (if wanted) | own design system | `components/ui/**`, `index.css`, theme modules |

### 2.2 Missing client logic (copy pure modules)

| Missing | Copy from |
| --- | --- |
| Prompt links | `packages/shared/src/promptLinks.ts`, `apps/web/src/components/promptLinks/**` |
| Guest links / read-only live follow | `components/guest/**` + Rust guest share service |
| Portable handoff + context compaction | `packages/client-runtime/src/handoff.ts`, `apps/server/src/context-handoffs` spec, `ContextHandoffServiceV2` design |
| Thread search | `packages/contracts/src/threadSearch.ts` (spec), server implementation in Rust |
| AI thread titles | `apps/server/src/textGeneration/*` as spec; use dxbot's provider |
| Model manifest + catalog validation | `apps/server/src/provider/model-manifest.json`, `docs/internals/model-manifest.md` |
| Usage limits / subscription usage | `packages/shared/src/usageLimits.ts`, provider usage limit JSON |
| Diff/edit helpers | `apps/web/src/diff*.ts`, `diffFileActions.ts`, `reviewCommentContext.ts` |
| Composer small deltas (plain mode, cap, paste-to-attachment, code-block language) | `composer-*.ts` logic, not the whole editor |
| Protocol versioning | `packages/contracts/src/rpc.ts` as the spec; Rust handshake |

### 2.3 Missing host/runtime work (rewrite in Rust, do not copy)

| Missing | T3 source to read, then rewrite |
| --- | --- |
| WSL runtime on Windows | `apps/desktop/src/wsl/**`, `docs/user/install.md` |
| Cross-origin browser + CDP + passkeys + recording + annotations | `apps/desktop/src/preview/**`, `apps/server/src/preview/**` |
| Device service + `device_*` tools | `apps/server/src/device/**` |
| HTML render service + headless Chrome verification | `apps/server/src/htmlRender/**` |
| MCP server, MCP apps, outside-agent OAuth | `apps/server/src/mcp/**`, `mcpApps/**`, `auth/McpOAuth.ts` |
| Peer environments, relay, cloud identity | `apps/server/src/{peer,relay,cloud}/**` |
| Stable server launcher, staged updates, rollback | `apps/server/src/serviceLauncher.ts`, `docs/internals/server-updates.md` |
| Outbox + run finalization + PR sync reactor | `apps/server/src/orchestration-v2/{EventSink,EffectWorker,RunFinalizationService,PullRequestSyncReactor}.ts` |
| Protocol version negotiation | `packages/contracts/src/rpc.ts`, overview protocol-2 notes |
| Provider install/update + capability probes | `packages/provider-*/**`, `provider/agentContextBudget.json`, `*CapabilitiesProbe.ts` |
| Push / Live Activities / mobile apps | `apps/mobile/**` (native reference), APNs/FCM work |
| Eval harness + perf lane + licence notices | `evals/**`, `docs/internals/performance-regressions.md`, `third-party-licenses/spdx` |

### 2.4 Small deltas inside existing dxbot surfaces

- Composer: plain/rich toggle, 120k cap, paste-to-attachment, code-block
  language picker, drag-to-mention. (Everything else exists.)
- Terminal: Ghostty renderer behind the existing PTY transport.
- Browser: cross-origin/CDP path, server-owned contexts.
- Files: PDF paging, audio, delimited tables, worker-backed binary preview.
- Queue: steer into a live run.
- Notifications: OS notification host op, badges, push.
- Diff: source picker + worker pool.
- Settings: wire the already-ported inheritance model and keybindings page.

---

## 3. What to copy, concretely

Destination: `src/features/t3ui/` for adapted copies (not `src/vendor/`, which
is verbatim and never edited). Every copied directory gets `PROVENANCE.md`
with `6fc9144a`, MIT notice and adaptations.

### Copy first (no equivalent in dxbot)

| Source | Files | Destination | Notes |
| --- | --- | --- | --- |
| `apps/web/src/components/pullRequest/**` | 106 | `t3ui/pullrequest/` | new surface; wire to `features/integrations` |
| `apps/web/src/components/device/**` | 34 | `t3ui/device/` | new surface; Rust service behind it |
| `apps/web/src/components/guest/**` | 6 | `t3ui/guest/` | visual replies + guest view |
| `components/DiffPanel.tsx`, `DiffPanelShell.tsx`, `DiffWorkerPoolProvider.tsx`, `ChangedFilesTree.tsx`, `components/diffs/**` | ~20 | `t3ui/diffs/` | worker pool + source picker |
| Selected `components/files/**` preview leaves (`DocumentPages`, `AudioPreview`, `DelimitedTablePreview`, `BinaryViewer`, `binaryPreview.worker`, `BrowserDocumentFrame`) | ~10 | `t3ui/files/preview/` | not the tree, not the panel |
| `apps/web/src/terminal/ghostty/**` + WASM + font | 16 | `t3ui/terminal/ghostty/` | replaces xterm renderer only |
| `components/ui/**` + `index.css` + theme modules (`themePalette`, `vscodeThemeImport`, `openVsxThemes`, `appearance*`) | ~61 | `t3ui/ui/`, `t3ui/theme/` | only if adopting T3 visual language |
| `RightPanelTabs.tsx`, `RightPanelSheet.tsx` | 2 | `t3ui/panel/` | optional chrome |
| `packages/shared/src/{promptLinks,usageLimits,threadPullRequests}.ts` | 3 | `t3ui/shared/` | pure helpers |
| `packages/client-runtime/src/{handoff,diffCount,filePreview,composerThreadItems}.ts` | 4 | `t3ui/runtime/` | pure, no provider deps |
| `packages/contracts/src/{rpc,threadSearch,pullRequest,device,usage}.ts` | 5 | `t3ui/contracts/` | types/spec for the missing surfaces, not all 74 |
| `provider/model-manifest.json` | 1 | `t3ui/provider/` | data, validated in Rust |

### Copy later, presentation only (optional T3 look)

`CommandPalette*.tsx` presentation, `Sidebar*` leaves, chat message/timeline
leaves, `ChatMarkdown.tsx` presentation. Keep dxbot's stores and data; do not
copy `ChatView.tsx`, `Composer*` containers, or the T3 sidebar state.

### Do not copy (already exists or wrong runtime)

| T3 source | Why |
| --- | --- |
| `components/chat/**` containers, composer, composer logic | dxbot has Plate composer, queue, chips, stash, approvals, subagents |
| `ChatMarkdown` logic beyond presentation | dxbot has markdown, citations, selection |
| `Sidebar.tsx` state, `Sidebar.snooze.ts`, thread stores | dxbot has shelves, bookmarks, unread, attention |
| command palette store/bus | dxbot has a palette; only presentation is wanted |
| settings pages/fields | dxbot has 7+25 settings tabs |
| theme editor | dxbot's is deeper |
| terminal transport/PTY, browser pane | dxbot has PTY host ops and a same-origin browser |
| `apps/server/**`, `apps/desktop/src/**` | Node/Electron; rewrite in Rust/Tauri |
| `packages/provider-*`, `source-control-*`, `secret-provider-*` | Node/Effect; port data/contracts, not code |
| `apps/mobile/**` native modules | Kotlin/Swift; reference for Tauri mobile plugins |
| Clerk, relay, cloud, hosted app, store pipelines | services/processes, not files |

---

## 4. Adapters needed (only for what is copied)

| Copied thing | Adapter |
| --- | --- |
| T3 UI kit + panel chrome | `.t3-surface` theme scope; map tokens to dxbot theme |
| DiffPanel + worker | `@pierre/diffs`; worker entry; dxbot diff data |
| Files preview leaves | dxbot `fs.*`/`host_git` file reads |
| PR surface | existing `features/integrations` clients; new PR RPC |
| Device surface | new Rust device service |
| Guest/visual replies | Rust `htmlRender` + sandboxed WebView |
| Ghostty terminal | dxbot PTY host ops; keep split layout |
| Prompt links / guest links | dxbot deep links + Rust share service |
| Contracts subset | Rust wire types; no `effect` dependency unless a copied file needs it |
| Command palette/sidebar presentation | dxbot stores, Next navigation shims |

This avoids the 424-file Effect dependency and the 510-file client-runtime
adoption that revision 1 implied.

---

## 5. Waves (reduced)

| Wave | Work | Estimate |
| --- | --- | --- |
| W0 | `t3ui/` skeleton + PROVENANCE + licence notices; theme scope if adopting T3 look | 2-3 |
| W1 | Pull request surface + diff source picker/worker + files preview leaves | 10-14 |
| W2 | Device panel, guest/visual replies, HTML render service (Rust) | 12-18 |
| W3 | Ghostty renderer behind existing PTY | 4-6 |
| W4 | Rust host work: WSL, cross-origin browser/CDP, MCP server/OAuth, peer/relay, server launcher, outbox/run finalization | 30-50 |
| W5 | Client deltas: prompt/guest links, protocol versioning, AI titles, manifest, compaction, OS notifications | 12-20 |
| W6 | Mobile apps + push (native reference from T3) | 30-50 |
| W7 | Optional T3 visual reskin of chat/sidebar/command palette | 10-18 |

Estimates are agent-days, planning only.

---

## 6. Licensing and provenance

1. MIT permits the copy; keep the copyright/permission notice.
2. Each copied directory gets `PROVENANCE.md`: upstream `6fc9144a`, source
   path, destination path, adaptations, licence.
3. Do not copy T3 trademarks/logos/store assets with the code.
4. Generate our own third-party notices; do not copy their SPDX set.
5. A copied file with an unrecorded rewrite is a provenance defect.

---

## 7. Immediate next commits (delta-sized)

1. `chore(t3ui): skeleton + PROVENANCE + licence notices`
2. `feat(pr): copy pull-request surface leaves; wire integrations clients`
3. `feat(diff): copy source picker + worker pool; keep dxbot diff data`
4. `feat(files): copy PDF/audio/table/binary preview leaves`
5. `feat(device): copy device panel; add Rust device service`
6. `feat(replies): copy guest page renderer; add Rust htmlRender`
7. `feat(terminal): Ghostty renderer behind the existing PTY`
8. `feat(links): prompt links + guest links`
9. `feat(protocol): version handshake and capability advertisement`
10. optional `feat(ui): T3 visual language for chat/sidebar/panel`

Each commit removes the matching framed `/t3code/*` route only when the native
surface actually renders.

---

## 8. What this plan does not claim

- It does not claim the copied surfaces compile without the adapters named.
- It does not claim T3's runtime comes with its UI; the runtime is Rust work.
- It does not ask to replace working dxbot features with copies. Where a
  feature exists, the plan says so and reuses it.

MIT OR Apache-2.0
updated: 2026-10-31 00:00 UTC
