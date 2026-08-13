# Scope — 2026-08-13-first-live-green-preflight

**Working-route entry (Epoch 2 — Live-path enablement):**
> First live green preflight — ready:true against a real Pulse with journal and runs.db evidence

**Version capability:** `verification-matrix.json#v2-10` — *Live `ready:true` preflight* (`method:
dynamic-external`, `status: planned`, `chunk: null`). Whether this chunk CLAIMS it is a P5 decision, not a
P1 assumption — see **Expected outcome** below.

---

## What this builds

The first attempt to run Conductor's preflight readiness gate against a **real, running Pulse** and record
what it returns. Every one of the gate's five named preconditions, the canary fingerprint bridge, the run
contract's `[incident_formation]` warm-up mechanism, and the whole per-check read-back extraction have been
proven against the hand-rolled JSON-RPC stub and **never once against a live response**. This chunk is where
that changes.

Four things it produces, in dependency order:

1. **A live preflight leg, run to completion under the verified operator recipe** —
   `conductor preflight --json` against a live deterministic-L4 `pulse-app`, with the sidecar built and
   reachable on PATH, both env terms exported in the launching shell, and the poll budget at or above the
   contract's `min_canary_poll_seconds` floor.
2. **The three-arm workspace-key probe, re-run and recorded** — cwd = the Conductor repo root · a
   marker-less temp dir · the data dir itself. Each arm's `ready` and `blocked_precondition` appended to
   `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md`, the file that
   already holds the 2026-08-10 run. The probe's own verdict section names this re-run as its condition of
   resolution.
