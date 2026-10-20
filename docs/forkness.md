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
| DX repo identity | `forkness/Cargo.toml` → `repository = "https://github.com/essence-dx/forkness"`, `homepage` the same |
| Upstream it ports | `forkness/README.md` → `google-research/harness2` @ `1cfe254`, paper *Harness²: Recursive Agent Harnessing for an Open World*, arXiv `2610.14224` |
| Licence | Apache-2.0 both sides (`forkness/Cargo.toml` → `license = "Apache-2.0"`) |
| Workspace layout | `forkness/Cargo.toml` → members `dx-forkness-core`, `dx-forkness-client`, `dx-forkness-substrate`, `dx-forkness-bench`, `dx-forkness-cli` |
| Registry entry | `cli/src/update_registry.rs` → `("forkness", "essence-dx/forkness", "dx-forkness", "bin/dx-forkness")` |
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
| Differential parity | Eight tests compare the port against the **live** Python: the 20 prompt constants byte for byte (they are *generated* by `scripts/extract_prompts.py`, not transcribed), **245** CPython `random` (seed, length) pairs with 0 mismatches, and the `.xlsx`/`.pptx` renderers against `openpyxl`/`python-pptx`. |
| The benchmark runners | A rollout is launched the way the reference launches it, with the same argument lists, environment and deadlines — LAB's `opencode/run_opencode.py`, JobBench's `jb_opencode_runner.py`, and WorkBuddy's Harbor job. The WorkBuddy job YAML was diffed against the live Python: byte for byte identical. |
| The gate suite | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` under the **`pedantic`** group with no inline allows, `cargo test --workspace` (**346 tests, 0 failures**), `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo deny check`, two parity scripts, an asset provenance check, and the local demo with its budget identity. `make ci` runs the whole set and exits 0. |
| Measured coverage | **70.72% of lines** and 70.34% of regions, by `cargo llvm-cov`; the thin places are named rather than hidden. |
| The supply chain | `cargo deny check` passes with **no warnings at all**. Getting there took three advisories in `zip` and one in `quick-xml`; the fix was to move off them — `zip` is 9.0.5 and `quick-xml` 0.41.0 — and the renderers still match the reference byte for byte. The last duplicate crate version went the same way: `windows-sys` 0.59 (through `fs4`) and 0.61 (through `clap`) were two copies of the same Windows bindings in one binary, and moving to `fs4` 1.x removed it. `deny.toml` now **denies** a duplicate rather than warning about one. |
| The two state files | `forkness/TODO.md` carries every open item with its evidence and its done-when; `forkness/CHANGELOG.md` carries what changed, dated. The two are never edited in the same direction. |
| The promises are receipts | Three things the repository says about itself are **checked**, not asserted. The **minimum Rust version** is built and tested on 1.88.0 itself, by a CI job and by `make msrv`; the **absence of `unsafe`** is `forbid`den in the workspace lint table, so a probe `unsafe` block fails the build with `-F unsafe-code`; and the **dependency graph** is held to `Cargo.lock` by `--locked` on every cargo invocation, in CI and in the `Makefile` alike. |

## Validation To Check

| Validation | Purpose |
| --- | --- |
| `forkness/README.md` | Upstream mapping, port rationale, licence, and Rust version floor (edition 2021, minimum supported Rust version 1.88). |
| `forkness/Cargo.toml` | Workspace members, package identity, and DX repository declaration. |
| `forkness/crates/dx-forkness-cli/` | The `dx-forkness` binary and its subcommands. |
| `forkness/crates/dx-forkness-core/` | Core method and the `Score` invariant types. `pynrand.rs` is the CPython parity; `judge.rs` carries the "a judge failure is not a zero" invariant; `tree.rs` carries the leak tripwires. |
| `forkness/crates/dx-forkness-client/` | The improver and selector sessions, the audit trail and the content-addressed cache. |
| `forkness/crates/dx-forkness-substrate/` | Execution substrate boundary. |
| `forkness/crates/dx-forkness-bench/` | Benchmark harness and its `config.rs`; `src/runner/` holds the three rollout runners. |
| `forkness/docs/PARITY.md` | The parity matrix: every reference module, its Rust counterpart, its verdict, and the known differences. |
| `forkness/TODO.md` | The authoritative state. Every open item with its evidence and its done-when. |
| `cargo test --workspace` | The behaviour holds, with no network, no credentials and no benchmark. |
| `cargo +1.88.0 test --workspace --all-features` | It still builds and passes on the **declared** minimum Rust version, so the floor is a receipt rather than a claim. |
| `cargo test --workspace -- --ignored` | The port still matches the **live** Python. Needs `openpyxl` and `python-pptx`. |
| `make -C forkness ci` | Every gate the CI workflow runs, in one command. |

## Release Status — Honest

DX Forkness is **repository-managed and command-routed, not release-qualified**.

| Claim | Tier | Evidence |
| --- | --- | --- |
| The Rust port exists, builds, and produces `dx-forkness.exe` | repo-managed | `forkness/target/debug/dx-forkness.exe` on this machine |
| The DX repository is published | **repo-managed** | `gh repo view essence-dx/forkness` → `{"name":"forkness","visibility":"PRIVATE"}`; `git ls-remote origin` lists `refs/heads/main`; `main` tracks `origin/main` with no ahead marker. Created 2026-10-21 as `essence-dx/dx-forkness` and **renamed in place** to `essence-dx/forkness` the same day, so the pushed history and the repository settings carried over; the name now matches every sibling repo, and GitHub keeps a redirect from the old one. |
| A release artifact exists | **not true** | `gh release list --repo essence-dx/forkness` → empty. The repository exists; no release has been cut. |
| `dx tool install forkness` works | **not true** | the registry entry resolves (`cli/src/update_registry.rs:52-55`), but there is no release to fetch an asset from, so the house `dx-<tool>-<rust-target-triple>[.exe]` naming still has nothing behind it. |
| The upstream's published gains are reproduced here | **not true** | `python/README.md:41-45` states 11.5 points on WorkBuddy-Bench, 4.5 on JobBench and 11.8 percentage points on LAB's partial pass rate. This tree contains **no run artifacts**, so those figures carry no receipt in it. |

The `forkness/` workspace **has an `origin` remote** — `https://github.com/essence-dx/forkness.git`
— and `main` tracks it. The gate that remains is a **release**: a tagged version whose
assets follow the house `dx-<tool>-<rust-target-triple>[.exe]` naming, so
`dx tool install forkness` resolves an asset.

