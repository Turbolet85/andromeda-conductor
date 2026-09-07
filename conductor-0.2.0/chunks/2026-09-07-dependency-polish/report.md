# Report — 2026-09-07-dependency-polish

**Chunk:** Dependency polish — the indicatif bump that must preserve the stop-in-place hold spinner, plus the opentelemetry-proto default-features trim across both dep sites; carries seven folded CARRYs
**Date:** 2026-09-07
**Commits:** none since `last_wrap` (2026-09-07T08:55:21Z) other than the prior chunk's `c6247d8`; this chunk's work is uncommitted at report time

## Changes (structured — detectors read this)

- **Files:** 7 semantic + 6 formatting-only + 2 web-plane descriptors.
  **Semantic:** `Cargo.toml` · `Cargo.lock` · `deny.toml` · `crates/conductor-emit/Cargo.toml` ·
  `crates/conductor-run/Cargo.toml` · `crates/conductor-verify/Cargo.toml` ·
  `crates/conductor-cli/src/pause.rs` · `crates/conductor-emit/src/pii.rs` ·
  `crates/conductor-emit/tests/pii_payload_corpus.rs` · `crates/conductor-run/tests/pii_harvest.rs` ·
  `crates/conductor-run/src/{lib,execute,preconditions}.rs` (doc comments only) ·
  `crates/conductor-tauri/ui/{package.json,package-lock.json}`.
  **Formatting-only:** `crates/conductor-run/src/{canary,dispatch,drive,envelope,lifecycle,testkit}.rs`.
  *Basis:* each HEAD copy piped through `rustfmt --config-path rustfmt.toml` compared to the working copy —
  residual **0 lines** in those six; 9 lines in `execute.rs`, 4 in `preconditions.rs`, 38 in `lib.rs` (the
  re-pathed doc links), i.e. exactly the semantic edits. Measured twice (builder + operator).
- **Symbols / APIs:** `PiiCategory::ProviderKey` (new enum variant, appended LAST);
  `PiiCategory::all()` return type `[PiiCategory; 7] → [PiiCategory; 8]`;
  `PiiCorpus.values` `[String; 7] → [String; 8]`; `PiiCategory::field_key()` gains
  `ProviderKey => "service.provider_key"`; `generate()` gains a `ProviderKey` arm.
  **Remaining-caller fact:** `PiiCategory::all()` keeps its 13 call sites (code-graph rust plane,
  `rows: 13`, trace at `.andromeda/runs/2026-09-07T10-43-05-phase/tree-query-2026-09-07-dependency-polish.json`);
  none needed editing — all are arity-agnostic iterations.
  New test `pause::tests::suspend_freezes_the_bar_in_place_and_returns_the_closure_value`.
  Test renamed `exposes_exactly_seven_distinct_categories → exposes_exactly_eight_distinct_categories`.
  No IPC method, endpoint, port, socket or env var added or changed.
- **Crates / modules:** none added, none removed. Workspace roster unchanged at **9** members.
- **Dependencies:** `indicatif` **0.17 → 0.18** (workspace pin; resolves 0.18.6).
  `opentelemetry-proto` gains `default-features = false` at the **workspace** entry (`Cargo.toml:41`).
  `crates/conductor-verify` `[dependencies]` tokio gains the **`time`** feature.
  `crates/conductor-run` `[dev-dependencies]` `opentelemetry-proto` gains explicit
  `features = ["gen-tonic","trace","logs"]`.
  Web plane: `knip` added as a **devDependency** (`crates/conductor-tauri/ui`).
  **Lock delta 564 → 562 packages:** removed `number_prefix 0.4.0` + `const-hex 1.19.1`; added
  `unit-prefix 0.5.2`. `opentelemetry` and `opentelemetry_sdk` **remain** (see Spec claims disproved).
  *Basis:* `cargo update -p indicatif --precise 0.18.6`; `grep -c '^\[\[package\]\]'` on both revisions.
- **Schema / config:** no migration, no config key, no violation schema, no scrub/redaction shape changed.
  `deny.toml` `[advisories] ignore` loses `RUSTSEC-2025-0119` (its subject left the tree).
