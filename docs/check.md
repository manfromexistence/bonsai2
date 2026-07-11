# DX Check Achievements

`check` is the Rust validation and scoring engine for DX repository control,
adapter planning, rule-pack evaluation, and web-audit validation. Its strongest
achievement is turning "is this codebase healthy?" into repository-managed data,
diagnostics, and receipts instead of loose human opinion.

## Position

DX Check is the quality gate that should eventually read validation from WWW,
Style, Forge, Serializer, Zed, CLI, and other DX tools. It already owns the
core model for scoring, diagnostics, repository-managed inventory, adapter planning,
rule-pack loading, and web-audit import.

Professional positioning:

> DX Check is a Rust source-health engine that scores repository-managed project
> structure, tests, generated artifacts, naming, security defaults, adapter
> safety, rule-pack provenance, and web-audit validation with explicit diagnostic
> boundaries.

That statement intentionally avoids saying every repository is automatically
production-ready. Check measures and reports validation; it does not replace
runtime testing, security review, release QA, or product-specific acceptance.

## What Check Has Already Achieved

| Area | Achievement |
| --- | --- |
| Rust scoring engine | Defines `dx-check-engine` as a Rust library with a 500-point score model, report summary, diagnostics, rule-pack categories, and repository-managed checked paths. |
| Source inventory | Scans source and config files while ignoring generated DX receipts, serializer caches, dependency validation, and case variants that should not be treated as hand-authored source. |
| Adapter planning | Plans format, lint, typecheck, test, and web-audit tools for Rust, JavaScript, TypeScript, Python, Go, C, C++, and web targets with conservative blocking behavior for unsafe scripts. |
| C/C++ checks | Documents clang-format, clang-tidy, cppcheck, clangd, CMake build, and ctest target support through the extensionless `dx` manifest. |
| JavaScript safety | Rejects write-risk format scripts, chained delegations, invalid Biome configuration, and configured targets that cannot be proven locally. |
| Python adapter honesty | Supports Ruff and Black check-mode planning without pretending an unconfigured formatter is available. |
| Rule-pack system | Loads local and registry-backed rule packs, validates rule-pack identity and row shape, records category diagnostics, and supports strict local scope. |
| Registry trust | Tracks rule-pack lock metadata, BLAKE3 hashes, provenance, and strict Ed25519 signature verification paths for signed rule packs. |
| Default rule docs | Owns docs for AI-maintainable structure, generated-source leaks, file-size budgets, component budgets, naming, Rust source shape, security markers, repository-managed dependency boundaries, and test status. |
| Web-audit workstream | Provides a `dx-check-web-audit` binary, HTTP HTML metadata runner, native Lighthouse-shaped report support, official Lighthouse JSON import, timeout handling, serializer result import, and a gated JS Lighthouse runtime receipt. |
| Litehouse crate | Contains `dx-litehouse` as an internal crate for lightweight Lighthouse-shaped import/model/report work without making the whole engine depend only on external JS tooling. |
| Test coverage surface | Keeps focused tests for adapters, diagnostics, C-family handling, JavaScript safety, Litehouse, rule-pack trust, web-audit runtime, and engine reports. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `check/Cargo.toml` | Crate identity, Rust edition, serializer dependency, Litehouse dependency, signature/hash dependencies, and `dx-check-web-audit` binary. |
| `check/dx` | repository-managed manifest for supported targets, score maximum, C/C++ tools, and web-audit capability status. |
| `check/src/lib.rs` | Main analysis flow for inventory, rules, diagnostics, web-audit data, adapter plans, and score summaries. |
| `check/src/inventory.rs` | repository-managed file scanning and generated DX artifact exclusions. |
| `check/src/adapters/*.rs` | Adapter planning and safety gates for language/tool targets. |
| `check/src/rule_pack*.rs` and `check/src/rules/*.rs` | Rule-pack parsing, loading, validation, default rule evaluation, and category diagnostics. |
| `check/src/registry/signature.rs` | Ed25519 signature payload and strict verification boundary. |
| `check/src/web_audit*.rs` | Web-audit loading, runtime, equivalence, package verification, and Lighthouse-shaped validation paths. |
| `check/crates/dx-litehouse` | Lightweight Lighthouse import/model/report crate. |
| `check/docs/check/*.md` | Default rule documentation and user-facing fix guidance. |
| `check/tests/*.rs` | Regression surface for adapters, diagnostics, Litehouse, rule-pack trust, and web-audit runtime. |
| `cargo test` in `check` | Runtime authority when current engine behavior needs to be proven. |

## Public Positioning Boundary

Safe internal statement:

> DX Check owns a Rust scoring and diagnostics engine for repository-managed project
> health, rule-pack validation, adapter safety, generated-artifact boundaries, and
> web-audit import.

Safe public wording after refreshed checks:

> The current `dx-check-engine` test suite verifies adapter planning,
> diagnostics, rule-pack validation, trust metadata, source inventory filters,
> Litehouse imports, and web-audit runtime paths for the documented supported
> targets.

Unsafe statement without more verification:

> DX Check fully proves that every DX folder is production-ready, secure,
> release-qualified, benchmark-superior, and runtime-verified.

The strongest current result is measured source-health validation. The remaining
verification work is full ecosystem-wide adoption, current per-repo receipts, release
CI, hosted audit validation, and product-specific security/release review.

## Documentation Status

The `media` page now documents the tool catalog, receipt, dependency,
credential, and provenance concepts that should be connected to Check without
overstating runtime status. The next folder should be `metasearch`, because
it has query/result metadata, server endpoints, cache, and search-provider
status wording that need the same validation boundary.
