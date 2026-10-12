# Scope — 2026-10-12-panel-shaped-types-retired

**Working entry (`working-route.md:13`):** Panel-shaped types retired — channel payload, desktop envelope read,
picker summary, dialog-row limit, window log sink, webview fixtures leave; command line reaches every function
(v4-01, v4-09)

**Why it exists:** intent R1 (`intent.md:85-93`, ruled by the founder 2026-10-09). R1's OBSERVED lists, outside the
removed crate, "types shaped for the panel: the run-event channel payload, an envelope read 'for the desktop report',
a catalogue summary 'for the picker', a text limit sized for one dialog row, a Tauri log sink, two webview fixtures
with three tests". Its EXPECT: "no type, limit, fixture or sink exists for a panel … every function Conductor has is
reachable from the command line". The first chunk (`2026-10-11-conductors-window-retired`) removed the crate and
left these, naming this entry as their owner (its `research.md` §What the list missed, "Stays").

**Take-up word:** the founder's word to resume came 2026-10-12 through overseer1 («deployed, go»), relayed with this
take-up (`inputs#I1`); the operator's phase directive is `inputs#I2`. Stop at P5 for the operator's `yes`.

**Matrix target:** the entry names `v4-01` and `v4-09`. `v4-01`'s notes (2026-10-11 wrap P7.3) say what is still
open on it: the four functions with no command-line entry and the panel-shaped types, limit, fixtures and log sink,
"both owned by `Panel-shaped types retired`"; its masters' plain-words clause is noted as done at that wrap. `v4-09`
is also listed by `:47`, `:49`, `:107` and `:113`. Whether this chunk can make either fully verifiable is decided at
P4 and shown at the review, never assumed here.

## What this chunk removes or re-shapes (the entry's list)

Each is measured at P3; an item with another consumer is named with that consumer.

1. **The run-event channel payload** — `RunEvent` / `RunStage` and the `drive_run` path that emits them
   (`crates/conductor-run/src/drive.rs`). Verified (research.md §Graph impact): `drive_run` has no production
   caller and carries the abort flag that is the only graceful mid-run stop (CARRY item 2); `conductor suite`
   runs its own loop with no abort poll (`commands/suite.rs:31-36`). So the channel payload (`RunEvent` /
   `RunStage`, `Serialize` for the Tauri `Channel` alone) leaves, and `drive_run` stays re-shaped as the suite's
   driver.
2. **The desktop envelope read** — `read_envelope` (`crates/conductor-run/src/envelope.rs`, documented as existing
   because the window crate had no `conductor-report` edge). Verified: `conductor-cli` already declares both
   `conductor-report` and `conductor-run`, and `report` never reads `run_envelope` (`commands/report.rs:17-20`);
   CARRY item 3 needs exactly this reader, so its desktop framing leaves and it gains its CLI caller.
3. **The picker summary** — `ScenarioSummary`, `validate_selection`, `list_scenarios`
   (`crates/conductor-core/src/scenario_catalog.rs`, "for the desktop picker"). Verified: `list_scenarios` /
   `ScenarioSummary` have no production caller and are the read CARRY item 1 needs; `SUITE_SELECTION` and
   `validate_selection` have no caller outside their module (they guarded the Tauri `start_run` alone) and leave.
4. **The dialog-row limit** — `MAX_CHECKLIST_TEXT = 200` (`crates/conductor-core/src/scenario.rs:93-95`, justified
   as "each renders as one dialog row"). Verified: the bound is the garde `length` rule on both checklist halves
   (`scenario.rs`, two references) — a trust-boundary rule at load (`.claude/rules/security.md` §Input validation),
   so a bound must remain; what leaves is its panel justification, and whether the number changes is P4's.
5. **The window log sink** — the Tauri backend's `logs/conductor-tauri.jsonl`. Verified: in code it survives only
   as doc text (`crates/conductor-core/src/obs.rs:59-60`; `ObsSink::File` is also the cli's `--agent-mode` sink and
   stays); the masters' rows marked "no writer since 2026-10-11" (obs-plan §3, the log-format key, architecture's
   `logs/conductor-tauri.jsonl`) are held until this chunk (handoff Notes) and retire through the wrap's
   amendments, never a phase edit. `operator_pause_harvest.rs:229` still reads that path; it is
   `Window grading retired`'s (`:77`).
6. **The webview fixtures** — `crates/conductor-run/tests/envelope_fixture.rs` and `lamps_fixture.rs`, with
   `CONDUCTOR_E2E_SEED_DIR`'s setter-less test reader (`envelope_fixture.rs:32`, `:188`). Intent counts "two webview
   fixtures with three tests".
   - [premise-corrected: the files hold 4 + 2 tests (`git grep -n '#\[test\]'` over the two files), and their two
     fixture files `tests/fixtures/over-envelope.toml` and `tests/fixtures/lamps-journal.jsonl` have a third
     consumer, `journal_conformance.rs:157-164` (`the_committed_fixture_journal_conforms`)] Both fixture files and
     that one test leave with the two webview fixtures; `journal_conformance.rs`'s production-writer test stays and
     keeps the target's conformance subject.
