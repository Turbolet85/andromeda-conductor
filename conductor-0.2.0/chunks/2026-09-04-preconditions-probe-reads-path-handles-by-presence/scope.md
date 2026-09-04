# Scope — Preconditions probe reads path handles by presence

**Marker:** `2026-09-04-preconditions-probe-reads-path-handles-by-presence`
**Version:** conductor-0.2.0 · **Epoch:** 6a — Verification follow-ups
**Working entry:** _Preconditions probe reads path handles by presence — `handles-declared` is satisfiable and `boot` reaches the preflight_

---

## What this chunk builds

`conductor preconditions` currently **cannot exit 0 under any environment, however correct**. The
`handles-declared` subject grades all three `ANDROMEDA_PULSE_*` names through one truthy-only
predicate, and one of those handles carries a **path**, not a flag — so it can never satisfy the
gate. Because `agent-run boot` runs the probe as a leading short-circuit arm, **no environment has
reached the preflight through `boot` since `480bc66`**.

This chunk makes the subject satisfiable by grading each handle **by the kind of value it carries**:
presence for the path handle, affirmative truthiness for the two flags. It then proves the
consequence the entry names — that `boot` reaches the preflight — in both shells.

## The defect, as measured

| Coordinate | Verified at promotion |
|---|---|
| `crates/conductor-run/src/lib.rs:359` — `fn declares()` accepts only `"true"` / `"1"` (trimmed, lowercased) | ✔ read at HEAD |
| `crates/conductor-run/src/lib.rs:380-384` — `observe_preconditions` filters **all** of `OBSERVED_HANDLES` through `declares()` | ✔ read at HEAD |
| `crates/conductor-core/src/preconditions.rs:28-32` — `OBSERVED_HANDLES` = `[ANDROMEDA_PULSE_DATA_DIR, ANDROMEDA_PULSE_L4_DETERMINISTIC, ANDROMEDA_PULSE_MCP_ENABLED]`; the **first is PATH-valued** | ✔ read at HEAD |
| Live measurement 2026-09-04T06:41Z: `egress-reachable` ✓ · `sidecar-resolvable` ✓ · `handles-declared` unmet naming `ANDROMEDA_PULSE_DATA_DIR` alone · exit 1, with a child process independently confirmed to receive the value intact | ✔ `chunks/2026-09-04-sr-findings-remediation/report.md` §Spec claims disproved #1 |
| `scripts/agent-run.sh:84-87` + `scripts/agent-run.ps1:86-90` — `boot` short-circuits `exit 1` on a non-zero probe, skipping the preflight | ✔ read at HEAD |
| `480bc66` introduced that arm | ✔ `git show --stat` |
| The existing tests cannot catch it | ✔ every test in `preconditions.rs` builds `PreconditionObservation.declared` directly via the `declared(&[…])` helper; **none** exercises the env read |

**Corroboration worth recording:** `480bc66`'s own commit message states the probe reads "the three
`ANDROMEDA_PULSE_*` handles read **presence-only**". That is what the shipping chunk believed it
built; the code has always applied the truthy gate. The belief, not the intent, is what drifted.

## Boundaries

**In scope**
1. **Per-handle grading at the observation site.** The path handle is satisfied by presence (a
   non-empty value); the two flag handles keep the affirmative-truthiness gate, so an explicit
   `false` still declares the opposite of the term it would otherwise satisfy.
2. **A unit test over the ENV READ itself.** The gap is precisely that the pure evaluator is
   well-tested and the observation site is not tested at all. The new test must exercise the
   read → grade path, not reconstruct the `declared` set by hand.
3. **`boot` proven to reach the preflight** in both `agent-run.sh` and `agent-run.ps1` (test-plan
   §3 binds .sh/.ps1 parity).
4. **The two `declares()` call sites kept distinct.** `observe_run_contract` (`lib.rs:351`) grades
   the run contract's `shell-declaration` terms, whose `observed_env()` is a **narrower,
   flag-only** set (`ANDROMEDA_PULSE_L4_DETERMINISTIC` · `ANDROMEDA_PULSE_MCP_ENABLED`).
   Truthiness is *correct* there. A fix that loosens the shared predicate would silently widen the
   run-contract gate — the failure mode to design against.

**Out of scope**
- The pure evaluator `Preconditions::evaluate` and its ten tests. The defect is at the
  observation site; the seam's judgment is sound and its `declared: BTreeSet<String>` input shape
  is the contract both sides already honour.
- `OBSERVED_HANDLES` membership. The set is pinned in both directions by
  `the_observed_handle_set_is_pinned` and is correct — this chunk changes how a handle is graded,
  never which handles are observed.
- The run contract's own terms, `RunContract::observed_env`, and the preflight gate's five named
  preconditions.
- The `sidecar-resolvable` and `egress-reachable` subjects.

## Surfaces and contracts touched

- `crates/conductor-run/src/lib.rs` — `observe_preconditions` (the observation site) and the
  grading predicate(s) beneath it.
- `crates/conductor-core/src/preconditions.rs` — **only if** the per-handle kind must be declared
  as data beside `OBSERVED_HANDLES`; the evaluator stays untouched either way. `[inferred]`
