# Codebase Research — 2026-10-11-conductors-window-retired

## Scope
- **Depth:** moderate · **Reads:** 24 · **Globs/Greps:** 21 · **Graph queries:** 2 (rust plane, regenerated at
  the first query; trace `.andromeda/runs/2026-10-11T01-52-22-phase/tree-query-2026-10-11-conductors-window-retired.json`)
- **Harness rules consulted:** none — no live leg in this chunk. The gates are the bundled `run` default, cargo
  commands and the supply-chain pair. The `--live` arm is EDITED (its driven a11y step leaves) but never fired: ports
  4317/4318 are shared with Pulse's builder (`inputs#I1` §6).
- **Platform issues consulted:** none — no runner-only bullet. The one red Setup read (`d227e59`) is owned elsewhere
  (scope.md §CI read).
- **External inputs:** `inputs#I1` — the pc overseer's phase directive (closed removal list; Pulse-facing path
  untouched; size cut; boundaries; the P5 card) · `inputs#I2` — the operator's invocation (the founder's word to
  start code, 2026-10-09; stop at P5 for the yes).

## Files inspected
- `Cargo.toml` (full) — member line `:12`; `tauri = "2.10.3"` `:78` and `tauri-build = "2"` `:79` (`git grep -n
  -E '^(tauri|tauri-build)' crates/*/Cargo.toml` → only `crates/conductor-tauri/Cargo.toml:10,16,24`); the `regex`
  comment `:85` says "already locked transitively via urlpattern", which is false once the Tauri tree leaves.
- `crates/conductor-tauri/Cargo.toml`, `src/main.rs` (full), `src/commands.rs` (handler list) — 59 tracked files
  (`git ls-files crates/conductor-tauri | wc -l`), 47 of them under `ui/`. The window's eight IPC handlers are
  registered at `main.rs:22-31`. The self-obs sink helper `tauri_log_path()` lives in `main.rs:44-54`, inside the
  crate.
- `crates/conductor-run/Cargo.toml` `:41` — the `sha2` dev-dependency comment "already locked via
  tauri-codegen/wry" is false once the Tauri tree leaves. `sha2` itself stays reachable (graph read below).
- `deny.toml` (full) — 16 `[advisories] ignore` ids, all under a "Tauri 2 desktop-GUI tree" banner. Nine
  `[licenses] allow` entries, two of them under the same banner (`MPL-2.0`, `Apache-2.0 WITH LLVM-exception`).
- `.github/workflows/ci.yml` (index + `:1-62`, `:99-122`) — three jobs. In `rust`: `actions/setup-node` `:42-47`
  (cache keyed on the ui lockfile) and `Build frontend bundle (ui/dist)` `:49-56` exist only for the window crate's
  compile. The obs conformance, zero-unlogged-panics and run-journal conformance gates and the `agent-latest.jsonl`
  upload all sit in `rust` (`:185-241`) and survive. `frontend` `:243-273` and `a11y` `:274-783` leave whole. The
  comment `:100` names "the EDGEWEBDRIVER comment in the a11y job", which dangles once that job is gone. The dogfood
  step `:112-117` runs `scripts/agent-run.ps1 run` on the Windows runner, so CI measures the PowerShell twin's
  default arm after its edit.
- `scripts/agent-run.sh` (`:1-162`, `:229-352` index) — `UI_DIR` `:24`; `ensure_frontend` `:52-57`, called by `--unit`
  `:295`, `--integration` `:296`, `--e2e` `:304`, the bundled default `:318` and the live suite's driven a11y arm
  `:141`; `assert_a11y_verdict` and `A11Y_EXPECTED_SKIPS` `:253-291`; the `--e2e` arm `:297-316`; the driven a11y arm
  of `live_suite` `:136-143` with its stop-form lines `:152-153`; the selector's usage lines `:350`, `:398` and the
  header `:11`.
- `scripts/agent-run.ps1` (index) — the same members at `:17`, `:46-57`, `:66-103`, `:201-211`, `:222-223`,
  `:343-372`, `:410`, `:453`, `:7`.
