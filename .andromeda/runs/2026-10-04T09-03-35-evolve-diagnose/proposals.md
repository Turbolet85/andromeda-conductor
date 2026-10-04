# Evolve Diagnosis — conductor-0.3.0 · Epoch 5 — Polish & ship · 2026-10-04T09:03:35Z

Founder-ruled run order (2026-10-04, relayed by the overseer): this run follows the Epoch 4 diagnosis
(`.andromeda/runs/2026-10-04T08-53-35-evolve-diagnose/`). Epoch 4 is read here only as prior-epoch recurrence, never
as evidence for a proposal. Every proposal is obligation-free. Accept, reject, defer or modify any of them; nothing is
applied, queued or remembered.

**Chunk legend** (the evidence pointer is the ledger record id unless a path is given):
`SS` 2026-09-24-secret-scanning-ci-gate · `MG` 2026-09-30-mutation-gate-grades-every-tally-it-rests-on ·
`UK` 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed (`UK0` = its 2026-09-29 take-up, rolled back) ·
`SG` 2026-09-30-the-screen-reader-pass-grades-again-on-this-host · `SC` 2026-09-30-the-sr-cause-isolated-on-this-host ·
`SO` 2026-09-30-the-sr-pass-regrades-on-the-os-input-path · `CF` 2026-09-30-the-screen-reader-content-findings-fixed ·
`FG` 2026-09-30-full-gate-regression-over-the-moved-surfaces · `IR` 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix ·
`SI` 2026-10-01-per-run-span-identity-in-the-real-model-harness · `PA` 2026-10-02-p-075-assert-round-against-pulse ·
`FE` 2026-10-02-captured-fingerprint-values-elided · `RC0` 2026-10-02-p-075-re-round-on-incident-events (phase cancelled at P5) ·
`RR` 2026-10-03-p-075-re-round-on-incident-events · `HP` 2026-10-04-host-portable-tauri-ipc-tests · `NS` new-session orientation.
A step marked `—` means the record carries no `step` field (see Mechanism health).

## Mechanism health
- **Records:** 413 (203 step / 210 friction), from 2026-09-24T10:50Z to 2026-10-04T01:44Z. There are 16 chunk
  markers and 15 orientations. The route lists 14 entries. The two extra markers are legitimate partial runs: `UK0` (1
  take-up record, rolled back on the founder's nothing-skipped ruling) and `RC0` (phase 5/5, with plan soft-exit and
  validate halted, an exact rollback, re-promoted next day as `RR`).
- **Coverage:** all 14 route chunks carry the full set (phase 5 · implement 3 · wrap 5). There are no 0-pending wrap
  records in range, which the "at most curation" rule permits.
- **Unparseable:** 0. **Malformed `ts`:** 0 in this epoch (11 ledger-wide, all 0.2.0, listed in
  `q-retractions.json`).
- **Retractions:** none target this epoch. Ledger-wide: 1 unresolvable pre-boundary prose retraction (founder's
  manual discount), and **retraction targeted by retraction → founder review:** `2026-09-10T19:52:10Z-a` (0.2.0).
- **9 friction records lack a `step` field** (a writer omission, all from IR/PA on 2026-10-01/02):
  `2026-10-01T20:24:02Z-b`, `2026-10-01T20:41:51Z-b`, `2026-10-01T20:52:15Z-b`, `2026-10-01T20:52:15Z-c`,
  `2026-10-02T04:28:25Z-b`, `2026-10-02T04:35:05Z-b`, `2026-10-02T05:00:18Z-b`, `2026-10-02T05:00:18Z-c`,
  `2026-10-02T05:04:49Z-b`. They count in every by-type group and are shown as `—` in the
  per-step tables. They are excluded from per-step rates.
- **Untyped rate per step:** orientation 4/7 · research 4/17 · fix-loop 3/26 · reconcile 3/34 · distill 2/17 ·
  report 2/7 · gates 2/14 · validate 1/22 · plan 1/8 · code 1/10 · route-resolve 1/7. Total: 24/210. **6 of the 24
  fit an existing type:** 3 cd-persistence records (`tooling.host-shell`); route.py `FIRST:`
  (`contract.grammar-irregularity`, `2026-09-30T14:07:27Z-b`); a live-pulse symbol gap
  (`tooling.graph-symbol-missing`, `2026-10-02T14:54:25Z-b`); and a prompt-builder doc-field parse
  (`contract.in-pass-correction`, `2026-09-30T16:20:57Z-e`).
- **Problem-fact fill:** 69/203. **id fill:** 413/413.
- **Writer drift (folded):** 10 records carry `version: "0.3.0"`. The skill field is 106 bare and 307 prefixed.
- **Host change in range:** RR and HP ran on the new Linux host (first launch recorded at `2026-10-03T23:30:42Z-b`).
  Every earlier chunk ran on Windows/MSYS. Several themes below cross the change.
- **Calibration:** reconcile's `contract.in-pass-correction` went live on 2026-09-27. SS's 2026-09-24 reconcile
  untyped record (`2026-09-24T14:37:38Z-d`) predates it, which is era, not a candidate.

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 18 cases (+1 step-less) · weight 37
**Pattern:** 18 records over 15 validate runs, 12 chunks. P5 keeps catching what P4 authored. Planlint read 0 hits
on a plan where P5 then found three required resolutions (HP). Four predicate families repeat: check 6 size WARN ×4, every one
fence-driven; 4(6) ×4; 4(9) ×4; check 2 ×4. Three cases are review-only, of a kind no predicate attempts.
**Evidence:** all 19 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS | check 7 WARN on two new test-target .rs files that call no workspace symbol | — | 2026-09-24T12:10:27Z-b |
| SS | review-only: the P5 preview ended "so a hit turns the build red", a CI observation no step makes | dlg 1 · it 1 | `conductor-0.3.0/chunks/2026-09-24-secret-scanning-ci-gate/plan.md` |
| MG | 4(6) ×2 (properties no entry proved) + check 2 (an Expected amendment naming a non-master leaf) | it 1 | 2026-09-30T08:14:08Z-b |
| UK | review-only: three sibling-suite SR DOM assertions would FAIL once the scroll div loses its tab stop | dlg 1 · reads 5 | `conductor-0.3.0/chunks/2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/plan.md` |
| UK | check 6 size WARN 621 lines | — | 2026-09-30T09:22:30Z-c |
| SG | 4(4)/4(6) (no artifact keys; two criteria naming no entry) + check 8 (no witness NVDA applied the ini key) | it 1 · reads 2 | `.andromeda/runs/2026-09-30T11-38-29-phase/` |
| SC | process gap: a plan edit after the mechanical checks but before the review prompt re-ran nothing; validation.md's re-run clause covers only edits during the review | dlg 1 · it 1 | 2026-09-30T13:20:54Z-b |
| SO | 4(9) baseline: a commit-only-on-the-word gate exited 0 with its evidence file absent; `test -f` fail-closed added | it 1 | `.andromeda/cache/p5-control-sr-os/` |
| CF | 4(0) UNPARSED: five baselines with an apostrophe in a single-quoted TOML literal | it 1 | 2026-09-30T16:57:54Z-b |
| FG | 4(2): "no boot-path change, no smoke" with main.rs in the modify-set; smoke entry added | it 1 | 2026-09-30T21:25:54Z-b |
| FG | check 6 size WARN 666 lines (50-entry fence) | — | 2026-09-30T21:25:54Z-c |
| IR | check 2: a `matrix#v3-09 notes` amendment on a capability this chunk claims | it 1 | 2026-10-01T19:09:56Z-b |
| IR | 4(5): a status atom cited to the wrong line, copied from the prior plan's entry | reads 1 | 2026-10-01T19:09:56Z-c |
| IR | 4(9): an add-only probe blind to a removed markdown bullet (`-- x`) | it 1 | `.andromeda/cache/p5-controls/2026-10-01T18-42-55-phase/` |
| SI | 4(8) no non-priming probe; check 2 evidence files absent; 4(6) mutating control not named one-shot; check 5 `{n}` slots | it 1 | 2026-10-01T21:36:48Z-b |
| SI | 4(9) control exposed a jointly-unsatisfiable pair (doc comment containing "tracing" vs a probe grepping it) | it 1 | `.andromeda/cache/p5-controls/2026-10-01T21-06-20-phase` |
| PA — | check 2 (a Pulse-facing item under Expected amendments); check 8 (NotFresh vs Unattributable); check 6 WARN 665 | it 1 | 2026-10-02T05:00:18Z-c |
| RR | check 6 WARN 683 (39-entry fence); check 7 WARN (graph unavailable on Linux); 4(4) prose resolution | it 1 | `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/plan.md` |
| HP | planlint 0-hit, then P5: 4(6) a criterion with no probe; 4(9) an unmarked new entry; check 5 `{cmd}` ×4; check 7 WARN | it 1 | 2026-10-04T00:56:25Z-b |

