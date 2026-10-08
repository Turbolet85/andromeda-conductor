## 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09 — the sixth series joins the capture-ingest row's dated record and the exception's inventory
**Section:** §Input Validation → the real-model capture ingest row · §Security Anti-Patterns → Data Protection, the ratified exception's per-series inventory
**Change:**
- The row's `workspace_key` derivation was recorded "unchanged at `a2addb3`, at `5f77859` and at `f70be92`"; now "at `5f77859`, at `f70be92` and at `9bfefb8`".
- The row's launch-fallback list (the launches whose detection fell back to the data dir) gains "the 2026-10-07 sixth series'", with Pulse's boot line reading the leaf as the workspace basename.
- The row's measured set gains the 2026-10-07 sixth series (Pulse `9bfefb8`): its leaf occurs 0 times across its three captures, its rendering witness reads `verbatim` on all three, and each `## Previously Seen` suffix prints `<redacted>`.
- The inventory was "… the 2026-10-07 series' d1, d2 and d3 captures, and the 2026-10-07 capture run's d1, d2 and d3 captures"; it now ends "…, and the 2026-10-07 sixth series' d1, d2 and d3 captures" — one report body each. The sentence on the operator-owned argv recording still names the capture run only: nothing recorded prompts in the sixth series.
**Why:** three more committed captures of the already-ratified class exist, taken through the unchanged scrub chain, so the row's dated record and the exception's inventory lagged by one series. The extension is add-only; the earlier series' records stand as written.
**Kept:** the exception's scope and terms, the scrub chain, the key derivation and every reader are unchanged; no handle reader, harness form or scrub shape was added, so nothing was ratified. The series named "2026-10-07" and "2026-10-07 sixth" are two series (Pulse `f70be92` and `9bfefb8`); the second was driven on 2026-10-08 under a section fixed on 2026-10-07.
**Ref:** .andromeda/runs/2026-10-08T07-22-11-wrap/
