# obs extract

## Relevance
Partial — obs owns the a11y CI gate's telemetry artifact and the violation-record schema that this chunk's carve-out statement and dittography repair sit on top of; obs owns no keyboard/focus assertion itself.

## Constraints
- The only a11y telemetry artifact obs registers is the `a11y` job's violation record `runs/a11y/<run_id>.jsonl` (upload + in-job `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y`); a carve-out naming "what gates in their place" must name that gate as registered, not a re-described variant (per obs-plan §9 Telemetry artifact handling).
- That record's admitted shape is the eleven envelope keys plus the §9 resource tags, admitted because the gate asserts key PRESENCE / closed sets / host-path freedom and never key exclusivity — so it is a conformance gate over a record shape, never an assertion of any WCAG claim; an ownership statement must not describe it as owning a keyboard or focus claim (per obs-plan §3 Log format JSON schema).
- Any suite-local evidence path the carve-out names must use the runs-dir-relative form obs records (the Tauri sink's directory rides `CONDUCTOR_RUNS_DIR`; per-suite landings measured for the routine, driven and sr legs), never a bare project-root `logs/` literal (per obs-plan §3 Log file location).
- Whatever is named as standing in for a CI-unreachable suite must have a machine-parseable export; an operator-local arm qualifies only through an agent-readable record, never through operator review alone (per obs-plan §2 Agent-readable invariants; §11 CI).
- No artifact or statement authored here may carry an absolute host path, and §11's PII enumeration is explicitly non-exhaustive — it bounds the sites it names, never every channel (per obs-plan §9 Log conformance check; §11 PII Scrubbing).
- The bounded span-name set is closed and carries no `focus.*` member; obs mints no keyboard/focus span today (a11y-plan:111 *recommends* such spans for obs to add later), so no claim may be attributed to obs instrumentation as its owner (per obs-plan §11 Spans / Traces).
- Whether the routine `a11y` job's record + `journal_conformance` gate today already produce an obs-registered artifact that can back the hold-free half's attribution — and whether the driven arm emits any obs-registered artifact at all — is research's question; this plan states the target registration, not what the repo currently emits.

## Patterns to follow
- The §9 artifact-row shape (artifact · when · storage · agent access) is obs's canonical way to state that a gate exists; a carve-out sentence reads cleanly if it mirrors those four parts rather than inventing a new gate vocabulary (per obs-plan §9).
- The two-record-shapes discipline — self-obs base line vs Run-report envelope vs the per-check `CheckRecord` — means any sentence citing "the record" must say WHICH shape (per obs-plan §3 Log format JSON schema).
- Key-presence-not-exclusivity is the established mechanism that lets one conformance gate serve both the scenario record and the a11y violation superset; reuse that reasoning rather than proposing a second gate (per obs-plan §3).
- When a fact has no allowlisted field name, it rides the allowlisted `message` field — the standing precedent for any owner/count string, should one ever be emitted (per obs-plan §6 Boundary-call wrappers).

## Anti-patterns to avoid
- Never present a human-review-gated arm as the substitute gate without naming its machine-parseable export (per obs-plan §11 CI).
- Never widen the bounded span-name set, and never write a statement implying obs already emits `focus.*` / trap / restoration spans (per obs-plan §11 Spans / Traces).
- Never introduce an absolute host path or a bare project-root `logs/` literal into the statement or any artifact it names (per obs-plan §11 Logs; §3 Log file location).

## Contract bindings
- **obs §3 Log format JSON schema ↔ a11y-plan §3 "Structured violation JSON schema"** — a11y-plan:94-110 reproduces the obs envelope verbatim and is a two-sided bind (D-tests-obs-harness class). The CARRY 1 repair target (`a11y-plan.md:115`) sits five lines below that reproduction, so the byte-level read-modify-write must leave :94-110 untouched.
- **obs §9 artifact table ↔ the a11y CI job** — the very line being repaired (:115) restates obs-owned CI facts ("the `journal_conformance` gate run and the record uploaded"); a one-clause deletion must not perturb them.
- **obs §3 Log file location ↔ the driven / sr-leg operator-only arms** — the sinks the carve-out would name for the hold-dependent half.
- **obs ↔ tests §3** — tests owns the journal/envelope format; obs aligns to it, never the reverse (per obs-plan §3 heading note), so no format claim may be re-authored from the a11y side here.

## Acceptance criteria contributions
- If the carve-out names a CI gate, it names the `a11y` job's violation record `runs/a11y/<run_id>.jsonl` with its in-job `journal_conformance` under `CONDUCTOR_RUNS_DIR=runs/a11y`, as registered (per obs-plan §9 Telemetry artifact handling).
- No ownership statement authored this chunk attributes a keyboard or focus-order claim to an obs span or to obs instrumentation; the bounded span-name set carries no `focus.*` member (per obs-plan §11 Spans / Traces).
- Every suite path or artifact named in the new text is runs-dir-relative and free of absolute host paths (per obs-plan §3 Log file location; §9 Log conformance check).
- The dittography repair leaves the verbatim envelope reproduction (a11y-plan:94-110) and :115's obs-owned CI clauses byte-identical apart from the single duplicated clause removed (per obs-plan §3 Log format JSON schema — two-sided bind).

## Relevant amendment history
- **2026-09-07-a11y-ci-gate** — registered the a11y violation record's §9 artifact row and recorded the §3 envelope extension point as EXERCISED (13 keys locally / 15 under CI), with key-presence-not-exclusivity as the reason one gate serves both shapes. Why it matters here: this IS the gate a carve-out would cite, and the amendment exists because first operationalization exposed the spec's own stale description — the same failure mode a new ownership statement can re-introduce.
- **2026-09-02-screen-reader-manual-spec** — qualified every `conductor-tauri.jsonl` restatement as runs-dir-relative and recorded the three measured per-suite landings (routine, driven, sr-leg); also recorded a host-path channel measured OUTSIDE the three redaction sites, surfaced by the SR leg. Why: the driven and `sr*` arms are exactly the operator-only arms this chunk's carve-out covers, and the bare-literal defect it fixed is the one a new path reference would repeat.
- **2026-09-04-sidecar-spawn-without-a-console-window** — recorded that host-path channel CLOSED and the a11y leg's `<host-path>` scrub demoted to a second line of defence, superseding the 2026-09-02 reading. Why: anything this chunk says about the hold-dependent arm's evidence hygiene must reflect the superseding reading, not the 2026-09-02 one.
- **2026-06-16-emission-journal-writer** and **2026-09-06-run-report-envelope-conformance-gate** — settled the envelope at eleven fields and restated "required" as key PRESENCE. Why: these are what admit the a11y record's superset and what the a11y-plan reproduction adjacent to the repair site must keep matching.
- **2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate** — extended §9's gate enumeration and recorded that a CI-log line carrying absolute host paths is never routed into a telemetry artifact. Why: the precedent for how a newly-named CI gate is stated on the obs side, and the host-path rule a carve-out naming job output must respect.
