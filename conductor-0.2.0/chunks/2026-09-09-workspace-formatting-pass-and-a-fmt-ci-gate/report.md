# Report — 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate

**Chunk:** Workspace formatting pass and a fmt CI gate — the tree made `cargo fmt --check`-clean and the gate added to `.github/workflows/ci.yml` so it cannot re-rot
**Date:** 2026-09-09
**Commits:** none since `last_wrap` — HEAD is `6ec89dd` (the prior chunk); this chunk's work is uncommitted until P7

## Changes (structured — detectors read this)

- **Files:** 60 `.rs` across 8 crates (rustfmt output only, never hand-edited — derived as the file set `cargo fmt --all --check` named at HEAD, `fmtstat.py`) · `.github/workflows/ci.yml` (+8 lines, 0 deletions) · NEW `conductor-0.2.0/chunks/2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate/verify_fmt_token_invariant.py` (chunk evidence, not shipped — nothing under `crates/` or `scripts/` references it).
  Per-crate distribution (sites/files, from that same parse): `conductor-verify` 110/18 · `conductor-run` 64/8 · `conductor-emit` 51/16 · `conductor-timeline` 24/8 · `conductor-faults` 12/5 · `conductor-tauri` 10/2 · `conductor-cli` 7/1 · `conductor-core` 4/2. `conductor-report` carries 0 and is untouched.
- **Symbols / APIs:** **none** — no symbol, signature, call site, export, port, socket or env var added, removed or changed. Established mechanically, not asserted: **per-file token-multiset identity across all 60 files** (`verify_fmt_token_invariant.py` → `token-multiset differences: 0`), i.e. for each file the multiset of identifiers, string literals and numerals on the added side equals the removed side. 262 distinct string literals per side, none exclusive to either.
- **Crates / modules:** none added, removed or changed. The nine workspace members are unchanged (`Cargo.toml` untouched).
- **Dependencies:** **none** — `Cargo.lock` byte-unchanged (not in `git status --short`); no `[workspace.dependencies]` entry added; rustfmt is a toolchain component (`rust-toolchain.toml` `channel = "1.95.0"`, rustfmt present), never a workspace dependency.
- **Schema / config:** none. No migration, config key, violation schema or scrub/redaction shape touched.
- **Spec-master edits:** none in P1 — the two `Expected amendments (wrap)` entries below are P2's to apply.
- **Counts / qualifiers moved:** **none — verified.** The chunk moves no count any master states. The `rust` job's step count went 21 → 22 (`yaml.safe_load` of `ci.yml`), and no master states a step count; the CI **job** count is unchanged at 3. The 282-site / 60-file figures are this chunk's own measurements and appear in no master.
- **Dev-tool versions:** none — no external CLI tool installed or upgraded.
- **Harness / gate surface:** ONE added CI step. Job `rust` ("Rust gate (build · test · lint · supply-chain · coverage)", `windows-latest`) gains `Formatting gate (cargo fmt)` running `cargo fmt --all --check`, at **index 2**, immediately after `Install toolchain (rust-toolchain.toml)` and before `Swatinem/rust-cache@v2`. No `continue-on-error`, no `if:`, no new job, no runner label, no env handle, no `needs:` edge. Jobs `frontend` and `a11y` are byte-unchanged — proven, not asserted: the `ci.yml` diff is **8 insertions / 0 deletions**, and `a11y_sig` (md5 of the `sort_keys`-normalized `a11y` job) holds at `1a3138d5` across the edit. `scripts/agent-run.{sh,ps1}` are untouched; no new stage selector, no new harness verb (the command count stays five).
- **Cross-project / external claims:** **none new in this chunk.** (The `v2-24` disposition recorded at P7.3 rests on GitHub Actions run `34280136892` and runs `34162118841` / `34251573399` / `34256490781`, whose ground truth is the CI service, read via `gh run view`.)
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **`test-plan.md:469`** states the shipped CI arrangement "is measured as two jobs, both `windows-latest`", citing `.github/workflows/ci.yml:18`, `:178`. **Measured false this chunk:** `yaml.safe_load` of `ci.yml` reports **3 jobs** — `rust` (`windows-latest`), `frontend` (`windows-latest`), `a11y` (**`windows-2025`**). The claim was already false before this chunk (the `a11y` job landed at `2026-09-07-a11y-ci-gate`); this chunk measured it while editing the same file. It also carries line coordinates, which `a11y-plan §3` retired in favour of job NAMES after `:178` moved to `:188`. **Not this chunk's drift by origin — but it is a false present-tense claim about the very file this chunk edits, so it is dispositioned here rather than carried.**
  2. **The runtime-major hypothesis for `v2-24`** — that WebView2 runtime 151 was why the hosted runner's remote-debugging endpoint never opened — is **FALSIFIED**. Run `34280136892` installed Evergreen **152.0.4191.66** (Authenticode `Valid`, `O=Microsoft Corporation`; re-read seven minutes later by the diagnostics step) and probe (a) still reported `DevToolsActivePort first seen: never within 90s` with the app alive (`HasExited=False`) and three `msedgewebview2` children resident in Session 2. Dispositioned at P7.3 (`matrix.py defer`), not carried.
