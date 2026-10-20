# CAP — CapSoftware/Cap (open-source Loom alternative)

**Location on disk:** `G:\Dx\esportsfire\inspirations\cap` (untracked snapshot inside the `esportsfire` working tree; not its own git repo here).
**Upstream:** https://github.com/CapSoftware/Cap
**Website:** https://cap.so · Docs: https://cap.so/docs · Download: https://cap.so/download

Cap is the open-source alternative to Loom: fast screen recording, polished local editing,
instant share links, comments, transcripts, analytics, team workspaces, custom domains,
custom S3 storage, and full self-hosting. Use it for product demos, bug reports, onboarding,
tutorials, design reviews, engineering walkthroughs, async standups, client updates, etc.

---

## 1. What it is

- **Recording modes**
  - *Instant Mode* — uploads while recording, gives a shareable link the moment recording stops (fast feedback, bug reports, async updates).
  - *Studio Mode* — records locally, opens the editor, lets you export/share a polished video (product demos, tutorials, launches, client work).
- **Capture** screen + camera + microphone.
- **Sharing** public/private links, passwords, custom domains, viewer analytics, comments, reactions, transcripts, chapters.
- **Storage ownership** — Cap Cloud, or your own S3-compatible bucket (AWS S3, Cloudflare R2, Backblaze B2, MinIO, Wasabi), or fully local, or fully self-hosted.
- **Cap AI** — titles, summaries, clickable chapters, captions, transcripts.
- **Loom import** — migrate existing Loom videos into Cap.
- **Platforms** — macOS and Windows desktop apps, web dashboard, mobile, Chrome extension, CLI.

---

## 2. Tech stack (high level)

| Layer | Cap uses |
| --- | --- |
| Monorepo | Turborepo + Bun workspaces (`apps/*`, `packages/*`, `crates/*`) |
| Package manager / runtime | Bun 1.4.0, Node ≥ 20 |
| Rust | **1.88.0** (see `rust-toolchain.toml`) |
| TS lint / format | Biome 2.2.0 (tab indent, double quotes, `organizeImports`) |
| Database | MySQL + Drizzle ORM (`packages/database`) |
| Web framework | **Next.js 16.3** + React 19.2 (App Router, Turbopack) |
| Web backend | Effect (`@effect/platform`, `@effect/rpc`, HttpApi) |
| Styling | Tailwind CSS v4 |
| Desktop (shipped) | **GPUI** — Zed's UI framework (Rust), forked (`wingleeio/zed`); standalone workspace in `apps/desktop-gpui` |
| Desktop (legacy) | **Tauri v2** + SolidStart (SolidJS) — last shipped in 0.6.0, now kept as the dev harness only |
| Mobile | Expo ~55 + React Native 0.83 |
| Media processing | Custom FFmpeg fork (`CapSoftware/rust-ffmpeg`), wgpu 25, cpal (audio), whisper-rs (transcription) |
| License | **AGPLv3** overall; **MIT** for `cap-camera*` and `scap-*` crates |

---

## 3. Rust usage — very heavy (this is the recording engine)

Cap is Rust-first for all media. The root `Cargo.toml` workspace includes `crates/*`
(≈ 60 crates) plus `apps/cli`, `apps/desktop/src-tauri`, and `crates/workspace-hack`.

Key crate families:

- **Capture:** `scap-screencapturekit` (macOS), `scap-direct3d` (Windows), `scap-ffmpeg`, `scap-targets`, `scap-cpal` (audio), `cursor-capture`, `cursor-info`, `d3d-adapter`, `windows-capture`
- **Camera:** `camera`, `camera-avfoundation`, `camera-directshow`, `camera-mediafoundation`, `camera-windows`, `camera-ffmpeg`, `camera-effects`
- **Audio:** `audio`, `rnnoise` (noise suppression), `cpal-ffmpeg`, `scap-cpal`
- **Encoding / muxing:** `enc-ffmpeg`, `enc-avfoundation`, `enc-mediafoundation`, `enc-gif`, `cap-muxer`, `cap-muxer-protocol`, `mediafoundation-ffmpeg`, `mediafoundation-utils`
- **Rendering / export:** `rendering`, `rendering-skia`, `render-farm`, `export`, `frame-converter`, `gpu-converters`, `ffmpeg-hw-device`, `video-decode`
- **Recording orchestration:** `recording`, `project`, `media`, `media-info`, `timestamp`
- **Other:** `api`, `automation`, `flags`, `utils`, `fail`, `verify` / `verify-expect` / `verify-stimulus`, `cap-test`, `cli-install`

