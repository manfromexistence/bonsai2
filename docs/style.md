# DX Style Achievements

`style` is the Rust-based CSS scanner and generator for DX WWW. It gives DX a
Tailwind-familiar generated CSS workflow without making Tailwind, PostCSS,
Autoprefixer, or a local Node dependency chain the hidden release runtime.

## Position

DX Style's public release path is normal generated CSS: project files and style
entry files are scanned, `styles/theme.dx.css` remains the semantic token
source, and `styles/app.generated.css` is the browser-loaded output. Binary CSS
experiments remain internal; generated CSS is the product path.

Professional positioning:

> DX Style is a repository-managed CSS scanner/generator for DX starters, with
> theme-token strictness, static class scanning, supported Tailwind-familiar
> utilities, grouped-class read models, generated CSS receipts, and Zed/Web
> Preview review contracts.

That statement intentionally avoids full Tailwind parity. DX Style supports a useful
subset and owns a detailed compatibility matrix so unsupported Tailwind
surfaces become explicit diagnostics instead of silent false support.

## What Style Has Already Achieved

| Area | Achievement |
| --- | --- |
| Public CSS path | Defines `dx style build`, `dx style watch`, and `dx style check` around generated CSS plus style receipts. |
| Source scanning | Scans project-owned TSX, JSX, TS, JS, HTML, MDX, CSS, theme, globals, app, component, and Forge paths with receipt-visible source counts. |
| Theme ownership | Treats `styles/theme.dx.css` and CSS-first `@theme` tokens as DX-managed inputs rather than hidden Tailwind runtime state. |
| Tailwind-familiar subset | Supports many high-use utilities, variants, media/container queries, grouped syntax, token-aware colors, and common CSS directives while tracking remaining gaps. |
| Honest compatibility matrix | Maintains `TAILWIND_COMPATIBILITY.md`, official fixture inventory, fixture matrix, utility ledger, directive ledger, and live comparison receipt design for Tailwind v4.3 canonical behavior. |
| Unsupported diagnostics | Reports unsupported JavaScript config/plugin surfaces such as `@plugin`, `@config`, and legacy `@tailwind` instead of executing external JS or pretending parity. |
| Grouped classes | Expands grouped class syntax, records `GroupRegistry` alias-to-atomic data, supports compact `alias()` reuse, and emits grouping-error tokens for invalid syntax. |
| Editor read models | Provides grouped-class source spans, cursor-token context, source digests, dry-run receipts, registry receipts, reverse-CSS maps, and non-invertible reasons for editor consumers. |
| Web Preview review bridge | Owns visual generator catalog, recipe catalog, control catalog, CSS declaration hint catalog, source-apply contracts, review-only IPC shape, byte caps, and copied review packet expectations. |
| Source mutation safety | Keeps `source_mutation_enabled=false` until dry-run receipts, active source path/span/digest, explicit user apply, native review, editor write bridge, and runtime validation all agree. |
| Browser/PostCSS starter workstream | Provides scoped starter compatibility validation for import flattening, nesting, custom media/selectors, color fallbacks, logical fallbacks, prefix canaries, and browser compatibility receipts. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `style/README.md` | Public workflow, scanner boundaries, Tailwind subset, grouped classes, Zed/Web Preview contracts, and known gaps. |
| `style/TAILWIND_COMPATIBILITY.md` | Maintained Tailwind v4.3 gap matrix and compatibility positioning boundary. |
| `style/ARCHITECTURE.md` | Pipeline architecture, parser, group analyzer, CSS generator, output writer, app state, and binary experiment context. |
| `style/Cargo.toml` | Crate identity, dx-serializer dependency, parser/generator dependencies, features, and build dependencies. |
| `style/src/core/engine/feature_matrix.rs` | repository-managed Tailwind gap matrix. |
| `style/src/core/engine/utility_ledger.rs` | Utility documentation-area coverage and unproven canaries. |
| `style/src/core/engine/directive_ledger.rs` | CSS directive support and unsupported-by-design boundaries. |
| `style/src/core/engine/parity.rs` | Generated-output parity receipt contract. |
| `style/src/core/engine/grouped_class_*.rs` | Grouped-class contracts, receipts, read models, source digests, reverse CSS map, and source-apply gates. |
| `style/src/core/engine/visual_generator_*.rs` | repository-managed visual generator catalogs and controls for Zed/Web Preview. |
| `style/fixtures/*.json` | Checked-in contract fixtures for grouped classes, source apply, visual generators, Tailwind matrices, browser compatibility, and PostCSS compatibility. |
| `dx style build` / `dx style check` | Runtime authority when current generated CSS and receipts need to be refreshed. |

## Public Positioning Boundary

Safe internal statement:

> DX Style owns the generated CSS path for DX WWW starters, with Rust scanning,
> theme tokens, supported Tailwind-familiar utilities, receipt-visible source
> boundaries, grouped-class contracts, and Zed/Web Preview review gates.

Safe public wording after refreshed checks:

> The current `dx style build` and `dx style check` outputs generate CSS and
> receipts for the documented supported subset, while unsupported Tailwind
> config/plugin and unproven utility families remain explicit diagnostics.

Unsafe statement without more verification:

> DX Style is a complete drop-in Tailwind v4.3, PostCSS, Autoprefixer, and
> plugin ecosystem replacement across every class, directive, browser target,
> config file, and source-language shape.

The strongest current result is repository-managed CSS generation for DX starters. The
remaining verification work is full utility/value/modifier coverage, full source-graph
parity, complete directive/cascade parity, universal browser fallback parity,
and mutation-capable editor write verification.

## Documentation Status

The `check` page now documents the validation and receipt engine that should
consume Style, Forge, WWW, CLI, and Zed verification surfaces. The next folder should
be `media`, because media processing has repository-managed tool, receipt,
dependency, and provenance concepts that need the same validation boundary.