- `crates/conductor-cli/src/{cli.rs,main.rs,pause.rs,render.rs,commands/report.rs}` — the verb set and its reach
  (§Function reach below).
- `crates/conductor-run/tests/{envelope_fixture.rs,lamps_fixture.rs,live_suite.rs,operator_pause_harvest.rs}`,
  `crates/conductor-cli/tests/cross_surface_parity.rs` — what outside the crate serves the window (§What the list missed).
- `.gitignore` (`:15-60`) — `:21-23` (`crates/conductor-tauri/ui/logs/` and its comment), `:46-49` (the frontend
  block: `node_modules/`, `crates/conductor-tauri/ui/dist/`), `:51-52` (`crates/conductor-tauri/gen/`); `:19`'s
  comment names `conductor-tauri.jsonl`. `/ui/` `:53-57` is Pulse's fallout and stays.
- `scripts/mutation-roster.toml` (`:1-56`) — no `[[unit]]` or `[[entry]]` for the crate (units: `conductor-cli`,
  `conductor-verify`, `conductor-emit`). `:10-11` quotes a 2026-09-03 tauri tally line as a dated witness of the
  coordinate-free design: history, it stays.
- `crates/conductor-faults/tests/port_occupier.rs` (`:1-63`) and a `git grep 'bind\('` over `crates/*/{src,tests}` —
  every test bind is `:0`. The only reach to `127.0.0.1:4317` in the default suite is the preconditions probe's TCP
  connect (`conductor-core/src/preconditions.rs:102`), which binds nothing. `ss -ltnp` read nothing on
  4317/4318/4444/4445 at 2026-10-11 ~07:50Z.

## Graph impact
- **Dependency graph without the window** — `cargo metadata --format-version 1 --locked --offline`, walked from
  the eight remaining members over every dep kind and platform (scratch script, the method is in the plan's
  evidence step). Reachable: **562 → 270**; **292** packages only `conductor-tauri` reaches; 562 = the lock total.
  No `tauri*`/`wry`/`tao`/`webview2*`/gtk/gdk/`webkit*`/`soup*` name stays reachable.
- **deny.toml against that graph** — every crate the 16 advisory ignores name (`unic-char-range`, `unic-common`,
  `unic-char-property`, `unic-ucd-version`, `unic-ucd-ident`, the ten gtk-rs GTK3 crates, `proc-macro-error`) is
  among the 292. License holders after: `MPL-2.0` 5 → **0** (`option-ext`, `cssparser`… all leave); `Zlib` 21 → 1
  (`foldhash`); `BSD-3-Clause` 7 → 1 (`matchit`); `Unicode-3.0` 19 → 1 (`unicode-ident`); `BSD-2-Clause` 3 → 3
  (`arrayref`, `zerocopy`, `zerocopy-derive`); `BSL-1.0` 1 → 1 (`ryu`); `Apache-2.0 WITH LLVM-exception` 16 → 15,
  every holder an OR-expression (`blake3`, `rustix`, `wasi`…). So whether `cargo deny` still needs that allow is a
  reading over the new lock, not an inference.
- **Surviving symbols the window reaches** (query: refs from `crates/conductor-tauri/%` to other crates, with their
  other reference sites). None is a crate-private item that loses its last use. The zero-other-site rows are pub fields
  of pub types (`RunEvent.count`, `RunEvent.stage` @ `crates/conductor-run/src/drive.rs`; `ScenarioSummary.name` @
  `crates/conductor-core/src/scenario_catalog.rs`). `drive_run`, `read_envelope` and `validate_selection` keep only
  test callers and their `pub use` re-exports (`crates/conductor-run/src/lib.rs:33-34`,
  `crates/conductor-core/src/lib.rs:74-76`). `dead_code` does not fire on a reachable pub item, so the removal raises
  no `-D warnings` hit. The clippy line of the bundled `run` is the measurement that confirms it at implement.

