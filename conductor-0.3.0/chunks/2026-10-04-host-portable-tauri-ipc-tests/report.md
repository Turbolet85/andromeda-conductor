# Report — 2026-10-04-host-portable-tauri-ipc-tests

**Chunk:** Host-portable Tauri IPC tests — the conductor-tauri mock-runtime IPC tests pass on Linux and Windows alike
**Date:** 2026-10-04T01:20Z
**Commits:** `84a3759 chore(2026-10-04-host-portable-tauri-ipc-tests): operator pre-CI commit, for the run this chunk's
verdict reads` (the one commit since `last_wrap` 2026-10-04T00:39:22Z; base `f5076ad`, its parent)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-tauri/src/commands.rs` (inside `#[cfg(test)] mod tests` only) ·
  `crates/conductor-tauri/src/pause.rs` (inside `#[cfg(test)] mod tests` only). Basis: `git diff --name-only f5076ad --
  crates` → these 2 paths; `gate.py scope` → `clean — changed 2 · listed 2 · recorded 0`.
- **Symbols / APIs:** test-module helpers only, no production symbol:
  - `commands::tests::request(cmd, body)` → `request(window: &MockWindow, cmd, body)`; its `url` is now
    `window.url()` (the dispatching mock webview's own app URL) instead of the literal `"http://tauri.localhost"`.
    Callers: `invoke()` and `invoke_expecting_error()`, both in the same module, plus the new test (3 call sites, all
    in `commands.rs`'s test module — grep `request(&\?window` over the file).
  - `commands::tests::invoke_expecting_error` now panics when the rendered error contains `"{cmd} not allowed"` (an
    ACL/origin refusal), so its 3 callers (`run_report_errors_…`, `run_envelope_errors_…`, `list_scenarios_errors_…`)
    pass only when the HANDLER produced the error.
  - New test `commands::tests::a_foreign_origin_is_refused_on_every_host` — a `coverage_matrix` dispatch whose `url` is
    `https://example.invalid` is `Err` containing `"coverage_matrix not allowed"`.
  - `pause::tests::resolve_operator_hold_command_delivers_the_decision` — its inline `InvokeRequest.url` is
    `window.url()` too.
  - No IPC method, endpoint, port, socket or env var added or changed. The production `#[tauri::command]` handlers and
    their `tauri.command.<name>` span guards are untouched (the whole diff lies inside the two test modules).
- **Crates / modules:** none added/removed; `conductor-tauri` test modules changed.
- **Dependencies:** none — `grep -c "^name = " Cargo.lock` → 562 (unchanged); `Cargo.toml`/`Cargo.lock` absent from the
  diff (the boundary-guard probe: `last line 0`).
- **Schema / config:** none — no capability file (`capabilities/*.json`), `tauri.conf.json`, `.config/nextest.toml`
  (`retries = 0` ×2 held), `deny.toml` or `.github` change.
- **Spec-master edits:** none during the chunk (implement is read-only on the seven masters).
- **Counts / qualifiers moved:** `conductor-tauri` test count 26 → 27 (nextest summary, `gate.py` entry 2 log); on this
  Linux dev host the crate went `26 run: 20 passed, 6 failed` → `27 run: 27 passed`, and the workspace nextest went red
  (6 failed) → `1197 tests run: 1197 passed, 0 skipped`; Windows CI: `1200 tests run: 1200 passed` (1197 + the three
  `#[cfg(windows)]` tests in `crates/conductor-verify/src/spawn.rs`, counted by grep over `crates/`). No master states
  any of these values — python sweep of the seven masters for `\b2[67] tests\b|\b26\b…tests|1197|1200 tests`: 0 hits.
- **Dev-tool versions:** none — no host tool installed or changed this chunk (cargo/nextest/npm used as found).
- **Harness / gate surface:** none — no agent-run script, CI step or status shape changed. CI stays Windows-only; the
  Linux leg is the dev host.
- **Cross-project / external claims:**
  - tauri 2.11.3 (the `Cargo.lock`-pinned version, read from the local cargo registry source): the local-origin
    predicate `Webview::is_local_url` compares a request URL's scheme+domain against `AppManager::tauri_protocol_url`,
    which is `http://tauri.localhost` under `cfg!(windows) || cfg!(target_os = "android")` and `tauri://localhost`
    otherwise (`manager/mod.rs:339-346`, `webview/mod.rs:1698-1706`); a non-local origin resolves Remote and the ACL
    refuses an app command with `"{cmd} not allowed. Plugin not found"` (`ipc/authority.rs:409`). `WebviewWindow::url()`
    returns `crate::Result<Url>` (`webview/webview_window.rs:2379`); for `mock_context(noop_assets())` + a window at
    `WebviewUrl::default()` the mock webview's URL is `get_app_url(false)` → `tauri_protocol_url(false)`
    (research.md §The equality).
  - CI run **CI#37166666634** measured sha `84a37591f890` (the pre-CI commit): `verdict: green · checks 3/3 · wall
    715 s`; Rust gate success (image `windows-2025-vs2026`), Frontend gate success, A11y gate success (image
    `windows-2022`). This wrap's commit adds bookkeeping on top of that tree. Evidence:
    `chunks/2026-10-04-host-portable-tauri-ipc-tests/evidence/operator-pass.md`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The custom-protocol / mock-webview origin is NOT a single value `http://tauri.localhost`: it is per-OS
    (`tauri://localhost` on Linux/macOS). Stated sites: (a) `crates/conductor-tauri/src/commands.rs` test doc comment
    (fixed in this diff); (b) `.claude/rules/testing.md:65` — the 2026-06-27 Session Addition's "any other value (e.g.
    `tauri://localhost`) fails dispatch" and its prescription of the fixed literal (a rule file, not a master — P3
    curation owns it); (c) `.andromeda/architecture.md:214` (§Infrastructure Patterns → Build system) records the
    2026-09-01 measurement "opened on `http://tauri.localhost/`" for a `custom-protocol` build without naming its host
    — true on the Windows host it was measured on, to be QUALIFIED as the Windows form, never swapped for a Linux
    literal. Evidence: tauri 2.11.3 `manager/mod.rs:339-346`; the Linux gate run (27/27 green with the window-derived
    URL) and the inverse control below.
  - Measured with an inverse control (unplanned, in implement): restoring the Windows literal into `request()` on this
    Linux host reads `27 run: 19 passed, 8 failed` — the 5 `invoke` tests plus the 3 `invoke_expecting_error`
    callers, each now failing with "refused by the ACL/origin check, not failed by its handler" (before this chunk
    those 3 PASSED vacuously on Linux). File restored sha256-identical; gates 2/5/9 re-run green.
