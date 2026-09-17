# Report — 2026-09-17-keyboard-and-focus-order-coverage-ownership

**Chunk:** Keyboard and focus-order coverage ownership — the hold-dependent trap and restoration half's owner named, and its CI carve-out stated
**Date:** 2026-09-17
**Commits:** none since `last_wrap` 2026-09-17T11:41:47Z other than `3ddd405` (the predecessor's own wrap commit); this chunk is uncommitted at authoring time.

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-tauri/ui/test/a11y/claim-ownership.ts` (new) · `crates/conductor-tauri/ui/test/a11y/check-claim-ownership.ts` (new) · `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (one spec title) · `crates/conductor-tauri/ui/package.json` (one script) · `conductor-0.3.0/verification-matrix.json` (a `notes` line on `v3-03`, written by phase P5) · the chunk folder + two run dirs.
- **Symbols / APIs:** new TS exports, all in `claim-ownership.ts` and all consumed by `check-claim-ownership.ts` alone — `Suite` (type), `SUITE_SPEC`, `SUITE_INVOCATION`, `CI_SUITES`, `Claim` (type), `CLAIMS`, `NA_CLAIMS`. No remaining external callers: `grep -rlF "claim-ownership" crates/conductor-tauri/ui --include=*.ts` (node_modules excluded) → **2 hits · 1 import · 1 no-change** — `check-claim-ownership.ts` is the sole importer, and the second hit is a prose mention inside the comment this chunk added above `accessibility.e2e.ts:450`, not a reference. (Stated as 1 on first authoring and corrected here by running it: the token matched prose about the token.) No IPC method, endpoint, port, socket or env var added or changed.
- **Crates / modules:** none added, removed or changed. `crates/conductor-tauri/ui/` is not a Cargo workspace member; no Rust surface was touched (zero `.rs` delta).
- **Dependencies:** none. `tsx` was already a devDependency; `Cargo.lock` and `package-lock.json` are byte-unchanged (`git status --short` lists neither).
- **Schema / config:** none. No migration, no config key, no violation-schema or scrub/redaction shape changed. `knip.json` was edited and then reverted — byte-identical to HEAD (`git diff --quiet crates/conductor-tauri/ui/knip.json` → clean).
- **Spec-master edits:** none. All a11y-plan changes ride the Expected amendments below; `plan-template.md` §Discipline bars a spec master from a chunk's modify-set.
- **Counts / qualifiers moved:** **none — verified.** The retitled spec's OLD title (`operator-pause dialog: alertdialog role, focus trap, Escape resolves NoGo, focus restores`) is quoted in no master, rule file, script or contract: `grep -rnF "operator-pause dialog: alertdialog role" .andromeda/ .claude/ conductor-0.3.0/ scripts/ crates/` (excluding `/runs/`) → **1 hit, and it is this chunk's own `research.md:78`**. The routine arm's tally is unmoved (12 passing / 0 failing / 2 skipped) and `$A11yExpectedSkips = 2` at `scripts/agent-run.ps1:61` is untouched, so no documented derived value changed.
- **Dev-tool versions:** **none** — no host tool installed, upgraded or read changed. The `--e2e` leg ran against WebView2 runtime `153.0.4234.32` on the dev host, a re-read of the already-current runtime, not a bump.
- **Harness / gate surface:** one npm script added — `a11y:ownership` = `tsx test/a11y/check-claim-ownership.ts`, a Pulse-free, driver-free static checker. **No change to `scripts/agent-run.{sh,ps1}`, to `.github/workflows/ci.yml`, or to what CI runs.** No status or verdict shape changed; the checker prints its own verdict line (`ownership: every claim resolved`) as its last line and is asserted on that, not on its exit.
- **Cross-project / external claims:** none. No CI run was read; `3ddd405` was not on the remote when the phase read it (`No commit found for SHA`, HTTP 422).
- **Reverted / negative API facts:** the `knip.json` `entry` registration the plan's step 3 directed was written and then **deliberately reverted** — knip itself reported the pattern redundant, because it already reaches the checker through the `package.json` script the same step adds. Step 3's intent (static tooling sees the checker) is met by the script alone.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** **one, and its subject is route freight, not a spec master.** `working-route.md:39`'s CARRY 3 frames six surviving `msedgewebview2` processes as an open orphan-hygiene defect. Measured by PARENTAGE this chunk — the discriminator the CARRY itself names as missing and that no prior census took — all twelve present processes are children of ordinary desktop applications: six rooted at `SearchHost.exe`, six at `WhatsApp.Root.exe`, **none descending from `tauri-driver`, `msedgedriver` or `conductor-tauri`**. This chunk's `--e2e` leg left zero survivors of its own. Evidence: `evidence/process-census.md` (Win32_Process `ProcessId`/`ParentProcessId`/`CreationDate`, each parent resolved to its image name, taken before and after the leg per `.claude/rules/verification-harness.md:58`). **No spec master states this claim** — `grep -rlF "msedgewebview2" .andromeda/*.md` → 0 hits — so the disposition is the route's at P5, not an amendment.
- **Expected amendments (from plan):** five entries, all targeting `a11y-plan.md`, each located by its own measurement against that file:
  - **§5 per-claim owner sentences** for the five unattributed bullets — sites confirmed by reading each: `:354`, `:355`, `:362`, `:366`, `:370` all lack an `Asserted …` attribution clause (5/5 unattributed); `:353` already carries one and needs no sentence. Motivating fact: the ownership enumeration under *Symbols / APIs* above. **carried.**
  - **§11 Strategy carve-out substitute-gate sentence** (`:516`) — the carve-out already narrows to the hold-dependent half and already states WHY it is operator-local, but names neither the owning suite nor the CI substitute. Motivating fact: the checker's carve-out output names three claims owned by `operator-hold.e2e.ts` (`npm run a11y:driven`). **carried.**
  - **§5 `:366` F3** — the retired "(Radix AlertDialog default …)" attribution: `grep -c "Radix AlertDialog default" a11y-plan.md` → **1 hit, at `:366`**, contradicted by `:362`, `:131`, `:253` and `.claude/rules/a11y.md`, which all state the explicit `onCloseAutoFocus` + `restoreFocusTo` contract. **carried.**
  - **§5 `:355` and `:362` F4** — both describe the HOLD cycle as "across Proceed/Abort" with no checklist mention (measured: both lines contain `Proceed/Abort` and neither contains `checklist`), while `:131` and `:253` both include the operator-checklist `checkbox` rows. **carried.**
  - **§1 `:115` CARRY 1** — the verbatim dittography, **RE-MEASURED at this wrap**: `:115` is **3691 characters**, the clause `the one webview-automation stack running on the measured platform SET — ` stands at offsets **2555 and 2627**, delta **72** = the clause's own length (72). Unchanged from the phase reading; this wrap has applied no a11y-plan edit yet. **carried.**
- **Coverage of new surfaces:**
  - `claim-ownership.ts` (committed data enumeration, no runtime surface) → validation `n/a` · instrumentation `n/a` · PII `n/a` · tests `unit` (the checker asserts it) · a11y `n/a` · tokens `n/a`
  - `check-claim-ownership.ts` (static checker, reads repo source, no network, no subprocess) → validation `n/a` · instrumentation `n/a` · PII `redacted✓` (prints suite and claim names only, never a filesystem path) · tests `unit` (it IS the gate; its anti-vacuity arm proven able to fail by a known-positive control) · a11y `n/a` · tokens `n/a`
  - `npm run a11y:ownership` (harness verb) → validation `n/a` · instrumentation `n/a` · PII `n/a` · tests `unit` · a11y `n/a` · tokens `n/a`
  - `accessibility.e2e.ts:450` (existing spec, title only) → validation `n/a` · instrumentation `n/a` · PII `n/a` · tests `e2e` (re-run green in the `--e2e` leg, still skipping) · a11y `WCAG✓` (it asserts nothing by design; its claims are owned by the driven arm) · tokens `n/a`

## Deviations from intent

1. **The plan's step 3 knip.json registration was removed rather than added.** Justification: knip reported the pattern redundant — the `package.json` script the same step adds already makes the checker reachable in knip's import graph. Keeping a registration the tool flags would have traded one finding for another. Step 3's stated intent is met; `knip.json` is byte-identical to HEAD.
2. **The §5 `run-console-idle` bullet was split into two enumeration rows.** Justification: the bullet bundles SC 2.1.1 reachability and SC 2.4.3 order, which are asserted by two different specs (`accessibility.e2e.ts:408` and `:424`), so one bullet-keyed row could name only one owner. `v3-03`'s acceptance quantifies over claims, not bullets. The enumeration ships 10 rows over 9 §5 bullets for this reason.
3. **`CI_SUITES` and `SUITE_INVOCATION` were put to use rather than deleted** when knip flagged them unused. Justification: they carry the carve-out population, which is precisely the obligation the §11 amendment must discharge; the checker now prints it. Deleting them would have removed information the chunk exists to surface.

## Decisions & corrections

- **Operator fork, 2026-09-17 (three questions, all answered with the recommendation):** the a11y-plan prose rides wrap's amendment flow while /implement writes a committed enumeration and checker; `accessibility.e2e.ts:450` is closed this chunk; F3 and F4 ride this chunk's amendment set while F5 goes to the route.
- **The claimability half of that fork's rationale was reversed by a later measurement**, surfaced at the P5 review before approval: `v3-03` could not be claimed, because three §5 claims are asserted by no suite at all. It stays pooled with a failed-concretization `notes` line.
- **Sweep hazard — a token that names its own absence.** `matrix.py audit` greps `## Implementation notes` for the ledger-note Expected-amendment token; a parenthetical stating that this chunk routes NO such amendment was matched exactly like a real entry and reported as one, which would have handed wrap P7.3 an instruction for a note already written. Fixed by describing the form instead of spelling it. Same family as a probe matching the comment that explains why not to use a construct.
- **Sweep hazard — "named in a prior plan" is not "was ever green".** P5's novelty predicate keys on whether a command's program and scope tokens appear in a prior plan or report; `npm run knip` appeared in three, so it was not marked `new` and was never baselined — while exiting 1 at HEAD the whole time.
- **A count and a StartTime cannot attribute a process.** Every WebView2-hosting desktop app spawns under the one `msedgewebview2` image name; only parentage separates them.
- **A green check is not evidence the check can fail.** The checker's anti-vacuity arm was proven able to fail by a one-shot known-positive control, both readings kept in evidence.

## Outcome

Acceptance criteria, re-asserted against the diff:

- (a11y) every §5 claim in the enumeration in exactly one of three states, none absent — **MET**: 10 rows covering §5's nine bullets (`:353` split by SC), checker green.
- (a11y) no claim owned by a spec that asserts nothing — **MET**, and proven falsifiable: the control produced `FAIL run-console-HOLD focus trap: … asserts nothing — 0 expect( in its body` at exit 1.
- (a11y) hold-dependent claims name `operator-hold.e2e.ts` as single owner, no CI gate named — **MET**: three carve-out rows printed, owner `driven`, `CI_SUITES = ['routine']`.
- (tests) every owner resolves to a test-plan §6 registered suite — **MET**: `SUITE_SPEC` names exactly the three registered specs.
- (tests) `a11y` job spec set unchanged, routine arm still 12 passing / 0 failing / 2 skipped — **MET**: no `ci.yml` delta; leg verdict `[a11y] verdict asserted — 0 failed · 2 skipped (expected 2) · driven session present`.
- (arch) carve-out text states CI green with its configuration — **carried to the amendment, not yet applied** (owner: P2 of this wrap).
- (arch) only `knip.json`/`package.json` as registry edits, no new crate/port/env var/artifact — **MET, narrowed**: `knip.json` ended byte-identical, so `package.json` is the sole registry edit.
- (layouts) enumeration layout keys are shipped run-state names only — **MET**.
- (design) the two N/A rows cite the single-station no-router console — **MET**.
- (security) zero host-path tokens and zero internal struct names in committed files — **MET**: `grep -nE '[A-Za-z]:[\\/]|/home/|/Users/|%APPDATA%'` over the evidence tree → clean.
- (security) zero dependency delta — **MET**.
- (obs) no claim attributed to an obs span; no `focus.*` span added — **MET**.
- (tests) `cargo nextest run --workspace --profile ci` returns 0 — **MET**.

Gates, by `run` text:

- `(cd crates/conductor-tauri/ui && npm run typecheck:e2e)` — **green** (`exit 0`).
- `(cd crates/conductor-tauri/ui && npx tsx test/a11y/check-claim-ownership.ts)` — **green** (`exit 0`, `last line ownership: every claim resolved`). P5 baseline was `red` (ERR_MODULE_NOT_FOUND); red-before-green held.
- `(cd crates/conductor-tauri/ui && npm run knip)` — **red — not this chunk's: the same 12 findings on a detached worktree at the parent commit `3ddd405` with `node_modules` junctioned, exit 1, committed as `evidence/knip-at-HEAD-3ddd405.log`, byte-identical in its findings to the working tree's reading → owner: this wrap's P5 pin.** All 12 are unused exports in `src/components/ScenarioPicker.tsx`, `src/components/CoverageMatrix.tsx`, `test/a11y/screen-reader/parse-nvda-log.ts` and `rows.ts` — none touched by this chunk. The entry's `expect = ['exit 0']` was unsatisfiable at HEAD when authored.
- `test -n "$CONDUCTOR_MSEDGEDRIVER" && …` — **green** (`exit 0`, `last line DRIVER_RESOLVED`).
- `bash scripts/agent-run.sh run --e2e` — **`leg = 'live'`, driven by hand at /implement** (the gate tool never fires a leg): `exit 0`, `contains [a11y] verdict asserted` ✓, `lacks [a11y] leg skipped` ✓. 12 passing / 0 failing / 2 skipped, WebView2 `153.0.4234.32`, artifact `runs/a11y-e2e.log` fresh for that run (15 118 632 B, UTF-8 no BOM).
- `cargo nextest run --workspace --profile ci` — **green** (`exit 0`). Ran un-deferred: `verification-matrix.json` carries an uncommitted delta and `crates/conductor-report/tests/matrix_ledger_gate.rs` reads it.
- `cargo clippy --workspace --all-targets -- -D warnings` — **`defer`**, reason confirmed against Setup's changed-files list: zero `.rs` and zero `Cargo.*` delta. The prescribed per-file grep found `verification-matrix.json` hitting 3 Rust files, but two are test binaries and one a src path constant — a data file read at runtime cannot move a lint's verdict, which is the same distinction that made `nextest` run.
- `cargo audit` — **green** (`exit 0`).
- `cargo deny check advisories bans licenses sources` — **green** (`exit 0`).

Smoke: the plan lists a `role = 'self-verify'` entry and it ran at P2, so P3 recorded rather than re-drove it — `self-verify ran as a P2 gate ✓`.

**Outcome basis:** this session's own implement conversation (P1–P4 held in-window), plus the artifacts it produced on disk — the four files under `evidence/`, the gate trails in `.andromeda/runs/2026-09-17T15-59-38-implement/`, and the leg's `runs/a11y-e2e.log`. No operator directive intervened between implement and this report.

**Process hygiene:** re-measured at this wrap against the host process list.

| process | started by | final state |
|---|---|---|
| `msedgedriver` | this chunk's `--e2e` leg | terminated |
| `tauri-driver` | this chunk's `--e2e` leg | terminated |
| `conductor-tauri` | this chunk's `--e2e` leg | terminated |
| `node` (4) | this chunk's `--e2e` leg | terminated |
| `msedgewebview2` (12) | **not this run** — 6 rooted at `SearchHost.exe`, 6 at `WhatsApp.Root.exe`, by parentage | left running: not Conductor's to stop |
