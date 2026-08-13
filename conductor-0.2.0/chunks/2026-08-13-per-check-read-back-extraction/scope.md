# Scope — 2026-08-13-per-check-read-back-extraction

**Working-route entry (Epoch 2 — Live-path enablement):**
> Per-check read-back extraction — observed values from the corpus tools feeding the unchanged
> evaluate/classify path, degraded_mode mapping to KnownResidual

**Version capability:** `verification-matrix.json#v2-09` — *Real per-check read-back extraction.*

---

## What this builds

The bridge between what Pulse actually returns and what Conductor grades. Today
`crates/conductor-run/src/lib.rs:302-305` calls `query_incident_list`, throws the response away, and
substitutes the literal token `"incidents-listed"` as the observed value for **every** check in the
scenario. `evaluate_check` → `classify` then grades that placeholder. The whole comparison layer
(`compare`, the four `ComparisonKind`s, the tier-scaled SLO math, `classify`) is shipped and correct;
what is missing is the extraction that feeds it a real value.

1. **An extraction seam** turning a read-back tool result (`serde_json::Value`, the raw shape Pulse's
   hand-rolled sidecar returns) into the `observed: &str` that `evaluate_check` consumes.
2. **Per-check resolution** — today one observed string is reused across `scenario.expected`
   (`lib.rs:328-333` maps every check over the same `&observed`). "Per-check" means each
   `ExpectedCheck` is graded against the value that is actually relevant to it, so a `CountAtLeast`
   floor and a `Contains` candidate-token check stop reading the same string.
3. **`degraded_mode` → `ReportState::KnownResidual`** — a read-back served under Pulse's degraded mode
   is a pre-accepted residual, not a `Fail`. `ReportState` is **producer-assigned**: the `Scenario`
   model carries no `state` field, so this routing lives in the run/verify seam, never in a TOML.
4. **`execute_scenario`'s public signature preserved** — the composition root's shape
   (`pf, scenario, run_id, resolver → anyhow::Result<RunRecord>`) does not change; both shells
   (`conductor-cli`, `conductor-tauri`) keep calling it identically.

## Boundaries

- **Not the live green preflight.** `v2-10` / the next working-route entry owns `ready: true` against a
  real Pulse. This chunk must be provable against the hand-rolled JSON-RPC stub, with no live Pulse and
  no CI gate on one.
- **Not the family live proofs** (`v2-11`..`v2-16`, Epoch 3). This builds the extraction they will all
  consume; it does not prove any P-ID family live.
- **No new MCP tools and no new spawn code.** The four corpus-tool wrappers on `ReadbackClient`
  (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`)
  are shipped; consume them. The hardened fixed-path `.env(...)` spawn is untouched.
- **The comparison/classification layer is unchanged.** `compare`, `evaluate_check`, `evaluate_slo`,
  `classify`, `Verdict`, `ReportState` keep their semantics — the matrix acceptance says the extracted
  value drives them "unchanged". A change there would be a different chunk.
- **Verdict/error wall holds.** A malformed, absent or errored read-back becomes a typed `Blocked` /
  `Fail` **value**, never a `Result::Err` and never a panic. Transport failure stays a harness fault.
- **Redaction holds.** Observed values are redacted inside `classify` today; nothing extracted may leak
  a host path, an internal struct name, or corpus content into `runs.db` / the journal / the report.

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `crates/conductor-run/src/lib.rs` | `execute_scenario`'s read-back block (`:301-305`) + the per-check fold (`:328-342`) |
| `crates/conductor-verify/` | the extraction seam's likely home (it owns `slo.rs` / `verdict.rs` / `client.rs`); new module or an extension of `record.rs` |
| Run-report envelope | `verdict` / `state` / `latency_ms` become measured rather than placeholder-derived; the envelope's 11 fields are unchanged |
| `scenarios/*.toml` | read-only — no model change, no new `ComparisonKind`, no new field |
| `runs.db` / JSONL journal / Markdown report | unchanged schema; the values written become real |

## Absorbed annotations

**PREREQ (from `2026-08-13-dispatcher-determinism-goldens`) — `cargo audit`, TWELFTH consecutive.**
Deferred since `2026-08-08-sut-capability-manifest`, operator-RATIFIED at the 2026-08-10 wrap under the
L5 age trigger, re-pinning silently from there. It is an advisory-**DATABASE** fault — `duplicate
advisory ID: RUSTSEC-2026-0244`, reproduced byte-identically on cargo-audit 0.22.2 (the latest
published), so there is nothing to raise a floor to. Remedy is the **bounded wait alone**: re-run it,
record the result, and verify `cargo deny check` ran green as the overlapping signal. The standing
basis is *"audit SURFACE unchanged (no new `[[package]]`) + `cargo deny` verified green"* — re-verify it
literally against this chunk's own lockfile delta rather than echoing the last chunk's. Do **not** raise
the floor, do **not** add a `deny.toml` ignore, do **not** edit CI. Close the deferral the moment it
parses. (`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09/-08-10 · security-plan
§Dependency Security.)

## Open questions for P4

- `[inferred]` **Which tool serves which check.** `v2-09` names `query_incident_list` **and**
  `retrieve_telemetry_slice`; the working entry says "the corpus tools" (four exist). Which
  `ComparisonKind` reads which tool's result — and whether `retrieve_report` participates at all — is
  not stated by either source.
- `[inferred]` **How a check selects its observed value.** The `ExpectedCheck` model
  (`kind` / `class` / `expected`) carries no tool or field selector, and the scope forbids a model
  change. Whether per-check resolution is derived from `ComparisonKind`, from a composed
  multi-tool observation string, or otherwise, is P4's call.
- `[inferred]` **Where `degraded_mode` is observable.** `retrieve_report` takes a `degraded_mode`
  argument per the pinned contract manifest; whether the *response* carries a degraded flag Conductor
  can read is a premise to verify against the stub + Pulse's shapes before planning on it.
- `[inferred]` **Whether `fingerprints` gets populated.** The envelope's `fingerprints` field is written
  as `Vec::new()` at `lib.rs:341`, and `retrieve_telemetry_slice.fingerprint_refs` is the canary's
  proven fidelity carrier. Filling it is adjacent to this chunk's surface but is stated by neither the
  working entry nor `v2-09` — include or defer explicitly.
- `[inferred]` **Test tier.** `v2-09`'s `method` is `integration`; the deterministic proof surface is the
  hand-rolled `stub_pulse_mcp` child + in-process JSON-RPC stub. Confirm the stub emits shapes faithful
  enough to grade against.
