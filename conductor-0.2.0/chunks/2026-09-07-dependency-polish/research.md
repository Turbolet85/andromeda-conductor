# Codebase Research — 2026-09-07-dependency-polish

## Scope
- **Depth:** deep · **Reads:** 21 · **Globs/Greps:** 17 · **Graph queries:** 1 (rust plane)
- **Harness rules consulted:** none — no live leg in this chunk (scope Boundaries: "No live Pulse").
  The one live-capture COUPLING found is recorded under Open questions, not driven here.

## Files inspected
- `Cargo.toml` (workspace deps + members) — `indicatif = "0.17"` (`:64`), `opentelemetry-proto = "0.32.0"` (`:39`), `tonic = "0.14.6"` (`:40`); members list = **9**.
- `Cargo.lock` — resolves `indicatif 0.17.11`, `opentelemetry-proto 0.32.0`, `tokio 1.52.3`, `tauri 2.11.3`; `number_prefix 0.4.0` has **exactly one** dependent (`indicatif`).
- `crates/conductor-cli/src/render.rs` (`:130-160`) — `spinner(len)`: `ProgressDrawTarget::stderr()` when `stderr().is_terminal()` else `::hidden()`, `ProgressBar::with_draw_target(Some(len as u64), target)`, `ProgressStyle::with_template("{spinner} {pos}/{len} scenarios")`, `set_style`, `enable_steady_tick(120ms)`.
- `crates/conductor-cli/src/pause.rs` (`:17-50`, `:77-83`) — `PromptResolver { spinner: Option<ProgressBar> }`; `bar.suspend(render_and_prompt)` **is** the stop-in-place freeze design and layouts both mandate.
- `crates/conductor-cli/Cargo.toml` — declares `indicatif` + `inquire`, and **no `tracing`** (confirms obs-plan §4/§6's cli row).
- `crates/conductor-emit/Cargo.toml` — `opentelemetry-proto = { workspace = true, features = ["gen-tonic","trace","logs"] }`, **no `default-features = false`**; `metrics` NOT enumerated.
- `crates/conductor-run/Cargo.toml` (`[dev-dependencies]`) — bare `opentelemetry-proto.workspace = true` + `tonic.workspace = true` (no feature list at all → pure `default = [full]`).
- `crates/conductor-verify/Cargo.toml` — `[dependencies]:10` tokio features `["process","io-util","rt","macros","sync"]` (**no `time`**); `[dev-dependencies]:31` `["rt","macros","io-util","time","test-util"]`.
- `crates/conductor-verify/src/preflight.rs` (`:330-340`) — the `tokio::time::sleep(poll.interval)` call the standalone build fails on.
- `crates/conductor-emit/src/pii.rs` (`:32-108`, `:235-255`, `:320-346`) — the 7-variant enum, `all() -> [PiiCategory; 7]`, `field_key()`, `index()`, `PiiCorpus { values: [String; 7] }`, `seeded()`'s seven sequential draws, `generate()`'s arms, the arity test.
- `crates/conductor-run/tests/pii_harvest.rs` (`:88-100`, `:145-195`) — pinned live-leg log lines and the affix/needle assertions.
- `crates/conductor-emit/tests/pii_payload_corpus.rs` (`:95-105`, `:135-155`) — the two `all()` callers and an arity assertion.
- **Vendored `opentelemetry-proto-0.32.0`** — `src/lib.rs`, `src/proto.rs`, `src/transform/mod.rs`, Cargo feature graph.
- **Vendored `indicatif-0.17.11` and `indicatif-0.18.6`** — `src/progress_bar.rs`, `src/style.rs`, `src/draw_target.rs`.
- **SUT `andromeda-pulse` @ `83d4060`** — `crates/security/src/scrubber.rs` (`:1-10`, `:71-80`, `:96-112`, `:143-160`).

## Graph impact (from the code-graph query; rust plane, `db_state: fresh`)
- **`PiiCategory::all`** — `rows: 13` callers, exactly matching the CARRY's claim. 10 in `pii.rs` tests, 2 in `conductor-emit/tests/pii_payload_corpus.rs` (`:99`, `:138` — the CARRY cited `:101`/`:140`, off by 2 on the 0-indexed→editor-line conversion), 1 cross-crate in `conductor-run/tests/pii_harvest.rs:183` (CARRY cited `:185`).
- **The caller set is NOT the change set.** Eight further sites move with an eighth category and none is a call: `pii.rs:53` (return type `[PiiCategory; 7]`), `:90` (`values: [String; 7]`), `:325`/`:329` (the arity test's two `assert_eq!`), `:444` (span-attribute count), `:486` (log-record count), `pii_payload_corpus.rs:155`, and `pii_harvest.rs:191` (`assert_eq!(needles.len(), 33, "4 corpora x 7 values + 5 stable affixes")`) — a hard-coded DATA pin.
- **Read-the-hits exclusion:** `PiiCorpus::seeded(7)` at `pii.rs:334`/`:437`/`:482`/`:506` is a SEED numerically equal to the arity. Four false positives; must NOT be touched.

## Patterns detected
- **The freeze is one API call** (`pause.rs:40`): `Some(bar) => bar.suspend(render_and_prompt)`. Everything design-system §Component Patterns 1 and layout-templates §Signature placement mandate rests on `suspend`'s behaviour, not on any Conductor logic.
- **`render::spinner` has exactly one caller** (`commands/suite.rs:27`) and the bar reaches `pause.rs` only as `Option<ProgressBar>`. The A1 blast radius is two files.
- **Feature unification is the A2/B3 mechanism** (`conductor-emit/Cargo.toml` vs `conductor-run/Cargo.toml` dev-deps): the dev-dep site enumerates no features at all, so a workspace build unifies `full` back on even if `conductor-emit` is trimmed.
- **Per-variant matches are the B7 edit shape** (`pii.rs`): `generate()`, `field_key()`, and the test-side shape assertion at `:346` are each an exhaustive `match` — the compiler enumerates them for a new variant. The array types and `assert_eq!` literals are what it will NOT.

## Conventions to follow
- **Append-last for a seeded corpus** (`pii.rs:97-106`): one `ChaCha8Rng::seed_from_u64`, seven sequential `generate(...)` draws; `all()`'s doc (`:52`) states discriminant order = storage order = draw order. A mid-order insert changes every later value.
- **Prefix literals are committed, values are not** (`pii.rs:245`, `:346`; `pii_harvest.rs:188`): the generator's affix (`sk_live_`, `Bearer `, `password=`) is a committed constant and the secret body is RNG-derived. Security's acceptance as written ("no `sk_live_` literal appears in any committed file") is **already false at HEAD** by this convention — see Open questions.
- **Reconciliation moves the doc to the lock, never the reverse** (arch + layouts amendment precedent), and prefers SET-naming over a fresh literal that will re-stale.

## New files to create
- None. Every item is an edit to an existing manifest, source file, doc, or package descriptor.

## Files to modify
- `Cargo.toml` — indicatif pin `0.17` → `0.18` (A1).
- `Cargo.lock` — moves; committed + un-drifted; both supply-chain runners re-run over it.
- `crates/conductor-emit/Cargo.toml` — `default-features = false` on `opentelemetry-proto` (A2).
- `crates/conductor-run/Cargo.toml` — the same at the dev-dep site, which must now enumerate its features explicitly (B3; it has none today).
- `crates/conductor-verify/Cargo.toml` — `time` added to `[dependencies]` tokio features (B4).
- `crates/conductor-run/src/execute.rs` (`:28`, `:35`), `src/preconditions.rs` (`:56`, `:62`), plus the sites above the captured window — doc-comment link re-paths only (B6).
- `crates/conductor-tauri/ui/package.json` + `package-lock.json` — `knip` devDep + script (B5).
- **B7's set:** `crates/conductor-emit/src/pii.rs` (enum · `all()` return type · `field_key()` · `generate()` · `PiiCorpus.values` · `seeded()` · `:325`/`:329`/`:346`/`:444`/`:486`), `crates/conductor-emit/tests/pii_payload_corpus.rs:155`, `crates/conductor-run/tests/pii_harvest.rs` (`:188` affix list if the new category contributes one, `:191` needle count 33 → 37/38). **NOT modified:** that file's `rows_appended: 7` pins at `:96`/`:97`/`:150`/`:151` — historical witness values from the 2026-08-19 capture, correct at 7 permanently (see Open questions, withdrawn item).
- **Docs reconciled (shape produced here, amendment written at wrap):** `architecture.md` `:14`/`:42`/`:253` (tokio), `:27`/`:200`/`:260` (Tauri); `obs-plan.md` `:25`/`:654` (crate count), `:32`/`:39` (tokio); `test-plan.md` `:25` (crate count), `:30`/`:434` (tokio); `security-plan.md` `:84` (Tauri) + the Accepted-exceptions paragraph (B1c); `design-system.md` and `layout-templates.md` indicatif tooling lines.
- **Not modified, recorded:** `deny.toml` is read-only per scope — but see Open questions, where the indicatif bump may strand one of its entries.

## Open questions
- **The indicatif bump may strand a `deny.toml` entry, and scope forbids touching it.** `number_prefix 0.4.0` has exactly one dependent in the lock (`indicatif 0.17.11`) and is absent from `indicatif 0.18.6`'s dependency list, so RUSTSEC-2025-0119's `[advisories] ignore` loses its subject. security-plan's own 2026-06-23 amendment predicted this verbatim, and its anti-pattern forbids "leaving an ignore standing whose subject the bump removed" — which contradicts scope's "`deny.toml` read-only here". → blocks: **plan-decision** (P4 must either widen scope to the toml or record the stranded entry as owed).
- ~~**B7's live-capture pins cannot be re-derived in this chunk.**~~ **WITHDRAWN — measured false before it reached the plan.** The first reading held that `pii_harvest.rs:96-97`'s `rows_appended: 7` (cross-asserted at `:150-151`) goes stale when an eighth category lands, blocking B7 behind a live leg. It does not. `PINNED_LEG_LINES` is documented at `:88-90` as "the leg's verbatim witness lines, **byte-identical to the 2026-08-19 capture** (run `2026-08-19T20-42-57-839`)" and every assertion in that file parses those frozen strings — so `rows_appended: 7` is a **historical record of what that leg emitted when seven categories existed**, and it stays 7 correctly and permanently. An eighth category cannot retroactively change a past capture. What actually moves is one recomputed count: `leg_corpora()` × `PiiCategory::all()` goes 4×7 → 4×8, so `:191`'s `assert_eq!(needles.len(), 33, "4 corpora x 7 values + 5 stable affixes")` becomes 37 (or 38 if the new category contributes a stable affix), and `no_corpus_value_or_affix_reaches_a_pinned_line` still passes because the eighth category's values were never in the historical leg. **B7 is therefore not live-gated and can land structurally in this chunk.** The only live-dependent part is *proving Pulse scrubs* the new category, which is the harvest-tier claim already declare-only and already out of reach — a pre-existing limit, not a new blocker.
- **B7's gap is a mismapping, not an arity shortfall.** Verified at Pulse `83d4060`: `provider_key` matches `[sr]k_(?:live|test)_[A-Za-z0-9]{16,}` (`scrubber.rs:99`) while `api_key` needs the key=value form (`:77`), and Conductor's `ApiKey` generates `sk_live_` + 24 alnum (`pii.rs:245`) — so Conductor's existing category already emits a `provider_key`-classified value. → blocks: **plan-decision** (does the 8th variant take the un-covered shapes `sk-proj-`/`ghp_`/`AKIA`, and is the existing `ApiKey` generator re-shaped — which would move committed corpus values and collide with append-last?).
