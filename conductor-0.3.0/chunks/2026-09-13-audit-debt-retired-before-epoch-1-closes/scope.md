# Scope — Audit debt retired before Epoch 1 closes

**Marker:** `2026-09-13-audit-debt-retired-before-epoch-1-closes`
**Working entry:** `conductor-0.3.0/working-route.md:17` (Epoch 1 — Foundation: the measurements the closures rest on)
**Promoted:** 2026-09-13

## What this chunk builds

The Epoch 6b code audit (`.andromeda/runs/2026-09-13T11-34-19-code-audit/`) left three standing debts against
this workspace. This chunk retires them and leaves the next audit a measurable outcome:

1. **Mutation survivors** — every non-stub survivor ends *killed* or *carrying a cited accepted-deliberate
   classification*, with the classified set recorded as an enumerated SET (never a count).
2. **The emission-test fixture family** — the five `conductor-emit/tests/` clone pairs collapse onto one
   shared `tests/common/mod.rs`, on this project's own model.
3. **The eleven-key envelope array** — asserted verbatim in two crates, single-sourced beside the schema.

Plus one hygiene item riding as freight, not title: the `conductor-emit → conductor-core` unused dependency.

## Folded freight (the entry's `EVIDENCE:` annotation — re-verified at promotion)

Every coordinate below was folded as a HYPOTHESIS and re-measured against the artifact. Results:

| Freight claim | Verdict |
|---|---|
| 25 survivors across five `c-mutation-{unit}.json` (cli 3 · core 0 · run 5 · tauri 3 · verify 14) | **CONFIRMED exactly** |
| 8 of verify's 14 in `src/bin/stub_pulse_mcp.rs`; 25 − 8 = 17 in scope | **CONFIRMED** (lines 18·37·45·46·51·58·63·75) |
| `c-duplication.json` `top[0..4]` = five `conductor-emit/tests/` pairs at 44·42·39·38·36 lines, `egress.rs` in four | **CONFIRMED exactly** |
| `journal.rs:158-181` ↔ `record.rs:137-160`, 24 lines, same eleven-key array, both under `#[cfg(test)]` | **CONFIRMED** (`top[5]`, 24 lines) |
| `c-dead.json` `unused_deps` = `conductor-emit → conductor-core` | **CONFIRMED** (`{"conductor-emit": ["conductor-core"]}`) |
| Three `conductor.core` refs under `conductor-emit/src`: `error.rs:5` resolvable intra-doc link, `latency.rs:8` + `topology.rs:21` non-resolving prose backticks | **CONFIRMED exactly** |
| `observe` is `conductor-verify`'s most complex *production* fn (cognitive 17) | **CONFIRMED** — `serve_stub` (35) is higher but is test-side (`tests/common/mod.rs`) |

## The directive's class split does NOT reproduce — the measured split is the scope

The phase directive leaned "TWELVE are holes / FIVE are ratify-by-class candidates". Measured against
`test-plan.md` §12 Test Decisions Log — which carries a **five-member operator-ratified accepted-deliberate
roster** with citation homes — the true split of the 17 is:

**TWELVE are ALREADY-RATIFIED roster members** (not candidates; each already cites a rule):

| Survivor (today) | Roster member (§12) | Identity |
|---|---|---|
| `cli render.rs:161:5` `stdout_color`→false | member 5, cli PAIR (2026-09-05) | exact |
| `cli render.rs:171:5` `stderr_color`→false | member 5, cli PAIR | exact |
| `run preconditions.rs:44:5` `declares`→false | member 1, `declares` | fn+col; line moved `:50`→`:44` |
| `run canary.rs:56:8` delete `!` in `preflight` | member 3, class C | fn+col; line moved `:58`→`:56` |
| `run execute.rs:105:17` delete field `degraded` | member 3, class B | see [inferred] below |
| `run execute.rs:148:25` `-`→`+` | member 3, class B | fn+col; line moved `:136`→`:148` |
| `run execute.rs:148:25` `-`→`/` | member 3, class B | fn+col; line moved `:136`→`:148` |
| `tauri main.rs:18:5` `main`→() | member 2, tauri TRIPLE | exact |
| `tauri commands.rs:273:8` delete `!` in `start_run` | member 2, tauri TRIPLE | fn+col; line moved `:263`→`:273` |
| `tauri commands.rs:308:5` `run_thread`→() | member 2, tauri TRIPLE | exact |
| `verify spawn.rs:116:5` → `true` | member 4, verify PAIR (2026-09-04) | exact |
| `verify spawn.rs:116:5` → `false` | member 4, verify PAIR | exact |

