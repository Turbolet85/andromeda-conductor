**Directory structure (crate-per-seam Cargo workspace):**

```
conductor/
├─ Cargo.toml                 # workspace manifest (members + shared deps)
├─ Cargo.lock
├─ rust-toolchain.toml        # pin Rust 2024 / MSRV 1.94.1
├─ rustfmt.toml               # workspace fmt config — one key: edition = "2024"
├─ .gitattributes             # repo-wide LF pin (* text=auto eol=lf) + the named coverage-matrix.md control
├─ LICENSE-MIT · LICENSE-APACHE # the dual license — every Cargo manifest declares `MIT OR Apache-2.0`
├─ scripts/
│  ├─ agent-run.sh            # headless source-of-truth entrypoint
│  ├─ mutation-gate.py        # operator-local mutation-tally gate (`<unit>` | `selftest`); invoked by no CI step, wired into neither harness shell, no 6th command
│  ├─ mutation-roster.toml    # the gate's expected `missed` / `timeout` multisets per unit
│  └─ fixtures/mutation-gate/ # the committed `selftest` fixture tree (roster.toml · roster-bad.toml · arms.toml · arms/{name}/)
├─ crates/
│  ├─ conductor-core/         # runtime-agnostic engine library
│  ├─ conductor-timeline/     # deterministic seeded phase scheduler (tokio::time)
│  ├─ conductor-emit/         # OTLP raw-type emission primitives + exception events / fingerprint primitive (opentelemetry-proto + tonic)
│  ├─ conductor-faults/       # fault helpers (ramps, silence, port-occupier, fingerprint-storm)
│  ├─ conductor-verify/       # MCP read-back client (hand-rolled JSON-RPC), preflight gate, verdict logic
│  ├─ conductor-report/       # JSONL journal + Markdown report + runs.db (rusqlite) storage seam
│  ├─ conductor-run/          # run composition root lib (preflight + execute_scenario + persist + the suite driver drive_run (progress callback + Ctrl-C stop flag) + the fault-phase occupier guard; → conductor-faults) — used by the conductor-cli bin
│  └─ conductor-cli/          # `agent-run` bin (#[tokio::main(flavor="current_thread")] + anyhow)
├─ scenarios/                 # declarative scenario config (serde + garde), each naming its P-IDs — a P-ID may be named by several
├─ contracts/                 # pinned MCP contract manifest + the SUT capability manifest + the SUT load envelope + the SUT run contract + the scenario-assertion audit ledger + the P-025 measurement contract + the real-model leg posture (the two members no shipped code reads)
├─ runs/                      # per-run artifacts: the <run_id>.jsonl journal + <run_id>.md report are run_id-stemmed, never overwritten
│  └─ live-suite/             # `run --live` per-leg self-obs captures + the real-model arm's rm-capture.{txt,err} — leg-stemmed, each set cleared per invocation of its own arm (the one exception to the line above)
│  └─ span-landing/           # the span-landing operator pass's span-{a,b}.jsonl — the two named files cleared non-recursively before drive A; read by span_landing_live, a stale pair refused
│  └─ runs.db                 # embedded SQLite cross-run index
├─ coverage-matrix.md         # every manifest capability classified (definition of done)
└─ .github/workflows/         # GitHub Actions: fmt + build + test + clippy + the static gates (over committed data · repository hygiene)
```
