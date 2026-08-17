# Scope — 2026-08-17-fingerprint-semantics-token-leading

**Working entry:** Fingerprint semantics re-aligned to token-leading — identity-triple premise, variant
tests, the P-017 clause and storm grading against the SUT's current derivation.

**Class:** debt repayment against an external SUT change. Nothing in this repo regressed; Pulse changed
its exception-fingerprint normalization and four Conductor surfaces now encode a superseded rule. All
Conductor gates stay green over them, because they assert Conductor against Conductor — which is why this
is a chunk and not a hot-fix.

---

## The SUT change (verified read-only against `andromeda-pulse` at HEAD `efabe8e`)

Pulse's `normalize_frame` now guards its absolute-path arm with `is_token_boundary`: a path token counts
as absolute only at position 0 or when preceded by a non-path byte. Previously the Unix arm fired on ANY
`/`, and because `is_path_char` admits `/`, the skip swallowed the rest of the token — so only a path's
leading segment survived. That behavior is now named as the bug it was, in Pulse's own doc comment.

**The four normalization axes at `efabe8e`, each confirmed by Pulse's own tests** (not inferred from
reading the scanner):

| Axis | Effect on identity | Pulse test |
|---|---|---|
| `:line` / `:line:col` suffix | **stripped** → same fp | `compute_strips_line_column_suffixes` |
| hex address `0x…` | **stripped** → same fp | `compute_strips_hex_memory_addresses` |
| token-leading absolute path (Unix `/…` or `C:\…`) | **stripped ENTIRELY** → same fp *as another absolute* | `normalize_stacktrace_strips_only_absolute_paths` · `compute_strips_paths_and_line_numbers` |
| relative path | **preserved in full** → significant at EVERY depth | `compute_differs_for_relative_paths_differing_below_leading_segment` · `normalize_stacktrace_preserves_relative_paths_in_full` |

Worked forms, quoted from those tests: `at handler(/usr/lib/thing.rs:10)` → `at handler()`, while
`at handler(src/b/c.rs:10)` → `at handler(src/b/c.rs)`.

**The consequence that bounds the redesign:** no path variation can preserve identity for a
relative-path base. A relative variant differs at any depth; an absolute variant normalizes to the empty
form, which differs from any surviving relative path. So the old triple's "path variant shares the base
fingerprint" is **unrepresentable** — not merely wrong in its threshold arithmetic.

---

## What this chunk changes

1. **The variant model** (`conductor-emit/src/exception.rs`). The same-fp membership must be redrawn from
   the axes that remain: line and hex-address. `PathVariant` and `RelativePathVariant` both become
   different-fp variants — the below-leading-segment teaching split the previous chunk introduced
   collapses into one truth, so that distinction is retired rather than re-pointed.
   **[settled at P4: the same-fp set is a PAIR, `{Identical, Line}`, not a triple.]** The hex-address axis
   was rejected by the operator on research's finding that it needs a `render_stacktrace` shape change and
   works only trailing — new emitted content in a chunk whose premise is to move to the SUT and change
   nothing else. Both path members SURVIVE as different-fp witnesses at two depths, which is how "path is
   significant at every depth" gets asserted; only `Path`'s same-fp status is retired.
2. **The variant unit tests** (same file). `same_fingerprint_variants_match_the_base` currently asserts
   `PathVariant` keeps the base fingerprint; `the_path_variant_differs_below_an_unchanged_leading_segment`
   encodes the dead distinction. Both are re-aimed. The existing
   `stacktrace_carries_no_absolute_host_path` invariant must keep holding.
3. **The scenario** (`scenarios/fingerprint-storm.toml:9-13`). The prose encodes the superseded rule, and
   the variant-mix arithmetic it justifies is what makes the storm's threshold counting true. With the
   same-fp members redrawn, all same-fp occurrences count toward one storm again and the per-phase tier
   boundaries match their phase names.