- **Spec-master edits:** none by this chunk (all reconciliations are Expected amendments below).
- **Counts / qualifiers moved:**
  - PII categories **7 → 8** — stated in `conductor-emit/src/pii.rs` module doc + `PiiCategory` doc.
    *Basis:* the enum + `all()` arity.
  - `deny.toml` `[advisories] ignore` entries **17 → 16** (licenses allow unchanged at **9**).
    *Basis:* `awk '/^ignore = \[/,/^\]/' deny.toml | grep -cE '^\s*"RUSTSEC-'`, against the same over
    `git show HEAD:deny.toml`. Stated in security-plan §Dependency Security Accepted-exceptions.
  - `cargo audit` allowed warnings **18 → 17**. *Basis:* `cargo audit` tail, both revisions.
  - `Cargo.lock` packages **564 → 562** (basis above).
  - Workspace crates: **unchanged at 9** — the stale "8 workspace crates" prose is pre-existing
    (Expected amendments below), not moved by this chunk.
- **Dev-tool versions:** none — no external CLI tool installed or upgraded. (`knip` is a package-scoped
  devDependency, not a host CLI.)
- **Harness / gate surface:** one new npm script `knip` in `crates/conductor-tauri/ui/package.json`
  (report-only, **not** wired into CI or either `agent-run` shell — the chunk adds no gate). The five a11y
  script names are present and unrenamed. No `agent-run` verb, xtask verb, CI step, or status/verdict shape
  changed.
- **Cross-project / external claims:** Pulse's `provider_key` regex read at `andromeda-pulse`
  `crates/security/src/scrubber.rs:99` @ HEAD `83d4060` — `[sr]k_(?:live|test)_[A-Za-z0-9]{16,}` among five
  alternatives; its separate `api_key` pattern (`:77`) requires the key=value form. Basis: direct read of the
  SUT working tree on this host.
- **Reverted / negative API facts:** the member-level `default-features = false` (plan steps 4/5) was written
  and **reverted** — cargo rejects a member disabling defaults on an inherited workspace dep
  (`error: default-features = false cannot override workspace's default-features`). The flag landed at the
  workspace entry instead, which covers both dep sites.
- **Insufficient fixes (written, kept, not the remedy):** the eighth `PiiCategory` ships, is generated,
  and is asserted — but is **unreachable from any scenario**. `conductor-core`'s `PiiCategorySpec` still has
  7 variants and `conductor-run`'s `wire_category` (`dispatch.rs:282`) maps it 1:1 into the now-8-variant emit
  enum; mapping INTO a widened target is not a match-exhaustiveness error, so the compiler stayed silent and
  every gate is green. What it resolved: the corpus, the arity pins, and Pulse-shape coverage. Remainder
  owner: routed at P5.