## Function reach — the window's eight handlers against the command line (B4, `inputs#I1` §3)
| window handler | its function | command-line entry today |
|---|---|---|
| `start_run` | pick + start a scenario or the suite | `conductor run <name\|P-ID>` · `conductor suite [--filter]` |
| `coverage_matrix` · `unbacked_auto` | the capability coverage view + its `(N unbacked)` caption | `conductor coverage [--write]` (`render::coverage_summary`, `UNBACKED_AUTO` read in core) |
| `run_report` | read a past run's results | `conductor report [run_id]` |
| `resolve_operator_hold` | go/no-go at a hold | the `inquire` confirm (`crates/conductor-cli/src/pause.rs:46-62`) |
| `list_scenarios` | list the catalogue (name, P-IDs) before choosing | **none** — no listing verb; the hint at `main.rs:66` points at `scenarios/` |
| `stop_run` | stop a run between scenarios, keeping what finished | **none graceful** — Ctrl-C ends the process; `drive_run`'s abort flag has no CLI caller |
| `run_envelope` | a past run's load-envelope standing | **none** — `report` prints the results table only (`commands/report.rs:17-20`); the caption prints at run time only (`commands/run.rs:33`, `commands/suite.rs:40`) |
| (hold dialog content) | the hold's `[[checklist]]` items (induced + observation) | **none** — `hold_line` + `confirm` print scenario · P-ID · step · prompt, never `hold.checklist` (`render.rs:201-206`, `pause.rs:46-62`) |

## Patterns detected
- **Removal as one act, both shells** (`scripts/agent-run.sh:294-350` ↔ `scripts/agent-run.ps1:343-410`): every run
  arm has a twin. The closed selector refuses an unknown token with `usage:` and exit 2, so a removed `--e2e` falls
  to the existing refusal (`agent-run.sh:350`).
- **A `deny.toml` entry retires with its subject** (`deny.toml:9-12`, the 2026-09-07 `number_prefix` precedent): the
  comment says which path left. The same act at scale here.
- **Static gates are presence-guarded `rust`-job steps** (`ci.yml:76-80`, `:102-108`, `:123-139`): editing the
  workflow must leave each step's shape intact.

## Conventions to follow
- **The lock moves by pruning, never by bumping**: the new `Cargo.lock` must equal the old one minus the 292 packages,
  every remaining package at its old version and source (security-plan §Dependency Security; the 2026-09-02
  basis-by-package-count learning).
- **Supply-chain readings take the porcelain first and last** (`.claude/rules/security.md` Session Additions
  2026-09-05 / 2026-10-07 / 2026-10-08): `git -C $CARGO_HOME/advisory-db status --porcelain` empty before and after
  `cargo audit`.
- **Harness shells delete by pattern, never recursively** (2026-09-06 addition): removing arms adds no `rm -r`.

## What the list missed (scope item 8), measured
Leaves here (window-only, no other consumer):
- `actions/setup-node` in the `rust` job (`ci.yml:42-47`); the `ci.yml:100` comment's a11y-job clause.
- `ensure_frontend` / `Invoke-EnsureFrontend`, `UI_DIR` / `$UiDir`, and their calls in `--unit`, `--integration` and
  the default arm of both shells (not only in the webview arms); the driven a11y step of the `--live` suite and its
  stop-form lines.
