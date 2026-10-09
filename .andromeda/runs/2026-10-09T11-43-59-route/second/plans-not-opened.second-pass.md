# Plans — what Phase 1 opened and what it did not

_Route run `2026-10-09T11-43-59-route`, conductor 0.4.0. Current as of the SECOND pass (after the operator's edit of
the intent at the first Phase 4); the first pass's list stands in `first/plans-not-opened.first-pass.md`. Written by
the orchestrator from its own reads. Each master was read through a line-folded scratch copy (`fold -s -w 1800`),
because twelve to fourteen-kilobyte lines stand in four of them and a plain read clips a long line; line numbers
below are the FOLDED copy's. A section named in `1a-tree.md` was opened unless that file says otherwise._

| Plan | Opened (lines or sections) | Not opened | A read clipped? |
|---|---|---|---|
| `architecture.md` | whole — folded lines 1–271 (first pass) | the keyed §Infrastructure Patterns files under `.andromeda/registries/contracts/architecture/` (the body holds one pointer line for that section) | no |
| `input.md` | whole — 168 lines (first pass) | nothing | no |
| `security-plan.md` | whole — folded lines 1–411 (first pass) | `security-plan-amendments.md` and its archive (the decisions log moved there) | no |
| `obs-plan.md` | whole body — folded lines 1–578 (first pass) | the nine keyed §3 contract files under `.andromeda/registries/contracts/obs-plan/`; `obs-plan-amendments.md` | no |
| `test-plan.md` | WHOLE body now — folded lines 1–559. First pass: 1–285 and 394–443. Second pass: 286–393 (§6 from the third scenario to its end, §7, §8) and 444–559 (§10, §11, §12's one pointer line) | the six keyed §3 contract files under `.andromeda/registries/contracts/test-plan/` (listed, none read — the tests validator cites two of them); `test-plan-amendments.md` | one read was REFUSED in the first pass (folded 147–329 exceeded the read tool's bound and returned nothing); it was re-read in two ranges. No read of the second pass was refused or clipped |
| `design-system.md` | the heading list of the whole file; `## Surface: cli` whole — folded lines 280–337; the one line of §Anti-Patterns — Universal Bans that bans a styled credential surface (line 348, read through a search) | everything else: §Brand Identity, §Color Palette, §Typography, §Spacing, §Depth Strategy, §Border Radius, §Motion, §Iconography, `## Surface: desktop-webview`, the rest of §Anti-Patterns, §Self-Validation Protocol, §Design Decisions Log | no |
| `layout-templates.md` | the heading list of the whole file; `## Surface: cli` whole — folded lines 176–312 | the header and `## Surface: desktop-webview` (folded lines 1–175), §Decisions Log | no |
| `a11y-plan.md` | the heading list; every line that names the command line, found by a search over the whole folded file and read: twelve lines in §1 (27, 71–74, 88, 112, 121, 158), §4 (228, 249), §5 (281), §9 (386) and §11 (497) | the body of every section apart from those lines; the nine keyed §3 contract files | no |

## Does `a11y-plan.md` have a command-line part?

No. Read far enough to say so: every place the plan names the command line says the same thing — the surface is
"not-assertable" (no DOM, no widget tree, no reach for an a11y tool), it is left out of §2 and §3, skipped in §4 and
§5, excluded from §9's coverage report, and §11 forbids treating it as an a11y verification path. Its text-paired
status labels are called "output-stream discipline", not an assertion. So for this plan the directive's premise
holds: the window is its whole assertable subject. The a11y validator, which read the plan whole in the first pass,
reported the same.

## `design-system.md` and `layout-templates.md` are not window-only

Each carries a `## Surface: cli` section that describes the command line — the surface 0.4.0 keeps. Inside those
sections stand things the version retires (the hold-point signature, the `[MANUAL]`, `[RESIDUAL]` and `[HOLD]`
labels, the tier and latency columns) beside things it keeps (the text-label rule, the stdout/stderr discipline, the
verb structure, the recessive tint for non-lamp labels). The draft cites those two sections where an entry REPLACES
what they describe.

## What the second pass's reading changed in the draft

One entry. `Spawned sidecar and shared-machine gate retired` now names the sidecar's stub and its mutation-roster
rows: test-plan §8 holds the stub as the standing mock of the sidecar, and §10's accepted-deliberate roster holds
members that exist only because of the sidecar and the shared-machine probe (members 1, 3 class C and 4). Nothing
else moved: the rest of what was read — §6's last five scenarios, §7's fixtures and goldens, §8's mocking rules,
§10's thresholds, §11's bans — describes things existing entries already retire or replace, and each is now cited
in `1a-tree.md` beside its entry.
