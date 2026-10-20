# `hackin` — DX Rust port of the Strix autonomous AI pentesting engine

`hackin` is the DX product folder for **`dx-hackin`**: a Rust port of the
open-source Strix AI pentesting agent. The Python original is retained
read-only as the port's specification, in the same shape as `forkness` retains
the upstream Python implementation of `harness2`.

| Property | Value |
| --- | --- |
| Folder | `G:\Dx\hackin` |
| Product / binary name | `dx-hackin` |
| Declared in the hub manifest | `paths.hackin = hackin` (`dx:22`) |
| Product repo | `G:\Dx\hackin` (git repo, own history) |
| Reference tree | `hackin/reference/strix-python/` — vendored `usestrix/strix` @ `a0f8ea2`, ignored by this repo's git |
| Rust workspace | `hackin/crates/` — 7 crates |
| Upstream project | `https://github.com/usestrix/strix` (Apache-2.0) |

## What the upstream project is

Strix is an open-source autonomous AI penetration-testing tool. A team of LLM
agents runs the target's real code, drives a real browser and a real HTTP
interception proxy, writes proof-of-concept exploits, and validates them
before a finding is filed. It is not a static analyser: the differentiator is
evidence-backed findings rather than model confidence.

The upstream tree is ~59,600 production lines across three languages:

| Layer | Files | Lines |
| --- | --- | --- |
| Python (engine, CLI, TUI backend, viewer server) | 136 | 38,803 |
| Go (Bubble Tea terminal UI) | 57 | 12,203 |
| TypeScript / TSX (local web viewer SPA) | 66 | 8,073 |
| Jinja (agent system prompts) | 2 | 501 |

## What this folder holds

| Path | What it is |
| --- | --- |
| `crates/` | the Rust workspace — the product |
| `crates/dx-hackin-core/` | agent graph, `AgentCoordinator`, budget/pause policy, mailbox, atomic snapshots |
| `crates/dx-hackin-config/` | the full settings surface, env-var resolution, config-file loader |
| `crates/dx-hackin-llm/` | the turn guard (tool-call ids / arguments / volume), context budget, conversation compaction |
| `crates/dx-hackin-report/` | CVSS 3.1 calculator, SARIF 2.1.0 writer, coverage ledger, report state, artifact writer |
| `crates/dx-hackin-runtime/` | Docker sandbox lifecycle, bind mounts, manifest, deferred session |
| `crates/dx-hackin-proxy/` | the Caido GraphQL interception-proxy client |
| `crates/dx-hackin-provider/` | the model provider — an OpenAI-compatible client with the upstream's turn guard, retries and stream deadlines |
| `crates/dx-hackin-agent/` | the agent loop — the turn rule, the recovery nudge, the budget, the session store |
| `crates/dx-hackin-tools/` | the 42 tools the agents call (33 working, 9 honest stubs) |
| `crates/dx-hackin-cli/` | the `dx-hackin` binary |
| `reference/strix-python/` | the vendored Python original, read-only, git-ignored |
| `TODO.md` / `CHANGELOG.md` | the product ledger and history |
| `AGENTS.md` | how to work safely in this folder |
| `TECH.md` | the architecture reference — describes the Python original in detail |
| `.github/workflows/` | CI (fmt, clippy, test on stable and MSRV, docs, cargo-deny) and the six-target release matrix |
| `SECURITY.md`, `CONTRIBUTING.md`, `deny.toml` | the security policy, the contribution guide, the dependency policy |

## Porting approach

Each Rust module cites the Python `file:line` it was ported from, in its `///`
doc comments and in the crate's `lib.rs` module doc. Where the Rust cannot
match the Python's behaviour, the deviation is written down in the crate doc
**and** filed in `TODO.md` with evidence — the same discipline the hub applies
to its own ledgers.

Three upstream dependencies have no Rust equivalent, and each is behind a
trait in this port so the code can be built and tested without them:

| Upstream | Rust seam | Consequence |
| --- | --- | --- |
| OpenAI Agents SDK | `SessionItems`, `SummaryModel` | the agent loop itself is not ported yet |
| LiteLLM | `ModelCatalogue`, `TokenCounter` | token counting falls back to UTF-8 byte length (the upstream's own conservative fallback) |
| MCP SDK | not yet ported | the MCP dispatch tools are not implemented yet |

## Honest status

**repo-managed, and it runs.** The Rust workspace compiles, `cargo test
--workspace` is green (**605 passed, 0 failed**), and `cargo clippy
--workspace --all-targets -- -D warnings` is clean. `dx-hackin` drives a real
model through the ported engine: the request goes out, the tool calls come
back, the tools execute in a real Kali container, and the lifecycle tool ends
the run. It was verified end to end here against a local mock of the OpenAI
API — the root agent took two turns, called `think`, called `finish_scan`, the
report artifacts were written, and the process exited `0`.

The port now covers the **agent loop** (the rule that plain text never ends a
turn), the **provider** (an OpenAI-compatible client with the upstream's turn
guard, retry policy and stream deadlines), the **budget** (one shared guard,
the graduated warnings, the 90 % reserve), the **session store** (with the
optimistic guard), the **request log** (one bounded record per attempt, and
never a prompt), the **tool set** (42 tools), the **sandbox** (a real Kali
container, created before the first tool call and deleted at the end), the
**multi-agent graph** (a child really spawns, gets its own session, and
reports back), and the **operating doctrine** (the upstream's prompt, ported).

**Not claimed.** The port is **not** feature-complete, and it is **not**
release-qualified. The **MCP layer** is not ported, so the MCP dispatch tools
remain stubs. The **skill packs** are not ported, so the knowledge blocks
render empty. The **web-search client** is not ported. The **dedupe judge**
and the **pricing** are not ported. The **TUI**, the **viewer**, the **cloud
CLI** and the **telemetry** are not ported. No `dx-hackin` binary has been
released, and the upstream's published benchmark (96% on XBEN, measured on
upstream **v0.4.0**) is a property of the Python implementation, not of this
port. The scan was verified against a **mock** of the OpenAI API, not against
a real model, and the sandbox could not be brought up on the machine this was
built on because Docker is not installed there.

The authoritative, itemised state lives in `hackin/TODO.md` (what is open,
with evidence and a done-when check) and `hackin/CHANGELOG.md` (what changed,
dated, one record per event).

## References

- `hackin/AGENTS.md` — working rules for this folder
- `hackin/TODO.md` — open work, with evidence
- `hackin/CHANGELOG.md` — dated history
- `hackin/TECH.md` — the architecture of the Python original
- `hackin/crates/Cargo.toml` — the Rust workspace manifest
- `docs/forkness.md` — the same pattern, for the closest sibling product
