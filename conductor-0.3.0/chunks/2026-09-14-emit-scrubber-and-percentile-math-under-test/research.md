# Codebase Research — 2026-09-14-emit-scrubber-and-percentile-math-under-test

## Scope
- **Depth:** deep · **Reads:** 9 · **Globs/Greps:** 7 · **Graph queries:** 4 (rust plane)
- **Harness rules consulted:** none — no live leg in this chunk (no scenario, no SUT, no driver). The
  path-scoped `.claude/rules/testing.md` auto-loads on the touched files and was read at `:14-26`
  (its Mutation bullet is the governing statement); `.claude/rules/host-win32.md` loads
  unconditionally.
- All measurements below are at HEAD `d6105af`, cargo-mutants 27.1.0, 2026-09-14.

## Files inspected
- `crates/conductor-emit/src/latency.rs` (full, 294 L) — the whole percentile surface plus its six
  existing in-module tests. Decisive for both the accessor fork and the `quantile` disposition.
- `crates/conductor-emit/src/exception.rs` (`:140-175` fingerprint/normalize, `:300-360` the scrubber
  trio, test-fn index; 608 L total) — the trio's real code and the existing test oracle.
- `crates/conductor-emit/Cargo.toml` — dev-deps are `tokio` + `tokio-stream` only; **no proptest, no
  rstest** on this crate today.
- `Cargo.toml` (workspace) — `publish = false` (`:19`); `rstest = "0.26"`, `proptest = "1"` already
  `[workspace.dependencies]` (`:81-82`).
- `scripts/mutation-gate.py` (`:79-135` `main()`, full header) — the gate's real shape.
- `scripts/mutation-roster.toml` (full) — 12 rows, `conductor-cli` + `conductor-verify` only.
- `crates/conductor-run/src/dispatch.rs` (`:130-140`) — the destructuring site that produced the
  name-collision rows.
- `.andromeda/runs/2026-09-14T18-51-54-code-audit/c-mutation-conductor-emit.json` — the audit twin:
  `command`, `counts {mutants 115, caught 48, missed 57, timeout 10, unviable 0}`, `score 45.71`, and
  all 57 survivor rows.
- `conductor-0.3.0/chunks/2026-09-13-audit-debt-retired-before-epoch-1-closes/plan.md` (`:198-290`) —
  the `[[gate]]` precedent this chunk's Test Commands follow.

## Graph impact (rust plane; DB regenerated this run — 2555 nodes / 12365 edges)

**The three `LatencyProfile` accessors have ZERO callers and ZERO references.** Established in the
order the cookbook requires — existence first, so a 0-row result is a finding rather than a pattern
miss:

1. `symbol` probe → the accessor **`fn`s are indexed**: `latency/impl#[LatencyProfile]p50_ms()`
   @ `latency.rs:41`, `p95_ms()` @ `:46`, `p99_ms()` @ `:51` (editor lines; graph `def_line` is
   0-indexed 40/45/50).
2. `calls WHERE callee_name IN (…) AND callee_kind='fn' AND callee_file LIKE '%latency.rs'` → **0 rows**.
3. `refs` with the same filter → **0 rows**.
4. Cross-check `grep -rnE '\.p(50|95|99)_ms\(\)' crates/ scripts/` → **0 hits**.

The unfiltered query's 36 rows were all `callee_kind='term'` — a **name collision**, not usage:
`conductor-core` defines its own `EmissionShape#Latency#p50_ms|p95_ms|p99_ms` fields
(`phase_spec.rs:231-233`), and `conductor-run/src/dispatch.rs:134-139` destructures that enum variant
by pattern. The only in-crate `term` rows are `LatencyProfile::new`'s struct literal (`:33-35`), each
accessor reading its own field (`:41/:46/:51`), and **`quantile` reading the private fields directly**
(`:122-124`) — which is the fact that decides the fork.

**Consequence for the plan:** retiring the three accessors is a pure deletion with no caller threading
and no cross-crate blast radius; workspace `publish = false` means no external API is broken. This is
also the change that moves `dead.zero_ref_candidates` 33 → 30.

## The re-scoped population (the gate's real subject)

Derivation: `cargo mutants --list -p conductor-emit` (458 rows) and `… --shard 1/4` (115 rows),
positions matched back into the full list.

`--shard 1/4` is a **contiguous block, indices 115-229**, lying entirely inside the two target files.
`exception.rs` occupies 19-181, `latency.rs` 182-245. The two files hold **227** mutants; the audit
measured **115**; **112 have never been measured**.

