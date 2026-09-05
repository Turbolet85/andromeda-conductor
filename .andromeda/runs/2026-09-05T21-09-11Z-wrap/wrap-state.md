# Wrap in flight — 2026-09-05-audit-corrective

**Paused at:** P2 mid-cascade (bodies + sidecars APPLIED; the cascade sweep has run and its routing is decided,
the leaf re-derivation is not yet done). Context hit the 95% hard stop.
**Nothing is committed.** Master still says `pending`; the working tree holds implement's code plus this wrap's
spec amendments.

## Done

- **Setup** — 1 pending (`2026-09-05-audit-corrective`, master `:120`); branch `build/conductor-0.2.0`,
  **1 ahead** of origin; 42 changed paths. `state.yaml`: `last_wrap` 2026-09-05T16:46:25Z · `session_count` 117.
- **P4 code-graph (done early)** — `rust 2503 nodes / 11721 edges (26s)` · `ts 702 nodes / 1570 edges (2s)`.
  **`tree_db_refreshed_at` for P6 = `2026-09-05T21:09:00Z`.**
- **P1 report** — `chunks/2026-09-05-audit-corrective/report.md`, 289 lines. The fan-out's single source.
- **P2 fan-out — all 7 returned. 16 proposals.**
  | doc | proposals |
  |---|---|
  | arch | 1 (D-platform-claim, substantively the advisory-DB re-base) |
  | security-plan | 2 (D-security-deps primary + 1 `dependent-of`) |
  | test-plan | 13 (7 roster/count · 6 gate-form, incl. a SIXTH site the report missed) |
  | design-system · layout-templates · obs-plan · a11y-plan | 0 each — clean, reasons in `fanout-results.md` |
- **P2 validate** — all 6 checks run. Routine under the spec-wording→sound-impl rule; the two `escalate`-severity
  security-plan proposals were **pre-resolved by operator directive item 5** (it names both sites and the
  disposition), so no HALT was owed. **0 escalations open.**
- **P2 apply — 16 amendments landed, bodies verified before each sidecar:**
  - `architecture.md` §Infrastructure Patterns (Build system) — retired "external advisory-DB fault"; both
    runners green; porcelain-check-precedes-external rule stated.
  - `security-plan.md` §Dependency Security ×2 — the 2026-08-09 red re-adjudicated LOCAL; the red-audit
    dependency-admission regime marked CLOSED.
  - `test-plan.md` ×13 — six `--deny warnings` → bare `cargo audit`; §12 coordinates re-pointed
    (`preconditions.rs:50:5` · `execute.rs:95:27` · `execute.rs:136:25` ×2 · `canary.rs:58:8` ·
    `commands.rs:308:5`); a FIFTH roster member (conductor-cli tty-gate PAIR) added to §12 and §10; §4 + §12
    exit-3 samples dated; a present-tense duplicate coordinate in §12's `declares` entry dated.
  - Sidecars appended to all three `*-amendments.md`.
  - **Site-count correction recorded in the test-plan sidecar:** the report said FIVE `--deny warnings` sites on
    the basis `grep -n 'cargo audit'`; the true count is **SIX** (`grep -n -- '--deny warnings'`) — §12's
    Trigger-tooling entry writes it hyphenated and carries no `cargo audit` token. A claim about a FLAG must be
    swept on the flag.
- **P2 cascade — sweep RUN, routing decided, leaf re-derivation NOT yet done.** Results:
  | hit | routing |
  |---|---|
  | `.claude/docs/commands.md:36` — `cargo audit --deny warnings` | **LEAF — re-derive** (test-plan §Standard-Contracts leaf). THE ONE OUTSTANDING CASCADE EDIT. |
  | `.andromeda/test-plan.md:600` | already amended (the dated §12 correction legitimately names the retired flag) |
  | `.andromeda/architecture.md:196` | already amended (states what it is NOT) |
  | `.andromeda/security-plan.md:176` | **deliberately untouched** — the fault-CLASS definition asserts nothing about the current gate |
  | `.andromeda/playbook.md:93` | **judgment base — NOT a cascade edit.** Routes to propose→approve→append. Its external-decay rule's worked example still reads the 2026-08-09 red as an advisory-DB fault. |
  | `.claude/rules/security.md:50` | **curation home (`## Session Additions`) — NOT a cascade edit.** Routes to **P3 curation** as an in-place §Corrections extension of the 2026-08-09 entry (it already carries a `[corrected 2026-09-05]` tag from the 0-pending wrap naming the residue; check whether it now needs the CLOSURE too). |

