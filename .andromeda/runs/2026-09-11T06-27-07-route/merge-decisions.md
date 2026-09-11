# Merge decisions — conductor-0.3.0 route draft

_Phase 3. 21 suggestions from 5 validators: **11 applied · 4 adjusted · 1 rejected · 5 deferred to Phase 4**.
Next-version mode, so every Insert was additionally checked against master-route's 121 `complete` records
(0.1.0 + 0.2.0) — validators see only their plan and the draft, never what shipped._

## Applied

- security Rewrite · applied · `Hosted-runner endpoint cause probed` gains "host paths scrubbed" — deduped with the obs Rewrite below (same direction, two independent cites: security-plan §Anti-Patterns → Logging; obs-plan §9 log-conformance gate).
- obs Rewrite · applied · same suggestion as above; logged as the second cite rather than a second edit. Dropped "Edge/WebView2" and "loaded"/"runner" qualifiers to hold ≤25 words while adding the scrub clause.
- security Rewrite · applied · `Interpretation proven live` gains "through the emission path" — names the sanctioned injection route, which per security-plan §Anti-Patterns → Data Protection is what excludes the banned corpus-row staging.
- tests Rewrite · applied · `Diagnostic-quality cluster off the drift pin` gains "gate pin and committed matrix moving together" — deduped with the obs Rewrite below; concrete and load-bearing (dropping four ids reds both gate arms unless pin and artifact move in one change).
- obs Rewrite · applied · same direction as the tests Rewrite above; its cite (coverage-matrix completeness gate asserts the committed artifact) is the second basis for one edit.
- tests Rewrite · applied · `Real-model leg posture and grading rule` gains "per-leg quiet window" — matches the Phase 0 sequencing note independently derived from 0.2.0's in-lane round (Pulse dedupes against any open incident; each `conductor run` fires its own canary).
- tests Rewrite · applied · `Scenario-assertion audit gate` now names itself "a CI gate or an operator instrument" — a check in neither is enforced by nothing, which is this project's recurring false-green shape.
- obs Rewrite · applied · `P-025 measurement contract` now reads "Pulse-emitted observable" — forecloses writing the contract against Conductor's own envelope, which would be unimplementable by the party that must implement it.
- a11y Rewrite · applied · `A11y CI gate at an honest terminal` now reads "routine arm's asserted verdict green in CI" — a skipped-green run and a real pass are otherwise indistinguishable, the exact defect `2026-09-07-a11y-ci-gate` shipped its printed-verdict assertion to prevent.
- a11y Rewrite · applied · `Keyboard and focus-order coverage ownership` re-scoped to "the hold-dependent trap and restoration half" — **the orchestrator measured this at HEAD before applying**: `accessibility.e2e.ts` carries 2 `browser.keys` calls asserting SC 2.1.1 (`:373`) and SC 2.4.3 (`:393`), so the intent's F1.2 premise ("the routine arm carries ZERO `browser.keys` calls") is false at HEAD. It was true when measured at the `2026-09-07-a11y-ci-gate` P3 and was overtaken the same day by `2026-09-07-sr-findings-fixed`, whose master record states the routine arm gained the hold-free Operable pair. PREMISE-CORRECTION carried to the matrix `notes` + `observed_gap` for `v3-03`; the intent is immutable and is NOT edited.
- tests Rewrite · applied · `Full-gate regression` gains "every gate green under both runners" — absorbs the runner-portability half directly.

## Adjusted

