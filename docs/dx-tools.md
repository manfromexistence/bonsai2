# DX Tools

Folders that ship a `dx-<tool>` binary and are registered in `TOOL_REGISTRY`
(`cli/src/update_registry.rs`) or documented as DX tools in the root `README.md`.
Every one is an `essence-dx/*` repository unless noted.

| Folder | Repository | Binary | `dx` route | Notes |
| --- | --- | --- | --- | --- |
| `check` | `essence-dx/check` | `dx-check` | `dx check` | validation/scoring engine |
| `cli` | `essence-dx/cli` | `dx` | the CLI itself | hub entry point + TUI |
| `dcp` | `essence-dx/dcp` | `dx-dcp` | `dx dcp` | Development Context Protocol |
| `daemon` | `essence-dx/daemon` | `dx-daemon` | none yet | staged binary in `tools/bin/` |
| `driven` | `essence-dx/driven` | `dx-driven` | `dx driven` | AI-dev orchestration |
| `forkness` | `essence-dx/forkness` | `dx-forkness` | `dx forkness` | no release yet |
| `forge` | `essence-dx/forge` | `dx-forge` | `dx forge`, `dx add` | source/media VCS |
| `grimoire` | `essence-dx/grimoire` | `dx-grimoire` | `dx grimoire` | no release yet |
| `hackin` | `essence-dx/hackin` | `dx-hackin` | none yet | Strix port; runs locally |
| `hello` | `essence-dx/hello` | (none listed) | none | user-hosted calls; not in registry |
| `i18n` | `essence-dx/i18n` | `dx-i18n` | `dx i18n` | localization/TTS/STT |
| `icon` | `essence-dx/icon` | `dx-icon` | `dx icon` | icon search/catalog |
| `media` | `essence-dx/media` | `dx-media` | `dx media` | media processing toolkit |
| `metasearch` | `essence-dx/metasearch` | `dx-metasearch` | `dx metasearch` | search registry/server |
| `os` | `essence-dx/os` | `dx-os` | `dx os` | OS tooling |
| `script` | `essence-dx/script` | `dx-script` | `dx script` | script runner |
| `serializer` | `essence-dx/serializer` | `dx-serializer` | `dx serializer` | token-optimized serializer |
| `skilltune` | `essence-dx/dx-skilltune` | `dx-skilltune` | `dx skilltune` | no `.git` on disk here |
| `style` | `essence-dx/style` | `dx-style` | `dx style` | CSS scanner/generator |
| `token` | `essence-dx/token` | `dx-token` | `dx token` | token tooling |
| `tree` | `essence-dx/tree` | `dx-tree` | `dx tree` | tree tooling |
| `video-player` | `essence-dx/video-player` | `dx-video-player` | none yet | staged binary in `tools/bin/` |
| `www` | `essence-dx/www` | `dx-www` | `dx www`, `dx new`, `dx dev`, `dx run`, `dx graph`, `dx deploy` | web framework/runtime |

MIT OR Apache-2.0
updated: 2026-10-29 00:00 UTC
