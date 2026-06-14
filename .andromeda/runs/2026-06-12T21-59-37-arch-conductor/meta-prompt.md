## Output Protocol

Rules you MUST follow:

1. **Output = patches + changelog only.** Emit targeted patches (old text → new text) plus a changelog. Do NOT reproduce the full document.
2. **Patch format.** Each patch is:
   - `### Patch N: <short description>`
   - `**Old:**` followed by the *exact* current text from the document (verbatim, enough surrounding text to be unambiguous and uniquely locatable).
   - `**New:**` followed by the replacement text.
3. **Changelog.** After the patches, one line per change: `[Iteration N] [substantive|cosmetic] <description>`. `substantive` = changes a fact, resolves an ambiguity, adds/removes a constraint, fixes a real inconsistency. `cosmetic` = wording/formatting only.
4. **Patch budget (spend it in priority order).** You have a hard budget of 8 patches. Spend them strictly by the bucket order defined in the Analysis Protocol: **bucket 1 (downstream-blocking, `[priority: high]`) first**; bucket 2 (implementation-misleading, `[priority: high]`) only if budget remains; bucket 3 (signal-diluting, `[priority: medium]`) only if the fix is one line AND undeferrable. Never spend a patch on a bucket-3 cosmetic while a bucket-1 anchor is still unaddressed. Within a bucket, patch the dimension with the strongest (quoted, verified) anchor first.
5. **PROHIBITED:**
   - Full-document reproduction.
   - Restructuring sections without a stated reason.
   - Cosmetic changes labeled `substantive` (or vice versa).
   - Inventing technologies, versions, P-IDs, ports, env vars, MCP tool names, or crate names not grounded in the document.
   - Adding HTTP/REST/GraphQL surfaces, Docker/K8s, an ORM, a message broker, or native OS toasts — every one of these is an explicit non-goal or "N/A" in this document; "completing" them is a regression.
   - Naming a concrete logger, a non-Cargo test/assertion/property framework, an auth library, a frontend framework, a CSS tool, or a component library — these are specialist territory (see the Analysis Protocol out-of-scope step and Dimension 7). Patch only the *boundary requirement*, never the *pick*.
6. **If no issues found in a dimension:** write "No patches" for it. If the whole document needs nothing, output "No patches" and a single `[Iteration N] [cosmetic] reviewed, no changes` changelog line.

## Analysis Protocol

Do NOT scan-then-patch. Reason in these five explicit steps before emitting any patch. The document is ~174 non-blank lines and small enough to hold in full, but every cross-reference below must be confirmed against the actual section text, not memory.

**Step 1 — Read the whole document once, noting nothing.** Build a mental model of the 11 sections (Design Philosophy, Stack and Technologies, Established Decisions, Conventions, Standard Contracts, Occupied Resources, Infrastructure Patterns, Cross-cutting Patterns, Project Intent, Inherited Defaults, Existing Scopes) before forming any judgement.

**Step 2 — Cross-reference walk (concrete, enumerated — do each one).** Abstract "check internal consistency" is forbidden; walk these specific Conductor cross-references and flag any disagreement as a *substantive* bug:

