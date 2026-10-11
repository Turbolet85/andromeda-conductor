# Scope — 2026-10-11-conductors-window-retired

**Working entry (`working-route.md:11`):** Conductor's window retired — window crate, frontend and a11y CI jobs,
window scripts, webview arms, driver spawn forms, fetched-driver class, Tauri-tree audit exceptions leave (v4-01,
v4-09)

**Matrix target:** the entry names `v4-01` and `v4-09`. Both are also named by later markerless entries (`v4-01` by
`Panel-shaped types retired`, `:13`; `v4-09` by `:13`, `:47`, `:49`, `:107`, `:113`), so whether this chunk can make
either fully verifiable is decided at P4 and shown at the review, never assumed here.

**Why it exists:** intent §5 R1, ruled by the founder on 2026-10-09 (both projects go to a console form). The word
to start code was given the same day (`inputs#I2`): code starts on both projects, one builder at a time.

**Freight on the entry:** none (`route.py pins`: the one freight block on the markerless tail sits on `:15`).

## What this chunk removes

The entry's list is closed (`inputs#I1` §2). Each item is measured against the tree at P3; an item with another
consumer is named with that consumer and left, with the entry that removes it.

1. **The window crate.** `crates/conductor-tauri` leaves the workspace: its directory, its member line, its
   packages in `Cargo.lock`.
   - Workspace-level dependency entries that only this crate uses leave with it: `tauri` and `tauri-build`
     (`Cargo.toml:78-79`; no other member declares either — research.md §Files inspected).
   - The frontend under `crates/conductor-tauri/ui` (npm tree, lockfile, a11y and screen-reader suites) leaves as
     part of the crate directory: 47 of the crate's 59 tracked files.
2. **The `frontend` and `a11y` CI jobs** leave `.github/workflows/ci.yml`.
   - The `rust` job's frontend-bundle step (`ci.yml:49-56`) and the `actions/setup-node` step before it (`:42-47`,
     cached on the ui lockfile) exist only so the window crate compiles, and leave here; the job's Windows runner
     does not (item B2). The obs, panic and journal conformance gates already sit in `rust` and stay.
3. **The window scripts.**
   - They are `scripts/a11y-limited-token-launch.ps1`, `scripts/a11y-token-witness.ps1` and
     `scripts/webview2-cause-probe.ps1`. Their only consumers are the `a11y` job and each other (`git grep -l` per
     script name, records excluded).
4. **The webview arms of the run script.**
   - [premise-corrected: the `sr*` suites are npm scripts inside `ui/`, not shell arms; the frontend step rides every
     run arm] In both `scripts/agent-run.sh` and `scripts/agent-run.ps1`: the `--e2e` arm with its printed-verdict
     check (`assert_a11y_verdict` / `Assert-A11yVerdict`, the expected-skip count); the `--live` suite's driven a11y
     step and its stop-form lines; and `ensure_frontend` / `Invoke-EnsureFrontend` with `UI_DIR`, which `--unit`,
     `--integration` and the bundled default also call. The `sr*` suites leave with `ui/` (item 1). The PowerShell
     twin itself stays (item B2).
5. **The driver spawn forms.** Harness-spawn rule (b)'s forms that exist for the webview driver stack leave with
   what they spawn.
   - Forms 1-4 and 8 live in `crates/conductor-tauri/ui/` (`wdio.conf.ts`, `test/a11y/screen-reader/*.ps1`) and
     form 7 is `scripts/a11y-limited-token-launch.ps1`: all leave. Form 5 (`conductor cleanup` behind
     `valid_run_id` / `Test-RunId`) stays, the only governed form left.
6. **The fetched-driver class.** The CI-time fetch of the version-pinned msedgedriver, its Authenticode gate, and
   with them the one registered network exception.
7. **The Tauri-tree audit exceptions.** The `deny.toml` advisory ignores and license allows that exist only for
   the Tauri dependency tree.
   - Each entry is decided against the dependency graph AFTER the crate is gone (research.md §Graph impact): the
     16 advisory ignores and `MPL-2.0` lose every subject; `Zlib`, `BSD-2-Clause`, `BSD-3-Clause`, `Unicode-3.0` and
     `BSL-1.0` keep named holders; `Apache-2.0 WITH LLVM-exception` is decided by `cargo deny`'s reading over the
     new lock. `cargo audit` and `cargo deny check` both read green over the new lock.
