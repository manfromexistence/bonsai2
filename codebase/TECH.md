# TECH.md - `codebase/`: two vendored code-graph MCP servers, compared

**Scope.** `G:\Dx\codebase\` holds two independent git checkouts of third-party
projects. This document answers three questions about them: what they are, whether
they do the same thing, and whether either has what could be called "codebase
memory".

**How the facts below were obtained.** Every statement about the two projects is
*repo-managed*: it was read out of the files in the working tree, and the
`file:line` is named. Every statement about this machine is *executed*: the
command is named. **Neither project was built, installed, or run here.** Every
benchmark number attributed to either project is **upstream-claimed, not measured
in this hub** - do not quote it as a DX result.

**Provenance.**

| Folder | Upstream remote | Upstream HEAD at check time | Commits | Licence |
| --- | --- | --- | --- | --- |
| `codebase/codebase-memory-mcp` | `https://github.com/DeusData/codebase-memory-mcp` | `a222715` (2026-10-20) | 4381 | MIT (DeusData, 2025) |
| `codebase/codegraph` | `https://github.com/colbymchenry/codegraph` | `8beab37` (2026-10-20) | 1535 | MIT (Colby Mchenry, 2026) |

Different authors, different repositories, no shared history. Both were updated
on 2026-10-20, the day before this document was written.

---

## The three questions, answered

| Question | Answer |
| --- | --- |
| Are they the same thing? | **Same category, same interface, different depth.** Both are local-first MCP servers that parse a codebase with tree-sitter into a symbol/edge graph in SQLite and serve it to coding agents. Past that they diverge sharply, and they even disagree on agent interface design. |
| Do they have "codebase memory"? | **Asymmetric.** `codebase-memory-mcp` has a real, multi-layer memory story (7 layers, 5 of them present). `codegraph` has 2 of 7: a persistent derived index and a structural graph. It has no decision log, no embeddings, no trace memory, no cross-repo memory. |
| Are they used here? | **Neither.** No CLI on `PATH`, no index directory anywhere under `G:\Dx`, and no MCP entry in this machine's opencode config. They are reference checkouts, not tools in service. |

---

## Project A - `codebase-memory-mcp` (DeusData)

A native **C** codebase-knowledge-graph server. One executable is simultaneously
the MCP server, the CLI, the installer, and the update agent
(`codebase-memory-mcp/README.md:1046-1057`).

