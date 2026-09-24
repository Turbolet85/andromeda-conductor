# Scope — Secret-scanning CI gate

**Marker:** `2026-09-24-secret-scanning-ci-gate` · **Version:** conductor-0.3.0 · **Epoch:** Epoch 5 — Polish & ship
**Working entry:** `conductor-0.3.0/working-route.md:55` (taken up by `--chunk`; the markerless heads `:50` and `:52`
were skipped with their `BLOCKED-ON` premises re-verified at `4837210` — no real-model drive newer than
`runs/2026-09-23T07-39-39-845`, Pulse HEAD still `83d4060`, the commit the P-025 contract pins).

**Entry intent (verbatim title + hint):** Secret-scanning CI gate — no secret-shaped string in the workspace, the key
and cert ignores present, the build red on any hit.

## What this chunk builds

### A. The secret-scanning gate (the entry's own intent)
1. A CI step that fails the build when a secret-shaped string appears in the tracked `conductor-*` workspace, naming
   each hit (file + line, never the matched secret echoed back in full) — security-plan §Secret Management (`:259`),
   the chunk named at `:224`, and the two NEVER bullets (`:352`, `:356`: "the only thing that enforces the 'no secrets
   ever' invariant over time").
2. The same check reproducible locally (the build is red on any hit — the local run must be able to show it before CI
   does). VERIFIED (research): the project's static gates are crate test targets that `agent-run run`'s
   `nextest run --workspace --profile ci` (`scripts/agent-run.sh:314`) already executes, so a gate in that form has its
   local form for free and adds no sixth command.
3. `.gitignore` entries for the key and cert classes — security-plan `:224` names `*.p12` / `*.pem` / `*.cer` / `.env*`;
   measured at `4837210`: `.gitignore:26-27` carries `.env` and `.env.*`, and NONE of `*.p12` / `*.pem` / `*.cer`.
4. Tool selection: security-plan `:224` / `:259` defer it to setup-project / operator and forbid adopting "an
   unvetted tool"; Decisions Log open question (3) (`:396`) leaves it open. VERIFIED (research, security + arch
   extracts): a scanner fetched at job time would be a second member of security.md's third dependency class and a
   second non-loopback CI egress (arch §Occupied Resources — Ports) — an escalation, not a routine add. Because the
   plan explicitly defers the choice to the operator, P4 ASKS rather than leans.
5. A false-positive discipline: a measured baseline over the tracked tree, a positive control per pattern, and every
   allowlist entry with a stated reason. `[premise-corrected: the anticipated false-positive load does not
   materialise — 14 content patterns + a secret-file-name class over 3 670 tracked text files return 0 hits, and each
   pattern FIRES on a runtime-built synthetic sample (research §Measurements 3); the allowlist therefore starts EMPTY,
   and its discipline governs future entries, graded exact-set in both directions]`.

### B. CARRY folded from `:55` (hypothesis until P3 re-derives it)
6. A CI step "in the shape of the flakiness-budget step" that fails, naming the key, when a `${{ env.X }}` expression or
   a bare `if: env.X` names a key no workflow, job or step declares. Coordinates re-verified at `4837210`: the entry cites
   `ci.yml:61-70`, the directive `:64-72`; measured, the step's comment sits at `.github/workflows/ci.yml:61-63` and the
   step itself at `:64-71` (`:72` is the blank line after it). The trap's record: `.claude/rules/host-win32.md:136`
   (the 2026-09-17 entry — confirmed at that line); the in-workflow comment at `ci.yml:354`.
7. Measured (entry, at `edc0af8`; re-measured at `4837210`): 0 live `${{ env.* }}` / `if: env.*` references in
   `.github/workflows/`; the one hit is the comment at `ci.yml:354`. So the step guards the NEXT occurrence, and needs
   a seeded known-bad input to prove it can fail.
8. Open, to be answered by MEASUREMENT (operator relay F12): whether a key an earlier step writes to `GITHUB_ENV` is
   readable through the `env` context; if it is, such a key must not trip the check. VERIFIED OPEN (research
   §Platform issues consulted): three fetched GitHub docs pages leave it undocumented — the contexts page excludes
   "variables inherited by the runner process", the workflow-commands page shows only a SHELL read of a
   `GITHUB_ENV` key. No local measurement is possible (it is a hosted-runner semantic); P4 decides where the
   measurement comes from and how the gate treats such a key until it lands. Today's exposure is zero: the one
   `GITHUB_ENV` writer (`EDGEWEBDRIVER`, `ci.yml:342`) is read only in the step shell.

