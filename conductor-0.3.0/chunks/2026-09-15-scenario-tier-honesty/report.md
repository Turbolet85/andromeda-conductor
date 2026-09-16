# Report — 2026-09-15-scenario-tier-honesty

**Chunk:** Scenario tier honesty — every committed scenario's declared SLO tier holds its own summed phase
duration or states why not, the three situations kept distinct
**Date:** 2026-09-16
**Commits:** `fec5155` chore(setup-project): absorb the code-graph template delta · `dc757b3`
feat(2026-09-15-remaining-structurally-dead-declarations-retired) — both PRIOR to this chunk; this chunk's own
commit is P7's.

## Changes (structured — detectors read this)

- **Files:** 11 modified, all `scenarios/*.toml` — `ack-cooldown` · `auto-resolve-idle-window` ·
  `constellation-severity-live-wiring` · `findings-counter-refresh` · `halo-hue-encoding` ·
  `incident-auto-resolution` · `report-render-surface` · `service-constellation-discovery` ·
  `severity-tier-autonomous` · `severity-tier-suggested` · `threshold-hot-reload`. 1 new:
  `conductor-0.3.0/chunks/2026-09-15-scenario-tier-honesty/evidence/tier-ledger.md`.
  Basis: `git diff --stat -- scenarios/` → 11 files, +40/−17.
- **Symbols / APIs:** none. No Rust file changed (`git status --short` carries no `.rs` path); `SloTier`, its
  three variants and `deadline_ms()` are untouched, as are `check_budgets` and `evaluate_slo`.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none. `Cargo.lock` byte-unchanged; `cargo audit` scanned **562 crate dependencies**, the
  same figure the prior chunk recorded.
- **Schema / config:** no schema change. **Config DATA moved:** the `slo_tier` value of 8 scenarios was
  re-declared WITHIN the closed set — 2 to the ceiling (`ack-cooldown` `<20s`→`<90s`,
  `severity-tier-autonomous` `<5s`→`<90s`), 6 to the smallest tier that holds their duration
  (`constellation-severity-live-wiring` `<20s`→`<90s`, `severity-tier-suggested` `<20s`→`<90s`, and
  `findings-counter-refresh` · `report-render-surface` · `service-constellation-discovery` ·
  `threshold-hot-reload` each `<5s`→`<20s`). No `gap_ms`, `seed` or `jitter_ms` value changed — basis: every
  added/removed line in `git diff -U0 -- scenarios/` is a `slo_tier` declaration or a comment (probe returned
  no other line). All 36 scenarios re-parse under `tomllib`.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** **none — verified.** No master pairs any of the 8 re-tiered scenarios with a
  tier literal — basis:
  `grep -nE '(<the 8 names>)' .andromeda/*.md | grep -E '<5s|<20s|<90s'` → 3 hits, all outside the seven
  masters (`master-route.md`, an obs-plan **amendments sidecar**, `residuals.md`). The tier tokens the masters
  do carry are the closed-set enumeration itself, which is unchanged.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. The three `[[gate]]` probes are plan-listed measurement commands, not
  shipped gates — nothing was added to `ci.yml`, no Rust test, no `scripts/` file. The re-runnable mechanical
  check is the next route entry's work (`verification-matrix.json#v3-06`).
- **Cross-project / external claims:** none newly asserted. Four corrected comments PRESERVE existing
  delegated-timing claims about Pulse leaves (`metric.findings.counter_refresh_ms`,
  `metric.constellation.discovery_ms`, `metric.constellation.hue_update_ms`, the report-build duration); each
  was measured at a prior chunk against `andromeda-pulse` and none was re-measured or changed here.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** **none in the seven masters — verified.** A comment-marker-stripped,
  wrap-tolerant sweep of all seven for both retired wordings (`MCP round-trip`, `not the SLO budget`) returns
  **0 hits in every master**, so the 2026-09-10 obs-plan amendment that retired the round-trip gloss had already
  propagated across the spec tier. What this chunk measured false was stated in the **committed scenario
  corpus**, not in a spec: 7 files asserted a reason resting on the retired reading (4 the round-trip gloss, 3
  "…is phase timing, not the SLO budget"), all 7 corrected here. Basis for the mechanism they contradict:
  `conductor-run/src/execute.rs:68-69` stamps `journal_emitted_at` before `run_timeline_observed` (line 75)
  runs the whole timeline, lines 120-121 stamp `read_back_observed_at` after read-back returns, line 148
  carries the difference.
