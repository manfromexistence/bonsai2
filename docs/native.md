# DX Native Achievements

`native` is the DX-retained Tauri workspace. It keeps desktop and native-shell
runtime research available while DX tests narrow serializer-cache improvements
inside selected Tauri CLI and config hot paths.

## Position

Native should be the desktop and multi-platform shell research workstream for DX: a
place to prove focused Tauri CLI/config improvements without claiming the full
Tauri product, app runtime, WebView startup, IPC, build, bundle, installer, or
mobile/toolchain path is faster.

Professional positioning:

> Native is a Tauri-derived workspace with default-off DX serializer `.machine`
> caches for selected CLI/config hot paths, read-only timing support, mmap
> coverage, surface-matrix guards, release-boundary checks, and record-backed
> narrow command/helper validation.

That positioning is intentionally scoped. It says DX has real cache-path validation in
the changed surfaces. It does not say the fork is publish-ready, default-on, or
generally faster than upstream Tauri.

## What Native Has Already Achieved

| Area | Achievement |
| --- | --- |
| Upstream-derived workspace | Retains a Tauri fork with `origin` pointing to the DX fork and `upstream` pointing to the official Tauri repository. |
| Default-off feature gate | Keeps DX machine-cache behavior behind `dx-machine-cache` and `dx-machine-cache-mmap` features instead of enabling it in normal default builds. |
| Source-authoritative cache policy | Treats JSON, TOML, Cargo, and Tauri config sources as authoritative while `.machine` files remain disposable validated accelerators. |
| Read-only timing mode | Documents `TAURI_DX_MACHINE_CACHE_WRITE=0` so benchmark runs can use pre-generated sidecars without mutating cache artifacts during timed samples. |
| Official-release command validation | Records fixture-local command comparisons against official Tauri `v2.11.2`: `inspect wix-upgrade-code` at `50.854 ms` local cache-on versus `155.504 ms` official, and `migrate-stable-v2-noop` at `41.027 ms` local cache-on versus `94.448 ms` official. |
| Current-fork hot-path validation | Records parser-versus-machine wins for package manifest and lock metadata (`17.470x`), watch-folder discovery/Cargo projection (`13.708x`), `cargo metadata --no-deps` cache reads (`11.317x`), and package-version resolution (`4.978x`). |
| Negative-result discipline | Keeps tiny project-config and leaf-config negative timing receipts visible, and scopes the representative generated config result to one fixture-local `2.22x` result. |
| Mmap and direct-archive coverage | Keeps focused guard coverage for mmap-enabled cache readers and direct archived-field reads rather than relying only on full archived-tree materialization. |
| Surface-matrix governance | Maintains a central machine-cache surface matrix and guard suite so new benchmark surfaces must be named, bounded, and checked. |
| Release boundary | Fails closed around the local `dx-serializer` path dependency, Rust 1.85 / edition 2024 mismatch, cargo publication boundary, and default-on wording. |
| Disabled CI policy | Keeps workflow files disabled while the current operating rule is to keep GitHub Actions paused. |

## validation To Check

Use these files before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `native/README.md` | Upstream Tauri context plus the DX serializer performance fork summary and the headline measured surfaces. |
| `native/PLAN.md` | Stage-by-stage receipt history, benchmark boundaries, negative results, release blockers, and statement limits. |
| `native/crates/tauri-cli/ENVIRONMENT_VARIABLES.md` | Public operator documentation for `TAURI_DX_MACHINE_CACHE`, `TAURI_DX_MACHINE_CACHE_WRITE`, feature gates, and publish/default-on caveats. |
| `native/.scripts/ci/test-dx-machine-cache-release-boundary.ts` | TypeScript guard that checks serializer dependency metadata, default feature isolation, workflow publish boundaries, and docs/plan wording. |
| `native/.scripts/ci/test-dx-machine-cache-*.ts` | Focused guard suite for docs, surface matrix, projection validation, source matching, read-only mode, mmap coverage, MSRV boundary, and info app/config behavior. |
| `native/package.json` | Tauri workspace package metadata and existing script surface. |
| `native/.github/workflows/README.md` | Disabled GitHub Actions policy for this workspace. |
| `G:\Dx\test-outputs` | Receipt root named by `native/PLAN.md`; it was not present during this documentation pass, so receipt paths in the plan should be treated as recorded validation references until restored. |
| `node .scripts/ci/test-dx-machine-cache-release-boundary.ts` | Focused command to refresh release-boundary verification when the machine has enough headroom. |

## Public Positioning Boundary

Safe internal statement:

> Native has a focused, default-off DX serializer-cache layer that records real
> command/helper wins for selected Tauri CLI/config hot paths while preserving
> source files as the authority.

Safe public wording after refreshed checks:

> A current native statement may name the exact measured fixture, command/helper
> surface, commit or receipt source, cache-generation exclusion, write-disabled
> timing mode, and official-release or same-fork comparison boundary.

Unsafe statement without more verification:

> DX Native is faster than Tauri overall, the fork is default-on or publish-ready,
> app runtime is faster, WebView startup is faster, IPC is faster, `tauri dev`,
> build, bundle, watch, installer, mobile, or toolchain workflows are faster, or
> `.machine` makes every config read faster.

The strongest current achievement is disciplined performance research: narrow
hot-path wins are recorded, negative results are preserved, and release wording
are intentionally blocked until serializer packaging, MSRV, audit, and default
feature validation are solved.

## Documentation Status

The `tools` documentation pass is complete. Future native documentation should
focus on refreshed receipts, serializer packaging status, and any new Tauri
runtime surfaces that gain focused verification.
