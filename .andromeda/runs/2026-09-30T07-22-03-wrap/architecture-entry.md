
## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — sha2 test-only, breach remedied, second series
**Section:** §Stack and Technologies (Hashing / digest) · §Established Decisions · §Standard Contracts (Readiness gate, corpus access) · §Occupied Resources (the posture-contract row) · §Infrastructure Patterns (Build system)
**Change:**
- Hashing row: blake3 stays the only SHIPPED hashing dependency; `sha2 0.10` is added as a TEST-only `conductor-run` dev-dependency for the real-model harvest's sha256 digest pins. It was already locked via `tauri-codegen` / `wry`, so it adds no package.
- Build system: the dev-test stack list gains `sha2` beside `regex`.
- Corpus access: the 2026-09-29 breach now reads REMEDIED on 2026-09-30, with the frozen 2026-09-22 file's residual (was "and its route owner").
- Posture-contract row: the contract now names both drive series (2026-09-29, then 2026-09-30), each add-only and fixed before its first drive. The 2026-09-30 series is re-pinned to Pulse `fcc31b2`.
- Established Decisions: the dev-host clause now reads "0 failing, the expected-skip SET skipped" (was "12 passing / 0 failing / 2 skipped"). The routine arm gained its stall spec, and the run-anchored records at the same site are kept.
**Why:**
- Each line brings the body to what the chunk shipped. The registry-size check read OVER after the first apply (Established Decisions +76 B, Occupied Resources +15 B).
- Two clauses were trimmed to within target (38 111 B and 38 028 B against 38 115 B); the dropped detail lives here:
  - the 2026-09-30 section's digest `0091fe6f…` was recorded in its chunk's attempt ledger before `d1` and is held by `the_2026_09_30_series_rule_was_fixed_before_d1`;
  - the dev-host reading was taken at a coherent 154.0.4258.37 driver/runtime pair.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/
