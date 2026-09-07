# Scope — Dependency polish

**Marker:** `2026-09-07-dependency-polish` · **Version:** conductor-0.2.0 · **Epoch:** 6b — Polish & ship

**Working-route entry (verbatim head):**
> Dependency polish — indicatif bump preserving the stop-in-place hold spinner, plus
> opentelemetry-proto default-features trim

The entry carries **seven CARRY annotations** and no `PREREQ` / `BLOCKED-ON`. All seven fold into this
scope below. The standing external-decay advisory pin is **not** folded: it was CLOSED at
`2026-09-05-audit-corrective` (security.md §Session Additions), so no re-check pin rides this chunk.

---

## A. What this chunk builds (entry-stated)

**A1 — indicatif bump, preserving the stop-in-place hold spinner.**
Workspace pin is `indicatif = "0.17"` (`Cargo.toml:64`), lock resolves **0.17.11**. The bump must not
regress the operator-pause spinner's stop-in-place behaviour (`conductor-cli` presentation layer; the
paused-count mirror). Registry latest is **0.18.6**, and design-system §Surface: cli / Toolkit-Framework
already names `indicatif 0.18` as target state with the 2026-09-03 amendment assigning that line to
THIS entry — so 0.18 is the named target, not a guess.
`[premise-corrected: all four indicatif APIs Conductor uses are signature-identical between 0.17.11 and
0.18.6, verified in both vendored sources]` The premise that "a major-series move carries an API delta
the spinner code must be re-checked against" is FALSE for Conductor's used surface. Measured
2026-09-07: `ProgressBar::with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self`
(0.17.11 `progress_bar.rs:71` = 0.18.6 `:71`), `enable_steady_tick(&self, interval: Duration)`
(`:177` → `:181`), `suspend<F: FnOnce() -> R, R>(&self, f: F) -> R` (`:445` → `:465`) and
`ProgressStyle::with_template(&str) -> Result<Self, TemplateError>` (`style.rs:48`) are byte-identical.
`suspend` **is** the stop-in-place freeze, so the design/layouts signature constraint is carried by an
unchanged API. Residual risk is behavioural (rendering/draw scheduling), not API-shaped.

**A2 — `opentelemetry-proto` default-features trim.**
Workspace pin `opentelemetry-proto = "0.32.0"` (`Cargo.toml:39`). `conductor-emit` already enumerates
`features = ["gen-tonic", "trace", "logs"]` but does **not** set `default-features = false`
(`crates/conductor-emit/Cargo.toml`), so `default = [full]` still rides at both dep sites.
`[premise-corrected: derived from B3's falsified goal — see the closure below]` This section
originally read "the trim drops the dormant `opentelemetry` / `opentelemetry_sdk` footprint obs-plan §3's
Transitive note describes." **It cannot.** Those two crates ride the `trace` and `logs` features, which
also gate the generated message modules `conductor-emit` imports, so they are non-optional. What the
trim actually drops is `metrics`, `zpages`, `with-serde` (→ `serde`, `const-hex`, `base64`),
and `internal-logs` — NOT `profiles`, which `full` does not include (`profiles = []`, never enabled).
obs-plan §3 recorded this as an *evaluation*, and the evaluation's
result — the dormant SDK footprint is not removable at 0.32.0 while the raw-OTLP surface stands — is
itself a deliverable of this chunk.

---

## B. Folded CARRY freight

### B1 — CARRY (from `2026-08-08-dependency-advisory-remediation`): three doc-vs-artifact gaps
Pre-existing, none introduced by that chunk, all dismissed there under playbook:46. Reconcile the docs
to the resolved artifacts here, where the dependency surface is already open.