3. **The first empirical observation of `[incident_formation]`** — the warm-up pre-roll (benign traffic
   carrying the canary service out of Pulse's baseline bootstrap before the counted storm) plus the raised
   poll floor. Today the mechanism rests on ONE falsification (`cues_emitted: 0` with a correct storm shape)
   and zero confirmations. Whether a cue fires, a digest runs, and an incident forms is the measurement.
4. **The read-back key-diff pre-check** — one raw `retrieve_report` / `retrieve_telemetry_slice` result
   dumped and diffed against `extract.rs`'s expected keys BEFORE any measured verdict is trusted. This runs
   first among the read-back work, because its failure mode is silent (below).

## Expected outcome — planned for, not assumed

The operator supplied verified coordinates at take-up (recorded in full under **Absorbed annotations**),
and they change what this chunk should plan to deliver. The canary's dependency chain is longer than the
route entry reads: `retrieve_telemetry_slice` reads `incident.evidence_refs` (incident-derived), incidents
are workspace-keyed via `load_active_incidents(&ctx.workspace_root)`, and `pulse-app`'s key comes from
`resolve_workspace_for_incidents` — the *detected* root whenever `detect()` succeeds, which it does for
virtually any real cwd, falling back to `data_dir` only on detect failure. The sidecar keys on the raw
`ANDROMEDA_PULSE_DATA_DIR` string.

**Consequence:** no launch cwd is known to make the two keys agree, and arm 3 (cwd = the data dir itself) —
the only position where they could — runs straight into the open Windows `\\?\` canonicalization question.

So the **probable** outcome is: all three arms measured and blocked, **F10 confirmed live** (the divergence
is real and gates the live legs, no longer masked by the no-incident cause), and the fix landing as a
**Pulse-side chunk** rather than in this repo. That is a genuine, recordable result — a measurement that
discriminates between the two causes the precondition names, which the 2026-08-10 probe could not do. It is
NOT `ready:true`. The chunk must be planned so its value does not depend on which way the measurement goes.

## Boundaries

- **Not a Pulse-side fix.** If the key divergence is confirmed, the repair belongs to Pulse (the sidecar's
  keying, or `resolve_workspace_for_incidents`), not to Conductor. Scope law: Conductor does no Pulse
  process management and owns no Pulse source.
- **Not a CI gate.** A live leg needs a running Pulse; CI has none. Nothing here may become a required CI
  step, and nothing may make an existing gate depend on a live Pulse.
- **Not the family live proofs** (`v2-11`..`v2-16`, Epoch 3). This proves — or measures the blocker in
  front of — the *preflight*, not any P-ID family.
- **No new gate preconditions.** The five named preconditions are shipped and set; a live run that surfaces
  one of them is the gate working, not a gap to fill. Adding a sixth is a different chunk.
- **No silent extraction change.** If the key-diff finds a mismatch between a live response and
  `extract.rs`'s expectations, that is a *finding to record and (if in-lane) repair with a test*, never an
  undocumented adjustment of the readers to match whatever came back.
- **Verdict/error wall holds.** Live transport failure is a harness fault; a blocked preflight is
  `Ok(Blocked)` with its named precondition. Neither becomes a panic.
- **Artifact hygiene holds.** Every recorded arm names its cwd **by role**, never by absolute path (the
  2026-08-10 verdict file's own convention); `data_dir` stays redacted in the readiness envelope.
- **`intent.md` / `requirements.md` stay immutable.** A falsified premise is recorded in the matrix `notes`
  + this scope, never by editing either file.

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `crates/conductor-verify/src/preflight.rs` (+ `client.rs`, `spawn.rs`) | exercised live for the first time; read-only unless the key-diff proves a defect |
| `crates/conductor-verify/src/extract.rs` | the key-diff target — its expected keys (`markdown`, `degraded_mode`, `span_refs`, `fingerprint_refs`, `items[].{id\|incident_id,status,severity,title}`) checked against a real response |
| `contracts/pulse-run-contract.toml` | `[incident_formation]` (warm-up pre-roll + `min_canary_poll_seconds`) gets its first empirical test |
| `chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` | **appended** with the re-run's three arms (the prior chunk's evidence file is this chunk's recording surface) |
| `runs/` — JSONL journal + `<run_id>.md` + `runs.db` | the evidence pointer `v2-10`'s acceptance names |
| `verification-matrix.json` | `v2-10` — claim or a `notes` line recording the failed concretization |
| `scripts/agent-run.{sh,ps1}` `boot` | the documented live entrypoint wrapping `conductor preflight` |

## Absorbed annotations

**CARRY (from `2026-08-10-pulse-run-contract`, re-pinned from `2026-08-10-workspace-key-divergence-probe`)
— the three-arm probe owes its live re-run, and this is where a live preflight actually runs.** The missing
mechanism has SHIPPED but is untested live: `[incident_formation]` drives a warm-up pre-roll plus a poll
floor outlasting L3's 20–60s digest cadence. **The live leg is the empirical test of that mechanism: until
it runs, the warm-up rests on one falsification, never on a confirmation.** Re-run `conductor preflight
--json` from the three cwd roles and record which reason each arm blocks for, appending to
`two-launch-verdict.md`. Only if an incident actually forms can arm 3 answer the `\\?\` question.
**Full operator recipe, so the gated leg never rediscovers it:** (1) the sidecar BUILT *and* on PATH
(`cargo build -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` in the Pulse repo) — `spawn.rs`
resolves the fixed program name from PATH, and an unreachable sidecar makes every arm measure the
read-back-unreachable path, yielding evidence that *looks* like a result and is not (the pulse-run-contract
chunk hit exactly that); (2) `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` + `ANDROMEDA_PULSE_DATA_DIR` exported
in the SAME shell that launches both, since Conductor observes the L4 term only as a declaration in its own
environment; (3) allow at least `min_canary_poll_seconds` — `CONDUCTOR_PREFLIGHT_TIMEOUT` raises, never
lowers; (4) launch `pulse-app` from a cwd OUTSIDE this repo (it writes Specta/TauRPC bindings relative to
its own cwd).

**CARRY (from `2026-08-13-per-check-read-back-extraction`) — the per-check extraction is stub-proven only,
and its failure mode against a real response is SILENT.** `conductor_verify::observe` composes the graded
observation from `query_incident_list` + `retrieve_report` + `retrieve_telemetry_slice`, and its readers
were written against Pulse's SOURCE (`crates/mcp-server/src/tools.rs`), never against an observed live
response. Every reader deliberately degrades to empty on an unexpected shape rather than erroring (the
verdict/error wall), so a live field-name or nesting mismatch does NOT surface as a fault — it yields an
empty observation, which reads downstream as an ordinary `Blocked` or a failed `Contains`. This entry is
the first moment live responses reach those readers. **Dump one raw result and diff its keys against
`extract.rs`'s expectations BEFORE trusting any measured verdict** — a green run over an empty observation
looks identical to a green run over a real one.

**PREREQ (from `2026-08-13-per-check-read-back-extraction`) — `cargo audit`, THIRTEENTH consecutive.**
Deferred since `2026-08-08-sut-capability-manifest`, operator-**RATIFIED** at the 2026-08-10 wrap under the
L5 age trigger; it re-pins **silently** from there (no further ratification HALT). It is an
advisory-**DATABASE** fault — `duplicate advisory ID: RUSTSEC-2026-0244`, reproduced byte-identically on
cargo-audit **0.22.2** (the latest published), so there is nothing to raise a floor to. The standing basis
was re-verified literally at the last pin and held stronger than at any prior one: `Cargo.lock` moved
**zero lines**, `cargo deny check` green (advisories/bans/licenses/sources, true exit 0) as the overlapping
signal. Remedy is the **bounded wait alone**: re-run it, record the result, verify `cargo deny` ran green
against THIS chunk's own lockfile delta rather than echoing the last chunk's. Do **not** raise the floor,
do **not** add a `deny.toml` ignore, do **not** edit CI. Close the deferral the moment it parses.
(`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09/-08-10 · security-plan §Dependency
Security.)

