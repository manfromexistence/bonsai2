# DX Agents Achievements

`agent` owns the DX Agents runtime: the Rust-first agent layer that connects
providers, memory, gateway pairing, cron, skills, tools, receipts, and the Zed
bridge into a single DX-branded agent product.

## Position

DX Agents is a ZeroClaw-derived runtime fork that has been shaped for the DX
workspace. It intentionally keeps internal `zeroclaw-*` workspace crates where
that helps upstream comparison and mergeability, while public commands,
repository naming, release assets, docs, and Zed-facing contracts are branded as
DX Agents.

Professional positioning:

> DX Agents is the Rust-first agent runtime for the DX ecosystem, with provider
> routing, memory, gateway pairing, cron, skills, tool receipts, Zed bridge
> contracts, and production-focused secret-redaction hardening.

That positioning is strong because it names the actual runtime surfaces. It should not
be inflated into "all automation and provider workflows are live-proven" unless
the current receipts, provider smokes, and governed runtime checks say so.

## What Agent Has Already Achieved

| Area | Achievement |
| --- | --- |
| Product identity | Defines public package, binary, update, repository, installer, release, Docker, and service naming around `dx-agents` while retaining legacy readers for migration. |
| Provider routing | Preserves and extends provider-agnostic routing across OpenAI-compatible providers, Anthropic, OpenAI, Gemini, Ollama, Groq, OpenRouter, and other provider surfaces. |
| Secret handling | Moves provider keys to environment-variable seeding, encrypts route secrets on save/load, masks config responses, and sanitizes provider errors before they reach UI or logs. |
| Zed bridge | Exposes fixed JSON command contracts for status, snapshot, contract audit, social actions, automations, provider/model discovery, run receipts, receipt lists, import summaries, release gates, and validation packets. |
| Receipt model | Stores metadata-safe agent receipts under `G:\Dx\.dx\receipts\agents` and repeatedly documents closed redaction for credentials, prompts, transcripts, account targets, automation bodies, tool payloads, and receipt bodies. |
| Runtime foundations | Retains agent loop, security, approval, sandboxing, workspace boundaries, tool receipts, SOP, skills, cron, hardware, ACP, gateway, dashboard, and desktop support from the upstream-derived runtime. |
| Memory layer | Preserves SQLite and other memory backends, migration readers, learning receipts, memory stats, memory tools, and payload-free memory status validation. |
| Gateway and ACP | Keeps gateway pairing, dashboard/chat/config surfaces, WebSocket/HTTP access controls, ACP bridge tokens, pairing-code recovery, and loopback/container hardening in one runtime workstream. |
| Automation posture | Provides automation list/run/status and background-task receipt surfaces while keeping unsupported schedulers and live task execution out of metadata-only release workstreams. |
| Release governance | Adds dry-run/default-safe release automation, self-update repository selection, signed/typed release naming, container/deploy policy checks, and metadata-only promotion gates. |
| Maintainability direction | Splits CLI definitions, handlers, bridge contracts, report DTOs, redaction metadata, provider catalog safety, gateway helpers, auth, memory, cron, status, parity, and config logic into focused Rust modules. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `agent/README.md` | Public DX Agents overview, quickstart, inherited runtime strengths, config model, and development checks. |
| `agent/DX.md` | release-work ledger for provider, secret, bridge, receipt, release, install, and maintainability hardening. |
| `agent/AGENTS.md` | Product direction, single-canonical rules, security boundaries, project snapshot, and verification rules. |
| `agent/Cargo.toml` | Workspace members, public package/binary names, repository identity, dependency surface, and feature model. |
| `agent/docs/parity/openclaw-hermes-core.md` | Parity matrix for inherited and implemented runtime capabilities, metadata-only gates, and migration surfaces. |
| `agent/src/zed_agent_bridge*.rs` | Zed bridge contracts, report DTOs, redaction metadata, command maps, and surface definitions. |
| `agent/src/cli_agents_*.rs` | CLI command and handler surfaces for Zed-facing agent bridge workflows. |
| `agent/crates/zeroclaw-providers/src/*` | Provider routing, catalog, auth, reliability, and streaming error-sanitization implementation. |
| `agent/crates/zeroclaw-config/src/secrets.rs` | Secret storage and encryption surface. |
| `agent/crates/zeroclaw-runtime/src/agent/tool_receipts.rs` | Tool receipt ownership in the runtime layer. |
| `cargo metadata --no-deps` | Lightweight structure check when the repo is touched. |
| Targeted provider/config/gateway tests | Required for security or provider changes; avoid broad expensive checks during docs-only passes. |

## Public Positioning Boundary

Safe internal statement:

> DX Agents is a production-focused Rust-first agent runtime with DX-branded CLI surfaces,
> provider/model routing, memory, gateway, cron, skills, Zed bridge receipts,
> metadata-only release qualification checks, and secret-redaction hardening.

Safe public wording after refreshed checks:

> The current `dx-agents` build exposes the documented provider, model, receipt,
> gateway, social, automation, and Zed bridge JSON contracts with closed
> redaction for sensitive fields and current targeted test validation.

Unsafe statement without more verification:

> Every provider, social channel, automation, live task, desktop surface, and
> release automation path is production-proven end to end.

The strongest current result is runtime architecture plus security and bridge
hardening. The remaining verification work is live provider/channel coverage, governed
automation runtime verification, fresh targeted tests for touched modules, and eventual
release automation gates that move beyond metadata-only packets.

## Next Documentation Pass

The `flow` page now documents the local-first input and runtime layer that feeds
speech, activation, text refinement, and low-end model routing into Zed and DX Agents.
The next folder should be `forge`, because Forge is the DX source and media
version-control layer for packages, assets, remotes, and record-backed project
history.
