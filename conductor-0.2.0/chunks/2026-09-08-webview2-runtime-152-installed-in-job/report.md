# Report — 2026-09-08-webview2-runtime-152-installed-in-job

**Chunk:** WebView2 runtime 152+ installed in-job — vary the one variable every passing case shares and the
failing case lacks; `v2-24` claimed or deferred.
**Date:** 2026-09-08
**Commits:** none since `last_wrap` (2026-09-08T17:07:54Z) other than `115b71b chore(session): no chunk
wrapped — session 129`; this chunk's work is uncommitted at report time and rides this wrap's commit.

## Changes (structured — detectors read this)

- **Files:** `.github/workflows/ci.yml` (the `a11y` job only). Basis: `git status --short` — one tracked
  source file modified; `git status --porcelain` carries **no `.rs`, no `.ts`, no `.toml`, no lockfile**.
- **Symbols / APIs:** no code symbol added, changed or removed on either indexed plane (the change surface is
  YAML). **One new outbound network endpoint:** an HTTPS GET to Microsoft's Evergreen bootstrapper link
  (`go.microsoft.com/fwlink/p/?LinkId=2124703`), **CI-job-scoped only — no shipped binary reaches it**. No new
  env var is set or read; the step reads the pre-existing runner-process variable `RUNNER_TEMP` in the step
  SHELL (`$env:RUNNER_TEMP`), never via a `${{ env.* }}` expression. No port is bound. Conductor's shipped
  trust boundary (no inbound listener; `:4317` the sole deliberate bind) is untouched.
- **Crates / modules:** none — the nine-member workspace roster is unchanged.
- **Dependencies:** **no Cargo or npm dependency added.** `Cargo.lock` and `package-lock.json` are
  byte-unchanged and `cargo audit` re-measured **562 crate dependencies**, identical to the
  `2026-09-07-dependency-polish` figure. **But a NEW DEPENDENCY CLASS enters the `a11y` job:** a third-party
  binary (the WebView2 Evergreen bootstrapper, and through it the runtime it installs) **fetched and executed
  at CI time**, which appears in no lockfile and is bounded by **no existing supply-chain gate** — `cargo
  audit` / `cargo deny` scan the Rust crate graph, `npm audit` the frontend tree, and neither sees this.
  Its admitting control is the `Get-AuthenticodeSignature` gate (status `Valid` **and** an
  `O=Microsoft Corporation` signer, evaluated **before** execution, non-zero exit otherwise).
- **Schema / config:** none — no migration, config key, violation schema or scrub/redaction shape changed.
- **Spec-master edits:** none at P1 (P2 owns any).
- **Counts / qualifiers moved:** **none — verified.** The `a11y` job goes 11 → 12 steps, but no master states
  that count: `test-plan-amendments.md:421-423` (item 4, 2026-09-07) explicitly records the job's step set as
  **no longer closed**, so a new step moves no documented value and owes no §9 stage-table row. The advisory
  count read 1242 (was 1239 at 2026-09-07) — that is the external RustSec DB's own movement over a
  porcelain-clean local copy, not a project value.
- **Dev-tool versions:** none. The bootstrapper was downloaded to the dev host for signature validation,
  **never executed**, and deleted; this host's WebView2 runtime is unchanged at `152.0.4191.66`.
- **Harness / gate surface:** **two changes, both inside the `a11y` job.** (1) A new **GATE** step
  `Install WebView2 Evergreen runtime 152+ (gate)` between `actions/setup-node@v4` and
  `WebView2 driver + runtime versions (diagnostic)`: prints the runtime version before and after, fetches over
  HTTPS, gates on Authenticode, installs via `Start-Process -FilePath <bootstrapper> -ArgumentList '/silent',
  '/install' -Wait -PassThru` (array-form argv, no shell string, no `Invoke-Expression`), then asserts the
  post-install **major ≥ 152** and exits non-zero otherwise. It carries **no `continue-on-error`** and no
  `if:`. (2) One guarded path added to the staging `foreach` in `WebView2 session isolation (diagnostic)` so
  `runs/logs/conductor-tauri.jsonl` joins the staged diagnostics. Basis: `grep -n 'continue-on-error'` returns
  4 lines of which **3 are directives** (`:325` versions diagnostic, `:376` session isolation, `:499` diag
  upload — the 4th is prose inside a comment), **none on a gate path**; `grep -n 'http://'` returns exactly
  **1** hit, the pre-existing loopback `127.0.0.1:9515` POST; `grep -n 'SkipCertificateCheck\|Invoke-Expression'`
  returns **0**; `runs-on: windows-2025` intact at `:223`.
