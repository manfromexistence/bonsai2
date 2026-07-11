# DX Driven Achievements

`driven` is the Rust AI-assisted development orchestrator for DX. It owns rule
format parsing/emitting, editor sync, templates, validation, hooks, steering,
system/context analysis, and the worker coordination strategy receipt model that can keep
parallel AI work accountable.

## Position

Driven should be the coordination layer between human intent, AI coding rules,
editor-specific rule files, and multi-worker DX execution. Its strongest current
achievement is the repository-managed worker coordination verification system: wording, receipts,
captured command validation, redaction, handoffs, and completion checks.

Professional positioning:

> Driven is a Rust AI-development orchestrator with rule parsers/emitters,
> templates, validation, sync, steering, hooks, context analysis, DCP/DX
> integration, and a record-backed worker coordination workflow for accountable parallel
> work.

That statement intentionally avoids repeating stale broad performance or
production qualification wording from older architecture docs. The repo has real
modules and tests, but any public positioning about binary speedups, test totals, or
release status needs a fresh verification run.

## What Driven Has Already Achieved

| Area | Achievement |
| --- | --- |
| Rule source model | Models a unified rule set that can load, save binary `.drv`, and emit editor-specific formats. |
| Parser and emitter surface | Keeps parser modules for Aider, Claude, Copilot, Cursor, Windsurf, and unified rules, plus emitters for Claude, Copilot, Cursor, Windsurf, and generic output. |
| CLI surface | Exposes `init`, `sync`, `convert`, `template`, `analyze`, `validate`, `hook`, `steer`, and `strategy` subcommands. |
| Template system | Provides personas, project templates, standards, tasks, workflows, composer, registry, and generator-integration templates. |
| Context and scale analysis | Includes context scanner/indexer/provider modules, system info detection, project scale analyzers, and project-type recommendation structures. |
| Hooks and steering | Owns agent hook triggers/actions/conditions and steering rules with inclusion modes, manual keys, file matching, testing, and injection. |
| DX integration | Connects to `dx-serializer`, DX markdown, DCP, generator integration, cross-crate validation, and source-crate integration structures. |
| Binary/fusion/state modules | Keeps binary rule schemas, string tables, checksums, memory maps, hot cache, speculative loader, dirty bits, snapshots, shared rules, and atomic sync code. |
| worker coordination strategy | Models workstream wording, worker identity, pass numbers, worktree metadata, isolation plans, statement/release/next/complete flows, and state files. |
| Canonical verification receipts | Validates receipt schema, receipt ID, outcomes, command proofs, file proofs, worktree identity, and completion status. |
| validation capture | Captures command stdout/stderr digests, byte counts, truncation flags, durations, cwd, and timestamps with output cap policy. |
| Durable handoffs | Requires receipt plus next action for default next-pass advancement unless the caller explicitly opts into unsafe legacy behavior. |
| Redaction boundary | Renders JSON/Markdown receipts with secret redaction, canonical IDs, redacted payload digests, escaped tables, and preserved digest labels. |
| Strategy tests | Keeps focused tests for receipt validation, durable handoffs, output caps, timeout receipts, redaction, command validation, and workstream cycling. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `driven/README.md` | High-level purpose and CLI intent; treat broad performance wording as requiring fresh verification. |
| `driven/Cargo.toml` | Crate identity, binary/library targets, DX/DCP dependencies, parser/sync/template dependencies, and benchmark dependencies. |
| `driven/src/lib.rs` | Public module/export surface for parser, emitter, sync, templates, hooks, steering, strategy, integration, system info, scale, binary, fusion, security, state, and streaming. |
| `driven/src/main.rs` | Actual CLI command wiring, especially the `strategy` subcommands and durable handoff flags. |
| `driven/src/strategy/*.rs` | worker coordination state, worktree identity, handoff, receipt, redaction, orchestration, and validation contracts. |
| `driven/src/strategy/receipt.rs` | Canonical verification receipt validation, command verification rules, completion status, digest computation, markdown/json rendering, and redaction. |
| `driven/tests/strategy_contract_tests.rs` | Contract tests for receipts, handoffs, validation, redaction, validation, and markdown escaping. |
| `driven/tests/strategy_binary_cli_tests.rs` | CLI tests for strategy receipt, complete, next/handoff, output caps, timeouts, default durable-next behavior, and workstream cycling. |
| `driven/docs/ARCHITECTURE.md` | Older architecture/design notes; useful for context, but performance and release wording needs current verification. |
| `driven/.github/workflows/README.md` | Disabled GitHub Actions policy. |
| `cargo test -p dx-driven strategy -j1` or targeted strategy tests | Runtime authority when the strategy/receipt behavior needs fresh verification. |

## Public Positioning Boundary

Safe internal statement:

> Driven owns the DX rule-orchestration and worker coordination receipt model, including
> source parsers, emitters, templates, validation, sync, hooks, steering,
> context analysis, and canonical verification receipts.

Safe public wording after refreshed checks:

> The current Driven strategy tests prove receipt validation, captured validation,
> durable handoffs, completion status, output caps, timeout receipts,
> redaction, markdown rendering, and workstream cycling for the tested CLI paths.

Unsafe statement without more verification:

> Driven is fully production-ready, has all historical benchmark numbers current,
> ships on a past release date, or proves every binary/fusion/streaming/security
> module is complete.

The strongest current result is record-backed development orchestration. The
remaining verification work is fresh strategy test execution, architecture doc cleanup,
benchmark refresh, stale release wording removal, and CI design when Actions are
explicitly reopened.

## Documentation Status

The `extensions` documentation pass is complete. The next folder should be
`i18n`, because localization, TTS, STT, and local-first translation need a clear
boundary between repository-managed capability and model/provider availability.
