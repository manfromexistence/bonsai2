# Hello in a browser - where we are, what is missing, and what to ask for

**Written 2026-10-24.** This is a handoff file: the state of Hello's browser
path, the problem that is still unsolved, everything already researched, and a
ready-to-paste brief for a stronger model to brainstorm against.

Read this as: *native works, browser is half-built, and the half that is
missing is the half that decides whether the product can exist.*

---

## 1. The goal, in one paragraph

Hello is a voice and video room whose server is **one of the people in it**.
No central media server, no per-minute bill, no account. Native Hello does
this today and has been measured on one machine: a host process relays sealed
Opus and JPEG, a downlink fan-out tree carries the room, host migration keeps
it alive, and an offline "Hello Ports" design (durable bundles, DTN/BPv7) has
its first milestones built. The next surface is the browser: **a call that
opens from a link, free and unlimited, with no install and no server.**

---

## 2. Status at a glance

| Surface | Voice | Video / screen | Hosting | Offline | Verdict |
| --- | --- | --- | --- | --- | --- |
| **Native (Windows, Tauri CLI)** | works | works (screen, synthetic camera, real webcam path) | `hello host`, public relay, fan-out tree | designed; first milestones built, not a finished feature | working product path, measured on loopback |
| **Browser (WASM)** | works | receiving works; sending written, not measured with a real display | **yes - a host tab, measured** (star relay) | **no** | a working tab-only room; signaling is same-machine only |

The browser row is the problem this file is about.

---

## 3. What actually exists in the browser today

| Piece | Path | State |
| --- | --- | --- |
| Room protocol and crypto as WASM | `hello/crates/hello-web` | built; byte-identical to native (self-test pins the key and a sealed datagram) |
| Browser peer (handshake, Auth, heartbeat, audio, video, roster) | `hello/crates/hello-web/web/hello-browser.js` | rewritten; joins, hears, and draws |
| Tunnel: WebTransport + WebSocket to the room's UDP host | `hello/crates/hello-tunnel` | built; one UDP socket per browser session |
| One-command runner | `hello/scripts/run-web.ps1` | host + tunnel + page + browser |
| Node checks | `hello/scripts/test-web.ps1` | self-test, interop, tunnel |
| Real-browser check | `hello/scripts/test-browser.ps1` | two headless Edge windows over WebTransport |
| Receipt | `hello/docs/receipts/browser-webtransport-2026-10-24.txt` | 212/225 audio frames, six screen frames, host `peers=2` |
| Full account | `hello/WASM.md` | what works, what does not, nine defects fixed |
| Innovation shortlist | `hello/BROWSER-BREAKTHROUGHS.md` | six unclaimed ideas and their prior-art checks |

Those two commits are `7ddf2be` (the browser peer and the tunnel) and
`71f71f2` (the breakthrough shortlist) in `essence-dx/hello`, both pushed.

**The shape today:**

```text
browser tab  <--WebTransport/WebSocket-->  hello-tunnel  <--UDP-->  hello host
```

That is the flaw. It needs a native binary running somewhere reachable, and a
tunnel next to it. Nobody will install that to join a call.

---

## 4. The actual problem

> **The room can now be a tab. What is missing is finding each other without
> a server.**

That changed on 2026-10-24: `RoomHost` lets a browser tab host the room with
the real handshake (section 5.6). What remains:

- WebRTC gives a browser a peer-to-peer pipe, but only after **signaling**
  (a few KB exchanged through something). Today that something is the
  browser's `BroadcastChannel`, which works only across tabs of one origin on
  one machine.
- A few NAT pairs (roughly one in five) cannot connect directly and need a
  relay. A helper tab can be that relay (E2/E4), but only if it is online.
- The room must stay free, unlimited, account-free, install-free, and it must
  feel like a product, not a science project.

---

## 5. What has been established already

### 5.1 Transport facts (specs, not guesses)