- **Expected amendments (from plan):** **none listed.** The plan states "No spec amendment is owed", with a
  conditional instruction (line 298) to record one only if /implement found a master naming a tier for one of
  the 8 on an adjacent line. /implement found none; the sweep above re-confirms it at wrap.
- **Coverage of new surfaces:** none — this chunk introduces no external surface, hot-path operation or UI
  element. Data-only change to committed fixtures.

## Deviations from intent

None. All six Implementation Steps executed as written; the modify-set matched the plan's touchpoint list
exactly (11 + 1), with no file added or dropped.

## Decisions & corrections

- **A cascade gap worth naming: an amendment retired a claim in the spec tier and the COMMITTED DATA kept it
  for six days.** obs-plan retired the "Conductor's own MCP round-trip" gloss of
  `read_back_observed_at − journal_emitted_at` on 2026-09-10; at HEAD on 2026-09-16 four scenario TOMLs still
  stated it, and three more carried its sibling ("phase timing, not the SLO budget"). The cascade sweeps the
  seven masters and the derived doc tier — it does not sweep `scenarios/`, `contracts/` or any other committed
  data the project treats as fixtures. The masters were clean the whole time, which is exactly why nothing
  surfaced it.
- **Sweep hazard, operator-directed at the P5 review: a phrase wrapped across two comment lines is invisible to
  a single-line grep.** `grep -l 'MCP round-trip' scenarios/*.toml` returns **3**; the truth is **4**, because
  `service-constellation-discovery.toml` ends line 18 with "Conductor's own MCP" and resumes line 19 with
  "round-trip, not a Pulse-internal duration". The discriminating form strips the leading `#` per line, joins,
  then collapses whitespace. Pinned into `verification-matrix.json#v3-05`'s acceptance as the REQUIRED form,
  with the single-line grep named there as a known false negative, so a later re-verification or the `v3-06`
  gate cannot reach for it and read a false green.
- **A self-inflicted instance of the same class, at phase P1:** the first corpus measurement keyed phase
  duration on `duration_ms` and returned 0 ms for all 36 scenarios; the field is `gap_ms` (`grep -ohE` key
  census: 99 `gap_ms`, 0 `duration_ms`). Caught by the implausible all-zero column, not by a check.
- **Ledger precision, surfaced by /implement and not silently absorbed:** `v3-05`'s acceptance limb (a) reads
  "every scenario whose summed `gap_ms` exceeds its declared tier, **the nine** ceiling-declaring ones
  included, carries a stated reason". The requirement holds (11 of 11 carry one), and the nine ARE included —
  but post-change the over-tier set is **eleven**, the original nine plus the two Situation-2 files re-declared
  to the ceiling here. True but non-exhaustive; routed to a dated `notes` line at P7.3, acceptance untouched.
- **Style decision:** a corrected reason that outgrew one line uses a wrapped continuation comment indented
  under the `slo_tier` line. These files already use multi-line `#` blocks in headers but not as trailing-comment
  continuations, so the form is an extension of local style; validated by re-parsing all 36 under `tomllib`
  rather than assumed.
- **A true claim was preserved while its gloss was corrected.** The four delegated-timing headers assert both
  that the real budget grades at the harvest tier over Pulse's own leaves (TRUE, per obs-plan §4) and that
  `latency_ms` is an MCP round-trip (retired). Only the second was changed.

## Outcome

Acceptance criteria, each re-asserted against the DIFF rather than the plan's text:

- **MET** — No committed scenario declares a non-ceiling tier its own summed `gap_ms` exceeds: probe returns
  **0**, down from the P5 baseline of **8**. (`verification-matrix.json#v3-05`)
- **MET** — Every declared `slo_tier` is in the closed set and `SloTier` gains no variant: closed-set probe
  **0**; the diff contains no Rust file, so no variant could have been added. (`#v3-05`)
