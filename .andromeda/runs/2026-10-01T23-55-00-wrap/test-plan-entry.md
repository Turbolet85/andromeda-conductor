
## 2026-10-01-per-run-span-identity-in-the-real-model-harness — per-execution span identity and its two-tier proof
**Section:** §2 Test Strategy (operator-local real-wall-clock list) · §4 Unit Test Strategy → conductor-emit · §6 E2E → Real-model interpretation leg · §7 Golden artifacts row · §8 What to mock → Random sources · §9 Live-Pulse scenarios (the `live-pulse` gated SET) · §11 E2E (the SUT-side quiet-window class)
**Change:**
- §6: the 2026-10-01 d2 replay is repaid. The production path re-keys every trace export's span identity under `execute_scenario`'s `std::time` salt (`Dispatcher::connect(…, Some(_))` → `conductor_emit::rekey_trace_identity`), and content stays a pure function of seed. Two tests hold it:
  - `dispatch_wire`'s `two_same_seed_drives_inside_one_window_share_no_span_identity`, which goes RED with the re-key removed;
  - live, the `span_landing_live` witness: 253 507 ms apart inside Pulse's 600 s retention, zero refused appends.
- §2 and §9: the operator-gated `span_landing_live` witness joins the operator-local real-wall-clock list and the `live-pulse` gated target SET. It is auto-discovered, carries its own clippy line, mints no harness verb, and is never a CI leg.
- §11: the witness pass's 180 s window between its two same-seed drives joins the SUT-side quiet-window class.
- §4: the conductor-emit unit tier names `rekey_trace_identity`: a pure per-salt bijection that preserves linkage, keeps an empty id empty and moves no content byte.
- §7: the `dispatch_wire__*` goldens drive the UNSALTED dispatcher, the seed-pure identity tier; a salted test passes a fixed salt.
- §8: the identity salt is injected (`identity_salt: Option<u64>`: `None` / a fixed `Some(N)` in tests, `emitted_ms` in production). The primitive sits at the unit tier and the two-drive property at the dispatcher integration tier.
**Why:** the chunk ships per-execution span identity with its CI and live proofs. These are the plan's three expected test-plan amendments, all carried.
**Kept:** `:328` "same-seed re-run yields identical stream shape" (shape stays seed-pure) · §12 `:616` (Decisions Log history).
**Ref:** .andromeda/runs/2026-10-01T23-55-00-wrap/