**Proposal:** three directions, each tied to specific rows. (a) The recurring 4(6)/4(9)/check 2/check 5 predicates
could move into planlint, so P4 stops authoring what P5 catches. Epoch 4 recorded the same direction from 8 cases,
and the count rose to 19 here. (b) Check 6's ~400-line guide WARNs on every live-round plan because of fence size
(UK, FG, PA, RR). The guide could measure prose apart from the fence. (c) validation.md's re-run clause could cover
the pre-review window (SC).

### P2 — wrap/curation (+ reconcile) · `recall.corpus-recurrence` (by type) — 16 cases · weight 31
**Pattern:** 14 records over 14 curation runs (10 chunks), plus 2 at reconcile. The recurring rules come in three
clusters: the Bash transport and heredoc rules (5, all stopped by the guard), cd persistence (3), and sweep patterns
narrower than the claim's wording (4).
**Evidence:** all 16 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS | a probe typed `tr "\134"` through a quoted heredoc and the transport halved it (host-win32 2026-09-23) | — | 2026-09-24T14:38:54Z-c |
| SS | a derived test total (1067) stated as measured (report template; T1 2026-08-09) | reads 1 | 2026-09-24T14:38:54Z-d |
| MG | a basename grep in the deferral void matched `security.md` in a doc comment (T1 false-positive face) | reads 2 | 2026-09-30T08:44:47Z-b |
| MG | a cascade pattern keyed on one wording missed a leaf variant (T1 pattern-bounded clause) | reads 1 | 2026-09-30T08:44:47Z-c |
| SG | two log-reading commands with a doubled backslash, blocked by the guard (host-win32 2026-09-23) | reform 2 | 2026-09-30T12:38:16Z-b |
| SC | two more doubled-backslash commands (sed, grep), blocked by the guard | retries 2 | 2026-09-30T14:05:45Z-c |
| SO (reconcile) | `grep -c` over multi-KB single lines and an escaped pipe under -E gave line counts and false zeros (T1 2026-09-17) | reform 1 | 2026-09-30T16:20:57Z-f |
| CF | the SR leg's case-insensitive stop tokens satisfiable by incidental text (T1 false-positive face) | it 1 | 2026-09-30T20:36:18Z-c |
| FG | a doubled-backslash command and a cat heredoc writing a file, each stopped only by the hook | retries 3 | 2026-10-01T00:12:13Z-b |
| IR | a phase run committed a hygiene-refusable file past security.md 2026-09-30; moved to cache on the overseer's word | dlg 1 | `.andromeda/runs/2026-10-01T18-42-55-phase/p5-dryrun.MOVED.md` |
| PA | the cascade sweep keyed on "permanently" missed "permanent" (T1 token-proxy 2026-09-06) | retries 1 | `.andromeda/runs/2026-10-02T12-53-46-wrap/curation.md` |
| FE | a cd at the head of a call moved the session cwd twice (host-win32 2026-09-08) | retries 2 | 2026-10-02T16:37:24Z-b |
| FE | a 0.2.0 census keyed on a separator set read 13 where the truth is 14 (T1 2026-09-06) | retries 1 | 2026-10-02T16:37:24Z-c |
| RR | cd persistence recurred three times this session, on the Linux host as well | — | 2026-10-04T00:37:48Z-b |
| RR | a count-keyed sweep found 7 of 15 tool-set sites (T1 2026-09-02: sweep what the claim SAYS) | reads 2 | 2026-10-04T00:37:48Z-c |
| HP (reconcile) | a `cd {run_dir}; for f in …` probe moved the session cwd (host-win32 2026-09-08) | — | 2026-10-04T01:25:01Z-b |

**Proposal:** the guard-caught cluster now costs one retry each, because the hook enforces the rule. The cd cluster
has no such enforcement and keeps recurring (L2). The sweep-width cluster has a partial mechanical home: cascade.py
already refuses a pattern whose control never fires (`2026-10-02T16:35:43Z-d`). A direction is to move recall
failures that a check can enforce into checks: a guard for cd, and a variant-wording probe beside each cascade
pattern. Prose extensions to CLAUDE.md T1 have not reduced recurrence; the type counted 7 in Epoch 4 and 16 here. The
sibling type `recall.curated-rule-not-applied` (2, UK) is the same family.

### P3 — mostly phase/distill · `contract.token-proxy-check` (universal, by type) — 14 cases · weight 20
**Pattern:** nine of the fourteen are the orchestrator's hand-written H2 cite check. It is keyed on the whole bold
span or a shell word-split, and history agents add qualifiers inside the bold. It misfired in nine of the epoch's
distill runs. The other five are one-offs keyed on a proxy token.
**Evidence:** all 14 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SG | shell H2 probe split bold markers on whitespace; word fragments read unresolved (all 107 resolve) | reform 1 · reads 1 | 2026-09-30T11:47:29Z-b |
| SC | shell for-loop over `grep -o` split multi-word markers; 17 false unresolved | reform 1 · reads 1 | 2026-09-30T12:56:02Z-b |
| FG | H2 checker took the whole bold span; 21 cites with a disambiguator read unresolved | retries 1 | 2026-09-30T20:51:18Z-b |
| IR | H2 probe keyed the whole bold text; 9 cites with an agent suffix read unresolved | reads 1 | 2026-10-01T18:51:11Z-b |
| SI | H2 probe split on spaces; 87 false MISS rows; all 101 resolve on the marker token | retries 1 | `.andromeda/runs/2026-10-01T21-06-20-phase` |
| PA — | scripted H2 check matched whole bold spans; 12 false misses | reads 1 | 2026-10-02T04:35:05Z-b |
| FE | H2 probe took every bold span as a cite; 6 false unresolved | reads 1 | 2026-10-02T14:50:22Z-b |
| RC0 | H2 extractor took the whole bold span; 22 false unresolved | reads 2 | 2026-10-02T22:36:15Z-b |
| RR | H2 validator keyed first a heading's first token, then a whole-bold prefix; both false UNRESOLVED | reform 2 · reads 1 | `.andromeda/runs/2026-10-03T22-20-49-phase/` |
| MG (fix-loop) | the deferral-void basename grep matched `security.md` in a doc comment (false positive) | reads 2 | 2026-09-30T08:20:31Z-c |
| MG (reconcile) | a sweep pattern required "reads" before `timeout.txt`; a leaf said "never `timeout.txt`" (false negative) | reads 1 | 2026-09-30T08:43:45Z-c |
| SC (validate) | own check-4(5) probe matched "atom from" case-sensitively against an "Atom from" note | reads 1 | 2026-09-30T13:20:54Z-c |
| CF (code) | E0-08's stop token "Blocked" is satisfiable by the Scenario cell `lamps-fixture-blocked` | reads 1 | 2026-09-30T18:21:17Z-d |
| NS | supplementary probe keyed working-route entries on `^- `; false EMPTY tail | reads 1 | 2026-10-01T18:41:15Z-b |

**Proposal:** ship the H2 cite check as a tool keyed on the marker token, or constrain the history-agent prompt so
the bold span holds only the marker. Counting P13's four `contract.extract-format` records, the same check misfired 13
times in this epoch and 3 times in Epoch 4. The NS row is the third `- `-introducer misread across the two epochs. A
route.py title listing would retire it. The MG fix-loop row is the defer-void false positive of P19.

### P4 — several steps · `tooling.host-shell` (universal, by type) — 9 cases · weight 21
**Pattern:** five cases were guard refusals of doubled-backslash commands. Two were cd persistence. Two were host
grep divergences: Windows `grep -ciF` aborting, and the Linux ugrep complexity limit.
**Evidence:** all 9 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS (code) | a quoted-heredoc probe's `tr "\134"` halved to `\`, read as octal; a substring check printed False for a present fix | — | `.claude/rules/host-win32.md` (2026-09-23 correction) |
| SG (fix-loop) | guard blocked two log-reading commands with a doubled backslash; scratchpad python | reform 2 | 2026-09-30T12:27:10Z-d |
| SC (research) | an inline `powershell -Command` probe with regex backslash pairs refused; ASCII scratchpad .ps1 | reform 1 | 2026-09-30T13:00:34Z-b |
| SO (fix-loop) | guard blocked three doubled-backslash commands (sed scrub, leaf rename, host-path elision) | reform 3 | 2026-09-30T15:19:02Z-f |
| FG (distill) | a sed patch with doubled backslashes blocked; anchored Edit | retries 1 | 2026-09-30T20:51:18Z-c |
| FE (fix-loop) | a host-path check blocked for a doubled backslash; the retyped form matched markdown bold; third run anchored | retries 2 | 2026-10-02T16:09:16Z-b |
| FE (report) | `grep -ciF` aborted (exit 134) inside `$(…)`, printing blank counts; a cd persisted in the same call | retries 2 | 2026-10-02T16:27:07Z-b |
| RC0 (distill) | a leading cd in the entity probe persisted the cwd | — | 2026-10-02T22:36:15Z-c |
| HP (report) | ugrep rejected `.{0,60}(a\|b\|c).{0,60}` (complexity limit) and printed no hits, reading like zero | retries 1 | 2026-10-04T01:20:20Z-b |

