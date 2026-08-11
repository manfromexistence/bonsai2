# DX Tree

`tree` is the Rust directory-listing utility for DX. Its Cargo package is
`rust_tree` and its binary is `dx-tree`.

## Position

DX Tree produces indented directory listings with glob support, executable-file
handling, colored terminal output, and JSON serialization. It uses DX
Serializer as a local workspace dependency.

## Repository shape

- `src/`: CLI implementation.
- `tests/`: integration tests.
- `examples/`: usage examples.
- `store-assets/`: supporting assets.
- `wix/`: Windows packaging material.

## Validation

Run from `G:\\Dx\\tree`:

```powershell
cargo metadata --no-deps
cargo test -j1
cargo run --bin dx-tree -- --help
```

## Status boundary

Directory output depends on the requested path, ignore rules, and formatting
options. Validate JSON and terminal output separately when both are consumers.
