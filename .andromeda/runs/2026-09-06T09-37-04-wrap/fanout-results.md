# Fan-out results — 2026-09-06-operator-gated-live-suite

7 doc-agents, one per spec source, run in one parallel batch against `report.md`.
**17 proposals · 12 applied · 3 withdrawn at the source (escalation resolved by a code fix) · 1 applied NARROWED · 1 rejected.**

The returns carried the documented transport HTML-escaping inside YAML values (`&gt;-` block scalars,
`&lt;5s`); proposals were parsed semantically after decode, and no proposal's meaning turned on an escaped
character.

| doc | verdict |
|---|---|
| architecture | **3 proposals** — all applied |
| security-plan | **5 proposals, all `escalate`** — 2 applied, 3 withdrawn at the source |
| design-system | `proposals: []` |
| layout-templates | **1 proposal** — applied |
| test-plan | **5 proposals** — 4 applied, 1 applied NARROWED (see below) |
| obs-plan | **3 proposals** (primary + 2 dependents) — all applied, scope narrowed |
| a11y-plan | `proposals: []` |

## Applied

- **arch / D-platform-claim** — `architecture.md:69`: the universal "under deterministic L4 every read-back
  returns `degraded_mode`" retired; `degraded` restated as a per-read-back property, measured both ways.
  This DISPOSES the report's single `Spec claims disproved by measurement` entry (Validate check 6).
- **arch / D-arch-resources** (+1 dependent) — `runs/live-suite/{leg}.jsonl` registered in §Occupied
  Resources; the §Infrastructure Patterns `runs/` tree gloss narrowed so "never overwritten" no longer
  covers the leg-stemmed, per-invocation child.
- **layouts / D-layout-surface** — the cli surface map's stage-flag set names `--live` with its exit rule
  and the refuse-vs-skip distinction.
- **tests / D-tests-obs-harness** (+2 dependents) — §3 stage selectors, §9 sanctioned live-leg SET (+ the
  `live-pulse` feature now gating a target SET), §11's CI-ban path enumeration.
- **tests / D-tests-coverage** — §2's determinism exception set gains the `--live` suite.
- **obs / D-obs-instrumentation** (+2 dependents) — `degraded_mode_response` retired at §3, §4 and §6.
- **security / D-security-subprocess** (+1 dependent) — the leading `conductor preconditions` short-circuit
  now stated over the path SET `{boot, run --live}` at §Anti-Patterns Universal and the `PATH`-miss sentence.

## Applied NARROWED (Validate check 4 — absence needs evidence)

The obs proposals' `change` lines generalised to "no scenario extra is implemented" / "the extension point is
unexercised". Only ONE field was measured absent. The applied text says the extension point stands and nothing
is KNOWN to exercise it, naming `degraded_mode_response` as the retired instance — the wider universal was not
applied. (amendment-flow §Apply: the applied text is re-derived from the invariant + the report's fact, never
pasted from the proposal.)

## Rejected

- **tests / D-tests-coverage dependent — the `sleep(N)` carve-out.** The proposal asked §11 to PERMIT a fixed
  wall-clock wait on the operator-local gate. Rejected: the 150 s quiet window lives in the harness shell
  BETWEEN legs, not inside a test, and §11's ban governs in-test synchronisation — so the ban was never
  violated and needed no exception. A ban is not weakened to accommodate something outside its scope. Applied
  instead: a scope clarification stating the ban's subject, naming the quiet window as a firing-form
  precondition reproducing a SUT-side window that emits no signal, and re-asserting that nothing inside a test
  may sleep to synchronise.

## Escalated → resolved by a code fix (3 proposals withdrawn at the source)

`D-security-input` (escalate ×3) found the harness's first recursive delete — `rm -rf "$RUNS_DIR/live-suite"`
on a path derived from operator-supplied `CONDUCTOR_RUNS_DIR`, with no canonicalize, no `--`, no non-empty
assertion, and executing before any Rust-side `resolve_under` rejection could run. It proposed recording the
residual in §Input Validation and rescoping two bans (§Anti-Patterns → Input's canonicalize duty; §Code
Patterns clause (b)'s "In every form" universal).

Escalated under the never-routine security-boundary rule. **The operator chose to fix the code.** Both shells
now clear the capture dir with the file's own non-recursive `cleanup` idiom (`rm -f …/*.jsonl` ·
`Remove-Item '…/*.jsonl'`); `grep -nE 'rm -r|Remove-Item -Recurse'` over both scripts → 0 hits. With no
recursive delete to govern, all three proposals lose their subject and no security ban was widened.

## Out-of-scope observation (no proposal; carried, not dropped)

The a11y agent noted a PRE-EXISTING internal staleness in its own doc, unrelated to this chunk:
`a11y-plan.md:565` still says "reduced-motion emulation is the one platform-dependent assertion", which
`:218`, `:424` and `:597` already retired in an earlier chunk. Not this chunk's drift (the report changed no
a11y fact), so it generated no proposal. Recorded here and in the handoff so it is not lost.
