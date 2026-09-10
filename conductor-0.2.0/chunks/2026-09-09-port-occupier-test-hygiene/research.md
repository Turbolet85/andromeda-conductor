# Codebase Research — 2026-09-09-port-occupier-test-hygiene

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 8 · **Graph queries:** 2 (rust plane)
- **Harness rules consulted:** `.claude/rules/testing.md` — read structurally (89 lines / 72 KB, multi-KB
  per entry): the introducer index over all 45 Session-Additions entries, then L17, L88 and L89 in full.
  `verification-harness.md` NOT read in full — this chunk names **no live leg** (its measurements are
  `cargo test -p`, not a harness run against a real process), so the live-leg firing-form rule does not apply.

## Files inspected
- `crates/conductor-faults/tests/port_occupier.rs` (full, 112 lines) — 7 `#[test]` fns in ONE binary. Six
  construct a `PortOccupier`; the seventh (`the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines`,
  :71-112) installs the process-global subscriber and asserts on the lines it writes.
- `crates/conductor-core/src/redact.rs` (:1-60) — `ALLOWLISTED_FIELDS`; **`"port"` is present at :51**,
  under the "fault-application span attributes (obs-plan §4)" group. The assertion has a real subject.
- `crates/conductor-core/src/obs.rs` (:60-140 + grep over the file, 789 lines) — `init_observability`
  (:73), `build_subscriber` (:157), `install_panic_hook` → `std::panic::set_hook(log_panic)` (:172).
- `crates/conductor-run/tests/canary_obs_witness.rs` (head + shape) — 1 test, dedicated binary.
- `crates/conductor-verify/tests/readback_shape_witness.rs` (head + shape) — 1 test, dedicated binary.
- `crates/conductor-timeline/tests/obs_span.rs` (head + shape) — 1 test, dedicated binary.
- `crates/conductor-faults/Cargo.toml` (full) — `[dev-dependencies]` is `serde_json` alone, already
  present for exactly this assertion.
- `scripts/code-graph-cookbook.md` (:17-91) — schema + canonical queries.
- `.claude/rules/testing.md` — as above.

## Graph impact (rust plane, `db_state` warm; both planes built)
- **`init_observability`** — `conductor-core 0.1.0 obs/init_observability().` @ `obs.rs:73` (editor line;
  graph `def_line` 72 is 0-indexed). The process-global install site.
- **`ALLOWLISTED_FIELDS`** — `conductor-core 0.1.0 redact/ALLOWLISTED_FIELDS.` @ `redact.rs:21`.
- **`PortOccupier`** — `conductor-faults 0.1.0 port_occupier/PortOccupier#` @ `port_occupier.rs:21`.
- **Crate edges:** `conductor-faults → conductor-core` (outbound) and `conductor-run → conductor-faults`
  (the single inbound consumer). This chunk's delta is **test-target-only**, so its cross-crate blast
  radius is zero — no `src/` symbol changes, no signature changes, no caller threading.

## The mechanism, re-derived at HEAD (measured, not inherited)
The route entry's claim carried `measured at 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`, so
its evidence pointer was spot-checked. It holds, and the measurement refines it to a finer grain:

- `cargo test -p conductor-faults` → **exit 101**, `6 passed; 1 failed` (unit lib target: 23/23 green).
  Byte-consistent with the entry's three measurements.
- **The failing assertion is `the bound port survives the allowlist`** (`port_occupier.rs:104-108`).
- **Why:** the test's log file holds **14 lines carrying 13 `fault.port_occupier` records** — six `new`
  records with six distinct ephemeral ports (`60250, 60251, 60252, 60253, 60254, 60255`) plus six `close`
  records. Its own occupier contributes exactly one `new`. The finder at :90-100 is
  `.find(|o| span == "fault.port_occupier" && span_event == event)` — **first match wins** — and returned
  port `60254`, which is a *sibling's* occupier. `assert_eq!` then compares a sibling's port against this
  test's own and fails.
- **Why nextest is green:** process-per-test means exactly one `new` record exists in the process, so the
  first match is trivially the right one. **Why `--test-threads=1` is green:** serial execution means no
  sibling occupier is alive while the subscriber is installed.
- So the defect is not "a race" in the loose sense — it is **first-match record selection over a shared
  process-global sink**, and it is deterministic in kind and nondeterministic only in which sibling wins.

**Corollary discovered while measuring — the failure is SILENT.** `init_observability` installs
`log_panic` as the process panic hook (`obs.rs:172`), which routes the panic through `tracing` into the
file sink. The libtest failure block therefore carries **no `---- stdout ----` section and no panic
message**, and `-- --nocapture` adds nothing. The assertion text was recoverable only from the test's own
JSONL (`%TEMP%/conductor-faults-obs-{pid}/agent-latest.jsonl`, which the test deletes on success only, so
leftovers accumulate — 5 such directories were present). A maintainer hitting this red sees a bare test
name and nothing else.

## The four "unmeasured" crates — MEASURED
Run at HEAD, each exit code captured from the bare command before any pipe:

| Crate | `cargo test -p <crate>` | Result |
|---|---|---|
| `conductor-verify` | exit **0** | green (incl. `readback_shape_witness`) |
| `conductor-run` | exit **0** | green (22 targets) |
| `conductor-cli` | exit **0** | green |
| `conductor-tauri` | exit **0** | green |
| `conductor-report` | exit **0** | green (51 + 3 passed) — **not one of the entry's eight**; see below |

