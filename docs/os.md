# DX OS

`os` is the platform-specific operating-system integration workspace for DX.
It currently contains separate `posix/` and `windows/` implementations plus
examples and a local `.dx/` state area.

## Position

DX OS is a host-integration layer for operating-system functionality. It is
separate from the DX CLI engines and should expose platform differences behind a
stable boundary.

## Repository shape

- `posix/`: POSIX-oriented implementation files.
- `windows/`: Windows-oriented implementation files.
- `examples/`: usage or experiment material.
- `.dx/`: local workspace state and receipts.

## Validation

No root-level build manifest or dedicated runtime command was found during the
documentation pass. Validate each platform implementation using the build and
test instructions inside its corresponding subfolder before claiming support.

## Status boundary

The folder is present, but repository presence alone does not prove parity
between POSIX and Windows behavior.