- **B1a — `tokio 1.48.x` vs the lock's `1.52.3`.**
  Entry cites `architecture.md:14, :41, :229`. **Fold re-verify (2026-09-07): two of three coordinates
  have drifted.** Measured: `:14` ✓ (Stack table), `:41` → **`:42`** (§Established Decisions [Language /
  Runtime]), `:229` → **`:253`** (§Inherited Defaults). Lock confirms `tokio 1.52.3`.
  **Four further sites the CARRY never named**, found by a content sweep across the seven
  masters rather than by the cited coordinates: `obs-plan.md:32`, `obs-plan.md:39`, `test-plan.md:30`,
  `test-plan.md:434`. The first three explicitly cite arch as their Source, so they are cascade leaves
  of the arch fact rather than independent claims; `test-plan.md:434` states it directly in a tool table.
  Whether these four are in scope here or self-heal at the next cascade is a **P4 decision** — the
  CARRY's own text scopes itself to "arch §Stack/§Established Decisions/§Inherited Defaults".

- **B1b — Tauri `v2.10.x / latest 2.10.1` vs the resolved `2.11.3`.**
  Entry cites `architecture.md:26, :236`. **Fold re-verify: both coordinates drifted and the site count
  is wrong.** Measured arch sites: `:27` (Stack table, "bundler v2.10.x, latest 2.10.1"), **`:200`**
  (§Infrastructure Patterns — Deployment model, "Tauri 2 bundler (v2.10.x)") — a **third site the CARRY
  never named** — and `:260` (§Inherited Defaults). Lock confirms `tauri 2.11.3`.
  A fourth site sits outside arch: `security-plan.md:84` ("optional Tauri 2 (v2.10.x)
  ~3 MB GUI bundle").
  **Hazard recorded at the fold:** a `tauri ≥ 2.10.3` statement is a security FLOOR, not a version
  claim, and is **still true** at 2.11.3 — it must not be swept up as stale. At least six such floor
  sites exist (`security-plan.md:180`, `:370`; `test-plan.md:470`, `:508`; `design-system.md:412`;
  `layout-templates.md:334`). Two further sites state `tauri 2.10.3 (test feature)` as a *version*
  (`test-plan.md:306`, `:436`) and sit ambiguously between the two classes — P4 dispositions each.

- **B1c — `deny.toml` entry counts vs security-plan §Dependency Security.**
  Entry claims `deny.toml` carries **17** `[advisories] ignore` + **8** `[licenses] allow` while
  security-plan's Accepted-exceptions paragraph names exactly one of each (`number_prefix` /
  RUSTSEC-2025-0119 and the `Zlib` allow). **Fold re-verify: the advisories count holds (17 unique
  RUSTSEC ids), the licenses count does not — `deny.toml` now carries 9**, the ninth being
  `BSD-2-Clause` for `arrayref` (via `conductor-emit`→`blake3`), added after the CARRY was written and
  already recorded in arch §Infrastructure Patterns.

### B2 — CARRY (from `2026-08-09-sut-drift-check`): the "8 workspace crates" count
A fourth gap of the same class. The workspace has **9** members (`Cargo.toml` members
list, measured), stale since `conductor-run` was extracted on 2026-06-26. The tier conclusion is
unaffected; only the count is wrong. Site list is **three**, per the CARRY's own 2026-08-21 extension:
- `obs-plan.md:25` ✓ (coordinate holds)
- `obs-plan.md:645` → **`:654`** (coordinate drifted by 9)
- `test-plan.md:25` ✓ (coordinate holds — the third site the original CARRY did not name)

**Deliberate non-scope, carried verbatim:** the derived leaves inheriting this count
(`.claude/docs/obs-summary.md`, `.claude/docs/tests-summary.md`) were deliberately NOT corrected at the
2026-08-21 setup re-run — "a leaf re-derives what its master says, and fixing it ahead of the master
would mint a fresh divergence; they self-heal at the next cascade once these three sites land."

