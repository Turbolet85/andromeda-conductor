# Report — 2026-08-10-workspace-key-divergence-probe

**Chunk:** Workspace-key divergence probe — the two-launch verdict recorded + an app/sidecar workspace-key
mismatch detected as a distinct host-path-free named precondition, replacing the opaque corpus Blocked
(conductor-verify, v2-17)
**Date:** 2026-08-10
**Commits:** none since `last_wrap` (2026-08-10T15:52:00Z) — this chunk's commit is P7's; prior HEAD is
`773cb06 feat(2026-08-09-sut-load-envelope)`

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-verify/src/preflight.rs` (M) · `crates/conductor-verify/tests/preflight.rs` (M) ·
  `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` (new) ·
  `conductor-0.2.0/verification-matrix.json` (ledger: `v2-17` → `chunk`/`acceptance`/`notes`/`ref`/`implemented`) ·
  `.gitignore` (M — `/ui/`, see Deviations)
- **Symbols / APIs:** all **private to `conductor-verify`; no public surface added or changed**.
  New: `NotFound` enum (`EmptyCorpus` · `FingerprintAbsent`) + `NotFound::precondition()`;
  consts `WORKSPACE_KEY_PRECONDITION`, `CANARY_FINGERPRINT_PRECONDITION`.
  Changed (private): `CanaryFidelity::NotYet(String)` → `NotYet(NotFound)`; `poll_canary` return
  `(CanaryOutcome, Option<String>, Option<String>)` → `(…, Option<NotFound>)`.
  **Unchanged:** `ReadyState` (no field added/renamed), `run_preflight` / `preflight_boot` signatures,
  `CanaryOutcome`, `ReportState` (still five), the six-lamp set, `runs.db` schema.
- **Crates / modules:** none added, removed, or re-scoped. No new cross-seam dependency edge.
- **Dependencies:** **none** — zero delta; `Cargo.lock` and `package-lock.json` both un-drifted.
- **Schema / config:** none. No new `CONDUCTOR_*` env var, no port, no MCP tool, no `runs.db` column.
- **Spec-master edits:** none by /implement. **Expected (this wrap, owned path):** `architecture.md`
  §Standard Contracts — the readiness gate's named-precondition set gains a fourth member.
- **Counts / qualifiers moved:**
  - The readiness gate's named-precondition set **3 → 4** (version-mismatch · tools-absent · empty-canary
    → + app/sidecar workspace-key agreement). Stated as a set in `architecture.md` §Standard Contracts.
  - Workspace nextest **494 → 495** (+1 test: `a_blocked_precondition_carries_no_absolute_host_path`).
  - `conductor-verify` package tests **67 → 68**.
- **Dev-tool versions:** none for Conductor. NOTE (SUT-side, not a Conductor tool and not a lockfile
  dependency): the Pulse sidecar `andromeda-pulse-mcp` was **built** on this host as a probe precondition
  (`cargo build -p mcp-server --bin andromeda-pulse-mcp --features mcp-server`); no Pulse source modified.
- **Reverted / negative API facts:** the plan's step-5 `StubConfig` knob (a fingerprint-in-slice toggle)
  was **planned but deliberately not shipped** — `tests/preflight.rs::a_fingerprint_mismatch_is_blocked`
  already drives that cause with a mismatched `CanaryMarker`, so the knob would have duplicated shipped
  capability. `tests/common/mod.rs` is therefore unmodified.
- **Coverage of new surfaces:**
  - `blocked_precondition` = the workspace-key value (operator-facing string; cli `[BLOCKED]` line +
    `preflight --json` + self-obs log) → validation **n/a** (an emitted value, not an external input) ·
    instrumentation **✓** (rides the existing `verify.readback.preflight` span and its single `info`
    boundary line; no new span name, no second log statement) · PII **redacted✓** (asserted by
    `a_blocked_precondition_carries_no_absolute_host_path` via `conductor_core::redact_value`; the live
    probe additionally showed `data_dir: "<redacted>"` in the real envelope) · tests **unit/integ ✓**
    (4 legs) · a11y **n/a** (cli output is not an a11y-assertable surface, a11y-plan §1) · tokens **n/a**
    (renders under the existing `[BLOCKED]` prefix — no new token, no new ANSI entry, no seventh lamp)

## Deviations from intent

1. **Plan step 5's `StubConfig` knob not added** — justification above (duplicate of a shipped mechanism).
   Both not-found causes remain drivable and are now asserted mutually distinct.
2. **Two "untouched" shipped test legs edited** — `an_empty_canary_is_blocked` (renamed
   `an_empty_canary_names_the_workspace_key_precondition`) and
   `a_canary_call_error_is_blocked_distinctly_from_an_empty_corpus` both asserted the exact string this
   chunk replaces. An assertion pinning a replaced string must move; the legs' stimulus and intent are
   preserved.
3. **The Pulse MCP sidecar was built** (unlisted precondition). Without it every probe arm would have
   returned the *read-back-unreachable* precondition and measured nothing about the workspace key.
4. **The live probe is inconclusive on the key axis** — recorded as such, not reported as a confirmed
   divergence. Pulse's telemetry shows `cues_emitted: 0` with the service still in baseline bootstrap, so
   **no incident formed under any key** in any arm; arm 3 therefore leaves the Windows `\\?\`
   canonicalization question open. Full record: `two-launch-verdict.md`.
5. **`/ui/` added to `.gitignore`** — fallout from this chunk's own arm-1 launch: Pulse emits Specta/TauRPC
   bindings relative to *its* cwd, so launching it from the Conductor root wrote `ui/src/bindings/` here.
   Foreign artifacts that an untracked `git add -A` would have swept into Conductor's history. Operator
   declined deletion and directed the ignore, which also makes every future live arm recurrence-proof.

## Decisions & corrections

- **Operator, at phase P5:** the Expected-amendments note must not target `intent.md` / `requirements.md` —
  both are immutable by contract (intent is never written by any skill; requirements' ids are contractual),
  so a detector aimed there has **no sanctioned apply path**. Premise corrections route via **C20**: the
  matrix `notes` field + `scope.md`. The `architecture.md` half stays a master-owned amendment.
- **Operator, at phase P4 (design):** detection names the condition on the empty-list branch rather than
  predicting it from Conductor's cwd or declaring an expected key via a new `CONDUCTOR_*` handle.
- **Operator, at phase P4 (probe):** three arms, adding `cwd == data dir` — the only arm that could come
  back positive.
- **Operator, at this wrap:** `/ui/` → `.gitignore` (not delete, not commit); the L5 age trigger is due for
  the audit deferral; the live divergence question routes as a CARRY, not a coverage failure.
- **Research correction (C20, recorded in the ledger):** `workspace_detector::detect` records the
  `.andromeda` marker and VCS as **fields**, not success conditions — so the intent's "marker-less temp dir
  falls back to `data_dir`" premise is false and no cwd-based interim unblock is known to exist.

## Outcome

**Acceptance criteria: met.** The simulated mismatch yields the distinct named precondition (not the generic
corpus string) inside an unchanged readiness shape, returned as `Ok(Blocked)`; the string carries no absolute
host path; the two not-found causes are asserted mutually distinguishable; the three-arm live probe ran and
each arm's `ready`/`blocked_precondition` is recorded.

**Gates green** (1 fix-loop iteration, no fixes needed): `cargo nextest run -p conductor-verify` **68/68** ·
`cargo nextest run --workspace --profile ci` **495/495**, zero retries · `cargo test --workspace --doc` ok ·
`cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo deny check` **green** (advisories,
bans, licenses, sources — exit 0).

**`cargo audit` red — sixth consecutive, deferred:** byte-identical `error loading advisory database: parse
error: duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 on **0.22.2** (the latest published, so no
floor exists to raise). Advisory-**database** fault against a zero-dependency-delta chunk with both lockfiles
un-drifted → the bounded wait holds, with `cargo deny check` **observed** green as the overlapping signal.
Not a floor raise, not a `deny.toml` ignore, not a CI edit (security-plan §Dependency Security).

**Smoke ✓** — `bash scripts/agent-run.sh run` exit 0 (boot-path touched: the preflight gate is the
`agent-run boot` entrypoint). Real-binary evidence beyond the harness: three live `conductor preflight --json`
runs, each returning the new precondition with `data_dir` scrubbed, exit 1 per the gate's go/no-go contract.