| Property | Value | Evidence |
| --- | --- | --- |
| Language | Pure C. No language runtime, no API key, no hosted service | `README.md:18`, `README.md:66` |
| Tree-sitter | 162 grammars vendored and compiled into the binary | `README.md:1032`, `internal/cbm/vendored/grammars/MANIFEST.md` |
| Type resolution | "Hybrid LSP": a C implementation of language-server type resolution, run alongside tree-sitter. Full for 11 language rows (Python, TS/JS/JSX/TSX, PHP, C#, Go, C/C++, Java, Kotlin, Rust, Perl) | `README.md:1000-1028` |
| MCP tools | 17 (5 indexing + 12 querying) | `README.md:826-880` |
| Client surfaces | 45 (39 auto-detected, 6 conditional) | `README.md:630` |
| Query language | A read-only openCypher subset with its own lexer, parser, planner, executor | `README.md:902-912`, `src/cypher/` |
| Indexing pipeline | 33 passes: definitions, calls, imports, HTTP links, config, tests, K8s, Docker, Terraform, Ansible, CI, git history, similarity, semantics, cross-repo, and more | `src/pipeline/pass_*.c` (33 files) |
| Graph data model | 31 node labels, 34 edge types, including infra nodes (`Resource`, `Chart`, `Image`, `EnvVar`, `Table`, `Branch`) and cross-repo edges | `README.md:888-894` |
| Storage | One account-wide SQLite store at `${CBM_CACHE_DIR}` (default `~/.cache/codebase-memory-mcp`) holding every project's index and the config | `README.md:944`, `src/store/store.c` |
| Process model | Detached daemon with IPC, admission barriers, version cohorts, and shared jobs; MCP frontends are thin clients | `src/daemon/`, `README.md:145-157` |
| UI | "CBM Lens": React + Monaco + Three.js browser frontend - IDE-style source reader, local chat, 3D architecture map, Galaxy graph, ADR editing | `README.md:1046-1059`, `graph-ui/` |
| Tests | 9,388 C test cases claimed by the project badge | `README.md:6` (upstream-claimed) |
| Release posture | v0.11.0 shipped; CBM Lens Beta is on `main` only and **not in the release binary** | `README.md:26` |
| Supply chain | SLSA L3, Sigstore cosign, SHA-256 sums, CodeQL, per-release VirusTotal scans | `README.md:1066-1088` |

**Tool surface (17).** Indexing: `index_repository`, `list_projects`,
`delete_project`, `index_status`, `check_index_coverage`. Querying:
`search_graph`, `trace_path`, `detect_changes`, `query_graph`, `get_graph_schema`,
`compare_graphs`, `get_code_snippet`, `get_file_outline`, `get_architecture`,
`search_code`, `manage_adr`, `ingest_traces`.

---

## Project B - `codegraph` (colbymchenry)

A TypeScript/Node code-intelligence library + CLI + MCP server, with a native
**Rust** extraction kernel. Distributed on npm as `@colbymchenry/codegraph`
(`codegraph/package.json:2-4`).

| Property | Value | Evidence |
| --- | --- | --- |
| Language | TypeScript on Node, plus a Rust `cdylib` kernel behind napi | `codegraph-kernel/Cargo.toml` |
| Tree-sitter | Rust kernel for 20 languages; a wasm (`web-tree-sitter`) fallback path for the rest, with a test asserting both produce identical graphs | `codegraph/README.md:515`, `codegraph-kernel/Cargo.toml` |
| MCP tools | 8 exist; **only `codegraph_explore` is listed by default**. The other 7 (`node`, `search`, `callers`, `callees`, `impact`, `files`, `status`) are hidden unless `CODEGRAPH_MCP_TOOLS` is set | `codegraph/README.md:582-592`, `codegraph/src/mcp/tools.ts` |
| Client surfaces | 9 agents wired by the installer: Claude Code, Cursor, Codex CLI, opencode, Hermes, Gemini, Antigravity, Kiro, GitHub Copilot | `codegraph/README.md:794-806` |
| Query language | FTS5 full-text search plus a custom query parser. No Cypher | `codegraph/src/search/`, `codegraph/src/db/schema.sql:117` |
| Resolution | Import resolver, name matcher, 35 framework resolvers (Express, Next.js, Laravel, Rails, FastAPI, Django, Gin, Axum, Spring, Vapor, React Router, Expo, SvelteKit, Nuxt, Terraform, Play, and more) | `codegraph/src/resolution/frameworks/` (35 files) |
| Dynamic dispatch | A synthesis layer for flows static parsing cannot see: callbacks, EventEmitter, React re-render, JSX children, React Native native-to-JS events, django ORM descriptors, and a cross-tier pass | `codegraph/README.md`, `codegraph/src/resolution/callback-synthesizer.ts` |
| Storage | Per-project SQLite at `<project>/.codegraph/codegraph.db`, with a `CODEGRAPH_DIR` override and a separate `.codegraph-wsl` directory so WSL and Windows never share one index | `codegraph/src/db/index.ts:863-876`, `codegraph/src/directory.ts` |
| Process model | Per-project daemon with named pipes on Windows, PID and socket liveness watchdogs, writer locks, a query pool and worker pool | `codegraph/src/mcp/daemon*.ts` |
| UI | Svelte browser viewer. **Gated behind `CODEGRAPH_UI=1` and not released** - `codegraph ui` and `codegraph web` are refused without it | `codegraph/AGENTS.md` (`src/bin/viewer-gate.ts`) |
| Coverage | Measured cross-file coverage per language on named benchmark repos (95.8% TypeScript, 100% Python, 96.6% Go, 86.7% Rust, and so on) | `codegraph/README.md:849+` (upstream-claimed) |
| Telemetry | Anonymous usage stats, **default on**, opt out via `codegraph telemetry off`, `CODEGRAPH_TELEMETRY=0`, or `DO_NOT_TRACK=1` | `codegraph/TELEMETRY.md:1-30` |

**Design doctrine, stated plainly.** `codegraph/README.md:584`: *"one strong tool
steers agents better than a menu of narrower ones - fewer mis-picks, and it saves
context every session."* The seven secondary tools are deliberately unlisted
because their output already arrives inline on `codegraph_explore`. Its own
`AGENTS.md` records the experiments behind that: trace-first steering was tried
three ways and **regressed** wall-clock against baseline, so the project moved the
lever from "steer the agent" to "make the answer sufficient enough that the agent
stops reading".

---

## Do they do the same thing?

**Overlap.** Both build a static symbol/edge graph from tree-sitter parses, store
it in SQLite, resolve references across files, watch the project for changes, and
expose it to Claude Code / Cursor / Codex / opencode over MCP. On that axis they
are the same product.

**Divergence.** Past that point they are different products with different
ambitions.

| Axis | `codebase-memory-mcp` | `codegraph` |
| --- | --- | --- |
| Product identity | A knowledge-graph **platform** for code - graph database, query language, UI, daemon | A **retrieval engine** for agents - one tool, minimal context, maximum precision |
| Tool surface philosophy | Wide: 17 tools, each a specialist | Narrow: 1 exposed tool by design, 7 hidden |
| Language breadth | 162 grammars, benchmarked and tier-scored | 34 languages with per-language support notes |
| Resolution depth | Hybrid LSP type resolution on 11 language rows; 33 specialised passes | Name matching + 35 framework resolvers + dynamic-dispatch synthesis |
| Query language | openCypher subset | FTS5 + custom parser |
| Infra-as-code | First-class: Docker, compose, K8s, Helm, Terraform, Ansible, CI are indexed as graph nodes with cross-references | Terraform and Play get framework resolvers; no equivalent node model |
| Cross-repo | A first-class mode (`cross-repo-intelligence`, `CROSS_*` edges) | Single-project indexes; one session can query several by `projectPath` |
| Browser UI | Shipped in the release, 3D and IDE-style | Exists, but unreleased and gated |
| Runtime | Single native binary, no Node, no Docker | Bundles its own Node runtime; npm or curl install |
| Cross-repo identity | One shared account-wide store | One `.codegraph/` per project |

**Direct, documented disagreement.** The two projects made opposite bets on the
same question - how many tools should an agent see - and each published why.
`codebase-memory-mcp` ships a 17-tool menu with a full Cypher surface.
`codegraph` collapses to one tool and hides the rest, because its own measurements
showed a menu gets mis-picked. Neither is wrong; they are answers to different
questions, and picking one means picking its philosophy.

**Can you run both?** Nothing in either project prevents it. They use different
storage roots, different tool names, and different config files. But running two
competing code-graph MCP servers in one agent session means the agent chooses
between two sources of truth about the same code, with no rule for which wins.
That is a design decision, not a technical obstacle.

---

## Do they have "codebase memory"?

"Codebase memory" is ambiguous, and one of these projects is *named* after it, so
the question deserves a definition before an answer. Seven layers, from thinnest
to thickest:

| # | Layer | What it means | `codebase-memory-mcp` | `codegraph` |
| --- | --- | --- | --- | --- |
| L1 | Persistent index | A derived index that survives restarts and is reused across sessions | **Yes** - one account-wide store at `~/.cache/codebase-memory-mcp` | **Yes** - per-project `.codegraph/codegraph.db` |
| L2 | Structural graph | Symbols, calls, imports, inheritance, resolved across files | **Yes** - 162 languages, Hybrid LSP types | **Yes** - 34 languages, framework + synthesis resolvers |
| L3 | Semantic retrieval | Similarity search over meaning, not just tokens | **Yes** - 11-signal algorithmic embeddings, 768-dim random indexing, RoTSQ compression, `SEMANTICALLY_RELATED` edges | **No** - FTS5 lexical only. Its own code says "deterministic, no embeddings" (`codegraph/src/mcp/tools.ts:3926`) |
| L4 | Decision memory | A durable record of *why* the code is the way it is | **Yes** - `manage_adr` CRUD, byte-preserving section splice, ADR nodes in the graph | **No** - zero occurrences of "ADR" or "architecture decision" in `codegraph/src/` |
| L5 | Runtime memory | Observed behaviour, not inferred structure | **Yes** - `ingest_traces` stores observed call paths under a run label | **No** |
| L6 | Cross-repo memory | Knowledge spanning several repositories | **Yes** - `cross-repo-intelligence` mode, 6 `CROSS_*` edge types | **No** - one index per project, no cross-repo edge model |
| L7 | Visual memory | A browsable, spatial model of the system | **Yes** - CBM Lens: 3D architecture map, Galaxy, IDE-style reader, ADR editing in the browser | **Partial** - the viewer exists but is unreleased and gated behind `CODEGRAPH_UI=1` |

**Verdict.** `codebase-memory-mcp` has a genuine memory stack: 5 of 7 layers, with
L3, L4 and L5 being real, separate subsystems in the source
(`src/semantic/semantic.h` - 11 similarity signals; `src/store/store.c` - ADR
storage; `src/traces/traces.c` - runtime traces). `codegraph` has L1 and L2 and
nothing else. Its graph is a *re-derivable projection of the source tree*: wipe
`.codegraph/` and the next `codegraph init` rebuilds an equivalent graph, and
nothing that was not in the source is recovered.

**Two honest caveats.**

1. **Both are derived memories.** Neither stores agent conversation memory. Neither
   remembers what you asked it last week. `codegraph`'s references to "auto-memory"
   (`codegraph/docs/design/callback-edge-synthesis.md:228`) point at the *host
   agent's* auto-memory, not at its own.
2. **A naming trap in CBM.** `src/store/graph_mem.c` is **not** a memory feature.
   Its own header explains it: it is a read-only in-RAM CSR cache of the edge
   adjacency, built to avoid SQLite B-tree seeks on kernel-scale indexes
   (`src/store/graph_mem.h:1-30`). "Graph memory" there means "the graph, held in
   RAM". Do not read it as a memory subsystem.

---

## What is actually on this machine

Measured, not inferred. Commands are named so they can be re-run.

| Check | Command | Result |
| --- | --- | --- |
| CLI on `PATH` | `Get-Command codegraph, codebase-memory-mcp, cbm` | All three **NOT FOUND** |
| CBM store | `Test-Path "$env:USERPROFILE\.cache\codebase-memory-mcp"` | **False** - never created |
| CBM data root | `Test-Path "$env:USERPROFILE\.cache"` | True, but the CBM subdirectory is absent |
| Per-project indexes | `Get-ChildItem G:\Dx -Directory -Recurse -Depth 2` filtered on `.codegraph` / `.cbm` / `.codebase-memory` | **No matches** |
| codegraph dir override | `Test-Path "$env:USERPROFILE\.codegraph"` | **False** |
| Agent wiring | `Get-Content "$env:USERPROFILE\.config\opencode\opencode.jsonc"` | No `mcp` block at all - neither server is registered with opencode |
| Node deps / build output | `Test-Path codebase\codegraph\node_modules`, `codebase\codebase-memory-mcp\build` | Both **False** - neither tree is built |

**Unexplained residue, flagged rather than claimed.** Eight directories exist at
`%LOCALAPPDATA%\codegraph\<16-hex>\<16-hex>\`, each holding a 53,248-byte
`store.db`, all dated 2026-07-07 12:15. Their schema is **not** codegraph v1.6.2's:
they have a `graph` column on `edges`, a `files(path, mtime, hash, indexed_at)`
table, and `hash TEXT ... -- Content hash (xxh3)`, whereas v1.6.2 uses
`codegraph.db` with `nodes(id, kind, name, ...)`, `files(path, content_hash, ...)`
(`codegraph/src/db/schema.sql:20-77`). The strings `store.db` and `xxh3` appear
**nowhere** in the codegraph repository. Conclusion: residue from some other tool
that used the name "codegraph", not an index of either project in this folder. It
is also not a codegraph install - the install directory is
`%LOCALAPPDATA%\codegraph` itself (`codegraph/src/upgrade/remove-binary.ts:27`),
and no binary is present. Left in place: it is 426 KB total and the hub's rule is
proposal, not deletion (`AGENTS.md` section 2, rule 2).

---

## How they relate to the hub

**They are not DX products.** No `dx <tool>` route exists for either. The `dx`
manifest's `paths` block does not mention `codebase`. The hub README records one
of them under "Vendored third-party workspaces" as *"Vendored reference. Not
DX-authored, not registered."* (`README.md:133`).

**The README table is one row short.** It lists `codebase/codegraph` but not
`codebase/codebase-memory-mcp`, which exists on disk with a real upstream remote
and a clean working tree. `CHANGELOG.md:824` records that a "broken
`codebase-memory-mcp` clone" was removed on 2026-10-20; the folder's
last-write time is 2026-10-20 22:40, i.e. it was re-cloned after that entry was
written. The table has not caught up. The row is corrected in the same commit as
this file.

**They are child repos, two levels down.** `G:\Dx\codebase` is not a git repo of
its own (`git -C G:\Dx\codebase rev-parse --show-toplevel` returns `G:/Dx`), but
`codebase/codebase-memory-mcp` and `codebase/codegraph` each are, with their own
`origin` remotes. That satisfies the hub's rule-1 shape: they must never be
absorbed into the hub's index. The `.gitignore` change shipped with this file
keeps that true - it tracks `codebase/TECH.md` and nothing else beneath
`codebase/`, so `git add codebase` stages one file and cannot reach a nested
repository.

**No `docs/<tool>.md` page is owed.** The register in `docs/README.md` covers DX
achievements. These are upstream projects with their own achievement records;
this file is the hub's note on them, not a DX claim.

---

## Practical notes, if either is ever adopted

- **Install path (CBM).** `install.ps1` is present and Windows-native. The
  project documents a known Defender false positive on release binaries
  (`Trojan:Script/Wacatac.B!ml`, `README.md:103-109`) - verify hashes before
  flagging it as malware.
- **Install path (codegraph).** `irm .../install.ps1 | iex` or `npm i -g
  @colbymchenry/codegraph`. The installer wires agents but does **not** index;
  `codegraph init` is a separate step (`codegraph/README.md:108`). It is a
  documented consent surface - the installer asks about telemetry and defaults it
  on.
- **CBM Lens is not in the v0.11.0 binary.** Building from `main` is required to
  see the UI (`codebase-memory-mcp/README.md:26`).
- **Disk.** The hub's `G:` is at 97% used with ~12.7 GB free. A Linux-kernel-scale
  index is not a proposition on this volume. `codegraph` is the smaller footprint
  by design (one DB per project); CBM's 33-pass pipeline on a large repo will
  want room and RAM (`CBM_MEM_BUDGET_MB`, `CBM_WORKERS`).
- **Neither conflicts with the TPU track.** `AGENTS.md` section 2, rule 7 governs
  Bonsai/TPU throughput claims. Neither project is a throughput measurement, and
  neither result may be filed as TPU track work.
- **For the hub's own repos**, a code-graph tool is a plausible way to make the
  `docs/<tool>.md` register and the `dx` manifest checkable. That is a proposal
  to file in `TODO.md` with evidence, not a task to start silently.

---

## Evidence index

Every claim above resolves to one of these. Paths are relative to
`G:\Dx\codebase\`.

| Claim | File:line |
| --- | --- |
| CBM is pure C, 17 tools, 45 client surfaces | `codebase-memory-mcp/README.md:18`, `:24`, `:630` |
| CBM tool names, indexing and querying | `codebase-memory-mcp/README.md:826-880` |
| CBM node labels and edge types | `codebase-memory-mcp/README.md:888-894` |
| CBM Cypher subset | `codebase-memory-mcp/README.md:902-912` |
| CBM storage root `~/.cache/codebase-memory-mcp` | `codebase-memory-mcp/README.md:944` |
| CBM 162 languages, tiered benchmark | `codebase-memory-mcp/README.md:1030-1040` |
| CBM architecture map (33 passes) | `codebase-memory-mcp/README.md:1042-1060` |
| CBM Hybrid LSP, 11 language rows | `codebase-memory-mcp/README.md:1000-1028` |
| CBM semantic embeddings, 11 signals, 768-dim | `codebase-memory-mcp/src/semantic/semantic.h:1-45` |
| CBM ADR + trace subsystems | `codebase-memory-mcp/src/store/store.c`, `codebase-memory-mcp/src/traces/traces.c` |
| CBM `graph_mem` is a RAM cache, not a memory feature | `codebase-memory-mcp/src/store/graph_mem.h:1-30` |
| CBM 33 pipeline passes | `codebase-memory-mcp/src/pipeline/pass_*.c` |
| CBM Lens Beta not in the release binary | `codebase-memory-mcp/README.md:26` |
| CBM Defender false positive | `codebase-memory-mcp/README.md:103-109` |
| codegraph version 1.6.2, npm name | `codegraph/package.json:2-4` |
| codegraph single-tool doctrine | `codegraph/README.md:582-592` |
| codegraph pipeline and storage | `codegraph/README.md:492-521` |
| codegraph CLI reference | `codegraph/README.md:525-550` |
| codegraph 34 supported languages | `codegraph/README.md:810-847` |
| codegraph 9 agent surfaces | `codegraph/README.md:794-806` |
| codegraph measured cross-file coverage | `codegraph/README.md:849+` |
| codegraph per-project DB path | `codegraph/src/db/index.ts:863-876` |
| codegraph `CODEGRAPH_DIR` / WSL separation | `codegraph/src/directory.ts:1-70` |
| codegraph "deterministic, no embeddings" | `codegraph/src/mcp/tools.ts:3926` |
| codegraph viewer unreleased and gated | `codegraph/AGENTS.md` (`src/bin/viewer-gate.ts`) |
| codegraph telemetry default on | `codegraph/TELEMETRY.md:1-30` |
| codegraph install dir `%LOCALAPPDATA%\codegraph` | `codegraph/src/upgrade/remove-binary.ts:27` |
| codegraph Rust kernel + grammar pinning | `codegraph/codegraph-kernel/Cargo.toml` |
| codegraph 35 framework resolvers | `codegraph/src/resolution/frameworks/` |
| Hub README lists only one of the two | `README.md:133` (hub root) |
| Hub CHANGELOG records the removed clone | `CHANGELOG.md:824` (hub root) |

---

MIT OR Apache-2.0
updated: 2026-10-21 00:00 UTC
