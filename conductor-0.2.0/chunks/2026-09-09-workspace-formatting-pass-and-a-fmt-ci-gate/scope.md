# Scope — Workspace formatting pass and a fmt CI gate

**Marker:** `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`
**Version:** conductor-0.2.0 · Epoch 6b — Polish & ship
**Working entry:** `conductor-0.2.0/working-route.md:129` (minted at the
`2026-09-08-webview2-runtime-152-installed-in-job` wrap on the operator's directive; placement
operator-chosen at that wrap's P5, ahead of *Release build and bundle*)

**Surface intent (verbatim from the entry):** the tree made `cargo fmt --check`-clean and the gate
added to CI so it cannot re-rot.

---

## What this chunk builds

Two halves, both required — neither alone satisfies the entry:

1. **The formatting pass.** Apply the workspace's own `rustfmt.toml` across the tree so
   `cargo fmt --check` exits 0 at HEAD.
2. **The CI gate.** A fmt step in `.github/workflows/ci.yml` that runs the same check and turns the
   job red on violation, so the tree cannot silently re-rot. Today no workflow runs rustfmt at all,
   which is the mechanism by which the rot went unobserved for four days.

## Measured state at take-up (2026-09-09, re-verified against the artifacts)

The entry's CONTEXT freight was re-verified coordinate-by-coordinate at promotion. It holds, with
one correction and one strengthening:

| Claim as the entry states it | Measured at HEAD | Verdict |
|---|---|---|
| `cargo fmt --check` exits 1 | exit 1 (bare command; also `--all`) | holds |
| 282 flagged sites | 282 `Diff in` sites | holds |
| across **~40 files** | **60 files** — 30 under `src/`, 30 under `tests/` | **CORRECTED (+50 %)** |
| spanning nearly every crate | 8 of 9 — every crate but `conductor-report` | holds |
| `git status --porcelain` carries no `.rs` | none (only `friction-log.ndjson`, `session-handoff.md`) | holds |
| `grep -n 'fmt' .github/workflows/ci.yml` → 0 | 0 — and `ci.yml` is the ONLY workflow, so no workflow anywhere runs rustfmt | holds (stronger) |
| `rustfmt.toml` is `edition = "2024"`, stable, no unstable options | the file is exactly that one line | holds |
| toolchain pinned 1.95.0 with rustfmt as a component | `channel = "1.95.0"`; components include `rustfmt` (rustfmt 1.9.0-stable) | holds |
| test-plan §9 Lint row `:455`, failure-condition mentions `:477` / `:507` | all three exact; `:455` already names `cargo fmt --check` beside clippy | holds |
| `rustfmt.toml` shipped 2026-09-05 | added as a 1-line file in `59d5b7c`, commit date 2026-09-05 | holds |

Site distribution (sites / files per crate):

```
conductor-verify   110 / 18      conductor-faults    12 / 5
conductor-run       64 / 8       conductor-tauri     10 / 2
conductor-emit      51 / 16      conductor-cli        7 / 1
conductor-timeline  24 / 8       conductor-core       4 / 2
conductor-report     0 / 0  (the one clean crate)
```

The full 60-file list is reproducible with `cargo fmt --check` at HEAD; it is not pinned here
because the pass makes it empty.

## Folded freight from the working entry

The entry carries a `CONTEXT` block (no `PREREQ:` / `CARRY:` / `BLOCKED-ON:` annotations). Its
causal-mechanism claims fold below as their own bullets, each keeping the entry's own marker text
verbatim — statedness is not measurement at HEAD, so each is `[inferred]` until P3's scope premise
closure resolves it.

- **PARTLY VERIFIED (P3) — Root cause, operator-supplied, measured pipeline-side (backlog W90):**
  "`rustfmt.toml` (edition 2024) shipped 2026-09-05 with only `commands.rs` re-sorted, and CI never
  gained a fmt step." The *shipped-2026-09-05* and *no-CI-step* halves are verified and hold (the
  latter strengthened at P3: no `fmt` token appears in `ci.yml` OR in either `agent-run` script).
  The *only-`commands.rs`-re-sorted* half stays `[inferred]` and is deliberately **not** re-derived:
  it is a claim about the 2026-09-04 commit's history, both `commands.rs` and `spawn.rs` are clean at
  HEAD either way, and it shapes no step of this chunk.
