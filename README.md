# DX

`G:\Dx` is the hub for the DX ecosystem: one CLI (`dx`) plus a family of
Rust-first tools and products that each live in their own repository beside
it. This repository carries the hub's operating memory, routing manifest, and
public story. It is not a monorepo. Every child folder below is an independent
git repository with its own `origin`, and work on a product belongs in that
product's repo on that repo's branch.

The previous hub README is preserved as `DEPRECATED_README.md`. Read
`AGENTS.md` before touching anything here; it is the contract for how this
hub is worked on safely.

## Start here

```powershell
dx install            # bootstrap: download the CLI, extend PATH, install tools
dx tool list --json   # registry tools with install state and versions
dx --help             # the full public command surface
```

## DX tools

A tool is a CLI binary driven through `dx <tool>` and published by
`TOOL_REGISTRY` in `cli/src/update_registry.rs` (repo-managed). The registry
currently holds 19 entries: the `cli` entry (published as `dx.exe`) plus 18
companion tools.

| Tool | Folder | Repository | Binary | Command |
| --- | --- | --- | --- | --- |
| cli | `cli` | `essence-dx/cli` | `dx` | the CLI itself |
| check | `check` | `essence-dx/check` | `dx-check` | `dx check` |
| dcp | `dcp` | `essence-dx/dcp` | `dx-dcp` | `dx dcp` |
| driven | `driven` | `essence-dx/driven` | `dx-driven` | `dx driven` |
| forge | `forge` | `essence-dx/forge` | `dx-forge` | `dx forge`, `dx add` |
| i18n | `i18n` | `essence-dx/i18n` | `dx-i18n` | `dx i18n` |
| icon | `icon` | `essence-dx/icon` | `dx-icon` | `dx icon` |
| media | `media` | `essence-dx/media` | `dx-media` | `dx media` |
| metasearch | `metasearch` | `essence-dx/metasearch` | `dx-metasearch` | `dx metasearch` |
| os | `os` | `essence-dx/os` | `dx-os` | `dx os` |
| script | `script` | `essence-dx/script` | `dx-script` | `dx script` |
| serializer | `serializer` | `essence-dx/serializer` | `dx-serializer` | `dx serializer` |
| skilltune | `skilltune` | `essence-dx/dx-skilltune` | `dx-skilltune` | `dx skilltune` |
| style | `style` | `essence-dx/style` | `dx-style` | `dx style` |
| token | `token` | `essence-dx/token` | `dx-token` | `dx token` |
| tree | `tree` | `essence-dx/tree` | `dx-tree` | `dx tree` |
| www | `www` | `essence-dx/www` | `dx-www` | `dx www`, `dx new`, `dx dev`, `dx run`, `dx graph`, `dx deploy` |
| daemon | `daemon` | `essence-dx/daemon` | `dx-daemon` | none yet |
| video-player | `video-player` | `essence-dx/video-player` | `dx-video-player` | none yet |

`daemon` and `video-player` are DX tools with published release binaries in
`bin/` (verified by directory listing on 2026-10-09) that have no `dx` route
wired yet. Everything else in the table has a live `dx <tool>` command.

### Install, update, and discovery

`dx install` is the one-command bootstrap: it downloads the CLI into
`%LOCALAPPDATA%\dx\bin`, adds that directory to the user `PATH`, then launches
the installed CLI to install the 18 companion tools. `dx tool
list|install|update|doctor|repair` manages them per tool or `--all`, always
with `--dry-run` on mutating commands and `dx.tool.*.v1` JSON contracts.

Release assets follow one naming convention:
`dx-<tool>-<rust-target-triple>[.exe]`. On Windows one `dx-<tool>` file per
tool lands in `%LOCALAPPDATA%\dx\bin`. The `bin/` folder at the hub root holds
the locally staged per-triple builds (Windows x64/arm64, Linux x64/aarch64
plus musl, macOS x64/arm64, Android, and armv7 entries as of 2026-10-09).

### Honest command-surface status

- Added: `dx skilltune` runs the real `dx-skilltune` binary from the
  `skilltune/` Rust workspace (ReflACT skill-document training: `train`,
  `gate`, `rank`, `apply`, `prompts`, `config`). It works once
  `dx-skilltune` is installed or built locally.
- Added: `dx bot` and `dx typesafe` are registered and routed, but fail
  closed with an install hint until those workspaces publish a release
  binary. The bot workspace currently builds its Tauri app as `dxbot`, which
  does not match the `dx-bot-<triple>` asset convention; typesafe is C++
  and Python tooling with no `dx-typesafe` binary yet.
