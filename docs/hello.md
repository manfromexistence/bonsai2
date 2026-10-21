# DX Hello Achievements

`hello` is a **user-hosted call**: one participant's machine hosts the room and
the others join it, over sealed UDP, with no rented media server, no account and
no per-minute bill. The thesis is that a call should cost its participants
nothing but the bandwidth they already have.

It is not a DX tool in the house sense: there is no `dx hello` binary and no
`TOOL_REGISTRY` entry. It is a **product** in the corpus, and the hub knows it
the way it knows every other folder: by what it has actually achieved.

## Position

| Fact | Evidence |
| --- | --- |
| Folder | `G:\Dx\hello`, its own git history (tip `1004206`, 2026-10-21), 251 tests passing on this machine |
| Manifest path | `dx` manifest `paths` block → `hello = hello` (added 2026-10-21); `cli/src/config.rs` carries the matching `hello` field |
| What it is | A Rust workspace of nine crates: `hello-core` (wire format, sealing, tree, stats), `hello-net` (host, client, relay, tree), `hello-video` (capture, tiles, inter-frame codec), `hello-audio`, `hello-native` (DLL loader), `hello-cli`, `hello-demo` (room harness), `hello-metrics`, `hello-port` (durable store-and-forward) |
| Native boundary | C++ (Opus 1.5.2 wrappers) and Zig (SPSC ring, saturating mixer) behind one C ABI, orchestrated by CMake; the Rust side loads the DLL at runtime (`crates/hello-native`) |
| Wire format | Protocol **v10** (`crates/hello-core/src/protocol.rs:52`), 28-byte outer header with an epoch and an SVC layer descriptor, sealed with ChaCha20-Poly1305 under a key derived from the room passphrase |
| Desktop shell | Tauri v2, static HTML talking to Rust (`apps/hello-desktop`) |
| Its own ledger | `hello/MEMORY.md` (rules, measurements, next work), `hello/STATUS.md` (milestones), `hello/PLAN.md`, `hello/PROPOSAL.md`, `hello/docs/ARCHITECTURE.md` |

## What Hello Has Already Achieved

| Area | Achievement |
| --- | --- |
| A working call | Opus voice in both directions over real UDP, 50 fps on a 20 ms frame clock, 0 skipped frames; the jitter buffer, the mixer and the loss accounting are measured (`hello/docs/receipts/`) |
| Sealed rooms | Ephemeral X25519 per peer, HKDF with the room key, ChaCha20-Poly1305 bodies, wrong keys and wrong rooms refused |
| Zero-decode relay | The host never decodes media: it validates and forwards ciphertext. Measured 0.332 µs per audio recode, 3.01 M ops/s per core, so **CPU is free and bandwidth is the only currency** (`hello/social/CAPACITY.md`) |
| Downlink fan-out tree | One group copy to a few roots, forwarded unchanged: host egress `streams x min(fanout, N) x bitrate` instead of `x (N - 1)` |
| Reach without a server | Public relay fallback, a relay-assisted direct path, and an upstream fan-out where a peer with a bad path hands its media to a carrier that forwards it unchanged - the media is sealed for the host, so the carrier cannot read it |
| Forwarder placement | Ranked by measured loss and RTT, with a declared relay budget: a peer whose uplink is spent is not chosen as a forwarder |
| Host migration | The room survives its host leaving: a roster every heartbeat, a deterministic election, a promotion notice, and epoch-scoped group keys |
| Product surface | Per-peer volume, a listen-only role, a listen-only CLI command, a portable package with install and uninstall scripts, and a Tauri shell with device and camera pickers |
| An inter-frame camera codec | Written, measured and **opt-in**: it is a real motion-compensated codec, and the receipt says plainly that it does not yet beat the tile path (12,875 B/frame against 2,395 at 640x360) |
| The offline path, started | `hello-port`: signed manifests, a durable store, receipted delivery, a contact scheduler, and two adapters. Milestone 1A of **Hello Ports** - a call that keeps working when the internet does not |

## Validation To Check

| Validation | Purpose |
| --- | --- |
| `hello/docs/receipts/README.md` | The receipt register: what each receipt owns, and the commands that produced it |
| `hello/docs/receipts/tree-direct-2026-10-18.txt` | The downlink tree and the direct path, with the relay's `clients=0` as the independent check |
| `hello/docs/receipts/codec-inter-2026-10-21.txt` | The inter-frame codec's measurement, including the verdict that it does not yet win |
| `hello/docs/receipts/demo-camera-4.json` | A whole four-person camera room: bandwidth, CPU, memory |
| `hello/MEMORY.md` | The operational record: the rules, the measurements, the failures and what comes next |
| `hello/STATUS.md` | The milestone table, including what is still unverified |
| `hello/social/CAPACITY.md` | What one free box can serve, and the arithmetic behind it |

## What Is Not Claimed

- **No real-network run.** Every measurement is loopback on one machine. The
  direct path, the tree and the migration are all *proven on loopback*; the
  two-NAT check is still open (`hello/MEMORY.md` §9).
- **No webcam picture.** This machine's camera delivers about six frames a
  second of black picture, so the camera path is verified as a pipeline and
  not as a picture.
- **The inter-frame codec does not yet beat JPEG**, and the docs say so.
- **The offline stack is a design and a first milestone, not a product.** The
  `hello-port` crate carries a message between two ports with the network
  down; there is no radio, no satellite link and no call that survives.
