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
| **Browser (WASM)** | works | receiving works; sending written, not measured with a real display | **no** - needs a native host plus a tunnel process | **no** | demo, not a product |

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

> **A browser tab cannot be the room's server, and we cannot ask a user to run
> one.**

Everything else follows:

- A browser has no UDP socket, so it cannot be `hello host`.
- WebRTC gives a browser a peer-to-peer pipe, but only after **signaling**
  (a few KB exchanged through something) and only if **ICE** finds a path.
- A few NAT pairs (roughly one in five) cannot connect directly and need a
  relay. That is the only place a bill can appear.
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
| The browser peer | `hello/crates/hello-web/web/hello-browser.js` |
| The room protocol | `hello/crates/hello-core/src/protocol.rs` |
| One command to see it run | `hello/scripts/run-web.ps1` |

---

MIT OR Apache-2.0
updated: 2026-10-24 00:00 UTC