- **Spec claims disproved by measurement:**
  1. **obs-plan §3 OTel SDK init** records the `opentelemetry-proto` `default-features = false` trim as the
     way to shed the dormant `opentelemetry` / `opentelemetry_sdk` footprint. **Measured false at 0.32.0:**
     the crate's `trace` and `logs` features gate BOTH the generated message modules `conductor-emit` imports
     (`src/proto.rs`) AND the SDK transform modules (`src/transform/mod.rs`), and its `Cargo.toml` defines
     `trace = [opentelemetry/trace, opentelemetry_sdk/trace]`. One flag, both jobs — so no trim preserving the
     raw-OTLP surface can drop them. Confirmed post-trim: both crates are still in `Cargo.lock`. What the trim
     DOES drop: `metrics`, `zpages`, `with-serde`, `internal-logs` — of which only `const-hex` actually left
     the tree (`base64` and `serde` have other dependents). *Basis:* vendored
     `opentelemetry-proto-0.32.0/src/{lib,proto}.rs` + `src/transform/mod.rs` + `cargo info`; lock diff.
  2. **architecture §Established Decisions [Module Boundaries]** carries an E1 qualifier stating the standalone
     per-seam build is "an intent, not a guarantee", recording `cargo check -p conductor-verify --lib` RED.
     **Retired by measurement:** that gate is now exit 0, and the full per-member sweep is clean on all nine
     (7× `--lib`, 2× `--bins`, each exit 0) — measured twice, builder and operator. `--all-targets` was
     deliberately not used (it re-unifies dev-deps, the exact masking the sweep hunts).
  3. **`profiles` is not in the trim's drop list** — `full = [gen-tonic, trace, logs, metrics, zpages,
     with-serde, internal-logs]` excludes it and `profiles = []`, so it was never enabled. (Corrects this
     chunk's own plan text; no master states it.)
- **Expected amendments (from plan):**
  - `architecture.md §Stack` — indicatif `0.17` → **0.18**. CARRIED; fact in Dependencies.
    *Search:* `grep -nE 'indicatif[^0-9]{0,3}0\.1[0-9]'` over the seven masters → **1 hit** in architecture
    (`:34`), 1 in design-system (`:284`), 1 in layout-templates (`:170`).
  - `architecture.md §Stack · §Established Decisions [Language / Runtime] · §Inherited Defaults` —
    `tokio 1.48.x` → the resolved **1.52.3**. CARRIED as a pre-existing gap; this chunk changed no tokio
    version. *Search:* `grep -cE 'tokio 1\.48'` → architecture **3**, obs-plan **2**, test-plan **2**.
  - `architecture.md §Stack · §Deployment model · §Inherited Defaults` — Tauri `v2.10.x / latest 2.10.1` →
    **2.11.3**; the `≥ 2.10.3` FLOOR statements untouched. CARRIED as pre-existing.
    *Search:* `grep -nE 'v2\.10\.x|latest 2\.10\.1'` → architecture **4 hits** (`:27`, `:58`, `:200`, `:260`),
    security-plan **1** (`:84`).
  - `security-plan.md §Dependency Security` — the Accepted-exceptions paragraph reconciled to `deny.toml`'s
    **post-bump** set (**16** ignores + **9** allows); its `number_prefix` clause no longer describes the tree.
    CARRIED; fact in Counts / qualifiers moved. **The master neither forbade nor predicted the retirement** —
    `security-plan-amendments.md:33` carries a Follow-up ("a future `indicatif` 0.18 bump may drop
    `number_prefix`"), which is a pointer, not an obligation; the retirement's authority is `deny.toml:6-7`'s
    own requirement that every accepted advisory carry a justifying comment, which this entry's
    `via indicatif 0.17` comment no longer satisfies. *Search:* `grep -n 'number_prefix'` security-plan →
    **1 hit** (`:181`).
  - `obs-plan.md §1 · §12` — "8 workspace crates" → **9**; `§1` instrumentation table `tokio 1.48.x` →
    resolved. CARRIED as pre-existing. *Search:* `grep -cE '8 workspace crates'` → obs-plan **2**,
    test-plan **1**.
  - `obs-plan.md §3 OTel SDK init` — the Transitive note's scope corrected to **TWO** dep sites, and its
    `default-features` follow-up closed with the measured result. CARRIED; fact in Spec claims disproved #1.
  - `test-plan.md §1` — "8 workspace crates" → **9**; `§4`/`§12` `tokio 1.48.x` → resolved. CARRIED
    (search above).
  - `design-system.md §Surface: cli / Toolkit-Framework` — `indicatif 0.18` **becomes true** at this chunk
    (no edit needed for that token); `inquire 0.7` → resolved **0.9.4** is a separate stale value.
    CARRIED. *Search:* `grep -nE 'inquire[^0-9]{0,3}0\.[0-9]'` → architecture `:34` (states 0.9, correct),
    design-system `:284` (states 0.7, stale), layout-templates `:170` (states 0.7, stale).
  - `layout-templates.md §Surface: cli — Tooling context` — `indicatif 0.17` → **0.18**; `inquire 0.7`
    → 0.9.4. CARRIED (searches above).
  - `architecture.md §Established Decisions [Module Boundaries]` — the E1 qualifier narrowed. CARRIED and now
    **UNCONDITIONAL**: the plan made it conditional on a clean sweep, and the sweep came back clean on all
    nine. Fact in Spec claims disproved #2.
- **Coverage of new surfaces:**
  - `PiiCategory::ProviderKey` (corpus value, synthetic) → validation n/a (not external input) ·
    instrumentation n/a (no new span/log — the corpus rides existing emit builders) · PII: the value IS
    synthetic PII by construction, RNG-derived body with a committed affix (`ghp_` + 36 alnum), never
    host-derived · tests unit ✓ (`each_category_value_is_structurally_valid` asserts prefix, length 40 and
    charset; `exposes_exactly_eight_distinct_categories` pins arity) + integ ✓
    (`pii_payload_corpus` 8-record assertions) · a11y n/a · tokens n/a.
  - `knip` npm script (`crates/conductor-tauri/ui`) → validation n/a · instrumentation n/a · PII n/a ·
    tests: **report-only, no gate** (deliberate — the chunk adds no gate) · a11y n/a · tokens n/a.
  - `pause::tests::suspend_freezes_the_bar_in_place_...` (test surface) → tests unit ✓ · all others n/a.

## Deviations from intent

1. **`default-features = false` landed at the WORKSPACE entry (`Cargo.toml:41`), not the member sites the
   plan named (steps 4/5).** *Justification:* cargo rejects a member disabling defaults on an inherited
   workspace dependency — `error: default-features = false cannot override workspace's default-features`,
   measured at implement. The workspace entry is the only legal site and covers both dep sites at once;
   members keep their own feature lists. `Cargo.toml` was already a listed touchpoint, so the fix stayed
   in scope; an inline comment records why the flag lives there.
