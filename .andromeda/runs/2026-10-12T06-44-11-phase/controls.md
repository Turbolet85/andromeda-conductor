# P5 check 4 (9) controls — 2026-10-12-panel-shaped-types-retired

Fired AS THE ENTRY through `gate.py run --plan … --entry {n} --project-root {scratch clone}` (no `--run-dir`), over a
`git clone --shared` of the project at HEAD `06190485ec74ac07b1c797fbde6dc9441e60db2f`, in the session scratchpad.
Each entry fired unmodified first (it read what it read on the real tree), then with the control input. The inputs
lived in the clone only; the script is the session scratchpad's `controls.sh`.

| entry | unmodified (clone) | control input | control reading |
|---|---|---|---|
| 2 lock no-add | green · last line 0 | a `[[package]]` `zz-control-added 0.0.1` appended to `Cargo.lock` | red · last line 1 (must fail: held) |
| 3 lock count | green · 268 | the `zerocopy` package block removed | red · 267 (must fail: held) |
| 5 symbol census | red · 4883 B | the four fixture files deleted and every line carrying a census token removed from the six other hit files | green · exit 1, no output (must pass: held) |
| 6 sink census | red · `crates/conductor-core/src/obs.rs` | `obs.rs`'s lines naming `conductor-tauri.jsonl` removed | green · exit 1, no output (must pass: held) |
| 7 window prose census | red · 6217 B | every case-insensitive `tauri/webview/picker/gui/dialog row/desktop` line removed from the ten files | green · exit 1, no output (must pass: held) |
| 8 verb declared | red · 0 | a unit variant `    Scenarios,` added at the top of `Commands` | green · last line 1 (must pass: held) |
| 10 masters' no-interface line (added at review) | green · 3 | `.andromeda/a11y-plan.md`'s `**Conductor has no interface.**` reworded | red · 2 (must fail: held) |
| 9 signal feature | red · 0 | `crates/conductor-cli/Cargo.toml` tokio features `["macros", "rt", "signal"]` | green · last line 1 (must pass: held) |

Entry 13's literals: `error-baseline-spike` and `P-009` are TOML strings (`scenarios/error-baseline-spike.toml:18-19`)
parsed into `String` / `PId(pub String)` (`crates/conductor-core/src/scenario.rs:27`) with no transform; the CLI
already prints `P-009` verbatim in its own `coverage` output (`cargo run -q -p conductor-cli --locked -- coverage`,
exit 0, 1 line carrying `P-009`, measured 2026-10-12). The roll-up `scenarios` token is the line step 7 writes;
implement's first green firing after the verb lands is its must-pass reading.

Entry 14's literal: clap derives a subcommand's name from its variant in lowercase — measured on this binary's own
`--help` (baseline 14.log): `Preconditions` prints as `preconditions`, `Cleanup` as `cleanup`.