4. **The P-017 clause** (`.andromeda/architecture.md` §Read-Back Dependency Posture). Wording follows the
   same narrowing. **[corrected at val-1: this is NOT `/implement`'s edit.]** A spec master is never a
   touchpoint — routing one through `/implement` bypasses the amendment flow whole (no sidecar history, no
   playbook validation, and the cascade walks only APPLIED amendments). It rides the plan as an
   `Expected amendments (wrap)` entry and wrap's detectors apply it through the owned path.
5. **The storm grading** (`conductor-run/tests/storm_harvest.rs`). It grades on a single `fingerprint_hex`;
   that premise is restored by the redesign rather than rewritten around.

## Boundaries

- **Pulse is read-only.** No change is made in `andromeda-pulse`; this repo moves to the SUT, never the
  reverse.
- **No live leg is required to land this.** The re-alignment is provable at unit tier against the
  derivation Conductor already recomputes. A live re-proof of the storm family is a separate question and
  is not claimed here.
- **`v2-10` / `v2-11` stay `verified`.** They measured a state that really held; the SUT change
  invalidates only future legs. This chunk does not reopen them.
- **No new capability is claimed for the storm family.** This chunk restores an existing proof's premise;
  whether it also unblocks a NEW matrix claim is P5's call against the ledger.

## Inherited annotations (folded from the working entry)

- **CARRY (2026-08-17 Pulse visit):** the four surfaces above, the measured split shape (~12 base / ~6
  path → two storms; incident count probably still one since L2 coalesces per kind/scope/scope_id; per-phase
  tier boundaries false), and the fact that this entry is the drive-half unblock for Pulse's P-075
  blocker 2 — Pulse declined that capability on it and records that the re-alignment executes here.
- **PREREQ:** re-check `cargo audit` — 25th consecutive, standing deferral since
  `2026-08-08-sut-capability-manifest`, ratified at the `2026-08-10-workspace-key-divergence-probe` wrap
  (re-pins silently). Basis re-verified at the last wrap and unchanged; overlap is `cargo deny`, observed
  true exit 0 across all four classes. Remedy is the bounded wait alone — do not raise a floor, add a
  `deny.toml` ignore, or edit CI.

## Premises closed in P3

- `[premise-corrected: render_stacktrace at exception.rs:217-223 emits "at {fn} ({file}:{line})" with no
  address slot, and normalize_frame's trailing-space trim at :271-273 makes the axis work only at line
  END]` **The hex-address axis is usable, but only via a render-shape change and only trailing.** An
  address appended at the end of a frame line normalizes away completely; one inserted mid-line leaves a
  doubled space and breaks equality. Using it means changing what Conductor puts on the wire.
- `[VERIFIED]` **The absolute-path axis is unusable by construction.** `stacktrace_carries_no_absolute_host_path`
  (`exception.rs:513-521`) asserts no `C:\` / `/Users/` / `/home/` in the rendered stacktrace, and
  `:414-424` additionally asserts the variant stays relative. A base cannot be absolute, so the axis
  (same-fp only between two absolutes) is unreachable.
- `[VERIFIED]` **The dispatcher's variant distribution reproduces Pulse's predicted split exactly.**
  `dispatch.rs:101-103` cycles `variants[occurrence % variants.len()]`: phase 1's 6 split 2/2/2 → 4 base +
  2 path; phase 2's 12 split 4/4/4 → 8 base + 4 path; cumulative 12 base / 6 path. Pulse's P-075 note
  derived ~12/~6 independently and the two agree.
- `[VERIFIED]` **`storm_harvest.rs` needs no change.** `single_fingerprint` (`:63-73`) errors on more than
  one distinct `fingerprint_hex`, so restoring a single fingerprint makes it correct as written; its
  sibling predicates key on `cue_kind` / `occurrence_count`, neither path-semantics-dependent.

## Premise ADDED in P3 (not anticipated by this scope)

- **The derivation itself has drifted, not just its documentation.** `exception.rs:292-300` transcribes
  Pulse's PRE-token-leading `is_absolute_path_start` with no `is_token_boundary` guard, so Conductor
  computes a different fingerprint than Pulse for any frame path containing `/` — including the committed
  base fixture (`src/worker.rs` → `at …(src)` here vs `at …(src/worker.rs)` at the SUT). Transcribing the
  guard is therefore the CORE change of this chunk, and the variant model, prose and tests are downstream
  of it. Scope item 1 above understated this as a variant-model edit.
