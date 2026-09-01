# Report — 2026-09-01-desktop-a11y-sweep

**Chunk:** Desktop a11y sweep — the a11y leg calibrated to zero: the one genuine axe violation named and
fixed at the defect, the two subject-absent specs given the context-skip shape, and the accessible paths
asserted for real through a driven session rather than skipped green
**Date:** 2026-09-01
**Commits:** (none yet — this wrap commits the chunk)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-tauri/ui/src/styles/tokens.css` (token VALUES, both theme blocks)
  - `crates/conductor-tauri/ui/src/components/RunControls.tsx` · `RunControls.css`
  - `crates/conductor-tauri/ui/src/components/OperatorPauseDialog.tsx`
  - `crates/conductor-tauri/ui/src/App.tsx`
  - `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (routine arm, rewritten)
  - `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` (**new** — driven arm)
  - `crates/conductor-tauri/ui/wdio.conf.ts` · `package.json`
- **Symbols / APIs:** no Rust symbol, `#[tauri::command]`, IPC method, port or exported binary surface changed.
  TS-side: `OperatorPauseDialog` gains one optional prop `restoreFocusTo?: () => HTMLElement | null`
  (additive; its sole caller is `App.tsx`, and the prop is optional so the shipped Gallery call site is
  unaffected — **not** a sole-caller-only change: `Gallery.tsx` also renders this component and keeps
  compiling unchanged). `RunControls` keeps its prop signature; only Start's availability MECHANISM changed
  (native `disabled` → `aria-disabled` + a guarded `onClick`).
- **Crates / modules:** none added, removed or changed. Zero `.rs` delta; the workspace member roster is
  untouched.
- **Dependencies:** none added, none bumped. `Cargo.lock` and `package-lock.json` byte-unchanged.
- **Schema / config:** no migration, no config key, no violation-schema or redaction-shape change. `wdio.conf.ts`
  gains a `suites.driven` entry, narrows default `specs` to the routine file, sets `cwd: repoRoot` on the
  tauri-driver spawn, and raises `mochaOpts.timeout` 60s → 15min. `package.json` gains the `a11y:driven` script.
