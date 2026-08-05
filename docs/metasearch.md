# DX Metasearch Achievements

`metasearch` is the Rust search workspace for reusable query/result types,
engine adapters, an Axum server, and a CLI launcher. Its strongest achievement
is organizing a large search-provider surface behind explicit repository-managed
runtime, operator, and provider-health boundaries.

## Position

DX Metasearch should be the search layer for DX: reusable core types, a large
engine registry, cache-aware orchestration, health-aware server routes, and
operator-visible runtime status. It can support search inside DX tools without
turning adapter count into an inflated live-provider statement.

Professional positioning:

> DX Metasearch is a Rust workspace with shared search models, a large adapter
> registry, an Axum search/API server, cache and request coalescing, engine
> health tracking, OpenSearch support, crawler controls, and explicit operator
> status surfaces.

That statement intentionally separates implemented repository scope from deployed
service status. Operators still own TLS/proxy setup, environment-specific
rate limits, API keys or base URLs, CORS allowlists, and selected live-provider
probes.

## What Metasearch Has Already Achieved

| Area | Achievement |
| --- | --- |
| Rust workspace shape | Splits shared query/result/config/engine traits, adapter registry, Axum server, and CLI launcher into `metasearch-core`, `metasearch-engine`, `metasearch-server`, and `metasearch-cli`. |
| Engine registry | Loads a large built-in adapter catalog through `EngineRegistry::with_defaults(...)` while keeping adapter coverage separate from live upstream health. |
| Search orchestration | Owns cache-aware fan-out, in-flight request coalescing, engine health tracking, adaptive per-engine timeouts, explicit engine targeting, and bounded input normalization. |
| Web/API surfaces | Mounts HTML search, JSON search API, engine catalog, runtime config, operator status, autocomplete, OpenSearch, static assets, health, liveness, and status routes. |
| Query state honesty | Carries normalized category, language, page, safe-search, time-range, and explicit engine state through cache keys, HTML searches, and JSON responses. |
| Operator status | Exposes runtime warnings, provider summaries, skipped adapters, engine health snapshots, asset warnings, and deployment posture through JSON and HTML status surfaces. |
| status probes | Provides `/health`, `/livez`, and `/readyz`; status reports usable search capacity and provider verification status instead of pretending every adapter is live-probed. |
| Provider access model | Documents no-key, rate-limited, optional-key, required-key, self-hosted, brittle-scraper, disabled, and not-acceptable provider classes. |
| Security/default posture | Ships same-origin CORS by default, opt-in permissive CORS, explicit origin allowlists, response hardening headers, noindex/noarchive crawler controls, and local browser autocomplete. |
| Runtime asset validation | Validates template/static roots and required template/static files before binding the HTTP port, with browser-visible asset-integrity warnings. |
| Container/operator alignment | Documents runtime layout, non-root container assumptions, status-based healthchecks, graceful shutdown, and `SIGTERM` stop behavior. |
| CLI release path | Provides `metasearch serve`, config loading, host/port/template/static overrides, engine listing, and config inspection paths. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `metasearch/README.md` | Workspace purpose, supported routes, production-facing behavior, config, CLI, container notes, and operator notes. |
| `metasearch/docs/PRODUCTION_READY.md` | Source-state status scope and explicit operator-owned responsibilities. |
| `metasearch/docs/PROVIDER_ACCESS_MODELS.md` | Adapter coverage versus live-provider health boundary and provider access classifications. |
| `metasearch/TODO.md` | Remaining source follow-ups, including richer rate limiting, per-engine config, offline catalog generation, and scraper fixtures. |
| `metasearch/Cargo.toml` | Workspace members, Rust version, web/search/cache/performance dependencies, and stale upstream repository metadata to clean before publication. |
| `metasearch/crates/metasearch-core/src/*.rs` | Shared query, result, category, config, ranking, and engine trait types. |
| `metasearch/crates/metasearch-engine/src/registry.rs` | Built-in adapter registry and adapter metadata surface. |
| `metasearch/crates/metasearch-server/src/app.rs` | Route mounting, middleware, CORS, security layers, asset validation, and graceful shutdown. |
| `metasearch/crates/metasearch-server/src/routes/api.rs` | JSON search, engine catalog, runtime config, and operator status payloads. |
| `metasearch/crates/metasearch-server/src/routes/health.rs` | `/health`, `/livez`, and `/readyz` behavior plus provider status. |
| `metasearch/crates/metasearch-server/src/state.rs` | Runtime warnings, asset warnings, and required asset validation. |
| `metasearch probe --allow-network --engines <ids> --query <query>` | Selective live-provider verification when public wording requires current upstream validation. |
| `cargo test` in `metasearch` | Runtime authority when current source behavior needs to be proven. |

## Public Positioning Boundary

Safe internal statement:

> DX Metasearch owns a Rust search workspace with reusable models, engine
> adapters, Axum server routes, cache-aware orchestration, runtime warnings,
> provider metadata, and operator status endpoints.

Safe public wording after refreshed checks:

> The current metasearch source state supports the documented search/API/status
> routes, status probes, same-origin default posture, provider access
> metadata, and selected live-provider checks for the engines that were freshly
> probed.

Unsafe statement without more verification:

> Every registered search adapter is configured, reachable, live-proven,
> rate-limit-safe, deployment-ready, and superior to every existing metasearch
> service.

The strongest current result is implemented search infrastructure for this repo
scope. The remaining verification work is current build/test validation, selected live
provider probes, stale path/repository metadata cleanup, per-engine config
loading, generated offline engine catalogs, and scraper regression fixtures.

## Documentation Status

The `py` page now documents the Python workspace's CPython worktrees,
package-manager research, benchmarks, manifests, receipts, and archives without
flattening everything into one Git root. The next folder should be `providers`,
because the AI provider catalog has binary catalog, metadata sidecar, adapter,
and auth-boundary wording that must stay separate from live-provider verification.