- **VERIFIED (P3) — EVIDENCE, as the entry states it:** "the invocation is NOT the cause —
  `rustfmt.toml` is `edition = "2024"` (stable, no unstable options), the toolchain is pinned
  1.95.0 with rustfmt as a component, and the diffs are ordinary edition-2024 import ordering and
  line wrapping." The three configuration facts were re-verified at promotion. The diff-content
  claim is now verified too, on a firmer basis than the entry stated it: parsing the diff (ANSI
  stripped) gives +1581/−486 lines whose change classes are exactly import re-sorting (30 `use`
  statements, 45 use-list items) and expression re-wrapping (`bare-delimiter` 228 added vs 5
  removed; `method-chain` 279 vs 40). See `research.md` §1.
- **VERIFIED, IN A STRONGER FORM (P3) — the entry's `hypothesis:` marker, kept:** "a formatting-only
  diff can be partitioned from semantic change by byte-identity against `rustfmt --config-path
  rustfmt.toml`, keeping review cheap at ~40 files — the technique is established (`2026-09-06`
  crate-root recursion) but unmeasured at this scale." Measured: the partition holds, and the
  stronger available invariant is **per-file token-multiset identity** — for each of the 60 files the
  multiset of identifiers, string literals and numerals on the added side equals the removed side
  (60/60 identical, 0 exceptions; 262 distinct string literals per side, 0 exclusive to either).
  Because that is ONE whole-diff invariant rather than a 60-file reading task, **review cost does not
  scale with the file count**, which retires the entry's review-cost worry independently of the
  count correction (60, not ~40).

## Boundaries

- **Formatting-only.** No semantic change ships in the pass. The partition argument is the
  byte-identity technique above; anything rustfmt would not itself produce is out of scope.
- **`test-plan.md:455` is left standing, not retired.** It is a TARGET-state descriptor, and this
  chunk makes it true. This follows the `2026-09-06-coverage-completeness-gate` narrow ruling and
  the explicit guard carried through the prior chunk's wrap — no detector proposal to retire it is
  to be accepted here.
- **Rust only.** `rustfmt` governs `.rs`; no other formatter (prettier, black, the `ui/` TypeScript
  tree) enters scope. The npm side has its own gate and is untouched.
- **No behavioural CI change beyond the added gate.** The `a11y` job's open questions (the
  WebView2 endpoint, the driver-layer ECONNREFUSED) are a different thread and stay out.

## Surfaces and contracts touched

- **60 `.rs` files** across 8 crates (list above) — formatting only.
- **`.github/workflows/ci.yml`** — one added gate step in job `rust`.
- **`.andromeda/test-plan.md:455`** — becomes TRUE by this chunk; the row itself is not edited.
- **`verify_fmt_token_invariant.py`** in this chunk's own directory — added at validation-1 as an
  INTENT-INCOMPLETE amendment: planning surfaced that the formatting-only claim needs a *re-runnable*
  verifier, not just the one-shot P3 measurement, so the wrap light gate can re-assert it. Chunk
  evidence, not a shipped artifact — nothing under `crates/` or `scripts/` references it.

## Risk surfaced at planning (also intent-incomplete)

- **The `--fail-under-lines 60` coverage floor is genuinely at risk and is not a formality here.** The
  pass is net **+1095** lines (+1581/−486) with 30 of the 60 files under `src/`, which the floor counts
  (`tests/` is excluded by `--ignore-filename-regex`). Expression re-wrapping turns one long line into
  several, and whether the added lines count as covered cannot be known without running the collection.
  The working entry did not anticipate this and no extract raised it. If the floor drops below 60 that
  is a finding to record with its measurement, never a threshold to relax (test-plan §11 Quality).

