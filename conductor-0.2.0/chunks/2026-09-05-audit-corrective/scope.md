# Scope — 2026-09-05-audit-corrective

**Working entry (title, verbatim):** Audit corrective — render.rs survivors dispositioned, civil_from_unix boundary-date kill,
jsonrpc.rs timeouts owned, M2's three dedups, and conductor-run/src/lib.rs split along named seams with its inline tests
moved to tests/
**Epoch:** 6b — Polish & ship (its head; minted by operator directive at the 2026-09-05 0-pending adaptation —
`.andromeda/runs/2026-09-05T16-46-25Z-wrap/adaptation-record.md`; the founder ruled the `lib.rs` split INTO this chunk).
**Kind:** code-audit corrective. No capability claimed (like the two Epoch 6a disposition chunks); no live Pulse needed; a
source-touching Rust chunk, so the workspace gates run in full (no gate deferral).
**Basis artifact:** `.andromeda/runs/2026-09-05T08-35-52-code-audit/` — `proposals.md` (M1 · M2 · Informational · Skips ·
Notes), `c-mutation-conductor-{cli,core,verify,tauri,run}.json`, `c-duplication.json` (all 90 pairs), `record.json`
(the 4th `code-metrics.ndjson` record; trend at HEAD `59d5b7c` vs baseline `29c30b0`).
**Scope premise closure:** performed at P3 (2026-09-05) — every `[inferred]` bullet below is now VERIFIED (tag dropped) or
`[premise-corrected: …]`; two intent-incomplete facts were ADDED (marked `[added at P3]`). `research.md` carries the basis.

## Goal
Close the boundary-#4 code audit's code-facing findings at their defects, in one chunk, so Epoch 6b's polish work starts from
a measured-clean base: every named mutation survivor in the first-audited `conductor-cli` unit ends killed or
accepted-deliberate against a cited rule; the `civil_from_unix` arithmetic survivors are killed at a boundary date; the two
`jsonrpc.rs` timeout mutants get an owner; the three clone families M2 named are deduplicated; the composition root is split
along the seams it already names with its inline tests moved out of `lib.rs`; and the standing cargo-audit deferral is
closed by this chunk's wrap on a probe that reads the world's state.

## Work items (WHAT, with boundaries)

### W1 — `conductor-cli` survivors dispositioned (the `render.rs` block and the unit's other nine)
- Audit fact (proposals.md §Informational + §First-audited units; `c-mutation-conductor-cli.json`): first-ever cli score
  **58.59** — 117 mutants: 58 caught / 41 missed / 18 unviable; **32 of the 41 in `crates/conductor-cli/src/render.rs`**
  (`lamp_code` ×2 · `paint` ×2 · `hold_line` ×2 · `envelope_caption` ×3 · `coverage_summary` ×2 · `stdout_color` ×2 ·
  `stderr_color` ×5 · `coverage_table_styled` ×1 · `coverage_summary_styled` ×5 · `wire` ×2 · `latency` ×2 ·
  `fingerprints` ×4). The other nine are the same UNIT's: `main.rs:55 hint_for` ×2 · `commands/mod.rs:29 exit_code` ·
  `paths.rs:73` (`!` deleted in `Paths::load_all_scenarios`) · `paths.rs:130 env_seed` · `pause.rs:31` / `:104` `kind`
  ×4. All 41 are dispositioned — test-plan §12 records rosters per UNIT, not per file.
