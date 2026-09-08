# Codebase Research — 2026-09-08-webview2-runtime-152-installed-in-job

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 7 · **Graph queries:** 1 (rust plane)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural
  extraction (55.5 KB, over-cap): seeded sections `:1-39` + all 24 `## Session Additions` entries
  `:40-63` via offset-bounded reads. `.claude/rules/host-win32.md` (unconditional) and
  `.claude/rules/security.md` (unconditional) auto-loaded.
  *Note:* this rule's `paths:` are `scripts/agent-run.*` · `conductor-cli/**` · `conductor-verify/**` ·
  `crates/**/tests/**` — **none of which this chunk modifies**, so the rule would NOT have auto-loaded.
  Its retrieval rides this research-time read, which is exactly the failure mode entry `:58` warns about.

## Files inspected
- `.github/workflows/ci.yml` (`:219-262` job header · `:263-283` versions diagnostic · `:284-313` routine
  arm · `:314-399` session isolation incl. probes (a)/(b) · `:400-432` staging · `:433-470` conformance +
  uploads) — the entire `a11y` job. This is the only file the chunk modifies.
- `conductor-0.2.0/chunks/2026-09-08-hosted-runner-webview2-session/evidence/probe-verdict.md` (full) —
  the measured basis of the folded CARRY.
- `conductor-0.2.0/chunks/2026-09-08-hosted-runner-webview2-session/report.md` (`:108-125`) — the rust-gate
  deferral this chunk's PREREQ closes.
- `crates/conductor-tauri/src/main.rs` (`:36-66`, grep) — the backend log sink.
- `crates/conductor-cli/src/paths.rs` (`:91-95`, grep) — `agent_log_path`, the sibling derivation.
- `conductor-0.2.0/verification-matrix.json` — `v2-24` entry + its four `notes` narratives.

## Graph impact (from the code-graph query)
Query: `SELECT symbol, crate, kind, file FROM symbol WHERE name IN ('journal_conformance',
'agent_log_path','console_suppressing_flags') AND kind <> 'meta';` (plane `rust`, trace at
`{run_dir}/tree-query-2026-09-08-webview2-runtime-152-installed-in-job.json`).

- **`agent_log_path`** — `fn`, `conductor-cli`, `crates/conductor-cli/src/paths.rs`. Resolved.
- **`console_suppressing_flags`** — `fn`, `conductor-verify`, `crates/conductor-verify/src/spawn.rs`.
  Resolved.
- **`journal_conformance`** — **0 rows, and this is a query-shape fact, not an index gap.** It is an
  integration-test TARGET (`crates/conductor-run/tests/journal_conformance.rs`, confirmed present on
  disk, 11 482 bytes), invoked as `cargo nextest run -p conductor-run --test journal_conformance`; the
  test binary's name is not a workspace symbol. Two of three probe names resolved, so the DB is live —
  per the cookbook's rule, a 0-row is only *consulted-but-no-match* once the symbol is confirmed indexed,
  and here it is confirmed **not** to be a symbol at all.

**Blast radius: none.** The chunk's modify-set is `.github/workflows/ci.yml`, which is on no indexed
plane. No Rust or TS symbol changes signature, so there is no caller set to thread.

## Patterns detected
- **Handle-named boolean precondition** (`ci.yml:290-300`): prints `[precondition] CONDUCTOR_MSEDGEDRIVER:
  resolved` / `NOT RESOLVED` and `exit 1` — handle NAME + state, never the resolved path. The model for
  the signature gate's own output.
- **Diagnostic-vs-gate separation is structural** (`ci.yml:265`, `:316`, `:435`): `continue-on-error: true`
  appears at exactly three sites, all diagnostics or the diag upload. The two gate steps — "A11y routine
  arm" (`:284`) and "A11y violation-JSON conformance gate" (`:446`) — carry none.
- **Version strings are print-safe** (`ci.yml:263-283`): the versions diagnostic reads the EdgeUpdate
  registry client key and emits a bare version string, which is outside the host-path ban.
- **Probe (a) is already correctly shaped** (`ci.yml:337-352`): polls once per second for 90 s rather than
  sleeping once, which is what separates "the loader ignores the handle" from "it is slow" from "never".
  It needs no logic change — only a different runtime underneath it.
- **Conformance by REUSE, not re-listing** (`ci.yml:441-453`): the a11y record is asserted by pointing
  `conductor-run`'s `journal_conformance` at `runs/a11y`, guarded by an explicit empty-dir `::error::`
  so a leg that produced no record cannot pass silently.
