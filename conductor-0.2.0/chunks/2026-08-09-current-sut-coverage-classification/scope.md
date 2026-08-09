# Scope — Current-SUT coverage classification

**Marker:** `2026-08-09-current-sut-coverage-classification`
**Version:** conductor-0.2.0 · **Epoch 1 — Foundation: re-aim at the SUT**
**Working entry:** _Current-SUT coverage classification — capabilities in the manifest as auto, drive+observe, static-only or not-Conductor's, the boundary a recorded decision_
**Matrix capability:** `v2-03` (candidate; linked at P5 if this chunk makes its acceptance pass)

---

## What this chunk builds

Conductor's coverage classification currently enumerates exactly `P-001`..`P-060`
(`crates/conductor-core/src/coverage.rs`, `static COVERAGE: [CapabilityRow; 60]`). The accepted
capability set has since become **data** — `contracts/pulse-capabilities.toml`, `sut_version = "v0.3.0"`,
**82 ids** — and `conductor_core::check_sut_drift` already gates the difference against a pinned
residual ledger (`KNOWN_UNCLASSIFIED`, the 22 ids `P-061`..`P-082`).

This chunk closes that gap: **every capability the manifest accepts carries exactly one classification,
with zero unclassified entries**, and the "not Conductor's to verify" boundary is recorded as a
deliberate decision rather than implied by absence.

Three things follow from that:

