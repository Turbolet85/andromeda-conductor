# Codebase Research — 2026-09-02-mutation-tier-restored-for-conductor-tauri

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** ~10 · **Graph queries:** 3 (rust plane) · **Live probes:** 2
- **Harness rules consulted:** `.claude/rules/testing.md` — read as a structural extraction (index of all
  33 `## Session Additions` entries, then 6 read in full: `:52`, `:63`, `:64`, `:65`, `:75`, `:76`).
  `.claude/rules/verification-harness.md` NOT consulted: this chunk names no live-Pulse leg — its two
  probes are local `cargo nextest` invocations, so the live-leg firing-form rule does not engage.

## The two live probes (the mechanism, re-derived at HEAD)

Both ran against the shipped tree with only the *undeclared build artifact* varied. Together they
verify the load-bearing equality from both directions.

| # | invocation | artifact state | result |
|---|---|---|---|
| 1 | `cargo nextest run -p conductor-tauri` | `target/debug/conductor.exe` moved aside | **exit 100** — 13 run, 12 passed, **1 failed**: `path7_…` panics at `assert_cmd-2.2.2/src/cargo.rs:232`, `` `CARGO_BIN_EXE_conductor` is unset ``, with assert_cmd's own help text "move it to an integration test to gain access to `CARGO_BIN_EXE_conductor`" |
| 2 | `cargo nextest run -p conductor-cli` | same artifact still absent | **exit 0** — 36/36 passed, and cargo **REBUILT** `target/debug/conductor.exe` (new file, 15 022 592 B) as part of the invocation |

Probe 1 reproduces the audit's abort signature exactly (same test, same panic, same assert_cmd
line). Probe 2 is the control the audit never ran: the identical `Command::cargo_bin("conductor")`
call in `crates/conductor-cli/tests/cli_smoke.rs` is green on a missing artifact **because Cargo
builds a package's own bins for that package's test targets and sets `CARGO_BIN_EXE_conductor`**.
The pre-existing artifact was dated `Sep 2 16:11` — byte-matching the audit's "dated 16:11" claim.

## Files inspected
- `crates/conductor-tauri/src/commands.rs` (`:340-345`, `:343-400`, `:466-486`, `:486-575`) — the
  `#[cfg(test)] mod tests` block, its mock-runtime helpers, `stage_repo_root`, and the parity test in full.
- `crates/conductor-tauri/Cargo.toml` (full) — deps `conductor-core`/`conductor-run`/`serde`/`tauri`/
  `tokio`/`tracing`; dev-deps `tauri[test]`/`tokio[rt,sync,macros]`/`assert_fs`/`serde_json`/`assert_cmd`.
  **No `[lib]`, no `[[bin]]` stanza, no `conductor-cli` edge**; `:26` is the comment naming `conductor-cli`.
- `crates/conductor-tauri/src/` (listing) — `commands.rs` · `main.rs` · `pause.rs`. **No `lib.rs`.**
- `crates/conductor-cli/Cargo.toml` (full) — `[[bin]] name = "conductor"`; `[dependencies]` carry
  `conductor-core` · `conductor-report` · `conductor-run` · `tokio {macros, rt}`; `[dev-dependencies]`
  carry `assert_cmd` · `assert_fs` · `predicates` · `serde_json`.
- `crates/conductor-cli/tests/cli_smoke.rs` (`:1-40` + grep) — 6 `Command::cargo_bin("conductor")` sites
  and the `copy_*` staging helpers `stage_repo_root` explicitly says it mirrors.
- `crates/conductor-verify/tests/preflight_spawn.rs` (`:4`, `:24`) + `crates/conductor-verify/Cargo.toml`
  (`:25-28`) — the in-repo `[[bin]] + required-features + env!("CARGO_BIN_EXE_…")` precedent.
- `crates/conductor-run/tests/envelope_fixture.rs` (`:1-45`) — the committed-fixture + `CONDUCTOR_E2E_SEED_DIR`
  no-op-when-unset shape.
- `rust-toolchain.toml` — `channel = "1.95.0"` (**stable**).
- `Cargo.toml` (workspace, `:1-40` + `:82-84`) — the 9 members; `assert_cmd`/`assert_fs`/`predicates`
  already `[workspace.dependencies]`.
- `.gitignore` (`:11`, `:36`, `:46-50`) — `/target/`, `crates/conductor-tauri/ui/dist/`, `mutants.out/`,
  `mutants.out.old/` all ignored. `.andromeda/runs/` is **NOT** ignored (it is committed).
- `.andromeda/runs/2026-09-02T15-49-17-code-audit/` — `proposals.md:35-77`, `run-mutants.sh`,
  `c-mutation-abort-conductor-tauri.json`.
- `scripts/code-graph-cookbook.md` (full) — read before composing SQL, per the research discipline.

