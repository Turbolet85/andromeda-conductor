# Report — 2026-09-24-architecture-registries-compacted-under-the-read-cap

**Chunk:** Architecture registries compacted under the read cap — Established Decisions and Occupied Resources each
readable whole in one Read, no registered fact lost
**Date:** 2026-09-24
**Commits:** none since `last_wrap` 2026-09-23T21:02:05Z. HEAD is `f0d92bd`, and this chunk lands in the wrap commit.

## Changes (structured — detectors read this)
- **Files:**
  - New:
    - `scripts/arch-registry-check.py`.
    - Chunk-folder drafts: `compaction/established-decisions.md`, `compaction/occupied-resources.md`,
      `compaction/moved-history.md`, `compaction/disposition-ledger.toml`.
    - `evidence/compaction-verdict.md`.
  - Pipeline bookkeeping: master/working route (promotion), `friction-log.ndjson`, `session-handoff.md`, and
    the phase and implement run dirs.
  - At wrap P2, applied through the amendment flow: `.andromeda/architecture.md` and
    `.andromeda/architecture-amendments.md`.
- **Symbols / APIs:**
  - One new committed operator instrument, `scripts/arch-registry-check.py` (stdlib Python, run as
    `python -X utf8 scripts/arch-registry-check.py …`). Its subcommands are `measure (--rev REV | --file PATH)`,
    `status --drafts DIR`, `check --before-rev REV --drafts DIR`, `selftest --before-rev REV --drafts DIR`,
    `ledger-init --before-rev REV --out PATH` and `ledger-show --before-rev REV --drafts DIR [--judgment]`.
  - Exit codes: 0 pass / within target · 1 findings / OVER target · 2 harness fault with a path-free reason.
  - Constants: `READ_CAP_TOKENS = 25_000`, `BYTES_PER_TOKEN = 2.541` and `TARGET_FRACTION = 0.60`, giving a
    threshold of 38 115 B. `QUALIFIERS` is a closed 5-tuple.
  - Its only subprocess is `git show REV:PATH`, run as an argv list: REV is validated against
    `^[A-Za-z0-9_.~^/-]+$` and PATH is one of two hard-coded subjects.
  - DIR, `--file` and `--out` must be repo-relative: absolute paths, a drive colon or a `..` component are
    rejected.
  - No IPC method, endpoint, port, socket, env var or Rust/TS symbol. No CI step invokes it. No `agent-run` verb
    or flag was added, so the harness keeps its 5 commands.
- **Crates / modules:** none.
- **Dependencies:** none. The script is stdlib only and adds no `requirements.txt` delta. It runs on the host
  Python 3 already registered in arch §Stack's "Operator instruments (host runtime)" row.
- **Schema / config:** a new chunk-local TOML ledger shape: `[[sentence]]` rows of
  `{id, section, head, disposition, marker?, anchor?}`, with `disposition ∈ {kept, moved, in-sidecar,
  rewritten}`. The `id` is the first 12 hex digits of blake2b over `section + NUL + normalized sentence`.
  Nothing outside the instrument reads it.
- **Spec-master edits:** none by /implement (read-only). This wrap applies three planned arch amendments (below).
- **Counts / qualifiers moved:**
  - Section sizes (basis `measure --rev HEAD` / `check`):
    - `architecture.md` §Established Decisions goes from 49 134 B (77.3 % of the Read cap) to 37 907 B (59.7 %).
      /implement measured 37 658 B; the wrap P2 faithfulness review restored 249 B of basis to [CI/CD] before
      apply.
    - §Occupied Resources goes from 48 859 B (76.9 %) to 37 929 B (59.7 %).
  - The committed operator-instrument count goes from two to three. arch §Stack `:39` states it as "the
    code-graph pipeline … and the mutation-tally gate … Neither is invoked by any CI step, and neither adds a
    sixth `agent-run.{sh,ps1}` command". Sweep `grep -c 'mutation-gate.py\|arch-registry-check'` over the seven
    masters: architecture 1 · test-plan 4 · the other five 0.
- **Dev-tool versions:** none — host Python re-read at 3.14 (pythoncore-3.14-64); no install or upgrade.
- **Harness / gate surface:**
  - A new operator-local instrument, outside CI.
  - The chunk's gate block uses it:
    - `py_compile`, `measure --rev HEAD` (expect exit 1 + `OVER target`), `selftest`, `check`;
    - `ledger-show --judgment` (recorded), a `cat` of the drafts with 13 `contains` atoms, and `status`
      (recorded).
  - Wrap proposals (propose → operator approves → append to `drift-base.md`; never by /implement):
    `D-arch-collision` (the carried CARRY) and `D-arch-registry-size` (`measure --file .andromeda/architecture.md`,
    where `OVER target` is drift).
