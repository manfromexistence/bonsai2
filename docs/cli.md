# DX CLI Achievements

`cli` is the DX command and TUI control plane. Its achievement is not just a
large command list; it is the repository-managed front door that lets the rest of the
DX ecosystem behave like one product instead of many unrelated tools.

## Position

DX CLI owns the `dx` command. It routes operators into DX WWW, Forge, Check,
Serializer, Style, Icon, Media, Python, JavaScript, native work, agent bridges,
release validation receipts, and the Ratatui terminal shell from one configured
entry point.

Professional positioning:

> DX CLI is a repository-managed command interface that centralizes DX tool routing,
> receipt discovery, TUI handoffs, and host integration contracts while keeping
> high-risk runtime work behind explicit commands and approval boundaries.

That positioning is stronger than saying every command is production-ready today. The
CLI is already the ecosystem control plane, while some runtime, native-renderer,
and release gates still require current receipts before public release language.

## What CLI Has Already Achieved

| Area | Achievement |
| --- | --- |
| Root command identity | Defines the public `dx` binary through the `dx-cli` Rust package and keeps the workspace/vendor `dx-tui` identity separate from the root CLI version. |
| Default operator surface | Opens the Ratatui terminal shell when `dx` runs with no subcommand, while keeping advanced and lab commands hidden from first-time help. |
| Tool routing | Routes `dx new`, `dx dev`, `dx run`, `dx add`, `dx www`, `dx forge`, `dx check`, `dx style`, `dx icon`, `dx media`, `dx serializer`, `dx py`, `dx js`, `dx native`, and deploy/status paths through repository-managed DX folders. |
| Direct script dispatch | Routes JavaScript/TypeScript and Python files through DX-managed runtime paths before normal CLI parsing, so `dx main.js` and `dx main.py` use configured G-drive engines. |
| Workspace manifest | Reads the root extensionless `dx` manifest for workspace roots, tool paths, engine choices, cache locations, and TUI release defaults instead of scattering hardcoded paths. |
| Check receipts | Exposes `dx check` as a 500-point project-health and release status receipt surface with safe runner config, Zed panel fields, approval states, and validation import boundaries. |
| Runner safety | Keeps expensive checks, live browser validation, HTTPS probing, full E2E, and configured runners opt-in; shell strings, unsafe hosts, and workspace-escaping `cwd` values fail closed. |
| Zed and host contracts | Provides bridge, host-contract, host-export, host-verify, host-smoke, release, token, and agent validation commands so future Zed/Friday panels can consume fixed JSON contracts instead of scraping prose. |
| TUI adapter guardrails | Models adapter plans, state, shell rendering, intents, confirmation tokens, execute-plan previews, explicit execution, audit records, runbooks, and drilldowns before host panels are allowed to release engines. |
| Media and retained engines | Preserves mpv, tplay, viu, and the Ratatui shell as explicit retained engines while documenting the native-module extraction path. |
| Install governance | Provides install and update commands that prefer plan-only, backup, rollback, hash verification, signed envelopes, and explicit replacement gates. |
| Modularity direction | The repo has been extracting command orchestration, build status, graph status, host verification, agents bridge, and adapter logic into focused Rust modules instead of leaving all behavior in the binary entrypoint. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `cli/README.md` | Public CLI overview, command surfaces, dx-check receipt contract, adapter/TUI guardrails, host contracts, install flow, and Rust layout. |
| `cli/DX.md` | release-work notes, TUI bridge decisions, production-hardening history, Zed-facing command contracts, and receipt policies. |
| `cli/AGENTS.md` | Agent-facing contract for stable JSON/receipt commands and the no-heavy-build rule. |
| `cli/Cargo.toml` | Public Rust package identity, binary name, dependency surface, and workspace-owned path dependencies. |
| `cli/src/main.rs` | Startup boundary: direct script dispatch, CLI parse, no-arg TUI default, config loading, capability discovery, and command dispatch. |
| `cli/src/cli.rs` | Public and hidden command taxonomy, examples, safety wording, and routing intent. |
| `dx` | Root workspace manifest for configured folder paths, tool roots, media engines, cache paths, and TUI defaults. |
| `docs/policies/maintainability-baseline.json` | Existing large-file debt that must keep shrinking before `cli` can be called fully production-clean. |
| `git -C G:\Dx\cli status -sb` | Required before release wording because active local child-repo changes may exist during parallel DX work. |
| `dx ecosystem status --json` | Lightweight ecosystem status surface when the installed binary and receipts are current. |
| `dx check --latest-receipt --json` | Non-mutating check receipt reader when release/status validation exists. |

## Public Positioning Boundary

Safe internal statement:

> DX CLI is the working command and TUI control plane for the DX ecosystem,
> with repository-managed routing, receipt contracts, bridge commands, and guarded
> host/TUI handoff surfaces.

Safe public wording after refreshing receipts:

> The current `dx` binary exposes verified command routes for DX WWW, Check,
> Forge, Serializer, Style, Icon, Media, agent bridge, token, release, and host
> handoff workflows, with machine-readable receipts and explicit opt-in gates
> for risky runtime work.

Unsafe statement without more verification:

> Every `dx` command is fully production-certified, release-qualified, and clean of
> maintainability debt.

The strongest current result is control-plane maturity. The remaining verification work
is release validation: clean child worktree, current installed-binary smoke,
refreshed release/check receipts, and continued reduction of oversized Rust
files named in the maintainability baseline.

## Next Documentation Pass

The `zed` page now documents the editor surface where the CLI, WWW, agents,
receipts, tools, and future Friday workflow become visible to the operator. The
next folder should be `agent`, because Zed and CLI both depend on DX Agents for
provider, social, tool, automation, and Friday-facing workflows.
