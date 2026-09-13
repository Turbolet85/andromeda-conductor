# Report — 2026-09-13-audit-debt-retired-before-epoch-1-closes

**Chunk:** Audit debt retired before Epoch 1 closes — the 17 non-stub mutation survivors dispositioned, the five `conductor-emit/tests/` clone pairs shared, the eleven-key envelope array single-sourced, the `conductor-emit → conductor-core` unused-dep hypothesis tested.
**Date:** 2026-09-13
**Commits:** none since `last_wrap` (`9f079eb`) — this chunk's commit is P7's.

## Changes (structured — detectors read this)

- **Files:**
  - New: `scripts/mutation-roster.toml` · `scripts/mutation-gate.py` · `crates/conductor-emit/tests/common/mod.rs`
  - Modified (code/config, 18): `.gitattributes` · `Cargo.lock` · `crates/conductor-cli/tests/cli_smoke.rs` ·
    `crates/conductor-core/src/{lib,run_record}.rs` · `crates/conductor-emit/Cargo.toml` ·
    `crates/conductor-emit/src/error.rs` · `crates/conductor-emit/tests/{egress,error_spans,exception_events,latency_shaping,multi_service_topology,traffic_rate_ramps}.rs` ·
    `crates/conductor-report/src/journal.rs` · `crates/conductor-verify/src/{manifest,record}.rs` ·
    `crates/conductor-verify/tests/{readback,readback_shape_witness}.rs`
  - Basis: `git status --porcelain` at wrap Setup; diff `225 insertions / 292 deletions` (net −67 source lines).
- **Symbols / APIs:**
  - ADDED `conductor_core::ENVELOPE_KEYS_SORTED` (`[&str; 11]`, `run_record.rs`), re-exported at
    `conductor-core/src/lib.rs:66`. Consumers: `conductor-report/src/journal.rs` and
    `conductor-verify/src/record.rs`, both inside `#[cfg(test)]` modules. NOT wired into
    `conductor-run/tests/journal_conformance.rs`, which asserts key PRESENCE and must stay presence-shaped.
  - **REMOVED `ContractManifest::default_path()`** (`conductor-verify/src/manifest.rs`) — a `pub` fn with
    **zero call sites workspace-wide** (basis: graph `refs` scoped `%conductor-verify%` → 0 rows with
    `probe_hits default_path = 4`; `grep -rn 'default_path' crates/conductor-verify/` → 1 hit, the definition).
    It could never acquire one: `conductor-cli` does not depend on `conductor-verify`, so `Paths::resolve`
    reads the default from the literal at `crates/conductor-cli/src/paths.rs:32`. **No remaining callers** —
    this is a measured claim, not a default.
  - `conductor-emit/tests/common/mod.rs` exports `CapturingService`, `start_stub()`, `LastRequest` —
    test-only, `#![allow(dead_code)]`, not a shipped surface.
  - No new IPC method, endpoint, port, socket or env var. **No new `CONDUCTOR_*` handle.**
- **Crates / modules:** none added or removed. The nine workspace members are unchanged.
- **Dependencies:**
  - **REMOVED the `conductor-emit → conductor-core` edge** (`crates/conductor-emit/Cargo.toml:9`).
  - `Cargo.lock`: exactly one line removed (`- "conductor-core",`). **Package count 562 → 562** (basis:
    `git show HEAD:Cargo.lock | grep -c '^\[\[package\]\]'` = 562; same over the working copy = 562). Stated
    as a COUNT, never as byte-identity — the lock records edges as well as the package set.
  - No dependency added, none bumped. No new `deny.toml` adjudication.
- **Schema / config:**
  - **`.gitattributes` gains a repo-wide wildcard `* text=auto eol=lf`** above the existing
    `coverage-matrix.md text eol=lf` rule, and the comment's "this repo sets no other attributes" framing is
    rewritten. Measured basis: `core.autocrlf=true` in the SYSTEM gitconfig
    (`file:C:/Program Files/Git/etc/gitconfig`); `git ls-files --eol` = 3031 `i/lf` · 13 `i/none` · 4 `i/-text`,
    **no `i/crlf` and no `i/mixed`**, so nothing renormalizes. Verified in force: `git check-attr text eol`
    returns `text: auto` / `eol: lf` on an arbitrary tracked file, and `coverage-matrix.md` keeps its explicit
    `text: set` / `eol: lf`. The "LF will be replaced by CRLF" warning that fired on every file this chunk
    touched now fires **0 times** (`git diff --stat 2>&1 | grep -c` = 0).
  - The envelope's eleven keys and their order are **unchanged** — only the place the name list is written moved.