- **Cross-project / external claims:** the WebView2 **runtime version** facts, whose ground truth lives on the
  GitHub-hosted `windows-2025` image and on this dev host, not in this repo. Runner = `151.0.4129.101` (basis:
  the prior chunk's run 34234558853, recorded in `chunks/2026-09-08-hosted-runner-webview2-session/evidence/
  probe-verdict.md`); dev host = `152.0.4191.66` (basis: EdgeUpdate registry `pv` read, measured this session).
  The Evergreen bootstrapper's identity and signature are likewise external — measured this session as
  1 783 000 bytes, signature `Valid`, subject `CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond,
  S=Washington, C=US`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none. The install step's sufficiency is **unmeasured
  by construction** — it is a probe whose verdict requires a CI run that follows this commit — which is
  distinct from a shipped-but-insufficient fix.
- **Spec claims disproved by measurement:** **one, and its precise scope matters.**
  `test-plan.md:455` (§9 Pipeline structure, Lint row) names `cargo fmt --check` beside clippy as the Lint
  stage, and `:477` / `:507` list "fmt error" / "fmt failure" as build-failure conditions — **three sites,
  basis `grep -rn 'cargo fmt' .andromeda/*.md`**. Measured this chunk: `cargo fmt --check` exits **1** with
  **282 flagged sites across ~40 files** spanning nearly every crate, and `grep -n 'fmt'
  .github/workflows/ci.yml` returns **0** — no CI job has ever run it. The invocation is not the cause
  (`rustfmt.toml` is `edition = "2024"`, stable-only; toolchain pinned 1.95.0 with rustfmt as a component;
  the diffs are ordinary edition-2024 import ordering and line wrapping). **What is disproved is the
  satisfiability of that gate TODAY, not §9's target.** Per the `2026-09-06-coverage-completeness-gate`
  narrow operator resolution (retire only claims a job exists TODAY; leave target-arrangement descriptors
  standing), `:455` is a target-state row awaiting its implementing chunk — so the disposition is an
  **owner, not a retirement**: see *Decisions & corrections* item 4. **Detectors: do not propose retiring
  `test-plan.md:455`.**
- **Expected amendments (from plan):** three entries, all **carried**.
  1. **security-plan §Security Anti-Patterns → Code Patterns rule (b) AND §Dependency Security** — carried;
     its facts are the *Dependencies* and *Harness / gate surface* bullets above. Sites located by
     `grep -nE 'five' .andromeda/security-plan.md | grep -iE 'form|spawn'` → **1 hit, `:357`** (the two-rule
     spawn-class body carrying the governed-form enumeration), and
     `grep -nE '^#+.*Dependency Security' .andromeda/security-plan.md` → **1 hit, `:165`**. Both hits are in
     security-plan; no other master is named owner. Direction: register the in-job installer execution as a
     governed spawn form (or record why it is not one), **and** record the CI-fetched runtime as a dependency
     class whose admitting control is Authenticode verification, stating whether always-latest Evergreen is
     acceptable or wants pinning. The §Dependency Security half is the **operator's P5 review addition**.
  2. **architecture.md §Occupied Resources — Environment variables AND Ports** — carried; its facts are the
     *Symbols / APIs* bullet. Sites located by
     `grep -nE 'egress target|^\*\*Ports|^\*\*Environment variables' .andromeda/architecture.md` → **3 hits**:
     `:144` (**Ports** header), `:145` (`127.0.0.1:4317` recorded as an "OTLP/gRPC **egress target**" — the
     registry demonstrably registers egress, which is what makes the new endpoint's absence a gap), `:178`
     (**Environment variables** header). Direction: register the Evergreen fetch as a **CI-job-scoped egress**,
     explicitly not a shipped-binary one; and record the install step's own basis beside the existing
     `WEBVIEW2_*` pair, whose registered lifetime reads "that one `continue-on-error` diagnostic step".
  3. **test-plan §6 (Mode cell) / §9 (Matrix builds) and a11y-plan §1 / §11** — carried as **owed-but-not-yet-
     due**: these sites wait on the probe's CI result, which follows this commit, so the fact they need does
     not exist yet. Sites located by `grep -n '152' .andromeda/test-plan.md` → **3 hits** (`:56`, `:307`,
     `:469`) and `grep -nE '152|session creation' .andromeda/a11y-plan.md` → **5 hits** (`:115`, `:333`,
     `:457`, `:465`, `:516`). Both masters carry hits, so both are legitimately named. Direction when the run
     lands: extend the measured runtime × driver **PAIR SET** with this run's member — never substitute a
     fresh literal for the set.
- **Coverage of new surfaces:**
  - `Install WebView2 Evergreen runtime 152+ (gate)` (CI job step; no shipped-code surface) → validation
    **n/a** (no scenario config; its inputs are a fixed URL and a fixed argv) · instrumentation **n/a**
    (a workflow step logs to the job log, not to the obs stream) · PII **redacted✓** (every emitted line is a
    version string, a status token, or a fixed label; **no path is printed** — verified by inspection of all
    nine `Write-Output` sites) · tests **unrunnable-here** (its decisive proof is a hosted-runner CI run that
    necessarily follows this commit; its *logic* was dev-host validated — see Outcome) · a11y **n/a** ·
    tokens **n/a**.
  - `runs/logs/conductor-tauri.jsonl` staged into `session-diag` (diagnostic artifact) → PII **redacted✓**
    (the file is Conductor's own self-obs sink, already governed by the obs field-allowlist; the staging copies
    it by leaf name into a temp dir and prints only a file COUNT) · instrumentation **n/a** · tests
    **unrunnable-here** · validation / a11y / tokens **n/a**.

## Deviations from intent

1. **The plan's one New file was not written.** `chunks/{marker}/evidence/probe-verdict.md` is listed under
   *Codebase touchpoints → New files*, and the plan's own step 10 says to write it **after the CI run** — which
   cannot occur until this change is committed and pushed. *Justification:* deferred by the plan's own design,
   not omitted; the artifact's content does not exist yet. Its owner is the follow-up entry that reads run (a).
2. **An acceptance criterion was authored that this chunk cannot satisfy.** The plan's criterion
   "(tests) `cargo clippy …` **and `cargo fmt --check`** return 0" bundles a tree-wide property into a chunk
   whose modify-set is one YAML file. *Justification:* none — this is an authoring defect. It came from the
   tests extract's faithful reading of test-plan §9 without checking whether the tree satisfies it, which is
   the standing 2026-08-11 learning (a spec describes TARGET state) recurring. See *Outcome* for how the
   criterion is dispositioned.

## Decisions & corrections

1. **Operator, P5 review — Expected amendment widened to two surfaces.** The security-plan entry must name
   **§Dependency Security** beside §Code Patterns rule (b): a signed installer fetched and executed at CI time
   is a new supply-chain dependency of the `a11y` job and the Authenticode check is its control, so wrap's
   escalation judges both surfaces, not only the spawn-form count.
2. **Operator, P5 review — read matrix entries through the tool.** An entry's full fields come from
   `matrix.py show --dir conductor-0.2.0 --id v2-24`, not inline python. Applied; the tool surfaced `title`,
   `observed_gap` and `requirement`, and `requirement` ("…and gated in CI") proved **stricter** than the
   `acceptance` the decision had been reasoning from — strengthening the declined claim rather than changing it.
3. **`v2-24` was NOT claimed at phase P5, deliberately** — claim-reachability: the acceptance requires the
   specs to run in the `a11y` CI job, whose run follows the push, so no step in the chunk's lifecycle can set
   `ref`. The identical cap was claimed and then un-claimed one chunk earlier for exactly this reason. A
   failed-concretization `notes` line was written (+2894 chars; status/ref/acceptance untouched).
4. **Operator wrap directive — the red `cargo fmt --check` has NO in-diff cause and does not block this
   chunk.** Treated under the playbook's external-decay rule (`playbook.md:93`), whose principle is that a red
   with no in-diff cause is not the discovering chunk's drift and must always produce an **owner**. Root cause
   supplied by the operator, measured pipeline-side (backlog **W90**): `rustfmt.toml` (edition 2024) shipped
   **2026-09-05** with only `commands.rs` re-sorted, and CI never gained a fmt step. **Owner: a route entry** —
   a workspace formatting pass plus `cargo fmt --check` as a CI gate. A new chunk ahead is a **trajectory**
   decision, so it is presented at route-resolve (P5) for the operator's word; placement is not wrap's to
   decide.
5. **Operator wrap directive — `v2-24` stays `planned` at this wrap.** The probe's CI run follows the push, so
   the claim/defer fork belongs to the follow-up entry that reads run (a). Recorded in the **handoff**, not in
   the ledger.
6. **Two self-authored gate defects were caught by running them, not by review.** At phase P4 a security
   Test Command used a PCRE negative lookahead under `grep -E`, which matched nothing and exited **0** — a gate
   that could never fail; rewritten as a positive probe asserting exactly one known loopback hit. At implement
   P2 the fmt gate proved a gate that could never pass. Both sat in the same Test Commands block; the P5
   mechanical checks verify a command's presence and shape, never its satisfiability.

## Outcome

**Acceptance criteria, re-asserted against the DIFF** (not against the plan's text):

| # | criterion | verdict against the diff |
|---|---|---|
| 1 | (security) signature evaluated **before** execution; non-`Valid`/non-Microsoft signer exits non-zero; no `continue-on-error` on either | **MET** — the `Get-AuthenticodeSignature` block precedes `Start-Process`; both branches `exit 1`; dev-host validated on both arms (a tampered copy returned `UnknownError` and was rejected) |
| 2 | (security) HTTPS with certs intact, fixed program + fixed argv; no `-SkipCertificateCheck`, `http://`, `Invoke-Expression`, shell-string assembly | **MET** — `grep` returns 0 for the banned tokens and exactly 1 `http://` (the pre-existing loopback POST); the install is `Start-Process -FilePath … -ArgumentList '/silent','/install'` |
| 3 | (security) `cargo audit` 0 and `cargo deny check advisories bans licenses sources` 0 over a committed un-drifted `Cargo.lock`; a red adjudicated by the porcelain probe first | **MET** — porcelain 0 lines (HEAD `8a1eb4f9`), audit exit 0, deny `advisories ok, bans ok, licenses ok, sources ok` |
| 4 | (arch) `runs-on: windows-2025` explicit; **no gate step carries `continue-on-error`**, count stays 3 on diagnostics/upload | **MET** — `:223` intact; 3 directives at `:325`/`:376`/`:499`, all diagnostics or the diag upload |
| 5 | (arch) runner-process vars read in the step SHELL, never `${{ env.* }}`; no `CONDUCTOR_*` claim | **MET** — the step uses `$env:RUNNER_TEMP` only |
| 6 | (tests) `cargo nextest run --workspace --profile ci` returns 0 | **MET** — 902 tests run, 902 passed, 0 skipped, across 55 binaries |
| 7 | (tests) `cargo clippy --workspace --all-targets -- -D warnings` **and `cargo fmt --check`** return 0 | **SPLIT: clippy MET** (exit 0, 0 findings); **fmt UNMET as authored** — see below |
| 8 | (obs) conformance gate + upload steps present, after the routine arm, byte-unchanged | **MET** — steps at indices 10 and 11 with `CONDUCTOR_RUNS_DIR: runs/a11y` unchanged |
| 9 | (obs) pre/post versions and `[diag] (a)` grep-readable; no runner path in committed evidence | **CI-produced — not yet assertable** (no local producer by construction; the evidence file is deviation 1) |
| 10 | (a11y) routine arm unchanged apart from the preceding install | **MET** — the routine-arm step is byte-unchanged; only its predecessor is new |
| 11 | (a11y) the probe's outcome recorded either way | **CI-produced — not yet assertable**; owner is the follow-up entry |
| 12 | (layouts) no interactive prompt/elevation in the headless path; signature refusal prints a host-path-free sanitized line and stops | **MET** — `/silent` install, `-Wait`; refusal prints `[precondition] WEBVIEW2_BOOTSTRAPPER_SIGNATURE: …` with no path |
| 13 | (design) no token/lamp/caption added; remote-debug enablement confined to CI | **MET** — no UI surface touched; neither WebView2 env handle is set by any shipped Rust/TS/JSON source |

**Criterion 7's fmt half is UNMET as authored and is NOT matrix-linked** (this chunk claimed 0 capabilities),
so it routes as a **P2 escalation** — pre-dispositioned by wrap directive item 1: no in-diff cause (282 sites
reproduce at HEAD; `git status --porcelain` carries no `.rs`), therefore not this chunk's drift, does not block
it, and its owner is a route entry presented at P5. The criterion itself was mis-authored (deviation 2): it
asserted a tree-wide property a one-YAML-file chunk cannot affect.

**Gates green (commands run):** `git -C $CARGO_HOME/advisory-db status --porcelain` (0 lines) · `cargo audit`
(0) · `cargo deny check advisories bans licenses sources` (0) · `npm --prefix crates/conductor-tauri/ui run
build` (0) · `cargo clippy --workspace --all-targets -- -D warnings` (0) · `cargo nextest run --workspace
--profile ci` (0, **902/902**) · the YAML parse and four structural greps. **Red:** `cargo fmt --check` (1) —
dispositioned above. **The absorbed PREREQ is discharged:** the prior chunk deferred exactly
`nextest --workspace` and `clippy`; both ran green here, closing the chain opened at
`2026-09-08-hosted-runner-webview2-session`.

**Smoke:** skipped — no boot-path or UI-surface change (the sole touchpoint is a workflow file; the plan lists
no `agent-run.sh`). The plan states plainly why a bare `agent-run.sh status` was not listed: it exits 0 while
proving nothing.

**Dev-host validation (plan step 8), the substitute for an unreachable CI proof:** the step's logic was
exercised locally with the installer deliberately NOT run — this host is at `152.0.4191.66` and CI at
`151.0.4129.101`, so a local install would exercise install-over-newer (a no-op) and could not measure the CI
upgrade path. Validated: the registry read (returned `152.0.4191.66`), the HTTPS download (1 783 000 bytes),
the signature PASS branch (`Valid`, `O=Microsoft Corporation`), the signature **FAIL** branch (a byte-corrupted
copy returned `UnknownError` — proving the gate catches tampering rather than merely passing on a good file),
and the version-major assertion across four inputs (**151 → exit 1**, 152/153 → pass), which is precisely the
no-op detection the design turns on.

**Outcome basis:** implement's P4 report as given, **plus the operator's wrap directive** (items 1–3), which
changed the fmt disposition from "surfaced, awaiting decision" to "external-decay treatment; owner = a route
entry presented at P5", fixed `v2-24` at `planned` for this wrap with the fork recorded in the handoff, and
specified the P7.3 tool form. No other post-implement artifact informs this report.

**Process hygiene** (implement P4's census, **re-measured at this wrap** — the host process list is readable):

| process | started by | final state |
|---|---|---|
| `powershell.exe` (validation script) | this run | terminated |
| cargo / npm / rustc (gate suite) | this run | terminated |
| `msedgewebview2` ×6 | **not this run** — all six started 2026-08-13 18:06, 26 days ago (basis: `Get-Process \| Select-Object Id, StartTime`, not the process-name pattern) | left running: pre-existing; the operator's |
| `MicrosoftEdgeWebview2Setup.exe` (downloaded, never executed) | this run | file removed from TEMP |

A post-clean census of this run's four process families (`conductor-tauri`, `msedgedriver`, `tauri-driver`,
`MicrosoftEdgeWebview2Setup`) returns **0**.
