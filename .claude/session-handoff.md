# Session Handoff

**Last Updated:** 2026-08-20T22:12:40Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **27 ahead** after this chunk commit)
**Status:** clean
**Last Commit:** 2026-08-20-verifier-self-hardening — the verifier's own machinery gained the assertions
it was missing, and the six it will never have got a reason instead

## Position
- Done: **2026-08-20-verifier-self-hardening** — Conductor's asserting machinery hardened. **22 named
  mutation survivors dispositioned: 16 killed + 6 classified accepted-deliberate.** `conductor-run/src/lib.rs`
  survivors measured **27 → 21** (delta exactly the six journal stamps); `conductor-verify`'s two audited files
  scored **33 caught / 5 missed / 11 unviable / 2 timeout** with zero missed among targeted sites. The
  runner-portability defect is fixed at the cause. nextest **661 → 669** zero-retry.
- Next: **Read-back seam survivors closed** — the new Epoch-4 entry, inserted ahead of severity-lifecycle:
  the five out-of-family survivors (`jsonrpc.rs:41:22` ×2 id-counter · `:65:9` notify · `preflight.rs:357`
  ShapeWitness ×2). `/andromeda-phase` to promote + plan. It carries the **34th audit PREREQ** in
  PROBE-AUTO-SATISFY form.

## Work done
Two new test binaries (`canary_obs_witness.rs`, `jsonrpc_line_bound.rs`) + 6 tests added across
`preflight.rs` (3) and `conductor-run/src/lib.rs`'s test module (3, **test-module only** — touchpoints widened
by founder ruling). `conductor-cli` lost its unused `tracing` dep (`Cargo.lock` −1 line, **zero packages
added or removed**); `conductor-verify` dev-dep gained tokio `test-util`; `.gitignore` +2 (`mutants.out/`).
Gates: nextest **669/669** · both crates green under `cargo test` (the portability criterion's own evidence) ·
clippy · doctests · scoped `cargo mutants` both crates · CLI boots, 36/36 cli tests.

## Drift resolved
7 doc-agents / 18 detectors: **5 clean · 11 proposals → 6 applied · 5 rejected · 4 escalations resolved with
the operator · 0 open.** All six amendments are test-plan (§4 cargo-mutants registration + `cargo test -p` as
a runner-portability gate · §9 scoped mutation as an operator instrument · §10 survivor-disposition rule ·
§11 Integration process-global-singleton ban · §12 decisions entry), 5 sidecar entries.
**Rejected:** all 3 obs proposals — they rest on a mis-citation (obs-plan §4's **cli** row names the `tracing`
crate + subscriber; the `#[tracing::instrument]` mandate is on the desktop-webview/ipc-internal rows), the
**second occurrence this session**, so a `playbook.md` rule was appended. Also rejected 2 test-plan §11 edits
that would have WEAKENED standing bans (replacing the golden-test clause; carving a real-time exception the
ban already grants). Record: `.andromeda/runs/2026-08-20T21-45-00-wrap/fanout-results.md`.

## Notes
- **No capability claimed, none flipped** — coverage stays **17/32 verified · 15 unclaimed**. This is a
  test-integrity chunk; every unclaimed cap needs a live Pulse leg and this chunk drives none.
- **`declares` ×6 are accepted-deliberate, not debt.** `.claude/rules/testing.md` (2026-08-10) prescribes
  reading env at the caller so the edge stays thin and both branches test with no `unsafe` env mutation.
  Killing them needs `unsafe set_var` in a shared-process module — the exact hazard this chunk removed. The
  edge was **not** restructured. Recorded in test-plan §10 + §12.
- **Curation: T1 0 · T2 1 · T3 0** (filtered 4). Added to `testing.md`: a magnitude bound alone cannot prove a
  helper READS a clock — assert magnitude + cross-helper agreement + advance across a REAL pause. Three
  rejections were dedup-against-generated-body: this wrap's own cascade had just written them.
- **Mutation tooling traps now recorded** (test-plan §4, `rules/testing.md`): `-f` resolves from the
  **workspace root**, `--test-tool=nextest` is required, and **`Found 0 mutants to test` exits 0** — a silent
  no-op indistinguishable from a clean pass.
- **Audit-ledger items for the next boundary's audit:** the `stub_pulse_mcp` mutation-scope exclusion; the
  `declares` accepted-deliberate classification with its rule citation; and that `c-mutation-*.json`'s
  `command` field omitted `--test-tool=nextest` while §B3 claimed it was recorded there (measured this chunk).
- **Audit PREREQ:** 33rd discharged in FULL form (the lock moved, so the compact basis was re-derived); the
  34th rides the new entry in PROBE-AUTO-SATISFY form with that re-derived basis stated once.
- **Live-leg housekeeping** still open: ten leg dirs under `%TEMP%/pulse-legs/`.
- **Last failed command:** none.
