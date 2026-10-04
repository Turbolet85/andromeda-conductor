# Liveness before the kills — the five survivors measured LIVE at P3

Copied at /implement from the P3 run's tallies under the gitignored `target/phase-mutants/` (read from `missed.txt` /
`caught.txt` / `timeout.txt` / `unviable.txt` and `outcomes.json`; never from the exit). HEAD `dab66dc`, cargo-mutants
27.1.0 (`outcomes.json` `cargo_mutants_version`), Linux dev host, run 2026-10-04T12:56Z → 12:57Z (`started.txt`).
Ran on the overseer's take-up directive; test-plan §9 keeps the instrument at the epoch-boundary audit, so this file is
the before-reading only — no after-run exists in this chunk (the kills are proven by inverse controls,
`inverse-controls.md`).

## conductor-emit — `xor_in_place`

Command: `cargo mutants -p conductor-emit -f crates/conductor-emit/src/identity.rs --re 'xor_in_place' --test-tool=nextest --jobs 2 --output target/phase-mutants/emit`

Tallies: total 3 · missed 2 · caught 1 · timeout 0 · unviable 0 (`outcomes.json` equal; conserves 3). Exit 2 (carries
no verdict; read from the tallies).

`missed.txt`:

```
crates/conductor-emit/src/identity.rs:49:15: replace ^= with &= in xor_in_place
crates/conductor-emit/src/identity.rs:49:15: replace ^= with |= in xor_in_place
```

`caught.txt`:

```
crates/conductor-emit/src/identity.rs:48:5: replace xor_in_place with ()
```

## conductor-run — `emit_canary_storms`

Command: `cargo mutants -p conductor-run -f crates/conductor-run/src/canary.rs --re 'emit_canary_storms' --test-tool=nextest --jobs 2 --output target/phase-mutants/run`

Tallies: total 6 · missed 3 · caught 3 · timeout 0 · unviable 0 (`outcomes.json` equal; conserves 6). Exit 2 (carries
no verdict; read from the tallies).

`missed.txt`:

```
crates/conductor-run/src/canary.rs:203:14: replace > with >= in emit_canary_storms
crates/conductor-run/src/canary.rs:206:61: replace * with + in emit_canary_storms
crates/conductor-run/src/canary.rs:206:61: replace * with / in emit_canary_storms
```

`caught.txt`:

```
crates/conductor-run/src/canary.rs:202:5: replace emit_canary_storms -> anyhow::Result<()> with Ok(())
crates/conductor-run/src/canary.rs:203:14: replace > with < in emit_canary_storms
crates/conductor-run/src/canary.rs:203:14: replace > with == in emit_canary_storms
```