The route entry warned that "fixing the fault crate may simply advance the first red rather than turn the
loop green." **Measured false at HEAD:** `conductor-faults` is the only red.

**A second hole, found at the P5 review (operator).** The wrap gate's loop enumerated only the **eight**
crates the formatting pass touched, but the workspace has **nine** members (`cargo metadata --no-deps`), so
`conductor-report` was outside the gate entirely — the same blind spot as a short-circuit, by a different
mechanism, and `test-plan` §4 states the contract per crate rather than per touched crate. Measured green
here. The plan's sweep therefore derives its list from the manifest and covers all nine.

## Patterns detected
- **One-test-per-binary for a process-global-singleton test** (`conductor-run/tests/canary_obs_witness.rs:1`,
  `conductor-verify/tests/readback_shape_witness.rs:1`, `conductor-timeline/tests/obs_span.rs`): all three
  crates that call `init_observability` from an integration test already isolate that test in its own
  `tests/*.rs`, each holding **exactly one** test fn — all three `#[tokio::test]`
  (`canary_obs_witness.rs:23`, `readback_shape_witness.rs:26`, `obs_span.rs:15`), **not** plain `#[test]`,
  so a literal `grep -c '#\[test\]'` over them returns 0 and is the wrong probe for the property. Two state
  the rule in their own doc comments —
  `readback_shape_witness.rs:1-5` reads "alone in its own test binary… serializing the file would hide the
  shared state rather than remove it (test-plan §11). **One test per binary holds under both runners.**"
  `conductor-faults/tests/port_occupier.rs` is the **only** violator of a convention already applied three
  times in-tree.
- **The subject's own siblings are the contaminant** (`port_occupier.rs:11-66`): **five** of the six
  siblings construct a `PortOccupier` — `:12`, `:24`, `:34` (two `occupy` calls, the second deliberately
  failing to a typed error, so one successful bind), `:43`, `:53` — while `:62`
  (`default_target_is_the_otlp_ingest_port`) constructs nothing and only asserts the constant. The occupier
  emits its span at its RAII bind site inside `conductor-faults`, so any sibling running concurrently with
  the installed subscriber writes into the same file. The six `new` records are those five plus the
  witness's own occupier.
- **`close` records carry no attributes** (measured: `port=None`, `fault_type=None`), matching obs-plan §4's
  "attributes on the `new` record alone" — so `record("close")` at :109 asserts existence only and is
  unaffected by the selection defect.

## Conventions to follow
- **Doc-comment the isolation reason at the top of the new file**, naming the process-global mechanism and
  test-plan §11 — the shape both `canary_obs_witness.rs:1-6` and `readback_shape_witness.rs:1-5` use.
- **Naming:** `{subject}_{aspect}_witness.rs` (`canary_obs_witness.rs`, `readback_shape_witness.rs`).
- **No manifest change needed:** `serde_json` is already a `[dev-dependencies]` entry of `conductor-faults`
  and dev-deps apply to every test target in the crate.
- Cite graph lines as `line + 1` (SCIP ranges are 0-indexed; cookbook §Reading the result).

## New files to create
- `crates/conductor-faults/tests/port_occupier_span_witness.rs` — the single test
  `the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines`, moved verbatim from `port_occupier.rs`,
  with the isolation-reason doc comment. Its assertion is carried across **unchanged**, which is what
  preserves its power to fail if `port` leaves the allowlist.

## Files to modify
- `crates/conductor-faults/tests/port_occupier.rs` — remove the moved test (:68-112) and the imports it
  alone needs (`conductor_core::{ObsSink, init_observability}`, `serde_json::{Map, Value}`; `TcpListener`,
  `IpAddr`, `Ipv4Addr` stay — the six remaining tests use them). No other change.
- No `src/` file, no `Cargo.toml`, no `.config/nextest.toml`, no `ci.yml`. Zero cross-crate blast radius
  (crate-edge query above).

## Open questions
- **Should the moved test's temp-dir key change?** It is `conductor-faults-obs-{process::id()}`; leftovers
  accumulate because the cleanup at `:111` is skipped on panic. → blocks: `implementation-scope` (a
  cosmetic hygiene call for /implement; it does not affect the fix's correctness either way).
- **Does the new binary need a known-positive control?** `testing.md:89` requires a criterion's probe to be
  able to FAIL on the property's negation. → blocks: `plan-decision` — resolved in P4: the plan carries a
  control step (temporarily drop `"port"` from `ALLOWLISTED_FIELDS`, observe the moved test go red, revert)
  rather than asserting preservation by inspection.

## Finding surfaced for another entry (not this chunk's to fix)
`scope.md` flagged the sibling entry's panic-hook CARRY as possibly the same defect. **It is not** — that
CARRY is about two `take_hook`/`set_hook` pairs in `obs.rs`'s own unit tests racing each other, whereas
this chunk's defect is record selection over a shared sink. But the CARRY's **line coordinates are stale**:
it names `obs.rs:491-496` and `:532-537`; at HEAD the two pairs are at **`:535-540`** (in
`panic_hook_emits_one_error_line`) and **`:582-587`** (in `panic_hook_redacts_host_path_in_payload`) — a
~44-50 line shift consistent with the formatting pass. The mechanism it describes is real and still
present; only the coordinates moved. Recorded here for the *Release build and bundle* entry
(`working-route.md:133`) that owns it.