8. **What exists only for the window and the list missed** (`inputs#I1` §2), measured in research.md §What the
   list missed. Leaving here: the `rust` job's `setup-node` step; the frontend step in every shell run arm; the
   window's `.gitignore` lines; the `tauri` / `tauri-build` workspace dependencies; the Tauri clause of the `regex`
   and `sha2` manifest comments; the `ci.yml:100` comment's a11y-job clause. Env handles losing every reader here:
   `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, `CONDUCTOR_A11Y_STRICT`, `EDGEWEBDRIVER`, the `WEBVIEW2_*` pair,
   `RUNNER_TEMP`, `TEMP`. Not touched: `.gitattributes`, the coverage, mutation and secret-scan rosters, the hooks
   and settings (none names the window).
   - Staying, with the entry that removes it: `CONDUCTOR_E2E_SEED_DIR`'s reader and the two webview fixtures, the
     channel payload, envelope read and picker summary, and `cross_surface_parity.rs` → `Panel-shaped types
     retired` (`:13`); the recorded `tauri-dialog` witnesses in `operator_pause_harvest.rs` → `Window grading
     retired` (`:77`); `live_suite.rs`'s wdio stop-form header → `Log-file harvest and live suite retired` (`:71`);
     Tauri-naming doc comments in surviving code → `:13` for its own types, `No-survivor sweep` (`:136`) for the
     rest; `.claude/rules`, `.claude/docs`, `CLAUDE.md` text → the wrap's amendments, then `Records restated`
     (`:134`).

## Where it ends

- The tree builds, tests and passes its full local gate with no window crate, and CI is green on the push that
  carries it (`inputs#I1` §5).
- No crate's dependency graph holds the window toolkit: a reading of the post-removal `Cargo.lock` and `cargo
  metadata`, not an argument from the member list. Measured beforehand: 562 → 270 packages, 292 only the window
  crate reaches, no Tauri-family name left (research.md §Graph impact).

## Boundaries

- **B1 — panel-shaped types outside the crate stay.** The run-event channel payload, the desktop envelope read,
  the picker summary, the dialog-row limit, the window log sink and the two webview fixtures are
  `Panel-shaped types retired`'s (`working-route.md:13`).
  - [premise-corrected: graph query — every such item is pub and re-exported, so `dead_code` does not fire] Removing
    their only consumer leaves them with test callers and `pub use` re-exports only; no crate-private item loses its
    last use (research.md §Graph impact). The bundled `run`'s clippy line confirms it at implement.
- **B2 — the other operating system stays.** Windows runners, `scripts/agent-run.ps1` as a twin and any non-window
  PowerShell are `Linux-only base CI`'s (`:15`), with the `CARRY:` on that entry.
- **B3 — what Conductor reads of Pulse stays as it is** (`inputs#I1` §4). Nothing is re-pointed at the engine or
  its door; that is `Engine read through the door`'s (`:19`). If the removal forces a touch of the Pulse-facing
  path, P4 stops and says where.
- **B4 — reachability from the command line** (`inputs#I1` §3). "Every function Conductor has is reachable from
  the command line" is `v4-01`'s clause and the next entry's subject.
  - research.md §Function reach lists the window's eight handlers against the command line's verbs. Four
    functions lose their only entry point: the catalogue listing, a graceful mid-run stop, a past run's
    load-envelope standing, and showing a hold's checklist items. P4 says whether this chunk or the next gives each
    its command; nothing is left unreachable between the two.
- **B5 — no new surface** (`inputs#I1` §6). A new listening socket, place on disk, credential or command word halts
  for the operator's word. Ports 4317 and 4318 are shared with Pulse's builder on this host: no run that binds
  them without asking.
- **B6 — the masters.** Phase amends no spec source. The masters, rules and summaries that describe what this chunk
  removes are corrected by the wrap's amendments.
  - The full restating ("the masters say in plain words that Conductor has no interface", `v4-01`) is not this
    chunk's alone: the console design is restated by `Run read from the command line` (`:47`, which names "console
    design restated without window or hold") and the first-read records by `Records restated` (`:134`). P4 lists
    the expected amendments this removal makes true.
- **B7 — size** (`inputs#I1` §5). If the removal does not fit one chunk, P4 proposes the cut: a first chunk ending
  with a tree that builds, tests and passes CI with no window crate, and what the second holds.

## CI read at Setup (the last flip `355a678` through HEAD)

| sha | verdict | run | wall |
|---|---|---|---|
| `321dc8f` | green, 3 of 3 checks | CI#38039792389 | 687 s |
| `7a2b8e5` | green, 3 of 3 checks | CI#37990683772 | 699 s |
| `9e8b30f` | green, 3 of 3 checks | CI#37937147976 | 687 s |
| `f6fdf49` | untested, pushed under a later tip (`9e8b30f`, green) | — | — |
| `d227e59` | red: `Rust gate (build · test · lint · supply-chain · coverage)`, first fail at +310 s | CI#37932450560 | — |
| `97dea7f` | green, 3 of 3 checks | CI#37770212262 | 691 s |
| `355a678` | green, 3 of 3 checks | CI#37767498147 | 696 s |

**The red on `d227e59`, dispositioned.** Its failing subject is the matrix-ledger gate reading only the bolded
capability-id form (the handoff's note; `conductor-report::matrix_ledger_gate`). It does not intersect what this
chunk builds. It is not unowned and not folded here: the operator's word is that it is not a red to carry into this
chunk (`inputs#I1` §1), `f6fdf49` re-spelled the ids, the three later tips read green, and the gate's own repair is
the `CARRY:` on `Linux-only base CI` (`working-route.md:15`).
