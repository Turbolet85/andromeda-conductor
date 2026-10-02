
## 2026-10-01-per-run-span-identity-in-the-real-model-harness — the span-landing witness as a guarded test-binary reader
**Section:** §Input Validation (path-handle row · new Span-landing witness ingest row · `ANDROMEDA_PULSE_DATA_DIR` spawn row · declaration-only read-set row) · §Security Anti-Patterns → Input · the phase-scoping paragraph (`:222`)
**Change:**
- `CONDUCTOR_RUNS_DIR`'s test-binary readers go from THREE to FOUR, adding the operator-gated `conductor-run/tests/span_landing_live.rs` (2026-10-01). It resolves the handle through `capture_paths::runs_dir_from` → `resolve_under` from birth. The same enumeration is restated at §Security Anti-Patterns → Input ("two … captures" / "all three" → three / "all four") and at the phase-scoping paragraph ("Those three" → "Those four").
- A new §Input Validation row, **Span-landing witness ingest**, records untrusted SUT output: every `agent-latest.jsonl*` under `ANDROMEDA_PULSE_DATA_DIR`'s `logs/`, plus the frozen `span-{a,b}.jsonl` journals.
  - It reads both handles through the shared `capture_paths` guards: canonicalize + is-dir, and `resolve_under`. Failures are path-free (`e.kind()` only).
  - It does one bounded `serde_json` decode per line.
  - It prints exactly ONE integers-only `span-landing: PASS|FAIL …` line and commits nothing from the ingest, so no scrub chain is owed.
- The data-dir handle's test-binary VALUE reader becomes plural (the real-model capture and the witness) in the spawn row and the read-set row. The witness spawns no sidecar, so neither reader relaxes or satisfies the spawn duty.
**Why:** the duty binds per READER and each reader is recorded, never assumed. The validation was present from birth, so this is a record of a validated boundary, not a widening. Playbook `:124` was considered and its precondition failed: no new input class (the same handle, the same guard module, the same Pulse-log class `real_model_live` already ingests), no new crossing, no write. It applied as routine under `:308`.
**Ref:** .andromeda/runs/2026-10-01T23-55-00-wrap/
