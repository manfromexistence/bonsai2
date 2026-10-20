# DX Code Achievements

`code` is the DX editor surface. It is the place where the CLI, WWW, Agents,
Forge, Check, Style, receipts, source sets, media, and future Friday workflows
become visible to the operator inside a real GPUI editor fork.

## Position

DX Code is an upstream-derived Zed checkout with DX-specific editor surfaces
layered into the product. Its current achievement is repository-audited integration:
the repo records a large amount of DX UI, receipt, bridge, and safety work, but
the local handoff explicitly keeps final runtime verification, native release verification,
broad Cargo validation, and visual QA behind governed validation windows.

Professional positioning:

> DX Code is the repository-audited editor shell for the DX operating model, with
> real GPUI surfaces for agents, release status, source views, receipts,
> project/media exploration, Web Preview safety, Forge, Check, Style review,
> and semantic DX tool identity.

That statement keeps the upstream editor and DX overlay separate. DX should not
statement all upstream Zed features as DX achievements, and should not statement final
product status until the governed verification window refreshes the native runtime
validation.

## What DX Code Has Already Achieved

| Area | Achievement |
| --- | --- |
| Editor role | Establishes `code` as the DX editor surface for GPUI integration, AI panel polish, DX Agents hooks, token meters, source views, Progress views, and release-safe runtime wiring. |
| repository-level honesty | `DX.md`, `todo.txt`, and `AGENTS.md` repeatedly separate source-verified changes from runtime/native verification, Cargo health, server verification, and visual QA. |
| release workspace | Adds DX release/status/source/verification concepts to the editor so source rows, Sources views, Progress views, status state, and record-backed validation can be rendered inside the product. |
| Agent surfaces | Extends the AI surface with fullscreen Agent views, response markers, profile-aware controls, Connections, Tools, Automations, provider/social state, trusted bridge metadata, and record-backed unavailable states. |
| No unsupported execution | Keeps automation creation, tool approvals, provider login, QR payloads, and runtime-verified states disabled or unavailable until DX Agents emits explicit backend receipts or governed runtime verification. |
| Web Preview safety | Hardens Web Preview navigation, completion events, favicon handling, IPC bounds, platform queues, onboarding handoff, and DX Style generator review without treating a WebView demo as product verification. |
| Project Panel and media | Adds storage-root, cloud-root, file-operation, media-shelf, generated-media, video-frame, and folder-storage intelligence while documenting cache, path, and stale-work guardrails. |
| Forge and Check panels | Presents Forge and Check as integrated editor surfaces over source/receipt data, with tabs, rows, keyboard navigation, and status honesty instead of green status by assumption. |
| DX Style review bridge | Connects Web Preview and native review around DX Style grouped-class, reverse-CSS, source-apply, dry-run, and future mutation contracts while keeping Apply blocked until receipts and runtime verification exist. |
| Semantic DX icons | Moves DX identity through centralized semantic icon routing for Agent, Style, deploy gateways, settings, open-file, media, provider, cloud, and tool surfaces instead of scattered literal icons. |
| Source guard registry | Maintains a broad set of lightweight `script/dx-*-source.test.ts` guards that check source contracts, UI wording, receipt fields, fanout boundaries, and repository-level wording hygiene without running the editor. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `code/DX.md` | Primary release-work ledger for DX-specific editor work, current verification workstream, deferred runtime verification, and source guard registry. |
| `code/AGENTS.md` | Zed-derived repo operating contract, forbidden heavy commands, and source-inspection workflow rules. |
| `code/PLAN.md` | Forward feature plan for editor shell polish, Web Preview onboarding, dock/sidebar stacking, activity bar, plugins/workflows, and automations. |
| `code/todo.txt` | Current status of source-verified, build-verified, pending, and deferred runtime work. |
| `code/script/dx-handoff-source-guard-registry.test.ts` | Handoff docs, guard registry, and repository-level wording hygiene guard. |
| `code/script/dx-launch-workspace-source.test.ts` | release workspace source-contract guard. |
| `code/script/dx-agent-workspaces-source.test.ts` | Agent workspace taxonomy and Connections/Tools source-contract guard. |
| `code/script/dx-forge-panel-source.test.ts` | Forge panel source-contract guard. |
| `code/script/dx-check-panel-source.test.ts` | Check panel source-contract guard. |
| `code/script/dx-style-panel-source.test.ts` | Style panel and Web Preview generator source-contract guard. |
| `code/script/dx-project-panel-source.test.ts` | Project Panel, storage, file-operation, and media source-contract guard. |
| `code/script/dx-runtime-proof-status-source.test.ts` | Runtime-verification status source-contract guard. |
| `git -C G:\Dx\code status -sb` | Required before release wording; this docs pass observed `dev` clean but ahead of `origin/dev` by one commit. |
| `git -C G:\Dx\code diff --check` | Lightweight repository hygiene check when source files are touched. |
| Authorized `just run` window | Required for native runtime verification; do not run it during repository-level documentation or low-load governance passes. |

## Public Positioning Boundary

Safe internal statement:

> DX Code has a production-focused repository-audited editor integration layer for DX release,
> Agents, receipts, Forge, Check, Style, Web Preview, Project Panel, media, and
> semantic tool identity.

Safe public wording after refreshed governed verification:

> The current DX Code build launches and renders the documented DX surfaces with
> record-backed status, guarded Web Preview behavior, and repository-managed tool
> handoffs, while preserving upstream Zed behavior.

Unsafe statement without more verification:

> The DX Code editor fork is fully runtime-proven across native UI, WebView,
> agent, provider, tool, Forge, Check, Style, media, and release workflows.

The strongest current result is repository-audited editor integration. The remaining
verification work is governed runtime validation: native release, visual review, current
source guards, build health, receipt freshness, provider/tool login verification, and
syncing the current `dev` commit to its canonical remote.

## Next Documentation Pass

The `agent` page now documents the runtime that DX Code and CLI both depend on for
provider, social, tool, automation, and future Friday-facing workflows. The next
folder should be `flow`, because Flow is the local-first input/runtime layer
that feeds speech, activation, text refinement, and low-end model routing into the DX
operator experience.
