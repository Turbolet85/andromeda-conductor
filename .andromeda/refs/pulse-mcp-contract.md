# Pulse MCP read-back contract — pinned from Pulse source (2026-06-14)

> Conductor's `auto`-class verification rides Pulse's MCP sidecar. These four facts were read directly from
> the Pulse repo (`crates/mcp-server/`), NOT inferred — they correct/complete the arch draft's MCP sections.
> Source of truth: `D:\dev\projects\andromeda-pulse\crates\mcp-server\`.

## 1. Transport — stdio child-process ✅ (arch got the shape right)

- Standalone binary **`andromeda-pulse-mcp`** (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs`,
  `[[bin]]` in `crates/mcp-server/Cargo.toml`). Built ONLY with `cargo build --features mcp-server`
  (non-default; double-gated by the `mcp-server` cargo feature + runtime env, see §3).
- Speaks **line-framed JSON-RPC 2.0 over stdin/stdout** (`run_stdio_loop`: `tokio::io::stdin()` lines →
  dispatch → `stdout.write_all(bytes + "\n").flush()`).
- Conductor: spawn it via rmcp `TokioChildProcess` (stdio). Correct as drafted.

## 2. Protocol version — `2024-11-05` (arch draft hard-coded the WRONG number)

- `crates/mcp-server/src/jsonrpc.rs:7` → `MCP_PROTOCOL_VERSION = "2024-11-05"`. The `initialize` result
  returns exactly this string + a **minimal `capabilities` object** + `serverInfo`.
- Pulse's server is **hand-rolled** JSON-RPC, NOT rmcp (`use rmcp as _;` — the dep is pulled only to satisfy
  the feature gate; the wire protocol is `jsonrpc.rs` by hand).
- **Consequence for Conductor:** the rmcp CLIENT must negotiate/accept `2024-11-05` (older than rmcp's
  default) and tolerate Pulse's minimal capabilities. The arch draft's readiness-gate example showed
  `negotiated_protocol_version: "2025-11-25"` — replace with `2024-11-05`. The pinned contract manifest
  asserts `2024-11-05`; a strict newer-version default in the rmcp client is the exact silent-mismatch the
  preflight exists to catch — pin it explicitly.

## 3. Data-dir coordination — THE load-bearing fact (missing from the arch draft)

- The sidecar reads the corpus at **`{data_dir}/corpus/corpus.db`** where `data_dir = resolve_data_dir()`:
  `ANDROMEDA_PULSE_DATA_DIR` if set, else the platform default (`%APPDATA%\andromeda-pulse` on Windows /
  `$XDG_CONFIG_HOME` or `~/.andromeda-pulse` on Linux).
- The incident **`workspace_root` is derived as `data_dir.to_string_lossy()`** (matches the live app's
  `incident_workspace_key`). The read-back tools filter by workspace.
- **Therefore:** Conductor MUST spawn the sidecar with `ANDROMEDA_PULSE_DATA_DIR` set to the SAME directory
  the live Pulse-under-test is using. Otherwise `query_incident_list` reads an empty/foreign corpus and every
  auto scenario silently reports zero incidents (false pass-as-empty or perpetual blocked). This belongs in:
  the readiness gate, Occupied Resources (env), and Cross-cutting config of the arch.
- **Preflight must round-trip a CANARY**, not just check tool presence: emit one known incident, then assert
  `query_incident_list` returns it from the shared corpus — proves the data-dir + workspace wiring end-to-end
  before any scenario trusts read-back.

## 4. Encrypted corpus + concurrent read (flag — verify in preflight)

- `corpus.db` is **encrypted at rest** (P-049); the sidecar opens it via
  `OsKeychainBackend::new("com.andromeda.pulse")` → needs the SAME OS user + keychain entry as the live
  Pulse (automatic on one dev host, same user). `Corpus::open` non-fatal on keychain failure — it degrades
  to incident tools absent, which Conductor must read as **blocked**, not pass.
- The sidecar opens a **second connection** to a corpus the live Pulse holds open for writing. Read-while-
  write across two processes is the integration risk; the preflight canary (§3) is also the proof this works
  on the host. **Prior art to mine:** `crates/mcp-server/tests/sidecar_subprocess.rs` (the existing
  two-process subprocess test).

## Workspace attribution note (affects P-043 / P-032 / connection-lifecycle scenarios)

Conductor's OTLP `service.name` does NOT set Pulse's workspace — Pulse attributes incidents to its
data-dir-derived workspace. So "which workspace does the Pulse-under-test file Conductor's incidents under"
= the data-dir from §3. Pin one data-dir per run; the P-032 git-grounding scenarios additionally need that
data-dir to BE (or sit inside) a real git workspace with known commits.

## Net for the arch review

Ratify the draft, but feed these into the review dialogue so three arch sections carry real values:
- **Established Decisions [MCP Read-Back] + Standard Contracts [readiness gate]:** protocol `2024-11-05`
  (not 2025-11-25); add the canary round-trip to the preflight.
- **Occupied Resources [env] + Cross-cutting [config]:** add `ANDROMEDA_PULSE_DATA_DIR` (must equal the live
  Pulse's) alongside `ANDROMEDA_PULSE_MCP_ENABLED` + the `mcp-server` feature.
- Everything else in the MCP seam (stdio child-process, blocked-state, version-pinned manifest, preflight
  gate) is correct as drafted.