- `.gitignore` `:21-23`, `:46-52` (the window's log, frontend and `gen/` lines; `node_modules/` has no other npm tree:
  `git ls-files '*package.json'` → the crate's alone).
- `Cargo.toml` `tauri` / `tauri-build` workspace dependencies; the `regex` and `sha2` comments become true again by
  dropping their Tauri clause.
- Env handles losing every reader with the crate, the `a11y` job or the shell arms (`git grep -l` per handle over
  `crates scripts .github`, the crate excluded): `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, `CONDUCTOR_A11Y_STRICT`,
  `EDGEWEBDRIVER`, `WEBVIEW2_USER_DATA_FOLDER`/`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`, `RUNNER_TEMP`, `TEMP`. The
  `LOCALAPPDATA` hit in `conductor-core/src/redact.rs` is a scrub pattern, not a read: it stays.
- Harness-spawn rule (b): forms 1-4 and 8 live in `crates/conductor-tauri/ui/` (`wdio.conf.ts:127,213,245,417`,
  `test/a11y/screen-reader/{activate-window,send-keys}.ps1`); form 7 is `scripts/a11y-limited-token-launch.ps1`. All
  leave. Form 5 (`conductor cleanup` behind `valid_run_id` / `Test-RunId`) stays, the only one left.
- No secret-scan or workflow-env allowlist row and no mutation-roster unit names the crate (`git grep` over
  `crates/conductor-core/tests`, `scripts/mutation-roster.toml`); `.claude/settings.json` carries no window hook.

Stays, with its consumer and the entry that removes it:
- `crates/conductor-run/tests/envelope_fixture.rs` + `lamps_fixture.rs` (the two webview fixtures; the
  `CONDUCTOR_E2E_SEED_DIR` reader, an unset no-op once `wdio.conf.ts` leaves): `Panel-shaped types retired` (`:13`).
- `drive_run` / `RunEvent` / `RunStage` (the channel payload), `read_envelope` (the desktop envelope read),
  `ScenarioSummary` / `validate_selection` / `list_scenarios` (the picker summary) and
  `crates/conductor-cli/tests/cross_surface_parity.rs` (its "Tauri arm" drives `drive_run`, no `tauri` item):
  `Panel-shaped types retired` (`:13`).
- `crates/conductor-run/tests/operator_pause_harvest.rs` `live_leg` (frozen 2026-08-22 `tauri-dialog` witnesses, the
  0.2.0 `v2-29` evidence; graded without a window): `Window grading retired` (`:77`), which removes the hold and
  checklist.
- `crates/conductor-run/tests/live_suite.rs` (its header's wdio stop form): `Log-file harvest and live suite retired` (`:71`).
- Doc comments in surviving code that name the Tauri surface (`conductor-core/src/{lib,obs,pause,redact,run_journal,
  scenario,scenario_catalog,capability_manifest,coverage}.rs`, `conductor-run/src/{drive,envelope,execute,lib}.rs`):
  `Panel-shaped types retired` (`:13`) for its own types; the rest `No-survivor sweep` (`:136`).
- `.claude/rules/{frontend,a11y}.md`, the window text in `.claude/docs/*`, `CLAUDE.md` and the seven masters:
  the wrap's amendments (B6), then `Records restated` (`:134`).

## New files to create
- none — the chunk deletes and edits; its evidence lands under the chunk dir, outside the scope accounting

## Files to modify
- `crates/conductor-tauri/` — deleted whole: the crate, its `ui/` tree, fixtures, icons, capabilities
- `Cargo.toml` — the member line and the `tauri` / `tauri-build` workspace dependencies leave; the `regex` comment loses its Tauri clause
- `Cargo.lock` — pruned of the 292 packages only the window crate reached; no version moves
- `crates/conductor-run/Cargo.toml` — the `sha2` comment loses its tauri-codegen/wry clause
- `deny.toml` — the Tauri-tree advisory ignores and license allows whose subject left the lock leave
- `.github/workflows/ci.yml` — the frontend and a11y jobs and the rust job's node and frontend-bundle steps leave
- `scripts/agent-run.sh` — the e2e arm, the a11y verdict check, the driven a11y step and the frontend step leave
- `scripts/agent-run.ps1` — the same arms leave from the twin
- `scripts/a11y-limited-token-launch.ps1` — deleted
- `scripts/a11y-token-witness.ps1` — deleted
- `scripts/webview2-cause-probe.ps1` — deleted
- `.gitignore` — the window's log, frontend and gen lines leave

## Open questions
- Does `cargo deny check licenses` still need `Apache-2.0 WITH LLVM-exception` once the Tauri holders leave? Its 15
  remaining holders all carry an OR alternative that is allowed. → blocks: implementation-scope (the entry stays or
  goes on the reading over the new lock; never decided by inference)
