# Scope — Emit scrubber and percentile math under test

**Marker:** `2026-09-14-emit-scrubber-and-percentile-math-under-test`
**Version:** conductor-0.3.0 · **Epoch 2 — Scenario assertion hygiene** (first entry)
**Working entry:** `conductor-0.3.0/working-route.md:22` · **Promoted:** 2026-09-14

---

## Intent (from the working entry)

Bring `conductor-emit`'s first-measured mutation survivor population down to a **stated floor**, and
**classify the ten timeouts**. Placed first in this epoch on the standing doctrine that a code-facing
audit finding is fixed before the next boundary measures it (operator directive, 2026-09-14 wrap).

This is a test-authoring chunk on an already-shipped surface. It changes **test coverage and the
mutation gate's reach**, not the emission behaviour those tests describe — with one bounded exception
(the accessor retire/assert fork, §Decisions deferred to P4).

---

## The population — RE-SCOPED from the shard to the two files

The entry's counts are scoped to `--shard 1/4`. The phase directive supplied three facts the entry
could not, and **each was re-verified read-only at HEAD `d6105af` this session** (cargo-mutants 27.1.0,
`cargo mutants --list -p conductor-emit`, listing only — no build, no test):

| Claim | Status |
|---|---|
| `--shard 1/4` is a **contiguous block**, indices **115–229 of 458** — the second quarter, not every fourth mutant | **measured — holds** |
| The block lies **entirely inside the two target files** | **measured — holds** |
| **67** of `exception.rs`'s **163** mutants · **48** of `latency.rs`'s **64** | **measured — holds** (67+48 = 115) |
| The two files hold **227** mutants; the audit read **115** of them, leaving **112 never measured** | **measured — holds** |
| The accessors' six mutants sit at **182–187**, inside the block | **measured — holds** — and they are the *first six* of `latency.rs` (which occupies 182–245; `exception.rs` occupies 19–181) |
| The audit twin's recorded `command` | **measured — holds** verbatim, plus a run-dir `--output` the directive omitted |
| Survivors **36** `exception.rs` / **21** `latency.rs`; `quantile` = **14**, not the prose's 11 | **measured — holds**, from the twin's own 57 survivor rows |

**One correction to the directive's own wording.** It names the unmeasured `latency.rs` tail as
"incl. `next_unit`". Measured, that tail is **16** mutants in **two** functions — `lerp` :138 (**9**)
and `next_unit` :143 (**7**). `lerp` is un-named in both the entry and the directive and is newly in
scope under the two-file gate form. The `exception.rs` head contributes the other **96**.

**Consequence the entry's targets cannot survive.** Retiring the accessors deletes indices 182–187 and
shifts every later index down by six, moving the shard's membership at both ends. So the entry's
prediction line — `emit shard 1/4 survivors 57 → the roster's count` — **cannot be re-measured on the
same population after this chunk**. Per the directive, this chunk **re-scopes that prediction to the
two-file population** (`exception.rs` + `latency.rs` whole, 227 mutants at HEAD, 221 if the accessors
go). The next audit will measure emit the same way, so **wrap's route-resolve must not re-pin the
shard number**.

---

## What this chunk builds

1. **`exception.rs` host-path scrubber trio under discriminating test** — 36 survivors, every one an
   arithmetic or comparison-boundary mutation no test discriminates:
   `is_absolute_path_start` :316 (**2**) · `skip_absolute_path` :320–329 (**11**) ·
   `skip_line_number_suffix` :340–347 (**23**). This is the stacktrace scrubber enforcing the
   project's own ban on host paths in committed artifacts, so the tests are also security-surface
   tests (`security.md` §Artifact hygiene).
2. **`quantile` :125–133 under test** — 14 survivors; the entry names a **property test** as the
   natural shape.
3. **The three `LatencyProfile` accessors** `p50_ms`/`p95_ms`/`p99_ms` :42/:47/:52 — 6 survivors
   (two each, `-> 0` and `-> 1`). **Retire or assert** — see §Decisions deferred to P4.
4. **`sample_durations_nanos` :113** — 1 survivor (`replace + with -`).
5. **The ten timeouts re-measured and classified** — the twin carries no per-mutant timeout rows
   (only `counts.timeout: 10`), so this chunk's own run is the first that can name them. Each ends
   **killed** or **rostered by class**.
