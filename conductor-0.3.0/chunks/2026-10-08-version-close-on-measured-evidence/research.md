# Codebase Research — 2026-10-08-version-close-on-measured-evidence

## Scope
- **Depth:** moderate · **Reads:** 19 · **Globs/Greps:** 24
- **Harness rules consulted:** none — no live leg in this chunk. `.claude/rules/verification-harness.md:78` was
  read as CARRY 1's re-point target, not as a recipe.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the pc overseer's relay for the version close (three sentences, the four
  CARRY dispositions, the stops, the next-version direction check); `inputs#I2` — the invocation directive (a
  boundary question goes to the operator; the review states what a diagnosis and an audit cost and answer);
  `inputs#I3` — the operator's answers to the two P4 forks; `inputs#I4` — the founder's ruling on the contract's
  readers, relayed by the pc overseer, and the operator's overrule on the ordinal; `inputs#I5` — the operator's
  word at the P5 review (step 6 is no longer conditional; the ordinal stays; re-plan)

## Files inspected
- `scripts/agent-run.ps1` (228-231) — line 230 is a whole comment line ending "(host-win32.md)"; the block around
  it is comment text about `Start-Process` and BOM-less capture files.
- `scripts/a11y-token-witness.ps1` (158-161) — line 160 is a whole comment line ending "(host-win32.md)".
- `.claude/rules/verification-harness.md` (78) — the 2026-09-16 surviving-grandchild entry: `Start-Process …
  -PassThru` without `-Wait`, then `WaitForExit()`, waits on the child alone. This is the first comment's lesson.
- `.claude/docs/session-learnings.md` (87-88) — heading "2026-10-07 — The drive-letter host-path anchor also
  matches a registry provider form …  (host leaf entry of 2026-09-11 …)". This is the second comment's lesson.
- `crates/conductor-run/tests/real_model_grading/mod.rs` (140-175) — `pre_registered(evidence, heading, sha256)`
  reads `contracts/pulse-real-model-leg-posture.md` through `committed`, slices one `## ` section by heading,
  compares its sha256 with the digest the series' attempt ledger recorded and with the pin in test source. A
  missing section or a mismatch fails the test.
- `crates/conductor-run/tests/evidence_pin/mod.rs` (1-50) — `committed(name)` joins the name to the workspace
  root taken from `CARGO_MANIFEST_DIR`, reads the file, and panics with the repo-relative name and `e.kind()` on
  a read fault. No environment handle, no operator value. `check_digest` returns the two digests on a mismatch,
  never the text.
- `contracts/pulse-real-model-leg-posture.md` (36-50, headings) — `## Regime` (`:42-49`) says the member is
  reader-less, "No Rust code reads it", and gives notice that the committed-manifest input-boundary duties attach
  in full if a future chunk makes it runtime-read. The six digest-pinned sections are the dated ones (`:334-887`);
  `## Regime` is pinned by nothing.
- `.andromeda/architecture.md` (183, 184) — the P-025 row and the posture row, each one multi-KB line.
- `.andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md` (30) —
  the `contracts/` tree line, "(the two members no Rust code reads)".
- `CLAUDE.md` (14) and `.claude/docs/conventions.md` (9) — the two leaves, same words.
- `.andromeda/security-plan.md` (113, 338) — the fixed-path-manifests row, whole; the model-text scope.
- `.andromeda/playbook.md` (225-246; the three `verdict: escalate` patterns at 97, 109, 124) — the reader-less
  member rule, and the boundary-widening escalate pattern.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (455-480) — names the P-025 contract in comments only.
