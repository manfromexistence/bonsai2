# DX Forkness Achievements

`forkness` is the DX recursive-agent-harnessing surface: a Rust port of Google's
[`Harness²`](https://github.com/google-research/harness2) that ships one
`dx-forkness` binary instead of a Python runtime with seven console scripts.

## Position

DX Forkness is a **DX tool** in the house sense: a binary driven through
`dx forkness` and published by `TOOL_REGISTRY` in `cli/src/update_registry.rs`.

| Fact | Evidence |
| --- | --- |
| DX binary identity | `forkness/README.md`: "The port ships one `dx-forkness` binary with subcommands." |
| DX repo identity | `forkness/Cargo.toml` → `repository = "https://github.com/essence-dx/dx-forkness"`, `homepage` the same |
| Upstream it ports | `forkness/README.md` → `google-research/harness2` @ `1cfe254`, paper *Harness²: Recursive Agent Harnessing for an Open World*, arXiv `2610.14224` |
| Licence | Apache-2.0 both sides (`forkness/Cargo.toml` → `license = "Apache-2.0"`) |
| Workspace layout | `forkness/Cargo.toml` → members `dx-forkness-core`, `dx-forkness-substrate`, `dx-forkness-bench`, `dx-forkness-cli` |
| Registry entry | `cli/src/update_registry.rs` → `("forkness", "essence-dx/dx-forkness", "dx-forkness", "bin/dx-forkness")` |
| Live command | `cli/src/cli.rs` → `Forkness(DxToolchainForwardArgs)`; `cli/src/command_dispatch.rs` → `Command::Forkness(args) => run_forkness(...)` |
| Manifest path | `dx` manifest `paths` block → `forkness = forkness` |
| Local build output | `forkness/target/debug/dx-forkness.exe` exists on this machine |

## What Forkness Has Already Achieved

| Area | Achievement |
| --- | --- |
| One-binary delivery | The Rust port replaces the upstream Python release's seven console scripts, `uv`, a pinned Bun runtime, and a per-benchmark virtualenv with a single `dx-forkness` binary and a Rust toolchain. |
| Correctness by construction | The method's two hard invariants — *no answers* and *a judge failure is not a zero* — are enforced by the type system through a `Score` type rather than by runtime checks. |
| Reference-preserving port | The original Python implementation is kept beside the port as the executable specification rather than being discarded. |
| Benchmark harness | A dedicated `dx-forkness-bench` crate carries the benchmark surface separate from the core and substrate. |
| Substrate separation | `dx-forkness-substrate` isolates the execution substrate from `dx-forkness-core`, so the method can be ported to new substrates without rewriting the harness. |

## Validation To Check

| Validation | Purpose |
| --- | --- |
| `forkness/README.md` | Upstream mapping, port rationale, licence, and Rust version floor (edition 2021, minimum 1.75). |
| `forkness/Cargo.toml` | Workspace members, package identity, and DX repository declaration. |
| `forkness/crates/dx-forkness-cli/` | The `dx-forkness` binary and its subcommands. |
| `forkness/crates/dx-forkness-core/` | Core method and the `Score` invariant types. |
| `forkness/crates/dx-forkness-substrate/` | Execution substrate boundary. |
| `forkness/crates/dx-forkness-bench/` | Benchmark harness and its `config.rs`. |

## Release Status — Honest

DX Forkness is **repository-managed and command-routed, not release-qualified**.

| Claim | Tier | Evidence |
| --- | --- | --- |
| The Rust port exists, builds, and produces `dx-forkness.exe` | repo-managed | `forkness/target/debug/dx-forkness.exe` on this machine |
| The DX repository is published | **not true** | `gh repo view essence-dx/dx-forkness` → `Could not resolve to a Repository` |
| A release artifact exists | **not true** | no repository, therefore no release |
| `dx tool install forkness` works | **not true** | fails closed with `API 404` on the missing repository |

The local `forkness/` workspace has **no `origin` remote** — it is a local
checkout only. Publishing the `essence-dx/dx-forkness` repository and a
release under the house `dx-<tool>-<rust-target-triple>[.exe]` asset naming is
the gate.

Do not write "published" or "installable" about `dx forkness` without a
receipt.

## Public Positioning Boundary

Safe internal statement:

> DX Forkness is the DX Rust port of Google's `harness2` recursive agent
> harnessing framework, shipping one `dx-forkness` binary with enforced
> method invariants.

Safe public wording after a release exists:

> DX Forkness ships as `dx-forkness` and is installable through `dx tool
> install forkness`.

Unsafe statement today:

> DX Forkness is the fastest recursive agent harness, installable with one
> command.

## Open Work

- Create and push the `essence-dx/dx-forkness` repository, then publish a
  release whose assets follow the house `dx-<tool>-<rust-target-triple>` naming
  so `dx tool install forkness` resolves an asset. Tracked in `cli/TODO.md`
  under "Open: New Tool Release Gaps (grimoire / forkness)".
- The local workspace has no `origin`; decide whether it is pushed to
  `essence-dx/dx-forkness` directly or moved into a tree that already is.

MIT OR Apache-2.0
updated: 2026-10-20 00:00 UTC
