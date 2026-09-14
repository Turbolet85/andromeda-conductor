# Report — 2026-09-13-p-025-measurement-contract-for-pulse

**Chunk:** P-025 measurement contract for Pulse — which Pulse-emitted observable, at what resolution, over what window, and what constitutes a hard grade
**Date:** 2026-09-14
**Commits:** none since `last_wrap` (2026-09-13T21:09:03Z) — this wrap authors the chunk's first commit

## Changes (structured — detectors read this)

- **Files:**
  - `contracts/pulse-p025-measurement-contract.md` — NEW, 168 lines (`wc -l`)
  - `crates/conductor-run/tests/delegated_timing_harvest.rs` — +11/−3, doc comment only (`git diff --stat`); basis for "doc only": every changed line is a `//!` line, measured `git diff … | grep -vE '^[+-][+-]' | grep -vcE '^[+-]//!'` → **0** non-doc changed lines
  - `scenarios/halo-hue-encoding.toml` — +4, comment lines only; TOML re-parsed after the edit (`tomllib`): name `halo-hue-encoding`, 2 phases, 1 checklist, 0 expected — structure unchanged
  - `conductor-0.3.0/verification-matrix.json` — `v3-07` claimed at phase P5, `status: implemented` + `ref` at implement P2
- **Symbols / APIs:** **none.** No public fn, IPC method, endpoint, export, port, socket or env handle added or changed. The new `contracts/` artifact has **NO Rust reader** — deliberate, and the fact the registry detector needs: the other four `contracts/` members are runtime-read and bounds-checked at load, this one is addressed outward and is read by nothing in-tree.
- **Crates / modules:** none.
- **Dependencies:** **none** — `git diff --exit-code -- Cargo.lock` exit 0, no output (gate 7). `cargo audit`: 1245 advisories · 562 crate dependencies · 7 allowed warnings (6 unmaintained + 1 unsound); `cargo deny check advisories bans licenses sources` all four ok.
- **Schema / config:** one new committed SUT-facing document, `contracts/pulse-p025-measurement-contract.md` — the **fifth** `contracts/` member and the **first with no Rust reader**. No migration, no config key, no scenario key: the scenario edit is comment-only and the serde/garde load path is untouched.
- **Spec-master edits:** none at P1 — the two expected amendments are applied at P2.
- **Counts / qualifiers moved:** **`contracts/` member count 4 → 5.** Basis: the directory-tree comment at `architecture.md:233` enumerates four manifests by name, and §Occupied Resources → On-disk artifacts enumerates them as four bullets at `:175`–`:178` (`grep -cF 'contracts/pulse-'` → 5 hits, of which `:176`/`:177`/`:178` are the three `pulse-*` bullets; `:175` is the MCP contract manifest). No other derived value moved.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none — no `agent-run` script, xtask verb, CI step or status/verdict shape changed.
- **Cross-project / external claims:** ground truth in the **Pulse** repo (`andromeda-pulse`), read read-only at HEAD **`83d4060`** (branch `chore/migrate-pulse-to-v3`), re-verified at phase P3 and extended at the P5 review:
  - the hue observable is an IPC resolver — the frontend computes the duration and the backend validates-and-logs it (`crates/ui-bridge/src/telemetry.rs`); the leaf's allowlisted field set is exactly `duration_ms` + `severity_tier`, asserted by Pulse's own test (`pulse-app/tests/unit_observability_allowlist_delegated_timing.rs`)
  - the canvas computes `nowMs − last_seen_unix_nano / 1e6` and keeps the MAXIMUM across changed dots (`pulse-app/ui/src/widget/ConstellationCanvas.tsx`)
  - `last_seen_unix_nano` has five writers, none on the ingest path; the steady-state one is the 15 s tick's refresh gated on `current_quiet_duration_seconds == 0` (`crates/triage/src/lifecycle/registry.rs`), integer-truncated to whole seconds (`crates/triage/src/baseline/activity_floor.rs`)
  - **NEW measurement, this chunk:** an incident's `priority_tier` is **immutable after opening**. Basis: `grep -rnE '\.priority_tier\s*=|priority_tier:\s*[a-z]' --include=*.rs crates/ pulse-app/src/` → **8 hits · 0 changed · 8 no-change** (every one a construction site or DTO projection; the sole assignment is `services_router.rs`'s write to the DERIVED `ServiceListItem` field), and no persistence UPDATE touches the column (`crates/corpus/src/contract.rs` incident updates set `status` / `updated_unix_nano` / `resolved_unix_nano` / `payload` / `read_unix_nano` only).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** **`obs-plan.md:349`** states "a SUT change that stamps `last_seen` from span arrival (or copies the activity floor's `last_observed_unix_nanos`) would lift it". **Measured false this chunk:** both candidates address the **staleness term only**. With `last_seen` ingest-stamped, a service emitting at 2/s reports ~500 ms and the 2 000 ms budget starts PASSING while the emitted quantity remains *the age of the newest span at paint time* — a green grade over the wrong quantity. The slowest-wins and quantization terms survive either change. Site search: `grep -cF 'would lift it'` → **1** in `obs-plan.md`, **0** in each of the other six masters (architecture · security-plan · design-system · layout-templates · test-plan · a11y-plan). Disposition: routed to Expected amendment 2 below.
