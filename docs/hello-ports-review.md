# Hello Ports - a design review, and the sources it was checked against

**What this is.** A review of a proposed "Hello Ports" design for the hello
product: ports that hold messages and carry them between people who are not on
the same network, not at the same time, and possibly not on the same continent.
The review was written in a session that could not read this repository - its own
line 5 records that - so it reviews a *proposed* design rather than the code
that exists. The design it reviews is not in this repository either; what the
repository now holds is milestone 1A of it, in hello/crates/hello-port.

**Why it lives here.** It is a hub-level design document, not a product
document, and it was sitting unindexed in a product tree. It is indexed in
docs/README.md and referenced from docs/hello.md.

**Checked at source on 2026-10-21.** Every claim the review makes about the
outside world was verified against its primary source:

| Claim in the review | Source | Result |
| --- | --- | --- |
| The Mercury's ARQ data socket is 8301, not 8300 (control 8300, broadcast 8100) | Rhizomatica/mercury mercuryv2, mercury.ini.example: *"ARQ TCP base port (control = base_port, data = base_port + 1) (default: 8300)"* and *"Broadcast TCP port (default: 8100)"* | confirmed |
| Mercury is GPL-3.0-or-later | the same repo's main.c header: *"either version 3, or (at your option) any later version"* | confirmed |
| The ARQ path uses Reed-Solomon over GF(256); the broadcast path uses a RaptorQ fountain carousel, with no mode negotiation | docs/CAROUSEL-ARQ.md, docs/BROADCAST-FILE.md | confirmed |
| At DATAC3, 1 kB takes about half a minute | docs/BROADCAST-FILE.md, verbatim | confirmed |
| Android's satellite manifest value is the application package name, not `true` | the platform's satellite guidance, and Google's own pi_diff/36.1 page listing PROPERTY_SATELLITE_DATA_OPTIMIZED as added in API 36.1 | confirmed |
| Apple documents carrier-constrained.appcategory and carrier-constrained.app-optimized, and a Live Activity can be active for up to eight hours | Apple's own DocC: *"The key that defines an app's category for accessing a carrier-provided satellite network"*, *"A Boolean value that indicates whether your app is optimized for..."*, *"A Live Activity can be active for up to eight hours unless its app or a person ends it before this limit."* | confirmed |
| p7 0.12.0 (2026-10-17) is the current release, with validation, FFI-safety and RFC 9171 alignment; the crate handles bundle encoding and decoding, not transmission | crates.io, the GitHub release body, and the README: *"This library only handles encoding and decoding of bundles, not transmission or other processing of the data."* | confirmed |
| An Ed25519 signature is 64 bytes | RFC 8032 | confirmed |
| The capacity arithmetic (168.75 MB per 30 s at 45 Mbps; 1.67 kbps for 50 small messages a minute; a 192-byte Merkle path) | recomputed from the review's own figures | confirmed to the byte |

**Two claims could not be read at source from this machine** and are recorded
as unverified rather than as agreed: the Bao range-proof format and the Walrus
reconstruction procedure (both documents serve no text to a plain fetch), and
the exact Android page that states the manifest value (its URL now 404s, though
the constant's API level is confirmed by Google's own diff page).

The review's own corrections stand, and the architecture it recommends is
sound: the source creates authenticated pieces, untrusted relays forward them
unchanged, and the delivery states describe evidence rather than hopes.

---
I’ll check the new claims against the source docs and repositories, then separate what’s ready to implement from what still needs testing. I can outline the `hello-port` integration, but I can’t inspect or modify your `G:\Dx\...` code without the relevant files.The core “Ports + ferries + optional radio backhaul” design still looks promising, but the new specification has several concrete errors: Mercury’s default data port is 8301, Android documents the package name—not `true`—as the satellite metadata value, and distinct RLNC symbols do not necessarily provide enough independent data to decode. I also found a newer `bp7` release dated October 17, 2026, so I’m checking the version-specific claims.**The overall architecture is worth building, but I would not merge this document as a “verified, pollution-proof” specification yet.** Rechecking the primary sources exposed several concrete errors—including the Android manifest value, Mercury’s port configuration, the coding guarantees, and the dependency version.

The right next step is **one small `hello-port` implementation with measurable acceptance tests**, not another expanded architecture.

