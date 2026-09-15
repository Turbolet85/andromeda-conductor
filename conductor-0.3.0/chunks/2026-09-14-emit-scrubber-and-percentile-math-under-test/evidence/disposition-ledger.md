# Mutation disposition ledger — `conductor-emit`

**Chunk:** `2026-09-14-emit-scrubber-and-percentile-math-under-test`
**Tier:** `python -X utf8 scripts/mutation-gate.py conductor-emit`
**Population:** `crates/conductor-emit/src/exception.rs` + `crates/conductor-emit/src/latency.rs`, whole
(the roster's declared `files` scope). Measured 2026-09-14; cargo-mutants 27.1.0, `--jobs 2`, the audit's
timeout regime unchanged (no `--timeout-multiplier`; the tool auto-set 28s from a 5s baseline).

This file is the `citation_home` the roster's `conductor-emit` rows point at. `.andromeda/test-plan.md`
§12 owns the ratified classes and their reasoning; this is the chunk's measured record behind them.

---

## Why the population is the two files, not the audit's shard

The 2026-09-14 code audit measured `conductor-emit` through `--shard 1/4` and reported 57 survivors.
That shard is a CONTIGUOUS block — indices 115-229 of 458 — lying entirely inside these two files, so it
read **115 of their 227** mutants and left 112 never measured. Retiring the three `LatencyProfile`
accessors deletes six mutants and shifts every later index, so the shard is not re-measurable on the same
population. The gate's subject is therefore the two files WHOLE, and the audit's 57 is a reading of a
strict subset rather than a baseline this run can be differenced against.

## The run

Two runs, both over the same 221-mutant population: the first with the chunk's new tests for `quantile`
and `sample_durations_nanos` only, the second after the `exception.rs` helper assertions were added.

| | run 1 | run 2 (final) |
|---|---:|---:|
| mutants | 221 | 221 |
| caught | 192 | 202 |
| **missed** | **12** | **1** |
| timeouts | 16 | 15 |
| unviable | 1 | 3 |

The eleven `exception.rs` survivors of run 1 were killed by assertions on each helper's own returned
index or boolean; the surviving one is below.

**The unviable count moved 1 → 3, and neither new entry is a genuine unviability.** Both build logs end
`LINK : fatal error LNK1104: cannot open file …conductor_emit-….exe` — the Windows linker could not open
its output because the previous mutant's test binary was still held, alongside
`Blocking waiting for file lock on package cache`. That is host contention during the tier, not a
mutated source that fails to compile. Neither can hide a survivor, and run 1 says so directly:
`exception.rs:286:9 replace && with || in is_hex_address_start` was **CAUGHT** there, and
`exception.rs:340:7 replace += with -= in skip_line_number_suffix` was a **TIMEOUT** — a detection. So
both are accounted for across the two runs; 218 of 221 were tested in run 2 and the three untested are
the one true unviable plus these two, each already observed dead. The tier was not re-run a third time
for a cosmetically clean tally: a re-run would re-roll which builds lose the race, and the evidence for
these two is already in hand.

## Accepted-deliberate — the SET

One member.

### `member-emit-1` — `replace < with <= in quantile`

- **File:** `crates/conductor-emit/src/latency.rs`
- **Mutation:** `replace < with <= in quantile` (the `u < 0.5` branch guard)
- **Class:** `ratified`
- **Rule:** an **equivalent mutant**. The guard's two branches meet at `u = 0.5` with the same value:
  the first evaluates `lerp(0, p50, u / 0.5)` at `u / 0.5 == 1.0` exactly, the second
  `lerp(p50, p95, (u - 0.5) / 0.45)` at `(u - 0.5) / 0.45 == 0.0` exactly, and both are exactly `p50`.
  `u = 0.5` is the only input the flip re-routes, so the mutated function returns a bit-identical `f64`
  for every input in the domain and no assertion over values can distinguish it. Measured under EXACT
  `f64` comparison (never a tolerance) across 5 profiles x 10 008 sample points: **0 differing points**.
- **Evidence:** `quantile_is_exact_at_and_around_every_breakpoint` pins ten exact values at and around
  every breakpoint and kills the other thirteen `quantile` mutants.

**Scope of the acceptance — deliberately ONE mutant, not the comparison class.** Four sibling mutants of
the same shape look equally equivalent and are NOT: the branches beyond `u = 0.5` are one line
algebraically but not in IEEE754, because `(0.95 - 0.5) / 0.45` is not exactly `1.0` and the two tail
divisors round differently, so a re-routed input returns a neighbouring float. All four die on exact
assertions. An earlier prediction that five were equivalent rested on a model comparing with a `1e-9`
epsilon, which swallows those ~7e-15 differences; the operator re-measured exactly at the P5 review and
the prediction was corrected before implementation. Accepting the class rather than the member would
have silently re-admitted four killable mutants.

### Rejected alternatives, as part of the disposition

- **A tolerance band around the breakpoints** — rejected. It does not weaken to a pass, it is the reason
  the four siblings survived the audit at all: the existing convergence test grades within +/-12%, and
  any epsilon wide enough to feel safe re-creates every one of them. Exact equality is what discriminates.
- **Restructuring `quantile` so the boundary becomes observable** — rejected. It is a behaviour change to
  production code made to satisfy a mutation tool, which this chunk's scope forbids (tests only for the
  scrubber, a deletion-only edit for the accessors) and which `test-plan` §11 forbids generally.

## Timeouts — fifteen, classified by class, none rostered

`missed.txt` does not carry them and the gate does not read `timeout.txt`, so this is their record.

All fifteen are the same mechanism: a mutation of a loop's own advance, or a whole-body replacement of a
function whose result drives a loop's advance, inside the three byte-scanning helpers. Each leaves a
`while` loop whose index never passes its bound, so the process hangs rather than returning a wrong
answer, and cargo-mutants' auto-set 28s timeout ends it. A hang IS detection — the harness cannot observe
a value that never arrives — so these are not unobserved returns and no assertion strengthens them.

| helper | timeouts | shape |
|---|---:|---|
| `skip_absolute_path` | 6 | whole-body to `0`/`1`; `+=` to `-=` / `*=` on the two advances |
| `skip_line_number_suffix` | 4 | whole-body to `0`/`1`; `+=` to `*=` on the advances |
| `skip_hex_address` | 4 | whole-body to `0`/`1`; `+=` to `-=` / `*=` |
| `normalize_frame` | 1 | `+=` to `*=` on the scan index |

Run 1 measured sixteen; the sixteenth, `skip_line_number_suffix`'s `+=` to `-=`, hit the link contention
above in run 2 and was recorded unviable there. It is a timeout — i.e. detected — on the reading that
actually tested it.

This matches `.claude/rules/testing.md`'s 2026-09-03 reading — a TIMEOUT means the harness never got to
observe — with the difference that its remedy (bound the await) applies to an async wait, while these are
synchronous infinite loops with no await to bound.

## Unviable — one genuine, recorded not dispositioned

`replace FingerprintVariant::derive -> ExceptionSpec with Default::default()` does not compile
(`ExceptionSpec` has no `Default`), so it never entered the tested population. It is unviable in both
runs. The other two entries in run 2's `unviable.txt` are the link-contention pair explained above, not
unviable mutants.

## Gate verdict

`missed 1 · caught 202 · expected 1` — the multiset matches, and
`MUTATION GATE conductor-emit: PASS`. `cargo mutants` itself exited 3, which carries no verdict in either
direction (`.claude/rules/testing.md`); the tally is what decides.