Identity is by **function + COLUMN**, never line arithmetic — §12's own re-pointing discipline.

**FIVE are genuinely unowned** — named by no roster member, this chunk's real kill work:

- `conductor-cli/src/cleanup.rs:16:5` — `cleanup -> anyhow::Result<ExitCode>` → `Ok(Default::default())`
- `conductor-verify/src/client.rs:187:9` — `ReadbackClient::resolve_incident` → `Ok(Default::default())`
- `conductor-verify/src/extract.rs:98:22` — `==`→`!=` in `observe`
- `conductor-verify/src/extract.rs:113:22` — `==`→`!=` in `observe`
- `conductor-verify/src/manifest.rs:35:9` — `ContractManifest::default_path` → `Default::default()`

12 + 5 = 17. The directive's 12/5 reached the same total by two errors that cancel: it collapsed
`spawn.rs:116`'s two polarities into one item, and counted `stub_pulse_mcp.rs:18`'s `main → ()` — which its
own ruling excludes — as the second `main → ()`.

**Two directive items are specifically foreclosed by a ratified record:**
- `spawn.rs:116` "the killing test needs a positive AND a negative path" — §12 member 4 records that exact
  agreement test as **CONSIDERED AND REJECTED**: it kills exactly one of the two arms depending on whether
  `andromeda-pulse-mcp` happens to be on `PATH`, making the score host-dependent, where a roster must mean the
  same thing on every host (§10 zero-flakiness).
- `commands.rs:273` and `canary.rs:56` as "the two inverted guards … holes" — both are ratified members
  (tauri TRIPLE / run class C).

## The acceptance target `survivors 25 → ≤ 8` is unreachable by its own mechanism

`.claude/rules/testing.md:19` states the gate: *"`missed.txt` holding EXACTLY the run's accepted-deliberate
survivors — empty only when none is accepted, **since such a survivor by construction survives***". An
accepted-deliberate classification does **not** remove a mutant from the population; the survivor stays in
`missed.txt`. The project uses no `#[mutants::skip]` and ships no `mutants.toml` (verified: no `mutants` dep,
no skip attribute, cargo-mutants 27.1.0).

So with all five unowned survivors killed and the twelve ratified members standing, the measurable outcome is
**25 → 20** (19 if a re-attempt shrinks the roster by one), never ≤ 8. The `≤ 8` figure is reachable only by
also removing the 8 stub mutants from the population — which the same directive puts out of scope.

**Resolution (keep the goal, change the mechanism):** acceptance is the §12/testing.md:19 discipline —
*every one of the 17 is killed or cites a rule, and the accepted set is recorded enumerated* — plus the
measured survivor count as a consequence, not as the bar. Carried to the P5 review as the one place the
operator sees it whole.

## In scope

- **Kill the five unowned survivors**, each with a killing assertion that answers testing.md:72's three
  reachability questions first (visibility · what gates sit above · does the vehicle ENTER the mutated fn).