I cannot inspect or modify `G:\Dx\hello` or `G:\Dx\social\engine` from this conversation. The integration below is therefore a **proposed implementation boundary**, not an applied patch or validation of your existing code.

# 1. Keep source-authenticated forwarding—but correct its guarantees

Your public-mesh decision is sound:

> **The source creates authenticated pieces; untrusted relays forward those pieces unchanged.**

Bao provides verified streaming of byte ranges, and Walrus documents Merkle authentication of coded slivers. These support the general approach, but neither automatically validates your proposed `dxs-ecc` integration. 

Four corrections are necessary.

### A. A precommitted \(K+R\) symbol set is finite, not rateless

If the author commits to 64 encoded symbols, relays have 64 authenticated symbols available. They cannot invent additional authenticated repair symbols under that same finite commitment.

For v1, call this:

**“Source-authenticated systematic erasure coding with a finite repair set.”**

That is useful. It does not need the “rateless” label.

### B. Distinct symbols are not necessarily independent

For RLNC, the condition is:

\[
\operatorname{rank}(C)=K,
\]

not simply “received \(K\) different symbol IDs.”

For example, two different coefficient vectors can be scalar multiples and provide only one independent equation. If you need a strict **any-\(K\)** guarantee, select a suitable MDS code and supported parameters; do not assume generic RLNC provides it. The FEC literature explicitly distinguishes ideal any-\(K\) recovery from practical codes requiring additional symbols. 

### C. Merkle proofs authenticate commitment membership—not correct encoding

A malicious relay should not be able to modify an authenticated symbol without detection, assuming the cryptographic construction and implementation hold.

However, a malicious **author** can commit to incorrectly encoded symbols. Inclusion proofs alone do not establish that those symbols form a consistent erasure code. Walrus’s documented reconstruction procedure includes additional reconstruction and hash verification. 

Replace:

> “100% immune to pollution; not a single byte of poison can enter a buffer.”

With:

> **“Rejects unauthorized modifications to committed symbols before admitting them to the trusted decode store. Does not, by itself, prevent malicious-source encoding, spam, withholding, or resource exhaustion.”**

### D. Proof overhead and verification time must be measured

For an ordinary binary Merkle tree with **64 leaves and 32-byte sibling hashes**, a standalone inclusion path contains:

\[
6\times32=192\text{ bytes}.
\]

For a 1,024-byte symbol, that is **18.75% additional bytes**, before identifiers, coding information, and transport framing.

Bao’s actual range-proof format is not interchangeable with an arbitrary custom Merkle-tree format. Choose one specified encoding rather than calling both “Bao proofs.” 

Delete the **0.2 µs / <1 µs** claims until benchmarks separately measure parsing, manifest-signature verification, cached-manifest proof verification, and persistence.

## The live-room authentication proposal also remains unresolved

**Room membership is not proof that a recoded packet is algebraically correct.**

A shared room MAC key allows a malicious room member to authenticate its own garbage. Ordinary packet MACs also do not automatically remain valid after intermediate linear combinations.

Furthermore, TESLA deliberately uses **delayed key disclosure**, requires loose time synchronization, and introduces authentication delay. It does not establish “immediate verification with zero authentication delay.” 

For now, choose explicitly:

- **Trusted recoders:** accept and document the compromised-relay risk.
- **Untrusted relays:** forward source-authenticated pieces unchanged.
- **Untrusted recoding:** separate cryptographic research and review milestone.

Do not describe the third option as solved.

# 2. Correct the transport integrations

## Mercury: useful, but ARQ and broadcast are different interfaces

The documented defaults are:

| Interface | Default port | Purpose |
|---|---:|---|
| Control | `8300` | Commands and asynchronous status |
| ARQ data | `8301` | Application byte stream |
| Broadcast | `8100` | KISS-framed packets |

So **`127.0.0.1:8300` is not the default data socket**. Establish and manage the ARQ session through the control interface, then exchange application bytes through `8301`. 

Your description also mixes two coding mechanisms:

- Mercury’s current **ARQ** architecture documents Reed–Solomon carousel blocks and adaptive rate selection.
- Its **broadcast file transfer** uses a **RaptorQ fountain carousel**. Broadcast stations must select the same mode; that plane does not negotiate each receiver’s rate. 

Do not give the calculator a universal “1,000–4,800 bps Mercury” range. The broadcast documentation includes slower modes and reports approximately **half a minute for 1 kB at its default DATAC3 mode**. Measure useful bundle throughput for your selected mode and conditions. 

