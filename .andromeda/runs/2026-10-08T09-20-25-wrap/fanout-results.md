# Fan-out results — 2026-10-08-capture-canary-pairing-window-corrective

Seven Explore doc-agents, one parallel batch, each sent the amendment-flow prompt verbatim with its doc, the report,
its keyed-contract render where the doc is migrated (architecture · test-plan · obs-plan · a11y-plan), and its
detectors (28 detector slots over the seven prompts — 5 · 4 · 3 · 3 · 5 · 5 · 3 — equal to the 28 `doc:` names over
the drift-base's 22 entries, asserted before the batch). Each return was read for transport entities (`&lt;` /
`&gt;` / `&amp;`: none in any return) and parsed as YAML; YAML comments beside a `proposals:` list are part of the
YAML return, so no return was changed by stripping and no raw twin is kept.

Rejected before the checks: for a source the report does not carry 0 · for a coordinate neither the report's bullets
nor its last section holds 0. Both proposals cite only coordinates the report states or its **New text, by line**
section lists (checked row by row below).

## design-system — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no UI element rendered (the Coverage row reads tokens
n/a); every old value of the Counts / qualifiers bullet grepped in the doc at 0 hits (one `pairing` hit is a font
pairing in the typography rationale); neither disproved claim retires a platform, runner or driver verdict.

## layout-templates — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no new surface or region; the numeric values and the
`pipeline-fault` qualifier at 0 hits in the doc; its four `canary` mentions are the preflight readiness canary, a
different mechanism, carrying no count; no platform verdict retired.

## a11y-plan — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no interactive element added; neither the a11y
violation schema nor the obs log schema moved; no platform verdict retired; one `canary` hit in the key file for the
screen-reader test pattern concerns Pulse's dedupe of a new canary and states nothing about the pairing window.

## security-plan — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no new external-input surface (test-support code
reading the same Pulse log the capture-ingest row governs; an unparseable stamp or one equal to the instant is never
selected; the line still leaves through `emit`); sidecar spawn, preflight and data dir untouched; no dependency added;
no platform verdict retired; the pairing-window pattern at 0 hits in the doc.
One item it named outside its proposals, verified here and carried to P5: security-plan names the posture contract
nowhere (`grep -c 'leg-posture' .andromeda/security-plan.md` → 0), while its fixed-path-manifests row says a committed
artifact parsed at runtime "earns its row whether or not a shipped binary is the one parsing it"
(`.andromeda/security-plan.md:113`). Whether the contract's test-binary reader earns a row there is a question for
the version close; it joins the new `CARRY` on that entry. Not a change of this chunk, so not an amendment here.