6. **The previously-unmeasured 112** of the two files come under the gate for the first time —
   `exception.rs`'s head (96) and `latency.rs`'s tail (16: `lerp` 9, `next_unit` 7). Their survivors
   are unknown at promotion and are part of this chunk's floor.
7. **The fixture family's last clone pair folded** — `conductor-emit/tests/common/mod.rs` ||
   `tests/pii_payload_corpus.rs` at 11 L, onto the shared module. Cited at
   `.andromeda/runs/2026-09-14T18-51-54-code-audit/proposals.md:189`; it is **not** in
   `c-duplication.json`, whose `top` is the top-10 by line count with a 14 L floor.

---

## The floor (acceptance shape)

Per the directive: **the two files' survivors == the test-plan §12 roster's members**, and every
timeout either killed or rostered by class.

`.claude/rules/testing.md:19` is what makes this the reachable floor rather than a wished count — a
ratified accepted-deliberate survivor **stays in `missed.txt` by construction**, so the floor is
"every survivor is a ratified roster member", never "zero survivors".

---

## Surfaces + contracts touched

- `crates/conductor-emit/src/exception.rs` · `crates/conductor-emit/src/latency.rs` — the mutation
  targets. Source edits only if the accessor fork resolves to *retire*.
- `crates/conductor-emit/tests/` — the new tests; `tests/common/mod.rs` absorbs the clone pair.
- `scripts/mutation-roster.toml` — gains `conductor-emit` rows for anything ratified
  accepted-deliberate. **It has none today** (only `conductor-cli` and `conductor-verify`).
- `.andromeda/test-plan.md` §12 Test Decisions Log — owns the ratified classes the roster joins to via
  `member`. A §12 edit is **wrap's amendment channel**, never a phase or implement edit.
- `scripts/mutation-gate.py` — the committed gate instrument; see the blocking finding below.
- **Not touched:** the emission behaviour itself, the OTLP wire path, any scenario, any other
  `conductor-emit` module (`pii.rs`, `rate.rs`, `message.rs`, `client.rs`, `logs.rs`, `topology.rs` —
  231 mutants, all outside this chunk).

**Adjacency to watch, not to fix here.** `conductor-emit`'s doc gate is currently pinned to a COUNT —
`expect = ['exit 0', 'lacks conductor_core', 'contains generated 5 warnings']` (audit-debt
`plan.md:213-219`) — because five pre-existing private-intra-doc-link warnings sit at
`exception.rs:70`, `:155`, `:158` ×2 and `pii.rs:102`. Fixing those five is an **Epoch 5** entry
(`working-route.md:47`), explicitly not this chunk. But this chunk edits `exception.rs` and possibly
`latency.rs`, so if a source edit moves that warning count the pin breaks. Any `cargo doc` gate this
plan carries must be read against the pin, and the count re-measured rather than assumed.

---

## Blocking finding: the directive's gate form is not expressible through the shipped instrument

The directive states the gate form as the audit's command with `--shard 1/4` replaced by the two files
whole, says "the light gate re-runs it", and names **"the project's prior form of a mutation gate … the
audit-debt plan's (`timeout = 3600`, an `artifact` tally dir)"**. Measured, that prior form is
`[[gate]] run = 'python -X utf8 scripts/mutation-gate.py {unit}'`
(`conductor-0.3.0/chunks/2026-09-13-audit-debt-retired-before-epoch-1-closes/plan.md:238-256`, two
entries, `timeout = 3600`, `artifact = 'target/mutation-gate/'`,
`expect = ['exit 0', 'contains MUTATION GATE {unit}: PASS']`). So the directive names **both** the
two-file `-f` command **and** the instrument that would run it — and the two are in tension:
**measured at HEAD, `scripts/mutation-gate.py` cannot express that command**, on three counts read
directly from its `main()` (`:79–130`):

- it takes **exactly one argument**, the unit — `len(sys.argv) != 2` → usage, exit 2;
- it builds `["cargo","mutants","-p",unit,"--test-tool=nextest","--jobs","2","--output",…]` — **no
  `-f`, no `--shard`**, so it runs the **whole crate: 458 mutants**, not the two files' 227;