## Graph impact (rust plane; `db_state` built, all probe names resolved)

**Line convention:** graph rows carry 0-indexed SCIP ranges. Every `:line` below is the **EDITOR** line
(graph value + 1), each re-confirmed against the file by `grep -n` — so the graph's `486` / `471` / `501`
are cited here as `487` / `472` / `502`.

- **`path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db`** — `conductor-tauri`,
  `crates/conductor-tauri/src/commands.rs:487`. The test module IS indexed, so its call edges are real data.
- **`stage_repo_root`** — `crates/conductor-tauri/src/commands.rs:472`, **exactly ONE caller**: the parity
  test itself (`:502`). The helper relocates with the test cleanly; nothing else depends on it.
- **Parity test's outbound calls** — `stage_repo_root` (`:502`) · `preflight` (`:533`) · `drive_run` (`:536`) ·
  `read_run_journal` (`:551`, `:552`). Every callee is `conductor_core` / `conductor_run`.
- **`crate_edges`** — `conductor-tauri` → `conductor-core`, `conductor-run` only, with **zero inbound
  edges** (a leaf bin); `conductor-cli` → `conductor-core`, `conductor-report`, `conductor-run`,
  `conductor-verify` (the last is a usage edge through a re-export, not a manifest line — the cookbook's
  "resolved, NOT manifest-declared" caveat). **No bin↔bin edge exists in either direction.**

## Patterns detected
- **The parity test uses NO `tauri::*` item** (`commands.rs:486-575`). Its "Tauri arm" calls
  `conductor_run::preflight` + `conductor_run::drive_run` directly — the same composition `start_run`'s
  thread runs — while the module's `tauri::test` mock-runtime helpers (`test_app`, `main_window`,
  `invoke`) serve the six *other* `#[test]` fns. The parity test's only `conductor-tauri`-specific
  dependency is its **location**.
- **Same-package bin binding is the established in-repo shape** (`preflight_spawn.rs:24`):
  `env!("CARGO_BIN_EXE_stub_pulse_mcp")` against a `[[bin]]` declared in `conductor-verify`'s own
  manifest — the pattern `.claude/rules/testing.md:52` (2026-06-21) codified.
- **`cli_smoke.rs` is the same test shape one package over**, already portable: staging helpers by
  `env!("CARGO_MANIFEST_DIR")/../..`, `assert_fs::TempDir` sandbox, `.env(...)` injection lever.
  `stage_repo_root`'s own doc comment says it "Mirrors `cli_smoke`'s helpers".
- **Cargo passes both `[dependencies]` and `[dev-dependencies]` to test targets** — `cli_smoke.rs`
  compiles today against `conductor-cli`'s dev-deps, and its `[dependencies]` (`conductor-core`,
  `conductor-run`, `tokio {macros, rt}`) are the exact set the parity test needs.

## Conventions to follow
- **Mutation verdict from the tallies, never the exit code** — `mutants.out/missed.txt` empty + named
  survivors in `caught.txt`; `Found 0 mutants to test` is a no-op, not a pass (test-plan §4).
- **`--test-tool=nextest` + workspace-root `-f` paths + gitignored `--output`** — the audit's
  `run-mutants.sh` already satisfies all three, so the invocation is not the defect.
- **Runner portability is a standing gate** — green under BOTH `cargo nextest run -p <crate>` and
  `cargo test -p <crate>`, never fixed by a runner pin, `retries`, or `--test-threads=1` (test-plan §4/§10).
- **Own-test-binary relocation is the plan-prescribed remedy** for a test whose isolation needs exceed a
  shared `#[cfg(test)]` module (test-plan §11; `.claude/rules/testing.md:52`).
- **Dependency-delta basis is the PACKAGE COUNT** (baseline measured this phase: **564**), plus
  `cargo deny check advisories bans licenses sources` green over the new lock — never lockfile
  byte-identity (`.claude/rules/security.md`, 2026-09-02).

## New files to create
- `crates/conductor-cli/tests/{name}.rs` — **if** the relocation form wins: the parity test as its own
  integration-test binary in the package that declares the `conductor` bin, carrying `stage_repo_root`
  (its sole caller moves with it). Exact filename is the plan's to fix.

## Files to modify
- `crates/conductor-tauri/src/commands.rs` — remove the parity test + `stage_repo_root` from the
  `#[cfg(test)] mod tests` block (relocation form), or re-bind its CLI arm in place (build-at-test-time
  form). The six mock-runtime `#[test]` fns and `pause.rs`'s six stay put either way — they are the
  IPC-tier proofs security's extract requires remain killable by `-p conductor-tauri`.
- `crates/conductor-tauri/Cargo.toml` — only if the winning form changes the dev-dep set. Under the
  relocation form `assert_cmd` becomes unused by `conductor-tauri` and its dev-dep line is removable
  (a lock EDGE change, package count unmoved — the exact shape the prior chunk measured in reverse).