- **Expected amendments (from plan):**
  - architecture §Infrastructure Patterns → Build system — the 2026-09-01 origin measurement is Windows-only; qualify
    the custom-protocol origin as per-OS → **carried**: the Spec-claims-disproved bullet (c) above. Site search: python
    sweep of the seven masters for `tauri\.localhost|tauri://localhost|mock…origin|Plugin not found|not allowed\.
    Plugin` → 1 hit, `architecture.md:214`; 0 hits in the other six.
  - Curation at wrap — `.claude/rules/testing.md:65` mock-origin gotcha → **carried** to P3 (a rule file, not a
    master): the Spec-claims-disproved bullet (b).
- **Coverage of new surfaces:**
  - `a_foreign_origin_is_refused_on_every_host` (test) → validation n/a · instrumentation n/a · PII n/a · tests unit
    (mock-runtime dispatch tier) · a11y n/a · tokens n/a
  - No new external surface, hot-path op or UI element.

## Deviations from intent
- **One unplanned verification step (implement P2):** the inverse control above — a temporary edit restoring the old
  literal, run, then restored byte-identically (sha256 checked) and gates 2/5/9 re-run green. Justification: the plan's
  gates prove the fix passes, but no listed entry proved the step-3 guard can FAIL; the control shows it discriminates.
  No file outside the two listed ones was touched.
- **Doc-comment form:** step 2's comment wraps the denial text in backticks (`` `"<cmd> not allowed. Plugin not
  found"` ``) so rustdoc never reads `<cmd>` as an HTML tag. Within the step's intent.
- scope record: none — `gate.py scope` clean (P1 of this wrap: `changed 2 · listed 2 · recorded 0`), 0 recorded.

## Decisions & corrections
- Overseer directives (2026-10-04, at implement): anchor every diff-shaped probe to the chunk base `f5076ad`, never
  `HEAD`; stop this repo's rust-analyzer flycheck cargo tree by PID before each heavy cargo step (measured: none was
  running at either check); this chunk needs no pulse-app and no ports; stop after the gates and before the operator
  pass. The operator pass (hygiene, pre-CI commit, push, CI read) was then performed by the agent on the overseer's
  explicit word relaying the founder's "run the plan".
- Founder directive at wrap (autonomous mode): a question that would need the founder (trajectory / epoch split) is
  recorded with a recommended option as pending his word, not blocking; the Epoch 5 split stays undecided.
- A claim made in the evidence draft ("three `#[cfg(windows)]`-gated tests") was written before it was measured, then
  verified by grep (exactly three, all in `conductor-verify/src/spawn.rs`) and given its basis before anything cited it.
- Sweep hazards found this chunk: (1) on this Linux host `grep` resolves to **ugrep**, which REJECTS a pattern combining
  bounded repeats with alternation (`.{0,60}(a|b).{0,60}`) — `error at position 187 … exceeds complexity limits` — and
  prints no hits, so a multi-term context sweep reads empty; the python `re` sweep is the form that worked. (2) A Bash
  call that `cd`s into the cargo registry and then greps a relative path was refused by a `./secrets/**` deny rule the
  harness could not resolve against the `cd` target; absolute paths without `cd` pass.
