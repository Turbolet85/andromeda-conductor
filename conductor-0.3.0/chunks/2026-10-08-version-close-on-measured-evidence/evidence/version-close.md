# Conductor 0.3.0 — the version's close record

Written 2026-10-08 by the chunk `2026-10-08-version-close-on-measured-evidence`, the last entry of 0.3.0. This
record claims no capability and re-grades none. It states what the ledger already holds, on the basis each
capability was measured on, and it names what is known and not closed.

## Where 0.3.0 stands

The ledger, read on 2026-10-08 with `matrix.py coverage --dir conductor-0.3.0` (tool stamp
`matrix v1.4 · bd649850`), printed:

    verified 11/11 · deferred 0 · planned 0 · implemented 0 · unclaimed 0
    claimed-unfinished: none
    done-test: YES — every entry verified|deferred and unclaimed 0
    chunks referenced: 11 distinct

## The 11 capabilities

Every row was read from the ledger on 2026-10-08, one `matrix.py show --dir conductor-0.3.0 --id` call per id.
Each of the 11 reads `status: verified`. The basis is the ledger's `ref`, cut at its first dash; the whole `ref`,
the acceptance and the notes stay in the ledger. The pointer for each row is its owning chunk's folder,
`conductor-0.3.0/chunks/` followed by the marker in the row.

| Id | Title | Method | Owning chunk | Measured basis |
|---|---|---|---|---|
| `v3-01` | Hosted-runner cause measured, not listed | manual | `2026-09-11-hosted-runner-endpoint-cause-closed` | `conductor-0.3.0/chunks/2026-09-11-hosted-runner-endpoint-cause-closed/evidence/reading.md` |
| `v3-02` | A11y CI gate at an honest terminal | a11y | `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration` | `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts::routine arm`. Kept from the ref, its configuration: CI run 35208593666 (headSha fc4a9c2), a11y job conclusion success: 12 passing / 0 failing / 2 skipped |
| `v3-03` | Keyboard and focus-order coverage has a stated owner | a11y | `2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed` | `crates/conductor-tauri/ui/test/a11y/check-claim-ownership.ts` (npm run a11y:ownership → "11 claims · 9 owned (5 operator-local, carve-out) · 2 n/a-by-construction · 0 recorded gaps" / "ownership: every claim resolved"). Kept from the ref, its configuration and owner arms: three specs of `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` under `CONDUCTOR_A11Y_STRICT=1 agent-run run --e2e` (16 passing, 2 skipped, expected 2, webview2 154.0.4258.37); one spec of `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` under `npm run a11y:driven` (2026-09-30 10:41-10:45Z, exit 0, 1 passing) |
| `v3-04` | Structurally-dead assertions retired as a class | unit | `2026-09-15-remaining-structurally-dead-declarations-retired` | `crates/conductor-core/src/scenario.rs` |
| `v3-05` | Every scenario's tier is honest about its own duration | unit | `2026-09-15-scenario-tier-honesty` | by-construction: plan `[[gate]]` probes 1-3, all green at implement 2026-09-16 |
| `v3-06` | The scenario-assertion audit is mechanical and re-runnable | unit | `2026-09-16-scenario-assertion-audit-gate` | `crates/conductor-core/tests/scenario_audit_gate.rs` |
| `v3-07` | P-025 measurement contract stated for Pulse | by-construction | `2026-09-13-p-025-measurement-contract-for-pulse` | by-construction: `contracts/pulse-p025-measurement-contract.md` states all four elements as named sections |
| `v3-08` | P-025 graded hard at its real value | dynamic-external | `2026-09-29-hue-shift-budget-graded-hard` | the harvest-tier test `crates/conductor-run/tests/delegated_timing_harvest.rs::tests::p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value` |
| `v3-09` | Real-model interpretation leg | dynamic-external | `2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09` | the harvest-tier test `crates/conductor-run/tests/real_model_grading/series_2026_10_07_sixth.rs::v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth` |
| `v3-10` | Diagnostic-quality cluster backed by an exercised path | unit | `2026-09-29-diagnostic-quality-cluster-off-the-drift-pin` | `crates/conductor-run/tests/real_model_harvest.rs::{a01..a21 (P-033), p031_*, p034_*, p044_*, the_pinned_capture_is_blocked_on_every_further_grade}` + `crates/conductor-core/src/scenario.rs::real_model_interpretation_declares_the_known_cause_under_the_real_model_posture` + `crates/conductor-report/tests/coverage_gate.rs` + `cargo nextest run --workspace --profile ci` (check_scenario_backing, check_sut_drift) |
| `v3-11` | Secret-scanning CI gate | unit | `2026-09-24-secret-scanning-ci-gate` | `crates/conductor-core/tests/secret_scan_gate.rs` |

Two notes on reading the table:

