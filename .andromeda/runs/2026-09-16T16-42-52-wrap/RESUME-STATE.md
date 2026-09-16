# Wrap resume state — 2026-09-16-a11y-ci-gate-at-an-honest-terminal

Written at the 85 % context alarm, mid-P2-apply. **The wrap is NOT complete and NOTHING is committed.**
Resume from this file plus `fanout-results.md` in the same run dir.

## Done so far

- **Setup** — 1 pending chunk; branch `build/conductor-0.3.0`; `HEAD 38d21d1`; 0 ahead; code-graph
  refresh fired in background (log in this run dir).
- **P1 report** — `chunks/2026-09-16-a11y-ci-gate-at-an-honest-terminal/report.md` written, with all four
  plan expected-amendments dispositioned and their locating greps.
- **P1 evolve checkpoint** — appended (2 records).
- **P2 fan-out** — 7 doc-agents done. **7 proposals** (arch 5, security-plan 1, test-plan 1); design-system,
  layout-templates, obs-plan, a11y-plan all `proposals: []`. Full detail + every claim site re-derived
  verbatim: `fanout-results.md`.
- **P2 validate** — done. Three escalations raised to the operator, who answered `continue`; conservative
  reading taken (register the SHIPPED mechanism, SET-name the tally, record the remedy measured-insufficient).
- **P2 apply — PARTIAL. `architecture.md:60` DONE (3 edits, all landed):**
  1. dev-host runnability de-literalized to the measured runtime SET (12/2 at 152 → 10/2/2 at 153, the two
     regressions being the hold-free Operable pair, NOT session creation);
  2. the a11y asserting step's invocation path recorded as launched through
     `scripts/a11y-limited-token-launch.ps1` / `a11y-token-witness.ps1`;
  3. the remedy class recorded MEASURED INSUFFICIENT with the three-leg basis and the successor as owner.

## REMAINING — apply these, in order

1. **`architecture.md:224`** — register `scripts/a11y-limited-token-launch.ps1` (CI-only limited-token
   launcher for the a11y asserting step) and `scripts/a11y-token-witness.ps1` (CI-only launched-leg entry
   point + token witness) in the `scripts/` tree, beside `webview2-cause-probe.ps1` and carrying the same
   qualifier — "invoked solely by ci.yml's a11y job, wired into neither harness shell, adds no 6th command".
2. **`architecture.md:244`** — the second occurrence of the a11y invocation path (CI/CD approach
   parenthetical); correct it to the launcher path. `dependent-of` the `:60` edit already applied —
   a single-site apply would leave this standing.
3. **`security-plan.md:363`** — rule (b) SIX → SEVEN governed harness-spawn forms. Register the SHIPPED
   mechanism: the a11y job's asserting step launches the routine leg through
   `scripts/a11y-limited-token-launch.ps1`, a **`runas /trustlevel:0x20000`** launch (the scheduled-task
   mechanism was written, shipped to four CI runs, and REPLACED in the same chunk — do NOT register the
   scheduled task; the drafted text in `evidence/surfaced-argv-composition.md` is stale on this point).
   Form facts: program fixed and resolved; argv is five literals authored in `ci.yml`; **`-File` only,
   never `-Command`** — which is what keeps it outside rule (b)'s eval class; the command-line composition
   is FORCED by the API (`New-ScheduledTaskAction -Argument` is `System.String`; `runas` takes one command
   string) rather than chosen; the launcher rejects whitespace and shell metacharacters on every element
   before composing (measured exit 90 with a named `A11Y_LIMITED_TOKEN_ARGV: REJECTED` precondition on both
   controls); no operator-supplied value reaches argv. Rule (a) — the MCP sidecar spawn, `.env(...)`
   handling, the `2024-11-05` pin — is untouched by this chunk and stands unchanged.
   Operator-ratified under playbook `:124` (boundary widening, always a human's call).
4. **`test-plan.md:461`** — the `a11y` job step set: keep the `continue-on-error` DIAGNOSTIC step set named
   as a SET and carry its fourth member shipped this chunk, `Remote-debugging-pipe route (diagnostic)`
   (`if: always()` + `continue-on-error: true`, after the existing diagnostics, before the
   `a11y-session-diag` upload); and record that the asserting step launches through the limited-token
   launcher rather than calling `agent-run.ps1` directly. SET-naming per playbook, never a fresh literal.
5. **Sidecars** — one entry per applied amendment in `architecture-amendments.md`, `security-plan-amendments.md`,
   `test-plan-amendments.md`, each carrying the sweep's count.
6. **Cascade** — re-derive the CLAUDE.md / `.claude/rules/*` / `.claude/docs/*` distillations for the amended
   claims, preserving `USER:*` and `## Session Additions` verbatim.

## THEN the remaining phases

- **P3 curation** — the session conversation is the ONLY source. Raw material is in `report.md`
  §Decisions & corrections and `evidence/WRAP-DECISIONS.md` §7: the deny-only `IsInRole` conflation; the
  clipped-view class (3 instances, 2 readers); the inherited-handle mechanism (`$output = & cmd` AND
  `Start-Process -Wait` both block; waiting on the process object does not); an instrument under-counting
  its own subject (`OrphansLeft`); a probe validated outside its own strict-mode context.
- **P4 code-graph** — check `.andromeda/cache/.refresh-done` / `.refresh-stale`; non-blocking.
- **P5 route-resolve** — **WILL HALT (trajectory edit), and that halt is sanctioned.** The next entry is the
  medium-integrity launch (`CreateRestrictedToken` + `SetTokenInformation`). Freight per
  `evidence/WRAP-DECISIONS.md` §2: the three-leg table; integrity buys the SESSION not a green arm;
  `:384`/`:397` must additionally hold at the runner's 152; window station a NAMED unvaried risk. Also
  carry the hygiene CARRY (one defect, three surfaces) and the `CENSUS_NAMES` surface in SR territory.
- **P6 state + handoff.**
- **P7 gates + commit** — light gate over the plan's `[[gate]]` block; **un-claim `v3-02`**
  (`matrix.py unclaim --dir conductor-0.3.0 --id v3-02`) BEFORE the coverage gate, per the operator ruling —
  it is claimed at `status: planned` / `ref: null` and the gate would otherwise HALT
  `coverage-incomplete: v3-02`; then master `pending → complete`, flip-compaction, commit.

## Standing facts a fresh window will not have

- `HEAD 38d21d1`, branch `build/conductor-0.3.0`, 0 ahead, nothing committed by this chunk yet.
- Light-gate expectations: entries 1–4 green; entry 5 (`cargo nextest --workspace`) has `defer` in the plan
  but the deferral was VOIDED at implement (uncommitted `verification-matrix.json` is read by four Rust
  files) and ran by hand at **986 passed**; entries 6–7 are `leg = 'operator'` — never re-run, re-verified by
  the recorded run ids `35079315258`, `35095825604`, `35102123985`, `35103823579`, `35111618735`.
- No probe branch outstanding (`git ls-remote --heads origin 'ci-probe/*'` → 0).