### B3 — CARRY (from `2026-08-11-faithful-emission-dispatcher`): the trim spans TWO dep sites
**Causal-mechanism claim, marker text preserved:** "the `opentelemetry-proto`
default-features trim now spans TWO dep sites, not one. … Trimming default-features on `conductor-emit`
alone will therefore NOT drop the dormant crates from the dev tree."
**The TWO-SITE half is VERIFIED** (both sites measured; feature unification across the workspace means
trimming one leaves the other's `full` default in force).

`[premise-corrected: at opentelemetry-proto 0.32.0 the `trace`/`logs` features gate BOTH the generated
message modules Conductor needs AND the SDK transform modules — one flag, both jobs — so no trim that
keeps the raw-OTLP surface can drop `opentelemetry`/`opentelemetry_sdk`]` **The trim's stated GOAL is
unattainable.** Measured 2026-09-07 in the vendored crate: `Cargo.toml` defines
`trace = [opentelemetry/trace, opentelemetry_sdk/trace]` and `logs = [opentelemetry/logs,
opentelemetry_sdk/logs]`, while `src/proto.rs` gates `tonic::trace::v1`,
`tonic::collector::trace::v1`, `tonic::logs::v1` and `tonic::collector::logs::v1` behind those SAME
two features, and `src/transform/mod.rs` gates the SDK conversions behind them too. `conductor-emit`
imports from every one of those gated modules, so `trace` and `logs` are non-optional and the dormant
SDK crates ride them by construction. The **achievable** win is dropping `metrics`, `zpages`,
`with-serde` (and with it `serde`, `const-hex`, `base64`) and `internal-logs` — NOT `profiles`, which
`full` does not include (`profiles = []`, never enabled) — all of which
`default = [full]` currently enables at both sites. obs-plan §3 recorded this as an *evaluation*
follow-up, so the honest deliverable is the evaluation's RESULT plus the achievable trim — never a
forced trim claiming a footprint it cannot deliver.

Fold re-verify confirms both sites exist: `conductor-emit/Cargo.toml` `[dependencies]`
(`opentelemetry-proto = { workspace = true, features = [...] }`) and `conductor-run/Cargo.toml`
`[dev-dependencies]` (`opentelemetry-proto.workspace = true` + `tonic.workspace = true` — the loopback
wire stub for the per-family dispatch assertions, since `conductor-emit` cannot depend on
`conductor-run`). obs-plan §3's Transitive note scopes the footprint to `conductor-emit`'s dep alone;
the obs-plan §3 amendment was deliberately NOT taken at that wrap (dismissed under playbook 2026-06-21:
test-only use of a library the spec narrows to production scope), so the fact is carried here, where
the trim is actually performed, rather than as spec drift. No runtime edge was added.

### B4 — CARRY (from `2026-09-04-sidecar-spawn-without-a-console-window`, operator WRAP directive item 3)
**`crates/conductor-verify` does not compile standalone.** Fold re-verify (2026-09-07) — **every
coordinate holds**: `cargo check -p conductor-verify --lib` is RED at HEAD, `tokio::time::sleep` at
`preflight.rs:336`, tokio's feature list in `[dependencies]` (`Cargo.toml:10`) is
`["process","io-util","rt","macros","sync"]` with **no `time`**, while `[dev-dependencies]`
(`Cargo.toml:31`) carries `["rt","macros","io-util","time","test-util"]`.
**Causal-mechanism claim, marker text preserved:** "Workspace feature unification hides it,
so the crate builds in the workspace and passes `cargo test -p conductor-verify` (dev-deps supply the
feature) while failing alone; no CI job builds a member standalone, so nothing catches it."
Remedy as stated: declare `time` in `[dependencies]` and re-verify with `cargo check -p conductor-verify
--lib`; **then sweep the other members the same way**, since the same masking applies to any crate whose
feature is dev-only. Landing this is what lets architecture §Established Decisions [Module Boundaries]
— QUALIFIED at that wrap under escalation E1 to stop promising a standalone per-seam build — be narrowed
back. **The arch narrowing itself is wrap's to author, not this chunk's.**

### B5 — CARRY (from the 2026-09-05 boundary-#4 code audit, operator directive at the 2026-09-05 0-pending adaptation)
**Install `knip` as a `crates/conductor-tauri/ui` devDependency plus a `knip` npm script.** Fold
re-verify: `knip` is absent from both `package.json` and `node_modules` ✓; the package currently
declares 10 scripts (`dev`, `build`, `typecheck`, `typecheck:e2e`, `a11y`, `a11y:driven`, `a11y:sr`,
`a11y:sr-empty`, `a11y:sr-error`, `preview`) and 19 devDependencies. The audit's A5 web column
(`dead-code-web`) is `tool-missing: knip absent` (proposals.md §Skips), so the web plane has had no
dead-code series at any boundary. `package-lock.json` moves with it and `npm audit --omit=dev` must stay
clean (dev-tree advisories accepted at dev-tree grain, security.md).
**StrykerJS is DECLINED by the founder (2026-09-05)** — small UI; the e2e/a11y legs cover it — so the
`mutation-web` skip is not a gap to fill: the NEXT code-audit record lists it under `skips[]` as
`declined` (that reason verbatim), never `tool-missing`. **Not this chunk's to write** — recorded so the
disposition is not lost.

### B6 — CARRY (from `2026-09-05-audit-corrective`): 7 rustdoc intra-doc link warnings
The `conductor-run` split left doc comments that resolved inside the one 1944-line module now pointing
across sibling modules. **Fold re-verify: measured live at HEAD 2026-09-07 —
`cargo doc --no-deps -p conductor-run` exits 0 and emits exactly `generated 7 warnings`**, confirming
the CARRY's count. Named targets: `[RunRecord]`, `[HeadlessResolver]` ×2, `[persist]`, `[canary_gate]`,
plus two public-doc-links-to-private-item on `dispatch` and `observe_run_contract`. Sites observed in
the measured output include `execute.rs:28` (`persist`), `preconditions.rs:56` (`canary_gate`),
`preconditions.rs:62` (`observe_run_contract` private-link).
**Causal-mechanism claim, marker text preserved:** "NO gate enforces rustdoc warnings (no
`cargo doc` / `RUSTDOCFLAGS` in `ci.yml` or either `agent-run` shell), so nothing is red — this is owned
housekeeping beside this entry's `cargo check -p conductor-verify --lib` red, not a regression to
hot-fix." Fix at the link (re-path or `crate::`-qualify); the public API is unaffected (27 items,
diff-identical across the split).