**Proposal:** see L1 (guard), L2 (cd) and L7 (host tools). This type has persisted across ten epochs (L10). The
direction stays at the tool and letter level: script-by-path as the default form, a cd guard, and a host-capability
probe at orientation.

### P5 — wrap/reconcile · `contract.in-pass-correction` — 8 cases · weight 21
**Pattern:** corrections inside reconcile fall into two recurring sub-shapes. The first apply of architecture
amendments overran the registry size target (SI and RR here, and RP in Epoch 4). Detector or cascade
inputs were mis-assembled (FE ×2, RR). The rest are one-off text slips.
**Evidence:** all 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| FG | a security-plan table cell first written with a backticked `\|` that would split the cell | retries 1 | 2026-10-01T00:11:08Z-b |
| SI | first-write A2/A3 pushed §Occupied Resources to 39137 B (over 38115); four compression passes | it 5 | `.andromeda/runs/2026-10-01T23-55-00-wrap/fanout-results.md` |
| SI | a security-summary parenthetical placed between "since" and the marker | it 1 | 2026-10-02T00:13:03Z-d |
| FE | the per-doc detector split skipped D-platform-claim (its `doc:` lists all seven); caught by a count vs 22 ids | retries 1 | `.andromeda/runs/2026-10-02T16-24-28-wrap/` |
| FE | a cascade pattern whose control never fired; cascade.py refused the sweep (exit 3) | retries 1 | `.andromeda/runs/2026-10-02T16-24-28-wrap/` |
| RR | first apply put §Established Decisions at 38830 B and §Occupied Resources at 38392 B over 38115; two re-measures | it 2 | 2026-10-04T00:36:13Z-b |
| RR | an obs-plan sidecar entry named the wrong §4 heading | it 1 | 2026-10-04T00:36:13Z-c |
| RR | a cascade pattern id of 17 chars refused (1–16) | retries 1 | 2026-10-04T00:36:13Z-d |

**Proposal:** two tool-level directions. (a) Measure the registry before the write, so a first apply cannot overrun:
apply could run `scripts/arch-registry-check.py measure` over the draft. Overruns cost 2, 5 and 2 iterations across
three wraps. (b) A shipped prompt and detector-split builder that honours a multi-doc `doc:` field. L5 is
the same cause seen through the problem facts.

### P6 — implement/fix-loop · `retry.fix-iterations` — 4 cases · weight 41
**Pattern:** the screen-reader legs needed repeated live fix rounds, gated by operator slots. CF alone took 4 rounds
over 7 slot runs with 9 dialogue rounds.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SC | arm S's session script failed twice before any stimulus (relative NVDA paths; PS5 has no `[ushort]`), invisible to the parse check | it 2 · retries 2 | 2026-09-30T13:49:13Z-b |
| SO | sr-empty #1 red: under OS Tab the reset cycle never read the BODY stop; injected reset cycle | it 1 · retries 1 | `conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/confound-control.md` |
| SO | live sr #1 red at S1-04: after a batched OS Shift+Tab NVDA logged later Tabs as shift+tab; delayed Shift release | it 1 · retries 2 | same |
| CF | 4 fix rounds over 7 slot-gated runs: BODY stop, caret position, S2-07 Shift+Tab + NVDA clock step + footer, pre-bind rows + aria-label | it 4 · retries 6 · dlg 9 | — |

**Proposal:** SC's script failed twice before any stimulus, on PS5 runtime-only errors. SO's reset-cycle and Shift
failures were found only live. One direction is a no-stimulus dry-run step before an operator slot: run the session
script's typed functions and key path with nothing sent. SO's 9-key probe and SC's later dry test are the working
examples. This is mostly project-level discovery of NVDA/OS-input behaviour, so the pipeline change is that slot
pre-check.

### P7 — several steps · `contract.narrow-basis-claim` (universal, by type) — 10 cases · weight 15
**Pattern:** a hand tally, a figure recalled from memory, or a claim written before its command ran (five cases).
The rest came from narrow globs or patterns, and from a doc-agent's false absence.
**Evidence:** all 10 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS (report) | the test count stated 1067 → 1077 with 1067 presented as measured; it was derived | reads 1 | 2026-09-24T14:04:47Z-b |
| SG (research) | the SR census glob `*nvda-pass.json` missed seven per-run-named records; scope stated the wrong last announced run | reform 1 · reads 2 | `conductor-0.2.0/chunks/2026-09-07-sr-findings-fixed/evidence/` |
| SG (plan) | fork 1 asked on a UIA claim drawn from one freeze stack without checking whose window it was | dlg 1 · reform 1 · reads 3 | `runs/sr-leg/nvda-speech.empty.log` |
| IR — (wrap) | a report draft dispositioned a security-plan hit without reading it | reads 1 | `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/report.md` |
| SI (reconcile) | the a11y doc-agent asserted "quiet window" absent; `grep -noF` finds it at :268 | reads 1 | `.andromeda/runs/2026-10-01T23-55-00-wrap/fanout-results.md` |
| FE (plan) | scope stated 15 committed capture files from a hand summary; `git ls-files` counts 14 | reads 1 | 2026-10-02T15:02:53Z-c |
| FE (route-resolve) | a CARRY stated 8 pinned tools from a recalled memory note; the contract holds 4 | retries 1 | 2026-10-02T16:39:05Z-b |
| RC0 (research) | 60 graph rows / 9 refs from a hand tally; the trace read 58 / 8 | reads 1 | `.andromeda/runs/2026-10-02T22-27-37-phase/tree-query-…json` |
| RC0 (research) | a `grep → 0` claim written before the grep ran (it then held) | reads 1 | 2026-10-02T22:42:55Z-e |
| RR (report) | a count-keyed sweep found 7 sites; an enumeration-by-name sweep found 8 more | reads 2 | 2026-10-03T23:47:20Z-b |

**Proposal:** the same family as P2 and as Epoch 4's P6/P9. The write-time direction is that any stated count or
absence carries its producing command beside it, and a writer refuses one without it. This would catch the five
written-before-derived cases at the moment of writing.

### P8 — several steps · `contract.premise-falsified` (universal, by type) — 7 cases · weight 14
**Pattern:** three of the seven are folded CARRY/CONTEXT premises that were false when taken up (UK, SG, FG). Two are
plan premises about live NVDA or Pulse behaviour, and two are P3 inferences about Pulse source.
**Evidence:** all 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UK (take-up) | the :63 CARRY says claim-ownership.ts "already names all four"; it has three rows and no SC 2.4.7 | reads 1 | `conductor-0.3.0/chunks/2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed/scope.md` |
| SG (research) | the folded CARRY named the 2026-09-04 record as the last announced run; the 2026-09-07 records heard later | reads 1 | research.md Variable ledger (2026-09-30T11:58:22Z-c) |
| CF (code) | research/scope: T-01 drivable as a second in-session run with no condition; Pulse's canary dedupe needs ≥150 s quiet | reads 2 | 2026-09-30T18:21:17Z-c |
| CF (fix-loop) | plan step 7 starts browse walks from BODY; NVDA's caret follows last DOM focus and binds only at the first OS Tab | it 2 · retries 2 | 2026-09-30T20:05:40Z-d |
| FG (research) | folded CARRY C2: twelve knip findings and an a11y-plan §11 rule; knip reports 15 and §11 holds no such rule (it came from frontend.md:54) | reads 2 | 2026-09-30T20:57:42Z-b |
| PA — (phase) | P3 inferred P-037 needs Pulse's Report window open without reading `isOpen = effectiveId !== null`; caught after approval | dlg 1 · it 1 · reads 3 | `.andromeda/runs/2026-10-02T04-26-44-phase/` |
| RC0 (research) | the round-request says creation records no incident event; Pulse S writes `created`, returned as `unknown` | reads 2 | `conductor-0.3.0/chunks/2026-10-02-p-075-re-round-on-incident-events/research.md` |

**Proposal:** the CARRY trio connects to P16 (carry-context gaps). A direction at the CARRY writer (wrap
route-resolve) is to tag each CARRY fact as measured, with its pointer, or inferred, and to anchor coordinates by
content rather than line number. Take-up then knows which clauses to re-verify. Every case was caught before it did
damage.

