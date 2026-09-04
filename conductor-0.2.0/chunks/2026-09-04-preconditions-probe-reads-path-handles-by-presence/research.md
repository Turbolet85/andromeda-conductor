# Codebase Research — 2026-09-04-preconditions-probe-reads-path-handles-by-presence

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 7 · **Graph queries:** 3 (rust plane, all non-empty)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural
  extraction (59 lines / 43.4 KB; lines 18, 47, 49, 53, 58, 59 each run 2–13 KB), index by header +
  per-entry introducer, then offset-bounded reads covering every indexed span. `.claude/rules/testing.md`
  auto-loaded on the `crates/**/tests/**` read. Both carry decisive facts below.

## Files inspected
- `crates/conductor-run/src/lib.rs` (330–410, 1640–1700) — the defect site, its sibling call site, and the
  run-contract tests.
- `crates/conductor-core/src/preconditions.rs` (full, 340 lines) — the pure evaluator + its 10 tests.
- `crates/conductor-cli/src/commands/preconditions.rs` (full) — the verb, its exit rule and `--json` shape.
- `crates/conductor-cli/src/render.rs` (113–221) — `preconditions_caption` / `_styled`.
- `crates/conductor-cli/src/main.rs` (10–80) — obs init + dispatch.
- `crates/conductor-cli/tests/cli_smoke.rs` (355–450) — the three existing probe edge tests.
- `crates/conductor-verify/src/spawn.rs` (98–123) — `sidecar_resolves_on_path` / `resolves_on`.
- `scripts/agent-run.sh` (78–89) · `scripts/agent-run.ps1` (80–100) — the `boot` leading arm, both shells.
- `.andromeda/test-plan.md` (§10 :501, §12 :608–610) — the mutation roster.

## Graph impact (rust plane, `db_state` warm; every probed name resolved)
- **`declares`** — **exactly 2 callers**, both in `conductor-run/src/lib.rs`:
  `observe_run_contract` @ `lib.rs:350` and `observe_preconditions` @ `lib.rs:381` (graph lines are
  0-indexed; grep reads `:351` / `:382`). This is the whole blast radius of the predicate, and it is
  precisely the two-disposition split scope named as boundary #4.
- **`OBSERVED_HANDLES`** — 12 refs, confined to two crates: `conductor-core/src/preconditions.rs`
  (`:84` `undeclared()` + 8 test sites), `conductor-core/src/lib.rs:54` (re-export),
  `conductor-run/src/lib.rs:25` (import) and `:379` (the observation site). **No third crate reads it**,
  so changing its TYPE would touch 12 sites in 2 crates while adding a sibling touches none.
- **`observe_preconditions`** — **exactly 1 caller**: `conductor-cli/src/commands/preconditions.rs:19`.
  No Tauri caller, no test caller.

## Patterns detected
- **Thin env wrapper over a pure inner function taking values** (`spawn.rs:98-123`): `sidecar_resolves_on_path()`
  reads `PATH`/`PATHEXT` and delegates to `resolves_on(path, path_ext, program)`, whose doc comment states the
  rationale verbatim — *"every input arrives as a value, so both arms are reachable in a test with no env
  mutation (which edition 2024 makes `unsafe`)"*. This is the in-repo precedent for this chunk's test seam,
  it is cited by security-plan as the boolean-grade-fact pattern, and it sits in the very module the probe's
  second subject already calls.
- **Pure evaluator, observation at the caller** (`preconditions.rs` module doc + `Preconditions::evaluate`):
  the judgment takes `PreconditionObservation` by value and reads no env, no socket, no filesystem. The
  defect is entirely upstream of it, in how `declared` is BUILT.
- **Host-dependence handled by asserting shape, not arm** (`cli_smoke.rs:358-363`): the helper's doc comment
  says *"The verdict is host-dependent by design … the assertions below key on the SHAPE the probe must always
  emit, never on which arm it took"*, and `preconditions_names_every_subject_it_reports` guards its
  unmet-specific assertions behind `if !satisfied`.
