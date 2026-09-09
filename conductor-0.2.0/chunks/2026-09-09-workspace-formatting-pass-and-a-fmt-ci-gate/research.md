# Codebase Research — 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate

## Scope
- **Depth:** moderate · **Reads:** 6 · **Globs/Greps:** 18 · **Scripted analyses:** 4
- **Harness rules consulted:** none — this chunk names no live leg (no scenario run, no real process,
  no driver). `.claude/rules/host-win32.md` auto-loaded and was applied to the probe shells; it is a
  host recipe file, not a harness-invocation rule.
- **Code-graph: NOT APPLICABLE, not unavailable.** The chunk changes no symbol, no signature, no call
  site and no crate edge (proven below by per-file token-multiset identity), so the caller-threading
  enumeration the graph serves has no subject and zero queries is the correct count. This is a SCOPE
  fact, explicitly distinct from a graph degradation: both `.andromeda/cache/rust/tree.db` and
  `.andromeda/cache/ts/tree.db` are present and fresh (refreshed 2026-09-08T20:36Z, matching
  `state.yaml`), and neither the cookbook nor `code-graph.py` was needed.

## Files inspected
- `.github/workflows/ci.yml` (structure + the `rust` job's 16 steps, lines 16–187) — the gate's host.
- `Cargo.toml` (workspace block) — member list and the `default-members` question.
- `rustfmt.toml` (full, 1 line) · `rust-toolchain.toml` (full) · `.gitattributes` (full).
- `crates/conductor-report/src/journal.rs` — copied out as the line-ending control subject.

## Graph impact
Not applicable — see Scope. No symbol is added, removed, or re-shaped.

## Patterns detected
- **CI is three named jobs in one workflow** (`.github/workflows/ci.yml`): `rust` — "Rust gate (build ·
  test · lint · supply-chain · coverage)", `windows-latest`, lines 16–187; `frontend` — "Frontend gate
  (npm audit · build)", `windows-latest`, `:188`; `a11y` — "A11y gate (routine arm · axe · contrast ·
  violation JSON)", `windows-2025`, `:219`.
- **The lint half runs through the harness, not a bare cargo call** (`ci.yml:68-72`): the step
  `Test + lint (dogfood agent-run)` is `shell: pwsh` running `.\scripts\agent-run.ps1 run`. There is no
  standalone `cargo clippy` step. So the "Lint stage" test-plan §9 describes is realized inside
  `agent-run`, which matters for where a fmt gate can sit.
