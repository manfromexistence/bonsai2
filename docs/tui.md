# DX TUI Achievements

`tui` is the DX terminal user interface layer. It provides the interactive
terminal shell that operators use to route commands, inspect receipts,
manage agents, and control DX workflows from a single terminal surface.

## Position

DX TUI is the Ratatui-based terminal shell that serves as the default
operator surface when the `dx` CLI runs without a subcommand. It keeps
advanced and lab commands hidden from first-time help while exposing
the full DX tool routing, receipt discovery, and host integration
contracts through a keyboard-driven interface.

Professional positioning:

> DX TUI is a repository-managed terminal control plane that centralizes
> DX tool routing, receipt discovery, TUI handoffs, and host integration
> contracts while keeping high-risk runtime work behind explicit commands
> and approval boundaries.

That positioning is stronger than saying every command is production-ready
today. The TUI is already the default operator surface, while some
runtime, native-renderer, and release gates still require current receipts
before public release language.

## What DX TUI Has Already Achieved

| Area | Achievement |
| --- | --- |
| Default operator surface | Opens the Ratatui terminal shell when `dx` runs with no subcommand, while keeping advanced and lab commands hidden from first-time help. |
| Tool routing | Routes `dx new`, `dx dev`, `dx run`, `dx add`, `dx www`, `dx forge`, `dx check`, `dx style`, `dx icon`, `dx media`, `dx serializer`, `dx py`, and deploy/status paths through repository-managed DX folders from the TUI. |
| TUI adapter guardrails | Models adapter plans, state, shell rendering, intents, confirmation tokens, execute-plan previews, explicit execution, audit records, runbooks, and drilldowns before host panels are allowed to release engines. |
| LLM integration | The `tui` section in the root `dx` manifest controls `launch_llm_by_default` so the TUI can be configured to start with or without an LLM session. |
| Workspace manifest | Reads the root extensionless `dx` manifest for workspace roots, tool paths, engine choices, cache locations, and TUI release defaults instead of scattering hardcoded paths. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `tui/README.md` | Public TUI overview, repository layout, development workflow, and contribution guidelines. |
| `tui/DX.md` | TUI-specific development notes, build performance, common issues, and testing guidance. |
| `tui/Cargo.toml` | TUI crate identity, dependencies, and workspace membership. |
| `tui/src/` | TUI source modules for rendering, input handling, and adapter logic. |
| `dx` | Root workspace manifest for configured folder paths, tool roots, TUI defaults, and `launch_llm_by_default` setting. |
| `git -C G:\Dx\tui status -sb` | Required before release wording because active local changes may exist during parallel DX work. |

## Public Positioning Boundary

Safe internal statement:

> DX TUI is the working terminal control plane for the DX ecosystem,
> with repository-managed routing, receipt contracts, adapter guardrails,
> and explicit opt-in gates for risky runtime work.

Safe public wording after refreshing receipts:

> The current `dx` TUI exposes verified command routes for DX WWW, Check,
> Forge, Serializer, Style, Icon, Media, agent bridge, token, release, and
> host handoff workflows, with machine-readable receipts and explicit
> opt-in gates for risky runtime work.

Unsafe statement without more verification:

> Every `dx` TUI command is fully production-certified, release-qualified,
> and clean of maintainability debt.

The strongest current result is a functional terminal control plane. The
remaining verification work is release validation: clean child worktree,
current installed-binary smoke, refreshed release/check receipts, and
continued reduction of oversized Rust files named in the maintainability baseline.

## Next Documentation Pass

The `tui` page now documents the terminal control plane that DX CLI and
other DX tools depend on for interactive operator workflows. The next
folder should be `desktop`, because the desktop editor surface is the
primary consumer of TUI routing and adapter handoff capabilities.