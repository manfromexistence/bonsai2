# DX Providers Achievements

`providers` is the Rust AI provider catalog and CLI boundary for DX. It keeps
local provider/model data, runtime provider metadata, auth requirements, aliases,
and catalog validation in source instead of scattering model access knowledge
across prompts and one-off scripts.

## Position

DX Providers should be the provider intelligence layer for the DX CLI, agent
runtime, and future Friday work: know what providers exist, how they are
identified, which adapters are implemented, which are catalog-only, what auth is
required, and which wording needs live verification.

Professional positioning:

> DX Providers is a Rust provider catalog CLI with a checked-in rkyv/memmap
> provider snapshot, provider metadata sidecar, alias resolution, catalog
> validation, freemium/auth boundaries, and multiple runtime adapter
> implementations.

That statement intentionally avoids saying all providers are live-connectable. The
local catalog snapshot, metadata sidecar, runtime adapters, and exposure-status
fields are source validation; live auth flows, network calls, quotas, and current
official provider coverage require fresh verification.

## What Providers Has Already Achieved

| Area | Achievement |
| --- | --- |
| Binary catalog | Stores provider/model data in `data/providers.rkyv` and loads it through rkyv validation plus memory mapping. |
| Catalog validation | Validates archive metadata, provider counts, model counts, provider source labels, duplicate provider IDs, duplicate model IDs, timestamp shape, version shape, and OpenCode Zen free-model coverage. |
| Catalog maintenance commands | Provides `catalog validate`, `catalog refresh-freemium-models`, and `catalog normalize` style flows through the catalog command module. |
| Provider metadata sidecar | Exports `data/provider-metadata.generated.json` with schema, source commit, provider count, alias count, content hash, redaction statement, identities, freemium metadata, and alias index. |
| Secret boundary | States that generated metadata contains identifiers, model IDs, and environment variable names only, not credential values, tokens, cookies, or API keys. |
| Exposure statuses | Separates `verified_working`, `implemented`, `pending_backend`, and `catalog_only` provider states instead of treating every catalog entry as runtime-ready. |
| Auth vocabulary | Models browser sessions, local runtime, OAuth, public key, API key, cloud account, free tier, free credits, premium account, paid, and catalog-only access. |
| Alias resolution | Resolves aliases, runtime IDs, and database IDs back to canonical provider metadata through repository-managed normalization. |
| Runtime adapters | Contains provider code for OpenAI-compatible providers, Gemini/OAuth, Qwen, Cloudflare, OpenCode Zen, and other implemented provider surfaces. |
| Web-session placeholders | Tracks ChatGPT, Codex, Gemini, Antigravity, and Claude web surfaces as pending-backend or browser-session boundaries instead of claiming invisible automation is done. |
| Large integration snapshots | Keeps Gemini CLI and Qwen Code snapshots as tracked source references, while avoiding submodule ambiguity. |
| Workflow policy | Keeps GitHub Actions disabled in-repo with an explicit README so CI is not accidentally re-enabled. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `providers/README.md` | Public catalog numbers, runtime-provider list, command examples, status, and current caution about refreshing official coverage. |
| `providers/Cargo.toml` | Rust crate identity, CLI dependencies, provider/auth dependencies, rkyv/memmap dependencies, and current package metadata. |
| `providers/data/providers.rkyv` | Checked-in binary provider/model catalog snapshot. |
| `providers/data/provider-metadata.generated.json` | Generated metadata sidecar with provider/alias counts, source commit, redaction statement, exposure statuses, auth, env var names, and aliases. |
| `providers/src/providers/rkyv_loader.rs` | rkyv archive structs, memory-map loading, validation, malformed archive rejection, and catalog read/write helpers. |
| `providers/src/catalog_archive.rs` | Catalog validation, normalize, freemium refresh, provider-scoped model IDs, and OpenCode Zen coverage checks. |
| `providers/src/provider_metadata.rs` | Canonical provider metadata, auth/access/exposure enums, alias lookup, free model IDs, and identifier uniqueness tests. |
| `providers/src/providers/*.rs` | Runtime provider adapter implementations and placeholder boundaries. |
| `providers/integrations/gemini-cli` and `providers/integrations/qwen-code` | Tracked integration snapshots; treat as source references, not automatically verified runtime integrations. |
| `providers/.github/workflows/README.md` | Disabled GitHub Actions policy. |
| `providers catalog validate` | Runtime authority for the current checked-in catalog when verification is allowed. |

## Public Positioning Boundary

Safe internal statement:

> DX Providers owns a Rust AI provider catalog with binary archive loading,
> repository-managed provider metadata, auth/exposure boundaries, alias resolution,
> catalog validation, and multiple adapter implementations.

Safe public wording after refreshed checks:

> The current Providers CLI validates the checked-in catalog snapshot, resolves
> the documented metadata aliases, and exposes the providers whose runtime
> adapters were freshly tested with valid credentials or documented no-key
> access.

Unsafe statement without more verification:

> All 184 catalog providers, every listed model, and every runtime provider are
> live, authenticated, quota-valid, up to date with official LiteLLM/Models.dev,
> and verified today.

The strongest current result is repository-managed provider intelligence. The remaining
verification work is live-provider verification, current official-catalog refresh,
credential-safe runtime receipts, cleanup of placeholder license/docs, and
clearer guidance for the large tracked Gemini/Qwen integration snapshots.

## Documentation Status

The `dcp` page now documents the Development Context Protocol's large
source/test surface and protocol/security boundaries without turning source
depth into universal benchmark or production qualification wording. The next folder
should be `driven`, because it is the AI-assisted development orchestrator and
worker coordination receipt system that connects rules, sync, templates, validation,
steering, and DX strategy.
