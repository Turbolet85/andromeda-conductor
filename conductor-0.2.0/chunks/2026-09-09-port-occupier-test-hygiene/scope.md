# Scope — `2026-09-09-port-occupier-test-hygiene`

**Working-route entry (`conductor-0.2.0/working-route.md:131`, verbatim title + scope hint):**
> Port-occupier test hygiene — the fault crate's port-occupier test green under the shared-process runner,
> free of its process-global observability dependency

**Promoted:** 2026-09-09 · **Version:** conductor-0.2.0 · **Epoch:** 6b — Polish & ship

---

## Reading of "free of its process-global observability dependency" (val-1, intent-incomplete)

The working entry's title clause admits two readings, and P5's validation-1 resolved it as
**intent-incomplete** rather than a plan defect — recorded here because the phrase is the operator's own
wording and the choice is visible in the review:

- **(a) literal** — the test should stop depending on the process-global subscriber (e.g. a thread-local
  `with_default` subscriber instead of `init_observability`).
- **(b) adopted** — the test should be free of that dependency's *consequences*: isolated so the
  shared-process runner is green, while still asserting on what the SHIPPED `init_observability` writes.

**(b) is adopted**, on three grounds that (a) contradicts: `test-plan` §11 (`test-plan.md:538`) prescribes
*"a test binary of its own"* for this exact singleton and names no subscriber substitution; all three
in-tree applications of that remedy (`canary_obs_witness.rs`, `readback_shape_witness.rs`, `obs_span.rs`)
still call `init_observability`; and asserting against a test-constructed local subscriber would weaken the
proof — the assertion's whole point (obs-plan §4) is that the *production* subscriber's allowlist lets
`port` survive. The entry itself says it "names the cause, not a mitigation", so it does not prescribe (a).

## What this chunk is

The runner-portability contract that `test-plan` §4 + §12 state — *every crate green under
`cargo test -p <crate>` as well as under nextest* — is violated in-repo. This chunk makes it true for
`conductor-faults`, and then **measures** whether it is true for the four crates the violating gate never
reached.

The entry names the **cause, not a mitigation**, deliberately. Three remedies are pre-banned by
`test-plan` §11 and were rejected explicitly at `2026-09-06-operator-gated-live-suite`: **a retry, a runner
pin, and an in-test `sleep`**. None of them may appear in this chunk's solution.

## The violation, as the entry states it

- **Subject:** `crates/conductor-faults/tests/port_occupier.rs`, test
  `the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines`.
- **Three measurements**, `measured at 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`:
  - parallel libtest exits **101** (6 of 7 pass),
  - `--test-threads=1` exits **0** (7 of 7),
  - the identical failure reproduces **at HEAD in a clean worktree with its own `CARGO_TARGET_DIR`**
    (50 `Compiling` lines — a genuine fresh build).
- Per-file **token-multiset identity held across all 60 files** of the formatting chunk's diff, which
  **excludes the formatting pass as a cause**.
- **No green ever existed for this form.** `cargo test -p conductor-faults` (plain) is named by **0** prior
  plans or reports in either version; the crate's integration suite has always run under
  `cargo nextest run -p conductor-faults` (13 prior mentions, 7/7 green). The only prior plain-form mentions
  are its `--doc` arm (5), which the `2026-06-19-port-occupier-fault` report records as running **0 doctests**
  deliberately — so that arm never executed this file at all.
- **Provenance:** the subject file dates to `ae9e697` (2026-06-19); the failing test and its span assertion
  entered at `cfb30dc` (2026-08-16, `2026-08-16-fault-application-spans`, +41 lines to that file).

## Mechanism claim (folded from the entry, marker text preserved)

- **VERIFIED at P3** — **Mechanism**, `measured at 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`:
  *the test installs a process-global tracing subscriber through `init_observability` and asserts on the
  lines it writes — sound under nextest's process-per-test isolation, racing its six siblings under
  libtest's shared process.* Re-derived at HEAD (exit 101, 6/7) and **refined**: the defect is *first-match
  record selection* over the shared sink — the log holds 6 concurrent `new` records with distinct ports
  (60250–60255) and `.find()` returned a sibling's (60254), so the failing assertion is
  `the bound port survives the allowlist`. See `research.md` §The mechanism, re-derived at HEAD.

## The second half the entry insists on — the unmeasured four