## Premises closed at P3

- **RESOLVED — which CI job hosts the gate:** job **`rust`** ("Rust gate (build · test · lint ·
  supply-chain · coverage)", `windows-latest`, `ci.yml:16-187`). Its lint half today runs through the
  harness, not a bare cargo call: the step `Test + lint (dogfood agent-run)` invokes
  `.\scripts\agent-run.ps1 run`, and there is no standalone `cargo clippy` step. Neither `ci.yml` nor
  either `agent-run` script contains any `fmt` token, so the gate has no existing home in the
  workflow or the harness. Named by JOB, never by line, per a11y-plan §3 and test-plan §9.
  *Step ORDER within the job remains open — carried to `research.md` §Open questions as a
  plan-decision for P4.*
- **RESOLVED — the invocation form is `cargo fmt --all --check`.** `Cargo.toml` declares no
  `default-members` key, which is why the bare and `--all` forms both measure 282 sites today; they
  diverge the moment such a key is added, which would silently narrow the bare form. arch's
  member-completeness criterion is satisfied only by the explicit form. (test-plan §9's Lint row
  names the bare form; `--all --check` is a strict superset and makes the row true — surfaced at
  the P5 review as a deliberate, argued choice rather than decided silently.)
- **OPEN, by construction — whether the pass leaves `nextest`, `clippy` and the coverage floor
  green.** Unknowable before the pass runs; token-multiset identity makes regression very unlikely
  but is not a substitute for running them. All three ride the plan's Test Commands.
- **RESOLVED — no `#[rustfmt::skip]` is needed.** The tree carries **zero** today; no flagged file
  holds a snapshot assertion, a hand-aligned token table, an ANSI/status literal or a garde
  annotation; and per-file token-multiset identity holds in all 60.
- **RESOLVED — the crate-root recursion effect does not apply.** The 2026-09-06 learning describes a
  chunk whose real delta was small being widened by a crate root pulling in its module tree. Here the
  whole workspace is formatted deliberately, so the diff IS the complete intended set — nothing is
  pulled in unexpectedly.

## Premises added at P3 (not anticipated by the working entry)

- **The gate is line-ending invariant — tested, not asserted.** `.gitattributes` carries exactly one
  rule (`coverage-matrix.md text eol=lf`), so `.rs` files ride `core.autocrlf` (this host: `true`);
  they measure `i/lf w/lf`. `rustfmt`'s `newline_style` defaults to `Auto`, and the same clean file
  passes `--check` at **exit 0 in both LF and CRLF form**. A CRLF checkout cannot redden the gate.
- **Every sensitive surface the seven extracts named is outside the pass.** `garde` (5 files in
  tree / 0 flagged), `info_span!` (5/0), `insta::` (4/0), `[PASS]`-class prefixes (8/0), `owo_colors`
  (1/0), `comfy_table` (1/0), `rusqlite` (3/0), `creation_flags` (1/0), and all of
  `conductor-core/src`, `conductor-cli/src`, `commands.rs`, `obs.rs`, `redact` and `conductor-report`
  (0). Every preservation constraint the design, layouts, obs, a11y and security extracts raised is
  therefore satisfied vacuously. Each zero was positive-controlled against the tree first.
- **Artifact preservation does not constrain step placement.** All three `rust`-job uploads
  (coverage `:109`, JUnit `:118`, obs self-observation `:182`) carry `if: always()`, so a fail-fast
  gate anywhere in the job cannot drop them — which dissolves the obs↔tests placement tension the
  fan-out surfaced.

## Out of scope

- Any semantic refactor riding the formatting diff.
- Retiring or rewording `test-plan.md:455`, `:477`, `:507`.
- The `v2-24` disposition and the a11y/driver threads (owned by the wrap that dispositions `v2-24`).
- Formatting anything outside the Rust workspace.