Do not write "published" or "installable" about `dx forkness` without a
receipt. "Published" is earned now; **"installable" is not**.

One more thing the CI receipt must not be misread for: the workflow ran on a runner
on 2026-10-21, GitHub created all nine jobs, and it then **refused to start any of
them** because the account's Actions billing is unsettled — the same wall that
stopped `essence-dx/bot`'s workflows three days earlier. A red X there is an
account setting, not a defect in the code. It is filed as `BLK-05` in the hub's
`TODO.md`.

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

- **Cut a release.** The repository exists and `main` tracks it; a tagged version
  whose assets follow the house `dx-<tool>-<rust-target-triple>` naming is what
  makes `dx tool install forkness` resolve an asset. Tracked in `cli/TODO.md` under
  "Open: New Tool Release Gaps (grimoire / forkness)".
- **Run the CI workflow on a runner.** The workflow is checked in and its content
  is verified — GitHub created all nine jobs — but the account's Actions billing
  stops them from starting. `BLK-05` in the hub's `TODO.md`.
- **One real benchmark rollout.** The upstream checkouts and their Vertex AI
  credentials are not on this machine, so no rollout has run and no figure the port
  reports has a receipt. Tracked in `forkness/TODO.md` as `FK-025` and `FK-034`.

MIT OR Apache-2.0
updated: 2026-10-21 00:00 UTC
