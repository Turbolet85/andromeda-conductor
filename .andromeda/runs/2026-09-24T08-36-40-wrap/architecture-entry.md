
## 2026-09-24-architecture-registries-compacted-under-the-read-cap — registries compacted under the read cap; the operator-instrument row gains a third member
**Section:** §Established Decisions and §Occupied Resources (each replaced whole) · §Stack and Technologies ("Operator instruments (host runtime)" row)
**Change:**
(1) Both registry sections were replaced verbatim by the chunk's drafts `compaction/{established-decisions,occupied-resources}.md`.
- Sizes:
  - §Established Decisions: 49 134 → 37 907 B.
  - §Occupied Resources: 48 859 → 37 929 B.
  - The limit is 38 115 B, which is 60 % of the 25 000-token Read cap at 2.541 B/token. Measured by `scripts/arch-registry-check.py measure --file .andromeda/architecture.md` → `registries: within target`.
- Kept: every label, registered name, port, crate and scoped qualifier.
- Left the body: dated narrative (chronologies, superseded readings, CI-run stories and elimination stories).
- Per-sentence accounting is in `conductor-0.3.0/chunks/2026-09-24-architecture-registries-compacted-under-the-read-cap/compaction/disposition-ledger.toml`: 579 rows (kept 352 · rewritten 150 · moved 68 · in-sidecar 9), and `check` PASSes.
- The 151 judgment rows were reviewed at this wrap, and 4 were corrected before apply:
  - a `CONDUCTOR_MSEDGEDRIVER` skip row anchored on the NVDA bullet was re-anchored on its own bullet;
  - [CI/CD] regains the dev-host green tally and its two-configuration basis;
  - [CI/CD] regains the three-leg basis of the integrity-label discriminator.
- The passages that left the body are the entries below, each headed by this marker, one per decision label or sub-registry. They are verbatim: either history, or the BEFORE wording of a sentence the body now states more briefly. History this sidecar already carried is not repeated; the ledger's `in-sidecar` rows name the entries that hold it.

(2) §Stack row: `scripts/arch-registry-check.py` is registered as the third committed operator instrument, and "Neither … neither" becomes "None … none".
**Why:** The body holds only current truth (this file's preamble), yet it had carried its own amendment log: 132 ISO dates, 13 CI run ids and 35 sha-like tokens across the two sections. It was also growing ≈1 KB a day at wrap toward the Read cap.

Cascade sweep over the seven masters, CLAUDE.md, `.claude/rules`, `.claude/docs`, playbook and drift-base:
- (a) The retired two-instrument wording: 1 hit, `.claude/docs/stack.md:40`, which was re-derived.
- (b) 4 696 quoted fragments of ≥ 3 words, of which 17 are present in HEAD's body and absent from the new one. None needed a change:
  - 8 are text between two separate code spans (a regex join, not a quote);
  - 4 are a master's own history with no arch citation on the line (security-plan :367, test-plan :471 ×3);
  - 4 sit in curation homes (CLAUDE.md :136, `verification-harness.md` Session Additions ×3);
  - 1 is a generic token.
- Every section citation into §Established Decisions / §Occupied Resources resolves to a claim the compacted body still states: 16 in security-plan, 5 in test-plan, 1 in obs-plan, 2 in a11y-plan, and 10 in the leaves.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Database]: passages moved out of the body

**Section:** §Established Decisions — [Database]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- The brief's stale `rusqlite 0.31 / SQLite ≥3.38` pin is ratified to 0.38.0 (a free maintenance/correctness win).

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [MCP Read-Back Client]: passages moved out of the body

**Section:** §Established Decisions — [MCP Read-Back Client]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- The hardened spawn (fixed program path + `.env(...)` data-dir + injection-reject) is unchanged.
- (The original rmcp-selection rationale → `architecture-amendments.md`.)

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Accepted Capability Set]: passages moved out of the body

**Section:** §Established Decisions — [Accepted Capability Set]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- This supersedes the original compile-time `001..=060` bound in `pid_format`, which the SUT outgrew.
- The `KNOWN_UNCLASSIFIED` residual ledger that briefly bridged the two is retired to `[]`.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Module Boundaries]: passages moved out of the body

**Section:** §Established Decisions — [Module Boundaries]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- the standalone per-seam BUILD now holds for every member, measured rather than gate-enforced.** Workspace feature unification can mask a feature a crate uses but declares only in `[dev-dependencies]`, so a crate compiles in the workspace and in `cargo test -p` while failing alone — the shape measured at HEAD 2026-09-04, when `cargo check -p conductor-verify --lib` was red on `tokio::time::sleep` (`preflight.rs:336`) because tokio's `time` feature sat in that crate's `[dev-dependencies]` and not its `[dependencies]`.
- **Repaired 2026-09-07** (`time` declared in `[dependencies]`), and the whole roster swept clean:
- The surviving caveat is narrower than the retired one:

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [CI/CD]: passages moved out of the body

