# Session Handoff

**Last Updated:** 2026-08-13T23:05:49Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **11 ahead** after this chunk commit)
**Status:** clean
**Last Commit:** 2026-08-13-first-live-green-preflight — the live leg ran, and the mechanism meant to make it green is disproved

## Position
- Done: **2026-08-13-first-live-green-preflight** — the `boot` budget now derives from the run contract in
  BOTH shells (was 30s in `.sh`, absent in `.ps1`, against a ~135s run), a shape witness logs each read-back
  tool's observed key set, and the three-arm live probe was re-run and recorded.
- Next: **Canary fingerprint-feed capture** — `/andromeda-phase` to promote + plan. Newly inserted at the end
  of Epoch 2 by operator decision; it is the one open Conductor-side piece and needs no incident.

## Work done
5 files (2 harness scripts + `extract.rs` + `preflight.rs` + `tests/readback.rs`), 0 new files, `Cargo.lock`
**zero lines**. Gates green in **1 iteration**: workspace `--profile ci` **577/577** (+3) zero retries,
doctest 7 suites, `clippy -D warnings` clean, `cargo deny` all four classes ok. Smoke ✓ on BOTH shells —
`boot` exit 1 in 4s/1s with stdout identical modulo `checked_at`, closing a `.sh`/`.ps1` parity gap.
**Live leg (operator-gated, 3 arms, 135s each):** every arm `ready:false` on the workspace-key precondition
for its *no-incident* cause; protocol negotiated, all four tools present, `data_dir` redacted, zero panics.

## Drift resolved
7 detectors → **5 amendments on 4 masters**, **0 escalations**, 0 open. arch §Occupied Resources (warm-up
claim measured false) · test-plan §3 `boot` Timeout (two derived budgets) · security-plan §Auth model +
§Input Validation (MCP_ENABLED locus; shell-side contract readers) · obs-plan §6 (boundary log gains the key
set). The obs one was **not proposed by its detector** — main raised it under the expected-amendments floor.
Cascade re-derived 2 leaf bodies; the citation grep hit exactly 1 preserve-verbatim curation home and routed
it to P3. Full record: `.andromeda/runs/2026-08-13T22-48-53-wrap/fanout-results.md`.

## Notes
- **The headline result is a falsification, not a green.** `[incident_formation]`'s warm-up cannot work:
  Pulse gates cue evaluation on `BootstrapState::Ready`, needing **3,600s per service, wall-clock**
  (`activity_floor.rs:33`, `cue/evaluate.rs:164`), and `baseline_state` persists 0 rows so every restart
  resets the anchor. `warmup_ms = 45000` is short by **80×** and unfixable by its own knobs — and the term is
  declared `check = "asserted"`, so the gate meant to guard it passes it unconditionally.
- **F10 is still unconfirmed**, for the second attempt. No incident forms under any cwd, so the workspace-key
  axis stayed unexercised. The three arms did establish cwd-invariance experimentally, and `incidents` holds
  **0 rows in total** — proving the empty read-back is genuine emptiness, not key filtering.
- **The `\\?\` question is answered from SOURCE, not measured** (labelled as such in the evidence): the app
  keys on a canonicalized path (`detect.rs:31` → `digest_runtime.rs:114`), the sidecar on the raw string, so
  they cannot be byte-equal on Windows from any launch position.
- **Key-diff: one third retired.** `query_incident_list` matched the committed baseline exactly.
  `retrieve_report` + `retrieve_telemetry_slice` were never reached (both need a non-empty corpus) — carried
  onto `fingerprint-storm live proof`, the first corpus-bearing live proof.
- **The operator recipe is FIVE items, not four** — `ANDROMEDA_PULSE_MCP_ENABLED=true` must be in Conductor's
  own environment (inherited by the sidecar; `spawn.rs` passes only the data dir). The first arm-1 attempt ran
  without it and was discarded. Pinned on the next entry, with the `spawn.rs` design question left undecided.
- **Fix-scope, kept separable:** 3 Pulse-side pieces (bootstrap reachability · workspace-key alignment ·
  nothing else assumed) + 1 open Conductor-side question (the fingerprint feed), which the new route entry
  now owns. Piece 2 is invisible until piece 1 lands.
- **`cargo audit` — FIFTEENTH red, silent re-pin** under the L5 ratification (origin
  `2026-08-08-sut-capability-manifest`). Byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` on 0.22.2,
  true exit 1 — advisory-DATABASE fault. Basis re-verified literally: `Cargo.lock` un-drifted at **zero
  lines**, `cargo deny` green as the overlap.
- **Curation:** T1 0 · **T2 2 in-place extensions** (verification-harness — the warm-up prescription it
  carried is falsified; testing.md — third occurrence of measure-the-mechanism-before-you-write, sharpened by
  a `check = "asserted"` term that cannot catch its own falsity) · T3 0. Filtered 3. No conflicts, no deferrals.
- **Verification matrix:** `v2-10` **declined, stays pooled** (`chunk:null`) — no arm reached `ready:true`;
  its `notes` gained the evidence pointer. Coverage **11/32**, unchanged by design.
- **Last failed command:** none.