- **Expected amendments (from plan):**
  1. `architecture.md` §Established Decisions [CI/CD] — **carried**; its motivating fact is the *Harness / gate surface* bullet above (the `rust` job gains a fmt gate step; the job set stays three). Search that located it: `grep -n 'CI/CD\|ci\.yml' .andromeda/architecture.md` — hits in §Stack CI/CD row, §Established Decisions [CI/CD], §Infrastructure Patterns Build system, §Infrastructure Patterns CI/CD approach, §Inherited Defaults CI/CD.
  2. `obs-plan.md` §9 CI Integration (Lint / typecheck row) — **carried**; motivating fact in the same bullet. Search: `grep -n 'Lint' .andromeda/obs-plan.md` → the §9 Pipeline-integration row naming `cargo clippy` alone. The shipped check belongs to that stage conceptually but runs as its own early step, so the amendment adds the check and states the placement; it does not describe the two as colocated.
  3. **No amendment is owed to `test-plan.md:455`/`:477`/`:507`** — target-state descriptors this chunk makes TRUE, standing deliberately under the `2026-09-06-coverage-completeness-gate` narrow ruling. **Do not propose retiring them.** (Distinct from `:469` above, which is a false *present-tense* claim and IS dispositioned.)
- **Coverage of new surfaces:** no new external surface, hot-path operation or UI element is introduced. The one new gate surface:
  - `ci.yml` job `rust` step `Formatting gate (cargo fmt)` → validation n/a (no external input; fixed toolchain program, fixed argv, no shell string, nothing operator-supplied) · instrumentation n/a (obs-plan §1 classes the CI pipeline Not-instrumentable) · PII n/a · tests ✓ (its own gate entry, plus the structural probe asserting it exists exactly once in the `rust` job) · a11y n/a (a11y-plan §9 Lint row is `n/a`) · tokens n/a

## Deviations from intent

1. **Gate `cargo test -p <crate>` (runner portability) is RED and is a NOTED DEFERRAL, not a fix.** `conductor-faults` fails `the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines`. **Owner: the route entry minted at P5, immediately before *Release build and bundle*.** Basis is three measurements, not the word "pre-existing":
   - **No green ever existed for this form.** `cargo test -p conductor-faults` (plain) is named by **0** prior plans or reports in either version; the crate's integration suite has always run under `cargo nextest run -p conductor-faults` (**13** prior mentions, 7/7 green), and the only prior `cargo test -p conductor-faults` mentions are the **`--doc`** arm (**5**), which the `2026-06-19-port-occupier-fault` report records as *"0 doctests (deliberately none — a runnable doctest would bind `:4317`)"* — so it never executed `tests/port_occupier.rs` at all. (Basis: `grep -rhoE 'cargo (nextest run|test) -p conductor-faults( --doc)?' conductor-*/chunks/*/{plan,report}.md | sort | uniq -c`.)
   - **The subject predates this chunk.** The file was added at `ae9e697` (2026-06-19); the failing test and its span assertion entered at `cfb30dc` (2026-08-16, `2026-08-16-fault-application-spans`, +41 lines to that file) — `git log -S` on the test name.
   - **The mechanism is a shared-process race, not a diff effect.** Parallel libtest: exit 101, 6/7. `--test-threads=1`: exit 0, 7/7. At HEAD in a clean worktree with its own `CARGO_TARGET_DIR` (50 `Compiling` lines — a genuine fresh build): the **same** failure, 6/7. And per-file token-multiset identity holds across all 60 files, so the diff cannot be the cause.
   The test installs a **process-global** tracing subscriber (`init_observability`, pid-keyed sink) and asserts on the lines it writes — sound under nextest's process-per-test isolation, racing its six siblings under libtest. A retry, a runner pin and an in-test `sleep` are each banned by test-plan §11 and were rejected explicitly at `2026-09-06-operator-gated-live-suite`, so the owning entry names the **cause**, not a mitigation.
