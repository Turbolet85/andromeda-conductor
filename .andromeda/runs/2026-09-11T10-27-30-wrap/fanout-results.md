# Fan-out results — 2026-09-11-hosted-runner-endpoint-cause-probed

7 doc-agents, one parallel batch. **6 proposals across 2 docs; 5 clean.** Zero escalations.

| doc | verdict | detectors evaluated |
|---|---|---|
| architecture.md | **3 proposals** | D-arch-resources ×2 · D-arch-decisions ×1 · D-platform-claim (no hit) |
| test-plan.md | **3 proposals** | D-tests-derived-count ×1 · D-platform-claim ×2 (one `dependent-of`) · D-tests-coverage / -framework / -obs-harness (no hit) |
| security-plan.md | `proposals: []` | D-security-input · -subprocess · -deps · D-platform-claim |
| design-system.md | `proposals: []` | D-design-tokens · D-design-derived-count · D-platform-claim |
| layout-templates.md | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim |
| obs-plan.md | `proposals: []` | D-obs-instrumentation · -stack · -redaction · D-platform-claim |
| a11y-plan.md | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim |

Raw twins saved for `arch` and `test-plan` (carried proposals) and `obs-plan` (decode changed bytes —
its return arrived with `&amp;` escaping). The other four were empty-and-clean; this file is their record.

## Proposals (all 6 validated ROUTINE — staged to apply)

### architecture.md
1. **D-arch-resources** · §Infrastructure Patterns — directory tree · `scripts/` gains
   `webview2-cause-probe.ps1` beside `agent-run.sh` (CI-only diagnostic, not a harness entry point, no 6th
   command). Basis `architecture.md:217-218`.
2. **D-arch-resources** · §Occupied Resources — Environment variables · register `TEMP` + `LOCALAPPDATA` on
   the shipped-artifact-READS basis. Basis: `grep -oE '\$env:[A-Za-z_][A-Za-z0-9_]*' .github/workflows/ci.yml`
   → 2×`TEMP` (`:396`/`:440`), 2×`LOCALAPPDATA` (`:490`/`:501`); `grep -nE 'LOCALAPPDATA|[^_]TEMP'
   .andromeda/architecture.md` → 0 hits. **Pre-existing gap, not introduced by this chunk.**
3. **D-arch-decisions** · §Established Decisions [CI/CD] · retire "with a hosted-image policy or session
   property the leading unmeasured candidate" — both halves measured false. Basis `architecture.md:59`.

### test-plan.md
4. **D-tests-derived-count** · §9 CI Integration, Pipeline-structure E2E-webview row · the literal "two
   `continue-on-error` DIAGNOSTIC steps" → name the SET. Basis `test-plan.md:460`.
5. **D-platform-claim** · §9 Matrix builds · retire the same candidate-pair verdict. Basis `test-plan.md:469`.
6. **D-platform-claim** (`dependent-of: D-platform-claim`) · §6 E2E Test Strategy, desktop-webview driver row
   · the §6 restatement of the same claim. Basis `test-plan.md:307`. **This is the duplicate-occurrence
   mechanism working** — a single-site apply would have left the disproved pair alive in the driver table.

## Validate — the 6 checks

1. **Playbook.** All 6 matched or cleanly no-matched:
   - #2 → rule `:143` (external handle a SHIPPED artifact READS) → **routine, apply**. Exactly the basis that
     registered `RUNNER_TEMP` and `EDGEWEBDRIVER`.
   - #3, #5, #6 → rule `:149` (a master's OWN explicitly-provisional claim retired by the measurement the
     sentence names as its precondition) → **routine**. All three bounding clauses hold: (a) DIRECTNESS — the
     sentence says "unmeasured candidate" and this chunk measured exactly those candidates, evidence CI run
     `34586959536` in the report; (b) EXPLICIT STATUS — the replacement states what is now measured and what
     stays OPEN; (c) not a PARTIAL result — the reading for the two named candidates is determinate (keys
     ABSENT on both hosts; `SessionId 2` / `UserInteractive True`), so the escalate arm does not fire. The
     chunk's *third* probe returning nothing does not make this partial: that candidate is named in `v3-01`'s
     `observed_gap`, not in the sentence being retired.
   - #4 → rule `:127` (a stale literal enumerating a SET the code owns → apply as SET-NAMING, never a fresh
     count) → **routine**. The proposal already proposes the set form rather than "three".
   - #1 → **no matching rule, and not surprising → apply.** Rule `:49` (per-item content in an
     already-registered directory → dismiss) was tested and **fails its load-bearing qualifier**: its own
     reasoning scopes it to a directory arch tracks at DIRECTORY grain holding config CONTENT
     ("config files, which arch tracks at directory grain"). `scripts/` is tracked at per-FILE grain — the
     tree names `agent-run.sh` explicitly — and the new file is an executable entry point, not config
     content. Per amendment-flow, a subject match with a false qualifier is NO MATCH, and a failed-precondition
     rule is not a participant in the collision test. No unease: the plan anticipated it and the report
     substantiates it.
2. **Cross-contradiction.** None. #3/#5/#6 retire the same claim at three sites consistently. One wording
   divergence handled at apply rather than escalated: #5 says name no replacement candidate, #3 names
   elevation as the surviving lead. Resolved by re-deriving both from the invariant — record elevation as an
   observed DIFFERENCE explicitly **not** a demonstrated cause, and name no new "leading candidate", since
   installing an unmeasured candidate as the lead is precisely the shape being retired (clause (b)).
3. **Intent-consistency.** The report diverges from the plan's acceptance (`v3-01` UNMET). **Justified** —
   measured, and operator-directed — so intent was incomplete; resolved through the ledger at P7 (un-claim +
   notes), not as an escalation.
4. **Absence needs evidence.** Every absence claim cites its search (a11y's `grep -ci diagnostic` → 0;
   test-plan's `:4444` sites at `:124`/`:581` naming no bind verdict; arch's 0-hit env grep). One inverse
   error caught — see check 5.
5. **Expected-amendments reconciliation.** Four of the plan's five doc entries were proposed by detectors.
   The fifth — **`a11y-plan.md` §9** — was **NOT proposed, correctly**: `grep -ci diagnostic
   .andromeda/a11y-plan.md` → **0**. The report named a11y-plan as a co-owner of the diagnostic-step-set fact
   **with no hit in it**, which is exactly the error the report template warns against. The fact has one
   owner, `test-plan.md:460`, already covered by #4. Report disposition corrected; **no amendment owed to
   a11y-plan.** Two detector finds beyond the plan's floor (#5, #6) — the floor working as a floor, not a cap.
6. **Disproved-claims disposition.** All three report entries DISPOSED: #1 → proposals #3/#5/#6 (three sites,
   two docs); #2 (the residuals `:4444` framing) → **routed to P5 route-resolve** per operator directive item
   3, `residuals.md` being outside the seven masters so no detector owns it; #3 (the chunk's own acceptance
   premise) → **P7 un-claim** per operator directive item 1.

**Escalations: 0.** No HALT owed.