- **Cross-project / external claims:** none. No SUT source was read. The drafts restate Pulse coordinates
  byte-for-byte from HEAD's master text, with no new reading.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `.andromeda/residuals.md:15` cites `architecture.md:69` for "its unmet-floor outcome already recorded
    accepted". `:69` is a blank line at HEAD (research M6), and after the compaction every line number in
    §Established Decisions moves.
    - Not a spec master: the plan routes it to "the wrap's sweep".
    - The handoff records `residuals.md` as having no sanctioned writer yet.
  - Nothing else. The plan's own example that the "No inbound listener" bullet carries no backtick span is false,
    since the bullet carries `--features tauri/custom-protocol`. That is a plan note, not a spec claim, and the
    instrument applied its first-span rule as written.
- **Expected amendments (from plan):**
  1. `architecture.md §Established Decisions` — replace the section with `compaction/established-decisions.md`
     verbatim. **Carried** (Counts bullet). Site: `grep -n '^## Established Decisions' .andromeda/architecture.md`
     → 1 hit (`:41`).
  2. `architecture.md §Occupied Resources` — replace the section with `compaction/occupied-resources.md` verbatim.
     **Carried** (Counts bullet). Site: `grep -n '^## Occupied Resources'` → 1 hit (`:143`). Both edits have
     their history appended to `architecture-amendments.md` inside this marker's entry, from
     `compaction/moved-history.md` (174 passages in 28 groups).
  3. `architecture.md §Stack and Technologies` — the "Operator instruments (host runtime)" row gains
     `scripts/arch-registry-check.py` as the third committed instrument, with no CI step and no sixth harness
     command. **Carried** (Symbols + Counts bullets). Site: `grep -n 'Operator instruments' .andromeda/*.md` → 1
     master hit (`architecture.md:39`) plus 1 sidecar history line. test-plan's 4 `mutation-gate.py` hits are
     §4/§9/§10 mutation-audit prose about that one instrument and state no instrument count.
- **Coverage of new surfaces:**
  - `scripts/arch-registry-check.py` (operator CLI) → validation ✓ (REV regex, repo-relative path guard,
    exit 2 on a missing subject) · instrumentation n/a (an operator instrument printing its verdict to stdout; no
    self-obs stream) · PII/redaction ✓ (findings name arm/section/NAME, never a path; the hygiene arm checks the
    drafts) · tests: selftest (a mutation per arm over the real drafts) plus the `measure` can-fail control · a11y
    n/a · tokens n/a.

## Deviations from intent
- **Drafts and ledger were script-composed, then hand-reviewed**, not hand-typed.
  - Scratchpad composers copied kept HEAD lines byte-exact and applied edits that each had to match exactly once,
    or a paragraph was replaced whole. This is what keeps `kept` rows genuinely verbatim.
  - A heuristic helper proposed dispositions. Every non-kept row was reviewed and 63 proposals overridden:
    restated current truth re-classed to `rewritten`, poor anchors replaced, and two nonsense span-less sidecar
    matches refused.
  - Justification: 579 rows and ~98 KB of prose. The mechanical verification (arm f) is the same either way.
- **Sizes above forecast:**
  - Measured 37 658 / 37 929 B against a forecast of ≈31-35 / ≈27-33 KB, leaving 457 / 186 B of margin under
    38 115 B.
  - History removal alone left §Occupied Resources about 1.5 KB over. The rest came from condensing verbose but
    current reader-registration prose (the per-handle env rows `:192`-`:202`, the `:177`/`:178` contract rows,
    `:172`, `:179`, `:201`), each sentence becoming a `rewritten` judgment row.
  - The plan named the env rows' "reader-registration narratives" as a cut target.
- **Rewrites that dropped content the HEAD sidecar does not already hold** also have their BEFORE wording copied
  verbatim into `moved-history.md` (106 of 150), so nothing is lost to the sidecar either. This is why
  `moved-history.md` is 49.8 KB.
- **Some bare parenthetical registration dates survive** on otherwise-verbatim sentences (for example
  "(added 2026-09-07)"), keeping those rows `kept`. Dated narrative (chronologies, superseded readings, CI-run
  stories, elimination stories) left the body.
- **Rust gates were run although the plan deferred them.**
  - `gate.py delta --defer-check rust` voided the deferral on a basename false positive: an untracked phase
    extract `security.md` matched a doc comment in `crates/conductor-verify/tests/jsonrpc_line_bound.rs` naming
    `.claude/rules/security.md`.
  - The letter's one-hit rule was followed. `gate.py run --only 8,9` still refuses `defer`-keyed entries, so both
    were driven by hand, green.

## Decisions & corrections
- **P5 resumed across a session boundary.** The prior window had reached the review with the operator's note
  that arm (d) alone cannot prove no-fact-lost, and folded it into arm (f). This window re-ran the full
  mechanical set over the edited plan: all pass, and every cited figure re-derived identically. The operator
  approved ("Yes").