- **Remote-debug enablement is CI-confined by construction** (closes the design extract's question):
  `crates/conductor-tauri/Cargo.toml` enables no `devtools` feature, `tauri.conf.json` carries no
  `devtools` / `remote-debug` key, and **neither `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` nor
  `WEBVIEW2_USER_DATA_FOLDER` is set by any shipped Rust/TS/JSON source** — both exist only inside the
  workflow's diagnostic step. So design's "must not reach the shipped build" criterion is already true
  and this chunk's job is to keep it that way, not to establish it.

## Conventions to follow
- **Anchor CI by step NAME, never by `ci.yml` line coordinate** — test-plan §9 (Matrix builds) and
  a11y-plan §3 (Configuration). This chunk's own insertion shifts every later line, so the convention is
  self-enforcing here. `scope.md` was corrected accordingly at this step.
- **Read the PRINTED verdict, not `$?`** — `verification-harness.md:58` (extended 2026-09-07): `--e2e`
  captures wdio output to `$RUNS_DIR/a11y-e2e.log`, reads the exit from the BARE command, and asserts
  three things: zero on the `Spec Files:` failed count (the line **omits** the `failed` term entirely when
  nothing failed), a skip tally within the expected set (mocha's `2 skipped` plus per-spec `-` markers —
  **never** the token `pending`, which appears nowhere in the output), and the
  `[webview2 <version> windows]` banner. Expected-skip set = the two live-hold subjects
  (`accessibility.e2e.ts:295`, `:301`); a **third** skip means fixture seeding failed.
- **Both-shells parity is environment-conditioned** — `verification-harness.md:17`, amended 2026-09-07: a
  CI-only environment change can diverge the CI leg from the dev host with **zero script delta**. That is
  precisely this chunk's shape, so a divergence here is an environment fact, never a script defect.
- **Census twice, pattern from what the leg can START** — `verification-harness.md:58` (extended
  2026-09-08). Directly binding on any dev-host validation: the prior chunk's pre-census omitted
  `msedgewebview2.exe` and six had no baseline, and **stopping msedgedriver does not kill the app it
  launched**, so each of its three validation runs orphaned a `conductor-tauri.exe` plus webview children.
- **`--features tauri/custom-protocol` decides the bundle, not the profile** — `verification-harness.md:57`
  (corrected 2026-09-01): a bare `cargo build --release -p conductor-tauri` still emits `cargo:rustc-cfg=dev`
  and opens on `devUrl`. Both harness shells already pass the feature for `--e2e`; the routine arm's own
  step performs that build (`ci.yml:311-313` comment), which is why the isolation diagnostic must run
  AFTER it — there is no binary to launch before.

## New files to create
- `conductor-0.2.0/chunks/2026-09-08-webview2-runtime-152-installed-in-job/evidence/probe-verdict.md` —
  the either-way verdict record: which branch the run landed on, the run id, the `[diag] (a)` line, and
  the pre/post runtime versions. Host-path-free.

## Files to modify
- `.github/workflows/ci.yml` — the `a11y` job only:
  1. **new gate step** installing WebView2 runtime 152+ (fetch → `Get-AuthenticodeSignature` → install),
     placed before "WebView2 driver + runtime versions (diagnostic)"; prints the runtime version
     **before and after** so a silent no-op is visible;
  2. **one line** added to the staging `foreach` in "WebView2 session isolation (diagnostic)" so
     `runs/logs/conductor-tauri.jsonl` joins the staged diagnostics.

No other file changes. No crate, scenario, contract, script or spec-master edit. **Caller threading: N/A** —
no symbol's signature changes, no value threads, no registry/allowlist entry is required by the workflow
edit. **Seam facts: N/A** — no dependency is added in any manifest and no type crosses a package boundary.

## Open questions
- **Is the in-job installer execution a SIXTH governed spawn form under security-plan §Code Patterns rule
  (b)?** The rule is a verbatim count of governed forms, currently five; it was widened four → five once
  before as an operator-ratified boundary widening with **no routine rule minted**, so a sixth escalates
  again. Research confirms the premise is real — the `a11y` job performs no non-loopback fetch today
  (the only `Invoke-WebRequest` is loopback `127.0.0.1:9515`), so this genuinely is a new external-input
  and execution crossing. → blocks: **plan-decision** (P5 review item; the ruling is the operator's, and
  the registration itself is a wrap Expected-amendment, never a phase edit).
- **Does a dev-host validation run of the new step precede the single budgeted push?** The prior chunk's
  precedent says yes and it paid for itself (two probe defects caught that would have failed silently
  under `continue-on-error` and consumed the push). But the install half cannot be validated here without
  actually changing this host's WebView2 runtime, which is not something to do casually. → blocks:
  **implementation-scope** (what subset of the step is dev-host-validatable — the signature check and the
  version-print logic, plausibly not the install itself).
