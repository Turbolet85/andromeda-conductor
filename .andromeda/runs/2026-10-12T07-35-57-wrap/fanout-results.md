# Fan-out results — 2026-10-12-panel-shaped-types-retired (wrap, resumed window)

Path check: before `tree 80f868eab76b4ed0bcefc51b13b29b588fc04a3d index ca558d33dba0ee64be67117cc8273edf974d79c5 head acad00d067e4feec6f35eacfed705124d6445a0d` · after `tree bb0997bc269d34804514eba72d98d16cfce335d6`, index and head unchanged · `git diff-tree` printed exactly the seven `A …/fanout-{doc}.md` lines. Every receipt's count equals its file's item count; no file needed a fix (no twin). Detector prompts: 28 doc-names over 22 drift-base entries (arch 5 · security 4 · design 3 · layout 3 · tests 5 · obs 5 · a11y 3).

architecture: 5 proposals · fanout-architecture.md
  note: no D-arch-collision hit (the Ctrl-C handler is an in-process signal registration, no bind); no D-platform-claim hit; §Stack's Terminal output rendering row enumerates comfy-table's uses as results + capability-coverage tables and does not name the new catalogue table (left to the orchestrator); standing rows `:50`, `:110`, `:54` confirmed true.
security-plan: 7 proposals · fanout-security-plan.md
  note: D-security-subprocess, D-security-deps, D-platform-claim no hit. Three observations: (1) the `#[cfg(unix)]` SIGINT test's fixed-argv `kill -INT <own pid>` from a cargo test binary — §Code Patterns' categorical clause covers it; (2) the checklist bound is length-only, so "one line per half" is a rendering claim, not one validation enforces; (3) the report does not state `conductor scenarios`' capability-manifest read route.