**Implementation recommendation:** start with the point-to-point ARQ adapter. Add broadcast only after durable transfer and authentication work.

Keep Mercury as an external modem process initially, and review its **GPL-3.0-or-later** distribution obligations before packaging it with Hello. 

## Android satellite: the manifest value needs changing

Google’s current documentation specifies the **application package name**, not Boolean `true`:

```xml
<!-- Replace com.example.hello with the actual application package. -->
<meta-data
    android:name="android.telephony.PROPERTY_SATELLITE_DATA_OPTIMIZED"
    android:value="com.example.hello" />
```

The API reference also identifies the public constant as added in **API 36.1**, so “Android 16+” is too coarse for version-specific implementation assumptions. 

For custom networking, Google additionally documents removing `NET_CAPABILITY_NOT_BANDWIDTH_CONSTRAINED` from the relevant network request and adapting when the network is constrained **or** uses satellite transport. Manifest metadata alone is not the entire networking integration. 

## Apple satellite: configure categories and the actual connection

Apple documents both:

- `com.apple.developer.networking.carrier-constrained.appcategory`
- `com.apple.developer.networking.carrier-constrained.app-optimized`

The optimization entitlement is not a universal satellite-service authorization. Carrier-specific availability and categories still matter. Apple explicitly includes messaging and limited-bandwidth social categories. 

For a gateway with several connections, apply policy to the **actual upstream connection**, not merely a global “phone is on satellite” flag. `NWConnection.currentPath` exposes the path that connection uses. 

Your **250-byte ceiling** can be a Hello design target. It is not an OS-defined limit or evidence of a guaranteed satellite rate.

Likewise, one phone forwarding traffic for 50 nearby devices remains a **prototype requirement**, not a capability established by these API documents.

## iOS Live Activity: promising, but not an immortal daemon

Apple does document foreground-like Bluetooth privileges in the background when the app has an instantiated Bluetooth manager and starts a Live Activity. This is worth implementing and testing. 

But a Live Activity can remain active for **up to eight hours**, and users can end it earlier. That is not a guarantee of permanent L2CAP connectivity, uninterrupted process execution, or a particular throughput. 

Your consent-based UI is a good proposal. Treat it as a **bounded relay session** that can stop and resume safely.

## `bp7`: update the version-specific assumptions

**As of October 21, 2026, the published release I verified is `bp7` 0.12.0, released October 17, 2026—not 0.10.7.** Its release notes include validation, FFI-safety, and protocol-alignment changes. Review and test those changes before choosing your pin. 

The version distinction matters:

- The **0.10.7** documentation marks its old WASM support section as defunct.
- The **0.12.0** documentation describes newer WebAssembly bindings.
- The library still explicitly says it handles **bundle encoding/decoding**, not transmission, routing, or persistent queuing. 

Adding the crate does not itself implement a data ferry.

# 3. Fix the capacity and regulatory assumptions

## A 45 Mbps link cannot load 1–10 GB in 30 seconds

Using decimal units, before overhead:

| Payload | Required rate for 30 seconds | Time at 45 Mbps |
|---|---:|---:|
| 1 GB | 267 Mbps | 2 min 58 sec |
| 10 GB | 2.67 Gbps | 29 min 38 sec |

At 45 Mbps, 30 seconds carries at most **168.75 MB**.

For the calculator, use:

\[
B_{\text{contact}}
\le
\frac{
g_{\text{measured useful}}
\max(0,T_{\text{contact}}-T_{\text{setup}})
}{8}
\]

Then apply storage, consent, energy, and airtime caps.

Also, **50 messages of 250 bytes every minute already require 1.67 kbps**, before receipts and other overhead. Fixed 60-second batching cannot prevent queue growth on a slower link.

A signed message also needs an honest wire budget: an **Ed25519 signature alone is 64 bytes**. Your “64-byte Class-0” minimum cannot describe a complete independently Ed25519-signed message with additional content and headers. 

## A cleartext flag does not establish radio compliance

Cleartext signed bulletins may be a useful mode, but `--unencrypted-bulletin` cannot certify legal operation.

For example, US amateur rules prohibit messages encoded to obscure their meaning; other operating restrictions also require consideration. A community or humanitarian label is not itself spectrum authorization. 