## architecture — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no registry-class resource, no dependency, no second
owner or writer of a registered artifact path (the contract gained an add-only dated block; its row's enumeration
stays accurate); the registry-size check is the orchestrator's; no platform verdict retired. It did not propose an
amendment for the "NO Rust reader" row: a reader is neither an owner nor a new resource, and the operator's
disposition is a `CARRY`.
One fact it named outside its proposals, verified here: the claim has a second architecture site the report had not
listed — the keyed contract for the directory structure, in its `contracts/` tree row
(`.andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:30`, "the two
members no Rust code reads"). The unbounded grep confirms it: 8 hits, 4 about the posture contract. The report's
bullet first read 7 hits and 3 sites, from a pattern that required 40 characters after the match; the bullet is
corrected in place and says so. The `CARRY` names all four sites.

## obs-plan — proposals: 1
Verdict: 1 proposal under `D-obs-instrumentation` (warning); the return says no invariant is strictly violated and
that it carries the report's Expected-amendments site under the §4 detector. Its comments, in substance: no hit on
`D-obs-stack`, `D-obs-redaction`, `D-obs-ci-gates`, `D-platform-claim`.

1. section: §4 Span / Trace Coverage → Scenario: Headless deterministic scenario run with MCP read-back
   verification → Real-model posture (2026-09-22) · change: keep the sixth series' sentence as written and add
   beside it a dated note — from this chunk the pairing reads a selected canary digest's inference across the
   emission instant, so a digest ticked just before the instant prints its log-borne outcome on the recorded d1 and
   d2 shapes; the two-line case and the read-from-Pulse's-log rule stand; the limit of the proof; no span, attribute,
   log line, allowlist entry or envelope key added · basis: `canary_pairing.rs` 353-366 · 368-382 · 462-489,
   `real_model_common/mod.rs` 84-86, `.andromeda/obs-plan.md:221`.
   Coordinates: each `canary_pairing.rs` range is a row of the report's last section; 84-86 is a row; `:221` is the
   report's. Its sweep is over its own doc and key files (line 221 only).
   **Disposition: apply** — check 1: the playbook rule "Accurate this-chunk addition" (routine: it reconciles the
   body's stated mechanism to what the chunk shipped, the report showing the line's grammar and the scrub chain
   unchanged), and the plan's P5-approved `Expected amendments (wrap)` entry names this change itself. Not a boundary
   widening: the pairing reads more lines of a log the capture already ingests whole and prints the same closed-enum
   fields. Checks 2-4: no opposing proposal; inside the chunk's intent; the absence claim cites its search. Check 5:
   this is the plan's obs-plan entry. The applied text is re-derived from the report, never pasted from `change`.

## test-plan — proposals: 1
Verdict: 1 proposal under `D-tests-derived-count` (warning). Its comments, in substance: no hit on
`D-tests-coverage` (the new path carries default-suite tests at the harvest tier the leg mandates, on in-test lines,
no network, no clock, no drive), `D-tests-framework`, `D-tests-obs-harness`, `D-platform-claim`; the numeric values
133 · 139 · 1233 · 1239 at 0 hits in the doc and its six key files.

1. section: §6 → Scenario: Fingerprint-storm → the Real-model interpretation leg bullet (`.andromeda/test-plan.md:260`)
   · change: keep the sixth series' sentence as written and insert after it, before the closing series-set sentence,
   a dated record — the instant selects the ticks and no longer drops lines; the recorded d1 and d2 shapes read
   `surfaced … deduped=true`; the d3 shape and an unplaceable tick get no line; pinned at the harvest tier by the
   default-suite arms of `canary_pairing.rs`, the pre-fix reading held beside them; the limit of the proof; no test
   or capture count · basis: `real_model_common/mod.rs` 71-80 · 83-86 (84-86), `canary_pairing.rs` 353-366 · 368-382
   · 408-432 · 434-460 · 462-489, `real_model_live.rs` 721-724.
   Coordinates: 71-80 and 83-86 are added ranges as printed; every other range is a row or an added range of the
   report's last section; `:260` is the report's.
   **Disposition: apply** — check 1: "Accurate this-chunk addition" (routine), and the plan's P5-approved entry names
   the change itself. Checks 2-4 as above. Check 5: this is the plan's test-plan entry. Re-derived at Apply.

## Validate — the remaining checks

- Check 2 (cross-contradiction): the two proposals edit different masters and say the same thing. None.
- Check 3 (intent-consistency): the report matches the working-route entry and the plan's acceptance criteria; its
  three deviations are justified and accepted by the operator; the scope record holds no line (`scope: clean`).
- Check 5 (expected amendments): the plan lists two entries, obs-plan §4 and test-plan §6; both were proposed. No row
  of `citation-dispositions.md` reads `claim false`.
- Check 6 (disproved claims), both DISPOSED:
  - the "NO Rust reader" claim (architecture's row and key file, two leaves) → routed to the working route: one new
    `CARRY` on the version-close entry, on the operator's recorded direction ("one new CARRY on the version-close
    entry for the registry row you noticed" — the operator, 2026-10-08, this wrap's invocation arguments; the pc
    overseer's wrap relay, section 3, says the same and why: correcting the row touches §Occupied Resources at 1 B of
    headroom). No amendment here; P5 writes it.
  - the plan's step-4 summary-line prediction → no master states it; both readings are in
    `evidence/failing-first.md`; its lesson goes to curation (P3).
- Escalations: none.