2. **P3 smoke narrowed to `agent-run.sh run --e2e`** rather than bare `run` then `status`. Justification: bare `run` re-executes nextest and clippy, already green as gates 6 and 5 in the same session; `--e2e` is the arm that actually boots the reformatted `crates/conductor-tauri/src/main.rs`. `status` ran separately as gate 11.
3. **Line-count figures differ by tool and BOTH are kept.** The plan states **+1581 / −486**, derived at P3 by parsing `cargo fmt --check` output with ANSI stripped (`classify2.py`). The realised pass measures **+1582 / −487** by `git diff --numstat`. Different hunk accounting between the two tools; direction and magnitude unaffected. Deliberately not collapsed to one number — a committed figure that does not reproduce under the obvious re-measurement is how a later chunk inherits a wrong premise.

## Decisions & corrections

- **Operator, P5 review — the a11y criterion was under-proven and was fixed at the evidence, not the wording.** "The `a11y` job is byte-unchanged" was backed only by a step count and a repo-wide flag count, neither of which can discriminate. Entry 11 was extended with `a11y_sig` (md5 of the `sort_keys`-normalized job, pinned `1a3138d5`). It earned itself the same session: during P1 my first `ci.yml` edit anchor matched **twice**, the second occurrence inside the `a11y` job, and only the Edit tool's uniqueness guard stopped it landing there.
- **Operator, P5 review — the nextest entry needed the COUNT beside the verdict.** Added `contains 902 tests run`, on this project's measured stale-binary precedent (friction `2026-09-03T19:12:52Z-b`: a shared `CARGO_TARGET_DIR` across two worktrees let a HEAD-built binary satisfy a later run at 263 passed where the tree defined 277, every line green). Measured here: 902 across 55 binaries.
- **Operator, P5 review — the advisory-db handle must not be bare.** `$CARGO_HOME` unset degrades to `git -C /advisory-db`, whose red reads as residue rather than an unset precondition. Changed to `${CARGO_HOME:-$HOME/.cargo}`.
- **Operator correction, and I had it wrong twice over.** My P5 corpus figure "7 defaulted / 0 bare" was false: the bare-form pattern required `}` or `"` immediately before `/advisory-db`, which `$CARGO_HOME/advisory-db` can never match (a guaranteed zero, not a measurement), **and** the plan under review was counted into its own corpus. Re-derived BY FILE with the plan excluded: **6 defaulted / 1 bare** before this chunk (the bare one at `conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/plan.md:231`, a complete chunk with nothing to fix); 7/1 with this chunk.
- **Operator ruling — gate placement is EARLY (index 2), with its counter recorded.** At index 8 a violation is reported only after the cache restore, `install-action`, `npm ci` + the vite bundle and `cargo build --workspace --locked` (~18 min end-to-end on the 2026-09-08 run); at index 2 it is seconds, and the lost signal is identical because the job is red before `Test + lint` either way. The counter, kept: with the gate at 8 a red push still yields Build and flakiness-budget results; at 2 it yields none.
- **What `--all` buys beyond member-completeness (operator, P5).** Entry 11 pins the CI step's literal text (`'fmt --all --check' in s['run']`), so this plan's gate entry and the shipped workflow step cannot drift apart silently — the W94 substitution class.
- **My own P4 defect, owned.** Gate 7 went into the plan without ever being run at HEAD, because check 9 baselines only `new = true` entries and I judged `cargo test -p` long-established. *"This command is established"* and *"this command is green on these crates"* are different claims and they came apart here — the same shape as the `cargo fmt --check` gate the previous chunk shipped that could never pass. Two of the last three chunks now.
- **Why no code-graph trace exists.** P5 check 7 WARNs by construction: the modify-set holds `.rs` files so the check applies, and no `tree-query-{marker}.json` was written. Deliberate, not oversight — the pass changes no symbol, signature or call site, so an impact query would inform no decision, and per-file token-multiset identity over 60/60 files is a strictly stronger structural claim. No trace was manufactured to clear a WARN.

## Outcome

**Acceptance criteria, re-asserted against the DIFF:**