- `v3-08` and `v3-09` rest on tests at the harvest tier, which grade a committed record of a live leg. No
  run-report envelope state is offered as proof of either: a real-model envelope reads `verdict` null by design
  (obs-plan §4, Critical Path 1, Real-model posture).
- `v3-02` and `v3-03` are stated for the configuration and the arms their `ref` names and no wider. Neither row
  claims WCAG conformance beyond its owning chunk's evidence, and neither claims a VoiceOver, an Orca or a
  Linux-host leg as run.

## Three statements

**1. `v3-09` is verified on the sixth pre-registered series, three drives, and no wider.** The pass is not
attributed to the Pulse-side remedy: no prompt was recorded in that series, and the capture run on the
pre-remedy build also read three `Identified`. The third position's live record on this host stays as stated:
2 misses of 4 live runs.

- Recorded in the ledger's notes on `v3-09`, the note of the 2026-10-08 wrap ("met on THREE DRIVES and read no
  wider", "NOT attributed", "2 misses of 4 — a rate, not a cause").
- Held by two tests in `crates/conductor-run/tests/real_model_grading/series_2026_10_07_sixth.rs`:
  `v3_09_is_met_by_the_2026_10_07_sixth_series`, and the ref test the ledger gives,
  `v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth`.

**2. The capture's pairing-window fix is proven on lines built from the sixth series' recorded stamps, with a
failing-first reading.** What the capture binary prints on a next real drive is unmeasured until someone drives
it. A canary tick stamped at or after the emission instant still gets no line.

- Recorded in obs-plan §4 (Critical Path 1, Real-model posture) and in test-plan §6 (Real-model interpretation
  leg), each in a note dated 2026-10-08.
- The ledger holds no note of its own on it: the corrective chunk claimed no capability.

**3. One known exception to "model text lives only in `evidence/`".** One rank-1 model statement stands outside
an `evidence/` tree, in the `report.md` of the frozen chunk
`2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive`. The founder ruled that it stays
as it is: "Оставить как есть" (the founder, 2026-10-08, relayed verbatim by the pc overseer). The frozen chunk is
left byte for byte. This record states the exception by its location only; the statement is quoted nowhere in
this chunk's files, and that file was not opened to write this record.

- Recorded in the ledger's notes on `v3-09`, the last note, dated 2026-10-08.

## Limits known and not closed

These are named as limits. They are not debts of this version.

- **The grading rule's known weakness.** The rule grades `Identified` a rank 1 that names `conductor` as a whole
  word and also names the canary. It is left unchanged for 0.3.0. No ranked hypothesis of any drive of the sixth
  series names the canary identity, so no grade of that series rests on it (the ledger's notes on `v3-09`).
- **The corrected capture on a live drive.** What the capture binary prints on a next real drive is unmeasured
  until someone drives it.
- **The third position.** Across the four live runs on this host the third position reads 2 misses of 4. That is
  a rate and not a cause.
- **Limits the plans already state**, cited here and not re-argued:
  - the PowerShell harness's red path is read from the script and not measured (test-plan §3, the `run`
    contract). The reason that contract gives, that the Linux dev host has no `pwsh`, stopped being true on
    2026-10-06; the red path is still unmeasured, and this chunk did not drive it;
  - SC 2.3.3 is asserted from the compiled rule (a11y-plan §6 Visual Design Verification, Motion tokens);
  - the Linux `xvfb` job is an unrun target (test-plan §9, Matrix builds);
  - the control-panel-launched half of Critical Path 7 is owed (test-plan §1, Critical paths).

## The founder's ruling on the contract's readers