- **Re-attempt the ratified classes this chunk TOUCHES, under compulsion** (testing.md:79: *"when a later
  chunk touches an accepted class, re-run it rather than carrying the count forward — a roster is a ledger of
  owed work"*), and record what shrank. No kill is promised: §12's reasons are specific and recently
  re-verified. **[P5 val-1 amendment — intent-incomplete]** This was first written as "the twelve"; the
  compulsion in testing.md:79 is conditioned on TOUCHING the class, and this chunk touches only
  `conductor-cli` (member 5's crate) and `conductor-verify` (member 4's crate). `conductor-run`'s
  `declares`/class-B/class-C members and `conductor-tauri`'s triple are **not** compelled here — recorded
  with the reason, never skipped silently.
- **Enumerate the stub 8 in the gate's roster as `provisional-next-audit`** — bookkeeping, not a
  disposition: a tally gate cannot be green while a survivor is unnamed, and the stub exclusion itself
  stays out of scope. Operator-ratified at this chunk's P4.
- **Re-point the four moved coordinates** in the §12 roster (`declares` `:50`→`:44`; `start_run` `:263`→`:273`;
  class B `:95:27`→`:105:17` and `:136:25`→`:148:25`; class C `:58`→`:56`). `test-plan.md` is a spec master —
  phase and implement are read-only on it, so this rides **wrap's amendment flow**, surfaced by /implement.
- **Share the emission-test fixture family** — five clone pairs over six files (`egress.rs`, `error_spans.rs`,
  `exception_events.rs`, `traffic_rate_ramps.rs`, `multi_service_topology.rs`, `latency_shaping.rs`) onto one
  `crates/conductor-emit/tests/common/mod.rs`, modelled on `crates/conductor-run/tests/common/mod.rs`
  (2026-09-05-audit-corrective).
- **Single-source the eleven-key envelope array** beside the schema it describes: `RunRecord` lives at
  `crates/conductor-core/src/run_record.rs:17`, and both clone sites already depend on `conductor-core`.
- **Test the dep-removal hypothesis** and act on the result.
- **Gates**: per-unit `cargo mutants` re-runs in the audit's own firing form, `--output` at a fresh path under
  `target/` (testing.md 2026-09-03 — a repo-root output dir is not gitignored), read only at the tool's own
  completion markers (collectors.md C1: `outcomes.json` `end_time` set AND `total_mutants` == `len(mutants.json)`),
  gating on the TALLIES, never the exit code (testing.md:19).

## Out of scope (by design)

- The **8 `stub_pulse_mcp.rs` survivors** and the standing stub exclusion — the next audit's confirm.
- The **first mutation measurement of `conductor-emit` / `-report` / `-timeline` / `-faults`** — never yet scored.
- **`conductor-core`'s shard rotation** — the audit scored shard 1/4 (126/504, `budget-exhausted` for the rest).
- Any change to `requirements.md` (immutable) or to a spec master (read-only here).

## Premise closure (P3, 2026-09-13 — each bullet VERIFIED or corrected against the artifacts)

- **VERIFIED — `execute.rs:105:17` is the standing class-B member re-pointed, not a new entrant.** The
  citation home (`2026-09-03-conductor-run-composition-root-survivors-dispositioned/evidence/disposition-ledger.md:56`)
  names class B at mint as ``lib.rs:518:27`` *(delete field `degraded`)* · ``lib.rs:559:25`` ×2 (`-`→`+` / `/`).
  Today's mutant carries that same mutation description, which is **unique across all 25 survivors**, and
  conductor-run's composition is unchanged (5 = one `declares` arm + three class-B + one class-C).
  **Note for wrap:** §12's identity rule is function + COLUMN, and the column moved 27 → 17 — so the rule is
  insufficient here; what settles it is cargo-mutants' own unique mutation DESCRIPTION.
- **VERIFIED — all five unowned survivors are reachable, each with a named in-crate vehicle.**
  `cleanup.rs:16` → a `cli_smoke.rs` assert_cmd subprocess (the mutant keeps `ExitCode::SUCCESS` but skips
  both the delete and the print, so the observable is the printed line / row count);
  `client.rs:187` → `readback.rs`'s stub, entering the wrapper the existing `resolve_with` helper bypasses;
  `extract.rs:98`/`:113` → `readback_shape_witness.rs`'s own test binary with ≥3 incidents and per-tool
  witness-line counts; `manifest.rs:35` → a direct `#[cfg(test)]` assertion (`pub`, zero call sites).
- **`hypothesis:` — RETAINED, evidence strengthened.** Dropping `conductor-core` from
  `conductor-emit/Cargo.toml` is expected to break only `error.rs:5`. The graph's usage-based `crate_edges`
  carries **no** `conductor-emit → conductor-core` row (a second, independent source agreeing with
  `cargo machete`); `conductor-emit/tests/` has zero `conductor_core` tokens; `src/` has exactly three
  doc-comment mentions, of which only `error.rs:5` resolves. The standalone baseline
  `cargo check -p conductor-emit --lib` is **exit 0** at HEAD with the dep present. No build has run
  WITHOUT the line — P3 is read-only — so the claim stays a hypothesis that /implement closes.
- **VERIFIED — the eleven-key const's home is `conductor-core`.** `RunRecord` is defined at
  `conductor-core/src/run_record.rs:17`, and the graph shows **both** clone sites' crates already carry
  `→ conductor-core`, so the placement adds no cross-seam edge. Added constraint found in P3:
  `journal_conformance.rs:96` asserts key **presence** ("extra keys are allowed"), so the const must not be
  wired into that gate as an exclusivity check — the two clone sites' exclusivity assertions are over
  `RunRecord`'s own serialization and are legitimate where they sit.

## Touched surfaces

`crates/conductor-cli/src/commands/cleanup.rs` · `crates/conductor-verify/src/{client,extract,manifest}.rs` (+
their test modules) · `crates/conductor-emit/tests/*` (+ new `common/mod.rs`) · `crates/conductor-emit/Cargo.toml`
· `crates/conductor-emit/src/error.rs` (doc link) · `crates/conductor-core/src/run_record.rs` ·
`crates/conductor-report/src/journal.rs` · `crates/conductor-verify/src/record.rs`.
