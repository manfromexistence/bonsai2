# DX Script

`script` is the Rust command-runner workspace for the DX ecosystem. Its Cargo
package is `dx-script`, with a library-compatible `just` name and a
`dx-script` binary.

## Position

DX Script provides task execution, command discovery, completions, examples,
and project automation through a DX-branded command surface. It depends on the
DX Serializer through a local path during workspace development.

## Repository shape

- `src/`: command-runner implementation.
- `crates/`: supporting workspace crates.
- `completions/`: shell completion assets.
- `examples/`, `tests/`, `skills/`: usage and verification surfaces.
- `www/`: documentation or web material.

## Validation

Run from `G:\\Dx\\script`:

```powershell
cargo metadata --no-deps
cargo test -j1
cargo build --bin dx-script
```

## Status boundary

The Cargo manifest is present and metadata validation is the fast structural
check. Build and test receipts are still required before release wording.