| file | fn | measured | never measured | total |
|---|---|---:|---:|---:|
| exception.rs | `skip_line_number_suffix` | 32 | 0 | 32 |
| exception.rs | `is_absolute_path_start` | **2** | **29** | 31 |
| exception.rs | `skip_absolute_path` | 30 | 0 | 30 |
| exception.rs | `is_hex_address_start` | 0 | **20** | 20 |
| exception.rs | `normalize_frame` | 0 | **17** | 17 |
| exception.rs | `skip_hex_address` | 0 | **10** | 10 |
| exception.rs | `is_token_boundary` | 0 | **7** | 7 |
| exception.rs | `exception_event` · `normalize_stacktrace` · `is_path_char` · `fingerprint` · `render_stacktrace` · `FingerprintVariant::derive` · `exception_trace_request` | 3 | 13 | 16 |
| latency.rs | `quantile` | 32 | 0 | 32 |
| latency.rs | `sample_durations_nanos` | 9 | 0 | 9 |
| latency.rs | `lerp` | 0 | **9** | 9 |
| latency.rs | `next_unit` | 0 | **7** | 7 |
| latency.rs | accessors ×3 | 6 | 0 | 6 |
| latency.rs | `latency_trace_request` | 1 | 0 | 1 |

**Two corrections to the entry's framing this table forces.**
1. The entry calls the trio a 36-survivor subject, but **`is_absolute_path_start` had only 2 of its 31
   mutants measured** — 29 of the trio's own mutants are unmeasured. The trio's true population is 93.
2. **83 of the 112 unmeasured mutants are in the scrubber/normalization family**
   (`is_absolute_path_start` 29 + `is_hex_address_start` 20 + `normalize_frame` 17 + `skip_hex_address`
   10 + `is_token_boundary` 7), which the entry names nowhere. The re-scoped run brings that whole
   family under measurement for the first time, so **the floor cannot be predicted as a number** — it
   is "every survivor killed or rostered", and only the run produces the roster.

**Security's open question is answered YES:** the never-measured head **does** reach the active
deprecated-crypto ban's implementation — 2 mutants directly on `fingerprint` (`:164:5`, both
whole-body replacements), plus 3 on `normalize_stacktrace` (incl. `:244:25 replace >= with <`) and 2
on `render_stacktrace`.

## Patterns detected
- **In-module `#[cfg(test)] mod tests { use super::*; }` is this crate's established unit-test home**
  (`latency.rs:146-294`, `exception.rs:353+`). The existing tests already call the private
  `sample_durations_nanos` directly (`latency.rs:156, 205, 215`). This resolves the tests extract's
  open question: the trio, `quantile`, `lerp` and `next_unit` are all crate-private `fn`s, and the
  file's own convention reaches them **without any `pub` widening** — so test-plan §11's ban and
  arch's "no `pub` for tests" rule are both satisfied by staying in-module.
- **The existing oracle is coarse, which is the mechanism behind the 36 scrubber survivors**
  (`exception.rs:469, :497, :600`): `relative_paths_are_significant_at_every_depth`,
  `token_leading_absolute_paths_normalize_to_the_same_empty_form` and
  `stacktrace_carries_no_absolute_host_path` all assert at the **fingerprint** level — two inputs hash
  the same or differently. A boundary mutation that leaves the normalized output unchanged for those
  specific inputs is invisible to it. Direct assertions on the trio's own returned indices are what
  discriminate.
- **`quantile`'s ±12 % convergence tolerance** (`latency.rs:189`) is why its arithmetic mutants
  survive: a `tail` perturbation moves only the top 1 % of 2000 samples, well inside a 12 % band.
- **`LatencyProfile::new` self-validates `p50 ≤ p95 ≤ p99`** (`:33`), pinned by
  `profile_requires_ordered_percentiles` (`:179`). obs's "the ordering invariant must remain
  test-held" is already satisfied here and is unaffected by retiring the accessors (the fields and the
  constructor stay).
- **`[[gate]]` shape for a mutation tier** — `run = 'python -X utf8 scripts/mutation-gate.py {unit}'`,
  `timeout = 3600`, `artifact = 'target/mutation-gate/'`,
  `expect = ['exit 0', 'contains MUTATION GATE {unit}: PASS']`, plus a `baseline` line
  (audit-debt `plan.md:238-256`).

