# Assertion-class enumeration — every live `[[expected]]` check, classified

**Chunk:** `2026-09-15-structurally-dead-assertion-class-retired` · **SUT:** Pulse HEAD `83d4060`
**Purpose:** `v3-04` asserts a UNIVERSAL — *every* committed scenario's declared checks are satisfiable
against a live Pulse, or retired. A universal is falsified by one member, so this file enumerates the whole
population rather than the four members the route entry names.

## Population (derivation beside every number)

| Quantity | Value | Derivation |
|---|---|---|
| Committed scenarios | 36 | `ls scenarios/*.toml \| wc -l` |
| Declaring ≥1 check (pre-change) | 11 | `grep -l '^\[\[expected\]\]' scenarios/*.toml \| wc -l` |
| `[[expected]]` blocks (pre-change) | 12 | `cat scenarios/*.toml \| grep -c '^\[\[expected\]\]'` |
| Already declare-only (pre-change) | 25 | 36 − 11 |
| `[[expected]]` blocks (post-change) | 8 | same command, after the four retirements |

`high-severity-log-capture` carries two blocks; every other declaring scenario carries one.

## What the graded surface can carry

Every substring check grades `Observation.text`, composed in `conductor-verify/src/extract.rs` from
**(a)** `query_incident_list`'s per-item `status`, `severity` and `title`, and **(b)** each incident's
`retrieve_report` `markdown`. `CountAtLeast` instead grades `evidence_count`, summed from
`retrieve_telemetry_slice.span_refs`.

Three SUT facts bound what any token can match, all measured at HEAD `83d4060`:

1. **`span_refs` is always empty.** `pulse-app/src/inference_runtime.rs:871` sets
   `EvidenceRefs.span_ids: Vec::new()` and is the ONLY non-test writer — every other writer in the Pulse
   tree sits inside a `#[cfg(test)]` module (`interpretation/src/markdown.rs:507` > cfg@376 ·
   `triage/src/digest/assembler.rs:802` > cfg@165 · `digest/retrieval.rs:170` > cfg@150 ·
   `incident/persistence.rs:359` > cfg@335 · `incident/registry.rs:400` > cfg@376).
   `crates/mcp-server/src/tools.rs:430-435` maps it 1:1 into `span_refs`.
2. **Cue names never render in PascalCase.** `crates/triage/src/contract.rs` declares
   `#[serde(rename_all = "snake_case")]` on `CueKind`, and its own test pins
   `ServiceWentSilent` → `"service_went_silent"`. The PascalCase spelling exists only as a Rust identifier.
3. **Under deterministic L4 the discriminating fields are constants.** `pulse-app/src/deterministic_inference.rs`
   pins `"severity": "autonomous"` and `"title": "Deterministic verification incident"`; severity renders
   through Pulse's lowercase mapping (`interpretation/src/markdown.rs:361-364`) as `"error"`.
   `retrieve_report` computes `degraded_mode = parsed_l4.is_none()` over `resolution_summary_text`
   (`mcp-server/src/tools.rs:378`), which the deterministic fixture never populates — so the report carries
   incident-derived fields and section headings, never L4-authored prose. Deterministic L4 is a launch
   condition of every verifiable run (`contracts/pulse-run-contract.toml`), so this is the operative mode.

## The 12 blocks, classified

### Retired by this chunk (4)

| Scenario | Check | Ground |
|---|---|---|
| `constellation-severity-live-wiring` | `CountAtLeast`/`Hard`/`"1"` (P-079) | grades `evidence_count`; fact 1 ⇒ `0 >= 1` false in every world |
| `findings-counter-refresh` | `CountAtLeast`/`Hard`/`"3"` (P-045) | same; graded the evidence count, never the findings count it was authored to assert |
| `pulse-run-contract` | `CountAtLeast`/`Hard`/`"1"` (P-073) | same; asserted incident formation but graded the evidence vector |
| `cross-incident-recurrence` | `Contains`/`Hard`/`"Previously seen"` (P-036) | **corrected ground** — the token IS emitted (`interpretation/src/markdown.rs:221` pushes `"## Previously Seen"`) and DOES reach the graded text (`extract.rs` pushes report markdown into `Observation.text`); it differs in CASE against a case-sensitive `Contains` (`conductor-verify/src/slo.rs`). The 2026-09-10 live round's "no producer emits the token" attribution is disproved. |

### Survivors that are SATISFIABLE (2)

| Scenario | Check | Producer |
|---|---|---|
| `root-span-error-scope` | `Contains`/`CalibrationRegion`/`"error"` | the lowercase severity label, rendered per fact 3 |
| `span-status-error-detection` | `Contains`/`Hard`/`"error"` | same |

**Qualification, recorded not smoothed:** both match only the severity label, which fact 3 pins to a
constant under deterministic L4. They are satisfiable — a token that can appear does appear — but what they
discriminate is the fixture's severity, not the scenario's own stimulus. That is a weaker defect than
structural death and is NOT this chunk's to resolve; it is recorded so the audit gate (`v3-06`) can decide
whether "satisfiable" is the right bar.

### Survivors that are STRUCTURALLY DEAD (6) — found by this enumeration, NOT retired here

| Scenario | Check | Ground | Failure direction |
|---|---|---|---|
| `activity-floor` | `Absent`/`Hard`/`"ServiceWentSilent"` | fact 2 — PascalCase never rendered | **vacuous PASS** |
| `service-went-silent` | `Contains`/`Hard`/`"ServiceWentSilent"` | fact 2 | permanent FAIL |
| `high-severity-log-capture` | `Contains`/`Hard`/`"ERROR"` | fact 3 — severity renders lowercase `"error"` | permanent FAIL |
| `high-severity-log-capture` | `Absent`/`Hard`/`"WARN"` | fact 3 — uppercase never rendered | **vacuous PASS** |
| `exception-event-capture` | `Contains`/`Hard`/`"exception"` | not in status/severity/title; `git grep -c --fixed-strings exception -- crates/interpretation/src/markdown.rs` → 0 | permanent FAIL |
| `threshold-hot-reload` | `Absent`/`Hard`/`"RetroactiveReeval"` | `git grep -c --fixed-strings RetroactiveReeval -- '*.rs'` → **0 across the whole Pulse tree** | **vacuous PASS** |

Three of the six fail in the VACUOUS-PASS direction, which test-plan §6 records as the same defect class as
the always-failing `Contains` and the more dangerous of the two — nothing ever fails to alert you.

## Consequence for `v3-04`

Retiring the four members named by the route entry does **not** achieve the requirement's universal: after
this chunk, **6 committed `[[expected]]` blocks across 5 scenarios still declare checks that cannot pass**.
This chunk did not retire them — the class was ratified as the four (route entry · `v3-04` notes ·
the operator's P4 fork answer), and widening to 5 more scenarios plus their pinning tests is a re-plan, not
an autonomous in-flight expansion.

Surfaced at implement P2 as a spec↔reality gap. The enumeration is the evidence; the disposition is the
operator's.
