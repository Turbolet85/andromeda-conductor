# Report — 2026-09-11-hosted-runner-endpoint-cause-probed

**Chunk:** Hosted-runner endpoint cause probed — three named one-line probes on the hosted `windows-2025` image, added as `continue-on-error` diagnostics to the `a11y` job and read from a real CI run; diagnose-only.
**Date:** 2026-09-11
**Commits:** `b7ef1bd` chore(ci): add WebView2 cause probes to the a11y job (diagnostic) — **authored by the operator mid-implement** (see Deviations)

## Changes (structured — detectors read this)

- **Files:** `.github/workflows/ci.yml` (+14, one step added) · `scripts/webview2-cause-probe.ps1` (new, 141 lines) · `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-probed/evidence/reading.md` (new, committed) · `runs/webview2-probe/{control-devhost.txt, a11y-job.log}` (new, **gitignored** per `.gitignore:17 /runs/`). Basis: `git show --stat b7ef1bd` (155 insertions, 2 files) + `git status --short`.
- **Symbols / APIs:** No Rust/TS symbol added, changed or removed — the chunk compiles nothing. One new **shell entry point**: `scripts/webview2-cause-probe.ps1`, invoked only by `ci.yml`'s `a11y` job (sole caller; it is not wired into either `agent-run` harness variant and adds no 6th harness command). **No env var is added, read or set** — deliberately: probe (3) uses `[System.Diagnostics.Process]`, `[Environment]::UserInteractive` and `[Security.Principal.WindowsIdentity]` rather than any `$env:` read. **No port, socket, or network egress is added**; the job's CI-scoped outbound target set stays at exactly one (the Evergreen fwlink).
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** **none** — `Cargo.lock` and `package-lock.json` both byte-unchanged; zero new packages in either tree. The probe uses only in-box PowerShell cmdlets and .NET types.
- **Schema / config:** none. No migration, no config key, no violation-schema or scrub-shape change.
- **Spec-master edits:** none at authoring time (P2 owns any).
- **Counts / qualifiers moved:** **none — verified.** No documented derived value (count, tally, qualifier) changed. Basis: the chunk adds a step and a script and moves no stated figure; the one figure it *produces* (the reading) is new, not a moved value.
- **Dev-tool versions:** none — no external CLI installed or upgraded.
- **Harness / gate surface:** **`.github/workflows/ci.yml`'s `a11y` job gains ONE step**, named `WebView2 cause probes (diagnostic)`, carrying `if: always()` + `continue-on-error: true` + `shell: pwsh`, placed between the existing `WebView2 session isolation (diagnostic)` and `Upload session diagnostics` steps. No existing step's logic, ordering, `if:` or `continue-on-error` posture changed — in particular the `Install WebView2 Evergreen runtime 152+ (gate)` step keeps no `continue-on-error` and no `if:`, and still performs its `Get-AuthenticodeSignature` check before the binary runs. The job's diagnostic count goes 2 → 3. No `agent-run.{sh,ps1}` change; the 5-command surface, the status envelope and the JSONL shapes are untouched.
- **Cross-project / external claims:** none — no fact here has ground truth outside this repo. (The readings are about the GitHub-hosted `windows-2025` image, read from this repo's own CI run `34586959536`, not from another repo.)
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** **the probe step's PLACEMENT.** The step is correct and shipped, and it produced two of its three readings; it did not produce the third, because it sits *after* the `WebView2 session isolation (diagnostic)` step, which stops the app it started. Measured from the captured log: that step's last line is stamped `10:14:58.9421153Z` and the probe's completion marker `10:14:59.5867824Z` — **0.645 s later** — while the same isolation step's own `tasklist` at `10:13:21.76Z` shows `conductor-tauri.exe` (pid 8984) and three `msedgewebview2.exe` alive. What it did resolve: candidates (1) and (3). Remainder owner: the follow-up Epoch-1 entry minted at P5.
- **Spec claims disproved by measurement:**
  1. **`architecture.md` §Established Decisions [CI/CD] names "a hosted-image policy or session property" as the leading unmeasured candidate pair.** Both halves are now MEASURED and neither holds. Policy: all five probed Edge/EdgeUpdate/EdgeWebView keys (machine and user hives) are **ABSENT on the hosted runner and ABSENT on the dev host**, so no policy difference exists to explain a behavioural one. Session: the runner is **`SessionId 2`, `UserInteractive: True`** — not session 0, not a service context — corroborated independently by the isolation step's own `tasklist` showing the app and its webview children on "Console" session 2. Evidence: CI run `34586959536`, job `A11y gate`, step `WebView2 cause probes (diagnostic)`; control from the same script on the dev host.
  2. **`.andromeda/residuals.md`'s `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` entry states the routine arm's red "has moved to tauri-driver never listening on `:4444` (four refused POSTs inside ~1.5 s against a 6.9 s driver bind)".** This run does not reproduce it: the captured log holds **ZERO `ECONNREFUSED`** (basis: `grep -cE 'ECONNREFUSED' runs/webview2-probe/a11y-job.log` → 0; operator re-derived the same count independently), and wdio **connected to the driver** at `10:09:50.296Z` and reached session creation, failing on `session not created: DevToolsActivePort file doesn't exist` at `10:10:50.734Z` and again via the driver-alone probe at `10:14:27.377Z`. A bind refusal is intermittent by nature, so the earlier observation is not refuted for the runs it was taken from — but the framing must not be inherited. Disposition: the residual annotation is re-worded at P5 to say the refusal is INTERMITTENT and that `v3-02` re-measures rather than inherits (operator directive, item 3).
  3. **Not a disproof but the chunk's own premise, recorded here for the coverage gate:** `v3-01`'s acceptance asserts the three readings are together *sufficient to decide* remediable-vs-permanent. They are not — see Outcome. Disposition: **UN-CLAIM** at P7 (operator directive, item 1), not a refine.
- **Expected amendments (from plan):**
  - `architecture.md` §Occupied Resources — Environment variables (register `TEMP` + `LOCALAPPDATA`) — **carried**; its fact is the Symbols/APIs bullet's "no env var is added" plus the pre-existing gap: `ci.yml` reads `$env:TEMP` at `:396`/`:440` and `$env:LOCALAPPDATA` at `:490`/`:501` on the same shipped-artifact-READS basis that registered `RUNNER_TEMP` and `EDGEWEBDRIVER`. Located by `grep -oE '\$env:[A-Za-z_][A-Za-z0-9_]*' .github/workflows/ci.yml` (7×`RUNNER_TEMP`, 4×`EDGEWEBDRIVER`, 2×`TEMP`, 2×`LOCALAPPDATA`, 2× each `WEBVIEW2_*`, 1×`CONDUCTOR_MSEDGEDRIVER`) against `grep -nE 'LOCALAPPDATA|[^_]TEMP' .andromeda/architecture.md` → **0 hits**. **Pre-existing, not introduced here.**
  - `architecture.md` §Infrastructure Patterns — directory tree (`scripts/` gains a non-harness script) — **carried**; fact in Files. Located: `architecture.md:216-217` lists `scripts/` with `agent-run.sh` as its only entry (1 hit).
  - `architecture.md` §Established Decisions [CI/CD] — record the readings and update the OPEN cause's status — **carried**; fact in Spec claims disproved #1. Located: `grep -c 'hosted-image policy or session property' .andromeda/architecture.md` → 1 hit.
  - `test-plan.md` §9 Pipeline structure (E2E-webview row) — the job's diagnostic step set 2 → 3 — **carried**; fact in Harness / gate surface. Located: `test-plan.md:460` states the literal ("plus (2026-09-07) two `continue-on-error` DIAGNOSTIC steps…"), **1 hit**. **`a11y-plan.md` §9 is NOT a co-owner and was wrongly named here at authoring** — `grep -ci diagnostic .andromeda/a11y-plan.md` → **0 hits**; that doc states no diagnostic-step count at all. Corrected at P2 Validate check 5; the fact has exactly one owner and no amendment is owed to a11y-plan.
  - `matrix#v3-01 notes` — **ledger-note — owner P7.3**, and additionally an UN-CLAIM per directive item 1.
- **Coverage of new surfaces:**
  - `scripts/webview2-cause-probe.ps1` (shell entry point, CI-only) → validation **n/a** (no external input: it takes one optional caller-supplied `-OutFile` from the workflow, never operator or network data) · instrumentation **n/a** (obs-plan §1 classifies the CI pipeline Not-instrumentable; the probe mints no span and no `tracing` line by design) · PII **redacted✓** (prints value NAMES not contents, module NAMES not `FileName`, registry keys rendered colon-free; proven by the committed hygiene gate reading `0`) · tests **✗ by design** — a diagnose-only probe asserts nothing, and its correctness is evidenced by running on both the dev host and the hosted runner in the same session · a11y **n/a** (no UI) · tokens **n/a** (no UI).
  - `ci.yml` `a11y` job step `WebView2 cause probes (diagnostic)` → validation **n/a** · instrumentation **n/a** · PII **redacted✓** · tests **n/a** (a `continue-on-error` diagnostic cannot be asserted on without becoming a gate) · a11y **n/a** · tokens **n/a**.

## Deviations from intent

1. **The commit and push happened mid-implement, and were the OPERATOR's act.** `v3-01`'s acceptance needs a reading from a CI run of *committed* code, and CI fires only on push, while `/andromeda-implement`'s constraints reserve committing for wrap. The skill **correctly refused to perform it itself** and surfaced the tension; the operator made both commits and the run resumed at the run-id gate. Recorded as a deviation with that attribution — **not a process defect on the skill's part** (operator directive, item 4).
2. **P3 smoke recorded the P2 result rather than driving `agent-run.sh run`.** That verb executes the workspace nextest/doctest/clippy this chunk deferred under the source-delta rule; re-running them would have voided the deferral for a chunk with zero compiled-source delta. Stated, not skipped silently.
3. **No `ref` / `status:implemented` written by implement.** The acceptance is not satisfied as written, and setting `implemented` over it is the silent flip the skill and the matrix contract both forbid. Surfaced instead; resolved at P7 as an un-claim.
4. **The plan's `role = 'smoke'` entry was vacuous for this chunk.** `bash scripts/agent-run.sh status` reads the newest envelope already on disk, so it passes identically whether or not this work happened — the envelope it printed (`latency_ms 35115`) belongs to a prior chunk. It was correctly listed to satisfy phase P5 check 4(2), but proved nothing here. Per operator directive item 4, a diagnostics-only chunk should instead take the letter's other arm — *"omitted if no boot-path, the absence stated in the section's prose"*. Routed to curation.

## Decisions & corrections

- **Operator directive (4 items) between implement's P4 report and this report**, and it changed the outcome: `v3-01` resolves as **UN-CLAIM, not refine** — the acceptance clause is what makes the capability worth holding, and this version exists to retire closures that rest on wording. The capability returns to the pool at its current status; the follow-up entry claims it when the measurement is whole.
- **Operator corrected my step-adjacency figures.** I reported `10:14:58.906Z → .582Z` = 0.68 s, having taken the third-from-last isolation line. The true last line is `10:14:58.9421153Z`; against the probe's marker `10:14:59.5867824Z` that is **0.645 s**. Re-derived from the log and corrected in `evidence/reading.md`.
- **I mis-read the `:4444` evidence twice before reading the hits.** A `-o` pattern first clipped the lines to zero, then a raw count of 7 looked like refusals; the 7 are `127.0.0.1:4444` URL mentions and there is no `ECONNREFUSED` at all. Settled only by printing the matched lines.
- **A gate authored at phase P5 nearly false-positived on its own subject.** The host-path anchor `[A-Za-z]:[\/]` also matches the PowerShell registry provider form `HKLM:\`. Resolved by probing in provider form and *reporting* in the colon-free reg.exe form — not by loosening the pattern.
- **The first evolve append of the implement `code` checkpoint failed loudly** with `Invalid \escape`, because two record values carry Windows backslashes. `evolve-system.md` requires the python-dict form with raw literals for exactly that case; the validated append refused the payload rather than landing a malformed line.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

- (`v3-01`) A recorded reading for each of the three named probes, each beside its dev-host control, together sufficient to decide remediable-vs-permanent — **UNMET, and routed.** The three readings exist and are recorded (including the null, recorded as a null, with its measured cause); two candidates are retired and one new lead named. The *decide* clause is not reached: probe (2) returned nothing, and elevation is a difference, not a demonstrated cause. Routed per operator directive to **un-claim** at P7 — not a refine, not a silent flip.
- (arch) Each added step carries `continue-on-error: true`; no existing step's logic, ordering, `if:` or `continue-on-error` changed; the install gate keeps its bare posture — **MET** (diff: one step added, zero existing lines modified).
- (arch) No second non-loopback outbound target, no port bound — **MET** (diff introduces no network call).
- (arch) The `a11y` job stays on `windows-2025`; the WebView2 runtime stays FLOATED — **MET** (neither line touched).
- (security) The governed harness-spawn form count remains SIX — **MET**: no probe constructs a child process; all three readings use cmdlets and .NET types.
- (security) The WebView2 fetch and install-gate steps keep no `continue-on-error`, keep the Authenticode check, keep the ≥152 floor — **MET** (unmodified).
- (obs/security/tests/a11y) Zero absolute host paths in the committed reading — **MET, proven not asserted**: the gate reads `0`.
- (obs) An absent key recorded as an explicit absent reading — **MET** (five `ABSENT` lines; three `no live process` lines).
- (obs) No span, no `tracing` line, no new telemetry-artifact path; existing uploads untouched — **MET**.
- (a11y/tests) Probes diagnose-only; `CONDUCTOR_A11Y_STRICT` + printed-verdict mechanism unperturbed — **MET**: the assertion parses wdio output captured to `$RunsDir/a11y-e2e.log`, a different stream from the job log the probes print to.
- (a11y) The reading names job and steps, states the run by id/event/branch/conclusion, attributes the red to the standing failure, draws no green/conformance/platform claim — **MET**.
- (tests) `agent-run.sh status` returns 0 and the workflow parses to exactly the three jobs — **MET** (see Deviations 4 on the former's vacuity).

**Gates** (the `[[gate]]` entries by `run`, in order):

| `run` | verdict |
|---|---|
| `cargo nextest run --workspace --profile ci` | **deferred** — `defer` key present; reason re-verified against the actual diff (zero `.rs`/`Cargo.toml`/`Cargo.lock`/toolchain delta). Not voided. |
| `bash scripts/agent-run.sh status` | exit 0 ✓ (`expect` exit 0) — vacuous for this chunk, see Deviations 4 |
| workflow YAML parse | exit 0 ✓ · `last line a11y,frontend,rust` held |
| probe script `-OutFile …control-devhost.txt` | exit 0 ✓ · `last line [probe] complete: 3/3 sections` held · artifact fresh 09:46:02Z |
| `gh auth status` | exit 0 ✓ |
| `gh run list … // error(…)` | exit 0 ✓ → `34586959536` (red pre-push by design; green after) |
| `gh run view <id> --log > …a11y-job.log` | **`leg = 'operator'`** — ran ONCE with `<id>` = `34586959536` substituted; exit 0, 1 894 939 B captured. **Never re-run at this wrap**; re-verified by this session's recorded artifact + a read-only `gh run list` query. |
| `grep -c "[probe] complete: 3/3 sections" …a11y-job.log` | exit 0 ✓ · `last line 1` held — the probes provably ran on the hosted runner |
| `grep -cE "[A-Za-z]:[\/]…" …evidence/reading.md` | exit 1 ✓ · `last line 0` held — the inverted expectation is this entry's green |

Smoke: recorded from the P2 gate; no boot-path or UI-surface change, so no app boot was owed.

**Outcome basis:** implement's P4 report, **as amended by the operator directive of 2026-09-11** (item 1 changed the `v3-01` resolution from refine-or-unclaim-open to un-claim; item 3 added the residual re-wording; item 4 re-attributed the commit deviation and routed the smoke-vacuity finding to curation). Post-implement artifacts named: `runs/webview2-probe/a11y-job.log` (1.89 MB, CI run `34586959536`), `runs/webview2-probe/control-devhost.txt` (962 B), `evidence/reading.md` (committed). A detector reading this report must not inherit implement's pre-directive framing.

**Process hygiene** (implement P4 census, re-measured here — the host process list is readable from this session):

| process | started by | final state |
|---|---|---|
| `powershell.exe` (probe gate) | this run | **terminated** |
| `gh` (auth / run-list / run-view) | this run | **terminated** |
| `gh run watch` (background) | this run | **terminated** — completed exit 0; 0 matches at census |
| `powershell` pids 11016, 29156 | **not this run** | left running — start times 2026-09-10, predate the run by a day; attribution by `StartTime`, per the harness census rule |
| CI run `34586959536` | the operator's push | completed on GitHub; no local process |

No process this chunk started survives. Nothing was left for the operator to stop.