| Fact | Source |
| --- | --- |
| `RTCDataChannel` can be **unreliable and unordered** (`ordered: false`, `maxRetransmits: 0`) - UDP-like datagrams | MDN, `RTCDataChannel` |
| WebRTC does **not** specify signaling; any channel works (WebSocket, fetch, QR, carrier pigeon) | MDN, *Signaling and video calling* |
| Signaling only exchanges SDP/ICE; media never touches it | MDN, same page |
| `serverCertificateHashes` lets a self-signed WebTransport certificate be pinned (no CA) | MDN, `WebTransport()` |
| WebTransport: Chrome 97+, Edge, Firefox 114+, Safari 26.4+ | MDN browser-compat-data, fetched 2026-10-24 |

### 5.2 Free building blocks (verified)

| Layer | Option | Cost |
| --- | --- | --- |
| Signaling | public **Nostr** relays (Trystero does this, MIT); or QR/copy-paste | $0 |
| Media | WebRTC peer-to-peer | $0 (peer bandwidth) |
| NAT fallback | a participant's tab relays (`peer-as-TURN`) | $0 |
| Last-resort relay | self-hosted `coturn` on Oracle Always Free (10 TB/month egress) | $0 |
| STUN | public STUN | $0 |

### 5.3 Prior-art checks (GitHub repository search, 2026-10-24)

| Query | Result |
| --- | --- |
| `keyless relay webrtc` / `webrtc relay cannot decrypt` / `browser sfu helper tab` | 0, 0, 0 |
| `webrtc peer assisted relay` | 3 repos, none a call product |
| `webrtc datachannel layered video` / `webrtc forward ciphertext group` | 0, 0 |
| `webrtc offline voice message` / `webrtc async voice message offline` | 0, 0 |
| `webrtc host migration` | 1 toy (Scrum Poker) |
| `webserial tnc` / `browser serial ham radio` | a few text-only niche apps, no sealed voice/DTN |
| `webrtc background tab host` | 0 |

Caveat, stated once and meant: **a zero result is not proof of being first.**
It means no public repository describes it in those words on that date.

### 5.4 What we learned the hard way

- A throw inside the datagram loop kills the peer silently. Every datagram
  handler must be isolated.
- A hand-copied protocol enum drifts. The packet kinds now come from Rust.
- `WebTransport.datagrams` is a stream pair, not a `write` method.
- `localhost` may resolve to IPv6 first; bind or address both.
- A QUIC datagram is ~1.2 KB while a room datagram is 1.4 KB. Any tunnel
  needs its own splitting (implemented).

### 5.5 What E1 and E3 measured (2026-10-24)

Two experiments were run to separate the brainstorm from the facts.

| Experiment | Result | Consequence |
| --- | --- | --- |
| **E1**: a negotiated, unreliable `RTCDataChannel` transferred into a Dedicated Worker, forwarding through a **hidden** tab for 400 s at 50 fps | **20,000 / 20,000 messages**, p50 1 ms, p95 1 ms, one 428 ms gap; no AudioWorklet, no Web Lock, no timer in the forwarding path | the worker-relay premise is **real**; the "silent audio + Web Lock keeps it alive" claim was unnecessary |
| **E3**: WebCodecs `scalabilityMode` for VP8/VP9/AV1, 66 configurations | temporal `L1T1`/`L1T2`/`L1T3` accepted; **every spatial mode (`L2*`, `S*`) rejected**, headless and headed | "one encode, drop spatial layers" is **not available** on this build; use two explicit encoders or temporal-only + tile-delta JPEG |
| **E3** hardware encode | not reported to the page in any run | no claim about hardware acceleration either way |
| **E2**: a separate helper tab with **no room key** forwards sealed datagrams to B | **100 / 100 opened**, 0 mismatches, p95 1 ms; helper's wrong-key open: `packet failed authentication`; `typeof SECRET === "undefined"` in the helper | the relay can be trusted with ciphertext it cannot read |
| **E4**: direct pair with `iceTransportPolicy: "relay"` and no TURN, plus a helper path | direct pair stayed `new` after 12 s (`gathering: complete`); helper path carried all 100 | a pair with no direct path still talks through a third tab - no TURN, no native process |