### B7 — CARRY (from `2026-09-06-coverage-completeness-gate`, deferred by operator decision at that chunk's P4; placement ruled at that wrap's P5 armed halt)
**Pulse's P-047 catalog is EIGHT categories, Conductor's corpus is still seven.** Verified at Pulse HEAD
`83d4060` (2026-09-06): `crates/security/src/scrubber.rs:3` reads "the 8 P-047 categories" with a live
`provider_key` arm (`:107-108`, cases at `:149-152` — `sk_live_…` / `sk-proj-…` / `ghp_…` / `AKIA…`).

Fold re-verify of the **Conductor-side** coordinates (2026-09-07) — all hold but one:
`crates/conductor-emit/src/pii.rs:32` "The seven" ✓ · 7-variant `PiiCategory` ✓ ·
`all() -> [PiiCategory; 7]` at `:53` ✓ · `PiiCorpus { values: [String; 7] }` at `:90` ✓ ·
7-element `seeded()` array literal at `:97-106` ✓ · `field_key()` match ✓ ·
the arity test `exposes_exactly_seven_distinct_categories` cited `:325` → measured **`:323`**.

Blast radius as stated by the CARRY (from the code-graph): 13 `all()` call sites — 10 in `pii.rs` tests,
2 in `conductor-emit/tests/pii_payload_corpus.rs` (`:101`, `:140`), 1 cross-crate in
`conductor-run/tests/pii_harvest.rs:185`. **P3 re-measures this** — a graph-derived count is a starting
point, not the basis.

**Causal-mechanism claim, marker text preserved:** "**Determinism constraint the original
CARRY never named (measured at `conductor-emit/src/pii.rs:97-106`): `PiiCorpus::seeded` draws all seven
values sequentially from ONE `ChaCha8Rng`, and `all()` is documented as discriminant order = storage
order = draw order — so the eighth variant must be APPENDED LAST; inserting it mid-order changes every
subsequent corpus value and breaks every committed expectation.**" The fold confirms the shape at the
named lines (one `ChaCha8Rng::seed_from_u64`, seven sequential `generate(...)` draws).

`v2-14` stays **VERIFIED** — it measured a state that genuinely held; a SUT change invalidates only
FUTURE legs. This is owned debt, never a re-opened chunk.
**Subject fit is acknowledged imperfect by the entry itself:** "this entry is otherwise indicatif +
opentelemetry-proto upkeep."

---

