# DX Icon Achievements

`icon` is the Rust icon search and catalog workspace for DX. It turns icon
selection into a repository-managed capability with local Iconify-style data packs,
compressed search indexes, CLI commands, optional web serving, and optional WASM
support.

## Position

Icon should be the visual-symbol layer of DX: searchable local icon metadata,
fast index loading, SVG export, pack discovery, and a reusable command surface
for DX-WWW, media tools, editor surfaces, and design workflows.

Professional positioning:

> Icon is a Rust icon search workspace with 229 local icon-pack JSON files,
> compressed zstd index artifacts, CLI search/export/download commands, an
> optional Axum web API, and optional WASM support.

That statement uses facts verified from the current checkout. The broader README
benchmark wording, throughput numbers, and competitor comparisons should be
treated as historical wording until a fresh benchmark run is captured.

## What Icon Has Already Achieved

| Area | Achievement |
| --- | --- |
| Local icon data | Keeps 229 JSON icon-pack files in `data/`, giving DX a local catalog instead of relying only on a remote icon API. |
| Compressed index artifacts | Stores `index/index.fst.zst` and `index/index.meta.zst`, totaling 6,318,053 bytes in the current checkout. |
| Rust crate surface | Exposes search engine, index, parser, search result, icon metadata, icon pack, perfect hash, bloom, precomputed, optimized, SIMD, and zero-allocation modules. |
| CLI commands | Provides an `icon` command for `search`, `export`, `download`, `packs`, and `logo`, with short aliases for operator use. |
| Root discovery | Resolves data and index paths from `DX_ICON_ROOT`, `DX_ICON_INDEX`, `DX_ICON_DATA`, `DX_HOME`, current ancestors, executable paths, user home paths, and `G:\Dx\icon`. |
| Search architecture | Combines exact lookup, prefix search, substring/fuzzy paths, precomputed data, bloom filters, and scoring by match type, length, popularity, and query position. |
| Index loading | Uses zstd-compressed FST and metadata files with a memory-mapped load path in `IconIndex::load_mmap`. |
| SVG export | Generates SVG files from local pack JSON bodies, width, and height metadata for command-line export and download flows. |
| Optional web server | Provides an Axum web binary with `/api/search`, `/api/svg/{pack}/{name}`, and `/api/download` routes behind the `web` feature. |
| Optional WASM target | Keeps a `wasm` feature and `src/wasm.rs` for browser/runtime integration experiments. |
| Benchmark tooling | Keeps compression, memory-map, performance, and stress benchmark binaries in source for future verification refreshes. |
| Disabled CI policy | Keeps workflow files disabled while the current operating rule is to keep GitHub Actions paused. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `icon/README.md` | High-level architecture, CLI usage, data wording, benchmark wording, and future-work notes. |
| `icon/Cargo.toml` | Crate metadata, feature flags, binary targets, dependencies, and optional web/WASM boundaries. |
| `icon/src/lib.rs` | Public module/export surface. Note that its compression comment still says LZ4 while the current index code and artifacts use zstd. |
| `icon/src/index.rs` | zstd save/load implementation and `load_mmap` path. |
| `icon/src/engine.rs` | Search flow, exact/prefix/bloom/fuzzy paths, fallback logic, and scoring. |
| `icon/src/bin/icon.rs` | Main CLI command surface, path discovery, search/export/download/logo logic, and SVG generation. |
| `icon/src/bin/web.rs` | Optional Axum routes and web limitations. Multiple-icon download currently returns `NOT_IMPLEMENTED`. |
| `icon/data/*.json` | Local icon-pack payload. Current checkout count: 229 JSON files. |
| `icon/index/*.zst` | Local compressed index artifacts. Current checkout total: 6,318,053 bytes. |
| `icon/src/bin/*benchmark*.rs` and `icon/src/bin/perf_test.rs` | Benchmark tooling that should be rerun before publishing speed wording. |
| `icon/.github/workflows/README.md` | Disabled GitHub Actions policy. |
| `cargo test -j1`, targeted CLI checks, or benchmark binaries | Runtime authority when search behavior, export behavior, index size, or performance wording needs fresh verification. |

## Public Positioning Boundary

Safe internal statement:

> Icon owns a Rust-based local icon catalog and search surface, backed by 229
> data-pack files, zstd-compressed index artifacts, CLI search/export/download
> commands, optional web routes, and optional WASM support.

Safe public wording after refreshed checks:

> A fresh benchmark can report the measured search latency, load time,
> throughput, index size, and memory use for the exact checked commit and
> machine that produced the report.

Unsafe statement without more verification:

> Icon is the world's fastest icon search engine, beats every competitor, has
> current sub-millisecond performance on every machine, supports bulk web
> download, or proves all README benchmark tables at the current commit.

The strongest current result is local ownership of icon data and search
infrastructure. The remaining verification work is benchmark refresh, web bulk-download
completion, source comment cleanup around compression wording, and runtime
checks for CLI/web/WASM surfaces.

## Documentation Status

The `build` documentation pass is complete. The next folder should be `js`,
because it is an upstream-derived Bun workspace with recorded `.machine`
metadata benchmark validation that must be described as fixture-local rather than
a full runtime speed result.