- it **fails closed on an empty roster** — `if not rows: FAIL — no roster rows for this unit` — and
  `conductor-emit` has zero rows today. **This is a second defect, not only a registration gap:** it
  cannot distinguish an unregistered unit from a registered one whose accepted-deliberate set is
  legitimately empty, so an all-killed unit can never pass. Plan step 8 repairs both.

This does not contradict the directive's *intent* (the two-file population is exactly what it asks
for); it means the instrument needs a small extension to say it. Cost makes the fork material: at the
audit's measured ~7 s/mutant (115 mutants in 801 s at `--jobs 2`), the two files are **~27 min** and
the whole crate **~53 min**, and the whole crate additionally drags in **231 never-measured mutants**
this chunk does not scope — every survivor among them would read as `SURVIVED, not in roster` and fail
the gate. Resolution belongs to P4; it is flagged here because it shapes the Test Commands and the
light-gate cost.

---

## Decisions deferred to P4

1. **Accessors: retire or assert.** The entry states the fork and its own resolution condition — "the
   plan deciding after it confirms whether the emission path reads the fields directly". The entry's
   supporting claim, kept verbatim: *"The accessors are a cross-metric agreement — zero-reference in
   `c-dead.json` `full_candidates` (33 rows; only `p95_ms` reaches the 20-row `top`) AND
   constant-replaceable unnoticed by the mutation collector."* Retiring them is also what moves the
   `dead.zero_ref_candidates` 33 → 30 prediction.
   **P3 CLOSED the resolution condition — the emission path does NOT read them.** Measured four ways
   at HEAD: the three accessor `fn`s ARE indexed (`symbol` probe, `latency.rs` def_line 40/45/50, so a
   0-row result is a finding and not a pattern miss); `calls` filtered to
   `callee_kind='fn' AND callee_file LIKE '%latency.rs'` → **0 rows**; the same filter on `refs` →
   **0 rows**; and `grep -rnE '\.p(50|95|99)_ms\(\)' crates/ scripts/` → **0 hits**. `quantile` reads
   the PRIVATE FIELDS directly (`latency.rs:122-124`), not the accessors. Every non-zero row in the
   unfiltered query was a **name collision** with `conductor-core`'s own
   `EmissionShape::Latency{p50_ms,p95_ms,p99_ms}` fields (`phase_spec.rs:231-233`), which
   `conductor-run/src/dispatch.rs:134-139` destructures by pattern — a different struct in a different
   crate. With workspace `publish = false` (`Cargo.toml:19`), retiring costs no external API.
   **Retire is the evidence-backed arm**; P4 states it.
2. **Gate form** — **RESOLVED at P4 as a decisive material lean:** extend `mutation-gate.py` with an
   optional per-unit file scope declared in `scripts/mutation-roster.toml`. Running the whole crate and
   gating outside the script were both considered and rejected with reasons (plan.md §Constraints &
   rejected approaches). Reasoning: only extending the script satisfies BOTH statements the directive
   makes — the two-file population and `mutation-gate.py`'s `[[gate]]` as the prior form — while
   keeping test-plan §4's named executable form and arch's operator-instrument registration intact.
3. **Timeout disposition** — killed vs. rostered-by-class. **Not an inference: a MEASUREMENT.** The
   audit twin carries only `counts.timeout: 10` and no per-mutant rows, so nothing short of this
   chunk's own run can name the ten. Re-classified from `[inferred]` to measurement-only at the P5
   val-1 backstop, which caught it standing after the P3 closure.

---

## Premise closure (P3, 2026-09-14 at HEAD `d6105af`)

- **VERIFIED → resolved.** Whether the emission path reads `p50_ms`/`p95_ms`/`p99_ms` directly: it
  does **not**. See §Decisions deferred to P4 item 1 for the four measurement bases. Fork 1 closes to
  *retire*.