Sidecar binaries shipped with the desktop app (`externalBin` in `tauri.conf.json`):

- `cap-muxer`, `cap-exporter`, `cap-cli`

Audio uses a **custom cpal fork** (`CapSoftware/cpal`) with fixes for WASAPI silent-buffer
handling and macOS stream release. Video uses a **custom ffmpeg-next fork** with hardware
device support. Screen capture on macOS uses a `cidre` fork (`CapSoftware/cidre`) wrapping
ScreenCaptureKit; on Windows it uses `windows-capture` + Direct3D.

---

## 4. Tauri usage — legacy, not the shipped app

Important nuance: Cap **no longer ships Tauri as the product**.

- `apps/desktop` = Tauri v2 + SolidStart frontend, version 0.6.1 — "last shipped in 0.6.0,
  kept for the dev harness."
- `apps/desktop-gpui` = the **actually shipped** desktop app (built via
  `bun run gpui:build <target>`), using **GPUI** (Zed's framework) in pure Rust, with its
  own standalone Cargo workspace. It is kept separate so the enormous gpui dependency tree
  and its git revisions don't pollute the root `Cargo.lock` / workspace-hack / CI, and to
  sidestep the root `[patch.crates-io]` entries that only exist for the Tauri app.
- Tauri is still used as the **dev harness** wrapper (SolidStart dev server at port 3002 +
  Tauri shell), and Tauri plugins (dialog, fs, global-shortcut, updater, deep-link,
  clipboard, os, process, shell, store, notification, http, window-state, positioner,
  opener, oauth, sentry) remain wired in `apps/desktop/src-tauri`.

So: Tauri v2 is present but is the **legacy/dev shell**; the production desktop is a
**native Rust GPUI app**.

---

## 5. Repository map (from README)

| Path | What lives there |
| --- | --- |
| `apps/desktop` | Tauri v2 desktop app with SolidStart UI and Rust backend (dev harness) |
| `apps/desktop-gpui` | GPUI desktop app that ships as Cap (Rust) |
| `apps/web` | Next.js web app — marketing, docs, dashboard, sharing, API routes, auth |
| `apps/cli` | Rust CLI |
| `apps/media-server` | Media processing service used by the web app |
| `apps/discord-bot` | Discord integration |
| `apps/chrome-extension` | Chrome extension |
| `apps/mobile` | Expo / React Native mobile app |
| `apps/render-farm` | Render farm service |
| `apps/status` | Status page |
| `packages/database` | Drizzle schema and database access (MySQL) |
| `packages/ui` | Shared React UI |
| `packages/ui-solid` | Shared Solid UI |
| `packages/web-backend` | Backend service layer |
| `packages/web-domain` | Web domain models and types |
| `packages/env` | Environment validation |
| `packages/sdk-embed` | Embed SDK |
| `packages/sdk-recorder` | Recorder SDK |
| `packages/recorder-core` | Recorder core (TS) |
| `packages/s3` | S3 helpers |
| `packages/utils` | Shared utils |
| `crates/*` | Recording, capture, camera, audio, encoding, rendering, muxing, export, tests |
| `capcodec/` | Codec baselines / decisions / findings / progress (research) |
| `scripts/*` | Setup, analytics, build, maintenance tooling |
| `infra/*` | Infrastructure configuration |
| `vendor/*` | Vendored crates (wgpu-hal, tao, tauri-nspanel) |

---

## 6. Comparison to `esportsfire` (G:\Dx\esportsfire)

| Aspect | esportsfire | Cap |
| --- | --- | --- |
| Web frontend | Next.js + Tauri | Next.js 16 + React 19 (web app) |
| Desktop shell | **Tauri v2** (Next.js wrapped in Tauri) | **GPUI (Zed)** shipped; Tauri v2 legacy dev harness |
| Desktop UI | Next.js (web tech in desktop) | SolidStart (Tauri legacy) / native GPUI (shipped) |
| Mobile | Tauri mobile (Android / iOS) | Expo / React Native |
| Rust | `overlay` crate + `server-chat` | ~60 media crates (capture, encode, render, export, transcription) |
| Database | (via scripts) | MySQL + Drizzle |
| Backend style | Next.js API routes | Effect-based HttpApi + Next.js handlers |
| Package manager | Bun 1.3.14 | Bun 1.4.0 |
| Rust toolchain | 1.77.2 | 1.88.0 |
| License | MIT | AGPLv3 (MIT only for camera / scap crates) |
| Scope | Esports tournament app | Screen recorder + sharing platform + self-host |

Shared DNA: both are **Turborepo + Bun + Next.js + Rust + Tauri** monorepos with a Rust
sidecar/overlay crate, targeting Windows + Android. Cap is far larger in media/Rust surface
and has migrated its shipped desktop from Tauri to GPUI, while esportsfire still ships via Tauri.

---

## 7. Build / dev commands

```
bun install
bun run env-setup && bun run cap-setup
bun run dev            # full stack (web + desktop)
bun run dev:desktop    # desktop only
bun run dev:web        # web only
bun run dev:windows    # windows helper (docker + turbo)
bun run build          # turbo build
bun run tauri:build    # desktop release (Tauri path)
bun run gpui:build <target>   # shipped desktop (GPUI)
cargo test -p <crate>  # Rust tests per crate
bun run db:generate && bun run db:push
bun run lint / format / typecheck
```

---

## 8. Notable details worth learning from

- **Custom forks** of cpal, ffmpeg-next, nokhwa, cidre, reqwest, tao, wgpu-hal,
  tauri-nspanel, screencapturekit — vendored under `vendor/` and patched in root `Cargo.toml`.
- **Workspace-hack crate** (`crates/workspace-hack`) via `cargo hakari` to unify features.
- **Recording reliability doc** (`docs/recording-reliability.md`) and a `cap-test` crate with
  discover / matrix / synthetic / benchmark suites.
- **GPUI chosen over Tauri** for the shipped app (transparent rounded windows + performance);
  Tauri retained as dev harness.
- **Effect** for type-safe API layer, **Drizzle** for DB, **Tinybird** for analytics.
- **Rust 1.88**, Tauri 2.5/2.8, specta for typed IPC bindings (`tauri-specta`).
- **Sidecars** built via `scripts/build-desktop-binaries.mjs` (cap-muxer, cap-exporter, cap-cli).
- **macOS framework** dependency: `Spacedrive.framework` (native-deps).

---

## 9. Self-hosting

Fastest path:

```
git clone https://github.com/CapSoftware/Cap.git
cd Cap
docker compose up -d
# available at http://localhost:3000
```

Deployment options: Docker Compose, Railway (one-click), Coolify (`docker-compose.coolify.yml`).
Production env: `CAP_URL`, `S3_PUBLIC_URL`, plus secrets before exposing. Desktop can point
at a self-hosted instance via `Settings > Cap Server URL`.

---

## 10. License

Portions licensed as follows:

- `cap-camera*` and `scap-*` crate families — **MIT** (see `licenses/LICENSE-MIT`).
- Third-party components — original owner's license.
- All other content — **AGPLv3** (see `LICENSE`).

---

MIT OR Apache-2.0
updated: 2026-10-18 00:00 UTC