Raw receipts: `hello/docs/receipts/browser-e1-e3-2026-10-24.txt` and
`hello/docs/receipts/browser-e2-e4-2026-10-24.txt` (commits `94091fb`,
`43658cf`). The harness was a temp CDP page set, not part of the repo.

Still open: the hidden-tab heartbeat clock (E1 sent from the visible tab),
real symmetric NAT (E4 removes the direct path by policy, not by a real
CGNAT), helper discovery without a server, a malicious helper, and video on
the relay path.

### 5.6 The tab host, and E5 (2026-10-24)

The browser is no longer a client only. `RoomHost` in `hello-web` is the real
`HostHandshake` and `Session` compiled to WASM; a page hands it one slot per
data channel, and it answers `Hello`, checks `Auth`, opens media and re-seals
each datagram per member. `room.html` + `hello-rtc-host.js` are the page and
its JS half.

| Measurement | Result |
| --- | --- |
| host tab + two guest tabs, direct data channels | 100 frames sent by each; A played 109, B 107; **host members 2, relayed exactly 200** |
| guest A through a helper tab opened **without the room key** (E5) | A played 109, B 109; host relayed 200; **helper forwarded 214** datagrams, no key |
| native processes in the room | **none** - no `hello host`, no `hello-tunnel`, no TURN |

Commands: `pwsh scripts/run-p2p.ps1`, `pwsh scripts/test-browser-p2p.ps1`,
`pwsh scripts/test-browser-p2p.ps1 -Helper`. Receipt:
`hello/docs/receipts/browser-p2p-2026-10-24.txt` (commit `372e233`).

What this settles: E5 is done - real Opus crossed a keyless helper tab. A
browser tab can host a room, so the product no longer needs a native process
to start one.

What is still missing: **automatic signaling across machines** (the manual
invite works), **host migration** when the host tab closes (E6), **real NATs**
(E7, blocked on hardware), the downlink tree (the tab host is star mode), and
video in the p2p test.

### 5.7 Manual invite, measured across two isolated browsers (2026-10-24)

`invite.html` is the no-server signaling path: the host creates one data
channel, waits for ICE gathering to complete, deflates the SDP and shows a
single code; the guest returns one code.

| Measurement | Value |
| --- | --- |
| host invite code | **651 characters** |
| guest reply code | **538 characters** |
| browsers | **two separate Edge processes**, separate profiles, no BroadcastChannel |
| result | guest `live`, host `members=1` |

Command: `pwsh scripts/test-invite.ps1`. Receipt:
`hello/docs/receipts/browser-invite-2026-10-24.txt`.

What it means: **cross-machine calling does not need a server**; it needs one
pasted code (or a QR image). What it is not: automatic. A single link that
joins with no paste needs a rendezvous, and every rendezvous is a third party
or a server: public MQTT, a signed Nostr event, or a tiny relay. The
60-line Nostr snippet floating around does not work as written (relays reject
unsigned events; `#d` is not an ephemeral tag), so that path needs a real
secp256k1 signer.

Still open: automatic signaling, E6 migration, E7 real NATs, tree/video.

### 5.8 One link, no paste, and video (2026-10-24)

`hello-signal.js` adds an MQTT-over-WSS carrier under the same signaling bus:
`BroadcastChannel` for tabs on one machine, a public anonymous broker
(`broker.hivemq.com:8884`) for different machines. SDP is AES-GCM sealed under
a key derived from the room secret, so the broker sees a topic hash and
ciphertext, never an IP address.

| Measurement | Result |
| --- | --- |
| browsers | **3 separate Edge processes**, separate profiles, no BroadcastChannel |
| signaling | `mqtt: connected` at the host; no code pasted |
| audio | 100 Opus frames sent; 103 / 110 played; host relayed exactly 200 |
| video | A sent 5 canvas JPEGs; the tab host relayed them as `Kind::Video`; B drew all 5 |
| native processes in the room | none |