- `crates/conductor-cli/Cargo.toml` — **no change required** under the relocation form: every crate the
  parity test imports is already a dependency or dev-dependency of this package (verified above).
- `conductor-0.2.0/verification-matrix.json` — `v2-25`'s `ref` is a source path
  (`crates/conductor-tauri/src/commands.rs::commands::tests::path7_…`) and goes stale on any relocation.
- `Cargo.lock` — asserted at package count 564 with `cargo deny` green; no new package under the
  relocation form.

## Scope premise closure

| scope premise | verdict |
|---|---|
| Proposal form (a) — "move to `crates/conductor-tauri/tests/`" — may not yield `CARGO_BIN_EXE_conductor` | **VERIFIED, and it is worse than stated.** Probe 1 measured the var **unset for `conductor-tauri` targets** (unit *and* integration — the var is set per *declaring package*), and `conductor-tauri` has **no lib target**, so an integration test there could not import the crate at all. The entry's "moved to the bin-owning package" (→ `conductor-cli`) and the proposal's "(a) move to `conductor-tauri/tests/`" are indeed different fixes, and only the former resolves the variable. |
| A plain `[dev-dependencies] conductor-cli` edge may not build the binary; artifact-deps are nightly-only | **VERIFIED.** `rust-toolchain.toml` pins stable `1.95.0`, so `-Z bindeps` is unavailable without moving a locked inherited default (arch extract: an arch decision this chunk may not make). |
| escargot would admit a new external package | **VERIFIED.** `grep -c escargot Cargo.lock` = **0**; the baseline package count is **564**. |
| The mutation run's cost is unmeasured | **STILL OPEN.** 43 planned mutants; the audit's `--jobs 2` / 2400 s budget aborted at the baseline, so no completing-run duration exists. Carried to the plan as a budget question, not a blocker. |
| Relocating the test disturbs `v2-25`'s `ref`, and it is unclear who repairs it | **VERIFIED, and now triangulated.** The ref is path-shaped. Three extracts raised it independently and agree on ownership: tests ("a route/wrap-owned reconcile item, not a test-plan mandate"), obs ("the matrix contract's question, not obs's"), layouts ("must follow it, or the layouts invariant loses its named guard"). |

**Mechanism-claim re-derivation.** The entry's two `measured`-marked claims were re-derived at HEAD
rather than inherited. (1) "reaches the CLI through `assert_cmd`'s `legacy_cargo_bin` fallback … while
`conductor-tauri` declares no dependency on `conductor-cli` at all" — **holds**: the manifest carries no
such edge in any section, the graph shows no bin↔bin edge, and probe 1 reproduces the panic the moment
the artifact is gone. (2) "genuinely green under `--workspace` … green under `-p conductor-tauri`
against a PRE-EXISTING artifact dated 16:11; it aborts on a fresh target dir" — **holds**, with the
artifact's `Sep 2 16:11` timestamp confirmed on disk before probe 1 moved it. Neither claim needed
correction, so no `[premise-corrected]` bullet and no extract was poisoned.

**One finding the scope did not anticipate** (recorded, not acted on): `v2-25`'s acceptance text names
"a `tauri::test` mock-runtime run and an `assert_cmd` CLI subprocess run", but the shipped `ref` test's
first arm calls `conductor_run::{preflight, drive_run}` **directly**, using no `tauri::*` item. That is
what makes a relocation nearly free — and it is also a divergence between a `verified` capability's
acceptance prose and its evidence. The chunk's scope explicitly excludes re-litigating the parity
assertion, so this is surfaced for the P5 review, not reopened here.

## Open questions
- Which binding FORM the chunk adopts — relocation to `crates/conductor-cli/tests/` (zero package
  delta, zero manifest change, satisfies test-plan §11 and assert_cmd's own prescription, but moves the
  test out of `conductor-tauri` and stales `v2-25`'s ref) vs. an in-place build-at-test-time binding
  (keeps the location and the ref, but is a FIFTH governed spawn form owing wrap's amendment flow per
  security's extract). → blocks: **plan-decision**.
- What shape the "runner-portability gate proves the test on a FRESH target dir" acceptance takes — a
  cold `CARGO_TARGET_DIR` run (true to the words, expensive against a ~23 GB warm target), or the
  `cargo test -p <crate>` + `cargo nextest run -p <crate>` standing pair plus the artifact-absent probe
  this research used (cheap, and it is what actually discriminates). → blocks: **plan-decision**.
- Whether `v2-25`'s stale `ref` is repaired by this chunk's implement step or surfaced to wrap's
  reconcile — the matrix contract governs a `verified` cap's ref, and phase sets `chunk` only on caps
  this chunk makes verifiable. → blocks: **implementation-scope**.