2. **The rustfmt PostToolUse hook on `crates/conductor-run/src/lib.rs` reflowed the crate's other 8 modules.**
   *Justification:* FORMATTING-ONLY, not corruption — editing a crate root makes the hook recurse the module
   tree. Basis stated in Changes → Files (residual 0 lines in six files; 9/4/38 in the three I edited).
   **Second fire of this class**, after the coverage-gate chunk. **Boundary #5's line metrics for
   `conductor-run` move on formatting alone** — a reader comparing raw line deltas across boundaries must
   discount these six files.
3. **`knip.json` entry-point config deferred.** *Justification:* step 10 scopes the work to "a devDependency
   plus a `knip` script … Run it once and disposition every finding in writing without deleting anything",
   and the touchpoint list carries `New files: None`. Choosing which entry points to declare is a design
   decision with real content, so it was surfaced rather than improvised. Consequence recorded below.
4. **`pii_harvest.rs` needle count → 38**, taking step 13's "or 38 with a new affix" branch. *Justification:*
   `ghp_` is a stable generator affix and that test's stated purpose is catching a leak by affix even when a
   recomputation drifts.
5. **The freeze test drives `ProgressBar::suspend` directly rather than `PromptResolver::resolve`.**
   *Justification:* `testing.md` bans spawning a real `inquire` confirm (no pty under the runner; it would
   hang against the zero-retry bar). The test pins the three properties the design constraint names.

## Decisions & corrections

- **Operator correction at the P5 plan review (five fixes).** The load-bearing one: I asserted that
  security-plan §Dependency Security *forbids leaving an advisory ignore whose subject a bump removed*, and
  cited the master for it in four places. **No such rule exists** — the master's only four `ignore` lines are
  `:119` (SR speech log), `:176` (advisory-DATABASE fault remedy), `:181` (the Accepted-exceptions record) and
  `:211` (secret-scanning gate). The sentence existed only in this run's security extract
  (`runs/2026-09-07T10-43-05-phase/security.md:23`), where the distiller anchored its own inference to the
  master, and I propagated it without grepping. Re-based on `deny.toml`'s own text. A retraction record was
  appended to the friction log.
- **Two operator sub-premises corrected rather than complied with.** (a) The 2026-06-23 sidecar DOES carry a
  Follow-up anticipating the retirement (`security-plan-amendments.md:33`), so the extract quoted that part
  accurately — what it invented was the obligation, not the anticipation. (b) `v2-32`'s `acceptance` was
  populated (254 chars), not null; `ref` was the null field, and the matrix contract assigns `ref` to
  /implement — so phase wrote `chunk` only and prescribed the ref string in the plan.
- **Sweep-pattern corrections, four this session** — each a non-empty grep satisfied by something other than
  the target: a bracketed doc-link pattern that could not match rustdoc's backtick form; route separators read
  as entries; a 1.6 MB `node_modules` match; and the `deny.toml` id-count probe reading the id inside the
  *retirement comment I had just written*. A fifth at report time: `indicatif 0.1[78]` missed the backticked
  `` `indicatif` 0.18 `` in two masters. All resolved by reading the hits, never by refining blindly.
- **`--all-targets` is banned in the per-member sweep** — it re-unifies dev-dependencies, which is the exact
  masking the sweep exists to find.

## Outcome

**Acceptance criteria — re-asserted against the diff:**

