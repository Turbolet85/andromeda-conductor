# Regression record — 2026-09-30-full-gate-regression-over-the-moved-surfaces

Chunk base `84f3b27`. Every row is one plan `## Test Commands` entry, numbered as the gate tool numbers them. Entries
1–29 and 36 ran through `gate.py` (trail: `.andromeda/runs/2026-09-30T21-26-56-implement/`); the `leg = 'operator'`
entries were driven by hand by this session in their exact firing form, on the operator's slot word, and their exits
were read from the bare command. Host paths are elided throughout: the Pulse release directory is named in words.

## Configuration (every webview / SR leg)

| runtime | driver | NVDA | OS build | input path |
|---|---|---|---|---|
| WebView2 154.0.4258.37 | msedgedriver 154.0.4258.37 (coherent, entry 28 before slots A and B) | 2026.2 (portable, per command) | Windows 10.0.26200.9457 | SR: os 35 · webdriver 8 · mixed 5 · none 3 (51 rows) |

Bundle under test: built by slot A's `--e2e` arm (`cargo build --release -p conductor-tauri --features
tauri/custom-protocol`) over `84f3b27` plus this chunk's working tree; the SR record stamps `build_commit` `84f3b27`.

## Gates

| # | entry | exit | atoms / reading |
|---|---|---|---|
| 1 | `cargo fmt --all --check` | 0 | no output |
| 2 | `agent-run.sh run --unit` (runner 1, nextest `--profile ci`) | 0 | `1137 tests run: 1137 passed, 0 skipped` — static corpus and hygiene gates inside |
| 3 | `cargo test --workspace` (runner 2) | 0 | 70 targets · 1140 passed · 0 failed · 0 ignored (baseline 1139 + step 3's test) |
| 4 | `cargo test --workspace --doc` | 0 | 7 targets · 3 passed |
| 5 | nextest `-E 'test(=run_refuses_a_p_id_named_by_several_scenarios)'` | 0 | `1 test run: 1 passed, 56 skipped` — all three atoms hold |
| 6 | smoke `conductor run P-017` | 1 | `error: P-017 is named by 2 scenarios (fingerprint-distinct, fingerprint-storm); run one by name` / `hint: pass one of the named scenarios instead of the P-ID` — all four atoms hold |
| 7 | clippy workspace `-D warnings` | 0 | — |
| 8 | clippy conductor-run `--features live-pulse` | 0 | — |
| 9 | per-seam build sweep (`--lib` ×7, `--bins` ×2) | 0 | 304 s |
| 10 | `RUSTDOCFLAGS="-D warnings" cargo doc -p conductor-emit` | 0 | was 101 at P3 (C1) |
| 11 | same, `-p conductor-run` | 0 | — |
| 12 | `cargo llvm-cov nextest --workspace --profile ci --no-report` | 0 | 2524 s |
| 13 | `cargo llvm-cov report --fail-under-lines 60 …` | 0 | TOTAL lines 93.98 % |
| 14 | advisory-db porcelain | 0 | no output (copy current) |
| 15 | `cargo audit` | 0 | 1277 advisories loaded · 562 crate dependencies · 7 allowed warnings |
| 16 | `cargo deny check advisories bans licenses sources` | 0 | advisories ok, bans ok, licenses ok, sources ok |
| 17 | `mutation-gate.py selftest` | 0 | `selftest: every arm detected` |
| 18 | `npm run build` | 0 | — |
| 19 | `npm run typecheck:e2e` | 0 | — |
| 20 | `npm run knip` | 0 | no findings (was 1 on 15 at P3; C2) |
| 21 | `npm run a11y:ownership` | 0 | `11 claims · 9 owned (5 operator-local, carve-out) · 2 n/a-by-construction · 0 recorded gaps` · last line `ownership: every claim resolved` |
| 22 | `npm audit --omit=dev` | 0 | `found 0 vulnerabilities` |
| 23 | stale ci.yml clause count | 1 | last line `0` (C4) |
| 24 | ci.yml non-comment changed lines | 1 | last line `0` |
| 25 | delta guard over the touchpoints | 1 | no output |
| 26 | rule (b) files numstat | 0 | no output — send-keys.ps1, activate-window.ps1, wdio.conf.ts byte-unchanged |
| 27 | `spawnSync(` count in the SR spec | 0 | last line `2` |
| 28 | driver/runtime coherence | 0 | `driver 154.0.4258.37 runtime 154.0.4258.37` (re-read before slot A) |
| 29 | **slot A** `CONDUCTOR_A11Y_STRICT=1 agent-run.sh run --e2e` | 0 | `[a11y] verdict asserted — 0 failed · 2 skipped (expected 2) · driven session present` · 19 passing · `[webview2 154.0.4258.37 windows]` · 176 s |
| 30 | remove the working SR record (slot B) | 0 | — |
| 31 | **slot B** `npm run a11y:sr-empty` | 0 | `Spec Files:` TAB ` 1 passed, 1 total` · 1 passing (1 m 10.7 s) |
| 32 | **slot B** `npm run a11y:sr-error` | 0 | `Spec Files:` TAB ` 1 passed, 1 total` · 1 passing (11.8 s) |
| 33 | **slot C** liveness probe | 0 | `sidecar on PATH` · last line `True` (and again before slot C') |
| 34 | bootstrap-override probe | 0 | 1 line — `triage.baseline.bootstrap_window.override` WARN, `reason=env_override`, `resolved_seconds 86399` |
| 35 | **slot C** `agent-run.sh run --live` | 0 | see Live suite below — all five atoms hold |
| 36 | seven base keys over the frozen legs | 0 | last line `0` |
| 37 | **slot C / C'** live `npm run a11y:sr` | 0 | first fire graded S2-01, S2-07 off (harness defect, below); re-fire 1 passing (5 m 52.6 s), `Spec Files:` 1 passed |
| 38 | evidence copy | 0 | `nvda-pass.json` + `live-suite/{h,b1,b2,a}.jsonl` |

Entries 39–47 (the record and hygiene probes) and 48–50 (hygiene, push, CI read) are recorded in the implement
report and the operator pass.

## Live suite (entry 35)

Pulse `7bb56ea` (`pulse-app.exe` sha256 `ff677eb06d087ada08ee71bc2296444fba5c7eec26fb40a861e7fbc1525e478e`;
`andromeda-pulse-mcp.exe` sha256 `2179caab9f7247a52cba867d52e0c7de14472dbc642433ca3d27bf56c34cc634`). Source basis:
the Pulse tree's only uncommitted paths are its `.andromeda/` records, its working-route, its handoff and a new chunk
folder; every tracked product file is older than the binary except the generated `pulse-app/ui/src/bindings/index.ts`,
which is byte-equal to HEAD — so the binary's sources equal `7bb56ea` product code.

Posture, read from Pulse's own log (fresh data dir leaf `fullgate`, launched 23:01:23Z by this session on the
operator's 4317 grant): `interpretation.model.load` `inference_mode: deterministic` · `app.boot.workspace_key`
basename `fullgate` · bootstrap override `resolved_seconds 86399` · `app.boot.otlp.grpc.bind` `127.0.0.1:4317`.

Measured wall-clock 23:02:08Z → 23:20:55Z (18 m 47 s; the plan's ≈ 20 min was an estimate).

| leg | scenario | printed | envelope state | latency_ms | window (UTC) | frozen lines |
|---|---|---|---|---|---|---|
| H | halo-hue-encoding | `[RESIDUAL]` | KnownResidual | 183847 | 23:02:10 – 23:06:01 | 784 |
| B1 | degraded-mode-report | `[MANUAL]` | ManualCheck | 6009 | 23:06:03 – 23:06:56 | 74 |
| B2 | degraded-mode-report | `[BLOCKED]` (designed dedupe) | Blocked | — | 23:06:57 – 23:09:13 | 233 |
| — | quiet window | 150 s | | | | |
| A | auto-resolve-idle-window | `[RESIDUAL]` | KnownResidual | 200041 | 23:11:45 – 23:15:52 | 66 |
| driven | a11y arm | `Spec Files:` 1 passed | | | 4 m 3.9 s | |

Leg A's grade is CONDITIONAL (verification-harness.md 2026-09-06 (e)): reached under bootstrap 86399 with Pulse's
uptime 10 m 22 s at the leg's start — recorded with its posture, never read as run-stable. No `[FAIL]` printed.

## SR harness defect on the live leg, and its one re-fire

The first live fire graded S2-01 and S2-07 `not-announced` with `heard: []`. Cause, measured from
`actions.live.jsonl`: each is the first stamp of a declared shared-window pair, and the pairs landed 53 ms
(S2-01→S2-02) and 62 ms (S2-07→S2-08) apart — past the parser's 50 ms shared-window tolerance — so the first row
got a few-ms slice while its speech ("Conductor · HOLD — operator pause"; "Start button…") landed in its partner's
window. S3-04→S3-05 landed 44 ms apart and graded as expected by luck. A grading defect of the harness, not a
product finding.

Fix at the cause, no threshold change: `stamp()` in `screen-reader.e2e.ts` takes `sharesPreviousInstant`, and the
three declared partners (S2-02, S2-08, S3-05) are stamped AT their first row's instant. Re-fire (slot C', fresh
leaf `fullgatesr`, same binary and posture, launched 23:33:26Z): each pair shares one stamp instant; 51 rows graded,
none off-expectation, S1-05 the only subject-absent row.

### SR replay condition (operator, before the re-fire)

The partner-stamp rule was replayed through the shipped parser (`writeNvdaPass`) over this session's own captured
sessions, the actions rewritten only where a declared partner's stamp differed from its first row's, into a scratch
copy of the record, and the graded rows compared byte-for-byte against the working record:

```
npx tsx <scratchpad>/replay-subject.ts empty <scratchpad>/replay-empty
empty: stamps moved 0 · rows 10 · graded rows byte-identical true
npx tsx <scratchpad>/replay-subject.ts error <scratchpad>/replay-error
error: stamps moved 0 · rows 4 · graded rows byte-identical true
```

The same replay over the first live capture graded all six shared-window rows `announced-as-expected` and every live
row expected. The working record's sha256 was checked unchanged after each replay.

## SR regrade (entries 30–38)

Subjects recorded after the base (20:40:02Z), one configuration: empty 22:59:35Z · error 23:00:02Z · live 23:42:41Z.
Clocks: empty 48 pairs (one 525 ms step calibrated) · error 3 pairs (max 96 ms) · live 71 pairs (max 199 ms); none
void. E0-10 `announced-as-expected os` — heard "Run report heading level 2", then "Run report region
ENVIRONMENT-SUSPECT…" (C5). T-01 `announced-as-expected` — "Scenarios completed: 3", "Conductor · idle". The operator
review is transcribed into `nvda-pass.json` `operator_review` (2026-10-01).

## C3 red-before-green control

`run_refuses_a_p_id_named_by_several_scenarios` run against the base `paths.rs` (restored from `84f3b27`, then the new
file put back and compared byte-equal): nextest exit 100, the test failing at its `.failure()` assertion — the untouched
binary exits 0 on the silent first-match pick. On the new code: pass, beside `run_resolves_a_scenario_by_p_id` and
`run_with_unknown_target_is_a_sanitized_error_with_a_hint`.

## Process census (this session's slots)

| process | started by | final state |
|---|---|---|
| pulse-app (leaf `fullgate`) + its 10-process tree | this session, slot C | terminated — graceful close timed out at 15 s, forced; 0 tree survivors |
| pulse-app (leaf `fullgatesr`) + its 10-process tree | this session, slot C' | terminated — as above |
| conductor-tauri · msedgedriver · tauri-driver · NVDA · node (legs A/B/C/C') | the legs | terminated — self-teardown; post-leg census empty |

After the last stop: no `msedgedriver` / `conductor-tauri` / `tauri-driver` / `nvda` / `pulse-app` /
`andromeda-pulse-mcp` image running, and no listener on 4317 / 4318 / 4444 / 4445.
