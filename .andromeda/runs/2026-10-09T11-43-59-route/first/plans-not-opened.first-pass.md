# Plans — what Phase 1 opened and what it did not

_Route run `2026-10-09T11-43-59-route`, conductor 0.4.0. Written by the orchestrator from its own reads. Each master
was read through a line-folded scratch copy (`fold -s -w 1800`), because twelve to fourteen-kilobyte lines stand in
four of them and a plain read clips a long line; line numbers below are the FOLDED copy's. A section named in
`1a-tree.md` was opened unless that file says otherwise._

| Plan | Opened (lines or sections) | Not opened | A read clipped? |
|---|---|---|---|
| `architecture.md` | whole — folded lines 1–271 (every section, §Design Philosophy to §Existing Scopes) | the keyed §Infrastructure Patterns files under `.andromeda/registries/contracts/architecture/` (the body holds one pointer line for that section) | no |
| `input.md` | whole — 168 lines | nothing | no |
| `security-plan.md` | whole — folded lines 1–411 (header, §Threat Model Summary, §Input Validation, §Data Protection, §Dependency Security, §Bootstrap phases, §Secret Management, §Error Handling, §Security Anti-Patterns with all eight subsections, §Security Decisions Log's one pointer line) | `security-plan-amendments.md` and its archive (the decisions log moved there) | no |
| `obs-plan.md` | whole body — folded lines 1–578 (§1, §2, §3's preamble, §4, §5, §6, §9, §10, §11, §12's pointer line; the body has no §7 or §8) | the nine keyed §3 contract files under `.andromeda/registries/contracts/obs-plan/` (listed through `registry.py contracts`, none read); `obs-plan-amendments.md` | no |
| `test-plan.md` | folded lines 1–285 and 394–443: the header, §1 whole, §2 whole, §3's preamble, §4 whole, §5 whole, §6 from its drivers table through the first two critical-path scenarios and the opening of the third, §9 whole | folded lines 286–393 and 444–559: the rest of §6 (critical-path scenarios 3 to 7 and the section's tail), §7 Test Data & Fixtures, §8 Mocking & Stubbing Discipline, §10 Quality Gates & Coverage Targets, §11 Test Anti-Patterns, §12 Decisions Log; the six keyed §3 contract files under `.andromeda/registries/contracts/test-plan/` (listed, none read) | one read was REFUSED, not clipped: folded 147–329 exceeded the read tool's bound and returned nothing; it was re-read as 147–227 and 228–285 |
| `design-system.md` | the heading list of the whole file; `## Surface: cli` whole — folded lines 280–337 | everything else: §Brand Identity, §Color Palette, §Typography, §Spacing, §Depth Strategy, §Border Radius, §Motion, §Iconography, `## Surface: desktop-webview`, §Anti-Patterns, §Self-Validation Protocol, §Design Decisions Log | no |
| `layout-templates.md` | the heading list of the whole file; `## Surface: cli` whole — folded lines 176–312 | the header and `## Surface: desktop-webview` (folded lines 1–175), §Decisions Log | no |
| `a11y-plan.md` | the heading list only | every section: §1 to §6, §9 to §12; the nine keyed §3 contract files under `.andromeda/registries/contracts/a11y-plan/` (listed, none read) | no |

## Why the test plan was not read whole

The directive asks for `security-plan.md`, `test-plan.md` and `obs-plan.md` whole "unless a read cannot hold them".
Two were read whole. The test plan is 173 722 bytes; after the first 285 folded lines and §9 the remaining 276
folded lines were left unread to keep the run's working context able to hold Phases 1 to 4. What the unread part
covers, by its headings: the last five critical-path scenarios of §6, fixtures, mocking, the quality-gate
thresholds, the test anti-patterns. No entry of the draft cites a section from the unread part.

## A premise of the directive this run measured differently

The directive (§PHASE 1) says `design-system.md`, `layout-templates.md` and `a11y-plan.md` have the window as their
whole subject. Read here: `design-system.md` and `layout-templates.md` each carry a `## Surface: cli` section that
describes the command line — the surface 0.4.0 keeps. Inside those sections stand things the version retires (the
hold-point signature, the `[MANUAL]`, `[RESIDUAL]` and `[HOLD]` labels, the tier and latency columns) beside things it
keeps (the text-label rule, the stdout/stderr discipline, the verb structure). The draft therefore cites those two
sections where an entry REPLACES what they describe. `a11y-plan.md` was not opened, so whether it has a
command-line part is not known from this run.
