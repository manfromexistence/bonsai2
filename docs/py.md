# DX Python Achievements

`py` is the DX Python research workspace. It preserves CPython acceleration
workstreams, package-manager research, benchmark tooling, manifests, receipts, and
reports without flattening embedded repositories and worktrees into one
dangerous Git root.

## Position

DX Python is not a finished Python replacement. Its value is that it keeps
experimental CPython acceleration work repository-managed, branch-preserved,
benchmarkable, and honest about what has and has not been proven.

Professional positioning:

> DX Python is an experimental CPython/package-manager workspace with preserved
> worktree branches, repaired worktree metadata, benchmark scripts, native-JIT plans,
> worker receipts, and source-boundary validation for selected acceleration work.

That statement intentionally avoids broad speed or production wording. The benchmark
report says official Python wins broadly in the recorded run; local native-JIT
only wins startup and `class_attr`, and current native paths were not yet
turning hot benchmark loops into broadly better native code.

## What Python Has Already Achieved

| Area | Achievement |
| --- | --- |
| Workspace preservation | Tracks policies, manifests, plans, receipts, reports, and benchmarks while leaving `cpython`, `package-manager`, and workstream worktrees as separate histories. |
| Branch model | Records `main` as release branch, `dev` as integration branch, and keeps GitHub Actions intentionally disabled until a Python-specific CI plan exists. |
| Worktree repair | Repairs stale Git pointer metadata from `G:\Dx\python` to `G:\Dx\py` and records the old/current root boundary in a manifest. |
| CPython fork preservation | Keeps embedded `cpython` clean on `lane/deoptimization` with origin/upstream remotes to the DX fork and upstream CPython. |
| Package-manager research | Keeps embedded `package-manager` clean as a separate `uv` research checkout rather than merging it into the hub repo. |
| worktree branch preservation | Records preserved CPython worker branches for executor formation, simple return backend, planner eligibility, deopt stack safety, range-loop runtime, range IR, and semantic boundaries. |
| Native-JIT plan | Captures the architecture for guarded native range-loop acceleration, deoptimization, diagnostics, Windows build paths, and correctness-first commit boundaries. |
| Honest status scoring | Names the broad native-Python dream as early experimental work and separates that from narrower range-loop milestones. |
| Benchmark harness | Provides a Python comparison driver for official/default/JIT/local/native-JIT scenarios, startup timing, worker tasks, checksums, and CSV output. |
| Benchmark report | Records a fresh comparison where official Python wins broadly and local native-JIT only wins selected startup/class-attribute surfaces. |
| Worker receipts | Preserves workstream receipts with files changed, functionality, commands/log paths, heavy-gate decisions, known risks, remaining work, and conservative score deltas. |
| Source-boundary discipline | Keeps artifacts/logs/receipts separate from CPython source, avoids build output wording, and documents when native-hit verification is missing. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `py/README.md` | Workspace purpose, repository model, branch model, first checks, and safety boundary for embedded repos/worktrees. |
| `py/manifests/workspace-repositories.json` | Embedded repo metadata, repaired worktree pointers, preserved branches, ignored payloads, and GitHub Actions policy. |
| `py/artifacts/plans/2026-05-30-native-cpython-acceleration-plan.md` | Native CPython acceleration architecture, status, risks, tasks, verification commands, and commit hygiene. |
| `py/artifacts/reports/python-speed-comparison-20260530-074441.md` | Recorded benchmark results and the blunt conclusion that official Python wins broadly. |
| `py/benchmarks/compare_python_speed.py` | Benchmark harness, scenarios, task set, startup timing, and CSV summary generation. |
| `py/artifacts/receipts/worker-lane-5-range-loop-runtime.md` | Example worker receipt with build/test verification, skipped native-hit verification, known risks, and remaining work. |
| `git -C py status --short --branch` | Root workspace cleanliness and active branch. |
| `git -C py/cpython status --short --branch` | Embedded CPython fork branch and cleanliness. |
| `git -C py/package-manager status --short --branch` | Embedded package-manager checkout branch and cleanliness. |

## Public Positioning Boundary

Safe internal statement:

> DX Python preserves experimental CPython acceleration and package-manager
> research with repaired worktrees, worktree branches, manifests, benchmark tooling,
> receipts, and honest reports.

Safe public wording after refreshed checks:

> The current Python workspace contains a preserved CPython fork/workstream system and
> benchmark validation for selected native-JIT experiments, with official Python
> still winning the broad recorded benchmark suite.

Unsafe statement without more verification:

> DX Python is a production Python compiler, broadly faster than CPython,
> runtime-native across all important code paths, CI-ready, or release-qualified.

The strongest current result is disciplined experimental preservation. The
remaining verification work is fresh root/embedded-repo status refreshes, current workstream
verification, broad native-hit verification, current benchmarks, source/archive
manifests after any move, and a dedicated Python CI plan when the machine and
repo are ready.

## Documentation Status

The `providers` page now documents the AI provider catalog's binary catalog,
metadata sidecar, adapters, and auth boundaries without confusing source
metadata with live-provider verification. The next folder should be `dcp`,
because the Development Context Protocol has a large source/test surface and
strong protocol/security wording that must stay inside repository-level and
benchmark-verification boundaries.