design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens / D-design-derived-count / D-platform-claim hold; the plan's expected amendment for §Component Patterns 3 is open (left to the orchestrator); the Found-and-not-owned width wording is also stated at design-system.md:158 and in §Spacing, §Platform-Specific Notes and §Per-Surface Bans (cli).
layout-templates: 5 proposals · fanout-layout-templates.md
  note: D-layout-derived-count and D-platform-claim no hit; the Found-and-not-owned item (`:116`, plus `:80`'s "width detected dynamically") is the orchestrator's to card.
test-plan: 6 proposals · fanout-test-plan.md
  note: parity sites :77, :89, :288 still true (no dependents); D-tests-framework and D-platform-claim no hit; the moved counts appear nowhere in test-plan.
obs-plan: 3 proposals · fanout-obs-plan.md
  note: no obs detector covers the plan's obs expected amendments (the Tauri sink retirement at obs-plan.md:75, :99, :396, the two key files and the two toml labels; the parity arm's naming at :126, :270, plus §1's Correlation bullet and cross-surface-parity Telemetry-trigger row); D-obs-stack, D-obs-redaction, D-obs-ci-gates, D-platform-claim no hit.
a11y-plan: 0 proposals · fanout-a11y-plan.md
  note: the new surfaces are cli output, not-assertable per a11y-plan §1/§11; no schema change; a11y-plan and its nine key files hold 0 hits for the swept tokens.

## Validate — dispositions

Opening rule (re-derivation tell): 0 proposals rejected — every `basis` is a report line, a master line, or a coordinate the report's New text section lists (`scenario.rs:80-81`, `catalog.rs:8-17`, `cli.rs:22-23`, `render.rs:113-115` / `:117-120` / `:98-111` / `:146-150`, `suite.rs:79-99` / `:134-157`, `report.rs:21-24`, `pause.rs:38-40`, `obs.rs:59-60`).

Checks: 1 playbook — every applied proposal lands under the plan's P5-approved `Expected amendments (wrap)` entry that names the change itself (recorded direction, `inputs#I4`/`inputs#I5`), or under `Accurate this-chunk addition` (routine) where it goes beyond the entry; the escalate-severity D-security-input proposals retire a handle with no reader or record value-free CLI surfaces — narrowings and records, none of `Boundary widening`'s crossings (no write on a read-only command, no new subprocess or IPC crossing, no new value reaching an argv, a shell or a path), and no locked decision is reversed. The two P4 answers (`inputs#I3`: the verb word `scenarios`, a stopped suite exiting 130) were read against `Boundary widening`: `conductor scenarios` takes no argument and loads through the same fixed `default_path()` → `resolve_under` capability-manifest route and validated `list_scenarios` load as `run`/`suite` (read at `crates/conductor-cli/src/paths.rs` 23-55); the Ctrl-C stop is an in-process OS signal registration carrying no value, no bind, no file; exit 130 is an outcome code — the playbook does not read either as a widening, so no widening halt (directive item 5). · 2 cross-contradiction: none in opposing directions. · 3 intent-consistency: scope record clean (changed 26 · listed 26 · recorded 0); deviations carry the plan's or the operator's word. · 4 absence claims: "no code reads it" / "named only by operator_pause_harvest.rs" rest on the report's gate readings (plan census exits 1). · 5 expected amendments: every entry matched or raised below. · 6 disproved claims: the report's one entry (v4-01 `observed_gap` "two webview fixtures with three tests" → measured 4 + 2 + 1) → routed to the ledger channel: a dated PREMISE-CORRECTION note on v4-01 at P7.3 — not acceptance text (the acceptance names no count), carried to the card for the operator's word since no plan or directive directs the note.

### architecture
- architecture#1 D-arch-resources · §Occupied Resources (on-disk artifacts) — `logs/conductor-tauri.jsonl` row → apply (check 1: expected-amendments entry, recorded direction; directive item 4)
- architecture#2 D-arch-resources · §Occupied Resources (environment variables) — `CONDUCTOR_E2E_SEED_DIR` row → apply (check 1: expected-amendments entry)
- architecture#3 D-arch-resources · §Occupied Resources (crate names) — `conductor-run` row restated as the suite driver → apply (check 1: expected-amendments entry), text re-derived from the report
- architecture#4 D-arch-resources · §Infrastructure Patterns → Directory structure (key file :23) · dependent-of D-arch-resources → apply (check 1: same entry, the claim's keyed restatement)
- architecture#5 D-arch-decisions · §Conventions (config) — `MAX_CHECKLIST_TEXT` reason → apply (check 1: expected-amendments entry); applied text claims a length bound only (security observation 2)
- architecture#6 (raised by main) · §Stack and Technologies → Terminal output rendering row — comfy-table's uses gain the `conductor scenarios` catalogue table → apply (check 1: `Accurate this-chunk addition`, routine; the detector's note)

### security-plan
- security-plan#1 D-security-input · §Input Validation — `CONDUCTOR_E2E_SEED_DIR` row (:112) → apply (check 1: expected-amendments entry; a narrowing, no reversal)
- security-plan#2 D-security-input · §Threat Model Summary entry points (:51) · dependent-of → apply (same)
- security-plan#3 D-security-input · §Bootstrap phases → input-validation-library-install (:209) · dependent-of → apply (same)
- security-plan#4 D-security-input · §Secret Management → Development (:242) · dependent-of → apply (same)
- security-plan#5 D-security-input · §Security Anti-Patterns → Input (:314) · dependent-of → apply (same)
- security-plan#6 D-security-input · §Input Validation — `[[checklist]]` reason (:107) → apply (check 1: expected-amendments entry); re-derived as a length bound — no single-line claim (observation 2)
- security-plan#7 D-security-input · §Input Validation — CLI arguments row (:116) → apply (check 1: expected-amendments entry; not a widening, above)
- observation (1) kill-spawn: no claim false — §Code Patterns' clause is categorical ("A spawn from a cargo TEST binary sits outside rule (b)") → no change · observation (3): the route is the existing fixed `default_path()` → `resolve_under` one (paths.rs read above) → no change

### design-system
- design-system#1 (raised by main, check 5) · §Surface: cli → Component Patterns 3 — the `conductor scenarios` catalogue listing as a static table beside `coverage` → apply (expected-amendments entry)

### layout-templates
- layout-templates#1 D-layout-surface · §Primary screens — `conductor scenarios` bullet → apply (expected-amendments entry)
- layout-templates#2 D-layout-surface · §Primary screens — `suite`'s stop line + exit 130 → apply (expected-amendments entry)
- layout-templates#3 D-layout-surface · §Primary screens — `report` prints the stored caption → apply (expected-amendments entry)
- layout-templates#4 D-layout-surface · §Component — Hero (hold, CLI mirror) — checklist lines → apply (expected-amendments entry: "the hold's indented checklist lines")
- layout-templates#5 D-layout-surface · §Component — Primary content block 1, table-shape paragraph · dependent-of → apply (`Accurate this-chunk addition`: the two-shape enumeration under-runs the third shape)

### test-plan
- test-plan#1 D-tests-obs-harness · §3 → Log format (key file :6) → apply (expected-amendments entry; lockstep with obs-plan#4)
- test-plan#2 D-tests-derived-count · §7 — fixture tree set (:316) → apply (expected-amendments entry)
- test-plan#3 D-tests-derived-count · §7 — round-trip enumeration (:317) · dependent-of → apply (expected-amendments entry, "both sites")
- test-plan#4 D-tests-coverage · §1 Coverage triggers Vector 1 (:83) → apply (expected-amendments entry)
- test-plan#5 D-tests-coverage · §1 Untestable zones — the Ctrl-C stop's two operator-accepted limits → apply (`Accurate this-chunk addition`: both limits are this chunk's and carried in the report by the operator's word, `inputs#I4` (3) / `inputs#I5`)
- test-plan#6 D-tests-coverage · §5 Cross-surface parity (:204) → apply (expected-amendments entry: the naming)

### obs-plan
- obs-plan#1 D-obs-instrumentation · §6 DB READ bullet (:406) → apply (`Accurate this-chunk addition`: `conductor report` is `read_envelope`'s first production caller and holds no span; the master states "INSIDE the caller's span" unqualified — read at :406)
- obs-plan#2 D-obs-instrumentation · §1 runs.db row (:40) · dependent-of → apply (same claim, read at :40)
- obs-plan#3 D-obs-instrumentation · §4 conductor-report row (:192) · dependent-of → apply (same claim, read at :192)
- obs-plan#4 (raised by main, check 5) · §3 → Logging stack / Log file location keys + §3/§6 body sites (:75, :99, :396) + the two `obs-plan-contracts.toml` labels — the Tauri backend sink retires → apply (expected-amendments entry; directive item 4; lockstep with test-plan#1)
- obs-plan#5 (raised by main, check 5) · §4 In-process ↔ headless parity (:126, :270) + §1 Correlation bullet / parity Telemetry-trigger row where they restate the arm — the arm's naming → apply (expected-amendments entry)

### a11y-plan
- (no proposals; nothing raised)

## Card items (not proposals — the operator's disposition)
- C1 Found and not owned (directive item 8): the width wording "detected dynamically from the terminal" for the results and coverage tables (layout-templates :116, :80; design-system :158 and the sites its note names) while the code uses `ContentArrangement::Disabled`. Lean: re-word at this wrap.
- C2 Disproved claim (check 6): a dated PREMISE-CORRECTION note on v4-01 at P7.3. Lean: write it.

## Escalate — resolved with the operator (2026-10-12, this window)
- C1 → **re-word now**, the operator's word: "masters follow the code: content-sized, never wrapped; the listing is the one terminal-sized table, said so." Raised as design-system#2 (§Spacing :85 · Component Patterns 3 :158 · Platform-Specific Notes :173 · Per-Surface Bans :200) and layout-templates#6 (suite mockup :80, :83 · Primary content block 1 :116) → apply.
- C2 → **write the note**, the operator's word ("Write the note (Recommended)") → `matrix.py note` on v4-01 at P7.3; check 6's one entry DISPOSED there.