**Arch's SUT-currency question, answered as far as this chunk can answer it.** `fingerprint`'s doc
comment (`exception.rs:158-160`) names its own provenance: *"Source of truth: `andromeda-pulse
crates/buffer/src/fingerprint.rs` (`compute_exception_fingerprint`), transcribed at HEAD `efabe8e`."*
**MEASURED at the P5 review — the transcription is CURRENT.**
`git log efabe8e..83d4060 -- crates/buffer/src/fingerprint.rs` returns **0 commits**: `efabe8e` is an
ancestor of `83d4060` (the Pulse HEAD the 2026-09-13 P-025 chunk measured against), and the derivation
has not moved between them. So the older sha is a **sha age, not a drift**, and arch's "whether the
shipped code still matches the SUT at its current HEAD" question is answered YES for this file.

This research originally recorded the pin as *unverified currency* and framed it as TIME-axis debt —
correct as a caution, wrong as a conclusion, and the operator settled it with the one command that
decides it. The plan is unaffected: assert the token-leading semantics as transcribed, which is what the
existing pins (`:469`, `:497`, `:531`) already do, and `exception.rs` stays **tests-only** here.
Re-stamping the doc comment's sha is wrap's to do or to carry, not a source edit in this chunk.

## Conventions to follow
- **Gate on the tally, never the exit code** — `mutation-gate.py` prints
  `cargo mutants exit {n} (carries no verdict — the tally decides)` and compares `missed.txt` against
  the roster as a **multiset over `(file, mutation)`**, discarding `line:col`
  (`mutation-gate.py:44-52, 113-130`; `.claude/rules/testing.md:19`).
- **Roster identity is the mutation DESCRIPTION + `member` key**, never `file:line:col`
  (`mutation-roster.toml` header; retired 2026-09-13).
- **`citation_home` must resolve from `ROOT`** at gate time (`mutation-gate.py:89-92`).
- **`-f` resolves from the workspace root**, so full crate paths (`.claude/rules/testing.md:19`).
- `proptest`/`rstest` are already `[workspace.dependencies]`, so `conductor-emit` adds them as
  `workspace = true` dev-deps — **no new package enters the lockfile** (the package count should hold
  at 562; per the 2026-09-02 rule, state the basis as the COUNT, not lockfile byte-identity, since the
  member's dependency list gains lines).

## New files to create
- `crates/conductor-emit/proptest-regressions/` — the first such directory in the workspace
  (`find` → none today); test-plan §7 requires counterexamples persisted here.
- `conductor-0.3.0/chunks/{marker}/evidence/disposition-ledger.md` — the `citation_home` any new
  roster row points at. **Must exist during /implement**, not only after wrap.

## Files to modify
- `crates/conductor-emit/src/latency.rs` — retire the three accessors (`:40-53`); no caller threading
  (0 callers, 0 refs, graph-confirmed).
- `crates/conductor-emit/src/exception.rs` — tests only; **no behaviour change** (security's
  anti-pattern: never widen the scrub to make a mutant die).
- `crates/conductor-emit/Cargo.toml` — `[dev-dependencies]` gains `proptest`/`rstest` (manifest rides
  this list per the caller-threading rule).
- `scripts/mutation-roster.toml` — gains the `conductor-emit` rows.
- `scripts/mutation-gate.py` — file scoping, if P4 takes that arm (see Open questions).
- `Cargo.lock` — dependency-list lines for the member; package count expected unchanged.
- **Companion sweep** (`grep -rn 'p50_ms\|p95_ms\|p99_ms'` over `crates/`, whole tree not `tests/`
  alone): 36 graph rows + the grep agree — every hit outside `latency.rs` belongs to
  `conductor-core::EmissionShape::Latency`, a different struct. **No change, and the reason is the
  name collision.** `crates/conductor-run/tests/dispatch_wire.rs:258-260, :290-292` are in that set
  and need no edit.

## Baselines measured (for the plan's `[[gate]]` entries)
- `cargo check -p conductor-emit --lib` → **exit 0** (arch's standalone-build criterion, green today).
- `cargo doc --no-deps -p conductor-emit` → **exit 0**, `0` occurrences of `conductor_core`,
  `generated 5 warnings` — **the audit-debt pin holds at HEAD**. A source edit that moves this count
  breaks that gate; retiring three `pub` accessors removes no intra-doc link, so it should hold, but
  the gate must re-measure rather than assume.
- `cargo nextest run -p conductor-emit --profile ci` → **exit 0, 77 tests run, 77 passed**.
- Audit twin: 115 mutants in 801 s at `--jobs 2` ≈ **7 s/mutant** → the 227-mutant two-file run is
  **~27 min**; the whole crate (458) would be **~53 min**.

## Open questions
- **The gate form** — `mutation-gate.py` takes one argument and builds `-p {unit}` with no `-f`, so it
  cannot express the two-file population; and it fails closed on an empty roster. Extending it with
  file scoping is the reconciliation the directive's own two statements require. → blocks:
  **plan-decision** (P4 resolves before synthesis; it sets the Test Commands and the light-gate cost).
- **Whether the ONE predicted-equivalent `quantile` mutant (`:126:10 <→<=`) is genuinely equivalent** —
  derived from a model of the Rust, not from a run. → blocks: **implementation-scope** (the roster's size
  depends on it; /implement measures and records what the run actually shows).
  **Corrected at the P5 review:** this research originally predicted FIVE equivalents. That rested on
  comparing with a `1e-9` epsilon, which swallows the ~7e-15 IEEE differences an exact `assert_eq!` on `f64`
  catches. Re-measured with exact comparison over 5 profiles × 10 008 points, only `:126:10 <→<=` has zero
  differing points; `:128:17 <→<=` (6), `:130:17 <→<=` (8), `:130:17 <→==` (1616) and `:130:17 <→>` (2012)
  all differ and are killable — branches 3 and 4 are one line algebraically but not in floating point. The
  epsilon was the defect, not the continuity mechanism.
- **What the 83 never-measured scrubber-family mutants do** under the new tests — unknowable before
  the run, and the largest single unknown in the floor. → blocks: **implementation-scope** (the file
  list is complete either way; only the roster's final size moves).
