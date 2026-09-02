# Scope — Live per-P-ID verdict lamps

**Marker:** `2026-09-01-live-per-p-id-verdict-lamps`
**Version:** conductor-0.2.0 · Epoch 5 — Verification surfaces
**Working entry:** _Live per-P-ID verdict lamps — coverage rows carrying each capability's actual verdict
state, not-yet-run rows visibly distinct from failed_

---

## What this chunk is

The desktop coverage matrix currently renders **classification only**. Every row's Status cell reads
"Not yet run", on every launch, forever — because nothing ever supplies a run outcome to it. This chunk
closes that: after a run, each capability row carries the verdict state its run records actually produced,
and a capability with no run record stays visibly distinct from one that failed.

It also absorbs a CARRY the route has held since `2026-08-09-sut-load-envelope`: the desktop surface is the
**one** of Conductor's three report surfaces that never got the load-envelope signal, so a GUI user cannot
tell an over-envelope run from a clean one.

Both halves land on the same surface (the webview + the thin Tauri command edge beneath it) and are proved
by the same `--e2e` leg, which is why they ride one chunk.

## Half 1 — the coverage lamps

**Stated by the working entry:** coverage rows carry each capability's actual verdict state; not-yet-run
rows are visibly distinct from failed.

- **VERIFIED (P3)** — **The presentational half already ships.** `CoverageMatrix.tsx:43-45` declares
  `lamps?: Record<string, Lamp>`, renders `<StatusLamp lamp={lamp} size="sm" />` when a lamp is
  present, and falls back to a `cov__unrun` "Not yet run" cell when absent (`.cov__unrun` is already
  `var(--text-tertiary)`, `CoverageMatrix.css:87`). The gap is that `App.tsx:246` renders
  `<CoverageMatrix rows={coverage} unbacked={unbacked} />` — **`lamps` is never passed**. So this is a
  data-wiring change, not a component build.
- **VERIFIED (P3)** — **No new command is needed for this half.** `run_report` returns
  `RunRecord[]` carrying `p_ids: string[]`, `verdict`, and `state`, and `lamp.ts:29-39`
  `lampForRecord(state, verdict)` is an arm-for-arm mirror of `conductor-core/src/lamp.rs:37-48`
  `Lamp::for_record` (verdict-first, `Blocked`/`KnownResidual` state-first). The join
  (records → `p_id → Lamp`) is computable from data already crossing IPC.
- **VERIFIED (P3)** — **A collision rule is required and is not yet decided.** `RunRecord.p_ids` is a
  list and many records may name the *same* P-ID (several scenarios, several runs). Which record's lamp
  a row shows — worst-verdict-wins vs most-recent-run — is an open design question this chunk must
  settle. It is a **P4 decision**, not a P1 assertion.
- **VERIFIED (P3)** — `CoverageMatrix.tsx:38` defers this work to "Epoch 10", a stale reference to the
  conductor-0.1.0 epoch numbering; it is also stale on mechanism (it says "runs.db join", but the join
  reads the JSONL journal via `run_report`). The comment is corrected as part of landing the join.

## Half 2 — the load-envelope banner (CARRY from `2026-08-09-sut-load-envelope`)

**Folded verbatim from the working entry** — the webview never got the load-envelope banner. The cli caption
and the Markdown report banner shipped; the third surface did not, so a GUI user sees no envelope signal at
all: an over-envelope run renders in the desktop run-report exactly like a clean one, with nothing saying
its read-back is not evidence about the SUT.

The entry prescribes the landing, and its named coordinates were re-verified at fold:

- **(a) a NEW read-only `#[tauri::command]` exposing the run's `EnvelopeStatus`.**
  `[verified at fold]` `RunsDb::get_envelope(&self, run_id) -> Result<Option<EnvelopeStatus>, RunsDbError>`
  exists at `crates/conductor-report/src/db.rs:222`. `[verified at fold]` `conductor-tauri/src/commands.rs`
  has **no reader** — its six commands are `list_scenarios`, `coverage_matrix`, `unbacked_auto`,
  `run_report`, `start_run`, `stop_run`. `EnvelopeStatus` is imported there, but only on the WRITE path
  (`run_thread` classifies and persists it).
