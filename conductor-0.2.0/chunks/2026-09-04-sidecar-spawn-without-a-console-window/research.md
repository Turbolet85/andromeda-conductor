# Codebase Research — 2026-09-04-sidecar-spawn-without-a-console-window

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 12 · **Graph queries:** 2 (rust + ts)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural
  extraction (52 050 B over 59 lines; `grep -n` index of the 6 section headers + all 20
  `## Session Additions` entries, then offset-bounded reads covering every indexed span). The
  entries that bear on this chunk: 48 (sidecar-on-PATH precondition), 50 (fresh-dir vs data-dir
  equality, as extended 2026-09-04), 53 (never `boot` before a preflight-firing leg; repo-relative
  `CONDUCTOR_*`), 54 (`[BLOCKED]` in ~0s = PATH), 57 (attended-GUI env block set in ONE paste),
  58 (process census + STOP form), 59 (SR foreground binding — **and its item (4), which records
  this chunk's exact defect**).

## Files inspected
- `crates/conductor-verify/src/spawn.rs` (1–110, 300–330) — the defect site, the consts, the
  existing unit test, and the in-file platform-branch precedent.
- `crates/conductor-verify/src/client.rs` (30–80) — `connect` / `connect_command` /
  `connect_transport`; the actual `command.spawn()` point.
- `crates/conductor-tauri/src/commands.rs` (155–230) — the two CARRY-1 doc comments, verbatim.
- `crates/conductor-core/src/run_journal.rs` (12–26) — `latest_run_id`'s `read_dir` failure arm.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` (55–140, 410–435) —
  `bringToForeground()` / `reactivateWindow()` and the single call site.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/activate-window.ps1` (full) — the shared script.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` (:181) — row R0-01.
- `crates/conductor-tauri/ui/src/App.tsx` (:241, :245, :272, :285, :309) — the four `role="alert"`.
- `.claude/settings.json` (:36) — the PostToolUse `rustfmt` hook, verbatim.
- `Cargo.toml` (:17) — `edition = "2024"`. `Cargo.lock` — `tokio 1.52.3`.
- `.github/workflows/ci.yml` (:18, :164) — **both** jobs `runs-on: windows-latest`.
- `tokio-1.52.3/src/process/mod.rs` (645–690) — `creation_flags` under `cfg_windows!`.
- `rustc 1.95.0` std `process.rs` — the five `get_*` getters on `std::process::Command`.

## Graph impact (from the code-graph query; per-plane DBs, both built 2026-09-04T19:16)
**Indexing note (this project's documented trap):** the code-graph reports lines **0-indexed**; grep and
editors are **1-indexed**. Every coordinate below and in `plan.md` is given 1-indexed, with the graph's raw
value in parentheses — all three sites were confirmed by grep, not converted arithmetically.

- **`build_command`** (rust plane, `callee_kind = 'fn'`) — **2 callers**, `rows: 2`:
  `ReadbackClient::connect` @ `crates/conductor-verify/src/client.rs:48` (graph 47), and its own unit test
  `build_command_targets_the_fixed_program_and_sets_only_the_data_dir_env` @ `spawn.rs:306` (graph 305).
  One production caller; the test is the crate-local companion that pins the artifact.
- **`reactivateWindow`** (ts plane) — **1 caller**, `rows: 1`:
  `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts:424` (graph 423; definition `:120`). The conditional
  retire has a blast radius of exactly one call site plus its `@reactivate` timeline stamp.
  For contrast, `bringToForeground` (definition `:68`) has **three** call sites — `:335`, `:546`,
  `:593` — serving the independent "a driver-launched window never self-activates" rationale.
- Signature impact: **none**. `build_command(&Path) -> Command` is unchanged by adding a builder
  call, so no caller threading is owed.

## Patterns detected
- **The spawn point is `connect_command`, not `build_command`** (`client.rs:55-57`): it takes the
  built `Command`, sets `.stdin(piped).stdout(piped).kill_on_drop(true)`, then `.spawn()`. It has a
  SECOND entry path — the child-spawn test passes the stub binary directly — so a flag placed at
  `connect_command` would also cover the stub, while a flag at `build_command` covers exactly the
  hardened sidecar the security mandate names. `build_command` is the tighter, mandate-matching home.
- **Piped stdio does not suppress console allocation.** `connect_command` already pipes stdin/stdout;
  on Windows a GUI-subsystem parent spawning a console-subsystem child still gets a console unless a
  `CREATE_NO_WINDOW`-class flag is set. This is why the defect survives the existing piping.
- **In-file platform-branch precedent, both forms** (`spawn.rs`): `#[cfg(windows)]` /
  `#[cfg(not(windows))]` block form in `platform_default()` (:44), `cfg!(windows)` runtime form in
  `resolves_on` (:129, :179), and **`#[cfg(windows)]`-gated unit tests** (:273, :288). The last is
  the direct precedent for testing a Windows-only helper.
- **The existing spawn test asserts via `cmd.as_std()`** (`spawn.rs:305-317`): `get_program()`
  contains `PULSE_MCP_PROGRAM`, and `get_envs()` has exactly one entry (`DATA_DIR_ENV` → the dir).
  Extending this test keeps the negative-test mandate intact rather than duplicating it.
- **The SR leg's activation script is wrapped by two functions with different rationales**
  (`bringToForeground` `:68` vs `reactivateWindow` `:120`) — only the second is this defect's
  workaround; its sole call site at `:424` sits immediately after row S1-01 and carries a comment
  naming the console window. The first is called three times (`:335`, `:546`, `:593`) and stays.

## Conventions to follow
- **Seam-local, no new dependency**: `tokio::process::Command::creation_flags(u32)` exists at
  `tokio-1.52.3/src/process/mod.rs:675`, inside `cfg_windows! { }`, delegating to std's
  `CommandExt::creation_flags`. Its doc states the flags "will always be ORed with
  `CREATE_UNICODE_ENVIRONMENT`", so setting the no-window bit alone is safe. **No `windows-sys`
  dependency is needed** — the flag is a `u32` literal (`CREATE_NO_WINDOW = 0x0800_0000`).
- **Consts carry a doc comment naming their rule** (`spawn.rs:15`, `:18`, `:22-23` — the `FORBIDDEN`
  const even documents what is deliberately NOT in it). A new flag const follows that shape.
- Crate-local `#[cfg(test)] mod tests` in the seam file, snake_case `#[test]` names
  (`spawn.rs:304+`).

## New files to create
- `rustfmt.toml` (workspace root) — one line, `edition = "2024"`. Honoured by both bare `rustfmt`
  (the PostToolUse hook) and `cargo fmt`; a setup re-render cannot overwrite it. **The seeded hook
  row is not edited.**

## Files to modify
- `crates/conductor-verify/src/spawn.rs` — add the flag const + apply it in `build_command` under
  `#[cfg(windows)]`; extend the existing unit test; correct the `sidecar_resolves_on_path` doc
  comment (:92) whose rationale cites the flagless spawn; correct the `build_command` doc comment
  (:79), which names `TokioChildProcess` — **a type absent from the code since the 2026-06-27 rmcp
  removal** (grep: that doc comment is its only occurrence in any `crates/**/*.rs`).
- `crates/conductor-tauri/src/commands.rs` — narrow the two "never an error" doc comments
  (:161-165 `run_report`, :217-221 `run_envelope`) to the `run_id = None` case they are true for.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` — **conditionally**: R0-01's reload
  drop (:599) is unconditional (CARRY 4); the `reactivateWindow()` retire (`:424` call site, `:120`
  definition, the `@reactivate` stamp) fires only if the live re-run measures no foreground handoff.
  `bringToForeground` (`:68`; called `:335`/`:546`/`:593`) is not touched.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — row R0-01 (:181) loses its
  "the leg still reloads, so this row does NOT yet discriminate" caveat once the reload drops.

## Open questions
- **Which element holds Chromium's sequential-focus-navigation starting point at mount?** (CARRY 3 /
  SR finding 1.) The freight says "mechanism unidentified, nothing guessed"; the leg's own comment
  (`screen-reader.e2e.ts:78-84`) attributes it to "the picker input at mount, measured 2026-09-02".
  The two are reconcilable only if something outside `src/` and outside cmdk's two guarded
  `.focus()` calls moves it. → blocks: **implementation-scope** — the fix cannot be designed until
  the holder is measured, and the leg's `tabsToStart` cycling loop would mask a fix if left in place.
- **Does the `sr` re-run reach a live incident at all?** Its validity rides three harness
  preconditions that are not this chunk's code: sidecar on `PATH` (rule 48/54 — a miss is
  `[BLOCKED]` in ~0s, indistinguishable from a real gate failure), `ANDROMEDA_PULSE_DATA_DIR` equal
  to the LIVE app's dir (rule 50 as extended 2026-09-04 — a Conductor-side fresh dir empties
  read-back), and a ≥120s + 30s quiet window with no `boot` beforehand (rule 53). → blocks:
  **implementation-scope** — these belong in the plan's leg invocation, not improvised at run time.

---

## Scope premise closure

Each `[inferred]` bullet in `scope.md`, resolved against the above. `scope.md` is amended in place.

| # | Premise | Verdict |
|---|---|---|
| 1 | tokio `creation_flags` feasibility / whether a dependency is needed | **VERIFIED** — exists at `mod.rs:675` under `cfg_windows!`; no new dependency |
| 2 | A fourth in-code site states the defect (`spawn.rs:92`) | **VERIFIED — and undercounted.** A **fifth** stale in-code claim sits at `spawn.rs:79` (`TokioChildProcess`), on the very function being edited |
| 3 | Windows-only construction must not break a non-Windows build | **VERIFIED, with a correction** — the `#[cfg(windows)]` precedent is in this same file (`:44`), but **CI compiles no non-Windows target** (both jobs `windows-latest`), so no gate would catch a cfg mistake |
| 4 | CARRY 1 may not apply (its condition was "the next entry touching conductor-tauri source") | **VERIFIED** — the fix is in `conductor-verify`, so the condition does not fire on its own; the comment correction stands as an independent deliverable |
| 5 | CARRY 2 (`rustfmt.toml`) should be ordered first | **VERIFIED** — the hook rewrites every `.rs` this chunk touches, and it defeated an anchored Edit mid-burst at the prior chunk |
| 6 | CARRY 3 carries a competing in-repo mechanism claim | **VERIFIED** — both claims stand; unresolved, and now an Open question blocking implementation-scope |
| 7 | No verification-matrix capability is an obvious claim | **carried to P5** — no research finding changes it |

**One premise NOT in `scope.md`, surfaced by research and material to the plan's test design:**
`std::process::Command` exposes exactly five getters (`get_program`, `get_args`, `get_envs`,
`get_current_dir`, `get_env_clear`) and **no creation-flags getter**, so "the flag is set on the
Command" is **not assertable as observable behaviour** through `as_std()`. A direct unit assertion
is unreachable. This is the tests extract's flagged question, answered: the flag VALUE can be pinned
by a pure const/helper test, while its APPLICATION is a test-plan §1 untestable zone whose real
measurement is the live `sr` re-run — never a manual "developer confirms no pane appears" step
(test-plan §11 → Universal bans it).
