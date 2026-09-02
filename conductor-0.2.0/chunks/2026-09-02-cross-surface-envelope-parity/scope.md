# Scope — 2026-09-02-cross-surface-envelope-parity

**Working entry (title clause, verbatim — Epoch 5 "Verification surfaces", `working-route.md:89`, the LAST
markerless entry of the epoch, so this chunk's wrap closes Epoch 5):**
> Cross-surface envelope parity — CLI and Tauri identical envelope for one seed, with the stale rmcp wording
> reconciled

The line additionally carries two `CARRY` annotations and one `PREREQ`; each is folded below with its
re-verification result.

**Matrix entries (two):**
- `v2-25` _Cross-surface envelope parity_ — requirement "CLI and Tauri produce an identical envelope for the
  same seed." · method `integration` · acceptance (outcome-level, route-authored): "For one scenario and seed,
  the Tauri-launched run and the headless run write identical verdict, state and seed to the same runs.db, and
  each writes its own emission journal." · `status: planned` · `chunk: null`.
  [verified against `verification-matrix.json` 2026-09-02]
- `v2-28` _Stale rmcp wording reconciled_ — requirement "`test-plan.md`, `tests-summary.md` and
  `verification-harness.md` describe the hand-rolled JSON-RPC client that replaced rmcp." · method
  `by-construction` · acceptance: "No rmcp claim describing Conductor's current read-back client remains in
  the three named documents; each describes the hand-rolled line-delimited JSON-RPC client." · `status:
  planned` · `chunk: null`. [verified against `verification-matrix.json` 2026-09-02]

**Intent:** F16 (`intent.md:240-241`) — "OBSERVED: CLI and Tauri share a core but their envelopes have never
been compared for the same seed. EXPECT: an identical-envelope proof, CLI vs Tauri, same seed." · F19
(`intent.md:251-252`) — "Stale `rmcp` wording survives in `test-plan.md` body, `tests-summary.md:44` and
`verification-harness.md` (~lines 18/33) — rmcp was removed in favour of the hand-rolled client."
[both verified at source]

---

## Operator PHASE directive (2026-09-02, take-up) — as given

1. **The msedgedriver PREREQ is DISCHARGED by the operator.** `D:\dev\tools\edgedriver\msedgedriver.exe` is
   **152.0.4191.53**, matching the WebView2 runtime 152.0.4191.53; the prior binary sits beside it as
   `msedgedriver-151.0.4129.101.exe`. **Confirm with `--version` at P3** and record the PREREQ as
   closed-by-operator in the plan. The rust-gate PREREQ stays the agent's — **re-verify the zero-`.rs`-delta
   premise rather than echoing it**.
2. **The banner CARRY needs an OVER-envelope run against a LIVE Pulse.** Pulse is **not running now**; the
   operator boots it on request (deterministic L4 + MCP, **fresh data dir, printed for the agent**). Plan the
   live leg, **then ask** — never assume a running Pulse, never reuse a prior data dir (the quiet-window rule
   in `.claude/rules/verification-harness.md`).

---

## What this chunk builds

1. **The v2-25 parity proof — CLI vs Tauri, one scenario, one seed.** Both surfaces already share
   `conductor-run`'s composition root, so parity is a claim about the two SHELLS agreeing, not about the
   engine. The proof compares what each surface actually persists: the `runs` envelope row (`verdict` /
   `state` / `seed`, and the rest of the eleven-field shape) written into one `runs.db` under a shared
   `CONDUCTOR_RUNS_DIR`, plus each run writing its OWN `<run_id>.jsonl` emission journal (distinct `run_id`s —
   parity is field equality, never artifact identity). The test-plan already names the driver pair for this:
   the `tauri::test` mock-runtime run and the `assert_cmd` CLI subprocess run pointed at the same
   `assert_fs::TempDir` DB (`test-plan.md:287`). [verified at source 2026-09-02]
2. **The v2-28 rmcp reconciliation** across the sites that actually still carry a present-tense claim —
   see the CARRY-1 fold below, which measured the CARRY's own enumeration to be incomplete in BOTH directions.
3. **The desktop load-envelope banner's rendered-DOM proof** (CARRY-2) — the one assertion the routine `--e2e`
   arm structurally cannot reach today, requiring an over-envelope subject that the journal-only fixture seed
   does not create.
4. **The rust gate deferred from the previous chunk** — `cargo nextest run --workspace --profile ci` +
   `cargo clippy --workspace --all-targets -- -D warnings`, mandatory here.

---

## Folded freight (route annotations + directive coordinates, re-verified)

### PREREQ — close the rust gate deferral [OWNED, agent]
`cargo nextest run --workspace --profile ci` and `cargo clippy --workspace --all-targets -- -D warnings` were
deferred at `2026-09-02-screen-reader-manual-spec` under the source-delta-proportional rule (that chunk shipped
TypeScript, Markdown, a PowerShell script and an NVDA profile only). This chunk touches Rust, so both run here
as **mandatory gates, not deferrable**.
- **[VERIFIED at P3 — premise TRUE]** `fff1068` carries **zero `.rs`, zero `Cargo.toml`, zero `Cargo.lock`**
  (69 files: 56 `.md`, 4 `.ts`, 4 `.json`, 1 `.yaml`, 1 `.ps1`, 1 `.ndjson`, 1 `.log`, 1 `.ini`). The deferral
  was sanctioned. One imprecision recorded, not a defect: the entry's prose ("TypeScript, Markdown, a
  PowerShell script and an NVDA profile only") under-enumerates by 7 files — Andromeda bookkeeping (friction
  log, run trace, `state.yaml`, matrix, evidence JSON) plus `crates/conductor-tauri/ui/package.json` (npm
  scripts; `package-lock.json` unchanged). None is a Rust source or Cargo manifest, so the rule's premise
  stands as stated.

### PREREQ — msedgedriver refresh [DISCHARGED by operator]
Recorded closed-by-operator on the directive above. **P3 confirms by running `--version` on the named path**;
the confirmation is evidence in the plan, not a re-opening. A mismatch between the printed version and the
directive's 152.0.4191.53 is surfaced to the operator, never silently worked around.

### CARRY-1 (from `2026-08-31-p-075-assert-round`) — the rmcp sweep
**The CARRY states:** the sweep is "ONE site smaller and one site sharper"; `obs-plan.md:35` was corrected at
that wrap; **what REMAINS is `contracts/mcp-contract.toml`'s header comment** ("the rmcp client negotiates DOWN
to it"); the derived-tier mentions (`docs/stack.md`, `docs/conventions.md`, `docs/gotchas.md`,
`rules/verification-harness.md:18`, `rules/testing.md:35`) were verified correct 2026-09-01; and the dated
`## Session Additions` entries naming rmcp are historical records to leave standing.

**Re-verification at HEAD (2026-09-02) — the CARRY's enumeration is incomplete in BOTH directions:**
- **CONFIRMED remaining:** `contracts/mcp-contract.toml:5` still reads "(the rmcp client negotiates DOWN to
  it)". [measured]