- `crates/conductor-core/tests/secret_scan_gate.rs` (183-213) — lists with `git ls-files` and reads each listed
  file's bytes.
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` (7-26) — the population arm walks every
  chunk's `evidence/` for a file name that starts `rm-capture` and ends `.txt`.
- The ledger, per id (`matrix.py show --dir conductor-0.3.0 --id v3-01` through `v3-11`) — owner, method, status
  and `ref` of each capability; the notes on `v3-09` in full.
- `.andromeda/residuals.md` (whole, by entry) — four entries: three `absorbed`, one `re-carried:0.3.0`, none
  `open`.
- The wrap letter (`andromeda-wrap-session/SKILL.md` 515-546) — the wrap pushes the build branch; it prints the
  evolve nudge when a chunk completes its epoch. It has no merge, tag or release step.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **pre_registered** — 6 calling tests on the rust plane, `calls` keyed on `callee_name` with `callee_kind = 'fn'`:
  `the_2026_09_30_series_rule_was_fixed_before_d1` @ `series_2026_09_30.rs:23`, the same test of each later
  series @ `series_2026_10_01.rs:25`, `series_2026_10_06.rs:25`, `series_2026_10_07.rs:25`,
  `series_2026_10_07_sixth.rs:26`, and `the_2026_10_07_capture_run_record_was_fixed_before_d1` @
  `capture_run_2026_10_07.rs:26` (all under `crates/conductor-run/tests/real_model_grading/`). Six module-level
  `use` rows ride the same result. A name grep agrees: 6 call sites and the definition. No caller sits in a
  shipped crate. This chunk changes none of them; the result is the measured reader set the corrected sentence
  describes.
- **committed** (`evidence_pin/mod.rs`) — 16 call rows, all in `conductor-run`'s test tree. `pre_registered` is
  the one caller that passes a `contracts/` path; the rest pass evidence paths.

## Patterns detected
- **A test-binary reader stated beside "never a shipped binary"** (`.andromeda/architecture.md`, the
  `CONDUCTOR_E2E_SEED_DIR` entry and the `contracts/scenario-audit-ledger.toml` entry): the registry's standing
  form for a reader that exists only at test tier.
- **A reader claim narrowed to what stays true** (architecture sidecar, `2026-09-30-the-sr-cause-isolated-on-this-host`):
  "Read ONLY by" became "Its only COMMITTED reader is … (never a shipped binary)", with no dated clutter in the body.
- **Free first, then correct** (architecture sidecar, `2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09`,
  and the trap in `2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir`, where the check read OVER after
  the first apply).
- **A citation of a retired home re-pointed to where the truth lives** (the `2026-10-04T01-45-46-wrap` entries of
  four sidecars): the same act as CARRY 1.

## Conventions to follow
- **Read from the ledger, never re-derive**: each capability's basis in the close record is the ledger's `ref`
  with its owning chunk; the count is read at write time (`matrix.py coverage`).
- **No capture-named file and no model text in this chunk's `evidence/`**: a file named `rm-capture*.txt` would
  move the population pin (`capture_population.rs:26`); the rank-1 statement is quoted nowhere.
- **No absolute host path in committed evidence**: data dirs are named by leaf.
- **A sha256 digest in an evidence document passes the secret-scan gate**: 14 committed evidence `.md` files
  already carry one (`git grep -l -E '\b[0-9a-f]{64}\b' -- 'conductor-0.3.0/chunks/*/evidence/*.md'`).
- **Exit read from the bare command**; the printed verdict outranks it (`host-linux.md` §Exit codes).

## What research measured

### The byte arithmetic of CARRY 3 and CARRY 4
- Headroom in §Occupied Resources: 1 B (38114 of 38115 B, `scripts/arch-registry-check.py measure --file
  .andromeda/architecture.md`, 2026-10-08).
- The posture row's false sentence is 106 B: "It is the **SECOND** `contracts/` member with **NO Rust reader**,
  taking the P-025 regime above unchanged."
- `[superseded at the P5 review, 2026-10-08 (inputs#I4 §3; inputs#I5)]` The first arithmetic paid for the
  correction by dropping "FIRST" from the P-025 row (−34 B) and rewriting the posture sentence without its
  ordinal (+32 B). The operator overruled it: the playbook's rule note says a historical ordinal "is not a
  uniqueness claim and stays as written" (`.andromeda/playbook.md:243`, confirmed with the user on 2026-09-18).
  Both ordinals stay as written.
- With both ordinals kept, three drafts of the posture sentence, measured by script over the row at HEAD
  (`fb48cee`), each keeping "the **SECOND** `contracts/` member with **NO Rust reader**" and the regime clause:
  - "… **NO Rust reader** but a test-tier digest hold (its `## Regime`), taking …" — 152 B, +46 B, 45 B to free;
  - "… **NO Rust reader** at runtime (a test-tier digest hold: its `## Regime`), taking …" — 160 B, +54 B, 53 B to
    free;
  - "… **NO runtime Rust reader**, taking …" — 114 B, +8 B, 7 B to free. This one names no reader and points
    nowhere; it changes the bolded phrase itself.
  The kind and tier of the reader live in the contract's dated block and in security-plan; the row points at the
  first (`inputs#I4` §3).
- One freeing candidate that is neither an ordinal nor a phrase a rule pins: the posture row repeats the P-025
  row's face-field sentence one row below it. "Carries `sut_version` · `captured_at` · `pinned_at` · `provenance`
  on its own face." becomes "Carries the same four face fields.", −52 B. It is the by-reference form the
  registry already uses. The phrase stands in no playbook rule and no gate reads it
  (`git grep -c -E 'on its own face|sut_version' -- .andromeda/playbook.md scripts/arch-registry-check.py
  crates/conductor-core/tests crates/conductor-report/tests .claude/rules` prints no file).
- Predicted with the first draft: 38114 − 52 + 46 = 38108 B. Free first, then correct. The second draft with the
  same freeing lands at 38116 B, 1 B over, so it needs more. These are arithmetic, not final wording; the wording
  is the wrap's, measured by the same instrument and proven lossless by it.
- If the wrap finds the by-reference form is not lossless, or that no true sentence fits without breaking the
  ordinal rule or the threshold, it stops and shows the operator the arithmetic (`inputs#I4` §3).
- §Established Decisions (38082 B) is not grown by this chunk: nothing the close states needs a word there.
- The key file's line and the two leaves are outside both measured sections. "no Rust code reads" becomes
  "no shipped code reads" (+3 B per site), true of both members.

### The rider — does security-plan's fixed-path-manifests row owe the test-binary reader a row
What the code does: one test helper opens the contract by a hard-coded repo-relative name, rooted at
`CARGO_MANIFEST_DIR`; reads no handle and no operator value; cuts one section; compares a digest. No value from
the file reaches any decision except "equal to the pin or not". A read fault carries the name and the error
kind. A missing section or a mismatch is a hard failure.

What the record says:
- The row's own criterion: "a committed artifact parsed at runtime earns its row whether or not a shipped binary
  is the one parsing it" (`.andromeda/security-plan.md:113`). Its four members are TOML manifests parsed into
  typed values that steer a run.
- For a row: the 2026-09-06 escalation resolved "record the reader" for the first test-binary reader of a
  `contracts/` file ("an unrecorded reader is the drift"); the 2026-09-13 P-025 amendment left this row unmoved
  because the member was reader-less, which a measured reader re-opens.
- Against a row in THIS place: digest-held committed files already have a home, the real-model capture ingest
  row (2026-09-30), not the manifests row; and 2026-10-02 holds a reasoned no-row decision for a test that reads
  no value.
- The contract's own `## Regime` notice: the duties "attach in full" if the file becomes runtime-read.

The reading: the helper does not parse the contract, it holds it. So it is outside the manifests row's class by
that row's own word "parsed". But the per-reader principle says a reader is recorded, never assumed, and the
honest record is one of two writes: a clause that names this reader (in the manifests row or beside the digest
pins in the ingest row), or a dated note that it sits outside the row and why. Choosing between them — and
deciding whether a test-time digest read is the "runtime-read" the contract's notice names — fixes where the
clause's edge is. That is a reading of what a boundary clause covers. It is brought to the review and written
nowhere by this chunk.

The ruling (`inputs#I4` §1): the operator put the question to the founder by dialog on 2026-10-08. His pick,
verbatim: «Нет, исправить фразу (Recommended)». A test-tier digest read is not the runtime read the `## Regime`
notice names, and the duties do not attach. The notice stands for a real runtime read. Where it lands
(`inputs#I4` §2): one dated add-only block at the end of the contract's `## Regime` section; the close record,
in his relayed words; the masters, written by the wrap and quoting him. No code changes and the helper is not
touched. Checked at this re-plan: the six headings a digest pins are the dated ones
(`git grep -h -A3 -F 'pre_registered(' -- crates/conductor-run/tests/real_model_grading`), so a block inside
`## Regime` moves no pin; and no live file cites the contract by line number (every hit of
`git grep -n -E 'pulse-real-model-leg-posture\.md`?:[0-9]'` sits in a frozen chunk folder). The date the block
states was re-derived: the first test read of the contract landed in commit `b8e7bca`, 2026-09-30
(`git log --reverse -S'committed("contracts/pulse-real-model-leg-posture.md")' -- crates/conductor-run/tests`);
the helper took its present name on 2026-10-04 (`91f0f04`).

### What an evolve diagnosis and a code audit cost and answer
- Both run only when the operator invokes them (`disable-model-invocation: true` in each skill).
- **Evolve diagnosis** of Epoch 5b. Question it answers: which pipeline frictions recurred across this epoch's
  eight chunks, by type and cost, as proposals for the founder. Its input is 220 friction records carrying this
  epoch's label (`grep -o '"epoch": ?"Epoch 5b[^"]*"' .andromeda/friction-log.ndjson`, one spelling). It changes
  nothing. The two diagnoses of 2026-10-04 each took about 9 to 10 minutes from first to last file in their run
  dir and wrote 131 KB and 169 KB. Their token cost is not recorded anywhere this chunk reads.
- **Code audit** at the Epoch 5b boundary. Questions it answers, as test-plan names them: whether
  `conductor-core`'s mutation tier holds without `--copy-vcs true`; roster rows for `member-1` to `member-3`,
  which need their crates' tiers re-run; the timeout rows that seed `conductor-emit`'s gate, which fails closed
  until then. It also appends one metrics snapshot and diffs it against the last. The 2026-10-04 audit's run dir
  spans about 21 minutes of file times and holds nine per-crate mutation result files; that span does not prove
  the mutation runs' own wall-clock, which the run dir does not state in one place. Since that audit no file
  under any crate's `src/` has moved (`git log --since='2026-10-04T12:00:00+02:00' --name-only -- 'crates/*/src'`
  lists none); what moved is test code, the two harness scripts and the posture contract. A mutation tier
  re-run would therefore measure the same shipped code as the last one, against a changed test suite.

## New files to create
- `conductor-0.3.0/chunks/2026-10-08-version-close-on-measured-evidence/evidence/version-close.md` — the close record
- `conductor-0.3.0/chunks/2026-10-08-version-close-on-measured-evidence/evidence/next-version-direction.md` — the founder's words and, under their own heading, the proposals that are not his

## Files to modify
- `scripts/agent-run.ps1` — one comment line re-pointed
- `scripts/a11y-token-witness.ps1` — one comment line re-pointed
- `contracts/pulse-real-model-leg-posture.md` — one dated add-only block at the end of its Regime section, on the founder's ruling

## Open questions
- What is written in the contract's own `## Regime` section → blocks: plan-decision. Resolved: first by the
  operator at P4 (`inputs#I3`, the chunk does not touch the contract while the founder is asked), then by the
  founder's ruling (`inputs#I4` §1, `inputs#I5`): the contract gains one dated add-only block.
- Where the founder's direction is recorded, and in what form → blocks: plan-decision. Resolved at P4 by the
  operator (`inputs#I3`): a document in this chunk's evidence tree plus one `open` residual line; no
  `conductor-0.4.0` directory.
- Whether the P-025 row's ordinal leaves the body to pay for the correction, against the playbook note that says
  an ordinal stays → blocks: implementation-scope. Resolved at the P5 review by the operator (`inputs#I4` §3;
  `inputs#I5`): it stays. The bytes are freed elsewhere, or the wrap stops and shows the arithmetic.
