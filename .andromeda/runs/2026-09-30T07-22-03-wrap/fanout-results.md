# Fan-out results — 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir

Seven Explore doc-agents, one parallel batch, prompts verbatim from `prompt-{doc}.md` (detector YAML from
`drift-base.md`; `D-platform-claim` scoped to all seven; no doc renders keyed contracts — `registry.py contracts`
read `NOT MIGRATED` for architecture/test-plan/obs-plan/a11y-plan and wrote none for the other three). Returns
stripped of commentary (each agent appended `#` notes after its YAML); no entity mangling in any return
(`&lt;`/`&gt;`/`&amp;` count 0). No raw twin warranted (every `proposals: []` return was unchanged by stripping
beyond its trailing `#` notes, whose substance is summarized per doc below).

## Verdicts
- **design-system.md** — `proposals: []` (notes: no UI element added; no moved count baked; no platform verdict retired).
- **layout-templates.md** — `proposals: []` (notes: the capture line is a test-binary print, not a cli/webview surface; no moved count present).
- **obs-plan.md** — `proposals: []` (notes: no must-trace op; `sha2` is no OTel SDK; the witness prints a class only; no CI step change).
- **a11y-plan.md** — `proposals: []` (notes: no interactive element; schema unchanged; `:424` states the 2026-09-01 BiDi verdict, not retired; flags expected amendment 5, `:424`'s diagnosis and the `12 passing` sites as outside its detectors — raised below).
- **security-plan.md** — 2 proposals (S1, S2).
- **architecture.md** — 4 proposals (A1-A4).
- **test-plan.md** — 4 proposals (T1-T4).

## Proposals and dispositions

| id | detector (sev) | doc · section | change (summary) | disposition · check that decided |
|---|---|---|---|---|
| S1 | D-security-input (escalate) | security-plan §Input Validation, real-model capture ingest row (`:121`) | four-stage scrub (`mask_workspace_key` first, key from the guarded data-dir path, Pulse's key a path), the class-only rendering witness; the reader checks sha256 digest pins, `PINNED_CAPTURE` gone | **apply — routine**, playbook `a chunk SHIPS the fix that a spec master's own body already names as route-owned-not-shipped` (the row names the mask "owned by the new `v3-09` series route entry"). `Boundary widening` checked first: no match — the mask NARROWS output, the new data-dir read goes through the existing guard in the same test binary, its value is never printed |
| S2 | D-security-input (escalate), dependent-of S1 | security-plan §Security Anti-Patterns → Data Protection (`:335`) | BREACH → remedied with the frozen-file residual; scrub list leads with the mask | **apply — routine**, the same rule (the body names both remedies route-owned); a remedy discharges a recorded breach, it ratifies no widening (security.md 2026-09-29) |
| A1 | D-arch-decisions | architecture §Stack → Hashing / digest (`:20`) | `sha2 0.10` test-only `conductor-run` dev-dependency; blake3 stays the only shipped hash | **apply — routine**, `Accurate this-chunk addition` |
| A2 | D-arch-decisions, dependent-of A1 | architecture §Infrastructure Patterns → Build system (`:212`) | dev-test stack list gains `sha2` | **apply — routine**, same |
| A3 | D-arch-resources | architecture §Occupied Resources, the posture-contract row (`:182`) | the row names the 2026-09-30 series section | **apply — routine**, `Accurate this-chunk addition` (the row exists; expected amendment 4) |
| A4 | D-arch-resources | architecture §Standard Contracts → corpus access (`:113`) | the 2026-09-29 breach clause → remedied + residual | **apply — routine**, the route-owned-fix rule (mirror of S2) |
| T1 | D-tests-coverage | test-plan §6 real-model leg (`:336`) | "pinned byte-equal" → sha256 digest pins, the source/tamper/mask arms | **apply — routine**, `Accurate this-chunk addition` (expected amendment 3). The other `byte-equal` hit (`:364`, the coverage-matrix render) is a different claim — no change |
| T2 | D-tests-coverage | test-plan §1 fifth env-handle class (`:87`) | `real_model_live.rs` gains a second guarded data-dir read, `workspace_key()` | **apply — routine**, same |
| T3 | D-tests-derived-count | test-plan §1 desktop-webview driver (`:56`) | "12 passing / 0 failing / 2 skipped" → the expected-skip SET, both run ids | **apply — routine**, the detector's prescribed form and the doc's own house rule (`:307`, moving count → SET) |
| T4 | D-tests-derived-count, dependent-of T3 | test-plan §6 surface/driver table (`:307`) | the same | **apply — routine**, same |

## Orchestrator raises (Validate checks 5 and 6)
| id | source | doc · section | change | disposition |
|---|---|---|---|---|
| O1 | expected amendment 5 + disproved claim 1 + 3 | a11y-plan §9 Pipeline integration + §11 → CI | the routine arm's responsive-probe wait (`RESPONSIVE_MS = 250`), the standing probe-bound stall arm, why a timer-scheduled stall cannot witness it, `Page/Frame is not ready` as axe's catch-all for any missed 1 000 ms readiness probe; both measured halves; CI#36681853843's configuration | **apply — routine** (report substantiates: Symbols, Spec claims disproved 1/3, Cross-project CI) |
| O2 | Counts moved (12 → 13) | a11y-plan §9 (`:465`) | the CI job's "green at 12 passing / 0 failing / 2 skipped" → the expected-skip SET, both run ids (consistent with T3/T4) | **apply — routine** |
| O3 | Counts moved (12 → 13) | architecture §Established Decisions (`:60`, the unanchored dev-host clause "(12 passing / 0 failing / 2 skipped)") | → the expected-skip SET with the 2026-09-30 dev-host reading | **apply — routine** |
| O4 | expected amendment 3 (§7) | test-plan §7 provenance SET (`:403`) | the committed live-leg capture evidence (a chunk's `evidence/`, read-only, sha256-pinned before grading) joins the sanctioned provenance set | **apply — routine** |

Not changed (read by offset, `windows.py` over each hit): architecture `:60 @c4164` and `:250 @c1436`, a11y-plan
`:115 @c1515` and `:516 @c2540` — each "12 passing / 0 failing / 2 skipped … as measured at run 35208593666", a dated
record true of its run; a11y-plan `:516 @c603` — the 2026-09-16 local narrative, historical. a11y-plan `:424` — the
2026-09-01 BiDi provenance narrative, true of that session (claim 3 disposed by O1).

## Check results
1. Playbook — 14 routine, 0 escalate (S1/S2 escalate-severity resolved by the route-owned-fix rule after the widening rule failed to match).
2. Cross-contradiction — none (S2/A4 and T3/T4/O2/O3 state the same facts in the same direction).
3. Intent-consistency — the deviations (stall form, rendering by path leaf, sidecar rebuild, agent-launched `pulse-app`) each carry the operator's word or a measured justification; scope record empty.
4. Absence-needs-evidence — every `12 passing` / `byte-equal` / `workspace key` hit read by offset window (above).
5. Expected amendments — 1 → S2 (+A4); 2 → S1; 3 → T1 (+T2, O4; the dev-test stack's `sha2` rides T1, `tokio-stream` at `:115` names a different stub); 4 → A1 (+A2), A3; 5 → O1 (+O2).
6. Disproved claims — 1 → O1; 2 → S1 (Pulse's key a path; test-plan `:307`'s one `workspace key` hit says nothing about a basename); 3 → O1 (`:424` unchanged, historical).
