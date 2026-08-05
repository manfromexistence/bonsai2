# DX DCP Achievements

`dcp` is the Development Context Protocol workspace: a Rust binary-first
protocol experiment with MCP compatibility, capability negotiation, binary
message parsing, transports, security receipts, and Python/TypeScript SDK
surfaces.

## Position

DCP should be the high-performance protocol path for DX tool and agent
communication. Its most important current achievement is not a fully proven
replacement statement; it is the amount of repository-managed protocol, compatibility,
security, transport, observability, and regression validation already in one repo.

Professional positioning:

> DCP is a Rust protocol workspace with binary envelopes, capability manifests,
> MCP/JSON-RPC compatibility adapters, signed authorized dispatch paths,
> replay/redaction guards, stdio/SSE/multiplex/TCP transport surfaces,
> observability hooks, and Python/TypeScript SDK packages.

That statement intentionally avoids saying every execution path enforces the final
security model or that current benchmarks prove universal MCP replacement. The
README itself lists remaining unproven areas around inbound signatures, SDK
parity, TCP TLS accept handling, cross-session replay isolation, stdio parity,
and multiplex replay protection.

## What DCP Has Already Achieved

| Area | Achievement |
| --- | --- |
| Binary protocol model | Defines binary envelopes, fixed-size invocation records, signed records, stream chunks, HBTP-style binary parsing, and canonical little-endian parsing boundaries. |
| Capability model | Implements bitset capability manifests, least-privilege negotiation, capacity-bound registration, duplicate rejection, and negotiated execution gates. |
| MCP compatibility | Contains legacy and complete MCP/JSON-RPC adapters, initialization lifecycle handling, tools/resources/prompts/completion surfaces, strict params validation, and migration helpers. |
| Signed dispatch path | Provides signed authorized router/server invocation paths with Ed25519 signatures, argument hashes, negotiated tool capabilities, schema validation, and replay freshness checks. |
| Replay and denial receipts | Records capability denials, request replay rejections, shutdown abuse rejection, validation receipts, and sanitized security audit events. |
| Parser hardening | Rejects malformed JSON-RPC IDs, unknown fields, reserved methods, unsupported batches, oversized payloads, invalid response shapes, null/scalar params, envelope flag errors, and truncated binary payloads. |
| Resource/prompt safety | Enforces negotiated resource/prompt IDs, configured roots, URI template literal suffix matching, subscription deduplication, and subscriber caps. |
| Transport surfaces | Includes stdio, SSE, stream chunks, multiplex, frame codec, TCP, shutdown coordination, and protocol negotiation modules with malformed-input guards. |
| Redaction and observability | Owns structured logging, tracing, metrics, bounded audit logs, dropped receipt counts, and redaction for secret-bearing field names and values. |
| Property-test corpus | Keeps a broad property/unit test surface under `tests/props` for bench, binary, capability, compat, context, dispatch, JSON-RPC, MCP, multiplex, observability, protocol, security, server, shutdown, SSE, stdio, stream, sync, and transport behavior. |
| SDK surfaces | Provides Python and TypeScript client packages with TCP, stdio, SSE, JSON-RPC, and MCP-style type surfaces. |
| Workflow policy | Keeps GitHub Actions disabled with a clear in-repo policy until CI is intentionally reopened. |

## validation To Check

Use these files and commands before repeating exact wording:

| validation | Purpose |
| --- | --- |
| `dcp/README.md` | Protocol overview, feature list, security validation ledger, validation commands, and remaining unproven areas. |
| `dcp/Cargo.toml` | Rust crate identity, CLI binary, cryptography, protocol, async, transport, and property-test dependencies. |
| `dcp/src/lib.rs` | Public module and export surface for binary, capability, compat, dispatch, transport, observability, reactor, server, shutdown, and security modules. |
| `dcp/src/binary/*.rs` | Binary envelope, invocation, signed, and stream parsing contracts. |
| `dcp/src/compat/*.rs` | MCP/JSON-RPC adapters, complete adapter, request replay, SSE, and stdio compatibility layers. |
| `dcp/src/security/*.rs` | Audit, redaction, replay, and signing boundaries. |
| `dcp/src/transport/tcp.rs` | TCP/TLS config boundary, protocol detection, connection handling, and the current plaintext refusal when TLS accept handling is not wired. |
| `dcp/tests/property_tests.rs` and `dcp/tests/props/*.rs` | Property-test entry points and focused property suites for protocol/security/transport behavior. |
| `dcp/languages/python/pyproject.toml` | Python SDK package identity and alpha status. |
| `dcp/languages/typescript/package.json` | TypeScript SDK package identity, build/test scripts, and current placeholder repository URL. |
| `dcp/.github/workflows/README.md` | Disabled GitHub Actions policy. |
| `cargo test security -j1 --quiet` and focused README validation commands | Runtime authority when current security/protocol behavior needs to be proven. |

## Public Positioning Boundary

Safe internal statement:

> DCP owns a large Rust source and test surface for binary protocol work,
> MCP/JSON-RPC compatibility, capability negotiation, signed dispatch,
> replay/redaction guards, transports, observability, and SDK experiments.

Safe public wording after refreshed checks:

> The current DCP test suite verifies the specific security, capability,
> protocol, parser, transport, and compatibility behaviors covered by the
> focused validation commands that were rerun on the current commit.

Unsafe statement without more verification:

> DCP is a fully production-ready MCP replacement with universal 10-1000x
> superiority, complete SDK parity, mandatory signatures on every inbound path,
> fully wired TCP TLS serving, and cross-session replay isolation everywhere.

The strongest current result is repository-level protocol/security depth. The
remaining verification work is fresh focused test validation, current benchmark validation,
SDK parity validation, publication metadata cleanup, TCP TLS accept enforcement,
cross-session replay scoping, and release-grade CI when Actions are reopened.

## Documentation Status

The `driven` page now documents the AI-assisted development orchestrator and
worker coordination receipt system that connects rules, sync, templates, validation,
steering, and DX strategy. The next folder should be `extensions`, because
host-extension adapters are repository-level and receipt-tracked but explicitly not
release-qualified, which needs a clear public boundary.