- Outcome: each of the 41 ends **killed** (a new or sharpened assertion) or **accepted-deliberate against a cited rule**,
  recorded in `evidence/disposition-ledger.md` in the shape the two Epoch 6a chunks set
  (`2026-09-03-conductor-tauri-survivors-dispositioned` · `2026-09-03-conductor-run-composition-root-survivors-dispositioned`);
  the accepted set lands in test-plan §12 at wrap (Expected amendment 1). The cli mutation re-run's `missed.txt` equals
  the accepted set exactly (the §10 rule, reconciled to the accepted SET at the tauri chunk's wrap).
- VERIFIED (render.rs:153-165): `stdout_color` / `stderr_color` read `IsTerminal` on the process's own handles AND
  `NO_COLOR` AND `TERM`. The three-condition logic is testable through a PURE seam (a function of
  `(is_terminal, no_color_set, term)` the two wrappers call with the live values), which kills the `&&`/`!=` mutants
  inside it; the wrappers' own constant-return mutants (`stdout_color -> false`, `stderr_color -> false`) remain
  observable ONLY with a real terminal on the process's handles — the read-the-host-boolean class the 2026-09-04
  `sidecar_resolves_on_path` acceptance already established (test-plan §12; a host-dependent kill would make the roster
  mean different things on different hosts). `stderr_color -> true` IS killable: a piped subprocess (assert_cmd) whose
  stderr must carry no escape byte. No env override is introduced (arch: no new `CONDUCTOR_*` handle).
- `[premise-corrected: render.rs's tests exercise the `*_styled(…, color)` cores only — no test calls the public wrappers
  `paint` / `hold_line` / `envelope_caption` / `coverage_summary`, so their bodies never execute under test]` The kill is
  therefore a call to each PUBLIC wrapper asserting content PRESENCE (the `[HOLD]` prefix + scenario · the
  `[ENVIRONMENT-SUSPECT]` label + cause · the manifest-derived roll-up shape · `paint`'s text) — presence assertions hold
  under both a tty and a pipe, so they are runner- and host-independent; escape-ABSENCE is asserted only through a piped
  subprocess. `wire` / `latency` / `fingerprints` die to results-table content (`<5s`, `1840`, `fp-1`, `(none)`);
  `coverage_table_styled:270` and `coverage_summary_styled:291/293/299` die to assertions that name WHICH cell carries
  the residual-mute tint and that the in-scope breakdown equals the string DERIVED from `coverage_matrix()` (never a
  literal); `lamp_code` dies to the closed by-name code set (114 / 203 / 179 / 146 / 246 / 60, design-system §Surface:
  cli / Tokens); `hint_for` dies to the hint TEXT on the "no scenario matches" edge (`cli_smoke.rs:140` asserts only the
  `hint:` label today); `exit_code` dies to a unit test in `commands/mod.rs` feeding a `Fail` record (test-plan §1 exit
  contract; no live Pulse can produce one through the binary); `kind` ×4 die to unit tests on the two `PauseResolver`
  impls.
- VERIFIED (security extract, §Input Validation env-var + CLI rows): `paths.rs:73` and `paths.rs:130 env_seed` are the
  ONLY two of the 41 inside the forbidden domain — must-kill (a subprocess run by name loads exactly the named scenario
  and no other; `CONDUCTOR_SEED=7` reaches the journal's `seed`, and `--seed` beats it). `exit_code` is must-kill by
  test-plan §1 (exit non-zero only on a hard Fail), not by security. Every other survivor is a design/layout invariant
  (never color-alone; manifest-derived roll-up) — killable, and killed.

### W2 — `civil_from_unix` boundary-date kill
- Audit fact: `conductor-core` shard 1/4 (126 of 504 mutants) scored 95.24; all 6 survivors are arithmetic mutants at
  `crates/conductor-core/src/obs.rs:224-226` inside `civil_from_unix` (Howard Hinnant's `civil_from_days`, the
  `now_rfc3339()` stamp behind `journal_emitted_at` — obs.rs:200-206; NOT the self-obs base line's `timestamp_ms`, which is
  epoch millis from `unix_millis()`).
- `[premise-corrected: two of the six are UNKILLABLE by any input — `:224:41` (`z - 146_096` → `+`, `/`) sits in the
  `else` arm of `if z >= 0`, and `secs: u64` makes `z = days + 719_468 ≥ 719_468` always, so the negative-era branch is
  dead code]` Disposition: delete the dead branch (`let era = z / 146_097;`, comment naming the unsigned domain) — a
  behaviour-preserving change under which the two mutants cease to exist; never an acceptance for code that can be
  removed. The four live mutants (`:226:33` `+ doe/36_524` → `-`, `*`; `:226:48` `- doe/146_096` → `+`, `/`) die to
  boundary dates: `2000-02-29T00:00:00Z` (the last day of an era, `doe = 146_096`) separates the 400-year term;
  `2100-03-01T00:00:00Z` (`doe = 36_524`, the first day the 100-year term is non-zero) separates the 100-year term —
  plus `2000-03-01` (era start) and `1999-12-31` as the era's neighbours. Shape: rstest `#[case]` rows on the pure fn.
- Outcome: the file-scoped re-run (`cargo mutants -f crates/conductor-core/src/obs.rs …`) shows 0 missed inside
  `civil_from_unix`; `civil_from_unix_known_values` (obs.rs:482) stays green.
- Boundary: core shards 2–4 (378 mutants) stay UNTESTED — a full core score is a future audit's; only the file-scoped
  re-run is in scope.

### W3 — `jsonrpc.rs` timeouts owned
- Audit fact (proposals.md §First-audited units + §Notes): `crates/conductor-verify/src/jsonrpc.rs:49:30` (`!=` → `==`
  in `JsonRpcSession::request`) and `:70:9` (`JsonRpcSession::write_message` → `Ok(())`) are TIMEOUT mutants — the test
  binary hangs under them until cargo-mutants' cap; they have crossed two boundaries unowned.
- VERIFIED mechanism (jsonrpc.rs:44-59, :69-75; `tests/common/mod.rs::serve_stub` echoes the request id): under `==` the
  loop `continue`s past every matching response and blocks in `read_line` forever; under `Ok(())` nothing is written, the
  stub never answers, `read_line` blocks forever. Both are HANGS, not unobserved returns (testing.md 2026-09-03).
- Outcome: OWNED test-side — a bounded await in the tests that drive `request` over the duplex / the spawned child
  (`tests/jsonrpc_correlation.rs` · `readback.rs` · `preflight.rs` · `readback_shape_witness.rs` · `preflight_spawn.rs`,
  via a `tests/common` helper), so each mutant becomes a FAIL and is KILLED; the conductor-verify re-run shows timeouts
  2 → 0 and both sites in `caught.txt`. The bound lives in test code ONLY: conductor-verify's `[dependencies]` tokio
  lacks the `time` feature (`Cargo.toml:10`; dev-deps carry it at `:31`), and repairing that manifest is *Dependency
  polish*'s (arch [Module Boundaries] qualifier) — a shipped `tokio::time::timeout` inside `jsonrpc.rs` is out of scope.
  `.config/nextest.toml` sets no `slow-timeout`, so no runner knob substitutes for the bound (and adding one would be the
  serialize-to-hide anti-pattern the zero-flake rule forbids).

### W4 — M2's three dedups
- Audit fact (proposals.md §M2; `c-duplication.json` stores all 90 pairs): clone pairs 84 → 86 → 90; the three families:
  (a) `crates/conductor-tauri/src/commands.rs` — VERIFIED: the 13-line pair at 173/232 is the `run_report` / `run_envelope`
  prologue (`runs_dir()?` → `resolve_under`-guard a supplied `run_id` / else `latest_run_id`, each `Err` through
  `sanitize_error`, the no-run arm logging and returning the empty value); the 12-line pair at 409/508 is the TEST
  helpers `invoke` / `invoke_expecting_error` building the same `InvokeRequest`. → ONE production prologue helper + ONE
  test-side request builder; the eight `tracing::info_span!("tauri.command.<name>").entered()` guards stay per handler
  (obs-plan §4), the mapped error strings stay byte-identical (the redaction boundary);
  (b) `crates/conductor-cli/src/render.rs:285-314` ↔ `crates/conductor-report/src/coverage.rs:82-101` — VERIFIED
  identical roll-up arithmetic (per-mode counts in `CoverageMode::ALL` order, the in-scope subtotal, the `(N unbacked)`
  qualifier on the auto term, the out-of-scope token), differing only in RENDERING (`**Capabilities**` bold + Markdown
  vs a plain "capabilities" caption with the out token tinted) — so the shared-formatter arm applies, never
  accept-and-record: ONE roll-up computation in `conductor-report` consumed by both renderers over the existing
  `conductor-cli → conductor-report` edge (`crates/conductor-cli/Cargo.toml:14`). obs-plan §4 (`:359`) names
  `summary_line` and `coverage_summary` by name — both entry points KEEP their names (thin wrappers over the shared
  computation), so no obs-plan rename amendment is owed;
  (c) a `crates/conductor-run/tests/common/mod.rs` fixture (no `common/` exists at HEAD) for the `Capture` +
  `start_stub` loopback-collector scaffolding shared by `dispatch_wire.rs` / `canary_wire.rs` / `canary_obs_witness.rs`
  — a shared MODULE only; `canary_obs_witness.rs` stays its own binary (test-plan §11: the process-global subscriber).
- Outcome: the named pairs no longer appear in a jscpd run over the workspace (jscpd 5.0.16 on PATH; the audit's form
  `jscpd crates --format rust --reporters json --output <dir> --silent`); behaviour byte-unchanged.
- Boundary: the other 87 pairs are NOT this chunk's; the clone count may still exceed 84 afterwards — the acceptance is
  the three families GONE, never a count target.
- VERIFIED (audit `c-mutation-conductor-tauri.json` at HEAD vs test-plan §12 :610): the tauri roster's coordinates are
  ALREADY stale before this chunk — §12 says `run_thread:296` / `start_run:263:8`; HEAD reads `commands.rs:307:5` /
  `:272:8` (the 2026-09-04 doc-comment edits moved them). W4a moves them again → re-pointed at wrap by function + column
  (Expected amendment 1); the tauri re-run must show the same three accepted mutants and 0 new survivors.

### W5 — `conductor-run/src/lib.rs` split along named seams, inline tests moved out
- Audit fact (proposals.md §M1): 1012 → 1115 → **1423** code lines (+27.6 % this boundary; hotspot #1; home of the 5
  accepted-deliberate `conductor-run` survivors; max cognitive 8). At HEAD `wc -l` reads 1944 lines; the inline
  `#[cfg(test)] mod tests` runs `:1006`–EOF (52 test fns); `dispatch.rs` is the only sibling; `tests/` holds 16 test
  binaries (`baseline_harvest` · `canary_obs_witness` · `canary_wire` · `connection_harvest` · `delegated_timing_harvest`
  · `dispatch_wire` · `envelope_fixture` · `lamps_fixture` · `lifecycle_harvest` · `lifecycle_live` ·
  `operator_pause_harvest` · `pii_harvest` · `restart_harvest` · `run_contract_pin` · `severity_harvest` · `storm_harvest`).
- Outcome: `lib.rs` becomes a thin crate root (`mod` + `pub use` + the crate docs) over sibling modules cut along the
  seams the route names — the **preconditions / run-contract observation** half (`load_run_contract` ·
  `observe_run_contract` · `declares` · `observe_preconditions`), the **canary warm-up / gate** half (`Preflight` ·
  `preflight` · `readiness` · `unreachable_state` · `CANARY_STORM_COUNT` · `CANARY_SERVICE_NAME` · `canary_storm_seed` ·
  `canary_warmup_seed` · `canary_spec` · `emit_canary_storm` · `canary_gate` · `emit_canary` · `warm_up_canary_service` ·
  `canary_poll` · `canary_blocked_state`), the **lifecycle / harvest helpers** (`LifecycleObservation` ·
  `LifecycleVerdict` · `select_resolve_target` · `AUTO_RESOLVE_IDLE_SECONDS` · `attribute_by_liveness` ·
  `evaluate_lifecycle` · `probe_resolve_lifecycle` · `active_incident_ids`), the **scenario-execution core**
  (`execute_scenario` · `ScenarioOutcome` · `route_read_back` · `state_for` · `manual_record` · `severity_rank` ·
  `now_ms` · `classify_fault` · `ramp_factor` · `fault_span` · `phase_guard` · `now_unix_nanos`), the **envelope /
  persistence** trio (`persist` · `read_envelope` · `classify_run`) and the **live driver** (`RunEvent` · `RunStage` ·
  `drive_run`). The crate's PUBLIC API is byte-stable — the code-graph measures 39 src-defined symbols referenced from
  outside `src/` (15 fns · 3 consts · the `Dispatcher` / `LifecycleObservation` / `LifecycleVerdict` / `RunEvent` /
  `RunStage` types with their variants and fields; research.md §Graph impact) — every one keeps its `conductor_run::`
  path via `pub use`, so `conductor-cli`, `conductor-tauri` and the 16 test binaries compile UNCHANGED.
- VERIFIED: the inline tests reach crate-PRIVATE items heavily (`route_read_back` ×6 · `declares` ×6 · `now_unix_nanos` ×5
  · `classify_fault` ×5 · `ramp_factor` ×5 · `phase_guard` ×4 · `state_for` ×3 · `manual_record` ×3 · `now_ms` ×3 ·
  `fault_span` ×3 · `observe_run_contract` ×2 · `active_incident_ids` ×2 · `warm_up_canary_service` ×1). The rule: a
  test of a private item moves WITH its item into the owning sibling's `#[cfg(test)] mod tests` (out of `lib.rs`, not
  into `tests/`); a test of `pub` items only (`execute_scenario` blocks · `drive_run` streams/aborts · `persist` ↔
  `read_envelope` round trip · `classify_run` · the envelope-suspect runs) moves to `tests/`. Both runners stay green
  (test-plan §4 runner portability).
- `[added at P3]` **The self-obs `target` field moves with the code** (obs-plan §3 — `target` is the emitting module path,
  an allowlisted identity field): lines emitted by relocated code read `conductor_run::<module>` instead of
  `conductor_run`. Span NAMES and the sibling topology (`scenario.run` at `execute_scenario`; `report.generate` /
  `db.insert_run` run-scoped siblings) are byte-identical; ONE test pins the old target literal —
  `crates/conductor-run/tests/severity_harvest.rs:454` (`"target":"conductor_run"` on the declare-only read-back line) —
  and is relaxed to the `conductor_run` PREFIX in the same step (the fixture strings at `baseline_harvest.rs:180` / `:328`
  are test DATA, untouched). Pre- and post-split `target` sets are recorded in evidence.
- `[added at P3]` **test-plan §12 carries FOUR accepted-deliberate coordinate groups inside `lib.rs`, all current at
  HEAD and ALL moved by the split** — `:362:5` (`declares`, §12 :609) and classes B/C at `:559:27` + `:600:25` ×2 +
  `:68:8` (§12 :611; verified: `:68 if !state.ready`, `:559 Observation { degraded: true, …}`, `:600 observed_ms -
  emitted_ms`). Expected amendment 1 re-points ALL of them by function + column (the §12-recorded method), not `:362:5`
  alone as the route freight said.
- Boundary: NO behaviour change; the 5 accepted-deliberate survivors keep their classification; `dispatch.rs` untouched;
  the a11y seed binaries `envelope_fixture` / `lamps_fixture` keep writing through the production `persist` (a11y-plan
  §4/§9). The `sizes.file_max` / `over_800` movement is the NEXT audit's measurement, not an acceptance (a count target
  invites a cosmetic split).

### W6 — the migrated standing cargo-audit deferral (PREREQ, folded)
- Folded from the route entry's `PREREQ (standing, external decay)` — FULL form, basis re-verified at the 2026-09-05
  adaptation (adaptation-record.md item 2). Every named coordinate was re-verified against the artifact THIS session and
  HEAD is unchanged since (`7418f9d` touched no code): upstream `e12b689b` 2026-08-09 16:57:02 +0200 "Move
  RUSTSEC-2026-0244 to correct crate (#3128)" · local `$CARGO_HOME/advisory-db` (`D:/dev/rust/cargo` on this host) HEAD
  `5a0ebedf` · pre-clean `?? crates/gettext-sys/RUSTSEC-2026-0244.md` · post-clean `status --porcelain` empty ·
  `cargo audit` exit 0 — 1239 advisories · 564 crate dependencies · `warning: 18 allowed warnings found` (17 unmaintained +
  1 unsound) · `cargo deny check advisories bans licenses sources` exit 0 · CI run 33954347685 step "Supply-chain — cargo
  audit" success.
- VERIFIED (mechanism): *cargo-audit's fetch INTO the existing copy never removed it* — the pre-clean state was an
  UNTRACKED file (`??`) beside a git HEAD equal to upstream's; a fetch + checkout never deletes untracked files, and
  cargo-audit fetches into the existing `$CARGO_HOME/advisory-db` clone rather than re-cloning. Basis: the observed
  `git status --porcelain` + `git log` state and git's checkout semantics — not a read of cargo-audit's source.
- `[added at P3]` **test-plan's stated gate form is RED by construction:** §9 Supply-chain stage (`:456`), §10 (`:478`,
  `:508`), §11 (`:567`) and §Plan (`:216`) all say `cargo audit --deny warnings`; measured 2026-09-05: that form exits **1**
  (`error: 18 denied warnings found!`) because the 17 unmaintained + 1 unsound advisories are the set `deny.toml`
  adjudicates, while CI (`.github/workflows/ci.yml:75`) runs the bare `cargo audit`, which exits 0. This chunk's gate is
  the bare form (the one CI enforces; security.md 2026-06-23 explains the audit/deny split); the five test-plan sites are
  Expected amendment 6 (the operationalize-the-gate rule, playbook :88).
- This chunk's `## Test Commands` lists the probe as TWO ordered steps — `git -C $CARGO_HOME/advisory-db status
  --porcelain` (must print nothing) THEN `cargo audit` (expected exit **0** with the 18-warning summary; exit read directly,
  never through a pipe; output redirected INTO `target/`) — plus the deny overlap. Its wrap CLOSES the deferral per
  route-resolve §Deferred-gate closure ("deferral since 2026-08-09 closed") and re-bases the two master sites that still
  state the retired fault (Expected amendment 3). Any RED at take-up or wrap is a NEW event (a fresh advisory against the
  static tree, or fresh residue) — surfaced, never re-pinned as the old signature.

## Surfaces / contracts touched
- Code: `crates/conductor-cli/src/{render.rs, paths.rs, pause.rs, commands/mod.rs}` + `crates/conductor-cli/tests/cli_smoke.rs`
  (W1; `main.rs` is asserted through the binary, not edited) · `crates/conductor-core/src/obs.rs` (W2) ·
  `crates/conductor-verify/tests/{common/mod.rs, jsonrpc_correlation.rs, readback.rs, preflight.rs,
  readback_shape_witness.rs, preflight_spawn.rs}` (W3 — `jsonrpc.rs` itself is NOT edited) · `crates/conductor-tauri/src/commands.rs`
  (W4a) · `crates/conductor-cli/src/render.rs` + `crates/conductor-report/src/coverage.rs` + `crates/conductor-report/src/lib.rs`
  (W4b; one new `pub` roll-up item re-exported) · `crates/conductor-run/tests/{common/mod.rs (new), dispatch_wire.rs,
  canary_wire.rs, canary_obs_witness.rs}` (W4c) · `crates/conductor-run/src/{lib.rs, six new sibling modules}` +
  `crates/conductor-run/tests/{new integration file(s), severity_harvest.rs}` (W5).
- No new crate · no new dependency edge (cli→report exists; conductor-run's edges unchanged) · no new env handle · no port ·
  no expected `Cargo.lock` package delta (564 → 564; an EDGE line moves only if a dev-dep is added — none planned) —
  `cargo deny` over the lock at wrap is the check.
- Contracts: test-plan §4 (mutation instrument) · §10 (survivor disposition) · §12 rosters (a NEW cli roster; the
  conductor-run FOUR groups + the conductor-tauri pair re-pointed) · security-plan §Input Validation forbidden domain
  (must-kill) · obs-plan §3/§4/§11 (span names + topology byte-identical; `target` values recorded) · architecture
  §Module Boundaries (the conductor-run seam's public API unchanged; the standalone-build qualifier untouched) · the
  redaction boundary (tauri command error strings unchanged; `Display` never `Debug`).
- Harness: `cargo nextest run --workspace --profile ci` · `cargo test -p <crate>` for each touched crate · `cargo test
  --workspace --doc` · `cargo clippy --workspace --all-targets -- -D warnings` · `cargo mutants` per unit / file (the
  audit's form: `--test-tool=nextest --jobs 2`, `--output` per-run unique under `target/`) · jscpd 5.0.16 · the audit
  probe two-step + `cargo deny` · the obs conformance gate · `bash scripts/agent-run.sh status` (boot-path smoke:
  `conductor-run/src/lib.rs` and `conductor-cli/src/paths.rs` are boot paths).

## Out of scope
- conductor-core mutation shards 2–4 (378 mutants) — a future audit's; only the `obs.rs`-scoped re-run for W2.
- The other 87 clone pairs; any clone-count or file-size target.
- conductor-verify's 12 standing survivors (8 in the `stub_pulse_mcp` test-stub binary, `client.rs:157`, …) and the two
  accepted `spawn.rs` mutants — dispositioned at their chunks; a shipped time bound in `jsonrpc.rs`.
- The `knip` install and the `tokio time` dev-only feature fix (CARRYs on *Dependency polish*); the ps1 `--e2e` printed
  verdict (CARRY on *Release build and bundle*); the A5 web column.
- Any behaviour change in the run seam, the CLI's output text, or the Tauri commands' error strings.

## Expected amendments (wrap)
1. **test-plan §12** — a NEW conductor-cli accepted-deliberate roster (W1's accepted set, each with its rule); the
   conductor-run roster coordinates `:68:8` · `:362:5` · `:559:27` · `:600:25` ×2 → their new `<module>.rs:line:col` sites
   (`:609` + `:611`, W5); the conductor-tauri pair `run_thread` / `start_run` → post-W4a coordinates (`:610`, already
   stale at HEAD: `:307:5` / `:272:8`).
2. **test-plan §4 / the per-unit score records** — cli's post-disposition score; conductor-verify timeouts 2 → 0.
3. **architecture.md:196 + security-plan.md:177** — the retired "external advisory-DB fault" re-based to the measured
   cache-residue cause when the deferral closes (W6).
4. **obs-plan §3 / §9 prose, only if it enumerates `conductor_run` as a literal `target` value** — research found none
   (the field is described as a module path); the `target` change lands in evidence, not in a master.
5. obs-plan §11 bounded span names — none change; the conformance gate is the proof.
6. **test-plan §9 Supply-chain audit stage (`:456`), §10 (`:478`, `:508`), §11 (`:567`), §Plan (`:216`)** — `cargo audit
   --deny warnings` → the bare `cargo audit` CI runs, with the reason (the `--deny warnings` form fails on the
   `deny.toml`-adjudicated unmaintained set; the audit/deny split is the gate design).

## Acceptance sketch (the plan concretizes)
- Ledger: all 41 cli survivors killed / accepted with a cited rule; the cli re-run's `missed.txt` equals the accepted set;
  conductor-verify re-run shows 0 timeouts with both `jsonrpc.rs` sites in `caught.txt`; the `obs.rs`-scoped core re-run
  shows 0 missed inside `civil_from_unix`; the tauri re-run shows the same three accepted mutants and 0 new.
- jscpd: the three named families absent; no NEW pair introduced by the split.
- Split: `conductor-run`'s public item set identical before/after (the 39-symbol external surface + every other `pub`
  item, listed from the code-graph `symbol` table before and after); both bins + the 16 test binaries compile unchanged;
  the nextest count is non-decreasing (853 → ≥ 853 — moved tests keep their names); span names + topology unchanged and
  `target` sets recorded.
- Gates: workspace nextest · per-crate `cargo test` · doctest · clippy · obs conformance green; audit probe two-step exit 0
  + `cargo deny` exit 0 (closing the deferral); `Cargo.lock` package count 564.

## Evidence shape
`chunks/2026-09-05-audit-corrective/evidence/` — `disposition-ledger.md` (per survivor: site · mutation · disposition ·
killing test or cited rule), the mutation run tallies per unit (`missed.txt` · `timeout.txt` · `caught.txt` counts, read
from the nested `mutants.out/`), the jscpd before/after pair lists for the three families, the conductor-run public-API
before/after listing, the pre/post `target` value sets, and the audit probe transcript (porcelain output · `cargo audit`
exit + summary line · `cargo deny` exit) — every artifact grepped host-path-free before commit.
