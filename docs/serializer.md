# DX Serializer Achievements

`serializer` is the DX data-format layer for human-authored config, AI context
packing, machine-readable receipts, manifests, caches, and fast runtime reads.
It gives DX a repository-managed alternative to treating JSON as both interchange
format and runtime data model.

## Position

DX Serializer is a Rust crate named `dx-serializer` with three related formats:
human-edited source files, token-efficient LLM files, and binary `.machine`
files backed by RKYV, optional compression, and memory-mapped access.

Professional positioning:

> DX Serializer gives DX a typed human-to-LLM-to-machine data path: readable
> source files for developers, compact LLM context output for agents, and RKYV
> machine files for fast validated runtime reads.

That positioning is stronger than repeating raw benchmark numbers without context.
The repo owns real benchmark and spec validation, but public speed/token wording
must point back to those reports and name the measured dataset and mode.

## What Serializer Has Already Achieved

| Area | Achievement |
| --- | --- |
| Three-format model | Defines human format as the version-controlled source of canonical behavior, `.llm` as generated AI context format, and `.machine` as generated binary runtime format. |
| LLM syntax | Uses compact key-value, array, object, and wrapped table syntax designed for deterministic parsing and token-efficient AI context windows. |
| Human syntax | Provides TOML/INI-like human files with sections, aligned keys, arrays, nested sections, and readable version-control diffs. |
| Machine format | Uses RKYV-backed archived structures, optional compression, mmap support, and hot-read paths for generated data that should not be reparsed as JSON. |
| Converter surface | Includes JSON, TOML, YAML, and TOON converter modules plus parser, formatter, tokenizer, optimizer, schema, safety, and serializer output modules. |
| Runtime integration | Is consumed by DX CLI, Forge, Check, and other DX tools as the shared serializer crate rather than a one-off script. |
| WASM and watcher paths | Includes optional WASM bindings and file-watch features for future editor/browser integration. |
| Token tooling | Provides optional token counting through tiktoken and Hugging Face tokenizer features. |
| Derive support | Ships an optional `dx-serializer-derive` crate for typed integration. |
| Bench validation | Maintains focused benches for RKYV comparisons, TOON comparisons, machine format performance, mmap, compression, arena, zero-copy, and comprehensive format checks. |
| Test depth | Includes integration, converter, roundtrip, property, parser, tokenizer, error-position, compression, zstd, and format-spec tests. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `serializer/README.md` | Public overview, three-format workflow, measured JSON-vs-machine verification tables, and usage examples. |
| `serializer/LLM_FORMAT_SPEC.md` | LLM format grammar, wrapped dataframe syntax, design decisions, and token-efficiency rationale. |
| `serializer/MACHINE_FORMAT.md` | Machine format tradeoffs, RKYV relationship, compression modes, benchmark context, and honest comparison notes. |
| `serializer/Cargo.toml` | Crate identity, feature flags, converters, compression, mmap, WASM, token-counting, derive, and benchmark targets. |
| `serializer/src/llm/*` | LLM parser, formatter, serializer, token, table, and RKYV conversion logic. |
| `serializer/src/machine/*` | Machine format API, RKYV compatibility, mmap, cache, direct I/O, compression, safe deserialize, and platform I/O modules. |
| `serializer/src/converters/*` | JSON, TOML, YAML, and TOON conversion surfaces. |
| `serializer/derive` | Optional derive macro crate. |
| `serializer/tests/*` | Roundtrip, parser, converter, property, and hardening tests. |
| `serializer/benches/*` | Benchmark ownership for any speed, size, or token-efficiency statement. |

## Public Positioning Boundary

Safe internal statement:

> DX Serializer provides a repository-managed data pipeline from editable human
> syntax to token-efficient LLM context and RKYV-backed machine files, with
> converters, tests, specs, and benchmark validation in the repo.

Safe public wording after refreshing benchmarks:

> On the named benchmark fixtures in the current repo, generated `.machine`
> files avoid JSON reparsing and the `.llm` format reduces context size versus
> JSON according to the refreshed Serializer reports.

Unsafe statement without more verification:

> DX Serializer is universally faster, smaller, safer, and more mature than
> every serialization format in every workload.

The strongest current result is data-path ownership. The remaining verification work is
fresh benchmark runs for the exact public wording, schema-evolution policy,
cross-language boundaries, and ensuring generated `.llm` / `.machine` artifacts
stay out of source archives unless a repo explicitly tracks them.

## Next Documentation Pass

The `style` page now documents the CSS scanning and generation layer that
consumes source data and feeds DX WWW, Zed, and Web Preview UI workflows. The
next folder should be `check`, because Check is the validation and receipt
engine that should consume Style, Forge, WWW, CLI, and Zed verification surfaces.
