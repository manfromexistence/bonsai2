# DX Build Achievements

`build` is the DX-retained Rolldown workspace. It keeps upstream JavaScript and
TypeScript bundler research close to DX while isolating upstream-derived build
tooling from the hub repository.

## Position

Build should be the bundler and package-build research workstream for DX: a production-focused
Rust/Node monorepo fork where DX can test repository-managed machine-cache ideas,
benchmark governance, and package-contract receipts without mixing those wording
into the main platform hub.

Professional positioning:

> Build is a Rolldown-derived Rust and pnpm workspace with an opt-in DX
> `.machine` cache design, validated machine metadata reads, package-json cache
> integration, and governed benchmark receipt policy.

That positioning is intentionally narrower than the timing table in the README. The
current local source receipt says the benchmark was not run, upstream was not
measured, and `speedupClaim` is `none`.

## What Build Has Already Achieved

| Area | Achievement |
| --- | --- |
| Upstream-derived workspace | Retains a Rolldown fork with `origin` pointing to the DX fork and `upstream` pointing to the official Rolldown repository. |
| Rust/pnpm monorepo | Keeps a large Rust workspace, pnpm workspace, package scripts, native binding packages, benchmark package, docs package, and Vite/Rollup-compatible bundler work in one research repo. |
| Opt-in machine cache | Documents `ROLLDOWN_DX_JSON_CACHE` as the default-off gate for project-local `.dx/rolldown/*.machine` read models. |
| Source-canonical policy | Keeps source files canonical and treats machine artifacts as rebuildable cache files that must fall back on miss, stale source, corruption, unsupported payload, or validation failure. |
| Machine metadata validation | Validates source path, source length, source hash, machine path, machine length, machine hash, cache policy, payload format, and maximum file sizes before accepting a cache hit. |
| Atomic cache writes | Writes machine artifacts and metadata through atomic paths, validates benefit thresholds, and can sync writes by default. |
| Narrow cache surfaces | Targets JSON imports, Vite JSON transforms, resolver/package metadata, optional peer dependency metadata, and selected config-derived data rather than turning every file into a cache candidate. |
| Package-json cache integration | Adds package-json machine-cache paths and tests for default-off behavior, borrowed hot lookups, path normalization, and no unnecessary source-byte cloning. |
| Governed benchmark plan | Defines upstream-stock, local-cache-disabled, and local-cache-enabled benchmark arms with required environment, output, cache, and official-baseline validation. |
| repository-level receipt guard | Generates and validates a source receipt that explicitly forbids speedup wording when no benchmark execution receipt exists. |
| Fixture gates | Records selected repository-managed fixture paths and a size gate so future timing runs cannot statement performance from arbitrary or too-small inputs. |
| Disabled CI policy | Keeps workflow files disabled while the current operating rule is to keep GitHub Actions paused. |

## validation To Check

Use these files and receipts before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `build/README.md` | Upstream Rolldown context plus the DX benchmark narrative. Treat the timing table as historical unless the verification log and current receipt are refreshed. |
| `build/package.json` | pnpm workspace scripts, package-manager version, and DX package-contract script. |
| `build/Cargo.toml` | Rust workspace membership, strict clippy policy, crate graph, and release profiles. |
| `build/meta/design/dx-machine-cache.md` | Source-canonical cache policy, opt-in environment boundary, target surfaces, and benchmark result policy. |
| `build/crates/rolldown_utils/src/dx_machine_cache.rs` | Core machine-cache implementation, metadata parser, validation rules, atomic writes, cache status, and tests. |
| `build/crates/rolldown/src/utils/dx_machine_cache.rs` | Rolldown crate re-export of the shared machine-cache implementation. |
| `build/crates/rolldown_plugin_vite_resolve/src/package_json_cache.rs` | Package-json optional peer dependency cache integration. |
| `build/crates/rolldown_resolver/tests/package_json_machine_cache.rs` | Source tests for default-off, borrowed lookup, path normalization, and cache opt-in behavior. |
| `build/packages/bench/receipt/source-receipt.ts` | repository-level benchmark receipt contract and validation. |
| `build/.dx/rolldown/benchmark-source-receipt.json` | Local receipt snapshot. The observed snapshot says `benchmarkStatus` is `not_run`, `upstreamComparison` is `not_measured`, `speedupClaim` is `none`, and benchmark execution is false. |
| `G:\Dx\rolldown-benchmarks\run-20260531-current-governed\logs\current-governed-official-vs-local-18x1280-now-30samples.log` | README verification-log path. It was not present when checked from this documentation pass. |
| `build/.github/workflows/README.md` | Disabled GitHub Actions policy. |
| Targeted Rust tests, package receipt validation, and a governed benchmark run | Runtime authority when cache correctness or speed wording needs fresh verification. |

## Public Positioning Boundary

Safe internal statement:

> Build keeps a Rolldown-derived workspace where DX can experiment with a
> source-canonical `.machine` cache, validated metadata reads, package-json
> cache integration, and governed benchmark receipt policy.

Safe public wording after refreshed checks:

> A current governed benchmark receipt can report speed only when it includes
> official baseline validation, local cache-disabled and cache-enabled arms,
> matching output hashes, pre-generated machine-artifact verification, environment
> metadata, and a passing speedup gate.

Unsafe statement without more verification:

> The current checkout proves the README timing table, beats official Rolldown,
> has current upstream comparison validation, or can publish a speedup result from
> the repository-level receipt.

The strongest current result is governance and architecture. The remaining verification
work is regenerating a clean benchmark receipt at the current `G:\Dx\build`
path, preserving verification logs, proving output equivalence, and rerunning targeted
cache tests when the machine has enough headroom.

## Documentation Status

The `js` documentation pass is complete. The next folder should be `native`,
because it is the upstream-derived Tauri workspace where DX serializer-cache
validation needs the same scoped wording.
