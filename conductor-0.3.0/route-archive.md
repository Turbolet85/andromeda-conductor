# Route archive — conductor-0.3.0

<!-- Writer: /andromeda-wrap-session P7 flip-compaction ONLY. Read by NO loop skill.
Cold history: each line is the VERBATIM working-route entry as it stood when its master
record went complete, archived before its annotation freight was stripped. Superseded at
take-up by the chunk scope fold and at flip by the master desc — never cite it for current
truth. NOT a complete pin chronicle: tail annotations cleared before an entry froze live
only in git and the P5 summaries. -->

## 2026-09-11-hosted-runner-endpoint-cause-probed — archived at the 2026-09-11 wrap
[2026-09-11-hosted-runner-endpoint-cause-probed] Hosted-runner endpoint cause probed — policy state, module versions, session identity; diagnose-only, the reading recorded whatever it says, host paths scrubbed

## 2026-09-11-hosted-runner-endpoint-cause-closed — archived at the 2026-09-12 wrap
[2026-09-11-hosted-runner-endpoint-cause-closed] Hosted-runner endpoint cause closed — the unread module-version probe placed where the app is alive, and the elevation difference varied  PREREQ: close rust gate deferral (deferred since 2026-09-11-hosted-runner-endpoint-cause-probed)

## 2026-09-12-ledger-gate-id-space-generalised — archived at the 2026-09-12 wrap
[2026-09-12-ledger-gate-id-space-generalised] Ledger gate id-space generalised — `requirement_ids` filters `starts_with("v2-")` while the same file's directory resolution was deliberately generalised, so the gate goes vacuous-then-red at every version transition; it is red now on `conductor-0.3.0`'s `v3-` ids (906 run, 905 passed, 1 failed) and red in CI on `c93a379` and `eecc7f4`. Surface measured: seven `v2-` literals at `:219 :221 :228 :250 :256-259` plus the id-space assertion at `:208` in `crates/conductor-report/tests/matrix_ledger_gate.rs`  BLOCKING: the version's workspace gate is red until this lands — nothing is scheduled ahead of it (operator directive, 2026-09-12 wrap)