**Operator directives at take-up (verified coordinates, 2026-08-13).** Four, supplied with the phase
invocation and treated as verified rather than as hypotheses to re-derive:
1. The canary dependency chain, verified in Pulse's frozen source (`digest_runtime.rs:109-127`,
   `andromeda-pulse-mcp.rs:74`, workspace-detector's own tests) — reproduced under **Expected outcome**.
   Plan for: all three arms measured and blocked, F10 confirmed live, fix = a Pulse-side chunk.
2. **Claim accordingly.** If P3 research reaches the same conclusion, `v2-10` is likely
   *not-provable-by-THIS-chunk* — the deterministic decline (cap stays pooled + one `notes` line recording
   the failed concretization) is the correct move, **not a weakened claim**. The chunk's provable value is
   the three-arm verdict evidence + the first live observation of the `[incident_formation]` mechanism.
3. The key-diff pre-check runs **before** trusting any measured verdict (as the CARRY states).
4. The route entry's operator recipe **stands verified** — items (1)–(4) of the first CARRY above.

## Amended at P5 (validation-1: intent-incomplete)

Planning surfaced three things this scope did not anticipate. Recorded here so the plan and the intent anchor
agree; the divergence is justified, not a defect.

1. **The documented live entrypoint kills the live leg, and the two shells disagree about it.**
   `scripts/agent-run.sh boot` wraps the CLI in `timeout ${CONDUCTOR_PREFLIGHT_TIMEOUT:-30}` while the shipped
   in-process budget is ≈135s (a 45s warm-up plus a ≥90s poll floor) — so the default kills at 30s and the
   recipe's own `=90` export still kills mid-poll. `scripts/agent-run.ps1 boot` has no timeout at all, so the
   same leg completes on PowerShell and dies on bash, breaking the identical-semantics rule. Repairing both
   (budget derived from `contracts/pulse-run-contract.toml`) is IN scope: without it the live leg cannot run
   through its own documented entrypoint.

2. **The key-diff needs a mechanism, and it resolved to observed-key-set logging.** No affordance existed to
   see a raw payload. The chosen shape is a pure top-level-key reader in `extract.rs` plus a message-borne log
   line at each raw-response arrival point — inside the redaction boundary, since key names are not corpus
   content, and adding no CLI surface. Its reach is bounded by the blocker being measured: `retrieve_report`
   is only reached on a `ready` preflight, so on the probable outcome the single witnessed shape is
   `query_incident_list`'s.

3. **The three arms vary `pulse-app`'s cwd, not Conductor's.** The route entry's "re-run `conductor preflight`
   from the three cwd roles" is misattributed — Conductor resolves all four `contracts/*.toml` handles relative
   to its own cwd, so running it from a temp dir or the data dir is a harness fault at contract load, not a
   measured arm. `two-launch-verdict.md:10` records what the 2026-08-10 probe actually varied.

## Open questions for P4

- `[inferred]` **What this chunk delivers into the repo if `ready:true` is unreachable.** Evidence files
  and a matrix `notes` line are records, not code. Whether the chunk also lands a durable artifact — a
  documented live-run procedure, a recorded Pulse-side defect hand-off, a test pinning the key-diff
  result — is P4's call, and the version's convention (every prior chunk shipped code + tests) makes an
  evidence-only chunk the exception that needs stating explicitly.
- `[inferred]` **Whether the key-diff can be pinned deterministically.** If a live response's keys match
  `extract.rs`, that agreement is worth freezing (a recorded fixture / golden) so the stub stops being the
  only witness. If they diverge, the repair is in-lane. Which of the two, and at what test tier, is not
  stated by the working entry.
- `[inferred]` **Whether a Pulse-side fix is attempted, prepared, or only recorded.** The operator names
  "fix = a Pulse-side chunk"; scope law puts Pulse source out of bounds for Conductor. What crosses the
  boundary as a *hand-off record* versus what is simply out of scope needs an explicit line.
- `[inferred]` **The warm-up's success criterion.** `[incident_formation]` is asserted to carry the canary
  service out of baseline bootstrap. The observable that says it worked (`services_ready: 1`? a non-zero
  `cues_emitted`? an incident row?) is Pulse-side telemetry, and which signal is authoritative — and how
  it is captured without reading `corpus.db` — is unstated.
- `[inferred]` **Arm 3 and the `\\?\` question.** It can only be answered if an incident forms. If none
  does, the question stays open for a second consecutive chunk; whether that is recorded as a standing
  residual (and where) is P4's to decide.