- `scripts/agent-run.{sh,ps1}` — the `boot` arm's behaviour is *asserted*, not necessarily edited.
- Spec re-verification surface: the five masters amended at the `2026-09-04-sr-findings-remediation`
  wrap **record the defect**, not a fix — architecture §Standard Contracts (Liveness equivalent),
  §Environment variables (`CONDUCTOR_PREFLIGHT_TIMEOUT`, `ANDROMEDA_PULSE_MCP_ENABLED`),
  `.claude/rules/verification-harness.md`, and the CLAUDE.md warnings block. When this fix lands
  they become stale-in-the-other-direction and are wrap's reconcile surface.

## Open premises (closed at P3 — see `research.md` §Scope premise closure)

- ~~`[inferred]`~~ **The test seam for the env read — VERIFIED.** The in-repo precedent is
  `conductor_verify::sidecar_resolves_on_path` / `resolves_on` (`spawn.rs:98-123`): a thin
  env-reading wrapper delegating to a **pure inner function taking values**, whose doc comment
  names this exact rationale ("both arms are reachable in a test with no env mutation (which
  edition 2024 makes `unsafe`)"). Not a threaded lookup parameter — a wrapper/pure-half split.
- **How "presence" is defined for the path handle — RESOLVED as a lean:** *set and non-empty after
  trim*. `declares` already trims (`lib.rs:360`) and an empty data-dir cannot satisfy the
  precondition it stands for. No artifact contradicts it; stated in the plan, not asked.
- **What proving "`boot` reaches the preflight" costs —
  `[premise-corrected: the witness is the PRESENCE of `ReadyState` JSON on stdout, not a green
  preflight. `verification-harness.md:18` records "no `ReadyState` JSON is emitted, which is how a
  skip is told from a failure" (measured 2026-09-03, exit 1 in both shells), and `:45` states that
  with no live Pulse `boot` is `ready:false` → non-zero BY DESIGN. The proof therefore does not
  need a green gate — but it does need the probe to exit 0, hence all three subjects, hence a live
  Pulse on `:4317` + the sidecar on `PATH` + the handles: an operator-gated live leg, never CI.]*
- ~~`[inferred]`~~ **CLI human/`--json` output shape — VERIFIED unchanged.** The payload is built
  in `commands/preconditions.rs:26-33` from the status; the fix changes only which handles land in
  `declared`, strictly upstream of the evaluator.
- `[inferred]` **Whether the per-handle KIND lives in `conductor-core` or at the `conductor-run`
  observation site — STILL OPEN, and a genuine fork:** architecture.md sanctions both placements,
  so no artifact decides it. It sets the modify-set (2 crates vs 1). Carried to the P4 fork.

## Absorbed at P3 (not anticipated by the working entry)

- **The mutation roster is an inherited obligation that should SHRINK.** test-plan §12 `:608`
  records the `declares` accepted-deliberate set as exactly four members (`lib.rs:358:5` → `false`,
  `:360:11`, `:360:21`, `:360:26`), all inside `declares`'s own body. `.claude/rules/testing.md`
  (2026-09-03) requires a chunk that TOUCHES an accepted class to re-run it rather than carry the
  count — and splitting the grading into a pure predicate should move most of those arms into
  killable value-taking code.
- **The satisfied branch is dead code today** (`commands/preconditions.rs:33`,
  `render.rs:209-211`) — this chunk makes it reachable for the first time.
- **No obs and no a11y work is owed** — the probe's self-obs lines already carry `run_id`
  (`main.rs:27-28`), and no a11y leg routes through `boot` (they ride `run --e2e`).
- **A live `boot` leg fires a canary**, so a fresh data dir per shell keeps two-shell evidence clean
  (`verification-harness.md:50` / `:53`) — hygiene, not a correctness requirement for this witness.

## Folded annotations

**PREREQ (standing, external decay) — the 50th re-pin.** Re-check `cargo audit`. Standing deferral
since 2026-08-09, ratified at the 2026-08-10 wrap. Basis: the RustSec advisory database itself will
not parse — a **database** fault, not a tool fault, so no floor raise exists to make (cargo-audit
0.22.2, the latest, reproduces it byte-identically). Overlap: `cargo deny check advisories bans
licenses sources` runs green every chunk. Full rationale: the
`2026-08-09-interpretation-correctness-posture` chunk report.

- **SIGNATURE:** `cargo audit` exit **1**, first diagnostic
  `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`;
  overlap `cargo deny` exit **0**. Reproduced byte-identically at the
  `2026-09-04-sr-findings-remediation` wrap (the 49th; package count 564 unchanged). **This is the
  50th.**
- **Read the exit DIRECTLY, never through a pipe** (`| head` / `| tail` report the pipeline's last
  stage — a measured green-masking-red failure on this host).
- Any deviation from the signature restores the FULL form and is reported.
- Per the standing security rule, **both** the audit probe and the deny overlap must appear in the
  plan's `## Test Commands` with their dispositions, or the wrap cannot re-pin the deferral.

No `CARRY` and no `BLOCKED-ON` on this entry.