- **Spec-master edits:** none at authoring — this wrap's P2 applies them.
- **Counts / qualifiers moved:** **YES — three token VALUES, and one derived qualifier claim they falsify.**
  - `--text-tertiary` dark `#717AA0` → `#838EBA`; `--text-muted` dark `#565F89` → `#727EB5`; `--text-muted`
    light `#6E7491` → `#636882`. Docs baking the OLD values: `design-system.md:50` (Tertiary row),
    `:51` (Muted row), `:220` (dark `:root` block), `:241` (light block).
  - **The derived qualifier:** dark `--text-muted` no longer shares a hex with `--count-blocked` (`#565F89`).
    Sites asserting that they DO share, now false: `layout-templates.md:130` (Mode cell — "the latter shares
    its hex with `--text-muted`, so a plain dim reads as `Blocked`") and `.claude/rules/frontend.md:25`
    (same sentence, cascaded). The *design intent* the claim protects — the out-of-scope Mode cell stays
    recessive via `--status-residual`, never `--status-fail`/`--count-blocked` — is unchanged and still
    correct; only its stated REASON is now stale.
  - Not moved: `--count-blocked`, `--status-manual`, `--status-residual`, `--border-emphasis` and every
    status/lamp token are byte-unchanged (`design-system.md:59-60`, `:70`, `:213`, `:237-238`, `:295` stay true).
- **Dev-tool versions:** none installed or upgraded. (`msedgedriver` / tauri-driver / Pulse binaries are the
  operator's host tools, unchanged this chunk.)
- **Harness / gate surface:** **YES.**
  - `--e2e` keeps its meaning (ensure-frontend → `--features tauri/custom-protocol` release build → `wdio run`)
    but now runs the **routine arm only**; the tauri-driver child is spawned with `cwd` = workspace root.
  - **New operator/local leg:** `npm run a11y:driven` (`wdio run wdio.conf.ts --suite driven`). Its FULL
    firing form is a required record (see §Deviations, item 4): a `PATH` prefix resolving
    `andromeda-pulse-mcp`, `ANDROMEDA_PULSE_MCP_ENABLED` / `_L4_DETERMINISTIC`, and
    `ANDROMEDA_PULSE_DATA_DIR` = **the live Pulse's** data dir, plus a quiet-window precondition.
  - Neither arm is wired into CI; test-plan §9's live-leg ban stands.
- **Cross-project / external claims:** ground truth in `andromeda-pulse` (HEAD `83d4060`), read at source:
  `pulse-app/src/inference_runtime.rs:811` — the incident dedupe predicate
  `.find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)` over
  `registry.list_active(&digest.workspace)`. This is the mechanism behind the driven arm's quiet-window
  precondition. Also: `andromeda-pulse-mcp` ships at `andromeda-pulse/target/release/` and is on no PATH here.
- **Reverted / negative API facts:** the first focus fix — switching Start to `aria-disabled` — was
  **kept but proved insufficient**: focus still landed on `<body>`. It was NOT reverted (a natively disabled
  button leaves the tab order, so the change is independently correct), but it is not the fix; the explicit
  `onCloseAutoFocus` restore is. Recording this so the `aria-disabled` change is not later read as the
  SC 2.4.3 remedy on its own.
- **Spec claims disproved by measurement:**
  1. **a11y-plan.md:419 (§6 Motion tokens) and :592 (§12 Decisions Log)** prescribe
     `browser.emulate('prefers-reduced-motion', 'reduce')` as the SC 2.3.3 drive path. **The API does not
     exist**: webdriverio 9.x declares exactly six emulate scopes — `clock`, `geolocation`, `userAgent`,
     `device`, `colorScheme`, `onLine`
     (`node_modules/webdriverio/build/commands/browser/emulate.d.ts:6-11`). This is broader than the plan's
     own "recorded-not-established under classic WebDriver on WebView2" caveat, which framed it as a
     platform question; :592 said "verify at implement", and this is that verification.
  2. **Radix restores focus to the trigger** (the assumption behind a11y-plan §5's "restores focus to the
     trigger on close", and behind `layout-templates.md` §Component — Hero). Measured false **for this
     dialog**: the hold arrives over a Tauri `Channel`, so the `AlertDialog` has no `Trigger`, and on close
     focus landed on `<body>` — twice, including after the invoker was made focusable. The contract is
     satisfiable only with an explicit `onCloseAutoFocus`.
  3. **This chunk's own v2-22 acceptance clause** ("each of the SIX lamp display labels…") — no
     release-bundle surface renders all six; recorded as a PREMISE-CORRECTION in
     `verification-matrix.json` `notes` at implement, awaiting this wrap's acceptance-lifecycle disposition.
- **Coverage of new surfaces:**
  - `--e2e` routine arm (`accessibility.e2e.ts`) → validation `n/a` · instrumentation `n/a` · PII `n/a` ·
    tests `e2e` (6 passing, 2 context-skipped, exit 0) · a11y `WCAG/focus/kbd✓` (axe 0 violations; 9+2 token
    pairs both themes; not-color-alone; reduced-motion) · tokens `design-token✓`
  - `a11y:driven` arm (`operator-hold.e2e.ts`) → validation `n/a` · instrumentation `n/a` · PII `n/a` ·
    tests `e2e` (1 passing, 54s, live Pulse) · a11y `WCAG/focus/kbd✓` (trap entry, Tab/Shift+Tab containment,
    Space toggle, `role=status` announce, Escape→NoGo, focus restoration) · tokens `n/a`
  - `RunControls` Start button (changed affordance) → validation `n/a` · instrumentation `n/a` · PII `n/a` ·
    tests `e2e` (driven arm asserts its `aria-disabled` state before use) · a11y `WCAG/focus/kbd✓`
    (stays focusable; `aria-disabled` conveys unavailability; guarded activation) · tokens `design-token✓`
    (both `:disabled` and `[aria-disabled='true']` render identically)
  - `OperatorPauseDialog.restoreFocusTo` (new prop) → validation `n/a` · instrumentation `n/a` · PII `n/a` ·
    tests `e2e` (driven arm asserts the restoration it enables) · a11y `WCAG/focus/kbd✓` (SC 2.4.3) ·
    tokens `n/a`

## Deviations from intent

1. **Scope widened beyond the plan's Files-to-modify — operator-authorised mid-implement.** The plan scoped
   `tokens.css`, `accessibility.e2e.ts`, `wdio.conf.ts`, `package.json` + the new driven spec. The driven arm
   found a real SC 2.4.3 defect whose fix needed `App.tsx`, `RunControls.tsx`/`.css` and
   `OperatorPauseDialog.tsx`. Implement stopped at the boundary and asked; the operator chose "widen scope —
   fix it now". Justification: v2-22's acceptance names focus restoration, so the alternative was shipping a
   known violation on a must-be-accessible path or un-claiming the capability.
2. **`wdio.conf.ts` changed more than "a second suite selector".** It also gained `cwd: repoRoot` on the
   driver spawn and a raised mocha ceiling. Justification: the app resolves `scenarios/` relative to its cwd
   through `resolve_under`, which **rejects absolute paths and `..` by design** — so no `CONDUCTOR_*` handle
   could reach the catalog from the ui package, and the launch cwd was the only available lever. The raised
   ceiling is required because wdio enforces `mochaOpts.timeout` itself (an in-test `this.timeout()` does not
   raise it) and the hold sits ~50s behind preflight's canary poll.
3. **The reduced-motion assertion is weaker than the plan's step 6 intended**, because the prescribed API does
   not exist (see Spec claims disproved #1). It asserts the compiled `@media (prefers-reduced-motion: reduce)`
   rule per-declaration (`animationName: none`, `transition: none`, both `!important`, selector `*`) and
   states the epistemic limit in-file: this proves the rule ships and is correctly shaped, **not** that the OS
   preference was exercised. SC 2.3.3 conformance is not claimed from it alone.
4. **The driven arm's firing form is a required record, not a bare invocation** (operator wrap directive).
   Three preconditions, each measured today, must be registered with the leg in test-plan §6/§9:
   - **(a) The PATH prefix.** `conductor-verify` spawns the sidecar by fixed NAME
     (`spawn.rs:15` — `PULSE_MCP_PROGRAM = "andromeda-pulse-mcp"`, "resolved from `PATH`, never an
     operator-supplied command"), and that binary is on no PATH on this host. Without the prefix, preflight
     returns BLOCKED in ~2ms with all four tools `absent` — which at row level is **indistinguishable from a
     genuine SUT-side gate failure**. A PATH addition is part of a firing form.
   - **(b) The data-dir EQUALITY precondition** Conductor's own code states:
     `conductor-run/src/lib.rs:93` — `"… ANDROMEDA_PULSE_DATA_DIR == live Pulse's data-dir"`. The sidecar
     reads the published workspace key from THAT dir; pointed at the platform default it reads a corpus Pulse
     never writes, yielding `result_count: 0` forever (measured today).
   - **(c) The QUIET-WINDOW precondition** — ≥ 120s idle + 30s resolver tick after ANY preflight canary,
     because `conductor preflight` fires its own canary storm and Pulse dedupes per `(kind, scope, scope_id)`
     tuple (`inference_runtime.rs:811`, the same predicate the architecture CARRY re-scoped). Running
     `agent-run boot` as a "cheap gate" immediately before the driven leg primes exactly the state the leg
     then collides with — the boot+run pairing plan-template's LIVE-LEG rule warns against. Cost when
     violated: two 12-minute runs.
5. **Workspace compile gates deferred** (`cargo nextest --workspace --profile ci`, `cargo clippy --workspace
   --all-targets`) under the source-delta-proportional rule: zero `.rs`, zero manifest, zero lockfile delta,
   and `--e2e` already compiles `conductor-tauri` in release with `--features tauri/custom-protocol`. Re-run
   at the next source-touching chunk.

## Decisions & corrections

- **Operator ruling — chunk shape (P5 review):** two arms, claim v2-22. A calibrate-only chunk was the
  alternative; rejected because the driven arm is the only thing that makes the capability's keyboard clauses
  non-hollow.
- **Operator correction — acceptance wording:** my concretized "the harness transport is run, not
  re-authored" was sharpened to **"the harness transport is run, not re-authored; specs may be extended,
  never relaxed."** My phrasing would have licensed deleting a failing spec — the exact hole the original
  clause guarded. I had flagged it as a wording loosening; it was a weakening, and the never-weaken
  checkpoint is what caught it.
- **Operator directive — judgment-class disposition:** the entry's "operator judgment pass" had to be
  dispositioned in the plan, not silently dropped. Recorded as **none identified as a chunk gate**, checked
  against a11y-plan: the screen-reader pass is v2-23's entry, cli keyboard verification is excluded from the
  a11y harness as not-assertable, macOS VoiceOver has no surface here; the holistic pass stays discretionary.
- **RETRACTION — my Pulse-side diagnosis was refuted by artifacts (operator wrap directive item 2).** I
  claimed (i) "pulse-app was launched without deterministic L4, so the non-deterministic 3B path explains
  `result_count: 0`" and (ii) "the `workspace-key` file is absent at HEAD `83d4060` though that HEAD publishes
  it". **Both are false.** The live Pulse runs against
  `…\Temp\pulse-legs\a11y-20260901-231019`, whose `run/andromeda-pulse.pid` reads **72032**, whose
  `run/workspace-key` **is** published (42 B), whose `corpus/corpus.db` is the live one, and whose log carries
  *"L4 deterministic mode active"*. `%APPDATA%\andromeda-pulse` is a **stale default** (pid 3400, dated Aug 14);
  its 10 MB `corpus.db` got its mtime from **my own sidecar opening it** — a lone reader, not a shared writer.
  Three inferences retracted: mtime-during-my-run ≠ shared writer; "no arguments" ≠ no env (the env came from
  the launcher script); and the `workspace-key` was measured in the **wrong directory**. Recorded because,
  unrecorded, a false Pulse-side claim travels as a future intake item.
- **Correction to my own first fix hypothesis:** "a natively `disabled` Start is an unfocusable restore
  target" was measured false — `aria-disabled` kept it focusable and focus still fell to `<body>`. The cause
  is the absent Radix `Trigger` (Channel-opened dialog). Kept the `aria-disabled` change on independent
  a11y grounds, and said so rather than presenting it as the remedy.
- **Route disposition (operator wrap directive item 3):** the self-collision's RELIABLE shape is the quiet
  window (item 4c). The DESIGN option — a per-run canary identity, salting the canary's `scope_id` so
  preflight and driven-run tuples differ — is **weighable, not decided**, and is pinned as a CARRY on the
  Epoch-6 *Operator-gated live suite* entry (its stated job: a re-runnable live-Pulse proof invocation, never
  a CI gate). Arm B never promotes toward CI.
- **Diagnostic discipline reaffirmed twice:** the shipped axe assertion reported only `violations.length`, so
  a two-month-old red named no rule; it now reports every violation by id/impact/nodes/help. The same defect
  recurred in my own driven-arm failure message, which dumped the entire rendered document — bounded to
  `aria-label`, else 60 chars for interactive elements, else the tag name.

## Outcome

**Acceptance criteria met** (both arms green; the one narrowed clause is recorded, not silently passed).

Gates run:
- `npm run typecheck` → exit 0 · `npm run typecheck:e2e` → exit 0
- `bash scripts/agent-run.sh run --e2e` → **exit 0** · 6 passing · 2 context-skipped (routine arm)
- `npm run a11y:driven` (full firing form, live Pulse) → **1 passing (54s)** (driven arm)
- **PREREQ, 42nd consecutive, COMPACT form — signature reproduced byte-identically:** `cargo audit` true
  exit **1**, first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`;
  `cargo deny check advisories bans licenses sources` true exit **0** (`advisories ok, bans ok, licenses ok,
  sources ok`). Each exit captured BEFORE any pipe. No deviation → the bounded wait stands (no floor raise,
  no `deny.toml` ignore, no CI edit). `Cargo.lock` byte-unchanged, so the compact basis survives.
- Deferred (noted, not skipped): `cargo nextest --workspace --profile ci`, `cargo clippy --workspace
  --all-targets` — zero compiled-source delta.

**Smoke:** the UI-surface self-verify ran as a P2 gate (live WebView2 151.0.4129.107 session against the
release bundle, both arms), so P3 recorded it rather than re-running. The driver stack bound and released
`4444`/`4445` with the session, leaving no process behind.

**The defect this chunk existed to find:** one genuine `color-contrast` violation (wcag2aa / SC 1.4.3,
18 nodes, all `--text-tertiary`) — fixed at the token. Measurement then showed axe's node list was a floor,
not the scope: two further §6 pairs failed unrendered, in both themes. And the driven arm found a second,
independent defect (SC 2.4.3 focus restoration) that no static check would have reached.