- **MET** — No `gap_ms` / `seed` / `jitter_ms` changed: every added/removed line in the diff is a `slo_tier`
  declaration or a comment.
- **MET** — Three situations separately identified: `evidence/tier-ledger.md` classifies all 36 (9 ceiling-posture
  / 2 was-over-every-tier / 6 was-mis-declared / 19 fits); the nine Situation-1 files keep their ceiling
  declaration as posture, recorded as such and not as defects. (`#v3-05`)
- **MET** — Every surviving stated reason true against HEAD's latency semantics: wrap-tolerant sweep **0**, down
  from **7**; `evidence/tier-ledger.md` carries the per-file disposition. (`#v3-05`)
- **MET** — Every edited scenario still loads clean through `Scenario::from_toml_str` (both validation arms):
  `conductor-core` 326/326 green, which includes the committed-catalog loaders.
- **MET** — Runner portability: `cargo nextest run -p conductor-core` **326 tests, 326 passed** AND
  `cargo test -p conductor-core` green across 4 targets (312 + 6 + 8 + 0).
- **MET** — `cargo nextest run --workspace --profile ci` → **979 tests run, 979 passed** (unchanged from the
  pre-chunk count, as a data-only change implies).
- **MET** — Envelope's eleven keys unchanged in count and name; every stated reason rides the scenario file as a
  TOML comment, adding no envelope extra or span attribute (no Rust or schema file in the diff).
- **MET** — The four delegated-timing scenarios' harvest-tier leaf budgets untouched; the corrected comments
  preserve those claims verbatim in substance.
- **MET** — No render site, sample row or spec gains a per-scenario tier literal (Spec-master edits: none;
  Counts moved: none — verified).
- **MET** — The closed tier set is unchanged, so both a11y envelope reproductions still match; no a11y-plan
  amendment owed.
- **MET** — `cargo audit` and `cargo deny check advisories bans licenses sources` green over an un-drifted
  `Cargo.lock`.

**Gates** (by `run` text, in order, from the /implement gate run — `entries 11 · green 11 · red 0 · recorded 0 ·
timeout 0 · not-run 0`):

| entry | verdict |
|---|---|
| closed-set probe (`python … slo_tier not in T`) | **green** · exit 0 · `last line 0` held |
| tier-honesty probe (`python … non-ceiling exceeded`) | **green** · exit 0 · `last line 0` held (baseline red 8) |
| retired-premise sweep (`python … either gloss`) | **green** · exit 0 · `last line 0` held (baseline red 7) |
| `cargo nextest run -p conductor-core` | **green** · exit 0 · 326/326 |
| `cargo test -p conductor-core` | **green** · exit 0 · 4 targets |
| `cargo nextest run --workspace --profile ci` | **green** · exit 0 · 979/979 |
| `cargo clippy --workspace --all-targets -- -D warnings` | **green** · exit 0 |
| `cargo fmt --all -- --check` | **green** · exit 0 · no output |
| `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` | **green** · exit 0 · `no output` held |
| `cargo audit` | **green** · exit 0 · 1246 advisories, 562 deps |
| `cargo deny check advisories bans licenses sources` | **green** · exit 0 |

No `defer` entry, no `leg` entry, no `recorded` entry, no timeout, no `<id>` substitution. Smoke: **skipped — no
boot-path / UI-surface change** (modify-set is scenario TOMLs plus a chunk evidence file; the plan lists no
`smoke` or `self-verify` entry and states that absence in its own prose). Not a headless skip.

**Outcome basis:** /implement's P4 report as given, in this same session's conversation — the friction-log
fallback was not read and is not this report's basis. No operator directive between implement and this report.

**Process hygiene:** implement's census, re-measured here against the host process list. cargo / rustc
subprocesses started by this run via the gate tool → **terminated**. No conductor / andromeda-pulse /
msedgedriver / tauri process started → `none started`. The census probe was validated on a known-positive
before being trusted: it sees 242 processes and 3 powershell instances while matching 0 of the target set, so
the empty result is a true zero rather than a broken probe.
