# Scope — Mutation tier restored for conductor-tauri

**Marker:** `2026-09-02-mutation-tier-restored-for-conductor-tauri`
**Epoch:** 6a — Verification follow-ups (the corrective FIRST chunk, so every later 6a chunk runs on
hardened machinery)
**Working entry:** the first markerless line of `conductor-0.2.0/working-route.md` (Epoch 6a head)
**Source:** operator WRAP directive 2026-09-02, item 1 · `.andromeda/runs/2026-09-02T15-49-17-code-audit/proposals.md:35-77`
(`mutation-tier-aborted`, §M1)

---

## The problem this chunk closes

`cargo mutants -p conductor-tauri --test-tool=nextest` **planned 43 mutants and tested 0** — exit 4,
`cargo test failed in an unmutated tree, so no mutants were tested`. The epoch's most-touched unit
(13 of the 22 changed source files) therefore carries **no test-quality measurement at all**.

The abort is not a run-discipline miss: the audit's own invocation
(`.andromeda/runs/2026-09-02T15-49-17-code-audit/run-mutants.sh`) already passes `--test-tool=nextest`
and a workspace-root `--output`, i.e. it satisfies the test-plan §4 discipline in full. The
unmutated-tree baseline itself fails.

## Re-verified coordinates (the fold's named artifacts, checked against the artifacts themselves)

Every coordinate the working entry and the audit proposal name was re-read at HEAD this phase.
All hold; none needed correction.

| claim | verified at | verdict |
|---|---|---|
| failing test `commands::tests::path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db` | `crates/conductor-tauri/src/commands.rs:487` | exact |
| it is a unit test inside a BIN target (`#[cfg(test)]`) | `crates/conductor-tauri/src/commands.rs:342` (`mod tests` at `:343`) | exact |
| the crate has no `tests/` directory | `crates/conductor-tauri/tests/` → `No such file or directory` | exact |
| the CLI arm calls `assert_cmd::Command::cargo_bin("conductor")` | `crates/conductor-tauri/src/commands.rs:516` | exact |
| `conductor-tauri` declares NO dependency on `conductor-cli`; the only mention is a comment | `crates/conductor-tauri/Cargo.toml:26` — a comment line naming `conductor-cli`'s `cli_smoke`; the manifest's deps are core/run/serde/tauri/tokio/tracing and dev-deps tauri/tokio/assert_fs/serde_json/assert_cmd | exact |
| `conductor-cli` is the package producing the `conductor` binary | `crates/conductor-cli/Cargo.toml:8-10` — `[[bin]] name = "conductor"` | exact |
| 43 planned / 0 tested / exit 4 | `.andromeda/runs/2026-09-02T15-49-17-code-audit/c-mutation-abort-conductor-tauri.json` | exact |
| the audit ran WITH `--test-tool=nextest` | `run-mutants.sh` (same run dir) | exact — the discipline was correct |

The abort evidence additionally records the panic text, which the working entry does not quote and
which **names a candidate form directly**: `` `CARGO_BIN_EXE_conductor` is unset `` …
`help: if this is running within a unit test, move it to an integration test to gain access to
CARGO_BIN_EXE_conductor`.

## Mechanism claims folded from the entry (marker text preserved verbatim)

