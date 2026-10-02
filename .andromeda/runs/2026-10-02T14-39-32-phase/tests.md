# tests extract

## Relevance
relevant — the residuals are held by harvest-tier tests over digest-pinned capture evidence, so the fix changes default-suite test arms, a digest pin and possibly a scrub primitive's unit arms.

## Constraints
- test-plan §6 (Real-model interpretation leg) requires every committed capture to be held by a sha256 digest pin over its LF-normalized content, and graded from the file only after the digest matches. A re-elided d3 capture therefore needs its pin moved in the same change. A one-byte tamper arm must still show the pin can fail, naming the file and never the text. A source arm must still hold that no committed capture text sits in test source. §6 currently describes the d3 all-digit prefix as one "the harvest counts exactly". Retiring that clause is the wrap's job, not this extract's. Research must answer which test arm holds that exact count, and whether it is the `the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key` arm the scope names.
- test-plan §7 (fixtures; the real-model-harvest clause) treats committed capture evidence as read-only, `CARGO_MANIFEST_DIR`-anchored and pin-checked before any grade reads it. It also says "a capture is evidence, never a fixture copied elsewhere". This bears on the frozen-2026-09-22 P4 fork. Removing that file is consistent with the clause only if no harvest arm reads it. Whether one does is research's question.
- test-plan §6 (Real-model leg) records each series' graded outcome, for example 2026-10-01 d1 `Identified` and d3 `NotIdentified`. Re-eliding d3 must leave its graded verdict byte-for-byte unchanged. The plan does not say whether the d3 grade reads the `fingerprint_hex` value. That is research's question, and the P4 fork depends on the answer.
- test-plan §4 (Framework) makes `cargo test -p <crate>` a standing runner-portability gate beside nextest: the suite must be green under BOTH runners. This applies to `conductor-run` and to whichever crate holds `elide_fingerprints`, if that primitive is changed.
- test-plan §10 (Quality Gates) requires `--fail-under-lines 60` coverage, zero flakiness and no nextest `retries`. A changed elision predicate in `src/` (the context-keyed fork) needs unit arms covering both its new branch and its kept branch.
- test-plan §4 (Mutation instrument) and §9 (Scoped mutation audit) run cargo-mutants only at the epoch-boundary audit, never per chunk. It is not a gate for this chunk.
- test-plan §6 (the storm-path row near the `storm_harvest.rs` clause) and §Critical paths grade Pulse's fingerprint reaction at the harvest tier. The `triage.pattern.storm.detected` line's `fingerprint_hex` is an asserted field in `crates/conductor-run/tests/storm_harvest.rs`. Under the wider P4 reading (eliding fingerprint values in harvest test source and conductor-0.2.0 evidence), any arm comparing that value to Conductor's own derivation would lose its subject. Whether `storm_harvest.rs`, `baseline_harvest.rs`, `pii_harvest.rs` or `severity_harvest.rs` compare the value or only its presence is research's question.

## Patterns to follow
- The real-model harvest's digest-pin triad, per test-plan §6: a pin checked before grading, a one-byte tamper arm, and a no-capture-text-in-source arm. Extend it rather than adding a parallel mechanism.
- Synthetic arms in `crates/conductor-run/tests/real_model_harvest.rs` (default suite) cover the extraction, token-surface, host-path-mask and workspace-key-mask behaviour, per test-plan §6. Add a synthetic arm in the same family for the new elision behaviour (keyed all-digit value elided, unkeyed all-digit run still passing) rather than relying only on the committed file.
- Table-driven variants use rstest `#[rstest]` + `#[case]` rows, per test-plan §4 (Conventions; Fixture pattern), with one row per value class: elided, hex8, all-digit keyed, and all-digit unkeyed.
- Canonical text output is golden-locked with an exact-string `assert_eq!` at unit level, per test-plan §4 (conductor-report bullet). insta stays in CI fail-don't-write mode, per test-plan §7 (Golden artifacts).

## Anti-patterns to avoid
- Never express the fixed residual as a re-pinned exact count of a surviving value. Re-express the arm as an observable property: zero un-elided `fingerprint_hex` values across every committed capture. This follows test-plan §11 Unit (never brittle assertions against fixture-internal values; test observable behaviour).
- Never run a live Pulse drive as a CI gate, and never fake Pulse's reaction to regenerate a capture. The fix operates on the committed tree only (test-plan §11 CI; §11 Test Strategy).
- Never accept golden or pin changes interactively (`cargo insta review`), and never lower the coverage threshold or add retries to get green (test-plan §11 Universal; §11 Quality).

## Contract bindings
- tests ↔ security: the elision definition (`elide_fingerprints`, "every fingerprint-shaped token") is security-plan §Security Anti-Patterns → Data Protection's. The harvest arms are its test-tier enforcement, so a change to the definition and the arm asserting it must move together.
- tests ↔ arch: the arch §Established Decisions [Read-Back Dependency Posture] passage cites the 2026-09-10 live envelopes, which test-plan §Critical paths and §6 name as the basis for the fingerprint-reaction claims graded at the harvest tier. The wider P4 reading touches evidence those grades rest on.

## Acceptance criteria contributions
- `cargo nextest run -p conductor-run --profile ci` is green. The arm that today holds the residuals exactly asserts zero un-elided `fingerprint_hex` values over every committed `rm-capture*.txt`, and the 2026-10-01 d3 graded verdict is unchanged (per test-plan §6 Real-model interpretation leg).
- The re-elided d3 capture's sha256 digest pin is updated in the same change. Its tamper arm still fails on a one-byte change, naming the file and not the text, and the source arm still holds that no capture text sits in test source (per test-plan §6; §7 real-model-harvest clause).
- `cargo test -p conductor-run`, plus `cargo test -p` for the crate holding `elide_fingerprints` if it changed, is green under the shared-process runner as well as nextest (per test-plan §4 Framework, runner-portability gate).
- If the context-keyed fork is chosen: a synthetic arm proves that a `fingerprint_hex`-keyed all-digit value renders `<fingerprint>` while an unkeyed all-digit run still passes unelided (per test-plan §6 synthetic arms; §11 Unit, public-seam behaviour).