- **Spec-master edits:** none in P1 (P2 owns any).
- **Counts / qualifiers moved:**
  - `.gitattributes` rule count **1 → 2**. Stated as ONE in two places: `architecture.md:205` ("The repo's
    sole `.gitattributes` rule") and the directory-tree comment `architecture.md:217` ("one rule:
    coverage-matrix.md text eol=lf"). Both are retired by this commit.
  - `conductor-verify` mutant population **148 → 147** (the removed `default_path` left the population).
  - Non-stub mutation survivors **25 → 20**; `conductor-cli` missed 3 → 2, `conductor-verify` missed 14 → 10.
  - `conductor-emit` test files carrying a private loopback stub **6 → 0** (one shared module).
- **Dev-tool versions:** none — `cargo-mutants` re-read at 27.1.0 (unchanged), `cargo-audit` and `cargo-deny`
  unchanged.
- **Harness / gate surface:**
  - NEW operator/local instrument `scripts/mutation-gate.py {unit}` — runs a unit's mutation tier into a fresh
    `target/mutation-gate/{unit}-{stamp}/` (gitignored; `git check-ignore -q` verified) and gates on the TALLY
    against `scripts/mutation-roster.toml`, never on the exit code. It adds **no 6th harness command**: the
    5-command `agent-run.{sh,ps1}` surface is untouched, and no CI step invokes it.
  - NEW committed data file `scripts/mutation-roster.toml` — the expected `missed.txt` SET per unit,
    coordinate-FREE (matched on `(file, mutation)` as a MULTISET), joined to test-plan §12 by a `member` key.
- **Cross-project / external claims:** none — nothing here asserts about Pulse.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **`architecture.md:205` + `:217` — "the repo's sole `.gitattributes` rule" / "one rule".** Measured false
     by this commit: the file now carries two rules. Evidence: the `.gitattributes` diff above.
  2. **The working entry's own predicted outcome, `survivors 25 → ≤ 8`.** Measured unreachable at phase P4 and
     confirmed here: an accepted-deliberate survivor by construction SURVIVES
     (`.claude/rules/testing.md:19`), the workspace ships no `mutants.toml` and no `mutants::skip`, so the
     twelve ratified members stay counted. Measured outcome **25 → 20**. Already dispositioned at P4 (the
     acceptance was re-based on test-plan §10's disposition discipline); recorded here for the ledger, not
     re-opened.
  3. **test-plan §12's identity rule (function + COLUMN) is insufficient.** The class-B `degraded` member's
     column moved 27 → 17 while its mutation description stayed unique across all 25 survivors. Evidence:
     `conductor-0.2.0/chunks/2026-09-03-conductor-run-composition-root-survivors-dispositioned/evidence/disposition-ledger.md:56`
     names it `lib.rs:518:27 (delete field degraded)`; today it measures `execute.rs:105:17`.
- **Expected amendments (from plan):**
  1. `test-plan.md` §12 — re-point four moved coordinates + refine the identity rule to the mutation
     DESCRIPTION + name `scripts/mutation-roster.toml` as the enumeration's executable form. **carried** —
     fact in `Counts / qualifiers moved` and `Spec claims disproved` #3. Sites located by
     `grep -n 'accepted-deliberate' .andromeda/test-plan.md` → 7 hits: the five roster MEMBERS at
     `:609` `:610` `:611` `:612` `:613`, plus `:228` (§4 mutation instrument) and `:502` (§10 disposition
     rule), which state the rules rather than the membership. test-plan is the only master with hits.
  2. `test-plan.md` §12 — record that `ContractManifest::default_path` was REMOVED, so the `conductor-verify`
     unit's mutant population drops by one. **carried** — fact in `Symbols / APIs` + `Counts / qualifiers moved`.
  3. `test-plan.md` §10 (`:496`) — it ENUMERATES named coverage exclusions ("generated code (tonic/prost OTLP
     codegen), the hand-rolled MCP stub (`stub_pulse_mcp` / `conductor-verify/tests/common`) and the tauri
     mocks, and rstest/insta fixtures"), while the shipped invocation uses ONE generic path regex. The two
     differ in BOTH directions: the regex is broader for any `tests/` dir (so the new
     `conductor-emit/tests/common` needs no edit) and narrower for the src-side `stub_pulse_mcp`, which lives
     in `src/bin/` and is therefore NOT excluded. **carried** — fact below under Coverage of new surfaces.
     Basis: `grep -c 'llvm-cov' .github/workflows/ci.yml` → **6** hits, of which `:110` `:111` `:114` carry
     `--ignore-filename-regex '[\\/]tests[\\/]'` (`:41` installs the tool, `:103` is a comment, `:105` is the
     `--no-report` run); `grep -c 'conductor-verify/tests/common' .andromeda/test-plan.md` → **2** hits, of
     which `:496` is the coverage sentence and `:245` describes the crate's test content, not an exclusion.
  4. **NEW this wrap (directive item 1, not in the plan's list):** `architecture.md` §Infrastructure Patterns
     `:205` + the directory tree `:217` — the sole-`.gitattributes`-rule claim. **carried** — fact in
     `Schema / config` + `Counts / qualifiers moved` + `Spec claims disproved` #1. Basis:
     `grep -n 'gitattributes' .andromeda/{architecture,security-plan,design-system,layout-templates,test-plan,obs-plan,a11y-plan}.md`
     → **3** hits across the masters, not 2: `architecture.md:205` and `:217` state the sole-rule claim and are
     RETIRED by this commit; `test-plan.md:80` says `coverage-matrix.md` is "LF-pinned by `.gitattributes` so
     its byte comparison is host-independent", which stays TRUE (the explicit rule still wins) and needs no
     edit. So architecture is the sole owner of the retired CLAIM — it is NOT the sole master that mentions
     the file.
- **Coverage of new surfaces:**
  - `conductor_core::ENVELOPE_KEYS_SORTED` → validation n/a (a const, no external input) · instrumentation n/a ·
    PII n/a (key NAMES, never values) · tests unit ✓ (asserted at both former clone sites, green under both
    runners) · a11y n/a · tokens n/a
  - `scripts/mutation-gate.py` → validation n/a (operator-local instrument, no external input; its only
    argument is a unit name matched against a committed roster) · instrumentation n/a (prints its own verdict;
    mints no span and no `tracing` call site) · PII n/a · tests — exercised by its own two gate runs, plus the
    roster-parse and early-exit paths run directly · a11y n/a · tokens n/a
  - `scripts/mutation-roster.toml` → validation ✓ (the gate asserts every row's `citation_home` resolves) ·
    instrumentation n/a · PII n/a (no host path; repo-relative citation homes only) · tests ✓ (both gates read it)
    · a11y n/a · tokens n/a
  - `crates/conductor-emit/tests/common/mod.rs` → validation n/a · instrumentation n/a · PII n/a · tests ✓
    (six binaries consume it, green under both runners) · a11y n/a · tokens n/a

## Deviations from intent

1. **`crates/conductor-core/src/lib.rs` edited — absent from the plan's touchpoints.** The crate exports
   `run_record` through an explicit list, so a new `pub` const is invisible to both consuming crates until that
   line changes; plan step 9 is unexecutable without it. Justification: a crate-root re-export is a transitive
   requirement of a listed change, which `codebase-research.md` names a boundary member rather than a
   gray-area call. Research gap, recorded.
2. **Plan step 5's "at least three incidents" implemented at ONE.** `StubConfig` has no incident-count knob
   (`canary_in_corpus` is a bool) and `conductor-verify/tests/common/mod.rs` is outside the touchpoints, so the
   plan's literal form needed an out-of-scope edit. For this mutation shape — `==`→`!=` on a loop index — one
   incident discriminates (correct logs 1 witness line, mutant logs 0), where the `delete !` shape
   `.claude/rules/testing.md:72(b)` was written for would not. **Measured, not argued: both `extract.rs`
   guards are in `caught.txt`.** Residual: N=3 would additionally kill an always-log mutant, which
   cargo-mutants does not generate at these sites.
3. **`traffic_rate_ramps.rs`'s helper signature** now takes the shared `LastRequest` alias instead of the
   spelled-out `Arc<Mutex<Option<ExportTraceServiceRequest>>>` — inside a listed file, a consequence of
   introducing the alias, and it removes three otherwise-unused imports.
4. **`.gitattributes` repair rides this commit** — not in the plan; an operator wrap directive (the founder's
   boundary-#5 ruling, owed since 2026-09-13 15:07). Its measured basis was re-derived here rather than
   inherited.

## Decisions & corrections

- **The directive's class split did not reproduce, and the measurement inverted it.** The phase directive
  leaned "TWELVE holes / FIVE ratify-by-class candidates"; matching the 17 against test-plan §12's five-member
  roster by function + column gives **12 already-ratified members / 5 genuinely unowned**. The 12/5 arithmetic
  reached 17 by two errors that cancel — collapsing `spawn.rs:116`'s two polarities into one item, and counting
  the stub's `main → ()` which the same directive excludes.
- **Operator correction at the P5 review (1):** a `#[cfg(test)]` pin for `ContractManifest::default_path` kills
  the mutant but leaves an unused `pub` fn the next audit still counts. Resolved on the SEAM — `conductor-cli`
  does not depend on `conductor-verify` — so the disposition became removal.
- **Operator correction at the P5 review (2):** the roster and test-plan §12 were two copies of one set with
  nothing comparing them. Resolved by making the roster **coordinate-free**: it drops `line:col`, the only
  field both copies carried and the only one that drifts, and joins to §12 by a `member` key.
- **Sweep hazard — an identical `(file, mutation)` pair is legitimate.** Two `stub_pulse_mcp.rs` survivors both
  read `replace == with != in stub_result`, distinguished in `missed.txt` only by coordinates a coordinate-free
  gate discards. A set comparison silently passes with one of them missing; the gate compares MULTISETS.
- **Sweep hazard — `grep 'Status'` matches `StatusCode`.** Trimming the emit test imports, a token survey for
  tonic's `Status` returned hits in three files that were all `opentelemetry_proto…status::StatusCode`, a
  different import that must stay. Reading the hits separated them; the compiler confirmed.
- **Operational fact, in no rule file:** `cargo mutants --output` creates its output LEAF itself but fails with
  `os error 3` when the parent chain is missing. Neither `collectors.md` C1 nor `.claude/rules/testing.md`
  states it, though both cover `--output` in detail (stale-by-construction, the nested `mutants.out/`,
  gitignoring). Cost the chunk's only fix-loop iteration.
- **The gate proved its own premise live:** `cargo mutants` exited **2** on both PASSING runs. A gate keyed on
  the exit code would have reported red over a clean tally — `.claude/rules/testing.md:19` firing in practice.

## Outcome

**Acceptance criteria, re-asserted against the DIFF:**

- (tests) Every one of the 17 in-scope survivors killed or carrying a cited accepted-deliberate classification,
  the accepted set an enumerated SET — **MET**. 5 killed/retired, 12 rostered with rule + citation_home.
- (tests) Both mutation gates print `MUTATION GATE {unit}: PASS` on the tally — **MET** (cli 112/2 missed/92
  caught; verify 147/10 missed/113 caught; both `missed == expected`).
- (tests) The five previously-unowned survivors: four in `caught.txt`, the fifth retired by removal and in
  neither file; net 25 → 20 — **MET**, exactly as the plan predicted.
- (tests) The roster carries no line or column; every row's `member` and `citation_home` resolve — **MET**
  (gate asserts citation_home; 0 dangling).
- (dead-code) `ContractManifest::default_path` gone from the zero-reference set; the
  `"contracts/mcp-contract.toml"` default at exactly one live site — **MET** (`paths.rs:32`).
- (tests) `nextest --workspace` 0; new/relocated tests green under `cargo test -p` verify/cli/emit — **MET**
  (909/909; all three per-crate runs reached every target).
- (tests) Workspace line coverage ≥ 60% — **MET** (94.46% line, 91.24% region).
- (arch) Envelope shape byte-unchanged; both former clone sites still assert as before; `journal_conformance`
  not rewired — **MET** (the const is the sorted key list both sites already compared against).
- (arch) No new crate, no new `CONDUCTOR_*` handle, no new cross-seam edge — **MET**. The const's home adds
  none: both consuming crates already carry `→ conductor-core` (graph `crate_edges`).
- (arch) Standalone per-seam property re-measured in its pinned form — **MET**
  (`cargo check -p conductor-emit --lib` exit 0, `--all-targets` not used; `conductor-core` absent from the
  check graph).
- (security) `cargo audit` + `cargo deny` exit 0 over the regenerated lock, delta as a package COUNT — **MET**
  (562 → 562; audit 7 allowed warnings; deny advisories/bans/licenses/sources ok).
- (security) The `extract.rs` kills pin the NOT-fresh direction — **MET** (the witness test drives the
  preflight arm on a corpus that never goes fresh and asserts the gate BLOCKS, alongside the populated arm).
- (security) No `corpus.db` handle opened, no corpus row staged — **MET** (the `resolve_incident` test runs
  through the stub's MCP tool surface).
- (security) No absolute host path or seam-crate struct name in a committed file; mutation `--output` under
  gitignored `target/` — **MET** (`git check-ignore -q target/mutation-gate` passes).
- (obs) Eleven §3 fields in §3 order, consumed as written; no new span, no `db.*` widening, no `tracing` call
  site added to `conductor-cli` — **MET**.
- (obs) `RUST_LOG` unset for the mutation and nextest runs — **MET**.
- (layouts) `conductor cleanup <run_id>` still prints its documented plain confirmation line — **MET**, and now
  ASSERTED for the first time (`cli_smoke.rs` had zero `cleanup` coverage).
- (design) No new ANSI code, palette row, lamp state or colour literal; the cleanup assertion keys on ASCII
  text — **MET**.
- (a11y) The key list still reproduces a11y-plan §3's eleven; no exclusivity assertion reachable from the a11y
  artifact path — **MET** (`journal_conformance` untouched and still presence-shaped).

**Gates** (the `[[gate]]` entries by `run`, in order — all green, 1 fix-loop iteration):

| `run` | verdict |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, zero warnings |
| `cargo check -p conductor-emit --lib` | exit 0 |
| `cargo doc --no-deps -p conductor-emit` | exit 0 · `lacks conductor_core` (0 occurrences) · `contains generated 5 warnings` |
| `cargo nextest run --workspace --profile ci` | exit 0 — 909/909 |
| `cargo test -p conductor-verify` / `-p conductor-cli` / `-p conductor-emit` | exit 0 each, every target reached |
| `python -X utf8 scripts/mutation-gate.py conductor-cli` | exit 0 · `contains MUTATION GATE conductor-cli: PASS` |
| `python -X utf8 scripts/mutation-gate.py conductor-verify` | exit 0 · `contains MUTATION GATE conductor-verify: PASS` |
| `git -C "$CARGO_HOME/advisory-db" status --porcelain` | exit 0 · `no output` (0 bytes) |
| `cargo audit` | exit 0 (7 allowed warnings) |
| `cargo deny check advisories bans licenses sources` | exit 0 |
| `cargo llvm-cov nextest --workspace --profile ci --no-report` | exit 0 — 909/909 |
| `cargo llvm-cov report --fail-under-lines 60 --ignore-filename-regex '[\\/]tests[\\/]'` | exit 0 — TOTAL 94.46% line |

No `defer` entry. No `leg` entry. No red. **Smoke:** skipped — no boot-path or UI-surface touchpoint, the
absence stated in the plan's prose and validated at phase P5 check 4(2). The CLI entry point was nonetheless
driven end-to-end inside the gate suite (`cli_smoke.rs` runs `Command::cargo_bin("conductor")` through
`run` → `cleanup` → `cleanup`).

**Outcome basis:** `/andromeda-implement`'s P4 report as given, PLUS one operator wrap directive received
after it (the `.gitattributes` instance repair, item 1 above) whose measured basis was re-derived in this
session rather than inherited. No other post-implement artifact.

**Process hygiene:** implement P4's census — taken after the two mutation tiers and the full gate suite, over
`conductor|cargo|rustc|mutants|andromeda|msedgedriver|tauri` — returned EMPTY. Re-measured at this wrap's
Setup: same pattern, still empty. `none left running`.
