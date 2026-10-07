
## 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09 — the capture row and the exception's inventory gain the 2026-10-07 series
**Section:** §Input Validation, the real-model capture ingest row · §Security Anti-Patterns → Data Protection, the capture exception's per-series inventory
**Change:**
- The workspace-key derivation's provenance was "at `fcc31b2`, unchanged at `a2addb3` and at `5f77859`"; it now ends "at `5f77859` and at `f70be92`".
- The series whose launch fell back to the data dir were "the 2026-09-30, 2026-10-01 and 2026-10-06 series"; the 2026-10-07 series joins them.
- The leaf-rendering measurement names the same four series, whose leaves occur 0 times across their three captures each; the home-rooted clause (each `## Previously Seen` suffix a POSIX path printing `<redacted>`) now names the 2026-10-06 series, the first on the Linux dev host, and the 2026-10-07 series.
- The exception's inventory ended at the 2026-10-06 series; it now adds that the 2026-10-07 series' d1, d2 and d3 captures carry one report body each.
**Why:** the 2026-10-07 series committed three more captures of the ratified class, through the unchanged chain: the workspace key (`rm-fifth-series`) occurs 0 times in them, the host-path probe over the chunk's evidence reads 0, and the key rendering reads `verbatim` on all three drives. Pulse's derivation file is unchanged at `f70be92`. The exception's scope and terms did not move, and no boundary widened.
**Kept:** no handle inventory moved. Architecture registered two Pulse model handles at this wrap, and Conductor reads neither, so §Input Validation gains no row for them.
**Ref:** .andromeda/runs/2026-10-07T09-46-39-wrap/