| Criterion | Verdict against the diff |
|---|---|
| `cargo fmt --all --check` returns 0 at post-pass HEAD | **MET** — exit 0, 0 residual sites |
| CI's `rust` job runs it as a named step, no `continue-on-error`, no `if:` | **MET** — `Formatting gate (cargo fmt)` at index 2; `yaml` parse confirms neither key present |
| The invocation covers every registered workspace member | **MET** — `--all`; `Cargo.toml` declares no `default-members` |
| No new occupied resource (job, runner, port, egress, env handle, crate, lock delta) | **MET** — diff is 8 insertions in one file; `Cargo.lock` not in `git status --short` |
| `Cargo.lock` byte-unchanged; audit + deny exit 0 | **MET** |
| Added step is fixed program + fixed argv, no shell string; no existing gate reordered/weakened | **MET** — `run: cargo fmt --all --check`; diff has 0 deletions |
| Source delta formatting-only, by per-file token-multiset identity | **MET** — `token-multiset differences: 0` over 60 files; guard's known-positive control fires both ways |
| nextest + clippy green, nothing quarantined or relaxed | **MET** — 902/902, clippy 0 warnings |
| Each of the 8 touched crates green under `cargo test -p <crate>` | **UNMET — noted deferral with a named owner** (Deviation 1). Routed to the P5-minted entry; not silently accepted, not fixed under pressure. |
| `--fail-under-lines 60` coverage floor still holds | **MET** — 94.07% lines |
| `a11y` job byte-unchanged, `rust` `if: always()` uploads still fire | **MET** — `a11y_sig 1a3138d5` unmoved; 0 deletions in the diff |
| No a11y/WCAG claim derived; Lint stage owes no a11y artifact | **MET** — none made |

**Gates (by `run`, in order):**

| `run` | Verdict |
|---|---|
| `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` | exit 0 · `no output` held |
| `cargo audit` | exit 0 |
| `cargo deny check advisories bans licenses sources` | exit 0 |
| `cargo fmt --all --check` | exit 0 · baseline was `red — exit 1, 282 sites`; red→green |
| `python -X utf8 …/verify_fmt_token_invariant.py` | exit 0 · `last line token-multiset differences: 0` held |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 · 0 warnings |
| `cargo nextest run --workspace --profile ci` | exit 0 · `contains 902 tests run` held (902 passed, 55 binaries) |
| `for c in …; do cargo test -p "$c" \|\| exit 1; done` | **exit 101 — RED, noted deferral, owner = the P5-minted entry** (Deviation 1) |
| `cargo llvm-cov nextest --workspace --profile ci --no-report` | exit 0 · 902/902 |
| `cargo llvm-cov report --fail-under-lines 60 --ignore-filename-regex "[\\/]tests[\\/]"` | exit 0 · **94.07%** lines |
| `python -X utf8 -c "…yaml…"` (ci.yml structural) | exit 0 · `last line jobs 3 rust_fmt_steps 1 coe 3 a11y_steps 12 a11y_sig 1a3138d5` held exactly |
| `bash scripts/agent-run.sh status` | exit 0 |

No `defer` key on any entry; no `leg` entry (no live leg — this chunk drives no external process). **Smoke:** fired on the boot-path condition (`crates/conductor-tauri/src/main.rs`, an entry point, is in the pass) — `agent-run.sh run --e2e` exit 0, **12 passing / 2 skipped**, `Spec Files: 1 passed, 1 total`, driven session attached (`[webview2 152.0.4191.66 windows]` banner present, so not a silent skip), verdict asserted from the printed line by the harness's own `assert_a11y_verdict`, skip tally 2 against an expected set of exactly 2.

**Outcome basis:** /implement's P4 report as given (outcome `surfaced`), **plus an operator directive issued between that report and this one** which (a) ruled the red gate a noted deferral that does not block, supplying its basis, (b) directed the owning route entry be minted at P5 before *Release build and bundle*, (c) directed the `v2-24` disposition to land here via `matrix.py defer`, and (d) confirmed both line-count bases stay. Post-implement artifacts read for this report: `git log`/`git log -S` on `crates/conductor-faults/tests/port_occupier.rs`, the prior-runner corpus grep over `conductor-*/chunks/*/{plan,report}.md`, and `yaml.safe_load` of `ci.yml`.

**Process hygiene** (implement P4's census, re-measured here — the host list is readable from this session):

| Process | Started by | Final state |
|---|---|---|
| cargo / rustfmt / clippy / nextest / llvm-cov | this run | terminated (synchronous) |
| tauri-driver · msedgedriver · conductor-tauri · msedgewebview2 (e2e leg) | the `--e2e` leg | **terminated** — post-run census byte-identical to the pre-run baseline |
| 6 × `msedgewebview2`, PIDs 16848–18140 | **not this run** — `Get-Process` `StartTime` 8/13/2026, ~27 days old | left running; not this run's to stop |
| `head-probe-1` git worktree (HEAD comparison) | this run | removed; `git worktree list` shows only the main tree |
