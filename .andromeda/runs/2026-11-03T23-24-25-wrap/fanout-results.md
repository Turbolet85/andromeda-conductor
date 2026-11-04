# Fan-out results — 2026-11-03-architecture-registry-check-retired

Batch stage1 · `fanout.py check`: `VERDICT: 7 of 7 pass · failed: none` · `proposals: architecture 1 · security-plan 0 · design-system 0 · layout-templates 0 · test-plan 0 · obs-plan 0 · a11y-plan 0`. All seven masters whole (`detail.py check`: `whole · 0 files` each), so no detail cascade; sampler not drawn (no extracted doc). No fix and no twin.

architecture: 1 proposals · fanout-architecture.md
- note: the D-arch-* and D-platform-claim detectors found nothing; the arch-md hits (`fct-bkwd4qxc`'s pointer to `architecture-amendments.md`, the key file's `master` field and comment) cite only the removed rendered pair, so the detectors left them to the citation sweep.
security-plan: 0 proposals · fanout-security-plan.md
- note: no drift; 0 hits for the retired items.
design-system: 0 proposals · fanout-design-system.md
- note: no drift; no UI surface.
layout-templates: 0 proposals · fanout-layout-templates.md
- note: no drift.
test-plan: 0 proposals · fanout-test-plan.md
- note: the three arch-md sites (`fct-zsffwtxs`, `fct-r5xeb3bp`, `fct-222nwunb`) cite «architecture.md §…» sections that still stand in doc:architecture, so the claims hold. A re-point is not this pass's to propose.
obs-plan: 0 proposals · fanout-obs-plan.md
- note: no body drift; the obs-plan SIDECAR entry `amd-yhtttkb7`'s tokio PROVISIONAL mark is left to the orchestrator.
a11y-plan: 0 proposals · fanout-a11y-plan.md
- note: no drift.

## Dispositions

fanout-architecture.md
1. apply — check 1: `D-retired` on an item the report's Retired bullet names (`scripts/arch-registry-check.py`), routine. It is also the plan's Expected amendment 1 (check 5), whose entry names the change itself: the clause leaves, the two standing claims stay, worded without a count.
2. apply — check 5 (Expected amendment 3, raised by main): §Stack and Technologies (Async runtime row, `fct-fjnaazvl`) · §Established Decisions ([Language / Runtime], `fct-h4z4u2a2`) · §Inherited Defaults (Language / runtime, `fct-jiaouq3r`). No body change: no body carries a PROVISIONAL mark (`rec.py find --doc all --regex 'PROVISIONAL|OPEN — '` → 0 hits). A sidecar entry carries the ratification of the tokio 1.52.3 → 1.54.1 bump, whose earlier entry `amd-ziotmlng` reads «PROVISIONAL». The founder confirmed it by dialog on 2026-11-03 in pc-overseer's session (CARRY `cry-qkxs4v53`, route commit `ddff6f2`). Main writes this entry (`architecture-entry-1.md`). · as: no body edit — the three sites already read tokio 1.54.1; the entry is main's

fanout-test-plan.md
1. apply — check 5 (Expected amendment 3, raised by main): §1 Test Scope Summary (`fct-efo3vjjh`) · §8 Mocking & Stubbing Discipline (`fct-odwlbdq3`). No body change. The sidecar entry ratifies `amd-qxfctp32`'s PROVISIONAL bump on the same founder dialog confirm (CARRY `cry-qkxs4v53`). Main writes this entry (`test-plan-entry-1.md`). · as: no body edit — both sites already read tokio 1.54.1; the entry is main's
- Expected amendment 2 (test-plan §10 / §11 citing `architecture.md §…`): not carried as an edit. The plan's entry left the call to the detectors, and test-plan's detector found the cited sections standing in doc:architecture, so the claims stay true. The old file-name spelling is recorded in the report's «Old `.andromeda/{doc}.md` reach» bullet for the operator's card.

fanout-obs-plan.md
1. apply — check 5 (the CARRY `cry-qkxs4v53`'s «wherever the records carry it», raised by main): §1 Obs Scope Summary (`fct-tia6ntih`, `fct-lqqdem2q`). No body change. The sidecar entry ratifies `amd-yhtttkb7`'s PROVISIONAL bump. Main writes this entry (`obs-plan-entry-1.md`). · as: no body edit — both rows already read tokio 1.54.1; the entry is main's

Check 2 (cross-contradiction): none, one body proposal. Check 3 (intent): the report diverges from no intent clause; scope record none. Check 4 (absence): proposal 1's sweep claim («hits only this fact») matches the pre-pass `arch-reg` 1 standing / `size-guide` 1 standing rows. Check 6: Spec claims disproved — none. Check 7: no extracted doc. Rejected for a source the report does not carry: 0. Escalations: 0.
