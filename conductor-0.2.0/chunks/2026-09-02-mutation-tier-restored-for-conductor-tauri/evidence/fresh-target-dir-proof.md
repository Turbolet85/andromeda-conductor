# Fresh-target-dir proof — the binding, measured cold

Chunk acceptance, second half: *"a runner-portability gate proves the test on a FRESH target dir."*
Recorded once as chunk evidence (the repeatable half is the standing gate below).

## The cold run

```
CARGO_TARGET_DIR=<a fresh empty dir outside the repo> \
  cargo nextest run -p conductor-tauri -p conductor-cli
```

| fact | value |
|---|---|
| exit | **0** |
| tests | **49 run, 49 passed, 0 skipped** (12 `conductor-tauri` + 37 `conductor-cli`) |
| the relocated test | `conductor-cli::cross_surface_parity path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db` — **PASS**, 0.579 s (49/49) |
| completed | 2026-09-03T05:10:57Z |

**The run was genuinely cold, verified rather than assumed** (exit 0 alone would not have shown it):

- **368 `Compiling` lines**, including `Compiling tauri v2.11.3`, `Compiling conductor-tauri v0.1.0`
  and `Compiling conductor-cli v0.1.0` — the whole dependency tree built from nothing.
- The target dir grew to **3.0 GB** from empty.
- `debug/conductor.exe` (15 190 528 B) was **produced inside that fresh dir** by the build itself —
  the declared edge, not a pre-existing artifact.

This additionally answers the open question the architecture extract raised at planning time: the
`ui/dist`-before-cargo build order (`tauri::generate_context!` resolves `build.frontendDist` at
compile time, and `ui/dist` is gitignored) **holds on a cold compile** — `conductor-tauri` compiled
from scratch here with no intervention.

## The standing gate (repeatable, seconds)

The cold run is evidence, not a gate — it is too expensive to re-run per chunk. What guards the
binding from here on:

| gate | result |
|---|---|
| `cargo nextest run -p conductor-tauri` | 12/12 pass |
| `cargo test -p conductor-tauri` | 12/12 pass |
| `cargo nextest run -p conductor-cli` | 37/37 pass |
| `cargo test -p conductor-cli` | 23 + 13 + 1 pass (the `1` is this test's own binary) |
| **artifact-absent probe** — `target/debug/conductor.exe` moved aside, both packages re-run | **tauri=0, cli=0** |

The artifact-absent probe is the one that discriminates the defect, and it is the direct
before/after measurement:

| | `-p conductor-tauri`, binary absent |
|---|---|
| **before** (research, 2026-09-02) | **exit 100** — `CARGO_BIN_EXE_conductor is unset` at `assert_cmd-2.2.2/src/cargo.rs:232`, 12 passed / 1 failed |
| **after** (this chunk) | **exit 0** — 12 passed, 0 failed |

`conductor-cli` passes the same probe because Cargo rebuilds the binary for that package's own test
targets — which is the whole point of the relocation.

## Dependency-delta basis

Stated as the **package count**, never as lockfile byte-identity:

- `Cargo.lock` packages: **564 → 564** (no package admitted or removed)
- the lock moved **2 edge lines** — `"assert_cmd",` and `"assert_fs",` dropped from
  `conductor-tauri`'s dependency list
- `cargo deny check advisories bans licenses sources` over the **new** lock: **exit 0**,
  `advisories ok, bans ok, licenses ok, sources ok`

This is the exact inverse of the shape the prior chunk measured (one edge line *added*, count
unchanged), and confirms the same rule: the count is the basis, the byte-identity is not.

## Standing supply-chain probe (the 45th)

Run before any lockfile-touching step, exit captured **before any pipe**:

- `cargo audit` → **exit 1**, `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`
- **Signature reproduced byte-identically** ⇒ the deferral re-pins as the 45th probe. Not a
  deviation, not a gate failure.
- `cargo deny check advisories bans licenses sources` (the overlap probe, run unconditionally) →
  **exit 0**, green both before and after the change.