- **Expected amendments (from plan):**
  - **carried** — `architecture.md` §Occupied Resources + §Infrastructure Patterns → Directory structure: register the fifth `contracts/` artifact and move the tree comment four → five. Motivating fact carried in **Counts / qualifiers moved** and **Schema / config** above. Sites: `grep -nE '^\s*(├|└|│).*contracts' .andromeda/architecture.md` → **1 hit, `:233`** (the tree comment naming four manifests); §Occupied Resources On-disk artifacts bullets at `:175`–`:178` (4 bullets, a fifth owed). Owner named on a hit basis — `architecture.md` only.
  - **carried** — `obs-plan.md` §4 Known-residual classification path → Delegated-timing family: add the contract pointer and narrow the "would lift it" clause to the measured premise correction. Motivating fact carried in **Spec claims disproved by measurement** above. Site: `grep -cF 'would lift it'` → **1 hit in `obs-plan.md` (`:349`), 0 in the other six masters** — so `obs-plan.md` is the sole owner and no other master is named.
- **Coverage of new surfaces:** the chunk introduces no external surface, hot-path operation or UI element. The one new artifact is an outward-addressed document with no reader, no ingest channel and no render site.
  - `contracts/pulse-p025-measurement-contract.md` → validation n/a (no reader, so no boundary) · instrumentation n/a · PII n/a · tests — the by-construction probes (six-heading enumeration + word-anchored host-path scan) · a11y n/a · tokens n/a

## Deviations from intent

1. **Step 9 shipped three comment lines where the plan said "a one-line pointer"** (`scenarios/halo-hue-encoding.toml`). Justification: the two lines immediately above the insertion point are the very sentence this chunk measured false — they name copying `last_observed_unix_nanos` or reading span arrival as what would lift the bound. A bare pointer would have sat directly beneath an uncorrected contradiction of the document it points at. The added lines state the insufficiency and then point. Comment-only; TOML structure re-verified unchanged.

## Decisions & corrections

- **Operator review at phase P5 supplied the start instant's backend source**, which changed the ask materially: `pulse-app/src/services_router.rs` already holds the full `Incident` records where it computes the tier, so the contract asks Pulse to **expose** an instant it has rather than **mint** one. All three dictated coordinates were re-verified against the SUT before use (the standing cross-project citation rule) and all three read exactly as given.
- **The review's open question dissolved rather than needing a convention.** It asked which instant applies when the tier changes by escalation vs by an incident opening; measurement showed **escalation is not a state this SUT can reach**, so the rule closes at two cases (rise → `opened_at_unix_nano`, fall → `transitioned_at_unix_nano`).
- **One number had three renderings across committed artifacts and the plan added the only wrong one.** The second 2026-08-21 reading is raw `36704.983642578125` in the fixture; the harvest module doc and `v3-07`'s `observed_gap` both carry the rounded `36705`; the plan and scope had carried a value truncated at the decimal, which is not a round. All artifacts now agree on the rounded form, with the raw value cited once at its fixture anchor.
- **Correction made mid-implement:** the advisory-database currency probe first read a copy under the user profile, but `CARGO_HOME` points elsewhere on this host, so that reading described a directory `cargo audit` does not read. Re-derived from the path named in the tool's own output; the real copy is also clean (0 porcelain lines). The gate was green, so nothing rested on it.
- **Sweep hazard found this chunk (for curation):** the word-anchored host-path pattern `\b[A-Za-z]:[\\/]` is satisfied by the gate command's OWN alternation literals when the pattern text itself is scanned — sweeping the chunk artifacts returned hits at `plan.md` that were all `/home/` · `/Users/` · `%APPDATA%` inside the quoted gate string, with **zero** drive-letter runs. Reading the hits, not refining the pattern, is what settled it.