The wrap gate that surfaced this is a `for … do cargo test -p "$c" || exit 1; done` loop over the eight
crates the formatting pass touched, in the order **core · timeline · emit · faults · verify · run · cli ·
tauri**, and it **short-circuits at the fourth**. Therefore:

- **Measured green** under the shared-process runner: `conductor-core`, `conductor-timeline`,
  `conductor-emit`.
- **Unmeasured** there: `conductor-verify`, `conductor-run`, `conductor-cli`, `conductor-tauri`.
- The entry's own instruction: *fixing the fault crate may simply advance the first red rather than turn the
  loop green, and this entry should expect to **measure** the remaining four rather than assume them.*

A chunk that fixes `conductor-faults` and declares the contract satisfied would be making exactly the
unmeasured-universal claim the entry forbids.

## In scope

1. **Fix `conductor-faults`** so `cargo test -p conductor-faults` (plain, parallel libtest, no
   `--test-threads=1`) exits 0 — without a retry, a runner pin, or an in-test `sleep`.
2. **Preserve the assertion's evidentiary value.** The test's stated purpose (its doc comment) is that the
   span brackets the bind *on the emitted lines*, because `fields(...)` in the macro prove nothing — the
   tracing field allowlist gates them (`obs-plan` §4). Any restructuring must keep a probe that can still
   **fail** if the `port` field stops surviving the allowlist. **VERIFIED at P3** — a finder keyed on the
   port itself would make that assertion unable to fail, which `testing.md:89` names as *not evidence*; P4
   must not resolve the race that way. `"port"` is confirmed present in `ALLOWLISTED_FIELDS`
   (`redact.rs:51`), so the assertion has a real subject.
3. **Measure the four unmeasured crates** under `cargo test -p <crate>` and record each result — green or
   red — as a fact, not an assumption.
4. **Disposition whatever the measurement finds.**
   `[premise-corrected: measured at P3 — all four crates exit 0 under plain `cargo test -p`, so no second
   red exists and the disposition fork never opens]` The route entry's warning that fixing the fault crate
   "may simply advance the first red" is measured FALSE at HEAD: `conductor-faults` is the only red of the
   eight, and fixing it turns the whole loop green. This chunk still *re-measures* all four at implement
   (a P3 measurement is not the chunk's gate), but it plans for green rather than for a fork.
5. **Keep both runners green.** `cargo nextest run --workspace --profile ci` must remain at its current
   green (902/902 across 55 binaries) — the fix must not trade one runner for the other.

## Out of scope

- Changing the runner discipline itself, or `test-plan` §11's ban on retries/pins/sleeps.
- The process-global **panic-hook** race in `conductor-core/src/obs.rs` — a CARRY owned by the *Release
  build and bundle* entry (`working-route.md:133`). **RESOLVED at P3: they are NOT the same defect** (that
  CARRY is two `take_hook`/`set_hook` pairs in obs.rs's own unit tests racing each other; this chunk's is
  record selection over a shared sink), so it stays out of scope. Two facts surfaced for its owner, not
  absorbed here: its coordinates are **stale** — at HEAD the pairs are `obs.rs:535-540` and `:582-587`, not
  `:491-496`/`:532-537` — and the same panic hook makes THIS chunk's failure silent (`research.md`
  §Corollary).
- Any spec amendment. `test-plan` §4/§12 state the contract correctly; this chunk makes the repo match it.

## Surfaces / contracts touched

| Surface | Nature |
|---|---|
| `crates/conductor-faults/tests/port_occupier.rs` | the failing test + its 6 siblings |
| `conductor_core::{init_observability, ObsSink}` | the process-global subscriber the test installs — **VERIFIED at P3: read-only for this chunk.** The remedy is test-side (one test per binary), the convention already applied in the three other crates whose integration tests call it |
| `test-plan` §4 · §12 | the runner-portability contract this satisfies |
| `test-plan` §11 | the banned-mitigation list this must not violate |
| `obs-plan` §4 | the tracing field allowlist the assertion exists to prove |
| `conductor-verify` · `-run` · `-cli` · `-tauri` test targets | measured, not assumed |

## Definition of done

- `cargo test -p conductor-faults` exits 0 under plain parallel libtest, repeatably.
- The allowlist-survival assertion still exists and can still fail on its negation.
- Each of the four unmeasured crates has a **recorded measured result** under `cargo test -p`.
- `cargo nextest run --workspace --profile ci` still green; `cargo clippy -D warnings` still green.
- No retry, no runner pin, no in-test sleep anywhere in the delta.