- **CONFIRMED correct, leave standing:** `.claude/docs/stack.md:15`, `.claude/docs/conventions.md:28`,
  `.claude/docs/gotchas.md:12,17`, `.claude/rules/verification-harness.md:18,44`,
  `.claude/rules/testing.md:35`, `.claude/docs/tests-summary.md:46`, `.claude/agents/code-reviewer.md:47` —
  each already records the 2026-06-27 removal. `.claude/rules/verification-harness.md:40,41` are dated
  2026-06-21 Session Additions (historical records). [measured]
- **[VERIFIED at P3, and WIDER than first measured — v2-28's own acceptance is unmet at HEAD.]** The
  acceptance names **`test-plan.md`**, and `.andromeda/test-plan.md` carries rmcp on **24 lines**
  (`grep -c`, verified — the tests distiller reported 26; the distiller's own enumeration was also partial,
  so neither list is the answer and the count is the basis). Present-tense sites describing the CURRENT
  client/stub include `:33 :53 :114 :143 :243 :254 :264 :272 :303 :317 :416 :431 :437 :457 :522 :554 :556`;
  `:88` (CVE trigger) and `:558` (STDIO-injection ban) name the vulnerability CLASS and may stand, by the same
  distinction drawn for the Rust doc comments. **Per-site classification is implement's pass, over the count,
  not over either enumeration.** `test-plan.md` is a **spec source — phase and implement are read-only on it**;
  the sanctioned channel is wrap's amendment flow at this chunk's wrap. P4 routes it there explicitly, because
  **v2-28 cannot be claimed without it.**
- **[VERIFIED at P3] NOT NAMED by the CARRY — a shipped Rust doc comment is false at HEAD.**
  `crates/conductor-verify/src/lib.rs:3` reads "Shipped so far: [`ReadbackClient`] — **the rmcp
  client/transport foundation**" — a present-tense claim about the shipped client, not a historical note.
  (Distinguish from `lib.rs:5` / `spawn.rs:5` / `client.rs:7` / `jsonrpc.rs:5` / `tests/common/mod.rs:3`, which
  name the rmcp *STDIO injection class* or record *why rmcp was rejected* — those are correct as written.)
  [measured 2026-09-02] P3 confirms the distinction; P4 decides whether it lands here (it is code, not a spec
  source, and it is the same defect class v2-28 names).

### CARRY-2 (from `2026-09-01-live-per-p-id-verdict-lamps`, operator wrap directive) — the banner's visual proof
**The CARRY states:** the desktop load-envelope banner shipped with that chunk but **no rendered DOM has ever
asserted it** — the a11y sweep ran before the surface existed, and the routine `--e2e` arm context-skips it
because the seeded fixture journal records no `run_envelope` row. Its contrast rides the render-independent
token-pair spec (`--status-residual` at 4.5:1 over both `--color-raised-1` and `--color-base`, both themes) and
its populated render rides `conductor_run::read_envelope`'s round-trip plus the `run_envelope` mock-runtime IPC
test. **What this entry adds:** seed an over-envelope run on the desktop (an `EnvironmentSuspect` row in the
fixture runs dir's `run_envelope` table, which the journal-only seed does not create), then assert the banner's
DOM label and run axe against that state.

**Re-verification at HEAD (2026-09-02) — every named coordinate exists:**
- The spec **already exists and already asserts the label**: `test/a11y/accessibility.e2e.ts:270-280` — `it('an
  over-envelope run banners its standing with the label in the DOM')` expects
  `report__envelope-label` to read `ENVIRONMENT-SUSPECT`, and calls `this.skip()` with the stated
  subject-absent reason when `[class~="report__envelope"]` does not exist. **So the work is to make the SUBJECT
  exist, not to author the spec.** [measured]
- `conductor_run::read_envelope` at `crates/conductor-run/src/lib.rs:852`, called from the eighth
  `#[tauri::command]` `run_envelope` (`crates/conductor-tauri/src/commands.rs:222-243`). [measured]
- The `run_envelope` table (`crates/conductor-report/src/db.rs:32`, insert at `:215`, read at `:222-232`) and
  `EnvelopeStatus::EnvironmentSuspect` (`crates/conductor-core/src/load_envelope.rs:97`). [measured]
- `seedFixtureRuns()` is the fixture seeder, invoked in `wdio.conf.ts`'s `onPrepare` at the ONE tauri-driver
  spawn site that also chooses `CONDUCTOR_RUNS_DIR` per invoked suite. [measured]

**[premise-corrected at P3: a live Pulse is NOT structurally required for the banner subject — `classify_run`
runs before driving and `persist` writes `insert_envelope` unconditionally on BOTH shells, so an
`EnvironmentSuspect` row lands with or without Pulse.]** The evidence, all at HEAD 2026-09-02:
- `conductor-cli/src/commands/run.rs:20-27` computes `classify_run(...)` **before** `preflight(...)` and then
  calls `persist(...)` unconditionally; `conductor-run/src/lib.rs:824-844` `persist` ends with
  `db.insert_envelope(run_id, envelope)` on every path.
- The Tauri shell does the same: `conductor-tauri/src/commands.rs:269` calls `classify_run`, and
  `conductor-run/src/lib.rs:917-946` `drive_run` persists unconditionally — including on the abort branch
  (`:937`).
- So a Blocked (no-Pulse) run still writes the `run_envelope` row. What the live Pulse changes is the
  **verdict quality** of the accompanying `runs` rows (real verdicts vs a uniformly Blocked spine), not
  whether the banner has a subject.

**What DOES bound the subject, measured:** a breach needs a scenario shape no committed catalog scenario has.
`phase_breach` (`conductor-core/src/load_envelope.rs:174-188`) judges emitting phases only, and
`phase_rate_exceeds` is exact integer math — `occurrences * 1000 > max_rate * gap_ms`, with `gap_ms == 0`
always breaching. At the pinned `max_sustained_rate_spans_per_s = 10000` that is **`occurrences > 10 × gap_ms`**,
so a phase of `gap_ms = 1, occurrences = 11` breaches while emitting **11 dispatches** — the subject costs no
dangerous load at all. (The storm term needs `gap_ms > 600000`, i.e. >10 minutes of real time, and is the
wrong lever.) **`check_load_envelope` has NO production caller** — only its own unit tests, one of which gates
the committed catalog (`load_envelope.rs:556`) — so a variant scenario served from an in-repo untracked
`CONDUCTOR_SCENARIOS_DIR` (the sanctioned convention, `verification-harness.md` 2026-08-19b) breaches at
`classify_run` without touching the catalog gate, and **no `[[exempt]]` widening or term re-tuning is needed**
(both banned by architecture.md §Occupied Resources).

**The fork therefore is not "seeded vs live" but "how strong a proof":** the CARRY's raw-INSERT mechanism is
independently banned by test-plan §7 (no developer-populated `runs.db`), so the real options are a
production-writer seed with no Pulse, or the operator's live GUI leg. **P4 resolves it and ASKS** — the
directive's instruction to plan the live leg stands, with this measurement offered so the boot is a choice
made on evidence rather than a believed necessity.

---

## Boundaries

- **No engine or seam change for parity.** Both shells already call `conductor-run`; if the proof needs a
  behavioural change to make them agree, that is a DEFECT finding to record and fix at its cause — not a
  licence to re-architect the run path.
- **Phase and implement are read-only on the seven `.andromeda/` masters.** The `test-plan.md` rmcp sites route
  to wrap's amendment flow. `intent.md` / `requirements.md` are immutable.
- **No second automation stack.** The banner proof rides the ONE WebdriverIO + tauri-driver stack and its
  existing spec; at most the fixture seed and the suite's subject env change.
- **No new inbound listener.** The trust boundary is unchanged: the only binds remain the `:4317` port-occupier
  fault and the dev-only `4444`/`4445` driver pair.
- **Never reuse a prior Pulse data dir** for the live leg (the quiet-window rule); the operator prints a fresh
  one on boot.
- **Not this chunk:** the Epoch-6 entries (A11y CI gate, sidecar console window, SR findings remediation,
  release build) — even where this chunk's findings touch them.

## Questions carried into P3/P4

1. Is the SR chunk's zero-`.rs`-delta deferral premise true at `fff1068`? (PREREQ, must be measured.)
2. Does `msedgedriver.exe --version` print 152.0.4191.53 at the named path? (Directive item 1.)
3. Seeded `run_envelope` row vs live over-envelope run for the banner subject — and if live, WHICH scenario
   shape breaches a sustained term, given the empty exemption ledger. (Directive item 2; the fork P4 resolves.)
4. Does the v2-25 parity proof have a home in the existing test surface (the `tauri::test` mock runtime + the
   `assert_cmd` CLI leg over one `CONDUCTOR_RUNS_DIR`), or does it need the driver-backed webview arm?
5. Does `crates/conductor-verify/src/lib.rs:3` land in this chunk's v2-28 work? (Code, not spec — but not
   named by the CARRY.)