- **Wrap P2 faithfulness review (the plan's owed item):** all 151 judgment rows were read, and 4 were found
  wanting and corrected before apply:
  - a `CONDUCTOR_MSEDGEDRIVER` skip sentence anchored on the NVDA bullet's identical phrase was re-anchored on
    its own bullet (the heuristic anchor matched the wrong registration);
  - [CI/CD] regains the dev-host green tally and its "measured green on two configurations" basis;
  - [CI/CD] regains the three-leg basis of the integrity-label discriminator.

  §Established Decisions ends at 37 907 B. Operator approvals this wrap: D-arch-collision (escalate) and
  D-arch-registry-size (warning) appended to `drift-base.md`, and the operator-instrument registration rule
  minted in `playbook.md`.
- **P4 operator decisions (prior window, recorded in the plan):**
  - the threshold is 60 % of the cap;
  - the standing enforcement is a wrap-time drift-base detector, not a CI step (the body grows only at wrap).
- **Sweep hazards found this chunk:**
  - (a) `gate.py delta --defer-check` greps uncommitted file BASENAMES fixed-string, so a phase extract named
    `security.md` matches any Rust comment citing `.claude/rules/security.md`. The hit must be read before it is
    believed.
  - (b) The instrument's sentence splitter never breaks after a closing `**` (`.**` is not "`.` + whitespace"), so
    bold-ending sentences merge with the next one. This is deterministic and harmless, but a reader counting
    "sentences" by eye will differ.
  - (c) A longest-common-run heuristic for rewrite anchors latches onto shared paths and chunk names (for example
    the `2026-09-11-…-closed` evidence path), not the restatement. Every auto-anchor needs reading.
- **Skill letter vs tool:** implement SKILL.md says to void a `defer` by re-running `--only {n}`, but `gate.py`
  prints `not run — defer (key)` for it and runs nothing. The route that exists is running the `run` by hand.

## Outcome
Acceptance criteria, re-asserted against the diff:
- **Each drafted section is ≤ 38 115 B.** MET: `check` prints AFTER 37 658 / 37 929 B, `within target`.
- **Labels hold.** MET: arm (b) is clean, with 22 `[Label]`s and 7 sub-registry labels byte-identical, in order,
  each exactly once.
- **No registered name is lost.** MET: arm (c) is clean. Every leading handle, crate name and port is kept, and
  every `QUALIFIERS` phrase stays on its bullet.
- **Every HEAD sentence is accounted for.** MET mechanically: arm (f) is clean over 579 rows (kept 352 · moved 68 ·
  in-sidecar 9 · rewritten 150), and arm (d)'s floor holds.
  - **OWED, not met:** the faithfulness review of the 151 judgment rows (150 rewritten + 1 span-less in-sidecar),
    rendered by `ledger-show … --judgment`. It is wrap P2's.
- **No host path in a drafted file.** MET: arm (e) is clean.
- **The instrument can fail.** MET: `selftest: every arm detected` (arm f three ways), and
  `measure --rev HEAD` → `registries: OVER target`, exit 1.
- **The 13 cross-master phrases are present.** MET: the `cat …` entry is green.
- **After the P2 apply: `status` reads `applied: yes` and `check` still PASSes.** MET at this wrap's P7 light gate
  (run dir `2026-09-24T08-36-40-wrap`):
  - `status`: `applied: yes` for both sections;
  - `check`: `arch-registry-check: PASS` (37 907 / 37 929 B);
  - the other entries: 5 green, 2 recorded;
  - by hand: nextest `1067 tests run: 1067 passed`, clippy exit 0 — the `defer` entries were run because the
    implement-time void still stands.

Gates (/implement run `2026-09-24T07-39-40-implement`):
- `python -X utf8 -m py_compile scripts/arch-registry-check.py` — green (exit 0).
- `… measure --rev HEAD` — green (exit 1 · `last line registries: OVER target`).
- `… selftest --before-rev HEAD --drafts …` — green (exit 0 · `selftest: every arm detected`).
- `… check --before-rev HEAD --drafts …` — green (exit 0 · `arch-registry-check: PASS`).
- `… ledger-show … --judgment` — recorded (report-only; 151 rows shown).
- `cat …established-decisions.md …occupied-resources.md` — green (exit 0 · all 13 `contains` atoms).
- `… status --drafts …` — recorded (`applied: no` ×2, by design at /implement).
- `cargo nextest run --workspace --profile ci` — `defer voided — .andromeda/runs/2026-09-24T06-20-18-phase/security.md`
  (a basename false positive). Run by hand: exit 0, `1067 tests run: 1067 passed, 0 skipped`.
- `cargo clippy --workspace --all-targets -- -D warnings` — defer voided (same). Run by hand: exit 0, 0
  warning/error lines.
- Smoke: skipped (no boot-path or UI-surface change).

Outcome basis: implement's P4 report, with this session's conversation present. No operator directive came between
implement and this report.

Process hygiene (implement P4 census, re-measured here): the two background `gate.py` runs and the hand-driven
`cargo nextest` / `cargo clippy` were all started by that run and have ended. `tasklist` shows no `cargo`,
`cargo-nextest`, `clippy-driver` or `rustc` process. This wrap started one background code-graph refresh (P4
records its end).
