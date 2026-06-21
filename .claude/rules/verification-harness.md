---
paths:
  - "scripts/agent-run.*"
  - "crates/conductor-cli/**"
  - "crates/conductor-verify/**"
  - "crates/**/tests/**"
---

# Verification Harness Rules

Path-scoped rules for the agent-driven verification harness — the `scripts/agent-run.*` driver, the CLI release-gate path, and the MCP read-back / preflight layer.

**Authoritative sources (bound):** `.andromeda/test-plan.md` §3 (Test Harness Contract) + `.andromeda/obs-plan.md` §3 (log format / service identity). The harness consumes the obs JSON log format; keep both sides in sync.

**Critical adaptation — Conductor has NO daemon and NO inbound listener.** `boot` is a preflight gate (not a daemon start), `status` reads disk artifacts (no HTTP/IPC endpoint), there is NO PID file, and there is no long-running heartbeat. Do not reintroduce daemon/PID/endpoint machinery.

## 5-command discipline (`scripts/agent-run.{sh,ps1}` — exactly 5, identical semantics across .sh and .ps1)
- **`boot`** — MCP preflight readiness gate: `conductor preflight --json` runs the rmcp `initialize` handshake (assert negotiated protocol `2024-11-05` + required-tool presence vs the pinned `contracts/` manifest + a data-dir canary round-trip). `ready:true` → exit 0; `ready:false` ⇒ every dependent scenario reports `Blocked` (never a silent downgrade). CI uses the rmcp stub (returns `ready:true`); the live-Pulse leg is `workflow_dispatch`/local only. Timeout 30s.
- **`run`** — `cargo nextest run --workspace --profile ci` + `cargo test --workspace --doc` + `cargo clippy --workspace --all-targets -- -D warnings`; scenario leg `conductor run <scenario|P-ID> --seed <s>` / `conductor suite`. Exit 0 = all `Pass`; non-zero = a hard `Fail` (nextest 100). Stage flags `--unit`/`--integration`/`--e2e`.
- **`status`** — read the Run-report envelope from `runs/<run_id>.jsonl` (`jq -e`) or the `runs.db` row (rusqlite bound-param SELECT). No `curl`/HTTP — there is no listener.
- **`cleanup`** — `rm -f runs/<run_id>.{jsonl,md}` + bound-param `DELETE FROM runs WHERE run_id=?1` + release the `:4317` port-occupier bind. Idempotent (re-run on a clean state is a no-op). NEVER touch Pulse (no process management).
- **`logs`** — read `runs/<run_id>.jsonl` (the emission journal — the left side of every SLO check; wall-clock `std::time` stamps) + sanitized stderr.

Do not add a 6th command without a test-plan amendment — the harness is small by design.

## Status envelope contract
- The Run-report envelope (`run_id`, `seed`, `scenario`, `p_ids`, `verdict ∈ {Pass,Fail,CalibrationRegion}`, `state ∈ {Pass,Fail,ManualCheck,KnownResidual,Blocked}`, `latency_ms`, `slo_tier`, `journal_emitted_at`, `read_back_observed_at`, `fingerprints`) is serialized identically into the Markdown report, the `runs.db` row, and each JSONL line.
- A `blocked` row populates identity fields only (measurement fields `null`); cross-run latency/percentile + P-036 fingerprint queries MUST exclude null-measurement rows.
- Artifacts MUST NOT leak absolute host paths or internal struct names.

## Headless invariant (agent-driven)
- The headless source-of-truth path is NEVER blocked on an interactive prompt — `inquire` operator-pause prompts are `isatty`-gated; when stdin is not a TTY the prompt is skipped per the non-interactive policy and the decision recorded to the artifact. An auth/interactive gate there silently breaks the release gate.
- Integration tests drive the same contract as production (rmcp stub over stdio; `Command::cargo_bin` for the cli) — no pixel-scraping, no UI automation depending on screen geometry.

## Determinism
- Same scenario+seed ⇒ same stream shape (seeded `conductor-timeline` + `start_paused` for scheduling). Journal stamps from `std::time::SystemTime`/`Instant`, never the virtual clock. Zero nextest retries.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-06-21: rmcp 1.7.0 client — pin the handshake to Pulse's `2024-11-05` by passing a `ClientInfo` (it `impl`s `ClientHandler`) built with `.with_protocol_version(ProtocolVersion::V_2024_11_05)` AS the `serve()`/`serve_client()` service; the rmcp client default is `ProtocolVersion::LATEST`, so an unconfigured client silently negotiates UP — the exact mismatch the preflight guards. Read the negotiated version back via `RunningService::peer_info().protocol_version` (`RunningService` derefs to `Peer`, giving `call_tool`/`list_all_tools`). SEP-1319: param/result structs (`CallToolRequestParams`, `ClientInfo`=`InitializeRequestParams`, `Implementation`, `InitializeResult`) are `#[non_exhaustive]` → build via `::new()`/`with_*()`, never struct literals. (mcp-read-back-client chunk)
