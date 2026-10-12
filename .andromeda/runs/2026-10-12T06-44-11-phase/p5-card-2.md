# P5 review, card 2 — 2026-10-12-panel-shaped-types-retired · the four edits folded

Card 1 (`.andromeda/runs/2026-10-12T06-44-11-phase/p5-card-1.md`) stands, apart from what follows. Your word on it
(`inputs#I4`) was a `yes` that carried four plan edits, so the edits are folded, the whole mechanical set is re-run,
and this card asks again.

## The four edits, as folded
1. **Step 6 names the SIGINT mechanism.**
   - Taken: the test binary's own fixed-argv spawn, `std::process::Command::new("kill").args(["-INT",
     &std::process::id().to_string()])`. A cargo test binary's own spawn is outside harness-spawn rule (b), per the
     `secret_scan_gate` precedent of 2026-09-24. No dependency is added, and the lock stays 268.
   - Not taken: `libc::kill` through a `libc` dev-dependency. It is an `unsafe` FFI call, which the security rules
     forbid without review. The plan's Constraints records why.
2. **v4-09's decision is in `plan.md`.** A new `## Acceptance Criteria` bullet says v4-09 stays pooled, gives the
   reason (`:47`, `:49`, `:107`, `:113`), and promises a dated note at P5 step 4 naming what this chunk adds toward
   it.
3. **Your acceptances are recorded under v4-01's criterion.**
   - "Following" is witnessed below the binary, and the SIGINT test runs on the Linux host until `Linux-only base CI`.
     Both are accepted, and v4-01's matrix note will say both in words.
   - The second Ctrl-C stays unwitnessed by your word, with no owner (Implementation notes).
4. **Step 9 names the resolver.** `PromptResolver::resolve` (`crates/conductor-cli/src/pause.rs:34-37`) prints
   `hold_line` and calls `confirm`, so the checklist lines go there, inside its spinner-suspended closure.
   `CliResolver::resolve` (`:114-119`) only dispatches and is unchanged. Research's two citations and `scope.md` item
   11 are corrected to match; scope carries a `premise-corrected` mark citing `inputs#I4` (4).

## Mechanical set, re-run whole after the folds
- `gate.py run --dry-run`: 24 entries, it parses.
- planlint: 0 hits (check 4 (17) still lists only the masters entry, read as before).
- `gate.py scope --lists-only`: 27 items parsed.
- `gate.py baseline`: 5 seen green, 16 new, 3 operator legs. Every new entry still carries its `baseline`; no `run`
  text changed in the fold.
- All modify paths exist. 371 prose lines. No `[inferred]` bullet left.

## Concretization preview — v4-01
Unchanged from card 1. The claim's `--note-file` will read, dated 2026-10-12: "following" is witnessed below the
spawned binary; the SIGINT test runs on the Linux dev host only until `Linux-only base CI`; the second Ctrl-C's exit
130 stays unwitnessed — each by the operator's word, `inputs#I4`.

v4-09: not claimed; one dated note, as card 1 said.

## Beyond the plan
- **Deferral to an existing entry:** until `Linux-only base CI — one Linux job keeping test, coverage-floor,
  zero-retry, supply-chain, secret-scan, static, own-log conformance and unlogged-panic gates; Windows runners,
  PowerShell harness twin leave (v4-07)` makes CI Linux, the unix SIGINT test is witnessed only on the Linux dev host.
  Accepted by your word (`inputs#I4` (3)); no edit to that entry.
- **Accepted without a witness:** the second Ctrl-C's immediate exit 130. It stays unwitnessed by your word
  (`inputs#I4` (3)), with no owner.