1. **A fourth classification.** `CoverageMode` today is `Auto` / `DriveObserve` / `StaticOnly`. The
   requirement names a fourth — **not-Conductor's** — for capabilities that belong to Pulse's own
   verification surfaces. It is a distinct concept from `StaticOnly` ("no dynamic telemetry dimension,
   stays with Pulse's own test matrix"): static-only says *nothing to drive*, not-Conductor's says
   *outside Conductor's remit by standing non-goal*. Adding the variant touches every exhaustive `match`
   on `CoverageMode` (label, `ALL`, serde wire spelling, and each renderer).

2. **The 22 new rows, classified.** The accounting is already decided in `intent.md` §4 F2 + §5 boundary
   and this chunk implements that decision rather than re-litigating it:
   - **Conductor's lane** — `P-067` (live service truth), `P-072` (Investigate result), `P-073`
     (deterministic L4 mode), `P-074` (exactly-one-incident storm coalescing), `P-079` (single-sourced
     workspace key). Intent gives per-id leanings (P-072 "candidate drive+observe via report read-back";
     P-067 "partially observable") — the plan fixes each mode.
   - **Pulse's own — UI/visual** — `P-061`..`P-066`, `P-068`..`P-071`, `P-080`..`P-082` (13), verified by
     Pulse's webview suite; Conductor's standing non-goal is "no UI automation of Pulse".
   - **Pulse's own — tooling** — `P-077` (demo injector), `P-078` (agent-headful self-verify harness).
   - **The delegation itself** — `P-075` / `P-076`. Intent §5 sets these apart from both groups
     ("P-075/P-076 are the delegation itself"): `P-075` is the four delegated timing budgets Conductor
     *closes* (intent line 211-212 — open until Theme-2/F12 land), `P-076` is Pulse's tauri-driver e2e
     that receives the UI set.

     **RESOLVED at P3 (amended — this was the one open classification call).** Pulse's own
     `verification-matrix.json` settles it: `P-075`'s method is literally `dynamic-external` ("proven by
     an external harness, e.g. Conductor") and `P-076`'s is `e2e` (Pulse's own). So the pair folds into
     the two existing buckets — `P-075` is Conductor's lane, `P-076` is Pulse's own — and no third
     category is needed. **P4 operator decision:** `P-075` is classified `drive+observe`, not `auto`,
     because three of the four timing budgets it aggregates (`P-025`, `P-027`, `P-037`) are already
     `drive+observe` in the shipped table.

3. **Retiring the residual ledger.** `KNOWN_UNCLASSIFIED` must shrink to `[]` in the same change.

## Data sourcing (the open input)

The manifest carries **ids only** — no titles, no categories. `.andromeda/refs/pulse-capability-spec.md`
and the `pulse-v0_2_0-capability-audit` stop at `P-060`; nothing in this repo carries a title or category
for `P-061`..`P-082`. `intent.md` names 15 of the 22 with a phrase each. Pulse's own repository is present
on this host (`D:\dev\projects\andromeda-pulse\`, with an `andromeda-pulse-0.3.0` version directory) and is
the natural authority — sourcing the titles/categories from it is a research task for P3.

## Representation decision (CARRY 1 — folded in)

`coverage.rs` is documented "`&'static str`-backed so the table is a `static` with no allocation and **no
runtime IO**", and its two exactly-sixty tests (`matrix_has_exactly_sixty_capabilities`,
`p_ids_are_contiguous_p001_to_p060_zero_gaps_no_dups`) hardcode 60. Widening it is therefore a
representation question the plan must settle explicitly:

- keep the classification **code-native** (widen the static to the full accepted set, make the two tests
  manifest-relative instead of literal-60) — preserves the no-runtime-IO property and keeps
  `check_sut_drift` meaningful as a two-source comparison; or
- move the classification into **manifest data** (extend the TOML schema with per-capability
  title/category/mode) — which makes the drift check compare a source against itself.

Either way both exactly-sixty tests move with the change.

**RESOLVED at P4 (operator decision): keep it code-native.** `static COVERAGE` widens in Rust and the
manifest stays ids-only — preserving the no-runtime-IO property, keeping `coverage_matrix()`'s
`&'static [CapabilityRow]` signature (so no consumer crate's signature moves), and keeping
`check_sut_drift` a genuine two-source comparison rather than a self-check.

**Amended at P3 — the literal-60 surface is far larger than "its two exactly-sixty tests".** Verified
first-hand: **7 assertion sites** (`core/coverage.rs:142`, `:155`; `report/coverage.rs:89`, `:104`;
`cli/render.rs:290`; `cli/tests/cli_smoke.rs:195`; `tauri/commands.rs:327`), **4 test-fn names** spelled
`…sixty…`, and **8 doc/help sites** — one of which (`cli/cli.rs:49`) is user-visible clap help text, not
a comment. Guard one false positive: `cli/render.rs:42` `Lamp::Blocked => 60` is an ANSI color code.

## Doc reconciliation (CARRY 3 — folded in)

Three specs still name the superseded 60 while the manifest carries 82. Verified first-hand at the
`2026-08-09-sut-drift-check` wrap; pre-existing, not that chunk's drift; reconciled **here**, where the
classification becomes current-SUT-wide:

- `.andromeda/architecture.md:33` — "60-P-ID coverage tables"
- `.andromeda/design-system.md:7, :257` — "all 60 capabilities" / "the full 60-row wall"
- `.andromeda/layout-templates.md` — self-contradictory: `:37` reads "82 loaded (manifest set)" while
  `:121` still says "the full 60-row wall (P-001..P-060)" — residue of the 2026-08-08 de-hardcoding
  sweep, which updated the header strip but not the component prose. **Amended at P2:** the layouts
  distiller found a *second* stale site the CARRY did not name — `:177`, the `conductor coverage` verb
  line ("render the static 60-P-ID coverage matrix"). **Amended at P5 (operator decision):** a *third* —
  `:222`, the suite-run wireframe caption, which bakes both `step 60/60 done` and a per-state tally
  summing to 60. Illustrative-sample status is not an exemption: a baked count in a sample is the same
  stale-derived-fact class, and leaving it would recreate the very self-contradiction this chunk
  eliminates. All three are reconciled here.

## Surfaces and contracts touched

- `crates/conductor-core/src/coverage.rs` — `CoverageMode` (+1 variant, `ALL`, `label`, serde wire),
  `CapabilityRow`, `coverage_matrix()`, the row table, the module's own doc comment (it still says
  "all sixty ... `P-001`..`P-060`" and "0.1.0 done").
- `crates/conductor-core/src/drift.rs` — `KNOWN_UNCLASSIFIED` → `[]`; its committed-artifact test then
  degenerates to the plain zero-drift assertion the doc comment anticipates.
- Every consumer that exhaustively matches `CoverageMode` — the `conductor-report` Markdown render, the
  `conductor-cli` coverage table, and the Tauri coverage view + `ui/src/lamp.ts` (enumerated at P3 from
  the code graph, not assumed).
- Read-only inputs: `contracts/pulse-capabilities.toml` (the accepted set), Pulse's v0.3.0 capability
  source (titles/categories).

## Boundaries — explicitly NOT this chunk

- **Rendering treatment of the new mode.** The next working entry — _Out-of-scope classification
  treatment_ — owns making not-Conductor's rows visibly distinct in the coverage matrix, the footer
  roll-up and the CLI table, and never `Blocked`/`Fail`. This chunk introduces the classification and
  keeps existing surfaces compiling and correct; it does not design that treatment.
- **Scenarios for the in-lane ids** (`v2-04`, P-067/P-072/P-079) — a later Epoch-1 chunk.
- **The coverage completeness gate** (`v2-26`) — Epoch 6; `check_sut_drift` already ships and stays the
  live gate here.
- **Widening `PId`'s shape or the manifest's role.** The accepted set stays sole-sourced from
  `contracts/pulse-capabilities.toml`; nothing here re-sources or widens it.
- No new scenario TOMLs, no emission/verify/report engine change, no dependency delta expected.

## Definition of done (chunk-level)

Every id the manifest accepts is classified into exactly one of the four modes; the not-Conductor's set
is explicitly enumerated with its rationale; `KNOWN_UNCLASSIFIED` is `[]` and `check_sut_drift` passes
against the committed artifacts; the three doc-vs-manifest 60/82 gaps are reconciled; the full gate set
(nextest workspace, doctest, clippy `-D warnings`, audit, deny, `agent-run.sh run`) is green.