**Section:** §Established Decisions — [CI/CD]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- then `cargo build` / nextest / clippy on the dev OS target verify the harness compiles and unit/golden tests pass, and a third job (`a11y`, `runs-on: windows-2022` since 2026-09-17, moved from `windows-2025` on the measured configuration) runs the routine webview a11y leg — `scripts/agent-run.ps1 run --e2e` under `CONDUCTOR_A11Y_STRICT`, invoked since 2026-09-17 through the leg's entry point `scripts/a11y-token-witness.ps1` DIRECTLY, at the hosted step's native HIGH integrity (its captured console and token witness printed by the step afterwards);
- the `scripts/a11y-limited-token-launch.ps1` launch that stood here from 2026-09-16 is retired from this step and survives only on the job's driver-alone diagnostics, the asserting step still carrying no `continue-on-error` and no `if:`.
- The job's WebView2 provisioning is likewise replaced:
- the `Install WebView2 Evergreen runtime 152+ (gate)` step and its `≥ 152` runtime floor are retired outright in favour of driver/runtime COHERENCE — a `Pin msedgedriver to the image's WebView2 runtime (gate)` step reads the runtime from the two `EdgeUpdate` client keys, fetches the driver for THAT version, Authenticode-gates it, asserts driver major == runtime major and publishes `EDGEWEBDRIVER`, carrying no `continue-on-error` and no `if:` — then the reused `journal_conformance` gate over `runs/a11y`, then the violation-record artifact upload (2026-09-07).
- **The a11y job ran RED on GitHub's hosted `windows-2025` image at runtime 152/153, as measured at runs 34157101273 · 34158355397 · 34160378753 · 34162118841 (2026-09-07, from a since-deleted `ci-probe/` ref) — a reading bound to THAT image and runtime pair, superseded as the arm's verdict by the green below.** It resolves the driver from the image and its RED wdio output reaches the job log — both fixed at `2026-09-07-sr-findings-fixed` and proven in CI — then fails at WebView2 session creation, before the `journal_conformance` gate or the record upload is reached.
- The leg also RUNS on the Windows dev host through tauri-driver, and its tally there is now a green too:
- 12 passing / 2 skipped at runtime 152 (2026-09-10);
- 10 passing / 2 failing / 2 skipped re-measured 2026-09-16 after the host floated to Evergreen 153.0.4234.32 unprompted;
- and 12 passing / 0 failing / 2 skipped on 2026-09-17 once the two regressions were fixed.
- Those regressions were the hold-free Operable pair (SC 2.1.1 reachability · SC 2.4.3 focus order) and NOT session creation, and both were ASSERTION defects rather than a platform property — one walk waiting on a `BODY` sentinel this webview's focus cycle need not contain — so the pair is now environment-independent, measured green on two configurations differing in runtime major, driver major and coherence.
- Hosted-runner runnability is MEASURED and PROVEN for the configuration named above, and unproven elsewhere — the configuration that achieved it is no longer probe-scoped:
- it is the shipped arrangement as of 2026-09-17 (operator ruling at the chunk's P4, re-ruled at P5 to ratify the driver pin).
- **Since 2026-09-17 the job pins the DRIVER to the runtime the image already carries, replacing the in-job WebView2 runtime provisioning it ran from 2026-09-08 (conditionally from 2026-09-12)** — ahead of the routine arm it reads the runtime from the `EdgeUpdate` client keys and fetches msedgedriver for THAT version (§Occupied Resources — Ports), admitted by a pre-execution `Get-AuthenticodeSignature` gate (status `Valid` AND an `O=Microsoft Corporation` signer, non-zero exit on either arm), then asserts driver major == runtime major;
- The `≥ 152` runtime floor is RETIRED and no longer exists in code:
- it was not necessary (a coherent 131/131 pair runs the arm green), not sufficient (a coherent 152/152 pair fails, run `34654076633`), and destructive on the working configuration (run `35185153012` installed 153 over a native 131).
- Whether a 152+ runtime actually opens the remote-debugging endpoint is now **measured, and the answer is no** — as measured at CI run `34280136892` (2026-09-08), the run that followed the commit shipping the probe:
- the gate installed Evergreen 152.0.4191.66 (Authenticode `Valid`, `O=Microsoft Corporation`, re-read by the diagnostics step seven minutes later) and probe (a) still reported `DevToolsActivePort first seen: never within 90s` with the app alive (`HasExited=False`) and three `msedgewebview2` children resident.
- The runtime-major hypothesis is therefore FALSIFIED;
- the cause stayed OPEN until 2026-09-12, when it was established as elevation (below).
- **Both halves of the former leading candidate pair — a hosted-image policy, and a session property — are now MEASURED and neither holds**, as measured at CI run `34586959536` (`conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-probed/evidence/reading.md`, step `WebView2 cause probes (diagnostic)`, against a same-script dev-host control):
- all five probed Edge / EdgeUpdate / EdgeWebView policy keys are ABSENT in both the machine and user hives on the hosted runner AND on the dev host, so no policy difference exists to explain a behavioural one;
- and the runner is `SessionId 2` with `UserInteractive: True` — not session 0 and not a service context — corroborated by the isolation step's own `tasklist` showing the app and its webview children on Console session 2.
- **The cause is now ESTABLISHED as elevation, and two further candidates are retired** (2026-09-12, `2026-09-11-hosted-runner-endpoint-cause-closed`).
- The third candidate's reading WAS taken:
- moving the module section inside the `WebView2 session isolation (diagnostic)` step's live window — the placement, not a longer timeout, was the whole defect — read `msedgewebview2 : 6 live process(es)` with its loaded module versions on the hosted runner beside a same-script dev-host control (as measured at CI run `34645345201`;
- Driver/runtime major SKEW is RETIRED as a cause by direct variation with a control:
- ELEVATION was then VARIED on the known-good host:
- an elevated leg on the Windows dev host reproduces `session not created: DevToolsActivePort file doesn't exist` where the non-elevated control is a recorded green, the pair differing on `IsElevatedAdmin` alone (as measured 2026-09-12, `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md`).
- **Elevation is NOT the whole discriminator, and "the app must not run elevated" is RETIRED as a general remedy** — the shipped asserting step runs at the hosted runner's native HIGH integrity and creates a session (run 35208593666).
- What the 2026-09-12 variation established still stands on its own scope:
- the Medium-integrity drop helped at runtime 153 on the dev host and BREAKS the session on `windows-2022` at 131, which is why the asserting step no longer takes it, and the remedy CLASS named here — a limited-token launch, then a re-measurement — is now itself MEASURED INSUFFICIENT (2026-09-16):
- both mechanisms shipped, a scheduled task and then `runas /trustlevel` replacing it in the same chunk, are correct and structurally unable to lower the MANDATORY INTEGRITY LABEL, which three dev-host legs established as the discriminator (admin+High no session · non-admin+High no session · non-admin+Medium session created) — `RunLevel Limited` cannot, because the runner's job account is the built-in Administrator with `FilterAdministratorToken` off and therefore has no filtered token;
- `runas /trustlevel` cannot, because it strips the Administrators group and leaves the label.
- The remainder — an explicit medium-integrity launch **[measured-insufficient 2026-09-17:
- the explicit medium-integrity launch WAS built and measured — it lowers the label as designed (parent `S-1-16-12288` → child `S-1-16-8192` on the runner, confirmed from inside the leg) and did NOT open the endpoint.
- The one CI run that opened a session invoked the leg DIRECTLY at High integrity.
- The remainder is no longer 'a medium-integrity launch'.]** — is owned by the successor route entry, `v3-02` having returned to the pool.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Timing-Tolerance Model]: passages moved out of the body