- **(b) a run-level banner in `ui/src/components/RunReport.tsx`** reusing `var(--status-residual)` with an
  **always-rendered DOM text label**, **outside the lamp column and never a seventh lamp** (the
  `not-conductors` Mode-cell precedent).
  `[verified at fold]` `RunReport.tsx` exists; `--status-residual` is declared in `tokens.css` in both
  themes (`#9A93A8` light `:22`, `#6E6478` dark `:71`).
- **Invariant restated by the entry and held here:** `ReportState` stays **five**, `LAMP_META` stays
  **six**. The banner is a run-level qualifier, not a status value.

## The verification premise the entry retires (folded verbatim, `[inferred]`)

> `[inferred]` "Deferred because at authoring time a UI change could not be verified on this host — **that
> premise is now retired: the Webview self-verify entry above resolved the gate on 2026-09-01, and the
> harness drives a real WebView2 session here (the gate is driver-availability, not display), so this chunk
> CAN verify its own UI change through `--e2e`** and `v2-07`'s acceptance names the Markdown report, which
> shipped."

Kept with its marker text verbatim per the promotion fold rule — statedness is not measurement at HEAD.

**VERIFIED (P3), with one narrowing.** `scripts/agent-run.sh:92` carries the `--e2e` arm; `wdio.conf.ts`
drives a real tauri-driver session; the predecessor chunk's master record and the harness rule both record
the measured WebView2 leg. The narrowing: the routine arm loads ONE spec file (above), so "verify its own
UI change through `--e2e`" holds only for assertions placed inside `accessibility.e2e.ts`.

`[verified at fold]` The `v2-07` half of that claim holds: `v2-07` is `verified`, and its `ref` names
`conductor-report/src/report.rs::tests::a_suspect_run_banners_the_breach_above_the_tally` — the **Markdown**
banner. No shipped cap covers the webview envelope surface, so Half 2 is a genuine uncovered gap rather
than a duplicate of `v2-07`.

## Folded PREREQ 1 — close the rust gate deferral

The entry directs: *re-verify the premise before echoing it.* Re-verified at fold against `ece1d24`
(the `2026-09-01-desktop-a11y-sweep` commit), not echoed:

- `[verified at fold]` **Zero `.rs` files** in the commit.
- `[verified at fold]` **Zero Cargo manifest delta** — no `Cargo.toml` touched.
- `[verified at fold]` **`Cargo.lock` byte-unchanged** — absent from the commit entirely.
- `[verified at fold]` **One nuance the entry's wording did not anticipate:** the commit DID touch
  `crates/conductor-tauri/ui/package.json`. The change is a **single script line** (`"a11y:driven": "wdio
  run wdio.conf.ts --suite driven"`) — **not a dependency delta** — and `package-lock.json` was untouched.
  The entry's named falsifier ("a dependency or manifest delta arriving here") therefore did **not** arrive
  in any form reaching the cargo tree; the zero-delta rationale for the RUST gate stands. The entry's
  shorthand "zero manifest" is imprecise at npm grain and exact at cargo grain.

**VERIFIED (P3)** — **The deferral closes by execution regardless of the above:** this chunk adds a
`#[tauri::command]` plus a `conductor-run` read function, so it carries a real `.rs` delta and
`cargo nextest run --workspace --profile ci` + `cargo clippy --workspace --all-targets -- -D warnings`
run here on their own merits, not as a repayment.

## Folded PREREQ 2 — `cargo audit` re-check, 43rd consecutive, COMPACT form

Standing deferral since `2026-08-08-sut-capability-manifest`; operator-ratified at the
`2026-08-10-workspace-key-divergence-probe` wrap.

- `[verified at fold]` **The compact basis holds:** `2026-09-01-desktop-a11y-sweep` touched no Cargo
  manifest and left `Cargo.lock` byte-unchanged (same evidence as PREREQ 1).
