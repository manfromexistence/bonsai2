# DX Grimoire Achievements

`grimoire` is the DX reverse-engineering surface: a CLI and an MCP server that
lets an agent understand binaries, applications, and runtime behavior down to
the binary level. It is the DX fork of the upstream `morluto/rea` project.

## Position

DX Grimoire is a **DX tool** in the house sense: a binary driven through
`dx grimoire` and published by `TOOL_REGISTRY` in `cli/src/update_registry.rs`.
It is not a vendored third-party checkout — the fork is DX-owned and the
package identity says so.

| Fact | Evidence |
| --- | --- |
| DX package identity | `grimoire/package.json` declares `"name": "dx-grimoire"` and `"mcpName": "io.github.essence-dx/grimoire"` |
| DX repo ownership | `git -C grimoire remote get-url origin` → `https://github.com/essence-dx/grimoire.git` |
| Upstream is tracked | `git -C grimoire remote -v` also shows `upstream https://github.com/morluto/rea.git` |
| Upstream merges are real | `git -C grimoire log --oneline -5` → `046daab36 Merge remote-tracking branch 'upstream/main'` |
| Registry entry | `cli/src/update_registry.rs` → `("grimoire", "essence-dx/grimoire", "dx-grimoire", "bin/dx-grimoire")` |
| Live command | `cli/src/cli.rs` → `Grimoire(DxToolchainForwardArgs)`; `cli/src/command_dispatch.rs` → `Command::Grimoire(args) => run_grimoire(...)` |
| Manifest path | `dx` manifest `paths` block → `grimoire = grimoire` (added 2026-10-20) |

## What Grimoire Has Already Achieved

| Area | Achievement |
| --- | --- |
| Reverse-engineering scope | One MCP surface for binaries, applications, and runtime behavior, exposed as both a CLI and an MCP server. |
| MCP contract surface | Ships a generated MCP tool catalog and per-view limits/unknowns bounding, so an agent sees what a view can and cannot assert. |
| Multi-language analysis | Repository layout carries dedicated bridges and views for JavaScript, Android/APK, ADB, application and artifact analysis, and binary diagnostics. |
| Provenance tracking | Preserves empty ESM import names in provenance and bounds limitations per analysis view. |
| Native acceleration | Carries a Rust native layer (`grimoire/native`, `grimoire/rust`) alongside the TypeScript surface, with real-binary layout and interface verification workflows. |
| Release plumbing | Release-please managed versioning and a `release.yml` workflow; current package version is 6.4.0. |
| Multi-runtime CI | Workflow set covers CI, timings, native-AOT fixtures, SQLite runtime, pages, and per-runtime real-binary checks. |

## Validation To Check

| Validation | Purpose |
| --- | --- |
| `grimoire/README.md` | Upstream-authored feature overview and install path. |
| `grimoire/package.json` | DX package identity, `dx-grimoire` bin mapping, and MCP name. |
| `grimoire/docs/mcp-contracts.md` | Generated MCP tool catalog the README links to. |
| `grimoire/PLAN.md` | DX-side plan of record for the fork. |
| `grimoire/AGENTS.md`, `grimoire/TECH.md` | Repo working rules and technical contract. |
| `grimoire/.github/workflows/` | CI and release workflow set. |
| `grimoire/bridge/`, `grimoire/native/`, `grimoire/rust/` | Bridge, native, and Rust acceleration surfaces. |

## Release Status — Honest

DX Grimoire is **repository-managed and command-routed, not release-qualified**.

| Claim | Tier | Evidence |
| --- | --- | --- |
| The DX fork exists and is wired as a DX tool | repo-managed | `package.json` name `dx-grimoire`; registry entry; `dx grimoire` command |
| A GitHub release artifact exists | **not true** | `gh release view -R essence-dx/grimoire` → `release not found` |
| The package is on the npm registry | **not true** | `npm view dx-grimoire` → 404 Not Found |
| `dx tool install grimoire` works | **not true** | fails closed with `API 404` on the missing release |

The upstream project publishes on npm; the DX fork `dx-grimoire` does not yet.
Do not write "installable" or "published" about `dx grimoire` without a receipt.

## Public Positioning Boundary

Safe internal statement:

> DX Grimoire is the DX fork of the upstream `morluto/rea` reverse-engineering
> project, published under the DX package identity `dx-grimoire` and wired as a
> `dx` tool with a CLI and MCP surface.

Safe public wording after a release exists:

> DX Grimoire ships as `dx-grimoire` and is installable through `dx tool
> install grimoire`.

Unsafe statement today:

> DX Grimoire is the fastest reverse-engineering tool, installable with one
> command.

## Open Work

- Publish a `dx-grimoire` release on `essence-dx/grimoire` with the house
  `dx-<tool>-<rust-target-triple>[.exe]` asset naming so `dx tool install
  grimoire` resolves an asset. Tracked in `cli/TODO.md` under "Open: New Tool
  Release Gaps (grimoire / forkness)".
- Decide whether the npm registry or GitHub releases is the primary DX
  distribution channel for a Node-published tool, and record the decision.

MIT OR Apache-2.0
updated: 2026-10-20 00:00 UTC
