# DX JS Achievements

`js` is the DX JavaScript runtime workspace. It keeps Bun/runtime research
available with a scoped package-metadata `.machine` cache experiment,
same-binary local JSON versus local machine benchmark validation, and
DX contract scripts.

## Position

DX JS is the upstream-derived JavaScript runtime workspace. Its achievement
is keeping Bun/runtime research available without turning the hub into a
monorepo, while providing repository-managed benchmark validation and
DX contract scripts.

Professional positioning:

> DX JS is a repository-managed JavaScript runtime workspace that preserves
> Bun/runtime research with scoped package-metadata caching, local benchmark
> validation, and DX contract scripts.

That positioning intentionally avoids saying the Bun runtime is production-ready
for all DX workflows. JS provides the research and validation infrastructure,
while runtime verification remains deferred.

## What DX JS Has Already Achieved

| Area | Achievement |
| --- | --- |
| Bun runtime research | Keeps Bun/runtime research available with a scoped package-metadata `.machine` cache experiment. |
| Benchmark validation | Provides same-binary local JSON versus local machine benchmark validation with DX contract scripts. |
| Contract scripts | Maintains DX contract scripts that check source contracts, UI wording, receipt fields, fanout boundaries, and repository-level wording hygiene. |
| Package-metadata caching | Uses `.machine` cache design for package-metadata reads with validated metadata reads and governed benchmark receipt policy. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `js/PLAN.md` | Forward feature plan for JS runtime research and benchmark governance. |
| `js/.tmp/dx-local-json-vs-machine-benchmark-summary.md` | Local benchmark summary with fixture-local wording. |
| `js/scripts/dx-local-json-vs-machine-benchmark.ts` | Benchmark script for local JSON versus machine validation. |
| `docs/achievements/js.md` | This documentation page. |

## Public Positioning Boundary

Safe internal statement:

> DX JS keeps Bun/runtime research available with scoped package-metadata caching, local benchmark validation, and DX contract scripts.

Safe public wording after refreshed checks:

> The current DX JS benchmark suite verifies local JSON versus local machine validation for the documented supported targets.

Unsafe statement without more verification:

> DX JS is a fully production-ready Bun runtime replacement with complete benchmark coverage and release-qualified validation.

The strongest current result is repository-managed benchmark research. The remaining verification work is full runtime adoption, current per-repo receipts, release CI, and product-specific validation.

## Next Documentation Pass

The `js` page now documents the JavaScript runtime workspace that keeps Bun research available. The next folder should be `native`, because native app research is the complementary upstream-derived workspace.