The posture contract (`contracts/pulse-real-model-leg-posture.md`, `## Regime`) said that no Rust code reads it,
and gave notice that the committed-manifest input-boundary duties attach in full if it becomes runtime-read.
Since the 2026-09-30 series a test helper reads it. Whether that read attaches the duties is a question about a
boundary clause, so it went to the founder. He ruled by dialog on 2026-10-08; the ruling was relayed by the pc
overseer (`inputs#I4` §1 in this chunk's folder). The three parts below are copied from that relay, each marked
for whose words it is.

The question, as the pc overseer put it to him (the overseer's words, verbatim):

> В контракте Conductor с Pulse (файл с условиями прогонов на настоящей модели) написано: «никакой Rust-код этот файл не
> читает; если начнёт читать во время работы, на файл ложатся все обязанности входной границы» (проверки, ограничения
> размера и прочее). На деле с 30 сентября его читает тестовый помощник: считает sha256 каждого замороженного раздела,
> чтобы поймать правку задним числом. Поставляемый код файл не читает. Считается ли такое чтение тем, из-за которого
> включаются обязанности входной границы?

His pick (the founder's own, verbatim): «Нет, исправить фразу (Recommended)»

The text of the option he picked, as he saw it (the option's text, verbatim):

> Тестовое хеширование не считается чтением во время работы: содержимое не разбирается и на поведение продукта не
> влияет. В контракт добавляется датированная поправка «поставляемый код не читает; тестовый помощник сверяет разделы по
> sha256, обязанности не включаются», то же в план безопасности и архитектуру. Цена: небольшая правка документов в
> закрывающем чанке, кода нет.

What follows is this record's own wording, not his.

What it settles: a test-tier digest read of the posture contract is not the runtime read the contract's notice
names, and the committed-manifest input-boundary duties do not attach to it. The notice stands as written for a
real runtime read: were shipped code to read or parse the file, the duties attach in full.

Where it lands, in three places:

1. the contract's own `## Regime` section, as one dated add-only block written by this chunk;
2. architecture's registry row for the contract and the directory-structure key line, written by this chunk's
   wrap;
3. security-plan's fixed-path-manifests row, or a dated note beside it, written by this chunk's wrap and quoting
   his pick.

No code changed for it, and the test helper was not touched.

## Open, with no owner

Three items were noticed at take-up and are on no route entry. They are named here so a reader sees them.
Whether any becomes a residual is the wrap's call.

- The reproduced run-report envelope schema types `verdict` with no null arm. It stands in obs-plan at four
  places and in a11y-plan at two; its owner is test-plan §3. A null `verdict` is older than the real-model
  posture: declare-only envelopes have read null since 2026-08-21.
- The masters and their leaves still call the webview legs' home "the Windows dev host" (test-plan §6 and §9,
  design-system's Platform line, a11y-plan). Which legs the Linux host can run is unmeasured.
- One clause of the retired Windows host leaf has no home in `host-linux.md`: "a PowerShell redirect writes
  UTF-16/BOM". It stands in `verification-harness.md`'s 2026-09-16 entry.

## The four carried items

1. **Two product-script comments cited a retired file.** Re-pointed by this chunk, one comment line in each
   script and no executed line: `scripts/agent-run.ps1` now cites `.claude/rules/verification-harness.md`, the
   2026-09-16 entry; `scripts/a11y-token-witness.ps1` now cites `.claude/docs/session-learnings.md`, the
   2026-09-11 host-leaf entry. This chunk executed neither script. Its plan was written on the premise that the
   Linux dev host carries no `pwsh`; measured at implement, that has been false since 2026-10-06 (`pwsh` 7.6.6 is
   installed). It was used here to parse only, and the readings are under Checks. The executing proof of both
   scripts stays the CI run on this chunk's pushed commit.
2. **The one known exception to model text living only under `evidence/`.** Stated in this record, the third of
   the three statements, by location only.
3. **Architecture's registries at the byte threshold, and the sites that said no Rust code reads the posture
   contract.** Both are the wrap's, in this order: first it frees bytes in architecture §Occupied Resources
   (38114 B of a 38115 B threshold at take-up), then it corrects the registry row, the directory-structure key
   line, and through them the two leaves. Both historical ordinals, "FIRST" in the P-025 row and "SECOND" in the
   posture row, stay as written. If no true sentence fits under the ordinal rule and the threshold, the wrap
   stops and shows the operator the arithmetic. This record is written before the wrap; what the wrap measured
   is in the chunk's `report.md`.
4. **The contract's own sentence.** Corrected by this chunk with one dated add-only block at the end of the
   contract's `## Regime` section, on the founder's ruling above. No existing line of the contract was edited or
   removed, and no digest-pinned section moved.

## What this close did not do

- No merge into `main`, no tag, no release, no version bump that publishes anything, no branch deletion. The
  only push is of the build branch.
- No GPU run and no live drive.
- No new development entry, no route for a next version, and no next-version directory.

Each of these waits for the founder's word.

## The next version's direction

The founder's direction for the next version, in his words, is in `next-version-direction.md` beside this file.

## Checks

- The quote in `next-version-direction.md` against the quote in this chunk's copy of the relay (`inputs#I1`),
  each with its line wraps folded to single spaces: the relay's quote is 344 characters, the document's is 344
  characters, and the two are equal. The document holds one opening quote mark.
- The table above holds 11 rows, one for each id from `v3-01` through `v3-11` and no other; two entries of this
  chunk's gate block count them.
- The two re-pointed scripts, read by PowerShell 7.6.6's own parser on the Linux dev host against their text at
  the chunk base `fb48cee` (parsed, never executed):
  - `scripts/agent-run.ps1`: parse errors 0 at the base and 0 now; 2375 non-comment tokens at the base and 2375
    now, the two streams equal; 134 comment tokens, 1 changed;
  - `scripts/a11y-token-witness.ps1`: parse errors 0 and 0; 1374 non-comment tokens and 1374, equal; 101 comment
    tokens, 1 changed;
  - the control, a copy of `scripts/agent-run.ps1` with one executed line appended: 2375 non-comment tokens
    against 2378, not equal.