- **`declares` already trims** (`lib.rs:360`): `v.trim().to_ascii_lowercase()` — trimming is the established
  local convention for these handles.

## Conventions to follow
- **Env at the caller, typed value into the gate** — `.claude/rules/testing.md` (2026-08-10) and
  `.andromeda/test-plan.md` §12 :608. The env read stays in `conductor-run`; anything graded stays pure.
- **Roster shrinks under compulsion** — `.claude/rules/testing.md` (2026-09-03, conductor-run survivors):
  an accepted-deliberate classification covers exactly the code the cited rule's mechanism lives in, does
  not inherit up to the enclosing function, and **when a later chunk touches an accepted class it re-runs it
  rather than carrying the count forward**. Record the roster as the enumerated SET, never a count.
- **Word-anchored drive-letter token** in any host-path negative test — `\b[A-Za-z]:[\\/]`
  (`.claude/rules/testing.md` 2026-09-03); `cli_smoke.rs:436-447` already implements it.
- **Both runners green** — `cargo nextest run -p <crate>` and `cargo test -p <crate>` (testing.md §Frameworks).

## New files to create
- (none) — every change lands in existing files.

## Files to modify
- `crates/conductor-run/src/lib.rs` — the observation site (`observe_preconditions`, `:376-390`) and the
  grading predicate(s) beneath it. `observe_run_contract` (`:347-355`) keeps calling `declares`.
  **[revised at the P5 review]** This line originally read "keeps `declares` unchanged". Freezing
  `declares` was measured self-defeating: all four of its accepted-deliberate mutants sit inside it
  (`:360:11`, `:360:21`, `:360:26` are the two `==` and the `||` on `v == "true" || v == "1"`; `:358:5` is
  the body), so a freeze makes the step-8 roster re-run a no-op while duplicating the truthiness rule in
  core. The shipped design delegates `declares`'s value test to a value-only `flag_declared` — identical
  semantics, one rule, and the run-contract path still cannot reach the presence arm. See plan.md step 5.
- `crates/conductor-core/src/preconditions.rs` — **only under design option A** (per-handle kind as data
  beside `OBSERVED_HANDLES`); untouched under option B. See Open questions.
- `crates/conductor-cli/tests/cli_smoke.rs` — **no change required** (verified: the three probe tests key on
  shape, not arm, and the satisfied branch already passes the `[PRECONDITION]` assertion). Listed only
  because it is the boundary that would show a regression.
- `.andromeda/test-plan.md` §12 :608 — the `declares` roster. **Expected amendment (wrap), not a touchpoint.**

*Caller threading:* the graph's caller set for both changed symbols is closed and small — `declares` has 2
callers (both in the modified file), `observe_preconditions` has 1 (the CLI verb, which passes the status
through unchanged and needs no edit). No signature reaches another crate: `PreconditionObservation.declared`
stays `BTreeSet<String>`, so the `conductor-run → conductor-core` seam is byte-stable and no re-export moves.
Seam fact: `conductor-core` is a NORMAL dependency of `conductor-run` (not dev-only), so a pure predicate
added there can back shipped code.

## Scope premise closure

- **`[inferred]` test seam → VERIFIED, and sharpened.** `sidecar_resolves_on_path` / `resolves_on`
  (`spawn.rs:98-123`) is the precedent, with its doc comment naming this exact rationale. The seam is a thin
  env-reading wrapper delegating to a pure predicate that takes the value — not a lookup-function parameter
  threaded through the caller. Tag dropped.