- Removed: `dx build`, `dx js`, `dx py`, and `dx agents` (plus the hidden
  `dx bundle` alias). They routed at the `build/`, `js/`, `py/`, and `agent/`
  trees, all of which are absent from this workspace, so every one of those
  commands failed closed at its workspace guard. `dx <script.js|ts>` and
  `dx <script.py>` direct script dispatch is unchanged.

## DX products

A product is a larger app or surface rather than a `dx <tool>` binary. Some
still expose a CLI route.

| Product | Folder | Repository | What it is | CLI route |
| --- | --- | --- | --- | --- |
| code | `code` | `essence-dx/code` | dx-code: GUI desktop code editor | `dx desktop sync-web` |
| tui | `tui` | `essence-dx/tui` | dx-tui terminal UI | none (hidden `dx tui` launches `cli/tui`) |
| bot | `bot` | `essencefromexistence/flow` | dx app for mobile and desktop | `dx bot` (fails closed until release) |
| typesafe | `typesafe` | `essence-dx/dx-typesafe` | dx decision-model system | `dx typesafe` (fails closed until release) |
| extensions | `extensions` | `essence-dx/extensions` | dx extensions | none |
| website | `website` | `essence-dx/website` | dx official website | `dx site` (hidden) |

## Client project (not DX)

| Folder | Repository | Status |
| --- | --- | --- |
| `dhakaraiment` | `manfromexistence/dhakaraiment` | Client work, not a DX tool or product. It will be relocated out of `G:\Dx` by its owner; left in place and untouched until then. |

## Hub infrastructure

| Path | Job |
| --- | --- |
| `dx` | The TOML-shaped routing manifest: `workspace`, `paths`, `tools`, `dx_www`, `media`, `tui` blocks. Several declared paths are dangling; confirm with `Test-Path` before acting on any of them. |
| `bin/` | Staged local binaries and the per-triple release builds. `dx.exe` and `dx.cmd` are the normal Windows entrypoints. |
| `docs/` | The achievement register: one page per folder plus `docs/README.md` rules. |
| `.dx/` | Hub runtime state and receipts. |
| `DEPRICATED_9-17-2026/` | Parking lot for retired folders and repos (40 items on 2026-10-09). Nothing in it is deleted; it is history kept out of the way. |
| `AGENTS.md` | The operating manual for this hub. Read it first. |
| `TODO.md`, `CHANGELOG.md`, `DX.md`, `PROMPTS.md` | Open work, dated change log, worker notes, prompts. |
| `build.ps1` | Hub build entrypoint. |
| `.gitignore` | Deny-everything (`/*`) with an allow-list. A new root file needs its `!/name` line or git cannot see it. |

## Parked folders

These folders used to sit at the hub root and were moved into
`DEPRICATED_9-17-2026/` (moves, not deletions) during the 2026-10-09 cleanup:

| Was at root | Now parked as |
| --- | --- |
| `.vscode`, `.workbuddy-ai`, `benchmark`, `test`, `zzz`, `pi-test`, `channels`, `exa` | `DEPRICATED_9-17-2026\<same name>` |
| `native` | `DEPRICATED_9-17-2026\native-2026-10-09` |
| `mobile` | `DEPRICATED_9-17-2026\mobile-DEPRECATED-20261009-1616` |

`mobile` was superseded by `bot`, which is the dx app for both mobile and
desktop. `native` was the C++ renderer workspace that `dx native` and
`dx ecosystem status` read; with the folder parked those surfaces report the
path as missing until the config path is repointed or `dx native` is retired.

## Environment facts

Measured on 2026-10-09 with `Get-PSDrive G`: `G:` is a ~477 GB volume with
~23 GB free. The `essence-dx/*` repositories are private, so anonymous GitHub
API calls return 404 and a full `cargo` build can fail with 401 on the
private `check` dependency (`cli/DX.md`). The hub is Windows (`win32`) with
PowerShell as the shell.

## Source of truth

| Question | Answer lives in |
| --- | --- |
| Where does tool X live on disk? | the `dx` manifest, `paths` block |
| Which repo publishes binary X, under what filename? | `TOOL_REGISTRY` in `cli/src/update_registry.rs` |
| What has folder X achieved? | `docs/<x>.md`, then the `docs/README.md` register |
| What is open or blocked? | `TODO.md` (hub), the folder's own ledger (products) |
| What changed most recently? | `CHANGELOG.md` (hub), `git log` (products) |
| How do I work here without breaking it? | `AGENTS.md` |

MIT OR Apache-2.0
updated: 2026-10-09 10:16 UTC
