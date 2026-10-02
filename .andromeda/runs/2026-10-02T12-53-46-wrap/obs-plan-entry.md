
## 2026-10-02-p-075-assert-round-against-pulse — a test-tier fingerprint-membership check; delegated timing re-graded at S
**Section:** §4 Fingerprint-storm → `verify.readback_fingerprints` · §4 Delegated-timing family
**Change:**
- Fingerprint-storm: was "no span attribute or shipped check computes it" (the emitted-vs-read-back match). Now no span attribute computes it, and one test-tier check does (2026-10-02). `lifecycle_harvest::p075_round_assertion_1_read_back_content_fidelity` grades the emitted fingerprint's membership in `fingerprint_refs` at Pulse S `03ec944`, over the P-075 round's digest-pinned capture. The match is computed in-process by the `live-pulse`-gated `p075_round_live` leg, never through a span or the envelope, so `fingerprints_read_back_count` stays a count.
- Delegated-timing family: re-graded 2026-10-02 at Pulse S `03ec944`, and all four PASS from each leaf's own field under the same rule:
  - P-025 worst 478.56ms in window, the rise anchored at 38.24ms;
  - P-027 worst 605.26ms;
  - P-037 0ms, one sample, fired with no desktop input;
  - P-045 worst 5.0ms of 163.

  Cited as measured at the chunk's `evidence/round-ledger.md` and held by `delegated_timing_harvest::tests::p075_round_assertion_{3..6}_*`.
**Why:** the P-075 assert round added the first check computing the match and re-graded the four bounds at a newer Pulse HEAD.
**Kept:** "no shipped check keys on the envelope array" stands, because the new check reads the leg capture, not the envelope. The 2026-09-29 P-025 PASS record and the `226554a` coordinate note stand beside the re-grade.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/