Commands: `pwsh scripts/test-mqtt.ps1`, and `pwsh scripts/test-browser-p2p.ps1`
for the same room over local signaling. Receipts:
`hello/docs/receipts/browser-mqtt-2026-10-24.txt`,
`browser-p2p-2026-10-24.txt` (commit `9ed8e6a`).

What this settles: the single-link, no-paste path exists and is measured, and
video crosses the tab host. What it does not settle: the browsers are on one
machine and one NAT; the broker is third-party metadata infrastructure; no
iPhone; closing the host tab still ends the room; the host is star mode, not
the downlink tree.

Remaining, in order: iPhone Safari run (HTTPS + tap-resume), host migration
(E6), two real networks (E7), the downlink tree in `RoomHost`.

### 5.9 The quality gate, and the honest score (2026-10-24)

`scripts/check.ps1` runs:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `cargo test --workspace` | **331 passed, 0 failed** (31 suites) |
| `node --check` on every JS file outside build output | **12 files, 0 failures** |
| browser suites (`-WithBrowser`) | p2p, helper, invite, mqtt: all pass |

`hello/QUALITY.md` records the result and the eight gaps that have no receipt:
iPhone Safari, two real networks, host migration, the tree, real screen
capture, a malicious helper, the third-party broker, and load/soak.

So the repository is **clean and fully gated**, not **100/100 production**:
the gate is green, the measurements exist, and the uncovered rows are named
instead of hidden. The house rule applies - no bare "production-ready" until
those rows have receipts.

### 5.10 Professional packaging: Rust and TypeScript (2026-10-24)

| Area | What changed |
| --- | --- |
| Browser source | `hello-browser`, `hello-rtc-host`, `hello-signal` are **TypeScript** with `strict`, `noUnusedLocals`, `noUnusedParameters`, `verbatimModuleSyntax`; `hello_web.d.ts` is the typed contract of the wasm exports |
| Build | `tsc` emits the `.js` the page loads; `npm run typecheck` / `build:web`; hand-written `.js` sources removed |
| Gate | `scripts/check.ps1` now runs `tsc --noEmit` too: 331 Rust tests, 3 TS modules, 9 JS files |
| Rust metadata | `repository`, `rust-version`, `readme`, `keywords`, `categories` on `hello-web` and `hello-tunnel`; crate READMEs |
| Toolchain | `rust-toolchain.toml` pins 1.99.0 with rustfmt, clippy and the wasm target |
| CI | `.github/workflows/ci.yml` runs fmt, clippy, tests, the wasm build and `tsc` on Windows for `hello-core`, `hello-web`, `hello-tunnel` (no CMake/Zig needed) |
| Project docs | `SECURITY.md` (threat model, reporting, known limits) and `CONTRIBUTING.md` (gate, TS rules, claim tiers) |

What this does **not** change: the eight product gaps in `QUALITY.md`
(iPhone, two real networks, migration, tree, real screen, malicious helper,
broker, load). The codebase is professionally packaged; the product is not
fully proven.

---

## 6. The six candidate breakthroughs (already shortlisted)

Full detail in `hello/BROWSER-BREAKTHROUGHS.md`. Summary:

| ID | Idea | Kills | Status |
| --- | --- | --- | --- |
| B-01 | **Blind relay tab**: a helper tab forwards ciphertext it cannot decrypt | SFU cost and relay trust | designed, not built |
| B-02 | **Peer-as-TURN**: a participant's tab relays for a pair that cannot connect | the last paid component | designed, not built |
| B-03 | **One encode, seal once, forward the ciphertext** + compound audio ticks | mesh upload/CPU growth | port of proven native work |
| B-04 | **Voice letter**: sealed voice note stored offline, delivered on next contact | "both must be online" | designed, not built |
| B-05 | **Browser → radio**: WebSerial to a KISS/Mercury TNC carrying Ports bundles | install between phone and HF radio | designed, not built |
| B-06 | **Room survives its host tab closing** | "don't close the tab" | port of native host migration |
| enabler | silent `AudioWorklet` + Web Lock keeps a relay tab unthrottled | background-tab throttling | known trick, not implemented |