## Remaining, in order

1. **Finish P2 cascade** — re-derive `.claude/docs/commands.md` (the `--deny warnings` line → bare `cargo audit`)
   and any other leaf whose provenance header names the three amended masters: `docs/{stack,conventions,commands,
   gotchas}.md`, `docs/{security,tests}-summary.md`, `.claude/rules/{security,testing,verification-harness}.md`,
   CLAUDE.md `GENERATED:setup:*` (warnings block is built from the plans' §Anti-Patterns). **Re-derive = recompute
   from the amended master, never a scan for the amended wording.**
2. **P2 playbook proposal** — propose the `playbook.md:93` correction to the operator (its worked example cites the
   retired external-fault reading); on approval append. Do NOT edit it directly.
3. **P3 curation** — the `.claude/rules/security.md:50` in-place correction above; plus the report's
   *Decisions & corrections*. **Operator directive item 4 also lands here or at the P2 evolve checkpoint:**
   append a retraction record per evolve-system §Retraction targeting friction id **`2026-09-05T20:33:45Z-c`**
   (`scope: "record"`) — the block-8 gate fired on a REAL host path (the resolved `$CARGO_HOME` in the ledger's
   audit transcript before redaction), so it is the gate WORKING, not `contract.vacuous-check-found`.
4. **P4** — already measured; just record it (numbers above).
5. **P5 route-resolve** — **close the standing cargo-audit deferral since 2026-08-09** on Test Commands #7 green;
   the P5 summary must record "deferral since 2026-08-09 closed" (route-resolve §Deferred-gate closure). The pin
   currently sits on this chunk's own (now-frozen) working-route line and must NOT be re-pinned forward.
6. **P6** — `state.yaml` (`last_wrap` = now · `session_count` **117 → 118** · `tree_db_refreshed_at`
   `2026-09-05T21:09:00Z`); `session-handoff.md` full overwrite.
7. **P7** — light gate (the plan's Test Commands; all were green at implement — the five mutation tiers are the
   expensive part and the `conductor-run` tier measured **13 min**, over the 10-minute foreground cap, so it must
   run detached and be polled) · drift = 0 gate · coverage gate (**no-op — 0 matrix entries name this marker**) ·
   master flip `pending → complete` + flip-compaction · `git add -A` + commit · stamp
   `.andromeda/cache/tree.db.commit` = the new HEAD.

## Facts a resumed session must not re-derive

- Gates: nextest **873** (from 853) · `cargo test -p` ×6 green (cli 51 · core 323 · verify 112 · tauri 26 ·
  report 50 · run 177) · doctest 0 · clippy 0.
- Mutation: cli **58.59 → 97.85** (41 → 2 = the accepted pair) · verify timeouts **2 → 0** · `civil_from_unix`
  **0 missed** · tauri 3 accepted unchanged · run **118 mutants both runs**, 5 missed = the accepted set.
- jscpd **90 → 84**, zero NEW pairs. `conductor-run` public API **27 items, diff identical**. `Cargo.lock`
  untouched at **564** packages.
- Audit probe: porcelain empty, `cargo audit` **exit 0** (1239 advisories · 564 deps · 18 allowed warnings),
  `cargo deny` exit 0.
- Smoke: minted `2026-09-05T20-34-45-675`, read back through `conductor report`.
- Process census: nothing this run started survives; no `:4317` listener; `pulse-app` not running.

## Operator directive — disposition

1. `testkit.rs` in Deviations + Touchpoints — **DONE** (report §Deviations 1, §Changes).
2. The 2 + 2 jscpd pairs dispositioned explicitly, never "absent" — **DONE** (report §Outcome).
3. The 7 `obs.rs` survivors' OWNER named — **DONE** (report §Outcome: boundary #5's full `conductor-core` tier).
4. The `contract.vacuous-check-found` retraction — **OWED** (item 3 above; target id `2026-09-05T20:33:45Z-c`).
5. P5 closes the deferral; amendments 1/3/6 through P2 — **P2 half DONE**, P5 half owed (item 5 above).