- **Version agreement across all sites.** Every pinned tool must read identically in **Stack and Technologies ↔ Established Decisions ↔ Conventions ↔ Inherited Defaults ↔ Infrastructure Patterns**. Walk each of: `tokio 1.48.x current_thread`, `opentelemetry-proto 0.32.0`, `tonic 0.14.6` (+ `tonic-prost 0.14.6` / `prost 0.14`), `rmcp 1.7.0`, `rusqlite 0.38.0` (+ `libsqlite3-sys 0.38.0`, bundled SQLite 3.51.1), `garde 0.23.0`, `thiserror 2.0.18`, `anyhow 1.0.102`, `Tauri 2 (bundler 2.10.x / 2.10.1)`, Rust 2024 / cargo 1.85 / MSRV 1.88.0. A tool pinned at four sites must agree at all four — flag a bare `tonic 0.14` against a `0.14.6`, or any reappearance of the brief's stale `rusqlite 0.31` / `SQLite ≥3.38` un-superseded.
- **Crate / member names.** Walk **Stack/Design Philosophy seam list ↔ Conventions ("crates named `conductor-<seam>`") ↔ Infrastructure Patterns directory tree ↔ Occupied Resources ("Crate names (workspace members)") ↔ Inherited Defaults**. Every crate in the tree must be a named member in prose and vice versa. In particular reconcile `conductor-core`: it is in the directory tree but check whether the Established Decisions module-boundary seam list and Occupied Resources name it as a member or only allude to "a runtime-agnostic core crate."
- **Env vars ↔ consuming surface.** Walk **Occupied Resources env-var list ↔ Cross-cutting Patterns (purpose) ↔ the artifact/dir each handle points at**. Every `CONDUCTOR_*` var (`CONDUCTOR_RUNS_DIR`→`runs/`, `CONDUCTOR_SCENARIOS_DIR`→`scenarios/`, `CONDUCTOR_CONTRACT_MANIFEST`→`contracts/`, `CONDUCTOR_SEED`→the seed) must resolve to a defined resource; `ANDROMEDA_PULSE_MCP_ENABLED` must read as Pulse-side / asserted-not-set-by-Conductor everywhere. Flag any var with no consuming surface, or any port/file/MCP-tool/Tauri-command referenced in one section but absent from Occupied Resources.
- **The Verdict/ReportState wall across its 4 sites.** Confirm `enum Verdict { Pass, Fail, CalibrationRegion }` and `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` read identically in **(a)** the Design Philosophy "Outcomes are values" bullet, **(b)** the Established Decisions error-handling entry, **(c)** the Conventions error-handling paragraph, and **(d)** the Standard Contracts run-report envelope footer. Flag any site that adds or drops a state, and flag any state→trigger mapping ("MCP tool absent → Blocked", "transport refused → harness error", "wrong incident → Fail") that conflicts between sites — or any enum state that is declared but never given a trigger anywhere.
- **Artifact paths emitted ↔ committed.** Walk **Standard Contracts (where `run_id` journals / reports / `runs.db` are emitted) ↔ Infrastructure Patterns directory tree (`runs/`, `<run_id>.jsonl`, `report.md`, `coverage-matrix.md`, `contracts/`)**. Flag any artifact emitted in one place but with no home in the tree, or a format shown by example in one place that disagrees with another (e.g. `run_id` hyphenated timestamp vs `checked_at` colon timestamp).

**Step 3 — Check each dimension with its anchor in mind.** For each Analysis Dimension below, hold the verbatim anchor quote against the live section text. If the anchored issue is already resolved in the current draft, write "No patches" for that dimension rather than inventing a new problem.