**Proposed safety rule:** never decrypt a private bundle merely to make it eligible for an amateur-radio route. Keep it queued for an authorized encrypted route unless the sender explicitly publishes a separate public bulletin.

# 4. The `hello-port` boundary I recommend

Start with one crate and **do not modify the live-call cryptography in milestone 1**.

Proposed structure:

```text
crates/hello-port/
  src/
    envelope.rs       bounded wire records and authenticated manifests
    store.rs          durable objects, quotas, expiry, deduplication
    receipts.rs       carrier acknowledgement vs recipient delivery
    scheduler.rs      destinations, priorities, contact byte budgets
    coding.rs         optional source-coded symbols; no public recoding
    adapters/
      file.rs         explicit ferry export/import
      local.rs        resumable local transfer
      mercury.rs      later: control session and ARQ data bridge
```

These are proposed modules, not claims about your existing workspace.

## Authenticated object boundary

For coded attachments, the signed manifest should bind at least:

```text
protocol version
author verification key
object/ciphertext digest and original length
expiry
codec identifier and version
generation layout
K, repair count, symbol size
symbol-tree commitment
```

Each authenticated symbol record must bind its **generation, symbol identifier, coding descriptor, and payload**. Authenticating payload bytes while leaving coefficients or interpretation metadata mutable would undermine the protection.

Recommended receive pipeline:

```text
Bounded parsing
    → verified manifest
    → verified symbol membership
    → durable storage
    → unchanged forwarding
```

Represent verified symbols with an opaque internal type so unverified network data cannot accidentally reach the decoder.

## Delivery states must describe evidence

Use something like:

```text
Queued → Stored by carrier → Received by destination → Application acknowledged
```

Do not mark a message delivered because a modem buffer emptied or a ferry accepted it.

BPv7 explicitly does **not** guarantee destination delivery by itself, and its delivery status reports do not mean the destination application has processed the payload. Hello needs its own authenticated application receipt and retry policy. 

Persist before acknowledging acceptance. Expect duplicate transport and make processing idempotent.

## Keep live media outside the durable message spool

My recommendation:

- Persist text, posts, attachments, and explicitly recorded voice notes.
- Do not promise to preserve every in-flight live-call packet.
- Gate voice and video separately using sustainable capacity, loss, jitter, and participant requirements—not one `30 kbps / 250 ms` threshold.
- When a call fails, offer an asynchronous message or recorded voice note; do not describe that as lossless continuation of the call.

# 5. Revised milestones and exit tests

| Milestone | Build | Required evidence |
|---|---|---|
| **1A: Durable ferry** | Two Ports, one carrier, authenticated text and resumable photo chunks | Delivery and return receipt with Internet disabled; restart after interruption without losing accepted data |
| **1B: Source coding** | Optional finite repair sets through a `dxs-ecc` adapter | Decode based on rank; reject modified symbols and metadata; survive duplicates, missing manifests, and mixed generations |
| **2: Mercury ARQ** | External-modem adapter using the documented control/data interfaces | Audio-loopback test, then an authorized radio test; measured payload throughput and recovery after disconnect |
| **3: Mobile relay/gateway** | Consent sessions and path-specific constrained-network budgets | Locked-screen tests, Live Activity expiration, battery measurements, simultaneous local/upstream operation, and carrier-policy validation |
| **4: Live integration** | Capability-gated connection to existing call transport | Per-mode call-quality tests and clear fallback to queued messaging |

For every milestone, include malformed input, storage exhaustion, priority abuse, replay, expiry, clock changes, and lost-carrier tests.

## Bottom line

**Approve the direction: Hello Ports + durable store-carry-forward + source-authenticated partial transfers + optional long-distance adapters.**

**Do not yet approve:** “pollution-proof,” “any \(K\),” sub-microsecond verification, guaranteed mobile background relaying, fixed satellite bandwidth, or lossless call fallback.

The useful product promise remains:

> **Hello preserves accepted messages and continues delivery across later contacts, carriers, and permitted gateways—even when the Internet is unavailable.**

That is consistent with the capabilities BPv7 was designed to support. Worldwide reach still depends on a deployed sequence of contacts before expiry; software cannot substitute for a missing physical route. 

For a repository-specific patch, the necessary starting files are the workspace `Cargo.toml`, the `dxs-ecc` encode/decode API, Hello’s existing transport interface, and its current identity/encryption boundary.
