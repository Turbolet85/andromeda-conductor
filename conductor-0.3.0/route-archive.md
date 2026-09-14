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

## 2026-09-13-audit-debt-retired-before-epoch-1-closes — archived at the 2026-09-13 wrap
[2026-09-13-audit-debt-retired-before-epoch-1-closes] Audit debt retired before Epoch 1 closes — seventeen surviving mutants killed or ratified by class, the emission-test fixture family shared, the envelope keys single-sourced  EVIDENCE: every coordinate below is from the 6b audit run dir `.andromeda/runs/2026-09-13T11-34-19-code-audit/` and was re-verified against the artifacts at this wrap. Mutants: 25 survivors across the five `c-mutation-{unit}.json` `survivors[]` (cli 3 · core 0 · run 5 · tauri 3 · verify 14); 8 of verify's 14 sit in `crates/conductor-verify/src/bin/stub_pulse_mcp.rs` and are NOT this entry's — applying the standing stub exclusion belongs to the next audit — so the seventeen in the title is 25 − 8. Fixtures: `c-duplication.json` `top[0..4]` is five `conductor-emit/tests/` pairs at 44 · 42 · 39 · 38 · 36 lines (`egress.rs` in four of them), carried unchanged since the Epoch 3 baseline. Envelope: `conductor-report/src/journal.rs:158-181` ↔ `conductor-verify/src/record.rs:137-160`, 24 lines, the SAME eleven-key array asserted twice inside inline `#[cfg(test)]` modules — the only new entrant in the top-10 clone list. Hygiene, freight not title: `c-dead.json` `unused_deps` is `conductor-emit → conductor-core`, standing since the baseline; grepping the crate name under `conductor-emit/src` returns THREE hits, of which only `error.rs:5` is a resolvable intra-doc link (`[`conductor_core::CoreError`]`) — `latency.rs:8` and `topology.rs:21` name the crate in prose backticks and resolve to nothing, so a take-up grep returning 3 confirms this rather than contradicting it (hypothesis: dropping the dep breaks the first and neither of the other two). The next audit measures the outcome: survivors 25 → ≤ 8, the top-10 clone list loses the five emit pairs, `unused_deps` empty. Out of scope by design, being the next audit's confirm: the first mutation measurement of emit / report / timeline / faults, and core's shard rotation (operator directive, 2026-09-13 wrap)

## 2026-09-13-p-025-measurement-contract-for-pulse — archived at the 2026-09-14 wrap
[2026-09-13-p-025-measurement-contract-for-pulse] P-025 measurement contract for Pulse — which Pulse-emitted observable, at what resolution, over what window, and what constitutes a hard grade