- Plan design held: the window-derived origin (not a `cfg!` literal pair) — no per-OS literal in a code line of either
  file (the literal probe reads 0).

## Outcome
**Acceptance criteria (re-asserted against the diff):**
- `cargo nextest run -p conductor-tauri --profile ci` exits 0 on Linux with `27 tests run: 27 passed, 0 skipped`; none
  skipped/`#[ignore]`d/`cfg`-excluded (the diff adds no attribute of that kind); `retries = 0` held ×2 — **met**.
- `cargo test -p conductor-tauri` exits 0 (27 passed) — **met**.
- `cargo nextest run --workspace --profile ci` exits 0 on Linux (1197/1197) — **met**.
- The three `invoke_expecting_error` callers pass only when the handler errs — **met** (the guard + the inverse control).
- `a_foreign_origin_is_refused_on_every_host` passes (Linux; and in CI on Windows) — **met**.
- No per-OS origin literal in a code line of `commands.rs` / `pause.rs` — **met** (probe `last line 0`, exit 1).
- Boundary guard `last line 0` against `f5076ad`; `Cargo.lock` 562; `cargo build -p conductor-tauri --bins` exit 0 —
  **met**.
- `cargo audit` and `cargo deny check advisories bans licenses sources` exit 0, advisory-db porcelain empty first — **met**.
- `clippy -D warnings` and `fmt --all --check` exit 0 — **met**.
- Production handlers and their span guards untouched; no span/log/OTel added — **met** (diff confined to the two
  `#[cfg(test)] mod tests` blocks).
- CI: the operator pass's push reads `verdict: green` — **met**: CI#37166666634 on `84a3759`, the Rust job's Windows
  nextest green with the same 27 `conductor-tauri` tests (27 distinct test ids in its log, the new test among them), the
  `a11y` job green on `windows-2022`.

**Gates (implement P2, run dir `2026-10-04T00-57-10-implement`; every entry by its `run`):**
- `( cd crates/conductor-tauri/ui && { [ -d node_modules ] || npm ci; } && npm run build )` — green · exit 0
- `cargo nextest run -p conductor-tauri --profile ci` — green · exit 0 · `contains 27 tests run: 27 passed, 0 skipped`
  held (re-run green after the inverse control)
- `grep -cE '^retries = 0$' .config/nextest.toml` — green · exit 0 · last line 2
- `cargo test -p conductor-tauri` — green · exit 0
- `grep -vhE '^\s*//' … | grep -cE 'tauri\.localhost|tauri://localhost'` — green · exit 1 · last line 0 (re-run green)
- `cargo build -p conductor-tauri --bins` — green · exit 0
- `cargo nextest run --workspace --profile ci` — green · exit 0 (1197 passed)
- `cargo clippy --workspace --all-targets -- -D warnings` — green · exit 0
- `cargo fmt --all --check` — green · exit 0 (re-run green)
- `git diff --name-only f5076ad… -- crates Cargo.toml Cargo.lock deny.toml .config .github | grep -cvE …` — green ·
  exit 1 · last line 0
- `grep -c "^name = " Cargo.lock` — green · last line 562
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` — green · no output
- `cargo audit` — green · exit 0
- `cargo deny check advisories bans licenses sources` — green · exit 0
- `gate.py hygiene` (`leg = 'operator'`) — run by hand in the operator pass: exit 0 · `hygiene: clean` (evidence file)
- the guarded push (`leg = 'operator'`) — exit 0 · `PUSHED_SHA=84a37591f890df82c6f262350a11f5a20c104e15` (evidence file)
- `ci.py conclusion --sha HEAD --wait 1500` (`leg = 'operator'`) — exit 0 · `verdict: green` · CI#37166666634 (evidence
  file)
- Smoke: skipped — no boot-path / UI-surface change (every edit inside `#[cfg(test)]`); the plan lists no smoke or
  self-verify entry.

**Watches:** none folded.

**Outcome basis:** the operator pass ran (pre-CI commit `84a3759`, the only commit since the base); its final HEAD's CI
run CI#37166666634 (green) is recorded in `evidence/operator-pass.md`. Implement's P2 gates were run on the identical
source tree (the pre-CI commit added only bookkeeping and the evidence file). Implement's P4 report is in this
conversation and is the basis for the gate verdicts and the inverse control.

**Process hygiene:** implement started only the gate run's cargo/npm/grep children and the inverse control's nextest —
all terminated (census measured with `ps` at implement P4); the operator pass's push and `ci.py` read both exited. No
pulse-app, no listener on `:4317`/`:4444`/`:4445` (`ss -ltnp`, measured). This wrap's own background code-graph refresh
is accounted at P4.
