# DX Native Achievements

`native` is the DX native app workspace. It keeps Tauri/native-shell research
available through default-off DX serializer `.machine` caches, read-only timing
mode, mmap coverage, command/helper benchmark receipts, and release-boundary guards.

## Position

DX Native is the upstream-derived native app workspace. Its achievement is keeping
Tauri/native-shell research available without turning the hub into a monorepo,
while providing repository-managed benchmark receipts and release-boundary guards.

Professional positioning:

> DX Native is a repository-managed native app workspace that preserves Tauri/native-shell
> research with default-off serializer caches, read-only timing mode, mmap coverage,
> command/helper benchmark receipts, and release-boundary guards.

That positioning intentionally avoids saying the Tauri runtime is production-ready for
all DX workflows. Native provides the research and validation infrastructure, while
runtime verification and default-on wording remain blocked.

## What DX Native Has Already Achieved

| Area | Achievement |
| --- | --- |
| Tauri research | Keeps Tauri/native-shell research available through default-off DX serializer `.machine` caches. |
| Timing mode | Uses read-only timing mode for benchmark validation without mutating repository state. |
| Mmap coverage | Provides memory-mapped file coverage for serializer cache validation and hot-read performance verification. |
| Benchmark receipts | Maintains command/helper benchmark receipts with governed receipt policy. |
| Release-boundary guards | Keeps release-boundary guards that block product-wide speed and default-on wording until verified. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `native/README.md` | Public native workspace overview and research status. |
| `native/PLAN.md` | Forward feature plan for native app research and release governance. |
| `native/crates/tauri-cli/ENVIRONMENT_VARIABLES.md` | Tauri CLI environment variable documentation. |
| `docs/achievements/native.md` | This documentation page. |

## Public Positioning Boundary

Safe internal statement:

> DX Native keeps Tauri/native-shell research available with default-off serializer caches, read-only timing mode, mmap coverage, benchmark receipts, and release-boundary guards.

Safe public wording after refreshed checks:

> The current DX native benchmark suite verifies serializer cache performance, mmap coverage, and release-boundary guards for the documented supported targets.

Unsafe statement without more verification:

> DX Native is a fully production-ready Tauri replacement with complete benchmark coverage and release-qualified validation.

The strongest current result is repository-managed native research. The remaining verification work is full runtime adoption, current per-repo receipts, release CI, and product-specific validation.

## Next Documentation Pass

The `native` page now documents the native app workspace that keeps Tauri research available. The next folder should be `desktop`, because the desktop editor surface is the primary consumer of native runtime capabilities.