---

## 7. Constraints any solution must satisfy

1. **$0 at every layer.** No paid TURN, no SFU, no cloud bill. Free tiers are
   allowed only if they never become a bill.
2. **No accounts, no install, no "run this server".** Open a link, click Join.
3. **Keep the room's own protocol and sealing.** The room key is the product;
   a relay must not need it.
4. **Unique, and checked.** Before claiming novelty, search and record what
   was found. "No public repo found" is the honest phrasing.
5. **Provable on this machine.** Two headless Edge windows over CDP, a native
   host where needed, a receipt in `hello/docs/receipts/`.
6. **Honest tiers.** Executed / repo-managed / runtime / release-qualified.
   Loopback numbers are labelled loopback.
7. **Degrade honestly.** If video cannot work without a SFU at a given size,
   say the number at which it stops, not "unlimited".

---

## 8. Open questions to brainstorm

These are the ones that actually matter. A good answer to any of them is worth
more than a feature list.

1. **Signaling without a server and without clunkiness.** Nostr relays work
   but are third-party infrastructure. Is there something better that is
   still $0 and needs no account? (BitTorrent trackers, MQTT public brokers,
   DNS TXT, GitHub gists, email, QR - compare robustness and privacy.)
2. **A relay that cannot listen.** A helper tab forwards sealed packets but
   has no key. How do peers discover/select helpers without a server? How does
   a helper prove it did not get the key? What stops a malicious helper from
   dropping or reordering (beyond the room's own loss handling)?
3. **Peer-as-TURN without leaking.** Routing A-B through C adds a hop and
   exposes metadata to C. Is there a two-hop scheme that keeps C from learning
   who talks to whom, at $0? Onion routing over DataChannels?
4. **A tab that can host.** Background tabs are throttled. Is a silent
   `AudioWorklet` + Web Lock enough, or does the tab need a Service Worker /
   `SharedWorker` / `WakeLock`? What actually keeps WebRTC alive in a
   background tab across Chrome, Edge, Firefox, Safari?
5. **Scaling past mesh.** One encode + group-sealed forwarding is the native
   answer. In a browser, how many peers can one uplink carry before the
   ciphertext fan-out itself saturates? What is the measured curve?
6. **Video without an SFU.** WebCodecs + layered send (two resolutions, the
   receiver picks a layer) is possible. Is that enough for a 6-person screen
   share over a mesh? Where is the cliff?
7. **Offline that is a product, not a demo.** Voice letters in OPFS are easy;
   delivery is the hard part. What is the smallest honest offline feature:
   same-LAN sync, QR/file handoff, or radio?
8. **Trust and abuse.** No accounts means no ban list. What stops a room code
   from being guessed, or a helper tab from being a botnet? (Room key already
   gates media; is that enough?)
9. **The one number.** What is the largest room, at what bitrate, on one
   ordinary uplink, with no server? If the answer is "6", say 6.

---

## 9. Copy-paste brief for a stronger AI

> You are helping design the browser version of **Hello**, a user-hosted voice
> and video room. Read the facts below, then produce **five candidate
> architectures** for a browser call that is free, unlimited, account-free and
> install-free.
>
> **What already works.** Native Hello hosts a room on one participant's
> machine: sealed rooms (X25519 + HKDF + ChaCha20-Poly1305), a zero-decode
> relay, a downlink fan-out tree, compound audio (many talkers under one AEAD
> tag per 20 ms), host migration, and a store-and-forward offline design
> (Hello Ports, BPv7 bundles). A browser peer exists: the same protocol
> compiled to WASM, plus a `hello-tunnel` process that bridges WebTransport
> or WebSocket to the native host. Two headless browsers have joined over
> WebTransport on loopback and heard each other.
>
> **What is missing.** The browser cannot host, and we cannot ask users to run
> a server or tunnel. Any architecture that needs a process the user starts is
> disqualified.
>
> **Hard constraints.** $0 at every layer (no paid TURN/SFU; free tiers that
> never become a bill are acceptable); no accounts; no install; keep the
> room's own sealed protocol; a relay must not need the room key; everything
> must be provable with two or more headless Chromium windows and a written
> receipt; every claim needs a tier and a citation.
>
> **Facts you may rely on.** `RTCDataChannel` supports unreliable/unordered
> mode; WebRTC signaling is unspecified and can run over anything, including
> public Nostr relays; ~1 in 5 NAT pairs needs a relay; a background tab is
> throttled; `getUserMedia`, `getDisplayMedia`, `AudioWorklet`, WebCodecs,
> OPFS, Web Locks, WebSerial and WebTransport exist in current Chromium.
>
> **For each architecture, give:** (1) the data path, drawn; (2) who relays
> and why they cannot listen; (3) the signaling channel and its failure
> modes; (4) the cost at 2, 6 and 20 people, in bytes/second and dollars;
> (5) what is genuinely new versus prior art - name the closest existing
> project and what it lacks; (6) the smallest experiment that would prove or
> kill it in under a week; (7) the failure mode that would embarrass us.
>
> **Then rank them** by (novelty x impact) / effort, and pick the one you would
> build first. Be specific and quantitative. Do not propose a server, an
> account, a paid relay, or "just use WebRTC mesh" without saying where it
> breaks. If something is impossible, say so and give the nearest possible
> thing.

---

## 10. Evidence index

| What | Where |
| --- | --- |
| Browser peer: what works, what does not, nine defects fixed | `hello/WASM.md` |
| Six unclaimed ideas and the searches | `hello/BROWSER-BREAKTHROUGHS.md` |
| The browser run's raw numbers | `hello/docs/receipts/browser-webtransport-2026-10-24.txt` |
| The native product's claims and receipts | `hello/README.md`, `hello/docs/receipts/` |
| The offline design | `docs/hello-ports-review.md`, `hello/OFFLINE.md` |
| The tunnel | `hello/crates/hello-tunnel/src/main.rs` |
| The browser peer | `hello/crates/hello-web/web/hello-browser.ts` |
| The room protocol | `hello/crates/hello-core/src/protocol.rs` |
| One command to see it run | `hello/scripts/run-web.ps1` |

---

## 11. First real implementation: embedded in EsportsFire (2026-10-24)

Hello is now the call in another product, not just in its own page.
`G:\Dx\esportsfire` vendors the browser bundle into `public/hello/` (with
`PROVENANCE.json`) and runs it behind `features/calls/`.

| Piece | Where | State |
| --- | --- | --- |
| Vendor + sync | `esportsfire/scripts/hello-sync.mjs` | 8 files, 697,894 bytes from `hello@63bb875` |
| Controller | `esportsfire/features/calls/call-controller.ts` | tsc + biome clean |
| Host-as-participant | `esportsfire/features/calls/loopback.ts` | new; unmeasured |
| Native lane | `esportsfire/src-tauri/src/hello_room.rs` | `cargo check --features hello-native` passes |
| Directory | `esportsfire/lib/calls/rooms.ts` + Turso | `db:drift` green |

**Two additions to `hello-web` came out of it**, and both are in this repo:

1. `HelloBrowserPeer` accepts **`workletUrl`**. `AudioWorklet.addModule`
   resolves against the document, not the module, so an embedding page that
   serves the bundle from `/hello/` cannot use the hard-coded
   `./hello-worklet.js`.
2. `HelloBrowserPeer` exposes **`senderId()`**, plus `onLevel`/`onMicLevel`.
   `onVideoFrame` reports a sender, and a UI that wants to name that sender has
   to join the room's id to its own presence id - they are different numbers.

`umbrella: what is still open` - `esportsfire/TODO.md` HELLO-1..HELLO-11. The
first of them is the honest one: **no call in EsportsFire has a receipt yet.**

---

MIT OR Apache-2.0
updated: 2026-10-24 00:00 UTC