## Outcome

**Acceptance criteria — 13, each re-asserted against the diff:**

| # | criterion | verdict against the diff |
|---|---|---|
| 1 | (core) contract exists, four elements as named sections | **MET** — six-heading enumeration returns 6 |
| 2 | (core) start instant is a Pulse-produced value; journal-relative terms named as excluded | **MET** — `latency_ms` / `budget_ms` / `effective_deadline_ms` each present once, as excluded subjects |
| 3 | (obs) observable named by exact leaf + literal field name + allowlist-admission requirement | **MET** — leaf, `duration_ms`, `severity_tier`, allowlist requirement all present |
| 4 | (obs) no Conductor span, envelope extra or eighth critical path | **MET** — diff adds no `.rs` logic, no span, no envelope field |
| 5 | (tests) workspace nextest green; the two P-025 pins pass unchanged | **MET** — 909/909; `2 tests run: 2 passed` |
| 6 | (tests) hard-grade predicate harvest-tier-assertable, breach stated, operator-gated live claim | **MET** — §The hard grade states all three |
| 7 | (security) zero absolute host paths / internal struct names in the artifact | **MET** — word-anchored probe exit 1, no output |
| 8 | (security) no new ingest channel, no direct Pulse file/DB access prescribed | **MET** — contract prescribes an emission change only |
| 9 | (arch) provenance on the artifact's face — HEAD `83d4060`, capture date, measurement-not-transcription | **MET** — the header block carries all three |
| 10 | (arch) no `CONDUCTOR_*` handle, no Rust reader, lockfile clean | **MET** — gate 7 exit 0 |
| 11 | (design) states from the closed six-member set; `v3-08` ungraded status as `Blocked` not `Fail`/`KnownResidual` | **MET** — only `Blocked` · `Fail` · `KnownResidual` · `CalibrationRegion` named, all in the set, and the Blocked/KnownResidual distinction is drawn explicitly |
| 12 | (layouts) no new lamp, bracket label, `ReportState`, table column, cli verb or flag | **MET** — no cli or render surface in the diff |
| 13 | (lint) clippy workspace clean | **MET** — exit 0 |

**Gates — 9 entries, in order, all green at implement; re-run at P7.1:**

1. `cargo nextest run --workspace --profile ci` — exit 0 · `909 tests run: 909 passed`
2. `cargo nextest run -p conductor-run --profile ci -E "test(p025)"` — exit 0 · atom `contains 2 tests run: 2 passed` held
3. `cargo test --workspace --doc` — exit 0
4. `cargo clippy --workspace --all-targets -- -D warnings` — exit 0
5. six-heading structure probe — exit 0 · `last line 6` held
6. word-anchored host-path probe — exit 1 · `no output` held
7. `git diff --exit-code -- Cargo.lock` — exit 0 · `no output` held
8. `cargo deny check advisories bans licenses sources` — exit 0
9. `cargo audit` — exit 0

No `defer` entry, none deferred under the source-delta rule (the chunk carries `.rs` delta plus a committed TOML a test reads, so every changed-surface gate was mandatory). No `leg` entry. **Smoke: skipped — no boot-path / UI-surface change**; the plan lists neither a `smoke` nor a `self-verify` entry and states both absences in its prose.

**Outcome basis:** implement's P4 report as given, plus this wrap's own re-derivation of the amendment site searches (the grep counts above were run at P1, not inherited). No operator directive changed the outcome between implement and this report; the wrap directive's four items are process work, not outcome claims.

**Process hygiene:** implement P4's census, re-measured here — `cargo` · `cargo-nextest` · `conductor-tauri` · `msedgedriver` · `andromeda-pulse-mcp` all report no matching tasks, and `netstat` shows no `:4317` listener. Every process this chunk's runs started was short-lived and self-terminating: `terminated`. `none started` for every external process.