### C. Operator relay (overseer1, item 3) — not route freight; folded as a hypothesis
9. `.claude/settings.json:14`'s generated-dir PreToolUse write guard lacks the backslash normalisation. Coordinates
   re-verified at `4837210`: `:14` pipes `jq … | tr -d "\r"` straight into `grep -qE "(^|/)(…|target|…)/"`; the fixed
   template `andromeda-setup-project/references/hooks-matrix.md:89` adds `| tr "\134" "/"` (two JSON backslashes before
   `134`) right after `tr -d "\r"`.
10. Mechanism claim, directive-measured, kept verbatim: "never fires for Write/Edit on this Windows host. It greps
    (^|/)target/ while the tools send backslash paths: 7 of 11 probe paths right, and all 4 backslash targets missed."
    RE-DERIVED at `4837210` (research §Measurements 1): on the stored command `src/x.rs` → 0, `target/x.rs` → 2, and both
    backslash payloads (`target\x.rs`, `C:\p\target\x.rs`) → **0** — the guard is blind to the host separator. The fix
    blocks no tracked file (0 of 3 674 under a guarded directory name).
11. Verification = setup `validation.md` §Hook smoke test step 6 (`:36-41`, re-verified): the command piped AS STORED,
    `printf` OCTAL payloads — `src/x.rs` exits 0; `target\134\134x.rs` and `C:\134\134p\134\134target\134\134x.rs` exit 2.
    Only the backslash arms discriminate. VERIFIED that the smoke runs from the Bash tool and discriminates the unfixed
    guard (research §Measurements 1); the directive's `[[gate]]`-probe form is P4's to confirm against the gate
    contract. The edit goes through the Edit tool (a typed backslash is halved by the Bash transport).

### D. Decisions taken at P4 (operator, 2026-09-24) — val-1 intent-incomplete amendment
12. Scanner form (item 4): an in-repo Rust static gate, `crates/conductor-core/tests/secret_scan_gate.rs`, run by a
    presence-guarded named step in the `rust` job. No fetched tool and no `scripts/` instrument.
13. CARRY form (item 6): the env-context check is a Rust static gate too (`workflow_env_gate.rs`). The flakiness-budget
    step's "shape" survives as the named step's position and its `::error::` presence guard, because an inline `grep`
    cannot carry the in-suite negative arms test-plan §6 requires.
14. F12 (item 8): a key written to `GITHUB_ENV` is ADMITTED, and a two-step CI probe (`GITHUB_ENV context probe
    (write)` / `(assert)`) pins that premise on every run. The operator's push produces the measurement.
15. The ignore set (item 3) widens to the gate's whole secret-file-name class (`.env*`, `id_*` key files,
    `*.pem|p12|pfx|key|cer|crt|jks|keystore`), so the ignore rules and the detector name the same classes.

## Boundaries
- Spec masters are read-only here; the tool-selection record (security-plan Security Decisions Log) and any gate
  registration land through wrap's amendment flow, not a phase or implement edit.
- `settings.json` is not a spec master — its fix is ordinary chunk work.
- Not in scope: the other Epoch 5 entries (`:57` mutation gate, `:59` keyboard/focus claims, `:61` docs/knip/P-ID
  resolution); history rewriting — the gate's subject is the TRACKED TREE at HEAD (VERIFIED clean: research
  §Measurements 3), so git history is not scanned and no rewrite is in scope.
- No new network egress in any shipped binary or dev-host leg; no new CI network fetch without an escalation.

## Surfaces touched
`.github/workflows/ci.yml` · `.gitignore` · `.claude/settings.json` · the gate test target(s) · possibly
`Cargo.toml` / `crates/conductor-core/Cargo.toml` / `Cargo.lock` (a `regex` dev-dependency edge). `[premise-corrected:
the project's static gates live as crate test targets (`crates/conductor-core/tests/scenario_audit_gate.rs`,
`crates/conductor-report/tests/coverage_gate.rs`) run by `agent-run run`'s workspace nextest, not as `scripts/` tools —
a CI-invoked `scripts/` scanner would be the first CI-invoked operator instrument (arch §Stack operator-instruments
row), and no separate local harness entry is needed; the final form is P4's fork]`.

## Folded sources
- Working entry `:55` title + hint + its one `CARRY:` block (991 chars; `route.py pins`: 1 block on `:55`).
- CI verdict at Setup: `4837210` — 3/3 checks `success` (Rust · Frontend · A11y) — no red to fold.
- Operator directive (overseer1) items 1-3.
- No standing audit/advisory re-check pin on the skipped heads (`:50`, `:52` carry `CONTEXT` + `BLOCKED-ON` only).
