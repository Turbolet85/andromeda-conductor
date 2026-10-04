# Inverse controls — the five Epoch 5 survivors, each killed by its new test

Run ONCE at /implement (2026-10-04, Linux dev host, tree = base `dab66dc` + this chunk's edits), from the project root, in
the exact IC1–IC5 forms of `plan.md` §Test Commands. Each form copies its source file into the gitignored
`target/inverse-control/`, installs a `trap … EXIT` restore, applies ONE mutation with `sed -i`, asserts the mutated text
occurs exactly once (`grep -cF … = 1`), runs ONLY the test that must kill it, and ends with `test $? -eq 100` — so a
control exiting 0 means nextest itself exited **100** (tests failed), never 101 (a build failure, which proves nothing).

Restoration was verified BYTE-IDENTICAL after every control: sha256 of both source files was read before the first
control and after each one, and every reading matched (`identity.rs` `ae7d2940…52f5`, `canary.rs` `279d0b8d…1070`).
The listed non-mutating guards re-prove it on every re-run: the restore guard (`last line 3`) and the two library
suites.

The mutation descriptions are cargo-mutants' own, as `liveness-before.md` records them in `missed.txt` at the base.

| control | mutation (cargo-mutants description) | hand edit (`sed -i`) | killing test | control exit | nextest exit |
|---|---|---|---|---|---|
| IC1 | `replace ^= with \|= in xor_in_place` | `*byte ^= m;` → `*byte \|= m;` (`identity.rs:49`) | `identity::tests::rekey_under_one_salt_twice_restores_every_id` | 0 | 100 |
| IC2 | `replace ^= with &= in xor_in_place` | `*byte ^= m;` → `*byte &= m;` (`identity.rs:49`) | `identity::tests::rekey_under_one_salt_twice_restores_every_id` | 0 | 100 |
| IC3 | `replace > with >= in emit_canary_storms` | `if n > 0 {` → `if n >= 0 {` (`canary.rs:203`) | `canary::tests::the_first_storm_leaves_at_the_call_with_no_gap_before_it` | 0 | 100 |
| IC4 | `replace * with + in emit_canary_storms` | `n * CANARY_STORM_COUNT` → `n + CANARY_STORM_COUNT` (`canary.rs:206`) | `canary::tests::every_real_model_canary_occurrence_carries_its_own_span_identity` | 0 | 100 |
| IC5 | `replace * with / in emit_canary_storms` | `n * CANARY_STORM_COUNT` → `n / CANARY_STORM_COUNT` (`canary.rs:206`) | `canary::tests::every_real_model_canary_occurrence_carries_its_own_span_identity` | 0 | 100 |

## The failing lines, as nextest printed them

IC1 — `1 test run: 0 passed, 1 failed, 137 skipped`

```
FAIL [   0.004s] (1/1) conductor-emit identity::tests::rekey_under_one_salt_twice_restores_every_id
thread 'identity::tests::rekey_under_one_salt_twice_restores_every_id' (1153557) panicked at crates/conductor-emit/src/identity.rs:164:9:
```

The `assert_eq!` on the whole request failed: after the second re-key under `0xC0FFEE` the root `trace_id` read
`[191, 199, 255, 55, …]` where the original is `[187, 67, 215, 35, …]` — under `|=` the second pass leaves `x|m`.

IC2 — `1 test run: 0 passed, 1 failed, 137 skipped`

```
FAIL [   0.004s] (1/1) conductor-emit identity::tests::rekey_under_one_salt_twice_restores_every_id
thread 'identity::tests::rekey_under_one_salt_twice_restores_every_id' (1154088) panicked at crates/conductor-emit/src/identity.rs:164:9:
```

Same assertion: the root `trace_id` read `[49, 65, 214, 3, …]` against the original `[187, 67, 215, 35, …]` — under
`&=` the second pass leaves `x&m`.

IC3 — `1 test run: 0 passed, 1 failed, 51 skipped`

```
FAIL [   0.025s] (1/1) conductor-run canary::tests::the_first_storm_leaves_at_the_call_with_no_gap_before_it
thread 'canary::tests::the_first_storm_leaves_at_the_call_with_no_gap_before_it' (1154619) panicked at crates/conductor-run/src/canary.rs:511:9:
storm 0 arrived 90s after the call; the gap belongs between storms only
```

IC4 — `1 test run: 0 passed, 1 failed, 51 skipped`

```
FAIL [   0.024s] (1/1) conductor-run canary::tests::every_real_model_canary_occurrence_carries_its_own_span_identity
thread 'canary::tests::every_real_model_canary_occurrence_carries_its_own_span_identity' (1155043) panicked at crates/conductor-run/src/canary.rs:523:9:
assertion `left == right` failed: one span identity per canary occurrence
  left: 14
 right: 36
```

IC5 — `1 test run: 0 passed, 1 failed, 51 skipped`

```
FAIL [   0.026s] (1/1) conductor-run canary::tests::every_real_model_canary_occurrence_carries_its_own_span_identity
thread 'canary::tests::every_real_model_canary_occurrence_carries_its_own_span_identity' (1155682) panicked at crates/conductor-run/src/canary.rs:523:9:
assertion `left == right` failed: one span identity per canary occurrence
  left: 12
 right: 36
```

## The readings against the P3 mechanism

Every printed value is the one the P3 re-derivation predicted (`scope.md` §Causal claims carried): storm 0 arrives
90 s after the call under `>=` (the 90 s `REAL_MODEL_CANARY_STORM_GAP` slept before storm 0); `*`→`+` yields seeds
19..=32 = **14** distinct identities; `*`→`/` yields 7..=18 three times = **12**; the correct code yields 7..=42 = 36.

The mutation instrument itself was NOT re-run (test-plan §9: epoch-boundary audit only); the next audit confirms these
kills under cargo-mutants.