## Boundaries (what this chunk does NOT do)

- **No spec-master amendment authored by this chunk.** The doc-vs-artifact reconciliations (B1, B2) land
  as *drift surfaced for wrap's amend flow*, per the standing rule that phase and implement never amend
  the seven masters. The reconciliation's SHAPE (which sites, which corrected values) is
  this chunk's product; the amendment write is wrap's.
- **No arch [Module Boundaries] re-widening.** B4 lands the fix; narrowing the E1 qualifier is wrap's.
- **No StrykerJS.** Founder-declined 2026-09-05 (B5).
- **No `.claude/docs/*-summary.md` edits** for the crate count (B2's deliberate non-scope).
- **No live Pulse.** Nothing here drives the SUT; no scenario is run, no read-back is asserted.
- **No new gate.** B6 explicitly notes no rustdoc gate exists (P3 confirmed: no `cargo doc` / `RUSTDOCFLAGS`
  in `ci.yml` or either `agent-run` shell). `[open-question: P4]` Whether to ADD one is a P4 ruling, not a given — the CARRY asks only for the links to be fixed.
- **`v2-14` is not re-opened** (B7).

## Surfaces / contracts touched

| Surface | Nature of touch |
|---|---|
| `Cargo.toml` (workspace) | indicatif pin (A1); possibly the `opentelemetry-proto` entry (A2/B3) |
| `Cargo.lock` | moves; must stay committed + un-drifted; `cargo audit` + `cargo deny` green over the new lock (security.md) |
| `crates/conductor-emit/Cargo.toml` | `default-features = false` on `opentelemetry-proto` (A2) |
| `crates/conductor-run/Cargo.toml` | the dev-dep site of the same trim (B3) |
| `crates/conductor-verify/Cargo.toml` | `time` moves into `[dependencies]` (B4) |
| `crates/conductor-cli` presentation | indicatif API surface if the bump is a major series (A1) |
| `crates/conductor-run/src/*.rs` | doc-comment link re-paths only, no code (B6) |
| `crates/conductor-emit/src/pii.rs` + its callers | the eighth `PiiCategory`, appended last (B7) |
| `crates/conductor-tauri/ui/package.json` + `package-lock.json` | `knip` devDep + script (B5) |
| `deny.toml` | **WRITABLE — amended at P5 validation-1 (intent-incomplete).** The P1 draft said "read-only here; the count is reconciled in the DOC, not the toml", written before research measured that `number_prefix` has exactly one dependent (`indicatif 0.17.11`) and is absent from 0.18.6, so the bump strands RUSTSEC-2025-0119's `[advisories] ignore`. security-plan §Dependency Security forbids leaving an ignore whose subject a bump removed, and that plan's own 2026-06-23 amendment predicted this retirement verbatim — a master outranks this scope draft. The chunk removes the stranded entry (plan step 8) and reconciles the prose to the post-bump counts; it adds no entry and widens no suppression. |

## Fold re-verify summary (what changed against the annotations as written)

Coordinates re-verified against the artifacts per `promotion.md`; **six corrections**, all recorded above:
`architecture.md` tokio `:41`→`:42`, `:229`→`:253`; Tauri `:26`→`:27`, `:236`→`:260` **plus an unnamed
third arch site `:200`**; `obs-plan.md` `:645`→`:654`; `pii.rs` arity test `:325`→`:323`; `deny.toml`
licenses allow **8→9**. Four unnamed tokio sites outside arch and one unnamed Tauri site in
security-plan were also surfaced. Named coordinates only — mechanism truth is P3's scope premise closure.

---

## Scope premise closure (P3, 2026-09-07)

Every `[inferred]` bullet was re-read against research. **8 closed: 6 VERIFIED, 2 PREMISE-CORRECTED**
(the corrections are written in place above, at A1 and B3, tagged `[premise-corrected: …]`).
One item was **retagged `[open-question: P4]`** because it is a ruling to make, not a premise to verify.

| Premise | Outcome | Evidence |
|---|---|---|
| A1 — a 0.18 move carries an API delta the spinner code must be re-checked against | **PREMISE-CORRECTED** | All four used APIs signature-identical across 0.17.11 / 0.18.6 in both vendored sources; `suspend` (the freeze) unchanged |
| B1a — four tokio sites outside architecture.md | VERIFIED | Content sweep: `obs-plan.md:32`, `:39`, `test-plan.md:30`, `:434` |
| B1b — a fourth Tauri site at `security-plan.md:84` | VERIFIED | Measured; distillers added more floor sites in design-system (2) and layout-templates (3) |
| B2 — the workspace has 9 members | VERIFIED | `Cargo.toml` members list = 9 |
| B3 — trimming `conductor-emit` alone will not drop the dormant crates | **PREMISE-CORRECTED** | Two-site half VERIFIED; the trim's GOAL is unattainable at 0.32.0 — `trace`/`logs` gate the needed message modules and the SDK transforms through one flag |
| B4 — workspace feature unification hides the standalone-build red | VERIFIED | `cargo check -p conductor-verify --lib` reproduced RED at HEAD: tokio 1.52.3 `pub mod time` gated behind the `time` feature, E0433 |
| B6 — no gate enforces rustdoc warnings | VERIFIED | `cargo doc --no-deps -p conductor-run` exit 0 emitting `generated 7 warnings`; no `cargo doc` / `RUSTDOCFLAGS` in `ci.yml` or either shell |
| B7 — `seeded` draws sequentially from one `ChaCha8Rng`, so the 8th variant must append last | VERIFIED | `pii.rs:97-106`: one `ChaCha8Rng::seed_from_u64`, seven sequential `generate(...)` draws in discriminant order |
| Boundaries — the reconciliation SHAPE is this chunk's product, the amendment write is wrap's | VERIFIED | All seven distillers independently stated it against their own masters |

**Two findings research added that no annotation named** — both change what the plan may claim:

1. **B7's arity pin set is far wider than the CARRY's 13 call sites, and it includes pins no caller
   query can return.** Beside the 13 `all()` sites the graph confirms (`rows: 13`, matching the
   CARRY), an 8th category also moves: `pii.rs:53` (`-> [PiiCategory; 7]`), `:90`
   (`values: [String; 7]`), `:325` / `:329` (the arity test's two `assert_eq!`), `:444` (span
   attributes), `:486` (log records), `pii_payload_corpus.rs:155`, and
   `pii_harvest.rs:191` — `assert_eq!(needles.len(), 33, "4 corpora x 7 values + 5 stable affixes")`,
   a hard-coded DATA pin. **Read-the-hits exclusion:** `PiiCorpus::seeded(7)` at `pii.rs:334` / `:437`
   / `:482` / `:506` is a SEED equal to the arity by coincidence and must NOT be touched.
   **A live-capture pin blocks part of B7:** `pii_harvest.rs:96-97` are frozen literal log lines from a
   real Pulse leg asserting `rows_appended: 7` (one log record per category, cross-checked by `:150`),
   so an 8th category makes them stale — and re-capturing them needs a live leg this chunk's own
   boundary forbids. P4 must rule how B7 lands against that.

2. **The "7 vs 8" framing mis-states the gap, and there is a pre-existing category MISMAPPING.**
   Verified at Pulse HEAD `83d4060`: Pulse's eight are jwt · bearer · api_key · secret_kv ·
   provider_key · email · credit_card · ssn, and `provider_key` matches
   `[sr]k_(?:live|test)_[A-Za-z0-9]{16,}` among other shapes (`crates/security/src/scrubber.rs:99`)
   while `api_key` requires the key=value form (`:77`). Conductor's `ApiKey` generates
   `format!("sk_live_{}", charset_run(rng, ALNUM, 24))` (`pii.rs:245`) — so **Conductor's `ApiKey`
   already emits a value Pulse classifies as `provider_key`, not `api_key`.** The genuinely absent
   category is `provider_key`'s other shapes (`sk-proj-` / `ghp_` / `AKIA` / `xox…` / `AIza…`).
   `scenarios/pii-scrub.toml:27` already records the older half of this ("the bare `sk_live_`
   provider-key shape matches no catalog pattern, recorded as SUT-visit intake") — that recall gap now
   appears CLOSED SUT-side, which is itself only confirmable on a live leg.