7. **Doc comments naming the window for these types** — `conductor-core` / `conductor-run` comments that describe
   these types as the window's (first chunk's research: "this entry for its own types; the rest `No-survivor
   sweep` (`:136`)"), and `crates/conductor-cli/tests/cross_surface_parity.rs`'s "Tauri arm", which drives
   `drive_run`.

## The command line reaches every function (the entry's second half + its CARRY)

CARRY, folded verbatim: "four functions lost their only entry point when the window left
(2026-10-11-conductors-window-retired, source: its research.md §Function reach) — the catalogue listing (no listing
verb exists; the command word is new, so the operator names it), a graceful mid-run stop (`drive_run`'s abort flag
has no CLI caller), a past run's load-envelope standing (`report` prints the results table only) and a hold's
`[[checklist]]` items (the CLI hold prints scenario · P-ID · step · prompt); also this entry's: the Tauri backend
log-sink rows the masters keep marked "no writer since 2026-10-11", and `CONDUCTOR_E2E_SEED_DIR`'s setter-less test
reader with its two fixtures".

8. **The catalogue listing** gets a command-line entry. Verified: `Commands` declares no listing verb
   (`crates/conductor-cli/src/cli.rs`), so it is a new verb, and the operator names it — P4 proposes the word with
   its alternatives and stops (`inputs#I2` §3).
9. **A graceful mid-run stop** gets a command-line entry, keeping what finished. Verified: only `suite` has
   scenarios to stop between (`run` drives one); its loop polls nothing and Ctrl-C kills it before `persist`, so
   nothing finished is kept today. A new flag or signal handling on an existing verb is proposed in the plan and
   shown at P5 by name (`inputs#I2` §3).
10. **A past run's load-envelope standing** gets a command-line entry. Verified on `report`, which prints the
    results table only (`commands/report.rs:17-20`); whether it needs a flag is P4's, shown at P5 by name.
11. **A hold's `[[checklist]]` items** get a command-line entry. Verified: `HoldPoint.checklist`
    (`crates/conductor-core/src/pause.rs:71-76`) already reaches `PromptResolver::resolve`
    (`crates/conductor-cli/src/pause.rs:34-37`, the interactive arm `CliResolver::resolve` dispatches to at `:114-119`),
    which prints `hold_line` then calls `confirm`; neither prints the checklist. [premise-corrected at the P5 review,
    `inputs#I4` (4): this bullet first named `CliResolver::resolve` at `:34`, which is `PromptResolver`'s] `Window grading retired` (`:77`) later removes the hold and the
    checklist, so P4 either prints the items at the CLI hold now, or names why the checklist is no longer a function
    Conductor has; nothing is left unreachable (`inputs#I2` §2).

## Boundaries

- What Conductor reads of Pulse stays as it is; nothing is re-pointed at the engine or its door — that is
  `Engine read through the door` (`:19`). A removal that forces a touch of the Pulse-facing path stops at P4 and says
  where (`inputs#I2` §4). Verified: no file in research.md's write set is on the read-back, preflight or emission
  path (`conductor-verify`, `conductor-emit`, `canary.rs`, `execute.rs` untouched).
- A new verb, listening socket, place on disk or credential halts for the operator's word (`inputs#I2` §3). The
  loopback-only trust boundary is unchanged. Verified against `inputs#I2`.
- Ports 4317 / 4318 are shared with Pulse's builder on this host: ask before any run that binds them (`inputs#I2`
  §7). Verified against `inputs#I2`; research plans no leg that binds them.
- If the chunk does not fit, P4 proposes a cut: a first chunk that ends green, and what the second holds
  (`inputs#I2` §6). Verified against `inputs#I2`.
- P5 lists every act that may meet the auto-mode classifier — each deletion of a directory or many files, each CI
  job or gate edit — so they are prepared before implement (`inputs#I2` §5). Verified against `inputs#I2`.
- Not this chunk's: the hold / checklist / window-timing harvest (`Window grading retired`, `:77`), `live_suite.rs`'s
  wdio header (`Log-file harvest and live suite retired`, `:71`), the remaining Tauri doc comments (`No-survivor
  sweep`, `:136`), the masters' restatement (`Records restated`, `:134`), the run-read verdict lines
  (`Run read from the command line`, `:47`).

## CI read at Setup

- `0619048` (the last wrap, the only commit since the last flip): green · `CI#38127777250` completed/success · 1/1
  check (Rust gate) · wall 380 s. No red to disposition.
