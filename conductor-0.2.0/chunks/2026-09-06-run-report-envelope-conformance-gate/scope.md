# Scope — Run-report envelope conformance gate

**Working entry (verbatim):** _Run-report envelope conformance gate — every run journal row
schema-complete and host-path-free, build failing on violation_

**One-line goal:** the run journal (`runs/<run_id>.jsonl`) gets the conformance gate the self-obs
stream has had since Epoch 10 — every row asserted schema-complete against its declared shape and
free of absolute host paths, failing the build on violation.

_Amended at P3 (scope premise closure). Verified `[inferred]` tags dropped; one bullet carries
`[premise-corrected]`; one section is new (intent-incomplete — planning surfaced what the scope
missed). Evidence: `research.md`._

---

## What this builds

- **A conformance gate over `runs/<run_id>.jsonl`.** obs-plan §9 `:535` already reserves it in
  writing — the log conformance gate validates the self-obs base-line schema "NOT the Section-6
  run-report ENVELOPE …, which is the scenario-RESULT record in `runs/<run_id>.jsonl` and gets its
  own (not-yet-built) conformance gate." This chunk builds that gate.
- **Schema-completeness over BOTH row shapes the journal carries.** obs-plan §3 establishes that the
  report seam writes two line shapes onto one journal — the eleven-field Run-report envelope and the
  nine-key per-check `CheckRecord`. "Every row" covers both, and the gate must DISCRIMINATE them
  rather than assert one shape over all lines. VERIFIED at HEAD: neither shape can deserialize as
  the other (disjoint required keys) and neither carries `#[serde(default)]`, so **a typed parse is
  simultaneously the discriminator and the key-presence check**.
- **Host-path-freedom over the same rows,** bound to the single source in `conductor-core::redact`.
  `[premise-corrected: is_host_path_token is PRIVATE (redact.rs:120); only redact_value and
  sanitize_error are pub, so the predicate is redact_value(s) == s — same source, no API widening]`
- **Build-failing enforcement**, settled at P3: a Rust test in the default nextest suite (red on
  every dev machine and in CI, and able to call `redact_value` directly), plus a CI step over the
  run journal the existing obs producer already writes — a REAL produced artifact a fixture cannot
  stand in for.

## The production defect this work exposes (new at P3 — intent-incomplete, not a defect in intent)

`conductor_core::read_run_journal` (`run_journal.rs:39`) parses **every** journal line as
`RunRecord`, so any run that emitted a per-check record fails the whole read. Measured with the
prebuilt binary: `conductor report 2026-08-21T18-53-35-135` → **exit 1**, `missing field \`seed\``;
the envelope-only control renders at exit 0. Two production callers are affected — the `conductor
report` verb (`conductor-cli/src/commands/report.rs:17`) and the Tauri `run_report` command
(`conductor-tauri/src/commands.rs:206`). Latent since `2026-08-21-per-check-latency-measurement`
added the second shape; test-plan §3's amendment (d) from that same chunk predicted it in writing,
but only the spec was corrected. It is in this chunk's scope because the gate's central requirement
IS shape discrimination, and a gate certifying rows two shipped commands cannot read would be the
contradiction the chunk exists to remove — with the reader and the gate sharing one rule.

## Carried in (folded from the working entry's CARRY — re-verified at HEAD `68c8014`)

**CARRY (from `2026-08-21-per-check-latency-measurement`): the 5-command harness's `cleanup` verb
deletes from `runs` alone and leaves ORPHANS.** Every coordinate re-read against the artifact:

- `scripts/agent-run.sh:233` — `DELETE FROM runs WHERE run_id = '$id';`, the file's only `DELETE`.
- `scripts/agent-run.ps1:254` — likewise the only one.
- A run also writes `run_check` (grain `(run_id, scenario, check_index)`) and `run_envelope`
  (run-grained, since `2026-08-09-sut-load-envelope`); cleanup touches neither.
- **The CONTRACT half is already correct** — `.claude/rules/verification-harness.md` §5-command
  `cleanup` and `.andromeda/test-plan.md:162` / `:164` each name all three tables. So this entry
  owns the **code fix only**: two deletes per shell plus the extended count-zero verification.
- The CARRY's "gated by `valid_run_id`" holds on both shells; the ps1's helper is named `Test-RunId`
  (`:58`, applied `:249`).

**The CARRY's prescribed FORM was falsified at P3 and re-decided by the operator at P4.** It says
"add the two **bound-parameter** deletes to both shells" while acknowledging in its next sentence
that the `sqlite3` CLI has no bind facility — unsatisfiable as written. Measured: **`sqlite3` is
absent from this entire host** (neither shell resolves it; `where.exe` finds nothing) and CI never
installs it, while both shells guard their DB arm on `command -v sqlite3` / `Get-Command sqlite3` —
so the delete has never executed anywhere observable. test-plan §3:162 independently prescribes the
cleanup body as `DELETE … WHERE run_id = ?1` with "(rusqlite bound parameters)". **Operator decision:
the teardown moves into the binary** — a `conductor cleanup <run_id>` verb over a new
`RunsDb::delete_run`, with both shells calling it instead of `sqlite3`. That is the only reading
under which the CARRY's own words and test-plan §3:162 are both satisfiable; it makes cleanup work
on this host for the first time and removes the undeclared host dependency. The `agent-run`
5-command count is untouched.

## Boundaries (what this chunk is NOT)

- **Not the self-obs stream.** `logs/agent-latest.jsonl` already has its gate (`ci.yml:112-160`);
  this chunk must not re-assert or widen it. The two record families are distinct by obs-plan §3.
- **Not a schema change.** The envelope's eleven fields and the `CheckRecord`'s nine keys are
  contract (arch §Standard Contracts). The gate ASSERTS the shipped shapes; a gate that would
  require a shape change is a finding to surface, not a licence to edit the contract.
- **Not the Markdown report.** `runs.db` enters only through the CARRY's cleanup deletes.
- **No live Pulse.** The hermetic producer already exists: a poisoned `ANDROMEDA_PULSE_DATA_DIR` is
  rejected before any sidecar spawn, yielding a Blocked envelope at exit 0 (`ci.yml:118-124`).
- **No new `CONDUCTOR_*` handle** — the three settled classes stay closed.

## Surfaces and contracts touched

| Surface | Why |
|---|---|
| `runs/<run_id>.jsonl` (the run journal) | the gate's subject — both line shapes |
| `conductor-core::redact::redact_value` | the single source of the host-path notion |
| `conductor-core::run_journal` | the shared discrimination rule + the reader defect |
| `conductor-report/src/journal.rs` | the writer whose output the gate judges |
| `.github/workflows/ci.yml` | the build-failing step over a produced journal |
| `scripts/agent-run.{sh,ps1}` `cleanup` | the CARRY's code fix — two deletes per shell |
| `runs.db` tables `runs` / `run_check` / `run_envelope` | the CARRY's orphan surface |

## Measured state at P3 (what the gate will find on its first run)

- **71 real journals on this host: 74 envelope rows + 1 check row, 0 other shapes, 0 unparseable.**
- **0 host-path hits** across all 71 plus the committed fixture.
- So the gate's first run is expected GREEN on real artifacts — a regression guard, not a bug hunt,
  matching the `2026-06-27-obs-ci-conformance-gate` precedent (PASS-on-real + FAIL-on-removed-field).
- Redaction is applied at ~6 PRODUCERS, never at the journal write site (`conductor-report` has zero
  `redact_value` calls) — so the property holds by convention with nothing asserting it, which is
  exactly what makes the gate worth building.