### P9 — implement/fix-loop · `tooling.environmental` — 6 cases · weight 16
**Pattern:** six environmental failures across three chunks. Two came from parallel sessions sharing the host
(foreground theft, CPU contention). Two came from first contact with the Linux host (a Wayland crash, Windows-pinned
tests). The other two were the NVDA focus-event host state and the advisory-db residue.
**Evidence:** all 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UK | NVDA binds the window, then hears no focus event until close; a base-bundle control reproduces it; regrade unobtainable on this host today | retries 3 | evidence/driven-leg.md Findings 2 |
| UK | a parallel agent session (pulse-builder) took the OS foreground from the SR leg twice | retries 1 · dlg 1 | evidence/driven-leg.md Findings 1 |
| SI | workspace nextest timed out at 1800 s during the conductor-tauri link, concurrent with another session's Pulse cargo build | retries 1 | `.andromeda/runs/2026-10-01T21-38-30-implement` (entry 6) |
| SI | advisory-db local copy carried an untracked RUSTSEC-0000-0000.md residue; re-read against a fresh clone | reads 1 | same (entry 15) |
| RR | first pulse-app launch on Linux crashed on a Wayland protocol error; relaunched with WEBKIT_DISABLE_DMABUF_RENDERER=1 on the operator's ruling | retries 1 · dlg 1 | 2026-10-03T23:30:42Z-b |
| RR | `cargo nextest --workspace` red on Linux: 6 conductor-tauri tests pin the Windows mock-webview origin | reads 2 | 2026-10-03T23:30:42Z-d |

**Proposal:** a pre-leg environment check, which Epoch 4's P15 also suggested, now has more to check: no competing
session on the slot (foreground, CPU), a display-backend handle on Linux, and the advisory-db cache's currency. The
RR tests case was repaid by HP in the same epoch. See L8 (shared host) and L11 (advisory-db).

### P10 — wrap/reconcile · `ambiguity.playbook-no-match` — 6 cases · weight 9
**Pattern:** this recurred in 6 chunks, after 3 in Epoch 4. Each proposal was applied on a recorded direction (P5
plan, operator relay, founder ruling) with "a rule to be proposed at the wrap card". Four of the six took no
dialogue.
**Evidence:** all 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| MG | test-plan §9 per-chunk → epoch-boundary reversal matched no rule; applied on recorded direction | — | 2026-09-30T08:43:45Z-b |
| SG | a configuration-bound SR finding extending a11y-plan §3's platform set matched no rule | — | 2026-09-30T12:36:52Z-b |
| SC | A1/A2 matched no rule (:149 clause (a) fails); applied under the relay's recorded direction | — | 2026-09-30T14:03:57Z-b |
| CF | extending the no-sleep ban's carve-out matched no rule; escalated, operator ruled | dlg 1 | 2026-09-30T20:34:41Z-b |
| FE | four proposals + one orchestrator-raised amendment matched no rule; settled by founder ruling + P5 list | — | 2026-10-02T16:35:43Z-b |
| HP | an arch proposal labelled D-platform-claim, whose class the sentence does not hold; applied on the plan's expected-amendment entry | reform 1 | `.andromeda/runs/2026-10-04T01-18-54-wrap/fanout-results.md` |

**Proposal:** the project has already absorbed Epoch 4's direction in practice: it applies on a recorded direction
and proposes the rule later. The remaining pipeline step is to codify that a recorded upstream direction (P5-approved
Expected amendment, founder ruling, operator relay) is a governing basis at reconcile. The playbook rule would then
be optional. HP adds a detector-class gap: "a measured value stated without its host".

