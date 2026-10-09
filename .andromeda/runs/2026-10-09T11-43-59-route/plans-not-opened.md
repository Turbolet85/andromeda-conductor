# Plans — what Phase 1 opened and what it did not

_Route run `2026-10-09T11-43-59-route`, conductor 0.4.0. Current as of the THIRD pass (after the operator's edit of
F7b at the second Phase 4); the earlier lists stand in `first/plans-not-opened.first-pass.md` and
`second/plans-not-opened.second-pass.md`. Written by the orchestrator from its own reads. Each master and each
long-lined key file was read through a line-folded scratch copy (`fold -s -w 1800`), because lines of six to
fourteen kilobytes stand in them and a plain read clips a long line; line numbers below are the FOLDED copy's. A
section named in `1a-tree.md` was opened unless that file says otherwise._

| Plan | Opened (lines or sections) | Not opened | A read clipped? |
|---|---|---|---|
| `architecture.md` | whole — folded lines 1–271 (first pass); its four keyed §Infrastructure Patterns files, whole (third pass): `build-system`, `ci-cd-approach`, `deployment-model`, `directory-structure-crate-per-seam-cargo-workspace` | nothing | no |
| `input.md` | whole — 168 lines (first pass) | nothing | no |
| `security-plan.md` | whole — folded lines 1–411 (first pass); it has no keyed files | `security-plan-amendments.md` and its archive (the decisions log moved there) | no |
| `obs-plan.md` | whole body — folded lines 1–578 (first pass); its nine keyed §3 files, whole (third pass): `bootstrap-phases-for-downstream-skills`, `correlation-no-distributed-tracing`, `heartbeat-ticks`, `log-file-location`, `log-format-json-schema`, `logging-stack`, `otel-sdk-init`, `service-identity`, `snapshot-paste-to-ai-integration` | `obs-plan-amendments.md` | no |
| `test-plan.md` | whole body — folded lines 1–559 (first and second passes); its six keyed §3 files, whole (third pass): `5-command-implementation`, `bootstrap-phases-derive-for-route-setup-project`, `log-format`, `pid-file`, `status-endpoint-shape`, `test-data-bootstrap` | `test-plan-amendments.md` | one read was REFUSED in the first pass (folded 147–329 exceeded the read tool's bound and returned nothing); re-read in two ranges. None since |
| `design-system.md` | the heading list; `## Surface: cli` whole (folded 280–337); the one line of §Anti-Patterns that bans a styled credential surface (348); and, third pass, §Brand Identity whole and §Color Palette from its rationale through §Semantic Colors and the Verdict-vs-ReportState note (folded 3–64) | §Color Palette — Border Progression, §Typography, §Spacing, §Depth Strategy, §Border Radius, §Motion, §Iconography, `## Surface: desktop-webview`, the rest of §Anti-Patterns, §Self-Validation Protocol, §Design Decisions Log | no |
| `layout-templates.md` | the heading list; `## Surface: cli` whole (folded 176–312) | the header and `## Surface: desktop-webview` (folded 1–175), §Decisions Log | no |
| `a11y-plan.md` | the heading list; every line of the body that names the command line (twelve lines in §1, §4, §5, §9, §11); its nine keyed §3 files, whole (third pass): `a11y-testing-tool-pick`, `bootstrap-phases-derive-for-route-setup-project`, `ci-integration`, `contrast-verification-harness`, `focus-management-test-harness`, `keyboard-test-harness`, `screen-reader-test-pattern`, `structured-violation-json-schema`, `wcag-criteria-mapping` | the rest of the body of §1 to §6 and §9 to §12 | no |

All 28 keyed contract files are now read (4 + 9 + 6 + 9; 79 444 bytes together). Eleven of them hold a line longer
than the read bound's safe width; each was read folded, none clipped.

## What the third pass's three closures changed in the draft

**The keyed contract files: no entry changed.** What they hold is cited beside the entries in `1a-tree.md`:
- test-plan `5-command-implementation`, `status-endpoint-shape`, `log-format`: the five verbs, the envelope and
  the journal are all defined over what `Harness verbs on the one form` and `Run record for the one form` already
  re-subject. Its `logs` retention line ("no rotation needed — bounded synthetic runs") and obs-plan
  `log-file-location` ("Rotation: N/A") are the two standing statements `Disk bound over ten hours` replaces.
- obs-plan `heartbeat-ticks` ("CLI: N/A — short-lived") and `logging-stack` (one writer, opened per command) are
  what `Own log over a run of hours` replaces.
- architecture `build-system` holds the frontend-bundle step, the `ensure_frontend` arm and the Tauri-tree audit
  exceptions — all inside `Conductor's window retired`; `deployment-model` says execution is "necessarily
  co-located on the dev host" — `Records restated` and `Fixed local address retired` own it.
- All nine a11y keys describe the webview and its three suite families; `Conductor's window retired` retires them.
  `a11y-testing-tool-pick` repeats that the command line has no tool pick.

**The two design-system sections: no entry changed; both rewrites stand.**
- "non-verdict readings apart": §Semantic Colors' note says the report state "adds three outcomes the machine did
  NOT pass/fail and which therefore must never collapse into `Fail`", and that the manual and residual treatments
  are distinct "precisely so neither silently reads as `Fail`". Two of those three treatments leave in Epoch 5, so
  a place outside the verdict set for a reading that is not a verdict is what the section's own rule asks for.
- "restated without window or hold": §Brand Identity's signature element is the paused-count hold-point, its
  domain anchors name the operator-pause hold and the five-valued report state, and §Surface Scale's levels are
  the window's (the frameless titlebar ground, the go/no-go dialog). The command line's design leans on both.

**The master-route record `2026-06-23-line-oriented-output-rendering`: checked, `complete`.** It delivered the
command line's rendering in 0.1.0, so no entry installs it again; `Run read from the command line` re-subjects it.

## Does `a11y-plan.md` have a command-line part?

No. Every place the plan and its nine keys name the command line says the surface is not-assertable; its
text-paired status labels are called output-stream discipline, not an assertion.

## `design-system.md` and `layout-templates.md` are not window-only

Each carries a `## Surface: cli` section that describes the command line — the surface 0.4.0 keeps. The draft
cites those sections where an entry REPLACES what they describe.