- **`[premise-corrected: 13 of the 14 are killable; ONE is equivalent]`** Whether `quantile`'s 14
  survivors are discriminated by a property test alone, or need boundary cases beside it. Neither: a
  property test cannot discriminate them, and **exact-value boundary assertions kill 13 of the 14**.
  Measured under EXACT f64 comparison across 5 profiles × 10 008 points (P5 review, 2026-09-14):
  only **`:126:10 <→<=` is equivalent** (0 differing points — at `u = 0.5` both branches evaluate to
  exactly `p50`, since `u/0.5` and `(u−0.5)/0.45` give exactly `1.0` and `0.0`). The other four
  comparison mutants — `:128:17 <→<=` (6 differing points), `:130:17 <→<=` (8), `:130:17 <→==`
  (1616), `:130:17 <→>` (2012) — **DIE** on an exact assertion: branches 3 and 4 are one line
  *algebraically* but not in IEEE754, because `(0.95−0.5)/0.45` is not exactly `1.0` and
  `(u−0.95)/0.04` and `(u−0.99)/0.01` round differently, so the re-routed branch returns a
  neighbouring float (`500.0` vs `499.99999999999994`; `2000.0` vs `2000.0000000000014`).
  **Correction of record:** this bullet previously claimed 5 equivalents. That rested on a model
  comparing with a `1e-9` epsilon, which swallows the ~7e-15 differences an exact `assert_eq!` on
  `f64` catches — the operator re-measured exactly and the epsilon was the defect, not the mechanism.
  The consequence for the plan is concrete: the boundary assertions must compare EXACT values, since
  a tolerance band wide enough to feel safe re-creates the survivor. Still a model of the Rust, so
  the run confirms it.
- **RESOLVED, and the answer changed the GATE rather than the roster.** Whether any `conductor-emit`
  survivor is genuinely accepted-deliberate: at most **one** (`:126:10 <→<=`), and possibly none once
  the run speaks. So the roster's `conductor-emit` set may legitimately be **EMPTY** — which the
  shipped gate cannot express: `mutation-gate.py:85-87` treats "no rows" as an UNREGISTERED unit and
  FAILs, conflating it with a registered unit whose survivors all died. That is the honest all-killed
  outcome, and the security criterion actively requires it for the scrub path, so the plan could
  otherwise only pass by rostering a survivor it is forbidden to roster. **Fix moved into the plan
  (step 8): the roster's per-unit DECLARATION is the registration; a declared unit with zero rows
  passes iff `missed.txt` is empty; FAIL stays for an undeclared unit** (operator review, 2026-09-14).
- **VERIFIED, with a sequencing consequence.** `citation_home` must resolve from `ROOT` —
  `mutation-gate.py:89-92` checks `(ROOT / citation_home).exists()` and fails the gate on a dangling
  path. The chunk's wrap-written `report.md` does **not** exist while /implement runs, so the row must
  cite an `evidence/` file this chunk CREATES during implement (the precedent row cites
  `.../evidence/disposition-ledger.md`). Citing the report would red the gate at implement time.
- **Still open — measurement only.** What the 112 never-measured mutants do under the existing suite.
  Now enumerated per function rather than as a bulk number (see research.md §Graph impact); only the
  run answers it, and it is inside the floor.
- **Still open — measurement only.** Whether `lerp` (9 mutants, none ever measured, called only from
  `quantile`) is killed transitively by `quantile`'s boundary tests or needs its own.
- **New, VERIFIED.** `exception.rs` does **not** delegate to `conductor-core::redact` —
  `grep -nE 'conductor_core|redact' crates/conductor-emit/src/exception.rs` → 0 hits, consistent with
  the dropped `conductor-emit → conductor-core` edge. So the workspace carries **two independent
  host-path rule sets**, and these tests pin the second one. Whether the two AGREE is a real question
  but is **out of this chunk's scope** — recorded, not reconciled here.

---

## Carried caveats

- Every survivor count inherited from the audit is scoped to **shard 1/4**. The other three
  `conductor-emit` shards are `budget-exhausted`, so the unit-wide figure stays unmeasured; the
  "57 → the roster's count" target must **not** be read as a unit-wide claim. This chunk narrows the
  claim to the two-file population rather than inheriting the shard scope.
- The `quantile` 11 → 14 correction is owed to the **next `code-metrics.ndjson` record's
  `corrections[]`**, per the relay's named channel — not an edit to that run's `proposals.md`.
  This chunk does not discharge it; it is recorded here so it is not lost.