- **`[inferred]` cost of proving `boot` reaches the preflight → PREMISE-CORRECTED.**
  `[premise-corrected: the discriminator is the PRESENCE of `ReadyState` JSON on stdout, not preflight
  greenness — `verification-harness.md:18` records "no `ReadyState` JSON is emitted, which is how a skip is
  told from a failure (measured 2026-09-03, exit 1 in both shells)", and `:45` states `boot` with no live
  Pulse is `ready:false` → non-zero BY DESIGN. So the proof does not need a GREEN preflight. It does need the
  probe to exit 0, which needs all three subjects — a live Pulse on `:4317` plus the sidecar on `PATH` plus
  the three handles — so it is an operator-gated live leg, never CI (test-plan §9).]*
- **`[inferred]` CLI `--json` shape → VERIFIED unchanged.** The payload
  (`{satisfied, unmet:[{subject,statement,causes}], checked_at}`) is built in `commands/preconditions.rs:26-33`
  from `status`; the fix changes only which handles land in `declared`, strictly upstream of the evaluator.
  Subject set, ids and finding shape are untouched. Tag dropped.
- **`[inferred]` presence definition → RESOLVED as a lean:** *set and non-empty after trim*. `declares`
  already trims (`lib.rs:360`), and an empty/whitespace data-dir cannot satisfy the precondition it stands
  for. No artifact contradicts it; stated in the plan rather than asked.
- **`[inferred]` `conductor-core` may need per-handle-kind data → still OPEN**, and it is a genuine fork —
  architecture.md explicitly sanctions BOTH placements. Carried to Open questions.

## Findings the scope did not anticipate

1. **The satisfied branch is dead code today.** `commands/preconditions.rs:33`
   (`println!("[PRECONDITION] every live-Pulse precondition is satisfied")`) and
   `render.rs:209-211` (`preconditions_caption` returning `None`) can never execute as shipped, because the
   probe cannot be satisfied. This chunk makes both reachable for the first time — they are unexercised
   paths, not proven ones.
2. **The mutation roster is an inherited obligation, and it should SHRINK.** test-plan §12 :608 records the
   `declares` accepted-deliberate set as exactly four members — `lib.rs:358:5` (→ `false`), `:360:11`,
   `:360:21`, `:360:26` — all inside `declares`'s own body, cited to the env-at-the-caller rule. Splitting the
   grading into a pure predicate moves those arms into value-taking code that is killable with no env
   mutation, so the set should shrink to the thin wrapper's read alone. testing.md (2026-09-03) makes
   re-running it mandatory rather than optional for a chunk touching the class.
3. **The existing edge tests will not break.** Verified by reading them: host-dependence is deliberate and
   documented, and the satisfied path still emits `[PRECONDITION]`.
4. **A11y legs do not route through `boot`.** `package.json` / `wdio.conf.ts` name no `boot` invocation —
   the routine arm is `run --e2e`. This answers the a11y extract's open research question: no a11y leg is
   blocked by the short-circuit today, and none is unblocked by the fix.
5. **The probe's self-obs lines already carry `run_id`.** `main.rs:27-28` mints a `run_id` and calls
   `init_observability("conductor", Some(run_id.clone()), …)` before dispatch, so the `tracing::info!` /
   `warn!` at `lib.rs:393-399` inherit it. This answers the obs extract's open question — obs-plan §3's base
   schema is already satisfied on the probe path and no obs work is owed.
6. **A live `boot` leg fires a canary, so leg hygiene applies.** `verification-harness.md:50 / :53` — a
   preflight canary leaves an OPEN incident that a following canary dedupes against. Two `boot` legs (one per
   shell) on one data dir would make the second `ready:false` on dedupe. **That does not invalidate the
   proof** (the witness is JSON emission, not greenness), but a fresh data dir per shell keeps the evidence
   clean for one restart's cost.

## Open questions
- Where does the per-handle KIND live — a sibling in `conductor-core` beside `OBSERVED_HANDLES` (one pinned
  list, but the kind is core's), or a match at the `conductor-run` observation site (grading is the
  observer's policy, core untouched)? architecture.md §Occupied Resources sanctions both, so no artifact
  decides it. → blocks: **plan-decision** (it sets the modify-set: 2 crates vs 1).
- Does the operator want the live two-shell `boot` proof run this chunk, given it needs a live Pulse plus the
  full five-handle firing form? → blocks: **plan-decision** (it decides whether the `boot` criterion is
  measured here or carried).
