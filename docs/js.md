# DX JS Achievements

`js` is the DX-retained Bun workspace. It keeps JavaScript runtime, package
manager, resolver, bundler, test-runner, and toolchain research available while
DX explores narrow `.machine` cache paths inside an upstream-derived runtime.

## Position

JS should be the JavaScript runtime research workstream for DX: a place to prove
specific resolver and package-metadata improvements with local receipts without
claiming broad superiority over upstream Bun or every runtime scenario.

Professional positioning:

> JS is a Bun-derived workspace with a scoped package-metadata `.machine` cache
> experiment, same-binary local JSON versus local machine benchmark validation,
> release-binary verification metadata, DX contract scripts, and disabled CI policy.

That positioning is intentionally narrow. It does not say the full Bun runtime,
package manager, bundler, test runner, or official upstream comparison is
proven faster.

## What JS Has Already Achieved

| Area | Achievement |
| --- | --- |
| Upstream-derived workspace | Retains a Bun fork with `origin` pointing to the DX fork and `upstream` pointing to the official Bun repository. |
| Scoped cache target | Focuses the completed `.machine` work on package metadata reads inside `node_modules`, where package JSON parsing can be replaced by generated machine metadata. |
| Fallback model | Preserves existing JSON behavior when cache entries are missing, invalid, unsupported, or stale. |
| Same-binary benchmark | Compares `local-json` against `local-machine` using the same local release binary, avoiding official-release drift in this branch's verification target. |
| Benchmark constraints | Excludes `.machine` generation and process startup from timed measurements, uses TypeScript fixtures, and keeps verification logging opt-in. |
| Recorded benchmark cases | Records small, medium, and large fixture cases with package JSON totals of 132,022 bytes, 2,099,346 bytes, and 16,781,714 bytes. |
| Measured local wins | Records `local-machine` trimmed means of 7.383 ms, 13.940 ms, and 27.903 ms versus local JSON trimmed means of 14.143 ms, 30.103 ms, and 70.668 ms for the three benchmark cases. |
| Hit-verification counters | Records machine-cache hit verification of 64/64, 128/128, and 256/256 package metadata reads for the small, medium, and large cases. |
| Release verification artifact | Keeps a release verification binary at `build/release-proof-a3bf895c944e-20260601-151841/bun.exe`, observed at 76,847,616 bytes in this checkout. |
| DX script surface | Adds `dx:contracts`, `dx:lighthouse:contract`, `dx:bench:features`, and `dx:bench:local-json-machine` scripts to the package metadata. |
| Lighthouse contract boundary | Keeps Lighthouse scoring metadata blocked by design until official Lighthouse execution, Chrome/LHR verification, and equivalence receipts exist. |
| Disabled CI policy | Keeps workflow files disabled while the current operating rule is to keep GitHub Actions paused. |

## validation To Check

Use these files and artifacts before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `js/README.md` | Upstream Bun context plus the DX package-metadata machine-cache summary and benchmark table. |
| `js/PLAN.md` | Current DX machine-cache status, benchmark scope, operating rules, next work, and completion criteria. |
| `js/package.json` | Workspace metadata, build/test scripts, and DX contract/benchmark scripts. |
| `js/scripts/dx-local-json-vs-machine-benchmark.ts` | Benchmark harness, fixture generation, local binary freshness checks, timing policy, verification counters, and output writing. |
| `js/.tmp/dx-local-json-vs-machine-benchmark-summary.md` | Human-readable benchmark summary. Observed snapshot: generated 2026-06-01, 40 runs, 8 warmups, generation and process startup excluded. |
| `js/.tmp/dx-local-json-vs-machine-benchmark-results.json` | JSON validation for local binary SHA, verification commit, fixture sizes, per-case samples, deltas, and machine-hit counters. |
| `js/build/release-proof-a3bf895c944e-20260601-151841/bun.exe` | Release verification binary referenced by the benchmark artifacts; present in this checkout. |
| `js/scripts/dx-lighthouse-runtime-contract.ts` | Metadata-only Lighthouse contract that reports `not_ready` until real runtime and equivalence verification exists. |
| `js/scripts/dx-*.test.ts` | Contract tests for DX scripts and benchmark/contract routing. |
| `js/.github/workflows/README.md` | Disabled GitHub Actions policy. |
| `bun ./scripts/dx-local-json-vs-machine-benchmark.ts` or `bun test --timeout 30000 ./scripts/dx-*.test.ts` | Runtime authority when the machine has enough headroom to refresh benchmark or contract verification. |

## Public Positioning Boundary

Safe internal statement:

> JS proves a same-binary local package-metadata cache experiment where
> `.machine` reads beat the same binary's JSON metadata path on the recorded
> small, medium, and large TypeScript fixture cases.

Safe public wording after refreshed checks:

> A current benchmark can report fixture-local package-metadata speedups only
> when it names the commit, binary hash, fixture sizes, run/warmup counts,
> timing exclusions, and machine-hit counters.

Unsafe statement without more verification:

> DX JS is faster than Bun overall, official upstream Bun was beaten, the
> package manager or bundler is globally faster, Lighthouse scoring is ready, or
> the old benchmark commit proves the current `dev` head.

The strongest current result is the scoped resolver/package-metadata validation.
The remaining verification work is refreshing the benchmark at current HEAD, promoting
only stable artifacts out of `.tmp`, and keeping Lighthouse/package-runtime
contracts blocked until they execute real official flows.

## Documentation Status

The `native` documentation pass is complete. The next folder should be `tools`,
because it needs a clear source-versus-artifact boundary before the achievement
register is complete.
