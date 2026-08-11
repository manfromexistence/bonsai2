# DX Token

`token` is the Rust token-counting utility for DX. Its Cargo package is
`dx-token` and it exposes both `dx-token` and `token` binaries.

## Position

DX Token counts tokens across files and folders using multiple tokenizers. It
supports recursive file discovery, JSON output, parallel traversal, regular
expressions, and a terminal-oriented colored display.

## Repository shape

- `src/main.rs`: CLI entry point.
- `Cargo.toml`: binary names and tokenizer/file-walking dependencies.
- `target/`: local build output; it is not source documentation or release proof.

## Validation

Run from `G:\\Dx\\token`:

```powershell
cargo metadata --no-deps
cargo build --bins
cargo run --bin dx-token -- --help
```

## Status boundary

Token counts depend on the selected tokenizer and file exclusions. Reports
should identify the tokenizer and input scope instead of presenting one count as
universal.
