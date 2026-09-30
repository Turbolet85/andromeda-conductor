# Report — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

**Chunk:** Mutation gate grades every tally it rests on — `scripts/mutation-gate.py` reads and grades `timeout.txt`
against a rostered timeout class, proven on fixture tallies with no cargo mutants run
**Date:** 2026-09-30
**Commits:** `a72533d` chore(2026-09-30-mutation-gate-grades-every-tally-it-rests-on): operator pre-CI commit, for the run
this chunk's verdict reads (the only commit since `last_wrap` 2026-09-30T07:42:16Z; parent `f33d6b7`, the chunk base)

## Changes (structured — detectors read this)
- **Files:** `scripts/mutation-gate.py` (modified) · `scripts/mutation-roster.toml` (modified) ·
  `scripts/fixtures/mutation-gate/` (new: `roster.toml`, `roster-bad.toml`, `arms.toml`, `arms/{15 arms}/` — 91 files,
  `git diff --name-only f33d6b7 -- scripts/fixtures | wc -l` → 91) · chunk `evidence/` (new: `selftest-control/` — 8 files,
  `selftest-verdict.md`, `operator-pass.md`). No Rust, TS, workflow, harness-shell, scenario or contract file changed
  (the plan's base-diff probe over `crates/ .github/ Cargo.toml Cargo.lock scripts/agent-run.{sh,ps1} contracts/
  scenarios/` against `f33d6b7` → no output, exit 0).
- **Symbols / APIs** (all in `scripts/mutation-gate.py`, an entry-point script — no committed file imports or invokes it):
  - CLI: new verb `mutation-gate.py selftest [--fixtures DIR]`; `DIR` defaults to `scripts/fixtures/mutation-gate`, must be
    repo-relative (an absolute path, a drive anchor or a `..` component → `selftest: --fixtures must be repo-relative
    (absolute or .. rejected)`, exit 2, value never echoed). Usage line is now
    `usage: mutation-gate.py <unit> | selftest [--fixtures DIR]` (exit 2 on any other argument shape).
  - `load_roster(unit, roster=ROSTER)` replaces `expected_for(unit)`: takes the roster PATH, returns a `Roster`
    NamedTuple (`expected` per rostered tally `missed`/`timeout` · `rows` · `declaration` · `total_rows`) or the one
    non-indented line `MUTATION GATE {unit}: FAIL — roster row lacks a valid tally: {file}: {mutation}`. It validates
    EVERY `[[entry]]` of the file, not only the graded unit's rows.
  - `grade(tally_dir, unit, roster=ROSTER) -> (verdict, lines)` — the grading seam, taking the tally directory itself;
    `main()`/`run_unit()` print its lines. Order: roster load → completion → empty population → presence (four tally
    files) → per-class count vs `outcomes.json` (present files only) → conservation (`missed+caught+timeout+unviable ==
    total_mutants`, from `outcomes.json`) → missed multiset → timeout multiset; every finding collected before the
    verdict; verdict line last.
  - New finding lines (two-space-indented): `TALLY MISSING: {class}.txt` · `COUNT MISMATCH {class}: {class}.txt has
    {n}, outcomes.json says {m}` · `NOT CONSERVED: missed+caught+timeout+unviable = {s}, total_mutants {t}` ·
    `TIMED OUT, not in roster (x{n}): {f}: {m}` · `ROSTERED, did not time out (x{n}): {f}: {m}`. The two `missed` lines
    (`SURVIVED, not in roster` / `ROSTERED, did not survive`) are byte-unchanged. `parse_tally(path, notes)` gains a
    `notes` list: its `  UNPARSED: {line}` notice now joins the grade's findings instead of printing directly (text
    unchanged).
  - Summary line changed: `missed {m} · caught {c} · timeout {t} · unviable {u} · total {T} · expected missed {em} ·
    expected timeout {et}` (was `missed {m} · caught {c} · expected {e}`).
  - `complete()` now prints `no outcomes.json under {out}` REPO-RELATIVE in POSIX form (was the absolute path — a host
    path on the not-readable FAIL line); its logic is otherwise unchanged. New helpers `rel()`, `run_unit()`,
    `selftest()`; the `cargo mutants` argv and the array-form `subprocess.run(cmd, cwd=ROOT)` are unchanged
    (`grep -cE 'shell=True|os\.system|os\.popen' scripts/mutation-gate.py` → 0).
  - `selftest` spawns no process and writes no file: it grades each `[[arm]]` of `{DIR}/arms.toml`
    (`name · unit · verdict · contains · findings · roster?`) over `{DIR}/arms/{name}/` and asserts verdict + a line
    containing `contains` + indented-line count + no absolute repo path in either slash form; then validates the real
    roster (`real roster: {n} rows, schema ok`); last line `selftest: every arm detected` (exit 0) or
    `selftest: ARM NOT detected — {names}` (exit 1).
- **Crates / modules:** none.
- **Dependencies:** none added — stdlib only (`typing.NamedTuple` joins the existing `tomllib`/`json`/`re`/`subprocess`/
  `collections` imports).
- **Schema / config:** `scripts/mutation-roster.toml` rows gain a REQUIRED `tally` key (`missed` | `timeout`, never
  defaulted); all 13 existing rows carry `tally = "missed"` (`grep -c '^tally = "missed"'` → 13; `grep -c '^\[\[entry\]\]'`
  → 13), 0 rows carry `timeout` (`grep -c '^tally = "timeout"'` → 0). Row keys are now `unit · file · mutation · tally ·
  class · member? · rule · citation_home`. The header's title line now reads "the expected `missed.txt` and `timeout.txt`
  multisets, per unit", the coordinate-free paragraph says "Tally lines" for "`missed.txt` lines", and a new paragraph
  states: `tally` required and never defaulted; a timeout is detection by hang, rostered so a caught→timeout move is graded,
  keyed coordinate-free as a multiset exactly as `missed`; `unviable` never rostered (host link contention moves it), graded
  by counts only; conductor-emit carries no timeout rows yet — "the epoch-boundary code audit seeds them from its measured
  `timeout.txt` (overseer, founder-delegated, 2026-09-30). Until then that unit fails closed on every timeout."
  Fixture `arms.toml` manifest schema as above (new, fixture-local).
- **Spec-master edits:** none (implement authors none).
- **Counts / qualifiers moved:**
  - The gate's graded tallies: 1 (`missed.txt`) → all 4 (two multisets + per-class counts + conservation). Stated at
    test-plan §4 `:228` ("compares `missed.txt` against … expected SET"), §12 `:619` ("never reads `timeout.txt`"),
    `.claude/rules/testing.md:19` last sentence ("The gate reads `missed.txt` and `caught.txt` only — it does NOT read
    `timeout.txt` …"), `.claude/docs/tests-summary.md:12` (mutation bullet), architecture §Stack `:39` names the gate only.
  - Roster tally classes: missed 13 · timeout 0 (new qualifier). Roster row count 13 unchanged.
  - Committed-fixture family (test-plan §7 `:403-404`): gains `scripts/fixtures/mutation-gate/`, the first member outside a
    crate-local `tests/fixtures/`, its meaning pinned by `selftest` (a Python verb, not a Rust round-trip).
- **Dev-tool versions:** none — host Python re-read at 3.14.3 at research (above `tomllib`'s 3.11 floor); no tool changed.
- **Harness / gate surface:** the operator instrument gains the `selftest` verb and a committed fixture tree. No CI step
  invokes the gate, no sixth `agent-run.{sh,ps1}` command (base-diff probe: `.github/` and both harness shells unchanged).
  No CI gate step added or removed.
- **Cross-project / external claims:** CI#36689204941 on `a72533d` (the pushed pre-CI commit): `verdict: green · checks 3/3
  · wall 642 s`, read by `ci.py conclusion --sha HEAD --wait 1200` (recorded at `evidence/operator-pass.md`). Its rust job's
  Secret-scan gate read the committed fixtures. This wrap's commit adds bookkeeping on top of that tree.
- **Reverted / negative API facts:** none.
- **Insufficient fixes:** none. (The class ships without its conductor-emit timeout rows by design — see Decisions.)
- **Spec claims disproved by measurement:** none. The stale sites listed under Counts are claims this chunk's CHANGE
  outdated, not measurement disproofs; the §9 per-chunk claim is reversed by a founder RULING (Decisions), not a measurement.
- **Expected amendments (from plan):** sites located by `grep -n 'mutation-gate\|mutation-roster' .andromeda/*.md` →
  test-plan 5 lines (`:228 :469 :506 :619 :621`), architecture 1 (`:39`), the other five masters 0; `grep -n 'tests/fixtures'
  .andromeda/test-plan.md` → 2 (`:403 :404`); `grep -nE 'per-chunk|per chunk|each chunk|every chunk'` over the seven
  masters + rules + docs + CLAUDE.md, filtered to `mutat` → 1 (`test-plan.md:469`), plus the semantic twin `test-plan.md:506`
  ("where a chunk runs the §4 mutation instrument" — no per-chunk token).
  - test-plan §4 Mutation instrument (`:228`) — carried: Symbols (grade order, new finding lines) + Counts bullet 1.
  - test-plan §9 Scoped mutation audit (`:469`) — carried: Decisions (founder ruling). Sites `:469` (states it) and `:506`
    (conditional "where a chunk runs"); 0 hits in the other six masters.
  - test-plan §12 `conductor-emit` member (`:619`) + its §4 restatement twin — carried: Counts bullet 1 + Decisions (owner).
    The "restatement twin" per the plan is §12's re-statement of §4; the located §12 mutation-gate lines are `:619` and
    `:621` (roster member IDENTITY — keyed `(file, mutation)`, which the timeout class now shares).
  - test-plan §7 committed-fixture family — carried: Counts bullet 3. BOTH §7 sites: `:403` (the tree SET) and `:404`
    (the per-fixture round-trip enumeration).
  - architecture §Stack "Operator instruments" row (`:39`) — carried: Harness / gate surface.
  - architecture §Infrastructure Patterns → Directory structure — carried: Files. The `scripts/` tree (`:226-230`) lists
    `agent-run.sh` and three `.ps1` files and carries NO row for `mutation-gate.py`, `mutation-roster.toml`,
    `arch-registry-check.py`, `code-graph.py` or the new `fixtures/mutation-gate/`. §Established Decisions / §Occupied
    Resources sit at 38 111 / 38 028 B of 38 115 (`arch-registry-check.py measure` this wrap) — the tree is outside both.
- **Coverage of new surfaces:**
  - `selftest --fixtures DIR` (operator CLI argument) → validation repo-relative guard✓ · instrumentation n/a (operator
    stdout, no self-obs) · PII n/a · tests selftest + control + `../outside` gate entry · a11y n/a · tokens n/a
  - roster `tally` key → validation fail-closed loader✓ · instrumentation n/a · PII n/a · tests `roster-tally-missing`
    arm + real-roster schema line · a11y n/a · tokens n/a
  - `grade()` failure lines (operator stdout) → validation n/a · instrumentation n/a · PII n/a (no host path: every arm
    asserts no absolute repo path; `no-outcomes` arm proves the `:72` line repo-relative) · tests 15 arms · a11y n/a ·
    tokens n/a

## Deviations from intent
- **An ABSENT tally file is not also compared against its roster multiset.** Plan step 3 f/g is silent on it; the step-7
  table pins `tally-missing` at `findings = 1`, which only holds if the multiset stage skips an absent file (as step 3d
  explicitly does for counts). Justification: the absence is already the finding; comparing an empty Counter to the roster
  would restate one fault once per rostered row.
- **The loader validates every roster row, not only the graded unit's.** Step 2 left the grain open. Justification: the
  roster is one document; a malformed row anywhere is a roster fault, and the selftest's real-roster line validates the
  whole file through the same loader.
- **Small additions inside the plan's intent:** a NOT-detected arm names every failed condition (step 6 says "which
  condition failed"); `selftest` exits 2 with `selftest: the fixtures dir holds no arms.toml` rather than a traceback
  (a traceback would print absolute paths); the unit path moved into `run_unit()` with `main()` as a dispatcher;
  `evidence/operator-pass.md` records the three operator entries (not listed in step 10; it is the `leg = 'operator'`
  entries' evidence home).
- Fixture specifics chosen: `mutants.json` placeholders are `{"fixture": i}`; `timeout-absent` removes the SHARED pair
  wholly (`x2`) so it differs from `timeout-multiset` (`x1`).
- scope record: none — `gate.py scope` clean (changed 93 · listed 93 · recorded 0 · absorbed 91), at /implement P4 and at
  this wrap's P1.

## Decisions & corrections
- **Founder ruling (2026-09-30, at take-up):** mutation testing runs at the epoch-boundary code audit ONLY, never per
  chunk; this chunk ran no `cargo mutants` in any form. This REVERSES test-plan §9 `:469` ("It runs per-chunk against the
  crates that chunk touched") — operator-directed carry at this wrap.
- **Owner recorded (overseer, founder-delegated, 2026-09-30):** the epoch-boundary code audit seeds conductor-emit's
  fifteen timeout rows from its measured `timeout.txt`; until then `mutation-gate.py conductor-emit` FAILs closed with one
  `TIMED OUT, not in roster` line per timeout, and that output is the measured input the rows are written from. Recorded in
  the roster header; this wrap records it in test-plan §12 where the audit reads it. No route change (operator: "No route
  change asked").
- Proof harness: an in-script `selftest` verb (the `arch-registry-check.py` precedent), no CI crossing.
- **Sweep hazard (deferral-void grep):** /implement's fixed-string grep of uncommitted BASENAMES over `crates/` hit
  `security.md` in a doc comment (`jsonrpc_line_bound.rs:3`, citing `.claude/rules/security.md`) while the uncommitted file
  was the phase run dir's `security.md` extract — a basename grep admits any same-named file; reading the hit settled it
  and the Rust deferral stood.
- **Hook guard:** the host's bash guard blocks any command carrying a doubled backslash (even inside a grep pattern);
  a hand-rolled host-path sweep was replaced by re-firing the plan's own sweep entry through the gate tool.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) selftest: exit 0, last line `selftest: every arm detected`, all 15 arms detected, `real roster: 13 rows, schema
  ok` — MET (`evidence/selftest-verdict.md`).
- (tests) harness can fail: control exits 1, last line `selftest: ARM NOT detected — pass-flipped` — MET.
- (tests) no `cargo mutants` ran: no gate entry names it; `test ! -e target/mutation-gate && test ! -e mutants.out` green
  after every gate-script entry — MET.
- (tests) roster: 13 `tally = "missed"`, 0 `tally = "timeout"`, header names the epoch-boundary code audit — MET.
- (security) array-form spawn (0 hits), `--fixtures ../outside` rejected at exit 2 without echo — MET.
- (security / obs) host-path sweep over roster + fixtures + evidence reads 0 (re-fired after `operator-pass.md` landed);
  `no-outcomes` proves the `:72` line repo-relative — MET.
- (security) fixtures tracked (`git check-ignore --no-index …` exit 1, no output); `secret_scan_gate` 5/5 passed — MET.
- (arch) nothing under the guarded paths changed against `f33d6b7` — MET.
- (obs) exit 0/1 + last `MUTATION GATE {unit}: PASS|FAIL` + one finding per indented line, asserted per arm — MET.
- (tests) CI green on the pushed pre-CI commit: CI#36689204941 on `a72533d`, verdict green 3/3 — MET.

Gates (/implement P2, two full runs, both `entries 19 · green 14 · red 0 · not-run 5`):
- `python -X utf8 scripts/mutation-gate.py selftest` — green (exit 0 · last line `selftest: every arm detected`)
- `… selftest --fixtures …/evidence/selftest-control` — green (exit 1 · last line `selftest: ARM NOT detected — pass-flipped`)
- `… selftest --fixtures ../outside` — green (exit 2 · contains `must be repo-relative` · lacks `outside`)
- `python -X utf8 scripts/mutation-gate.py` — green (exit 2 · contains the usage line)
- `… no-such-unit` — green (exit 1 · contains `unit is not declared`)
- `grep -c '^tally = "missed"'` — green (exit 0 · 13) · `grep -c '^tally = "timeout"'` — green (exit 1 · 0) ·
  `grep -c 'epoch-boundary code audit'` — green (exit 0)
- `test ! -e target/mutation-gate && test ! -e mutants.out` — green (exit 0)
- `grep -cE 'shell=True|os\.system|os\.popen'` — green (exit 1 · 0)
- `git check-ignore --no-index …` — green (exit 1 · no output)
- host-path sweep `cat … | grep -cE …` — green (exit 1 · 0); re-fired `--only` after `operator-pass.md`, green
- base-diff probe against `f33d6b7` — green (exit 0 · no output)
- `cargo nextest run -p conductor-core --test secret_scan_gate --profile ci` — green (5 passed)
- `cargo nextest run --workspace --profile ci` — deferred (`defer` key: zero Rust delta). Void check: the uncommitted
  basenames grepped fixed-string over `crates/**/*.rs` → 1 hit, `security.md` in a doc comment (not a file any test reads)
  → deferral stands.
- `cargo clippy --workspace --all-targets -- -D warnings` — deferred (same basis).
- `gate.py hygiene` (leg operator) — exit 0 · `hygiene: clean` (evidence/operator-pass.md)
- guarded `git push origin HEAD` (leg operator) — exit 0 · `PUSHED_SHA=a72533d64e9ea1e35d5a9360c88636143d8e7b62`
- `ci.py conclusion --sha HEAD --wait 1200` (leg operator) — exit 0 · `verdict: green` · CI#36689204941
- Smoke: skipped — no boot-path / UI-surface change.

Watches: none folded.

Outcome basis: /implement's P4 report (this conversation) plus the operator pass driven in the same session at the
operator's direction — pre-CI commit `a72533d` (the only commit, `git log --format='%h %s' f33d6b7..HEAD`), push, and CI
read, recorded in `evidence/operator-pass.md`. The final HEAD's tree differs from `a72533d` only by `operator-pass.md` and
the implement run dir's gate trail.

Process hygiene: none left running — implement census (`Get-CimInstance Win32_Process` filtered to cargo/nextest/rustc/
python/conductor/pulse/msedgedriver/tauri-driver) showed only the `ci.py` wait, which exited with its verdict; a re-census
after the CI read (`Get-Process cargo,cargo-nextest,rustc,python`) returned none.