- **Discharge unchanged:** reproduce the PROBE-AUTO-SATISFY signature — `cargo audit` true exit **1** with
  first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories bans licenses
  sources` true exit **0** — **capturing each exit code BEFORE any pipe**.
- Any deviation from that signature **restores the full form**. Remedy stays the bounded wait: no floor
  raise, no `deny.toml` ignore, no CI edit. Close the moment it parses.

## Boundaries — what this chunk does NOT do

- **Not a seventh lamp, and not a sixth `ReportState`.** The envelope banner is a run-level qualifier
  rendered outside the lamp column. `LAMP_META` stays six; `ReportState` stays five.
- **Not the cross-surface envelope parity entry.** _Cross-surface envelope parity_ (matrix `v2-25`) —
  CLI and Tauri producing an identical envelope for one seed, plus the stale rmcp wording — is a separate
  markerless entry below this one. `[verified against `working-route.md` + `verification-matrix.json`]`
- **Not the coverage completeness gate.** `v2-26` (a gate failing on any classification gap) is a
  different mechanism on a different surface. `[verified against `verification-matrix.json`]`
- **Not the screen-reader manual spec** (`v2-23`) and **not the A11y CI gate / violation JSON** (`v2-24`).
  This chunk's a11y proof rides the existing `--e2e` leg as an operator/local gate.
  `[verified against `verification-matrix.json`]`
- **Not re-authoring the driver harness.** `wdio.conf.ts`, the tauri-driver stack and the
  `CONDUCTOR_MSEDGEDRIVER` skip-guard landed in `2026-09-01-webview-self-verify-windows-host` and are used
  as-is. Adding specs is in scope; rebuilding the transport is not.
- **No new engine or seam behavior.** `conductor-core`'s lamp truth, `Verdict`/`ReportState`, and
  `RunsDb`'s schema are read, never re-authored. The new command is a thin read-only projection over an
  existing `RunsDb` method.
- **No Pulse UI automation.** Scope law unchanged: Conductor's own webview only.

## Surfaces and contracts touched

| Surface | Expected shape of the change |
|---|---|
| `crates/conductor-run/src/lib.rs` | **[premise-corrected]** a read-side counterpart to `persist` returning the run's `EnvelopeStatus` |
| `crates/conductor-tauri/src/commands.rs` | one NEW read-only `#[tauri::command]` projecting that function |
| `crates/conductor-tauri/ui/src/App.tsx` | build the `p_id → Lamp` join; pass `lamps`; fetch the envelope |
| `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` | stale "Epoch 10" comment corrected; join consumed |
| `crates/conductor-tauri/ui/src/components/RunReport.tsx` (+ `.css`) | run-level envelope banner, text label always in DOM |
| `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` | **[premise-corrected]** the routine suite loads THIS file only — new assertions ride it |

**[premise-corrected: `conductor-tauri`'s only crate edges are → `conductor-core` and → `conductor-run`
(code-graph, rust plane); it has NO edge to `conductor-report`, which owns `RunsDb`.]** The entry's
"(a) a NEW read-only `#[tauri::command]` exposing `RunsDb::get_envelope`" is right about the method and
about the absent reader, but the direct call it implies is not available at the shipped edge set. The
route is `conductor-run` — the composition root that already holds the `conductor-report` edge, already
`use`s `RunsDb`, and whose `classify_run` `conductor-tauri` already calls (`commands.rs:214`). Adding a
`conductor-tauri → conductor-report` edge instead would mint a new architectural edge for one read.

**[premise-corrected: the routine `--e2e` suite loads `./test/a11y/accessibility.e2e.ts` ALONE
(`wdio.conf.ts:75`); `operator-hold.e2e.ts` is the named `driven` suite.]** A new spec FILE would not be
reached by `--e2e`, so it could not satisfy Definition-of-done item 3.

## Definition of done

1. After a run, coverage rows render each capability's real verdict state with **text label + glyph**
   (never color alone); a capability with no run record renders a **distinct not-yet-run** treatment that
   is not a failure treatment.
2. The desktop run-report carries the load-envelope standing for the run, with an always-rendered DOM text
   label — an over-envelope run is visibly distinguishable from a clean one on the GUI surface.
3. Both are asserted by the `--e2e` leg, which stays green (exit 0), with the rust gates and the
   `cargo audit` compact re-check discharged.