**Step 4 — Out-of-scope discipline (apply before writing any patch).** If a finding would require writing **test cases** (tests' domain), **OTel span / metric / trace schemas** (obs' domain — note OpenTelemetry Semantic Conventions are *named* here as the shared vocabulary, which is in-scope; defining new attribute schemas is not), **`aria-` attribute names or specific WCAG conformance claims** (a11y's domain), **design tokens / component patterns / typography / color picks** (design's domain), or **threat-model / security-tier / compliance content** (security's domain) — do NOT patch. Likewise do NOT name a concrete **logger, non-Cargo test/assertion/property framework, auth library, frontend framework, CSS tool, or component library**. Instead, verify the document exposes the *boundary requirement* (the "what must hold" — e.g. "the JSONL journal is the agent-parseable ground truth," "the GUI must NOT emit native OS toasts," "the webview is bundled with no framework named"). Patch only if that boundary itself is unstated; never fill in the specialist's pick.

**Step 5 — Prioritize by downstream impact.** Sort candidate patches into the three buckets named in the Output Protocol budget rule. A wrong/missing cross-reference that a downstream `/andromeda-scope-arch` Phase 0 or `/implement` cannot reconcile from a single read is bucket 1. A pick that would mislead implementation (under/over-engineering, an unbounded validated invariant) is bucket 2. Generic wording, non-load-bearing cross-refs, and forward-look "N/A" dilution are bucket 3. Emit patches top-down through the buckets until the budget is exhausted.

## Analysis Dimensions

### 1. Cross-reference Integrity [priority: high]

Walk every technology, crate, env var, port, and artifact across the sections that should co-name it. Does each Stack row reappear with operating context in Conventions / Established Decisions / Cross-cutting Patterns rather than sitting orphaned in the table? Does every crate in the directory tree appear as a named workspace member in prose and vice versa? Does every `CONDUCTOR_*` env var resolve to a directory/artifact the document defines, and does `ANDROMEDA_PULSE_MCP_ENABLED` read consistently as Pulse-side everywhere?

**Anchor example:** Infrastructure Patterns, directory tree (vs Established Decisions module-boundary entry and Occupied Resources crate list)

> "│  ├─ conductor-core/         # runtime-agnostic engine library (usable outside Tauri)"

**Issue:** `conductor-core` is a first-class workspace member in the tree, but the Established Decisions [Module Boundaries] entry names the seams as "`conductor-timeline`/`conductor-emit`/`conductor-faults`/`conductor-verify`/`conductor-report` (+ `conductor-cli`, Tauri bin)" and Occupied Resources lists members as "`conductor-timeline`, `conductor-emit`, `conductor-faults`, `conductor-verify`, `conductor-report`, `conductor-cli`, plus the Tauri bin and a runtime-agnostic core crate" — the *name* `conductor-core` appears only in the tree (searched Established Decisions and Occupied Resources for the literal `conductor-core` — zero matches; both sections refer to it only descriptively as "a runtime-agnostic core crate" / "core"). The crate that the [Workspace / Core Structure] decision says "makes headless-drivable real" is the one member never named where members are enumerated.

**Why this matters:** `/andromeda-scope-arch` Phase 0 builds the crate dependency graph from the enumerated member list, not the ASCII tree; `/implement` writes `cargo new -p` and `[workspace] members` from the same list. A core crate that is in the tree but absent from both authoritative member lists is the seam most likely to be dropped from the workspace manifest or mis-wired — and it is the one every other crate depends on.

**Adversarial:** If `conductor-core` is committed in the tree but never listed as a workspace member, what happens when `/implement` generates `Cargo.toml` from the Occupied Resources member list and `conductor-timeline` tries to `use conductor_core::...` against a crate that was never added to `[workspace.members]`?

### 2. Internal Consistency [priority: high]

Does the Verdict/ReportState wall read byte-identically at all four sites (Design Philosophy bullet, Established Decisions error-handling entry, Conventions error-handling paragraph, Standard Contracts envelope footer)? Do all version pins agree across Stack / Established Decisions / Conventions / Inherited Defaults / Infrastructure Patterns? Do timestamp formats shown by example agree with each other?

**Anchor example:** Standard Contracts, readiness-gate JSON vs run-report JSON

> "  "checked_at": "2026-06-12T21:59:37Z""

**Issue:** `checked_at` uses a colon-delimited RFC-3339 time (`2026-06-12T21:59:37Z`), while the run-report envelope eleven lines later opens with `"run_id": "2026-06-12T21-59-37-..."` — a hyphen-delimited timestamp — and the directory tree names per-run artifacts `<run_id>.jsonl`. Two timestamp encodings (`HH:MM:SSZ` vs `HH-MM-SS`) appear in the same Standard Contracts section with no rule reconciling them (searched the document for any statement of the `run_id` timestamp format — none present; it is shown only by the two examples and the `<run_id>` tree placeholder). The same instant `21:59:37` is rendered both ways.

**Why this matters:** `run_id` is the primary key of `runs.db`, the journal filename stem, and the report filename stem. If implementation infers the format from `checked_at` (colon) it produces filenames illegal on Windows/NTFS — the stated dev host is Windows — and a `run_id` that disagrees with the `<run_id>.jsonl` convention, silently splitting the cross-run index from the on-disk journals it is supposed to index.

**Adversarial:** If `run_id` is "shown by example" as hyphenated but the only *stated* timestamp format in the section is the colon form of `checked_at`, which file write fails first when `/implement` opens `runs/<run_id>.jsonl` with a colon in the name on the Windows dev host?

### 3. Contract Completeness (MCP-preflight & run-report envelopes — NOT HTTP) [priority: high]

This document defines no HTTP API; the binding contracts are the MCP `initialize` preflight readiness gate, the OTLP egress liveness equivalent, and the run-report artifact envelope. Judge *those* for completeness — do NOT request `/health`, `/ready`, pagination, or REST shapes. Is every field's value domain closed? Is the version-mismatch path distinguished from the tool-absent path? Does the envelope show how a `blocked` row populates the fields it cannot measure?

**Anchor example:** Standard Contracts, run-report envelope

> "  "latency_ms": 1840,
>   "slo_tier": "<5s","

**Issue:** The envelope shows only a `state: "Pass"` row. Line 91 asserts that on `ready: false` "every dependent auto scenario is emitted into the report with `state: "blocked"`," yet the envelope never shows a `blocked` row, so the contract is silent on what `latency_ms`, `read_back_observed_at`, `journal_emitted_at`, and `verdict` hold when the scenario was never measured (searched the document for any `blocked`-state field-population rule or `null` — `null` appears nowhere; the only `blocked` field guidance is "the named precondition"). Separately, `slo_tier` is shown as the string `"<5s"` but is never declared a closed enum over `<5s`/`<20s`/`<90s`, and `latency_ms` is never typed as integer milliseconds.

**Why this matters:** Both `verdict` and `state` consumers (the Markdown reporter, the `runs.db` writer, the Tauri report view, and any downstream obs scope parsing the JSONL ground truth) must deserialize a `blocked` row. If `latency_ms` is non-nullable in the schema but unmeasurable for a blocked scenario, the writer either crashes or fabricates a `0`/`1840`-shaped number that pollutes seed-to-seed latency queries and the P-036 recurrence check with phantom data.

**Adversarial:** If `ready: false` forces forty scenarios to `state: "blocked"` and the envelope's `latency_ms` field has no nullable/omitted contract, what value does the `runs.db` row store for a scenario whose `read_back_observed_at` never happened — and what does a cross-run latency percentile query return once those rows are mixed in?

### 4. Downstream Readiness (bidirectional) [priority: high]

**Defect side:** Can `/andromeda-scope-arch` Phase 0 fill its 11-section template for a new scope with no questions? Can the crate build order (`conductor-core` → seams → `conductor-cli`/`conductor-tauri`) and the headless-is-the-release-gate ordering be derived from the tree and Design Philosophy without asking?

**Completeness side:** For each specialist, locate the fact; if absent OR present only implicitly (scattered, not aggregated), flag a patch that elevates it to a specialist-facing surface — WITHOUT naming the specialist's pick (see Step 4 / Dimension 7):
- **tests:** the headless `scripts/agent-run.sh` source-of-truth path, per-seam `cargo build -p` / `test -p`, the synthetic-injection gate (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`), golden tests, and the MCP-preflight readiness shape are the test substrate — flag if these read as a stated testability contract or only as scattered mentions.
- **obs:** the JSONL emission journal (agent-parseable ground truth), journal-relative SLO math, the Tauri 2 `Channel` live-counter stream, and the run-report envelope are the observability substrate — flag if the structured-output/journal format harness polling depends on is under-elevated. (Do NOT name a logging library.)
- **security:** the loopback posture (`127.0.0.1:4317` only, `:4318` unused, no inbound listener of Conductor's own), the `TokioChildProcess` stdio trust model for the launched Pulse server, the local-only/no-secrets/no-`DATABASE_URL` stance, and the deliberate `:4317` port-occupier fault are the trust boundary — flag if these aren't aggregated where a security scope would read them.
- **design:** the Tauri panel is "convenience only / thin shell" and "must NOT emit native OS toasts" — flag if this scope ceiling is implicit. (Do NOT pick colors/fonts/components.)
- **a11y:** input modalities are CLI/headless (source of truth) plus the Tauri start/stop/picker/operator-pause surfaces — flag if the operator-pause prompt and picker focus surfaces are mentioned only in passing rather than named as UI surfaces.

**Anchor example:** Occupied Resources, Ports

> "- `127.0.0.1:4317` — OTLP/gRPC **egress target** (Pulse's loopback ingest). Conductor is the client; the port is owned by Pulse, but Conductor must be co-located with it. The port-occupier fault scenario also binds `:4317` to exercise Pulse's reaction."

**Issue:** The single most security-relevant fact — that Conductor *deliberately binds* the loopback OTLP port `:4317` as a fault scenario — lives only in the Ports list. The trust-boundary facts a security scope needs are scattered: the loopback-only posture is here, the `TokioChildProcess` stdio trust model is in Occupied Resources "Service / process names" and the [MCP Read-Back Client] decision, and the no-secrets/no-`DATABASE_URL` stance is in the env-var list. There is no Cross-cutting Patterns "trust boundary" aggregation a security scope could read in one pass (the Cross-cutting Patterns section covers Config management, Development Style, Determinism discipline, Verdict/error wall, and Scope law — searched it for "loopback", "trust", "boundary", "TokioChildProcess" — none present).

**Why this matters:** When `/andromeda-scope-arch` spins up a security scope, it reads Cross-cutting Patterns and Occupied Resources for the trust boundary. A harness that intentionally binds a port it normally only talks to as a client is exactly the surface a security pass must reason about; if that fact is buried as one clause in a Ports bullet, the scope's Phase 0 either misses it or has to re-derive it from three sections.

**Adversarial:** If the port-occupier fault that binds `:4317` is documented only inside the Ports list and never aggregated into a trust-boundary surface, what does a downstream security scope conclude about Conductor's listener posture when its Phase 0 reads "No inbound listener of Conductor's own" two bullets later and takes it literally?

### 5. Decision Completeness [priority: high]

Does every `[Bracketed]` decision carry a rationale and name its rejected alternative? Are there enum states or thresholds that are declared but never given a trigger or a binding rule, and should be promoted to explicit decisions? Are there resolvable TBDs that could be pinned now (without inventing values)?

**Anchor example:** Standard Contracts, envelope footer (vs the enum's full state set)

> "`verdict ∈ {Pass, Fail, CalibrationRegion}`; `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`."

**Issue:** `ReportState` declares five states, but only three have a stated trigger anywhere: `Blocked` (preflight `ready: false`, line 91), `Fail` ("wrong incident", line 42), and `Pass`. `ManualCheck` and `KnownResidual` are declared in the enum at all four wall sites but have **no entry condition stated in the document** (searched for `KnownResidual`, `known-residual`, `ManualCheck`, `manual-check`, `degraded_mode` — `ManualCheck`/`KnownResidual` appear only in the three enum recitals plus the negative "never pass/fail/manual-check"; `degraded_mode` appears once, as a `retrieve_report` tool parameter on line 125, never wired to `KnownResidual`). The operator-checklist path (Project Intent: "via an operator checklist for visual claims") is the obvious `ManualCheck` trigger and the `degraded_mode`/`retrieve_report` path is the obvious `KnownResidual` trigger, but neither binding is decided.

**Why this matters:** A state in the type that no rule ever produces is either dead code or a silent gap. The reporter must classify every scenario into exactly one state; if `ManualCheck` and `KnownResidual` have no trigger, implementation will either never emit them (making the operator-checklist and degraded-mode paths fall through to the wrong state) or invent ad-hoc triggers that differ between the CLI reporter and the Tauri view.

**Adversarial:** If `KnownResidual` is a first-class report state but no decision maps any signal to it, which scenario outcome silently lands in `ManualCheck` or `Fail` instead — and how does a human reading the run report tell a known-accepted residual apart from a real failure?

### 6. Conventions Specificity [priority: high]

Are the `runs.db` data-model conventions concrete enough to implement without a follow-up question — column types, nullability, enum closure, and format? Are the naming laws stated as enforceable rules rather than shown only by example? Is the config-validation contract complete (every garde invariant carrying its bound)?

**Anchor example:** Conventions, Data model conventions (SQLite / `runs.db`)

> "Timestamps are stored as journal-relative values whose source of truth is the JSONL journal; wall-clock stamps for the journal come from `std::time::SystemTime`/`Instant` (never tokio's virtual clock, which would break journal-relative SLO math). Fingerprint arrays are indexed via SQLite JSON1."

**Issue:** "Journal-relative values" never states the stored type or unit. The run-report envelope shows `"latency_ms": 1840` (integer-looking) but the Conventions paragraph and the [Run-History Persistence] decision never declare `latency_ms` as integer milliseconds, never say whether `journal_emitted_at`/`read_back_observed_at` are stored as integer offsets or ISO strings, and never close `slo_tier` as an enum string over `<5s`/`<20s`/`<90s` (searched Conventions and Occupied Resources for `latency_ms`, `slo_tier`, `integer`, `nullable`, `TEXT` — only the JSON example values exist; no column typing anywhere). "Fingerprint arrays are indexed via JSON1" likewise does not say the stored element type (TEXT array? what's the JSON1 array element).

**Why this matters:** `runs.db` is hand-written raw SQL with no migration framework — the schema is fixed on first write and there is no ORM to coerce types. If `latency_ms` is created as `REAL` in one place and read as integer ms in another, or `slo_tier` is an open TEXT column instead of a checked enum, cross-run and seed-to-seed queries (the entire reason the index exists) return inconsistent results, and the P-036 fingerprint-recurrence check can miss matches across differently-typed rows.

**Adversarial:** If `latency_ms` has no declared SQLite column type and the only hint is one example value `1840`, what happens to the seed-to-seed latency comparison when one run inserts an integer and another inserts a float-formatted string into the same un-migratable column?

### 7. Specialist Content Boundary [priority: medium]

There IS a UI surface (Tauri 2 control panel), so this dimension applies — but Tauri 2 is the desktop *shell binding the primary language*, parallel to the backend, and is in-scope (like a mobile framework would be). Hunt the layers *inside* that shell. Does any section name a **frontend framework** (React/Vue/Svelte/SolidJS/Angular), a **CSS tool** (Tailwind/vanilla-extract/CSS Modules/styled-components), or a **component library** (shadcn/Radix/Material UI/Headless UI)? Does any section name a **logger** (tracing-subscriber/Pino/Winston as a *chosen* stack), a **non-Cargo test/assertion/property framework**, or an **auth library**? If yes → flag for removal into the owning specialist's territory.

Confirm the in-scope items are NOT mis-flagged: Tauri 2, the `Channel` IPC primitive, `#[tauri::command]`, the Tauri 2 bundler, serde, garde, thiserror, anyhow, rmcp, tonic, rusqlite, opentelemetry-proto, and the Cargo-native gates (`cargo test`, `cargo-nextest`, `clippy`, golden tests) are architectural picks that belong here — do not strip them. The current draft describes the webview as "bundled" with no framework named; confirm it stays that way and that no patch introduces one.

**Adversarial:** If a future iteration "helpfully" names a frontend framework or a CSS tool for the Tauri webview to make the GUI "buildable," which specialist's plan (design) is now pre-empted by an arch-level pick it never agreed to, and which value-domain decision gets made by the wrong agent?

### 8. Complexity Calibration [priority: medium]

Is the stack proportional to "Personal — solo developer, local dev host, no cloud, no multi-tenancy"? The doc already rejects ORM/migrations, a message broker, a web framework, containers, and a scenario DSL as over-scope. Flag any *remaining* enterprise pattern that slipped in, OR conversely any robustness genuinely missing for a deterministic verification harness (is the determinism discipline — seeded RNG + `current_thread` + `SystemTime`/`Instant` journal stamps — sufficient, or is a reproducibility gap under-addressed?). Does the seam count match "modular monolith, one deployable": eight workspace members (`conductor-core`, `-timeline`, `-emit`, `-faults`, `-verify`, `-report`, `-cli`, + the Tauri bin) for six named seams — any crate that is weight without a payoff, or any named seam (timeline · emission · faults · verify · report · control panel) lacking a home crate?

**Adversarial:** If load-testing is an explicit non-goal but a later iteration adds a connection pool, retry/backoff layer, or throughput knob "for robustness," which non-goal does it violate, and how does that extra machinery weaken the determinism-under-seed guarantee the whole architecture is built to enforce?

---

Read the document, walk the Analysis Protocol steps in order, then output patches and changelog within the 8-patch budget, bucket 1 first.