**Section:** §Established Decisions — [Timing-Tolerance Model]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- `slo_tier` keeps its closed three-value set and the latency formula is unchanged.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Read-Back Dependency Posture]: passages moved out of the body

**Section:** §Established Decisions — [Read-Back Dependency Posture]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- **The gate reached `ready:true` for the first time on 2026-08-16** (`canary_round_trip: "ok"`, `blocked_precondition: null`, exit 0).
- titles are scrubbed, and `retrieve_telemetry_slice.fingerprint_refs` — the former carrier — was populated from the **L4 model's `evidence_refs`** alone (`andromeda-pulse pulse-app/src/inference_runtime.rs:684-701`), which the deterministic-L4 fixture fills with a constant `det-*` triple (`det-span-…` / `det-template-0007` / `det-fingerprint-…`):
- the chain is `deterministic_inference.rs → interpretation/markdown.rs → mcp-server/tools.rs:436`.
- Pulse's own computed fingerprint lands in the `span_events.fingerprint` column, which no MCP tool reads — so at `efabe8e` it reached no read-back surface, and the old assertion could never have passed however the derivations were aligned.
- **At HEAD `83d4060` it reaches `fingerprint_refs` by another route:** an incident's refs are the model's refs ∪ the triggering cue's full-hex fingerprint (`grounded_fingerprint_hashes`, `pulse-app/src/inference_runtime.rs`, read at `83d4060` by `2026-09-22-interpretation-proven-live`) — as measured at `conductor-0.2.0/chunks/2026-09-10-release-build-and-bundle/evidence/leg1-2026-09-10T19-32-57-092.jsonl`, whose envelope `fingerprints` carries `0bddf438a748f326f07436169ea20a6a` beside the `det-*` triple, the value that run's own self-obs line `canary fingerprint computed …` states (its `.selfobs.jsonl` twin).
- **A SECOND axis was measured 2026-09-01 (Pulse HEAD `83d4060`) and closes the same way:** `incident_events` — the corpus table persisting created/resolved lifecycle events, and the one table whose content is NOT L4-authored — is written by `crates/triage/src/incident/persistence.rs::save_incident_event` and read only corpus-side (`crates/corpus/src/disposition.rs` + a `contract.rs` test helper), with ZERO references in `crates/mcp-server`;
- **But "no stronger claim exists" is RETIRED — a stronger one does, on a different axis.** Payload fidelity was unattainable at `efabe8e` and is PARTIAL at `83d4060` — one read-back value, the grounded cue fingerprint, varies with what Conductor emitted (above);
- incident 6 resolved and gone from the active set with `idle_seconds_at_resolve = 0.0`, against Pulse's 120s auto-resolve idle threshold (its corpus records the incident active 16:42:38→16:43:23, a 45s life), so the resolver is excluded by construction and Conductor's write is the only remaining cause — `conductor-run/tests/lifecycle_harvest.rs`.
- **Two mechanisms measured the same day bound how that attribution can be built.** (1) **At most ONE incident is active per DEDUPE TUPLE:** Pulse's producer dedupes a new incident against an OPEN incident carrying the same `(kind, scope, scope_id)` tuple — the workspace scopes the CANDIDATE SET, the tuple is the key, **as measured at `andromeda-pulse pulse-app/src/inference_runtime.rs:811`** (`.find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)` over `registry.list_active(&digest.workspace)`, HEAD `83d4060`).
- The originating leg's storms all shared one tuple, which is why it observed the effect at workspace grain (`created=false deduped=true` observed while one was open;
- every `created=true` occurred with the active set empty;
- a new incident formed the same second the prior was resolved) — **the leg evidence stands;
- This SUPERSEDES the leading-path-segment narrowing recorded 2026-08-16:
- the SUT changed its normalization, and until Conductor transcribed the guard its `fingerprint()` returned a DIFFERENT value than Pulse's for every slash-bearing path — a live derivation drift no Conductor-vs-Conductor gate could see.
- as measured 2026-08-21 (leg D), `findings-counter-refresh` CARRIED one `[[expected]]` at that time and still landed `KnownResidual`, because ITS read-back came back degraded, and its unmet `CountAtLeast` floor graded `CalibrationRegion` rather than failing (a sample floor never hard-fails).
- That measurement stands as the evidence for the route;
- the scenario itself has been declare-only since 2026-09-15, when that floor was retired as structurally dead, so it is no longer a LIVE example of a checks-bearing scenario taking this route.
- **`degraded` is a PER-READ-BACK property, never a mode-wide guarantee — the universal "under deterministic L4 every read-back returns `degraded_mode`" is RETIRED, measured false 2026-09-06.** Deterministic L4 has now been measured BOTH ways:
- `findings-counter-refresh` degraded (2026-08-21), while `degraded-mode-report` read back an OPEN, non-degraded incident and took the graded route to `manual_record` → `ManualCheck` (`latency_ms 6045`, run `2026-09-06T09-11-09-325`, leg B1 of `2026-09-06-operator-gated-live-suite`) — as measured at that chunk's `evidence/b1.jsonl` + `evidence/live-suite-verdict.md`, where `state_for` (`conductor-run/src/execute.rs`) returns `KnownResidual` iff `observation.degraded`, so a `ManualCheck` row proves `degraded == false`.
- only the universal is withdrawn.
- **That firing condition is UPTIME-BOUND, and the bound was invisible until 2026-09-06 — but the bound is the window IN FORCE AT BOOT, not a fixed hour.** The one-hour figure is `BOOTSTRAP_WINDOW_SECONDS = 3_600`'s DEFAULT;
- **That removes cause (a) and is NOT sufficient — cause (b) is untouched by the posture, and it is what still decides the leg.** As measured at `conductor-0.2.0/chunks/2026-09-06-halo-hue-budget-re-driven/evidence/hue-verdict.md`, two runs of the identical tree under the identical posture on 2026-09-07 graded differently:
- the evaluator was disarmed in BOTH (`silence_cues_emitted: 0`, `services_in_bootstrap: 2`), yet the arm was reached only when the window's last incident formed early enough to clear the 120s idle + 30s tick before read-back — created at +45s into the 200s window it resolved and the arm fired (`KnownResidual`), created at +137s it was ~62s old at read-back and the arm was missed (`ManualCheck`).
- As measured at `pulse-legs/a11y-20260906-110201/logs/agent-latest.jsonl.2026-09-06` (`2026-09-06-operator-gated-live-suite` and its 0-pending adaptation):
- the day's FIRST emitted silence cue fired 10:11:09.723Z, one hour after the canary service's first span (~09:11Z), and thereafter incidents formed mid-silence at 11:00:33 and 11:02:02 after cues of magnitude 3.0 → 5.0 → 7.03 — against `BOOTSTRAP_WINDOW_SECONDS = 3_600` (`andromeda-pulse crates/triage/src/baseline/activity_floor.rs:36`, evaluator per emit cycle at `crates/triage/src/cue/emitter.rs:186`, HEAD `83d4060`).
- This qualifies the leg-E narrative rather than retiring it:
- leg E reached the arm because it ran inside that window, not because a `curious` cue can never be refreshed.
- This RETIRES the 2026-08-20 reading that the canary's own error-rate cues necessarily keep the list non-empty:
- The arm was exercised, not modified.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Run-History Persistence]: passages moved out of the body

