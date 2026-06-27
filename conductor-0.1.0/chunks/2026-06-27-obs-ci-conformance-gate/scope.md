# Scope — Obs CI conformance gate

**Marker:** `2026-06-27-obs-ci-conformance-gate`
**Version:** conductor-0.1.0 · **Epoch:** 10 (Polish & ship) · **Mode:** `--chunk` pull-forward (Windows-doable)
**Working entry:** _Obs CI conformance gate — agent-latest.jsonl upload + log-schema conformance + zero-unlogged-panics check_
**Working-entry annotations:** none (no `PREREQ:` / `CARRY:` to fold in).

## What this chunk builds

A GitHub Actions **observability conformance gate** layered onto the existing `rust` job in `.github/workflows/ci.yml` (the same job that already carries the coverage + flakiness gates from `2026-06-27-ci-quality-gate-config`). It is the obs analogue of that coverage gate: it produces Conductor's self-observation artifact in CI, asserts it conforms to the binding obs schema + redaction boundary, asserts zero unlogged panics, and uploads it for agent retrieval. Three concrete additions:

1. **Produce `logs/agent-latest.jsonl` in CI.** A headless agent-mode invocation (e.g. `conductor … --agent-mode` via the `agent-run` harness) emits the self-observation `tracing` JSON stream to `logs/agent-latest.jsonl` (truncated per invocation, sibling of the runs dir, moves with `CONDUCTOR_RUNS_DIR`). No live Pulse is required — a **Blocked** preflight/run still emits self-obs lines, each carrying the service-identity fields + `run_id` (obs-plan §3). This is what makes the chunk Windows-doable.

2. **Log-schema conformance check (obs CI gate — obs-plan §9).** A gate step validates **every** line of `logs/agent-latest.jsonl`:
   - parses as JSON (one object per line);
   - carries the required **self-obs base fields** — `timestamp_ms`, `level`, `target`, the service-identity fields (`service.name` / `service.version` / `deployment.environment`), and `run_id` on every line (obs-plan §3 "two record shapes" clarification, line 229). _Reconcile with the §6/§9 envelope field list — see Open question below._
   - **redaction boundary holds (this IS Conductor's own artifact — unlike `lcov.info`):** no leaked absolute host paths in any field (drive-letter `X:\`, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`), no internal struct names; the allowlisted `target` module path (e.g. `conductor_core::obs`) is a documented identity field and is **preserved**, not flagged (obs-plan §9 line 532, §6/§11; security-plan §Error Handling). On any violation the step exits non-zero → build gated.

3. **Zero-unlogged-panics gate (obs CI gate — obs-plan §9/§10).** A gate step greps `logs/agent-latest.jsonl` (+ captured stderr where applicable) for unstructured panic backtraces (`^thread.*panicked`). Any match → CI FAIL. Only formatted `tracing::error!(panic=…)` JSON events are acceptable. This asserts the `std::panic::set_hook` capture invariant (obs-plan §10 always-required SLO) at the CI level.

4. **Upload `logs/agent-latest.jsonl` as a CI artifact** via `actions/upload-artifact@v4` with `if: always()` (obs-plan §9 telemetry-artifact table), reusing the upload scaffold the coverage/JUnit steps already established. Agent downloads + `jq`-parses it.

## Boundaries (what it is NOT)

- **CI-config + a conformance-check step/script only.** Mirrors the `ci-quality-gate-config` precedent: **zero engine/seam/model change** expected. If a tiny helper is needed (a `scripts/*` conformance checker, or a `#[test]` exercising the redaction allowlist), it stays presentational/tooling — it does not touch `conductor-core`/seam types.
- **No live Pulse, no Linux+xvfb, no webview** — fully runnable on the Windows runner (`windows-latest`), which is why it was pulled forward from Epoch 10 ahead of the display-gated "Desktop a11y verification".
- **Not the A11y CI gate** (Epoch 10, separate chunk) — that one routes axe/contrast/keyboard violations into the obs envelope and reuses this chunk's upload + gate-step + zero-retry scaffold.
- **Does not re-author the obs schema** — it asserts the existing obs-plan §3/§6 contract; it does not change what Conductor logs.
- **Redaction boundary scope:** this gate fires on **`agent-latest.jsonl` (Conductor's own artifact)** where redaction DOES apply — explicitly distinct from the `ci-quality-gate-config` finding that a third-party tool's `lcov.info` is outside the boundary (security.md Session Addition 2026-06-27).

## Surfaces / contracts touched

- `.github/workflows/ci.yml` — the `rust` job: add an agent-mode run that produces `logs/agent-latest.jsonl`, the conformance + zero-panics gate steps, and the artifact upload (`if: always()`). Possible new permission: none expected (read-only `contents`).
- `scripts/agent-run.{sh,ps1}` — the harness `run`/`logs` verbs already reference `logs/agent-latest.jsonl`; the gate may invoke an existing verb rather than a bespoke command (confirm in research).
- `crates/conductor-core/src/obs.rs` — the self-obs sink + `set_hook` panic capture (the behavior being asserted; read-only reference, not expected to change).
- obs-plan §3 (log JSON schema + two-record-shape clarification) · §6 (log coverage / required fields) · §9 (CI integration — log conformance check + zero-unlogged-panics gate, the spec) · §10 (zero-unlogged-panics SLO) · §11 (CI bans).
- test-plan §9 (CI integration) / §10 (quality gates) — sibling CI-gate conventions.
- The redaction boundary (obs-plan §6/§11 + security-plan §Error Handling).

## Open question (carry into P4 — plan)

The handoff flags an unresolved **`test-plan §3 ↔ obs-plan §3` dual-RECORD-SHAPE** tension: obs-plan §9's conformance text (line 531) lists the **run-report envelope** fields (`journal_emitted_at`, `seed`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`…), but `agent-latest.jsonl` is the **self-obs stream**, whose lines carry the smaller base set (`timestamp_ms`, `level`, `target`, service identity, `run_id`) per §3 line 229 — the envelope fields live on the `runs/<run_id>.jsonl` scenario-result record, not on every self-obs line. The plan must decide which schema the gate asserts against `agent-latest.jsonl` (expected: the **self-obs base set**, since that is what the file contains) and whether it also conformance-checks `runs/<run_id>.jsonl` against the envelope schema. Resolve via AskUserQuestion in P4 if the extracts/research do not settle it.

## Definition of done (intent-level; acceptance criteria finalized in plan.md)

- CI produces `logs/agent-latest.jsonl` on the Windows runner without a live Pulse.
- The conformance gate FAILS the build on a missing required field, a leaked absolute host path, or a struct-name leak; PASSES on the allowlisted module-path identity field.
- The zero-unlogged-panics gate FAILS on any `^thread.*panicked` line; PASSES when only formatted panic JSON is present.
- `agent-latest.jsonl` is uploaded as a CI artifact with `if: always()`.
- `ci.yml` remains valid YAML; the new steps reuse the established gate-step / `shell: bash` multi-command pattern; **zero engine/seam diff**.