- obs Rewrite · adjusted · `Scenario tier honesty` — direction applied ("inside the closed tier set", which forecloses the wrong fix of minting a fourth tier for the two scenarios beyond every tier), but the literal `slo_tier` enum name was dropped and the title shortened to hold ≤25 words while keeping "three situations kept distinct".
- obs Rewrite · adjusted · `Full-gate regression` — "log-conformance and zero-unlogged-panics gates" generalized into "every gate green", since a full-gate regression by definition runs the inventory that test-plan §9/§10 and obs-plan §9/§10 register; enumerating three of them in a ≤25-word route line is the wrong altitude.
- a11y Rewrite · adjusted · `Full-gate regression` — "at zero WCAG AA violations" folded into the same "every gate green" clause for the same reason; a11y-plan §10 makes it the `a11y` job's pass bar, so it is inside the inventory rather than beside it.
- obs Rewrite · adjusted · `A11y CI gate` — "naming which arm still produces the a11y violation record" could not fit beside the asserted-verdict clause (both validators loaded the same line). Carried to `v3-02`'s matrix acceptance and notes instead of the route line, so the obligation survives at chunk-plan altitude.

## Rejected

- security Rewrite · rejected · `Structurally-dead assertion class retired` + "the validated config boundary held" — declare-only is an existing validated shape (five scenarios already retired to it per `architecture-amendments.md:403`), and a declare-only scenario violating the config boundary would fail `from_toml_str` at load; the route line asserting an invariant the loader already enforces adds nothing, and the clause does not fit ≤25 words.

## Deferred to Phase 4 (operator decision — all five widen the version beyond its authored intent)

These are **not** rejects: each cites a plan section accurately, none is already-delivered (checked against all 121
master `complete` records), and two were found independently by two validators. But 0.3.0's intent scopes the
version to retiring four named compromises, and the intent's own framing says the out-of-scope list exists "so the
intake does not mint" work. Adding these would take Epoch 3 from 2 to 7 chunks and break the uniform-weight
discipline. The operator decides.

- design Insert + design Insert + a11y Insert · deferred · **deduped into one item**: the footer `contentinfo` status strip and the report-site operator-checklist render. Design cites layout-templates §Component — Footer ("DESIGNED, NOT SHIPPED … route-owned gap") and §Component — Operator-checklist; a11y independently cites §4's landmark table and §11's ban on a landmark-less window. Two domains reaching the same two surfaces from different plans is the strongest signal in this merge. The a11y phrasing covers both surfaces in one chunk.
- tests Insert · deferred · `Control-panel-launched parity at a terminal` — test-plan §1 Critical Path 7 / §6 Scenario 7 record the control-panel-LAUNCHED half as OWED on the tauri-driver leg, whose fate Epoch 3 decides. Genuinely coupled to this version's work, but it is a fifth capability nobody's intent asked for.
- security Insert · deferred · `CI-fetched WebView2 runtime disposition` — security-plan §Dependency Security's third dependency class floats always-latest with ONE stated exit condition (the `a11y` job actually gating), which `v3-02`'s terminal decides. The closest of the five to being in-scope: it is a consequence of this version's own decision rather than new work, so folding it into `v3-02`'s chunk plan is a live alternative to a chunk of its own.

## Phase 4 resolution of the deferred five (operator, round 1)

All five resolved; see `review-feedback-1.md`. **No Insert became a chunk** — the intent's scope held.

- security Insert · **folded** · into `v3-02`'s chunk plan rather than minted, on the operator's ground that the pin's only stated exit condition IS `v3-02`'s terminal fork. Rides the matrix `notes` for `v3-02`.
- design Insert ×2 + a11y Insert · **recorded as a residual** · deduped into one §Carried residuals entry, carrying an operator-supplied coupling that changes `v3-02`'s planning: if the routine arm starts gating, a11y-plan §11's landmark ban could turn the unshipped footer into a RED inside the very gate `v3-02` is trying to make green — so `v3-02` must measure the routine specs against the current console BEFORE promising a green terminal. Also carried to `v3-02`'s matrix `notes`.
- tests Insert · **recorded as a residual** · Critical Path 7's control-panel-launched half, in §Carried residuals for the next intake.

Separately at the same review, the 0.2.0 carried residual `secret-scanning-ci-gate` was **absorbed** as `v3-11`
with one chunk at the head of Epoch 5 — discharging its own "Revisit at 0.3.0 scoping" rather than re-carrying
it a third time. Chunks 13 → 14; capabilities 10 → 11.
