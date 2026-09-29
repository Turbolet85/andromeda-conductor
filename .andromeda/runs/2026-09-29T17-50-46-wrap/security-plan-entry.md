
## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the capture ingest row's new line classes and scrub, and the exception's recorded breach
**Section:** §Input Validation → Real-model capture ingest row · §Security Anti-Patterns → Data Protection (the `corpus.db` ban's exception)
**Change:**
- Ingest row, what the capture writes and prints:
  - One `evidence/rm-capture-{drive}.txt` per series drive.
  - The MCP re-read prints the report body verbatim from its first `## ` line.
  - The id sweep reads at most 64 ids, from the highest id seen minus 63, floor 1 (was "an id sweep to 64").
  - It also prints Pulse's `creating digest corpus retrieval rows:` line, the `canary:` lines and `scenario_storm=` on storm lines.
- Ingest row, the scrub:
  - Every printed line passes `redact_value`, then `mask_host_paths`, then `elide_fingerprints` (was: only the envelope fingerprints elided, to counts).
  - The capture itself never prints the workspace basename, but the verbatim report body shows Pulse's rendering of the workspace key (measured `[redacted: credit_card]`). Masking it before the next series is owned by the new `v3-09` series route entry.
- Data Protection:
  - The exception's scrub is now `redact_value` + `mask_host_paths` + `elide_fingerprints`, over every fingerprint-shaped token (was "envelope fingerprints elided").
  - The 2026-09-29 b2 capture carries a report body.
- A BREACH is recorded, never ratified:
  - The series pins in `crates/conductor-run/tests/real_model_series/mod.rs` put report text in test source, outside the `evidence/` tree.
  - The 2026-09-22 capture and `PINNED_CAPTURE` still carry one un-elided `fingerprint_hex` prefix.
  - Both remedies (a sha256 digest pin per evidence file; that prefix elided) are owned by the new `v3-09` series entry, and land before the next series.
**Why:**
- The chunk widened what the capture prints (the report body, for P-031/P-034/P-044) inside the ratified exception, and fixed a scrub gap.
- Escalated at this wrap. Ratifying the test-source copies would widen the exception, and a widening waits for the founder's live word (his 2026-09-27 ruling), so the overseer recorded a breach with a route-owned remedy.
- The workspace-key mask keeps "never printed" true without widening.
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/