**Section:** §Established Decisions — [Run-History Persistence]
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- its `Contains "Previously seen"` check was retired because Pulse emits the token as `"## Previously Seen"` and `Contains` is case-sensitive, so the declared token never matched.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Ports: `https://msedgedriver.microsoft.com`: passages moved out of the body

**Section:** §Occupied Resources — Ports: `https://msedgedriver.microsoft.com`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- it REPLACED `https://go.microsoft.com/fwlink/p/?LinkId=2124703`, the WebView2 Evergreen bootstrapper target admitted 2026-09-08 and retired with its install step, so the non-loopback egress count is ONE before and one after).
- The arriving crossing is strictly NARROWER than the one it replaced:
- the fwlink carried no version selector, so every job took whatever was latest, while this one cannot drift on its own.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Service / process names: `andromeda-pulse-mcp`: passages moved out of the body

**Section:** §Occupied Resources — Service / process names: `andromeda-pulse-mcp`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- Pulse MCP server (`andromeda-pulse-mcp`), spawned over PIPED stdio from a `tokio::process::Command` built by `conductor-verify/src/spawn.rs::build_command` and handed to `ReadbackClient::connect_command` (rmcp and its `TokioChildProcess` were removed 2026-06-27;
- the type has been absent from every crate `src/` since, and its last in-code mention — a stale doc comment — was corrected 2026-09-04).
- **On Windows the command carries the `CREATE_NO_WINDOW` creation flag** (`spawn::console_suppressing_flags()`, `#[cfg(windows)]`), so a GUI-launched sidecar raises no console pane and publishes no absolute exe path — as measured at `conductor-0.2.0/chunks/2026-09-04-sidecar-spawn-without-a-console-window/evidence/nvda-pass.json` (SR row S1-01 free of the pane;

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Crate names (workspace members):: passages moved out of the body

**Section:** §Occupied Resources — Crate names (workspace members):
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- carries the `conductor-run → conductor-faults` dependency edge, conductor-faults' first consumer — sitting above the seams and below both bins, shared by them), `conductor-cli`, plus the `conductor-tauri` bin.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Frontend asset subtree:: passages moved out of the body

**Section:** §Occupied Resources — Frontend asset subtree:
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- `node_modules/`, the build output `crates/conductor-tauri/ui/dist/`, and `crates/conductor-tauri/ui/logs/` (the RETIRED pre-2026-09-01 self-obs landing site — under the repo-root launch cwd with `CONDUCTOR_RUNS_DIR` unset the stream lands at the workspace-root `logs/`;
- the routine `--e2e` arm sets the handle and lands beside its fixture runs dir instead — see the `conductor-tauri.jsonl` bullet) are git-ignored;

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `logs/conductor-tauri.jsonl`: passages moved out of the body

**Section:** §Occupied Resources — On-disk artifacts / database: `logs/conductor-tauri.jsonl`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- **With `CONDUCTOR_RUNS_DIR` unset the path is CWD-relative**, and the a11y legs spawn tauri-driver with `cwd` = the workspace root (the app under test inherits it).
- The landing site is therefore **per-suite**, because the sink resolves as `runs_dir.parent()/logs` and the ONE tauri-driver spawn site chooses `CONDUCTOR_RUNS_DIR` per invoked suite (2026-09-02):
- the routine `--e2e` arm (`runs/e2e-fixture`) and the `sr-empty` suite land at `runs/logs/conductor-tauri.jsonl` (measured 2026-09-02 10:46:10Z), the operator-local `a11y:driven` arm (`runs/driven/runs`) at `runs/driven/logs/conductor-tauri.jsonl` (measured 2026-09-02 11:55:29Z, 11 457 B) and the `sr` / `sr-error` suites (`runs/sr-leg/runs`) at `runs/sr-leg/logs/conductor-tauri.jsonl` (measured 2026-09-02 10:44:45Z, 8 312 B).
- No a11y suite leaves the handle unset any more;
- a plain launch with it unset still lands at the CWD-relative workspace-root `logs/conductor-tauri.jsonl` (measured 2026-09-01 by the pre-change driven arm;
- the pre-cwd `crates/conductor-tauri/ui/logs/conductor-tauri.jsonl` left stale), so that root site is retired only as an a11y landing site, not as the unset-handle mechanism.
- That ui-package path is the retired pre-2026-09-01 landing site and stays git-ignored explicitly.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/pulse-load-envelope.toml`: passages moved out of the body

**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/pulse-load-envelope.toml`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- no phase declaring occurrences may run longer than `max_sustained_storm_ms` nor emit faster than `max_sustained_rate_spans_per_s` (`EmissionSpec::occurrences` is what made both computable).
- **The rate term counts WIRE RECORDS, and is an upper BOUND (shipped 2026-09-06, superseding the DISPATCHES reading measured 2026-08-20):** `phase_rate_exceeds` judges `occurrences × EmissionSpec::max_spans_per_dispatch() × 1000 > max_rate × gap_ms` (`conductor-core/src/load_envelope.rs`), exact integer math with no division.
- The per-dispatch count is measured PER `EmissionShape` arm against the dispatcher's own arm — `Ramp`/`Breathing` emit `sum(window_counts)`, `Latency` its `samples`, `Error` `depth + 1`, `Pii` 2 spans on traces or one record per category on logs, `Topology` one span per service, every other shape 1 — so the retired `samples` / `windows` reading held for `Latency` alone and understated a rate curve by its own rate.
- Under the shipped mapping the catalog's worst case is `halo-breathing-encoding` at ~232 records/s (a ramp bounded at 1160 records per dispatch over a 5 s window), ≈43× under `max_sustained_rate_spans_per_s = 10000` — **superseding the "~200× under the bound" figure, which held only for the `samples` reading** — so no verdict moves and the `[[exempt]]` ledger stays empty.
- The contract file itself is unchanged:
- the term keeps its name and its value, because it transcribes a SUT record about wire load and the fix brings the code to the name rather than the name to the code.
- This retires the SURFACED-not-authored disposition — the fix is AUTHORED by `2026-09-06-coverage-completeness-gate`, and it is neither of the two the prior record predicted (it is not `occurrences × samples`, and the term was not renamed).
- `phase_rate_exceeds` has exactly one caller, `phase_breach`, whose only two callers are `check_load_envelope` (the static gate) and `LoadEnvelope::classify` (the per-run caption), so the two cannot diverge.
- `check_load_envelope` (the static catalog gate) and `LoadEnvelope::classify` (the per-run judgment behind the `[ENVIRONMENT-SUSPECT]` caption) read ONE shared basis, so the gate and the caption cannot mean different things, and the `[[exempt]]` ledger — still held at exact-set equality in both directions, so it can only shrink under compulsion — is now **empty**:
- The artifact had predicted a different landing (that asserting SUMMED emitting-phase duration would retire the exemptions);
- measurement falsified it — summing disjoint bursts separated by quiet is not *sustained*, and both prior exemptions stay over a summed ceiling — so the shipped term bounds the longest single emitting window instead.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/pulse-run-contract.toml`: passages moved out of the body

**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/pulse-run-contract.toml`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- The `[[term]]` list carried SIX terms as of 2026-09-03, when it gained `mcp-enabled` (`check = "shell-declaration"`, `env = "ANDROMEDA_PULSE_MCP_ENABLED"`), the same class as the shipped `l4-deterministic` and needing no new machinery.
- It carries SEVEN as of 2026-09-22 (`grep -c '^\[\[term\]\]'` → 7):
- `l4-real-model` (`check = "shell-absence"` on `ANDROMEDA_PULSE_L4_DETERMINISTIC`, `posture = "real-model"`) — a CONDUCTOR posture term, as the header's PROVENANCE line states, not a transcribed SUT record — beside `l4-deterministic`, now tagged `posture = "deterministic"`.
- **A second term's `asserted` rationale is measurably FALSE, and the shape is the `warmup_ms` one:** `sidecar-built` reads "an unbuilt sidecar fails connect and surfaces the read-back-unreachable precondition upstream of this gate, so reaching term evaluation at all proves it" — but a `PATH` miss short-circuits to `[BLOCKED]` in ~0s UPSTREAM of term evaluation (as measured at `.claude/rules/verification-harness.md:54`, 2026-08-20;
- consistent with the ~2ms all-tools-absent reading in §Occupied Resources — Service/process names), so the term is satisfied-by-construction exactly when it is false and cannot catch its own falsity.
- It was deliberately NOT re-classified at `2026-09-03-live-pulse-preconditions-probed` (operator-selected):
- re-classifying moves what the preflight gate can block on, which is a larger change than probing preconditions.
- it does not carry the canary service out of Pulse's baseline bootstrap, and no pre-roll measured in seconds can.** Pulse gates the **baseline-derived cue families** on `BootstrapState::Ready` (`crates/triage/src/cue/evaluate.rs:164` — the ONLY such gate in `crates/triage/`, sitting inside `evaluate_service_went_silent`, the P-014-class silence cue), which requires `now − first_observed_unix_nanos ≥ BOOTSTRAP_WINDOW_SECONDS = 3_600` **per service, wall-clock** (`crates/triage/src/baseline/activity_floor.rs:33`, `:173-183`;
- that constant is the DEFAULT — `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` overrides it once at Pulse's boot, §Occupied Resources — Environment variables — which changes nothing about the disproof below, since no seconds-scale warm-up clears any window of that order), and the `baseline_state` corpus table holds **0 rows**, so every `pulse-app` launch resets the anchor to zero.
- The shipped `warmup_ms = 45000` is 45s against a 3,600s gate — short by **80×** and unfixable by any value of its own knobs, which is why the term's `check = "asserted"` (satisfied by construction) cannot catch its own falsity.
- **That diagnosis is TRUE only of the baseline-derived families, and the canary does not ride one (measured 2026-08-15).** The RetryStorm path consults no baseline at all (`crates/triage/src/pattern/storm.rs:245-285` keys solely on count-in-window against the thresholds and the `last_emitted` tier ladder), so "without a baseline the L2 cue evaluator never considers it" holds for service-went-silent and never bound the canary storm;
- Pulse forms incidents for a sustained storm with `baseline_state` at 0 rows, measured twice on two fresh data dirs (`andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/premise-check.md`).
- The warm-up's stated purpose is therefore disproved on a second axis — it aims at a gate this path never crosses — but it is harmless.
- **The claim that reaching a live incident requires a Pulse-side change (a test-mode bootstrap override, or populating `baseline_state`) is likewise measured FALSE and is retired**:
- no such change exists, `baseline_state` still holds 0 rows, and incidents form anyway.
- What actually gated the canary was a **tier band on Conductor's own side**:
- the storm must clear `DEFAULT_AUTONOMOUS_THRESHOLD = 10` (`crates/triage/src/pattern/storm.rs:78`, compared with `>=` at `:245`) because Pulse's Tier-1 coordinator accepts Autonomous cues alone (`crates/triage/src/cadence/coordinator.rs:390`), and the shipped `CANARY_STORM_COUNT = 6` sat in `5 <= 6 < 10` — cue floor cleared, Autonomous band missed, no incident possible.
- `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` §Re-run — 2026-08-13.
- **A SECOND gap sat beside it on the canary's own path (measured 2026-08-14/15) and is now CLOSED (2026-08-16) — its cause was CONDUCTOR-side, not Pulse-side.** The symptom:
- Conductor's storm reached the wire with its `exception` events intact and Pulse counted every span (the ingest heartbeat's `span_count` is a cumulative `fetch_add`, reading `15` = 3 warm-up + 12 storm at the 2026-08-15 leg), yet the `buffer.tick` trio read `span_events_seen: 0` · `observer_invocations: 0` · `fingerprints_computed: 0` with `rows_ingested: 1` — the fingerprint observer never invoked, and the loss localized between OTLP ingest receipt and the buffer's span-event enumeration.
- **The cause was a span-identity collision against a primary key.** Pulse's `spans` table is `PRIMARY KEY (trace_id, span_id)` (`andromeda-pulse crates/buffer/src/schema.rs:38`), while Conductor's `ok_span` stamped a CONSTANT identity — `vec![1; 16]` / `vec![1; 8]` — on every call, so every warm-up span after the first violated the key and was logged-and-skipped by `run_consumer`.
- That is exactly the producer-dependence observed:
- `inject_demo` derives `trace_id(seq)` / `span_id(seq)` per sequence and never collides.
- Fixed by `2026-08-15-canary-spans-pulse-fingerprints` (seeded per-call identity, disjoint warm-up/storm seed ranges);
- **One honest limit on the closure:** why the storm's 12 DISTINCT-id spans also appended zero rows on the prior leg was never directly observed — that leg captured no `duckdb.append` lines and its data dir no longer exists — so the appender-poisoning reading is **inferred, consistent with both measurements, and not proven**.
- Read the storm-detector telemetry accordingly:
- `tracked_fingerprints_count` is a **60s-windowed gauge over DISTINCT fingerprints, sampled at a 15s tick AFTER eviction**, so a working identical-fingerprint storm reads `1` regardless of its occurrence count — never the count itself (never 6 at the retired size, never 12 at the shipped `CANARY_STORM_COUNT`) — and a late sample reads 0 on a healthy path;
- That reading is what the 2026-08-16 leg confirmed from both directions:
- on the broken path both cumulative counters held 0 across 31 tick lines spanning three arms (twelve inside the retention window), while on the fixed path they read `2` and `1` with `tracked_fingerprints_count` still sampling `0` — a healthy storm, 77s past its 60s window.
- Never read that gauge as evidence of absence.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/scenario-audit-ledger.toml`: passages moved out of the body

**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/scenario-audit-ledger.toml`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- Its grounds are transcribed readings of `andromeda-pulse` at the head its own `sut_head` names, so a SUT rendering change rots them visibly rather than silently.
- It differs from the three manifests above in its resolution:
- `default_path()` is a hard-coded relative constant and `load()` takes an ALREADY-RESOLVED path, so the module never calls `resolve_under` and there is deliberately no `CONDUCTOR_*` override handle;
- it has NO shipped reader at all — its only reader is the crate-local gate test binary, which resolves the workspace root from `CARGO_MANIFEST_DIR`.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/pulse-real-model-leg-posture.md`: passages moved out of the body

**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/pulse-real-model-leg-posture.md`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- the launch posture, the emission profile (its premise that a hypothesis needs a multi-digest profile corrected in place 2026-09-22 — Pulse attaches the parsed interpretation at incident creation, so a single storm carries one;
- Its `provenance` is stated **per clause** — every Pulse coordinate is a transcribed SUT record read at HEAD `83d4060` (2026-09-18), while the ~110 s it once carried as a real-model incident-formation figure is a DETERMINISTIC-L4 measurement (`conductor-run/tests/lifecycle_live.rs:20`, under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`), mis-carried as real-model since `conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/plan.md:102` and corrected in place 2026-09-23:
- Its corrections are dated in-place records beside one dated measurement record.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_PREFLIGHT_TIMEOUT`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_PREFLIGHT_TIMEOUT`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- `scripts/agent-run.sh` 2026-09-04T17:07:17→17:08:04Z and `scripts/agent-run.ps1` 17:11:16→17:12:03Z, ~47s wall-clock each (warm-up 45s + poll), both exit 0 emitting `ReadyState` JSON with zero `skipped preflight` lines and neither script edited.
- The run contract raises it to an effective FLOOR it can no longer sit below (`[incident_formation].min_canary_poll_seconds`):
- the bare default is shorter than Pulse's own L3 digest cadence, so it could expire before an incident exists even once a cue fires.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_MSEDGEDRIVER`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_MSEDGEDRIVER`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- Read ONLY by `crates/conductor-tauri/ui/wdio.conf.ts` (never by a shipped binary) — the ONE config all three suite families fire, so the handle gates the routine `--e2e` arm, the operator-local `a11y:driven` arm and the `sr*` screen-reader suites alike — never written by Conductor, and never interpolated into a shell:
- Unset — or set to a path that is not an existing file — makes whichever arm requested it **SKIP at exit 0** with a host-path-free precondition plus a fetch recipe.
- under an affirmative `CONDUCTOR_A11Y_STRICT` the same unresolved handle exits NON-ZERO instead (2026-09-07), because a CI gate that can pass by skipping is banned — the guard itself is untouched, only what an unresolved handle costs.
- One of TWO handles in this namespace naming a HOST dev-tool this document does not itself define (`CONDUCTOR_MSEDGEDRIVER` · `CONDUCTOR_NVDA`), both validated and spawned the same way, so neither value is ever committed.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_NVDA`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_NVDA`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- added 2026-09-02).
- Validated at the harness edge exactly like `CONDUCTOR_MSEDGEDRIVER` (existence + `isFile` + shell-metacharacter rejection) and used only as the program of an array-form, detached spawn with a fixed argv (`-m --no-sr-flag -c <leg config dir> -l 12 -f <leg speech log>`, then `-q`), never interpolated into a shell.
- Unset — or set to a path that is not an existing file — makes the `sr` / `sr-empty` / `sr-error` suites **SKIP at exit 0** with a host-path-free precondition plus a fetch recipe naming the handle, never its value (measured 2026-09-02).
- both skip sites route through one `exitUnresolvedHandle()`, so an affirmative `CONDUCTOR_A11Y_STRICT` makes an unresolved NVDA handle exit non-zero too (2026-09-07).

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_E2E_SEED_DIR`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_E2E_SEED_DIR`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- It is the THIRD handle class in this namespace and its control model is neither of the other two:
- the value is not operator-supplied, so there is no `isFile` / shell-metacharacter guard to apply, and it never reaches the `conductor-cli` `canonicalize` edge because no shipped binary reads it — the repo-relative discipline lives at the wdio caller.
- Distinct from `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA`, which name HOST dev-tools this document does not define;
- this one names a path this document does.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_A11Y_STRICT`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_A11Y_STRICT`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- **A FOURTH handle class, and its control model is none of the other three:** it is FLAG-valued (affirmative `"true"` / `"1"`, mirroring `conductor_core::flag_declared`), never a path — so the `std::fs::canonicalize` rule has nothing to canonicalize and the wdio-edge `isFile` + shell-metacharacter guard has nothing to guard.
- Unset or falsy, the leg keeps its documented skip-at-exit-0 so an unconfigured dev host is never read as a defect;
- affirmatively declared, an unresolved `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` exits NON-ZERO, which is what stops the CI a11y gate from reporting success by skipping.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `ANDROMEDA_PULSE_MCP_ENABLED`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `ANDROMEDA_PULSE_MCP_ENABLED`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- as the run contract's SECOND `shell-declaration` term (`mcp-enabled`), and as one of the three `ANDROMEDA_PULSE_*` declaration subjects `conductor preconditions` observes.
- this flag and `ANDROMEDA_PULSE_L4_DETERMINISTIC` keep the truthy `"true"`/`"1"` gate (`conductor_core::flag_declared`) under the deterministic posture — under the real-model posture the L4 handle is graded for ABSENCE instead (its entry below) — while the PATH-valued `ANDROMEDA_PULSE_DATA_DIR` is met by presence-after-trim (`conductor_core::handle_declared`), so all three subjects are meetable (§Standard Contracts — Liveness equivalent).
- An absent declaration of THIS handle is named upstream as an unmet contract term AND distinguished at the probe:
- `ANDROMEDA_PULSE_MCP_ENABLED=false` leaves `handles-declared` naming that handle alone (measured 2026-09-04, `preconditions-probe-reads-path-handles-by-presence`).

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `EDGEWEBDRIVER`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `EDGEWEBDRIVER`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- Since 2026-09-17 the `a11y` job's pin gate SETS it through `GITHUB_ENV` to name the driver directory the job itself fetched and Authenticode-verified — the runner image's own value is deliberately overwritten, because that image ships a floating driver incoherent with its WebView2 runtime — and the asserting step then resolves `CONDUCTOR_MSEDGEDRIVER` from it in the step SHELL (`$env:EDGEWEBDRIVER`), because GitHub's `${{ env.* }}` expression context holds only workflow/job/step declarations and never a runner-process variable — resolving it there yielded an empty base and the handle reached the guard as the literal `\msedgedriver.exe` (measured 2026-09-07, run 34148079506).
- `crates/conductor-tauri/ui/wdio.conf.ts` remains the only READER of `CONDUCTOR_MSEDGEDRIVER` and the only site that validates it (`isFile` + shell-metacharacter rejection) before handing it to tauri-driver via `--native-driver`.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- a SHIPPED artifact names it in its own output — the `run --live` banner in both `scripts/agent-run.{sh,ps1}` — so a reader meeting it there can find it here.
- the operator's launch carries it, and it governs every leg of that run.
- Distinct from the two `shell-declaration` run-contract terms and from the three `ANDROMEDA_PULSE_*` subjects `conductor preconditions` observes:

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `WEBVIEW2_USER_DATA_FOLDER`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `WEBVIEW2_USER_DATA_FOLDER`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- They are read by the WebView2 loader alone — by no shipped binary and not by `crates/conductor-tauri/ui/wdio.conf.ts` — so neither is a spawn-resolution input and neither carries a configuration contract;
- Registered on the discoverability ground the `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` entry states:
- a reader meeting a name in a committed workflow must be able to find it here.
- Added 2026-09-08 (`2026-09-08-hosted-runner-webview2-session`), operator-ratified.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `RUNNER_TEMP`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `RUNNER_TEMP`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- `.github/workflows/ci.yml`'s `a11y` job reads it in the `WebView2 session isolation (diagnostic)` step and, since 2026-09-17, in the `Pin msedgedriver to the image's WebView2 runtime (gate)` step that replaced the retired Evergreen install.
- That pin gate DOES set an environment handle — `EDGEWEBDRIVER`, through `GITHUB_ENV` — but claims no `CONDUCTOR_*` name, so the `WEBVIEW2_*` pair's registered lifetime — that one `continue-on-error` diagnostic step — is unchanged by it.

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `TEMP`: passages moved out of the body

**Section:** §Occupied Resources — Environment variables: `TEMP`
**Change:** compacted under the read cap; the passages below left the body verbatim.
**Why:** the body holds only current truth; history lives here + in git (this sidecar's preamble).

- Registered 2026-09-11 as a PRE-EXISTING gap surfaced by `2026-09-11-hosted-runner-endpoint-cause-probed`, which itself reads no environment variable at all.
