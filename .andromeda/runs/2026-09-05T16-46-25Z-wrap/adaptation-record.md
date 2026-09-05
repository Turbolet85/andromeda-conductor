# Adaptation record — 0-pending wrap · 2026-09-05T16:46:25Z

**Path:** Setup step 6 — 0 master `pending`; tree dirty ONLY with expected-transient bookkeeping
(`session-handoff.md` second-writer line · `friction-log.ndjson` appends · `code-metrics.ndjson` +1 record) plus
two untracked `.andromeda/runs/` dirs (`2026-09-05T07-42-45-evolve-diagnose` · `2026-09-05T08-35-52-code-audit`,
boundary #4 closing Epoch 6a) → no-op path WITH an operator route-adaptation request (route-resolve §Operator-requested
adaptation) + P3 curation + the self-produced-fact channel. No P1/P2/P7; no light gate; no master flip; no compaction.

**Git-state at Setup:** `build/conductor-0.2.0` · upstream `origin/build/conductor-0.2.0` = HEAD `59d5b7c` (0 ahead — the
branch was PUSHED after the 2026-09-05T06:45Z wrap; the handoff's "53 ahead, unpushed" was stale at session start) ·
CI run 33954347685 (2026-09-05T08:06Z) green on both jobs, the first CI run since 2026-08-09.

**Directive:** operator context correction superseding the handoff's Next (phase on *Operator-gated live suite*): the code
audit's code-facing findings become Epoch 6b's corrective FIRST chunk; the founder's rulings land as route freight.

## Items and dispositions

### 1 · MINT the 6b head — APPLIED
`Audit corrective — render.rs survivors dispositioned, civil_from_unix boundary-date kill, jsonrpc.rs timeouts owned, M2's
three dedups, and conductor-run/src/lib.rs split along named seams with its inline tests moved to tests/` inserted at
`working-route.md:109`, directly before *Operator-gated live suite* (now `:111`), with a `   ↓` separator. Trajectory gate
satisfied by the recorded operator direction naming the entry, its placement AND its content (route-resolve: a pre-direction
naming both entry and disposition applies without a halt); the direction is cited in the entry's CONTEXT freight.
Basis RE-VERIFIED against `.andromeda/runs/2026-09-05T08-35-52-code-audit/proposals.md` before writing: cli 58.59 with
32 of 41 survivors in `render.rs` (§First-audited units) ✓ · core shard 1/4 with 6 survivors at `obs.rs:224-226`
`civil_from_unix` ✓ · `jsonrpc.rs:49:30` + `:70:9` TIMEOUT mutants "crossed two boundaries unowned" (§Notes) ✓ · clones
84 → 86 → 90 with M2's (a) commands.rs prologue / (b) CLI↔report coverage formatter / (c) `tests/common` wire fixture ✓ ·
`lib.rs` 1012 → 1115 → 1423 ✓ · `test-plan.md:609` names the roster coordinate `lib.rs:362:5` ✓ (carried on the entry as
the Expected amendment for that chunk's wrap). The founder's ruling — the split INTO this chunk — is stated in the freight.

### 2 · MIGRATE the standing cargo-audit PREREQ — APPLIED, basis FALSIFIED → FULL form (a reported deviation)
Stripped from *Operator-gated live suite* (`:111` now ends at its second CARRY) and re-pinned on the new head. The compact
form's basis ("the RustSec advisory DB itself will not parse — a DATABASE fault") was re-verified as the form requires and
measured FALSE, so the pin returns to FULL form until the corrective chunk's wrap closes it. Measurements, this session:
- `git -C $CARGO_HOME/advisory-db status --porcelain` → 0 lines (post-clean, 16:4xZ). Pre-clean (12:48Z, dashboard):
  `?? crates/gettext-sys/RUSTSEC-2026-0244.md` UNTRACKED beside the tracked `crates/gettext-rs/RUSTSEC-2026-0244.md`.
- Upstream: `e12b689b` 2026-08-09 16:57:02 +0200 "Move RUSTSEC-2026-0244 to correct crate (#3128)" — the deferral's
  first day. Local checkout HEAD `5a0ebedf` (2026-09-02) = a fresh clone's HEAD: the git tree matched upstream throughout.
- `cargo audit` on the default cache PRE-clean (12:2xZ): exit 1, `error loading advisory database: parse error: duplicate
  advisory ID: RUSTSEC-2026-0244` (the 52nd byte-identical reading: 51 wrap re-pins + this probe).
- `cargo audit --db <fresh clone in the scratchpad>`: exit 0. CI run 33954347685 step "Supply-chain — cargo audit":
  success (`ci.yml:74` is a bare `run: cargo audit`, no `continue-on-error`).
- `cargo audit` POST-clean (16:4xZ): exit 0 — `Loaded 1239 security advisories`, `564 crate dependencies`,
  `warning: 18 allowed warnings found` (17 `unmaintained` · 1 `unsound` · 0 yanked). `cargo deny` overlap exit 0.
- **Dictation drift corrected:** the directive said "one `unmaintained` warning"; the measured summary is 18 allowed
  warnings (17 unmaintained + 1 unsound). The route carries the MEASURED state.
- Exit codes read directly (redirect to file, `$?` from the bare command), never through a pipe.
NOT closed here (no light gate runs on this path). The pin names the corrective chunk's two-step probe (`status --porcelain`
empty, then `cargo audit` expected exit 0) and the closure per route-resolve §Deferred-gate closure, INCLUDING the two
master sites still stating the retired fault — `architecture.md:196` (§Infrastructure Patterns *Build system*) and
`security-plan.md:177` (§Dependency Security) — re-based in that wrap. Deliberately NOT amended here: the no-op path may
apply only self-produced facts, and a reality↔spec divergence noticed here waits for its chunk wrap (SKILL step 6).

### 3 · CARRY on *Dependency polish* (`:119`) — APPLIED (factual → AUTO, operator-directed)
knip as a ui devDependency + `knip` script (audit A5 web column `dead-code-web: tool-missing`, proposals.md §Skips :187);
StrykerJS DECLINED by the founder 2026-09-05 — the next audit record lists it under `skips[]` as `declined` (reason:
small UI; the e2e/a11y legs cover it), never `tool-missing` (:188).

### 4 · CARRY on *Release build and bundle* (`:123`) — APPLIED, mechanism labeled hypothesis
The operator's measurement ("`0 passed, 1 failed` with exit 0" at the SR chunk) was NOT found by grep in the SR chunk
folders, the 2026-09-04 wrap run dirs, `runs/` or `crates/conductor-tauri/ui/logs/`. Source facts verified at HEAD and
written into the CARRY: `scripts/agent-run.ps1:119-120` DOES read `$LASTEXITCODE` after `& npm run a11y`; `agent-run.sh:18`
+ `:105` run it under `set -euo pipefail`; `wdio.conf.ts:269` / `:288` `process.exit(0)` are the handle-unset SKIP paths.
So the leak, if it reproduces, sits below npm — carried as `hypothesis:` with a re-measure-at-take-up instruction, per
route-resolve's causal-claim rule. Remedy as ruled: the ps1 arm asserts the runner's PRINTED verdict; mirror in sh.

### 5 · Self-produced-fact amendments — APPLIED
- (a) `.claude/rules/security.md` Session Additions, 2026-08-09 entry — P3 §Corrections in-place correction, tagged
  `[corrected 2026-09-05: …]`: the "committed data in RustSec's advisory-db" clause measured false; the corrected rule
  (read the external artifact's CURRENT state — porcelain-clean cache or a fresh clone — before trusting a red) added.
  Exempt from the max-3 cap; 0 new Tier 1/2/3 entries; 0 filter rejections; no conflicts; nothing deferred.
- (b) `.andromeda/playbook.md` external-decay rule (:93): the current-state arm inserted BEFORE the tool/database fork;
  the "committed data" clause corrected in place; the CI-converse discriminator appended (CI green + local red = local
  fault). Mirrors the seeded template (`seed-templates/playbook.md:34`: "read from the external artifact's current state,
  never only a local copy of it").
- (c) `.andromeda/drift-base.md` D-platform-claim: invariant + check tightened — the trigger is a sentence in the doc that
  STATES the retired verdict, QUOTED in the proposal; a bare token, a descriptive mention, or a plan's TARGET-state CI
  arrangement is never a hit. Comment block cites the mis-fires: `2026-09-04T17-15-00-wrap/fanout-results.md` E2
  (design-system ×3, no rule matched) and `2026-09-04T20-15-00-wrap/escalations.md` E2 (test-plan §9 T4 + T5, dismissed
  as an unimplemented plan). `.andromeda/playbook.md:46` widened from plan↔plan-bind detectors to any report-triggered or
  token-keyed cross-doc detector, with an Extended 2026-09-05 clause naming the instance.

### 6 · `scripts/code-graph-cookbook.md` synced from the installed template — APPLIED
md5 `f41c1f86…` → `c691f620…` (matches the directive's expectation); diff 1 insertion / 1 deletion — the `line` / `def_line`
0-INDEXED rule at `:80` (cite `line + 1`). The two untracked boundary-#4 run dirs ride this commit via `git add -A`.

## Not done here, and why
- No master flip, no flip-compaction, no light gate, no coverage-gate flip — none run on the no-op path (coverage stays
  26/32 verified · 6 unclaimed: v2-04 · v2-21 · v2-24 · v2-26 · v2-27 · v2-32; no capability claimed).
- No code-graph refresh (source-free delta); `.andromeda/cache/tree.db.commit` is re-pointed to the new HEAD after the
  commit so the next phase does not rebuild needlessly.
- `pulse-app` (PID 62628 per the prior handoff) measured NOT running at session start; `:4317` has no listener.
