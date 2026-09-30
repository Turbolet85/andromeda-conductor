
## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — breach remedied, workspace key masked
**Section:** §Input Validation (the real-model capture ingest row) · §Security Anti-Patterns → Data Protection
**Change:**
- Ingest row, the scrub:
  - Every printed line now passes `mask_workspace_key` FIRST, then `redact_value` → `mask_host_paths` → `elide_fingerprints` (was three stages).
  - Every `workspace=` line's value is masked whatever it holds, and the key is masked wherever bounded outside `[A-Za-z0-9_.-]`.
  - The key is the data dir's basename, taken from the guarded `capture_paths::pulse_logs_dir_from` path, never a raw handle read.
  - Pulse stamps the key as a PATH (`fcc31b2` `workspace_key`), so the leaf matches only when its detection falls back to the data dir.
  - A class-only witness line `pulse-report workspace rendering:` replaces the value (was "masking it … owned by the new `v3-09` series route entry").
- Ingest row, the reader: `real_model_harvest` checks each committed capture against a sha256 digest pin and grades the file after the digest matches (was "asserts the pinned literal EQUAL to its section").
- Data Protection:
  - The exception's scrub list leads with `mask_workspace_key`.
  - The 2026-09-29 BREACH now reads REMEDIED, never ratified: digest pins, no capture text in test source, the graded 2026-09-23 copy elided.
  - Stated residual: the frozen 2026-09-22 file keeps its one `fingerprint_hex` prefix.
**Why:**
- This chunk shipped both remedies the body named route-owned. It narrowed what crosses the exception and widened nothing.
- The 2026-09-30 series measured the leaf 0 times across its three captures. The source arm's inverse control read red with a report line planted and green without.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/