- `[inferred]` **"Cause measured at that proposal's end-to-end resolution chain"** — the test reaches
  the CLI through `assert_cmd`'s `legacy_cargo_bin` fallback to `target/debug/conductor.exe`, while
  `conductor-tauri` declares no dependency on `conductor-cli` at all. (Coordinates re-verified above;
  the causal chain itself is P3's to close — statedness is not measurement at HEAD.)
- `[inferred]` **"It is genuinely green under `--workspace` (which builds the sibling bin) and green
  under `-p conductor-tauri` against a PRE-EXISTING artifact dated 16:11; it aborts on a fresh target
  dir."** — the greenness is contingent on a sibling package having been built, not on anything the
  test or its manifest declares. (P3 re-measures the fresh-tree arm rather than inheriting it.)

## What this chunk builds

1. **A declared binding for the `conductor` binary** so the `conductor-tauri` test suite's CLI arm no
   longer depends on a build artifact that no Cargo dependency edge guarantees. **The FORM is
   deliberately unnamed here — it is P3-research territory.** Three candidate forms are on the table
   (the working entry names all three; the audit proposal names two of them):
   - the test **moved to the bin-owning package** (the entry's phrasing — that package is
     `conductor-cli`);
   - the test **moved to an integration-test target** (the proposal's (a), and what assert_cmd's own
     panic text prescribes);
   - an **artifact dependency**, or an **escargot-style build** at test time (the entry's third form).
2. **The mutation tier restored** — `cargo mutants -p conductor-tauri --test-tool=nextest` completes
   against an unmutated tree that passes, yielding a real tally for `conductor-tauri` (tested > 0),
   with every named survivor dispositioned per test-plan §10: killed, or classified
   accepted-deliberate against a cited rule.
3. **A runner-portability gate** proving the parity test on a **FRESH target dir** — so the binding is
   asserted rather than assumed, and the next fresh-tree invocation cannot silently regress.

## Acceptance (from the working entry, verbatim in substance)

> the tier yields a score for `conductor-tauri`, and a runner-portability gate proves the test on a
> FRESH target dir.

## Boundaries — what this chunk does NOT do

- **Not** re-litigating the parity assertion itself. `v2-25` is `verified` and its acceptance content
  is unchanged by this chunk; only the test's *binding to the CLI binary* (and possibly its
  *location*) is in play.
- **Not** promoting mutation to a CI stage — test-plan §9/§464 pins it as an operator/local
  instrument, never a blocking CI gate.
- **Not** introducing a numeric mutation threshold — test-plan §500 pins disposition-of-named-survivors
  as the acceptance, explicitly not a `--fail-under` number.
- **Not** touching the other two audited units (`conductor-run`, `conductor-verify`); the
  composition-root survivor disposition is the SECOND 6a entry, a separate chunk.
- **Not** amending the test plan — any spec movement this chunk measures is surfaced for wrap's
  amendment flow, never written here.

## Surfaces and contracts touched

- `crates/conductor-tauri/Cargo.toml` — dev-dependencies, and possibly a `[[test]]` target
- `crates/conductor-tauri/src/commands.rs` — the `#[cfg(test)] mod tests` block (the test may relocate
  out of it)
- `crates/conductor-tauri/tests/` — a new integration-test target, if that form wins
- `crates/conductor-cli/` — only if the test moves to the bin-owning package
- `Cargo.lock` — the dependency-delta basis is the **PACKAGE COUNT** (and `cargo deny` green over the
  new lock), never lockfile byte-identity (security-plan, 2026-09-02 session addition)
- `conductor-0.2.0/verification-matrix.json` — `v2-25`'s `ref` currently names the test by path
  (`crates/conductor-tauri/src/commands.rs::commands::tests::path7_…`); if the test relocates the ref
  goes stale and must follow it
- `.andromeda/test-plan.md` §4 / §9 / §10 — read-only governing discipline

## Premises, closed at P3 (see `research.md` §Scope premise closure)

- **The proposal's form (a) does not work as literally stated — and the reason is worse than
  predicted.** Measured (probe 1): `CARGO_BIN_EXE_conductor` is unset for **every** `conductor-tauri`
  target, unit and integration alike, because Cargo sets the variable per *declaring package*; and
  `conductor-tauri` has **no lib target** (`src/` holds only `commands.rs` · `main.rs` · `pause.rs`),
  so an integration test under `crates/conductor-tauri/tests/` could not import the crate at all. The
  entry's "moved to the bin-owning package" (→ `conductor-cli`) and the proposal's "(a) move to
  `conductor-tauri/tests/`" are therefore different fixes, and only the former resolves the variable.
  The control (probe 2) proves the other direction: with the same artifact absent,
  `cargo nextest run -p conductor-cli` **rebuilt** `conductor.exe` and all 36 tests passed, including
  the 6 `cli_smoke` sites calling the identical `Command::cargo_bin("conductor")`.
- **A plain `[dev-dependencies] conductor-cli` edge does not build the binary; the artifact-dependency
  form is out of reach.** `rust-toolchain.toml` pins stable **1.95.0**, so `-Z bindeps` would require
  moving a locked inherited default — an architecture decision this chunk may not make.
- **escargot would admit new packages.** `grep -c escargot Cargo.lock` = 0; the baseline package count
  is **564**, and any delta must be stated as a package count with `cargo deny` green over the new lock.
- **STILL OPEN — the mutation run's cost.** 43 planned mutants; the audit's `--jobs 2` / 2400 s budget
  aborted at the baseline, so no completing-run duration exists. A plan budget question, not a blocker.
- **Relocating the test stales `v2-25`'s `ref`, and ownership is triangulated.** The ref is path-shaped
  (`crates/conductor-tauri/src/commands.rs::commands::tests::path7_…`). Three extracts raised it
  independently and agree: tests and obs call the repair route/wrap-owned rather than their own plan's
  mandate; layouts states the cross-surface envelope invariant loses its named guard if the ref does
  not follow.

## Discovered at P3 — recorded, not acted on

**The parity test uses no `tauri::*` item.** Its "Tauri arm" calls `conductor_run::preflight` +
`conductor_run::drive_run` directly (the same composition `start_run`'s thread runs), while
`v2-25`'s acceptance text names "a `tauri::test` mock-runtime run". The module's mock-runtime helpers
serve the six *other* `#[test]` fns. This is what makes a relocation nearly free — every crate the
test imports is already a dependency or dev-dependency of `conductor-cli` — and it is also a
divergence between a `verified` capability's acceptance prose and its evidence. The boundary above
excludes re-litigating the parity assertion, so it is surfaced at the P5 review rather than reopened.

## Folded annotations

**PREREQ (standing, external decay — operator directive 2026-09-02; this is the 45th probe):**
re-check `cargo audit`; the advisory DB has not parsed since 2026-08-09. Run it with the exit captured
**BEFORE any pipe** (`cargo audit; AUDIT=$?`) — expect exit 1 +
`error loading advisory database: … duplicate advisory ID: RUSTSEC-2026-0244` — and run
`cargo deny check advisories bans licenses sources` **UNCONDITIONALLY** as the overlap probe
(expect 0), never gated on whether the lockfile moved.

Chain, recorded honestly: 43 consecutive re-pins through `2026-09-01-live-per-p-id-verdict-lamps`;
ONE un-probed chunk (`2026-09-02-screen-reader-manual-spec`, which ran no supply-chain probe under its
zero-delta gate deferral, so its wrap re-pinned nothing); the 44th probe ran at
`2026-09-02-cross-surface-envelope-parity`; **this is the 45th**.

Compact/auto-satisfy stays available ONLY while the signature reproduces **byte-identically** — a
different error, or a parsing audit, **ENDS the deferral** and is reported, not re-pinned. Basis form
for a dependency delta under this red gate: assert the **PACKAGE COUNT** (the prior chunk measured
564 → 564 with the lock moving one edge line) and `cargo deny` green over the NEW lock — never
lockfile byte-identity. **This is an external-decay standing pin, NOT a source-delta gate deferral:
never clear it on a rust-gate closure.**

*(No `CARRY:` and no `BLOCKED-ON:` annotation on this entry — the `BLOCKED-ON` in Epoch 6a sits on
`A11y CI gate`, a different, still-markerless entry.)*