- **rustfmt resolves through the pinned toolchain** (`ci.yml`, step `Install toolchain
  (rust-toolchain.toml)` → `rustup show`): no in-workflow toolchain override anywhere in the job, so
  the runner uses `rust-toolchain.toml`'s `channel = "1.95.0"` with its `rustfmt` component — the same
  rustfmt as the dev host (1.9.0-stable). *(answers the arch extract's toolchain question)*
- **Frontend bundle precedes any compiling cargo step** (`Build frontend bundle (ui/dist)` at the job's
  relative step 6, `Build (locked)` at step 7) — a `cargo fmt` check compiles nothing, so the
  `tauri-build` coupling arch flags does not reach it either way. *(answers the arch extract's
  `ensure_frontend` question)*
- **Every artifact upload in the `rust` job is `if: always()`** — coverage (`:109`), JUnit (`:118`), obs
  self-observation (`:182`). *(answers the obs extract's artifact-preservation question: a fail-fast
  step placed anywhere in the job cannot drop these)*
- **Both obs CI gates exist today**: `Obs conformance + zero-unlogged-panics gate` (`ci.yml:144`) and
  `Run-journal conformance gate` (`:172`). Supply chain is two steps in the same job: `Supply-chain —
  cargo audit` → bare `cargo audit`, and `Supply-chain — cargo deny` → `cargo deny check`. *(answers
  the security and obs extracts' "do these exist and in which job" questions — all in `rust`)*

## Conventions to follow
- **Name the job, never the line** (`a11y-plan §3`, restated by the tests and a11y extracts): the fmt
  gate belongs to job `rust`, described by name in the plan and report.
- **A gate step carries no `continue-on-error` and no `if:`** — the shipped WebView2 install gate
  (`2026-09-08`) is the in-repo shape; the diagnostic steps around it are the ones that may.
- **`.gitattributes` carries exactly one rule** (`coverage-matrix.md text eol=lf`), so `.rs` files are
  governed by `core.autocrlf` (this host: `true`), not pinned. Measured: `.rs` files are `i/lf w/lf` in
  both index and worktree.

## Measured facts that decide the plan

### 1. The pass is formatting-only, proven per-file
`cargo fmt --check` at HEAD: **exit 1, 282 sites, 60 files, 8 of 9 crates** (`conductor-report` clean).
Parsing the diff with ANSI stripped: **+1581 / −486 lines, net +1095**.

The load-bearing equality the plan needs is *the pass alters no code, only its layout*. Verified as
**per-file token-multiset identity**: for each of the 60 files, the multiset of identifiers, string
literals and numeric literals on the added side equals that on the removed side.

```
files with fmt hunks: 60
files whose per-file token multiset is IDENTICAL: 60
files with a token-multiset DIFFERENCE: 0
distinct string literals — added 262, removed 262; 0 exclusive to either side
```

This is strictly stronger than the working entry's proposed byte-identity-against-rustfmt check and it
is one whole-diff invariant rather than a 60-file reading task, so **review cost does not scale with
the file count** — which retires the entry's `~40 files` review-cost worry independently of the
count correction (60, not ~40).

Change classes, from the same parse: import re-sorting (30 `use` statements, 45 use-list items — the
edition-2024 order, types before functions) and expression re-wrapping (`bare-delimiter` 228 added vs 5
removed; `method-chain` 279 vs 40). This **verifies the working entry's EVIDENCE clause** ("ordinary
edition-2024 import ordering and line wrapping") on a firmer basis than the entry stated it.

### 2. The gate is line-ending invariant — tested, not asserted
`rustfmt --print-config default` gives `newline_style = "Auto"`, which adapts per file. Tested directly
on a currently-clean file copied out of the tree:

```
LF   copy: rustfmt --edition 2024 --config skip_children=true --check → exit 0, 0 lines
CRLF copy: same command                                              → exit 0, 0 lines
```

So a CRLF checkout cannot make the gate red where an LF checkout is green. *(answers the arch
extract's checkout-invariance question, which `.gitattributes` alone could not)*

### 3. `default-members` is absent → the invocation fork resolves to `--all`
`Cargo.toml` lists all nine members and declares **no `default-members` key**, which is why bare
`cargo fmt --check` and `cargo fmt --all --check` both measure 282 sites today. They are equivalent
*now* and diverge the moment a `default-members` key is added, which would silently narrow the bare
form. arch's criterion ("the invocation must cover every registered workspace member so it cannot go
blind as members are added") is therefore satisfied only by the explicit form.

### 4. Every sensitive surface the extracts named is OUTSIDE the pass
Each token was positive-controlled against the tree before its zero was trusted (two probes —
`garde(` and `Fixed(` — first returned a false 0 from an unescaped `(` under `grep -E`, and were
re-run escaped):

| Surface the extracts asked about | In tree | In the 60 flagged files |
|---|---|---|
| `garde(` annotations (security) | 5 files | **0** |
| `info_span!` (obs) | 5 files | **0** |
| `insta::` / snapshot goldens (tests) | 4 files | **0** |
| `[PASS]`-class status prefixes (design, layouts) | 8 files | **0** |
| `owo_colors` (design) | 1 file | **0** |
| `comfy_table` (design, layouts) | 1 file | **0** |
| `rusqlite` bound params (security) | 3 files | **0** |
| `creation_flags` / `src/spawn.rs` (security) | 1 file | **0** |
| `conductor-core/src/**` incl. `obs.rs`, `redact` (obs) | — | **0** |
| `conductor-cli/src/**` (design, layouts) | — | **0** |
| `commands.rs` (obs `tauri.command.*` guard) | — | **0** |
| `conductor-report` (all) | — | **0** |

The only `src/` files in the pass outside the engine crates are `conductor-tauri/src/main.rs` and
`conductor-tauri/src/pause.rs`; `conductor-cli`'s single flagged file is
`tests/cross_surface_parity.rs`, and `conductor-core`'s two are both under `tests/`. Combined with the
token-multiset identity above, every preservation constraint the design, layouts, obs, a11y and
security extracts raised is satisfied **vacuously** — the pass does not reach those surfaces at all.

Two further extract questions, both answered by locating the real artifact rather than by a token
probe (a first probe for `journal_conformance` inside `*.rs` returned a meaningless 0 — the name is a
FILE, and the functions inside it are spelled `assert_journal_conforms`):

- **`journal_conformance` is `crates/conductor-run/tests/journal_conformance.rs`, and it is NOT among
  the 8 flagged `conductor-run` files** (those are `baseline_harvest`, `canary_wire`,
  `connection_harvest`, `lamps_fixture`, `lifecycle_harvest`, `lifecycle_live`, `restart_harvest`,
  `storm_harvest` — all under `tests/`). Both the `rust` job (`ci.yml:178`) and the `a11y` job
  (`:516`) run it via `cargo nextest run -p conductor-run --test journal_conformance`; the pass does
  not touch it. *(answers the a11y and obs extracts' question)*
- **The `a11y` job does declare `CONDUCTOR_A11Y_STRICT: '1'`** (`ci.yml:347`, commented at `:308`).
  The printed-verdict assertion the a11y extract pairs with it lives in `scripts/agent-run.sh` and
  `scripts/agent-run.ps1`, **not** in the workflow — so a `ci.yml`-only edit cannot disturb it.
  *(answers the a11y extract's question; recorded because the extract asked, though the `a11y` job is
  out of this chunk's scope and stays byte-unchanged)*

### 5. No `#[rustfmt::skip]` is needed
The tree contains **zero** `#[rustfmt::skip]` today, no flagged file carries a snapshot assertion or a
hand-aligned token table, and token-multiset identity holds in all 60. The scope's open premise about
needing a skip resolves to *no*.

## New files to create
None.

## Files to modify
- **60 `.rs` files** across 8 crates — formatting only, produced by `cargo fmt`, never hand-edited.
  The list is reproducible at implement time by `cargo fmt --check` and is not pinned here because the
  pass empties it. Distribution: `conductor-verify` 110 sites/18 files · `conductor-run` 64/8 ·
  `conductor-emit` 51/16 · `conductor-timeline` 24/8 · `conductor-faults` 12/5 · `conductor-tauri`
  10/2 · `conductor-cli` 7/1 · `conductor-core` 4/2.
- **`.github/workflows/ci.yml`** — one added gate step in job `rust`.

No caller threading is owed: no signature changes, so the boundary-member enumeration the reference
requires for signature-changing symbols has an empty subject. No manifest, registry, allowlist or
capability file is implicated — the gate names no new handle and adds no dependency.

## Open questions
- **Where in job `rust`'s step order does the gate sit?** → blocks: **plan-decision**. Two defensible
  placements: early (fail fast before the ~long build/test/coverage steps, cheapest feedback) or beside
  the existing lint step. The `if: always()` uploads mean artifact preservation does *not* constrain
  it, so the choice is purely feedback-time vs. stage-fidelity to test-plan §9's Lint row. P4 resolves.
- **Does the pass leave `nextest`, `clippy` and the coverage floor green?** → blocks:
  **implementation-scope**. Cannot be known before the pass runs; token-multiset identity makes
  regression very unlikely but is not a substitute for running the gates. The plan must name all three
  as Test Commands.
*(Two questions, both with a named blocked step. A third item — whether the entry's "only
`commands.rs` re-sorted" half is true — names no blocked step, so per this reference it is a NOTE, not
a question: it is a claim about the 2026-09-04 commit's history, `commands.rs` and `spawn.rs` are both
clean at HEAD either way, and it shapes no step. Recorded unverified in `scope.md` rather than
chased.)*