| Criterion | Verdict against the diff |
|---|---|
| `v2-32` (matrix) | **MET.** indicatif resolves 0.18.6; spinner freeze asserted by a test; `opentelemetry-proto` declared `default-features = false` (workspace entry, both members build); lock un-drifted (`cargo metadata --locked` exit 0); audit + deny exit 0 |
| (arch) `conductor-verify --lib` green + nine-member sweep | **MET** — exit 0; sweep clean on all nine, per-target |
| (arch) audit/deny green over the moved lock, porcelain first | **MET** — porcelain clean, then audit 0 / deny 0 |
| (arch) `conductor-emit` retains `["gen-tonic","trace","logs"]` | **MET** — unchanged; `metrics` still absent, matching the code (arch §Stack registers `metrics` — surfaced, not silently absorbed) |
| (design) spinner stops in place, not hidden/cleared/animated | **MET** — new unit test |
| (layouts) line-oriented, pipe-safe, headless never blocks | **MET** — spinner still `ProgressDrawTarget`-gated on `stderr().is_terminal()`; harness legs completed unattended |
| (layouts) no new label / lamp / `ReportState` / ANSI entry / token | **MET** — diff introduces none |
| (tests) both runners green on every touched crate | **MET** — nextest 902/902; `cargo test -p` emit 77 / run 195 / verify 112 / cli 52, all 0 failed |
| (tests) no golden changed; same seed ⇒ same shape | **MET** — the three `dispatch_wire__*` snapshots carry no PII corpus values and are unmodified in the diff |
| (obs) `conductor-emit` compiles + suite green; no OTel SDK initialized | **MET** |
| (obs) `conductor-cli` still declares no `tracing` dependency | **MET** — `crates/conductor-cli/Cargo.toml` unchanged in the diff |
| (security) no full corpus VALUE committed; affix convention followed | **MET** — `git grep` returns 11 hits, every one an affix constant, an assertion on one, or prose |
| (a11y) harness devDeps + five script names intact | **MET** — `package.json` diff adds `knip` only |
| (a11y) every knip finding dispositioned in writing, nothing deleted | **MET** — `evidence/knip-first-report.md`, 20/20 dispositioned |
| (tests) rustdoc `-D warnings` exit 0 | **MET** — 7 warnings → 0 |

**Gates green (commands run):** `cargo check -p conductor-verify --lib` · the per-member sweep (7× `--lib`,
2× `--bins`) · `cargo nextest run --workspace --profile ci` · `cargo test -p` ×4 ·
`cargo clippy --workspace --all-targets -- -D warnings` · `cargo test --doc --workspace` ·
`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p conductor-run` · advisory-db porcelain probe ·
`cargo audit` · `cargo deny check advisories bans licenses sources` · `cargo metadata --locked` ·
`npm audit --omit=dev` · `npm run typecheck` · `typecheck:e2e` · `npm run build` · the secret-hygiene
`git grep` · `bash scripts/agent-run.sh status`.

**`npm run knip` is REPORT-ONLY.** It exits **1** by knip's own convention ("findings present"); the plan's
Test Command states no exit expectation. The asserted signal is `evidence/knip-first-report.md`. Exit 1 is
**expected**, not a red and not a silent skip.

**Consequence for the next code audit:** boundary #5's A5 `dead-code-web` column must read
**"tool present, series unusable (100 % FP)"** — never a usable series — until a `knip.json` entry-point
config lands. All 20 findings are false positives of one class: WebdriverIO discovers specs through
`wdio.conf.ts` and loads devDependencies through its own plugin resolution, neither of which is an import
edge; four of the five flagged devDeps are a11y-plan §3-mandated by name and pin, and §11 forbids acting on
such a report.

**Smoke:** fired (binary/entry-point touchpoint `conductor-cli/src/pause.rs` + the plan's Test Commands name
`agent-run.sh`). `bash scripts/agent-run.sh run` exit 0 with 902/902 in the printed verdict; the bin was
additionally booted (`conductor --help` renders its seven verbs, exit 0). The UI condition did not apply — the
modify-set carries `package.json`/`package-lock.json` but no `.tsx`/`.ts`/`.css`, and `ui/dist` is unchanged.

**Outcome basis:** /implement's P4 report, PLUS an operator wrap directive (2026-09-07) that (a) fixed the
light gate's treatment of `npm run knip` as report-only, (b) directed the `knip.json` CARRY and the
boundary-#5 note, (c) fixed the deviation set carried here, and (d) made the E1 qualifier's retirement
unconditional on the twice-measured clean sweep and constrained the security-plan amendment's wording.
Post-implement artifacts read: `evidence/knip-first-report.md`, the operator's own sweep re-measurement.

**Process hygiene:** implement P4 recorded `none started` — every command was a bounded cargo/npm/git
invocation that exited. Re-measured at this wrap: probe live (269 processes visible); `pulse-app`,
`conductor*`, `msedgedriver`, `tauri-driver`, `nvda` all absent; no listener on `:4317`/`:4444`/`:4445`.
The chunk is hermetic — nothing to launch or stop.