### P11 — wrap/reconcile · `contract.cascade-miss` — 6 cases (+2 step-less) · weight 9
**Pattern:** a restatement of an amended claim, in a master or a leaf, that no detector proposed. In every case the
orchestrator's cascade sweep, or a hand read, caught it before commit.
**Evidence:** all 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SO | a11y-plan:268 kept "the agent arm — WebDriver-injected keys…" after the first apply edited the same line | it 1 | `.andromeda/runs/2026-09-30T15-22-00-wrap/cascade-dispositions.md` |
| SO | CLAUDE.md:34 and gotchas.md:54 listed the SR leg's children as NVDA alone since 2026-09-02; found by a read, not a pattern | reads 1 | same |
| CF | a11y-plan :113 restated a retired claim; the a11y agent swept §3 :268 only | it 1 | `.andromeda/runs/2026-09-30T20-18-56-wrap/cascade-dispositions.md` |
| CF | test-plan :307 "one preflight canary" falsified; the detector proposed the row but not this clause | reads 1 | 2026-09-30T20:34:41Z-d |
| IR — | architecture.md:113 summarizes security-plan's residual set; no detector proposed it | reads 1 | `.andromeda/runs/2026-10-01T20-39-22-wrap/cascade-dispositions.md` |
| IR — | the tests-summary.md:22 leaf still named the 2026-09-29 series as latest (the prior wrap's cascade left it) | reads 1 | same |
| SI | security-plan.md:222 restated the CONDUCTOR_RUNS_DIR reader enumeration; no detector proposed it | reads 1 | `.andromeda/runs/2026-10-01T23-55-00-wrap/cascade-dispositions.md` |
| PA | the cascade regex keyed on "permanently" missed the leaf's "permanent `degraded_mode`" | retries 1 | `.andromeda/runs/2026-10-02T12-53-46-wrap/cascade-dispositions.md` |

**Proposal:** the cascade sweep is doing the detectors' duplicate-sweep job. Epoch 4 had an untyped
"duplicate-sweeps under-ran" record too. One direction is for the doc-agent prompt to require a per-occurrence sweep
of each proposal's claim across the whole master and its leaves before returning. Another is to make the cascade
sweep the named owner of restatements and size it accordingly.

### P12 — wrap/curation · `ambiguity.filter-borderline` — 6 cases · weight 7
**Pattern:** in five of six chunks the deciding candidates scored exactly 0.6. Epoch 4 recorded the same in all three
of its cases.
**Evidence:** all 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS | both applied Tier-3 learnings scored exactly 0.6 before the conditional +0.2 | — | 2026-09-24T14:38:54Z-b |
| UK | one candidate scored exactly 0.6, rejected; its home became a route CARRY | — | `.andromeda/runs/2026-09-30T11-12-38-wrap/curation.md` |
| SC | three host-tool gotchas each exactly 0.6 before +0.2; the max-3 cap chose by consequence and deferred two | deferred 2 | 2026-09-30T14:05:45Z-b |
| SO | a learning scored exactly 0.6, rejected by the lean default (fact amended into two masters) | — | 2026-09-30T16:22:47Z-b |
| IR | two measured candidates exactly 0.6, rejected (homes elsewhere this wrap) | — | 2026-10-01T20:53:57Z-b |
| HP | the overseer's "anchor diff-shaped probes to the chunk base" read as both general and per-chunk; filtered task-specific | reform 1 | 2026-10-04T01:26:19Z-b |

**Proposal:** across two epochs, 8 cases put 16 candidates exactly on the threshold (7 in Epoch 4, 9 here). The scoring granularity appears
to land many candidates on 0.6. Directions: state the tie rule explicitly, add a finer signal, or move the threshold
off a reachable sum. SC's two deferred learnings have no later home (see appendix, deferred-forever, n=1).

### P13 — phase/distill · `contract.extract-format` — 4 cases · weight 4
**Pattern:** history agents put a disambiguating qualifier inside the bold marker. This is the producer side of P3's
H2 misfires.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UK | security history agent wrote 4 bold cites as marker + parenthetical qualifier | reads 1 | `.andromeda/security-plan-amendments.md:307,313,415,422` |
| SC | three history agents put a parenthetical inside the bold marker (arch 5, security 2, tests 1) | reads 1 | 2026-09-30T12:56:02Z-c |
| SO | the bold span carried marker + qualifier (security 1, tests 2); read resolved all 3 | reads 1 | `.andromeda/runs/2026-09-30T14-14-44-phase/` |
| CF | three cites carried a parenthetical sub-entry label after the marker | reads 1 | 2026-09-30T16:39:59Z-c |

**Proposal:** same fix as P3, from the producer side: the history-agent prompt could say "bold holds the marker
only; any qualifier goes after the bold".

### P14 — several steps · `contract.structural-blind-spot` (universal, by type) — 5 cases · weight 8
**Pattern:** three cases are drift-base detector classes that do not exist: a CI gate-set enumeration, a shipped
mechanism retiring a doc's described mechanism, and a report's "Spec claims disproved" bullet. One is a feature-gated
test target built by nothing. One is a drive-spacing rule blind to Pulse's span buffer.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS (reconcile) | no obs/security detector covers a CI gate-set enumeration; third CI-gate chunk running; D-obs-ci-gates minted | dlg 1 | 2026-09-24T14:37:38Z-c |
| UK (reconcile) | design-system's three detectors cannot see a shipped mechanism retiring the doc's own; the floor supplied all three amendments | reads 1 | `.andromeda/runs/2026-09-30T11-12-38-wrap/fanout-results.md` O1-O3 |
| IR — (implement) | the pre-registered drive spacing cannot see Pulse's span-identity buffer; d2's emission voided | — | `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/evidence/attempt-ledger.md` |
| PA (reconcile) | 5 of 12 applied amendments came only from the Expected-amendments floor; no detector keys on "Spec claims disproved" | reads 5 | `.andromeda/runs/2026-10-02T12-53-46-wrap/fanout-results.md` |
| RR (fix-loop) | the stub-server-gated preflight_spawn.rs is built by no CI job or harness verb; uncompilable for seven weeks | it 1 | 2026-10-03T23:30:42Z-c |

**Proposal:** (a) Drift-base detectors could cover three classes: gate-set enumerations, mechanism retirement, and
the report's "Spec claims disproved" bullet. Epoch 4's DL case and chain X6 point the same way. (b) RR's case matches
Epoch 4's P17: feature-gated test targets are invisible to both the code-graph and CI. One direction is a gate step
`cargo check --tests --all-features` (or the named feature set), so they at least compile somewhere.

### P15 — phase/plan · `retry.synthesis-rework` — 4 cases · weight 8
**Pattern:** P4 drafts contradicted themselves, or their own drafts, and were caught on read-back.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| MG | the fixture table contradicted steps 2–3 (tally-missing count, stdout notice, loader line shape); 6 anchored edits | it 1 | 2026-09-30T08:10:14Z-b |
| SC | the whole-plan rewrite's step-7 skip rule listed a RUN branch inside its SKIP list | it 1 | 2026-09-30T13:16:14Z-b |
| CF | two fence atoms rewritten (a trailing-space last-line atom; a cutoff from state.yaml vs the base commit's time) | it 1 · reads 1 | 2026-09-30T16:54:13Z-b |
| FE | scope/research cited the residual values verbatim, so the chunk's own drafts became residual sites | it 1 · reads 2 | 2026-10-02T15:02:53Z-b |

**Proposal:** every case was caught by the author before P5. The pipeline signal is that planlint has no
table-vs-steps consistency check. A direction is a planlint arm that cross-reads a plan's fixture or expectation
tables against the steps they summarize.

### P16 — phase/take-up · `input.carry-context-gap` — 5 cases · weight 5
**Pattern:** a route annotation (CARRY/PREREQ/CONTEXT) under-specified what take-up needed. Two PREREQs named "close
rust gate deferral" without naming the gates. Coordinates were stale. One ruling's text had been flip-compacted off
the route.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SS | the :55 CARRY cites ci.yml:61-70 / :64-72; measured :61-63 / :64-71 | reads 1 | 2026-09-24T10:50:04Z-b |
| MG | CONTEXT 2 frames timeout rostering as judgment; the unstated constraint is that no (file, mutation) keys survive | reads 3 | `conductor-0.3.0/chunks/2026-09-30-mutation-gate-grades-every-tally-it-rests-on/scope.md` |
| SC | PREREQ "close Rust gate deferral" names neither the gate entries nor where they are recorded | reads 3 | 2026-09-30T12:49:23Z-b |
| CF | the same PREREQ again; which defer stood came from the origin chunk's Gates table | reads 1 | 2026-09-30T16:33:20Z-b |
| RC0 | CONTEXT cites "founder ruling 2026-10-02 (as above)" but the ruling was flip-compacted off; found in route-archive.md:119 | reads 1 | 2026-10-02T22:29:17Z-c |

**Proposal:** directions at the annotation writer (wrap gates and route-resolve). A PREREQ template could name the
deferred entries and their record. Coordinates could be anchored by content. Flip-compaction could leave a
route-archive line pointer when compacting text another line cites "as above". Chain X5 (working-entry `thin` ×2)
and P8's CARRY trio are the same writer side.

### P17 — implement/code · `input.plan-step-ambiguous` — 5 cases (+1 step-less) · weight 6
**Pattern:** this persisted from Epoch 4's P2, at a lower rate (5/14 against 8/8). Each step was silent on an edge,
fixed a value the code cannot hold, or named something the dependency graph does not expose.
**Evidence:** all 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| MG | step 3 silent on an absent tally file; only the step-7 table implies skipping | — | 2026-09-30T08:19:09Z-b |
| CF | step 4 fixes the footer at `--space-sm` (8px), which cannot hold a 12px line | reform 1 | 2026-09-30T18:21:17Z-e |
| SI | step 3's helper takes a request type not nameable from conductor-run's normal deps; conductor-emit re-export | reads 2 | 2026-10-01T21:54:30Z-b |
| SI | step 5 (ii) asserts an exception fingerprint ATTRIBUTE the wire does not carry | reads 1 | 2026-10-01T21:54:30Z-c |
| PA — | steps 4–5 describe pin tests whose evidence exists only after step 9's round | — | 2026-10-02T05:04:49Z-b |
| RR | step 7 fixed the line keys but not the value printed for an empty events list | — | 2026-10-03T23:25:05Z-b |

**Proposal:** as in Epoch 4: a validate check that each step naming a type, dependency or design token agrees with
that artifact at HEAD (SI, CF), plus edge-case prompts for absent and empty inputs (MG, RR). Chains X2/X6 show plan
consumed as `thin` by fix-loop and code in five chunks.

### P18 — wrap/gates · `tooling.commit-mechanics` — 3 cases · weight 4
**Pattern:** two consecutive wraps wrote the commit's Coverage line as "11/11" without a reading (true: 10 verified +
1 deferred). One wrap committed CRLF run-dir files written by python text mode.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SC | `git add -A` warned CRLF on 13 run-dir files (python text-mode prompts, captures, phase logs) | — | 2026-09-30T14:11:52Z-b |
| PA | the commit's Coverage line "version 11/11" unmeasured; found after the push, left uncorrected | reads 3 | `git show 31f9d92` |
| FE | the Coverage line written 11/11 again; amended before the stamp and push after a direct ledger read | retries 1 | 2026-10-02T17:01:15Z-b |

**Proposal:** generate the Coverage line from `matrix.py`, which would need a summary verb, since FE notes that
`show` refuses an unfiltered listing. Have the pipeline's own python writers open files with `newline=''`. The CRLF
learning is already curated (host-win32 2026-09-15) and recurred, `2026-09-30T11:35:31Z-b`.

### P19 — implement/fix-loop + wrap/gates · `tooling.gate-deferral` — 6 cases (3 + 3) · weight 3 each
**Pattern:** the zero-Rust-delta deferral was re-pinned through MG → SG → SO, each pinned as a PREREQ on the next
entry. In MG and SG, the void check's basename grep matched `security.md` in a doc comment. The same false positive
appears in Epoch 4 (`2026-09-24T08:08:14Z-b`).
**Evidence:** all 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| MG (fix-loop) | nextest + clippy deferred on zero Rust delta; the basename grep hit once and did not void | deferred 2 | 2026-09-30T08:20:31Z-b |
| MG (gates) | deferred again; every Rust hit a comment line; pinned as PREREQ on the next entry | deferred 2 | 2026-09-30T08:47:44Z-b |
| SG (fix-loop) | deferred; the one basename hit (`security.md`) was a doc comment | deferred 2 | 2026-09-30T12:27:10Z-c |
| SG (gates) | deferred again; the doc-comment false positive again; re-pinned on the successor | deferred 2 | 2026-09-30T12:45:01Z-b |
| SO (fix-loop) | deferred; secret_scan_gate (reads untracked files) run alone, 5/5 | deferred 2 | 2026-09-30T15:19:02Z-e |
| SO (gates) | the unit defer voided (secret_scan_gate reads every file and the wrap changed masters); ran bare 1136/1136; clippy stood | deferred 1 | 2026-09-30T16:29:04Z-b |

**Proposal:** the void check could match repo paths rather than basenames, or skip comment lines. That retires a
false positive seen five times across two epochs (with P3's MG row and P2's MG row). SO's case shows a real void
condition: a Rust test reads untracked files. It could be a standing void rule rather than a per-chunk discovery. The
re-pinned PREREQ is what P16's SC/CF rows consumed as under-specified.

### P20 — phase/take-up (+ validate, —) · `input.out-of-pipeline-source` — 3 cases (+1 step-less, +1 validate with halt) · weight 3
**Pattern:** the decisive facts lived in the sibling Pulse repo: the clearing event, the round request, binary
digests, and the fact that cancelled RC0. This persisted from Epoch 4's P18 and now carries a halt.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IR | the BLOCKED-ON clearing event lives in the Pulse repo; verified by read-only git there | reads 2 | 2026-10-01T18:44:37Z-b |
| PA — | the assertion set, S and the binary identity live in Pulse's round-request.md / round-binary.md | reads 5 | 2026-10-02T04:28:25Z-b |
| RC0 | the round's decisive content lives in the Pulse repo (round-binary.md uncommitted) | reads 4 | 2026-10-02T22:29:17Z-b |
| RC0 (validate) | the fact that cancelled the chunk (S writes `created`, egressed as `unknown`) lives in the Pulse repo | halted 1 | `.andromeda/runs/2026-10-02T22-27-37-phase/cancel-record.md` |
| RR | the binary's sha256 values live only in an uncommitted Pulse working-copy edit; the relay said committed | reads 2 | `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/scope.md` |

**Proposal:** cross-repo rounds are now the normal shape. One direction is a sanctioned cross-project input channel
for take-up and research: the relay plus the cited Pulse files snapshotted into the chunk folder, with commit or
working-copy provenance. Then the decisive facts sit inside phase's input contract.

## Cross-step chains (starting heuristics)

### X1 — phase/research →research→ phase/plan — 3 chunks (UK, SG, SI) — also 3 chunks in Epoch 4
Producers closed `ok` while signalling `unresolved-questions` each time. SG's research also carried `narrow-basis`
and `premise-falsified` frictions and was consumed as `wrong`. Consumers: `2026-09-30T09:14:57Z-a`,
`2026-09-30T12:06:36Z-a`, `2026-10-01T21:30:01Z-a`. **This is the formally-ok-but-signalled shape again, across two
epochs and 6 chunks.** In UK and SI the fork was decided at P4 and needed reads P3 had not done. Direction: let an
`unresolved-questions` signal name the reads the plan will need under each fork arm.

### X2 — phase/plan →plan→ implement/fix-loop — 3 chunks (UK, CF, RR)
UK: the plan's `size-over-guide` and three designed dialogues fed an unsatisfiable atom (`Spec Files:` + TAB). CF:
`authority-resolved` ×2 fed the gate-8 allowlist and browse-start premises. RR: the plan's launch env lacked the
display-backend handle, and a gate named a test that had not compiled. Directions: P17 and P9.

### X3 — phase/plan →plan→ phase/validate — 3 chunks (SG, PA, RC0)
SG: check-4 resolutions. PA: `plan-oversize-643-lines` fed check 2/8/6. RC0: plan `missing`, cancelled before P4
wrote it. This is P1 seen as lineage.

### X4 — (pipeline artifact) →wrap-playbook→ wrap/reconcile — 3 chunks (SG, CF, FE) — 4 chunks in Epoch 4
Consumers: `2026-09-30T12:36:52Z-a`, `2026-09-30T20:34:41Z-a`, `2026-10-02T16:35:43Z-a`. Direction: P10.

### X5 — (route annotation) →working-entry→ phase/take-up — 2 chunks (UK, FE)
UK `2026-09-30T08:52:36Z-a` (the entry framed all four claims as unasserted; three looked unbuilt). FE
`2026-10-02T14:42:03Z-a` (the subject class was unstated). Direction: P16.

### X6 — phase/plan →plan→ implement/code — 2 chunks (CF, SI) — also 2 in Epoch 4
CF `2026-09-30T18:21:17Z-a` (four letter details failed against the code), SI `2026-10-01T21:54:30Z-a` (an
un-nameable type; an attribute absent from the wire). Direction: P17.

Single-chunk shapes: smoke→implement-outcome→report (MG) · plan→report (UK) · operator-directive→route-resolve (SG)
· operator-directive→report (SO) · research→fix-loop (SO) · report→reconcile (SO) · plan→gates (CF) · ci-verdict→take-up
(FG) · research→scope→validate `wrong` (PA) · smoke→conversation→curation (PA) · handoff→take-up (RR) ·
matrix/handoff/working-route→orientation (NS). **Plan is the hub of this epoch:** consumed as `thin`/`wrong`/`missing`
by validate, code, fix-loop, report and gates across 9 chunks.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 13 facts (T1: the Bash guard refuses heredoc-to-file and doubled-backslash commands)
**Facts:** `2026-09-30T10:06:53Z-a#0` UK code (python heredoc to file) · `2026-09-30T11:58:22Z-a#0` SG research
(sed backslash scrub) · `2026-09-30T12:12:11Z-a#0` SG validate (two scrub/control commands) ·
`2026-09-30T13:00:34Z-a#0` SC research (inline powershell) · `2026-09-30T21:35:57Z-a#0` FG code (cat heredoc) ·
`2026-09-30T23:47:14Z-a#0` FG fix-loop (two backslash commands) · `2026-10-01T00:31:13Z-a#1` FG gates (host-path
probe) · `2026-10-01T19:09:56Z-a#0` IR validate (baselines) · `2026-10-01T19:14:05Z-a#0` IR code (cat heredoc
script) · `2026-10-01T21:54:30Z-a#0` SI code (python heredoc) · `2026-10-02T05:04:49Z-a#0` PA code (`cat >>`
append) · `2026-10-04T00:36:13Z-a#0` RR reconcile (cat heredoc append) · `2026-10-04T01:25:01Z-a#1` HP reconcile
(cat heredoc). Seven are environment and six are process, all workaround. Correlates: P4's five guard-caught rows,
P2's five transport rows, and `recall.curated-rule-not-applied` `2026-09-30T11:28:40Z-c`. Prior epoch: Epoch 4 L1
(9 facts).
**Level hypothesis:** the guard works: nothing corrupt ran. But the forms it refuses are still the first forms
authored, 13 times here and 9 in Epoch 4. The last two facts are on the Linux host, where the backslash-collapse
hazard the guard was built for may not apply, while its heredoc arm still fires.
**Proposal:** the skill letters could name the Write-tool + script-by-path form as the first form, so authoring
starts where the guard ends up. Whether each guard arm still fits the Linux host is a separate check for the founder.

### L2 — band-aid — 6 facts (T2: a leading cd persists, or is refused)
**Facts:** `2026-09-30T11:47:29Z-a#1` SG distill · `2026-09-30T14:29:09Z-a#0` SO research (three calls) ·
`2026-10-03T22:29:27Z-a#0` RR distill · `2026-10-03T22:35:26Z-a#1` RR research · `2026-10-04T00:50:46Z-a#1` HP
research (a cd refused by a `./secrets/**` deny rule it could not resolve) · `2026-10-04T00:57:52Z-a#0` HP code
(the same). Correlates: P4 rows FE-report and RC0, three untyped records, and P2's three cd recalls. Prior epoch:
Epoch 4 `2026-09-24T06:34:29Z-b`, `2026-09-23T09:48:36Z-c`.
**Level hypothesis:** the curated rule (host-win32 2026-09-08) carries a "recurrence-despite-learning" deferral in
consecutive handoffs. Unlike the transport rules, nothing enforces it, and it recurs on both hosts.
**Proposal:** a PreToolUse check that flags a Bash command beginning with `cd` (or containing `cd X &&` / `cd X;`),
in the same family as the heredoc guard. That moves the rule from recall to a check. It would also stop the
deny-rule refusals HP met.

### L3 — band-aid — 8 facts (T3: tool output piped against the run-bare rule)
**Facts:** `2026-09-30T08:00:38Z-a#0` MG distill (`registry.py | tail -n 3`) · `2026-09-30T08:59:06Z-a#0` UK
distill (`sidecar.py | grep | head`) · `2026-09-30T14:23:10Z-a#0` SO distill (`| head -3`) ·
`2026-09-30T14:40:29Z-a#0` SO validate (`gate.py dry-run | grep -v`) · `2026-09-30T20:51:18Z-a#0` FG distill (`|
grep | head`) · `2026-10-01T19:05:05Z-a#0` IR plan (`planlint | tail`) · `2026-10-01T21:14:43Z-a#0` SI distill (`|
head -3`) · `2026-10-01T21:30:01Z-a#0` SI plan (`planlint | grep`). All are process/workaround, and every note
says the verdict line was still read. Prior epoch: `2026-09-29T20:21:47Z-a#0`, `2026-09-30T04:35:12Z-a#0`.
**Level hypothesis:** the agents keep trimming the same tools' output (sidecar.py summary ×4, planlint ×2). That
suggests the default output is longer than a reader wants, and the rule fights the tool rather than the risk. The
risk the rule guards against, a masked exit or a clipped verdict, never materialised in these 8.
**Proposal:** give the tools a compact default (verdict line first and last, rows behind `--verbose`). Alternatively,
the letter could allow a pipe when the tool's verdict line is fixed-position and the exit is read via `PIPESTATUS`.

### L4 — band-aid — 3 facts (T4: run-dir hygiene by hand)
**Facts:** `2026-09-30T14:11:52Z-a#0` SC gates (hygiene refused the wrap's own build-prompts.py and relay-named logs;
redacted in place; a first rewrite broke a line) · `2026-09-30T16:29:04Z-a#0` SO gates (hygiene flagged 2; a grep
found 8 scripts with absolute paths) · `2026-09-30T08:14:08Z-a#0` MG validate (removed-cause: a synthetic leak
control first written into the committed run dir). Correlates: untyped `2026-09-30T16:29:04Z-c` and P2's IR row.
Prior epoch: Epoch 4 `2026-09-29T18:22:32Z-a#0` (65 host paths in tool trails, "as the prior wrap did").
**Level hypothesis:** pipeline-written scripts and trails embed absolute paths, and the hygiene read sees only part
of them. Each wrap scrubs by hand.
**Proposal:** pipeline tools and generated scripts could write repo-relative paths at source (for example, the tools
dir via an env lookup, as SO did by hand). The hygiene read could cover every run-dir file type it is meant to.

### L5 — band-aid — 3 facts (T5: reconcile prompts hand-assembled)
**Facts:** `2026-09-30T11:27:00Z-a#0` UK (`{detectors_yaml}` passed as a pointer) · `2026-10-02T16:35:43Z-a#0` FE
(the same, instead of ~25 KB inline) · `2026-10-04T01:25:01Z-a#0` HP (the extractor keyed each detector to its first
doc token; D-platform-claim distributed by hand). Correlates: P5's FE row and untyped `2026-09-30T16:20:57Z-e`. Prior
epoch: Epoch 4 `2026-09-30T07:39:08Z-c`, the same D-platform-claim parse. That makes four occurrences over two epochs.
**Level hypothesis:** the letter asks for verbatim inlining that every wrap replaces with a pointer, and each wrap
re-writes its own detector splitter, re-introducing the same multi-doc bug.
**Proposal:** ship the splitter and prompt builder as a wrap-session script that handles multi-doc `doc:` fields and
reports per-prompt detector counts. The letter could sanction the pointer form it already gets.

### L6 — band-aid — 4 facts (T6: live legs driven outside gate.py)
**Facts:** `2026-09-30T13:49:13Z-a#0` SC fix-loop (entry 11 split so the driver never ran before its signature read)
· `2026-10-02T05:51:08Z-a#0` PA fix-loop (legs H/D/R/F via a scratch sequencing script) ·
`2026-10-03T22:51:37Z-a#0` RR validate (`--id` assumed to select an entry; ran the whole fence until stopped) ·
`2026-10-03T23:30:42Z-a#3` RR fix-loop (a scratch sequencing script per leg). Correlates: untyped
`2026-10-03T22:51:37Z-c` ("validation.md names baseline runs as P5's own work with no tool form") and
`2026-09-30T13:49:13Z-c`. Prior epoch: Epoch 4 L2.
**Level hypothesis:** gate.py has no sequenced-leg mode and its flag semantics surprise. Live rounds are driven
beside the tool, from scratch scripts built from the entries' own text.
**Proposal:** a gate.py sequence mode (ordered entries, stop on first red, bare exits), a select-one-entry flag whose
name says so, and validation.md naming the tool form for P5 baselines.

### L7 — band-aid — 5 facts (T7: host tool absent or divergent)
**Facts:** `2026-10-02T16:27:07Z-a#0` FE report (`grep -ciF` aborts, Windows) · `2026-10-02T22:26:05Z-a#0` NS
(`du -sh` aborts, Windows) · `2026-10-03T22:35:26Z-a#0` RR research (code graph unavailable: duckdb absent, Linux) ·
`2026-10-04T00:50:46Z-a#0` HP research (the same) · `2026-10-04T01:20:20Z-a#0` HP report (ugrep complexity limit,
Linux). All are environment/workaround. Correlates: `tooling.graph-unavailable` ×2, `tooling.graph-refresh-stale`
×2, research `ok-degraded` ×2.
**Level hypothesis:** each host has its own gaps, and each is discovered mid-step by a silent wrong result: blank
counts, an empty section, or a missing graph.
**Proposal:** a host-capability probe at orientation (grep flavour and `-i` health, duckdb/protobuf importable, du)
that prints what each later step should avoid. Health check 11 already WARNs on the graph, and the rest could join
it.

### L8 — band-aid — 3 facts (T8: parallel sessions share the host)
**Facts:** `2026-09-30T10:56:19Z-a#1` UK fix-loop (overridden: the operator quieted the pulse-builder session that
took NVDA's foreground) · `2026-09-30T20:05:40Z-a#2` CF fix-loop (removed-cause: NVDA started while the overseer's
terminal held the foreground; every window minimized before the re-fire) · `2026-10-01T23:22:23Z-a#0` SI fix-loop
(resources: nextest hit 1800 s under another session's build). Correlates: `tooling.environmental` UK-d and SI-b.
**Level hypothesis:** the multi-session workflow (overseer, pulse-builder and conductor on one desktop) is a
standing condition. Each leg rediscovers it as foreground theft or contention.
**Proposal:** a slot protocol in the harness or letters: before a foreground-sensitive or heavy leg, check for and
announce competing sessions, or take a host-wide slot lock. That replaces per-leg operator intervention.

### L9 — override — 7 facts with halt and soft-exit impact (a relayed directive superseded by the founder's live word)
**Facts:** `2026-09-30T13:16:14Z-a#0` SC plan (two live founder updates relayed mid-P4; re-synthesized twice) ·
`2026-09-30T16:24:10Z-a#0` SO route-resolve (the relay's route directive superseded by the founder's later ruling
retiring arm K) · `2026-10-02T00:13:03Z-a#0` and `2026-10-02T00:15:11Z-a#0` SI reconcile and route-resolve ("route
unchanged" overridden for one CARRY) · `2026-10-02T13:10:44Z-a#0` PA route-resolve ("Route unchanged (directive)"
overridden by the founder's live ruling) · `2026-10-03T07:45:09Z-a#0` RC0 plan (soft_exit 1: the menu answer
overridden to HOLD) · `2026-10-03T07:45:37Z-a#0` RC0 validate (halted 1: the cancel on the founder's word).
Correlates: untyped `2026-10-03T07:45:09Z-b` ("a menu answer relayed through a delegate was not the decider's word"),
and `operator-directive` consumed `thin` at SG `2026-09-30T12:40:34Z-a` (a stale line number) and SO
`2026-09-30T16:09:11Z-a`.
**Level hypothesis:** the pipeline treats a relayed directive, or a delegate's menu answer, as settled. The decider
then revises it. The rule being corrected is not a skill rule; it is the absence of one about how long a relayed word
binds. The security plan's 2026-09-29 entry already makes the founder's live word the only ratifier for a widening.
**Proposal:** record each relayed directive with its source and time, and treat a delegate-recorded answer to a
founder-owned fork as provisional until the founder's word is recorded. Then a later live word supersedes it by rule
rather than by override.

### L10 — chronic-degrade — four `tooling.*` lines, never halting
**Facts (per-epoch counts, read-only lookback):** `tooling.host-shell` 0.2.0 E3:14 · E4:13 · E5:3 · E6a:4 · E6b:8;
0.3.0 E1:10 · E2:2 · E3:2 · E4:5 · **E5:9**. `tooling.gate-deferral` 0.2.0 E1:3 · E2:9 · E3:8 · E5:4 · E6a:2 · E6b:2;
0.3.0 E1:3 · E3:2 · E4:1 · **E5:6**. `tooling.environmental` 0.2.0 E1:3 · E2:3 · E3:4 · E4:1 · E5:4 · E6a:2 ·
E6b:2; 0.3.0 E3:1 · E4:2 · **E5:6**. Code-graph degradation: E4 symbol-missing 3; **E5 symbol-missing 2 ·
unavailable 2 · refresh-stale 2**, with research `ok-degraded` ×2 (derived-without-graph).
**Level hypothesis:** silent costs the halt policy never surfaces. The graph line is new in kind. In this epoch the
graph was degraded by feature-gating on Windows, then absent altogether on Linux, so every research step since RR
has run without it.
**Proposal:** in-epoch directions are P4/L1/L2 (host-shell), P19 (gate-deferral), P9/L8 (environmental) and
L7/P14(b) (graph). This entry records persistence as the reason to fix at the tool rather than add another rule.

### L11 — removed-cause observation — advisory-db local clone residue (third epoch running)
**Facts:** `2026-10-01T23:22:23Z-a#1` SI fix-loop. An untracked `crates/matrix-sdk-crypto/RUSTSEC-0000-0000.md` was
found. This time it was left in place, and the audit was re-read against a fresh clone. Correlate:
`tooling.environmental` `2026-10-01T23:22:23Z-c`. Prior: Epoch 4 `2026-09-23T09:59:23Z-a#0` (moved aside); 0.2.0
Epoch 6b `2026-09-05T16:56:23Z-a`.
**Level hypothesis:** the cause (fetch-into-existing-clone leaving renamed advisories untracked) returns on its own
schedule. Each occurrence is handled by hand, a different way each time.
**Proposal:** the currency probe could self-heal (clean untracked advisory files, or audit against a fresh `--db`
clone by default), so the residue never reaches a gate.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — new-session/orientation — 3 cases → proposed type `input.ruling-not-in-ledger`
**Cluster:** `2026-10-02T14:38:25Z-b` (the handoff says the founder's ruling un-defers v3-09, while the matrix still
records `deferred` and coverage reads done-test YES) · `2026-10-02T22:26:05Z-c` (matrix v3-09 `deferred` while the
route says NOT deferred, so `matrix.py coverage` prints done-test YES on a version with an open capability) ·
`2026-10-02T22:26:05Z-b` (Rung 4 suggests the version close, which the founder's ruling places behind two BLOCKED-ON
entries). The correlate is consumed `matrix=thin` `2026-10-02T22:26:05Z-a`. **Draft criteria line:** *record
`input.ruling-not-in-ledger` when a mechanical verdict at orientation (matrix coverage, Rung selection) contradicts a
founder ruling that lives only in route, handoff or relay prose, naming the ruling's location and the verdict it
contradicts.* The cheaper fix may be a channel: a matrix verb that records a ruling's status change, so the ledger
and the ruling agree.

## Below threshold — no action
**Typed groups (per step, n < threshold)** — those marked → Pn are counted inside a by-type proposal above.
- phase/distill/contract.token-proxy-check 8 → P3 (it is above threshold by itself; reported under P3) · reconcile, fix-loop, validate, code, orientation token-proxy 1 each → P3
- implement/fix-loop/tooling.host-shell 3 → P4 · report 2 · distill 2 · research 1 · code 1 → P4
- phase/research/contract.narrow-basis-claim 3 → P7 · plan 2 · report 2 · route-resolve 1 · reconcile 1 · wrap — 1 → P7
- phase/research/contract.premise-falsified 3 → P8 · fix-loop 1 · take-up 1 · code 1 · phase — 1 → P8
- wrap/reconcile/contract.structural-blind-spot 3 → P14 · fix-loop 1 · implement — 1 → P14
- wrap/reconcile/recall.corpus-recurrence 2 → P2 · recall.curated-rule-not-applied 2 (UK curation + gates; host-win32 transport and CRLF rules) → P2 family
- wrap/route-resolve/contract.carry-no-owner 2 (SS stale ci.yml comment pinned to the nearest entry; SG a relay's stale `:65`)
- wrap/gates/tooling.light-gate-red 2 (SS a quoted credential-shaped literal tripped secret_scan_gate; CF the delta-guard allowlist cannot read scope-record.md)
- wrap/route-resolve/tooling.long-line-edit 2 (CARRY appends to multi-KB lines have no route.py verb; Epoch 4's L3 compaction gap has the same shape)
- phase/research/tooling.graph-symbol-missing 2 · tooling.graph-unavailable 2 · wrap/gates/tooling.graph-refresh-stale 2 → L7/L10
- implement/fix-loop/tooling.result-not-run-stable 1 (FG stamp writes 53/62 ms apart vs a 50 ms tolerance; the first number survived only in evidence)
- wrap/reconcile/ambiguity.escalation-rounds 1 (halted) · phase/validate/ambiguity.review-cycles 1 (a stray "0" not taken as approval)
- implement/fix-loop/contract.instrument-validity 1 (parse-nvda-log.ts assumed one clock; NVDA's stepped 2.5 s) · contract.spec-reality-gap 1 · contract.test-expectation 1 (`Spec Files:` + TAB)
- implement/code/contract.jointly-contradictory-instructions 1 (CF: S1-02 review row vs probe 22 vs the parser's grade, authored together, no check attempted)
- wrap/reconcile/input.report-insufficient 1 · contract.proposal-format 1 · contract.false-positive-proposal 1 · wrap/report/contract.detector-fact-gap 1
- phase/validate/input.out-of-pipeline-source 1 (halted) → P20 · phase — 1 → P20
- wrap/route-resolve/contract.grammar-irregularity 1 (UK `NVDA 2026.2` after ` · `; this wrap's own freight, reworded), plus untyped `2026-09-30T14:07:27Z-b` (`FIRST:`). These are the writer shape of Epoch 4's P3.
- implement/code/input.plan-step-ambiguous step-less 1 → P17 · phase/plan/input.research-thin 1 → X1 · phase/take-up/input.working-entry-thin 1 → X5 · phase/research/input.extract-signal-gap 1 · phase/distill/input.spec-source-gap 1
- wrap/curation/ambiguity.tier-routing 1 · wrap/gates/contract.coverage-hold 1 (v3-09 un-claim valve, plan-pre-stated)
- new-session/orientation: input.handoff-git-mismatch 1 · tooling.output-cap-overflow 1 (31.5 KB combined cat)
- universal by type below threshold: grammar-irregularity 1 · jointly-contradictory-instructions 1 · skill-reference-drift 1 (SS: codebase-research.md vs evolve/research.md on derived-without-graph vs graph-not-applicable) · output-cap-overflow 1

**Untyped clusters below F-4 (watch next epoch).**
- version-dir path assumed: `2026-09-30T16:30:33Z-b` (`.andromeda/working-route.md`), n=1. It recurs from Epoch
  4's U1 (3 cases), so it sits just under the recurring arm.
- design extract carried a wrong fact: `2026-09-30T16:39:59Z-b` (S3-06/S3-07 swapped) and `2026-09-30T16:45:55Z-b`
  ("Not yet run" inferred unshipped). n=2, both CF.
- plan entries designed red or unsafe by construction without saying so: `2026-09-30T12:45:01Z-c` (red under Branch
  B without an owner) and `2026-09-30T13:49:13Z-c` (the driver runs before its signature read). n=2.
- written forecast vs measured: `2026-09-30T12:27:10Z-e` (21 vs 24 rows). n=1.
- provenance/attribution: `2026-09-24T14:04:47Z-c` (the precedent's "made by the overseer" copied), and
  `2026-10-03T07:45:09Z-b` (a delegate's menu answer; → L9).
- singletons: `2026-09-24T10:59:44Z-b` (GITHUB_ENV resolution needs a hosted run) · `2026-09-24T14:37:38Z-d`
  (in-pass-correction era) · `2026-09-30T13:27:04Z-b` (the Write tool decoded a JSON `` escape) ·
  `2026-09-30T14:03:57Z-d` (an apply post-check asserted the old anchor absent) · `2026-10-01T00:05:33Z-b` (a
  hand-abbreviated sha256 tail) · `2026-10-03T22:51:37Z-c` (→ L6).
- fit existing types (see Mechanism health): `2026-09-30T20:57:42Z-c`, `2026-10-03T23:30:42Z-e`,
  `2026-10-04T00:47:47Z-b` (host-shell/cd) · `2026-09-30T14:07:27Z-b` (grammar-irregularity) ·
  `2026-10-02T14:54:25Z-b` (graph-symbol-missing) · `2026-09-30T16:20:57Z-e` (in-pass-correction; → L5).

**Problem-fact themes below threshold.**
- T9 launch handles not persisted (CONDUCTOR_NVDA located by host search): 2 (`2026-09-30T12:27:10Z-a#0`,
  `2026-09-30T13:27:04Z-a#0`). Epoch 4 had the model-env analogue as a friction only.
- Foreground/desk conditions beyond L8: covered there.
- Plan-letter vs code product-logic workarounds in implement (`2026-09-30T15:19:02Z-b#0`,
  `2026-09-30T18:21:17Z-a#1/#2/#3`, `2026-09-30T20:05:40Z-a#0`, `2026-10-01T21:54:30Z-a#1`,
  `2026-10-02T15:45:38Z-a#0`). These are product-logic, not a band-aid nature; they are the P17 family.
- Single-rule overrides: `2026-09-24T10:50:04Z-a#0` (a relay item added to scope) · `2026-09-30T10:56:19Z-a#1` (→
  L8) · `2026-09-30T12:06:36Z-a#0` (fork re-ruled after a premise correction) · `2026-09-30T13:20:54Z-a#0` (a
  conditional yes → P1) · `2026-09-30T14:03:57Z-a#0` (arch proposals rejected) · `2026-09-30T20:05:40Z-a#3` (clock
  calibration replaced) · `2026-09-30T20:40:13Z-a#0` (light-gate red ratified; Epoch 4 had
  `2026-09-29T19:43:16Z-a#0` on the same ASSERT arm, so n=1 in-epoch) · `2026-09-30T23:47:14Z-a#1` (a pre-re-fire
  replay condition) · `2026-10-02T12:56:06Z-a#0` (wrap P1 only, context 68%) · `2026-10-03T23:30:42Z-a#0`
  (Wayland relaunch).
- Removed-cause singletons: `2026-09-30T13:49:13Z-a#1/#2` (arm S starts; Edge background relaunch) ·
  `2026-09-30T15:19:02Z-b#1` (Shift release) · `2026-10-03T23:30:42Z-a#1` (preflight_spawn compile; → P14).
- **Deferred-forever, n=1:** friction `2026-09-30T14:05:45Z-b` (deferred 2). The two host-tool learnings deferred by
  the curation cap (Edge background relaunch; the Write tool's `\u` decode) have no later ledger record and no route
  CARRY.
- Deferrals checked and closed or routed: `2026-09-30T10:56:19Z-a#0` · `2026-09-30T20:40:13Z-a#1` ·
  `2026-10-01T20:24:02Z-a#0` · `2026-10-03T23:30:42Z-a#2`.
