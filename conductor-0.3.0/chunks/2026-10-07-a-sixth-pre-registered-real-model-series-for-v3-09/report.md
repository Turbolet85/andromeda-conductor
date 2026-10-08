# Report — 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09

**Chunk:** the sixth and last pre-registered three-drive real-model series for `v3-09`, against Pulse `9bfefb8`
(the corpus block narrowed to the triggering scope), its stop rule pre-registered before the launch
**Date:** 2026-10-08
**Commits:** `e3fa847` chore(2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09): operator pre-CI
commit, for the run this chunk's verdict reads (basis: `git log --format='%h %s' 902d12c..HEAD`, 1 commit; `902d12c`
is the parent of the oldest pre-CI commit and the chunk base)

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --numstat 902d12c -- crates contracts Cargo.lock Cargo.toml scripts scenarios .github`,
  6 rows; `gate.py scope`: changed 6 · listed 6)
  - `contracts/pulse-real-model-leg-posture.md` — 115 added, 0 deleted: the section `## The 2026-10-07 sixth
    series` at `761-875`, between `## The 2026-10-07 capture run` and `## The quiet window and serialization`.
  - `crates/conductor-run/tests/real_model_grading/series_2026_10_07_sixth.rs` — new file, 159 lines.
  - `crates/conductor-run/tests/real_model_series/mod.rs` — 25 added (`169-193`).
  - `crates/conductor-run/tests/real_model_grading/mod.rs` — 1 added (`19`).
  - `crates/conductor-run/tests/real_model_harvest.rs` — 15 added, 3 deleted (`68` · `71-73` · `1258` ·
    `1300-1303` · `1347-1352`); nothing between the rule markers moved (the rule diff against `902d12c` is empty).
  - `crates/conductor-run/tests/real_model_grading/capture_population.rs` — 1 added, 1 deleted (`9`).
  - The chunk folder: `evidence/attempt-ledger.md`, `evidence/rm-capture-d1.txt`, `-d2.txt`, `-d3.txt`, three
    round listings (`round-062859Z.txt`, `round-063605Z.txt`, `round-064534Z.txt`), `inputs/` (20 entries).
  - No file under `scripts/`, `scenarios/`, `.github/`, `crates/conductor-{verify,cli,emit,faults,timeline,core,
    report,tauri}/`, `crates/conductor-run/src/`, either manifest or the lockfile (the plan's frozen-path probe
    prints nothing against `902d12c`).
- **Symbols / APIs:** test-tree items only; no public function, IPC method, endpoint, export, port, socket or
  environment variable was added or changed.
  - `real_model_series/mod.rs`: `EVIDENCE_2026_10_07_SIXTH` (`170-173`) and `SERIES_2026_10_07_SIXTH`
    (`175-193`) — labels, file names and three sha256 digests.
  - `series_2026_10_07_sixth.rs`: `GRADING` (`9-13`), `capture_2026_10_07_sixth` (`15-18`),
    `SERIES_2026_10_07_SIXTH_RULE_SHA256` (`20-22`), `measured_2026_10_07_sixth` (`51-91`), and eight tests:
    `the_2026_10_07_sixth_series_rule_was_fixed_before_d1` (`24-31`),
    `each_2026_10_07_sixth_capture_matches_its_pinned_digest` (`33-36`),
    `each_2026_10_07_sixth_drive_recorded_the_current_rule_before_it_fired` (`38-41`),
    `the_2026_10_07_sixth_captures_carry_no_fingerprint_and_no_workspace_key` (`46-49`),
    `each_2026_10_07_sixth_drive_grades_as_the_ledger_records` (`93-97`),
    `the_2026_10_07_sixth_envelopes_carry_the_eleven_keys` (`99-116`),
    `v3_09_is_met_by_the_2026_10_07_sixth_series` (`118-141`) and
    `v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth` (`143-159`).
  - Callers kept: `v3_09_met` and `pre_registered` (`real_model_grading/mod.rs`) each gain a sixth caller; neither
    signature changed. The ref test is the first of its shape in the tree (basis: research.md §Graph impact, 0 hits
    for a ref test at `902d12c`).
- **Crates / modules:** no crate added, removed or re-edged. One test child module added,
  `real_model_grading/series_2026_10_07_sixth.rs`, registered at four sites: `real_model_grading/mod.rs:19`, and in
  `real_model_harvest.rs` the `use` line (`68`), the constants list (`71-73`), the `GRADING_MODULES` length
  (`1258`) and row (`1300-1303`), and the `graded_captures` loop (`1347-1352`).
- **Dependencies:** none added, none bumped. `grep -c "^name = " Cargo.lock` reads 562 before and after; no
  manifest changed.
- **Schema / config:** none. No scenario, config key, violation schema or scrub/redaction shape changed. The
  posture contract (`contracts/pulse-real-model-leg-posture.md`) gained one add-only dated section; it still names
  6 distinct environment handles (`grep -oE '(ANDROMEDA|CONDUCTOR)_[A-Z0-9_]+' … | sort -u | wc -l`: 6 at the
  chunk base, 6 now) and has no Rust reader other than the harvest's section digest.
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved:**
  - The posture contract's latest per-series Pulse pin: `f70be92` → `9bfefb8` (the new section, `761-875`). The
    binaries were built at Pulse `f18c631` and launched with Pulse at `18a872d`; the build inputs (`crates`,
    `pulse-app`, `xtask`, `Cargo.toml`, `Cargo.lock`) are equal to `9bfefb8`'s at both (`git diff --name-only
    9bfefb8 HEAD -- …` printed nothing at each read). Docs stating the latest pin: `architecture.md:184`
    (`grep -c f70be92`: architecture 1 · security-plan 1 · test-plan 1 · obs-plan 1 · the other three masters 0 ·
    `.andromeda/registries/` 0). The security-plan, test-plan and obs-plan hits are dated records of the
    2026-10-07 series and stay as records; only `architecture.md:184` states the LATEST pin.
  - `v3-09`'s standing: not met → **met on three drives** (the verdict below). Docs stating it: `architecture.md:70`
    ("the interpretation claim stays unverified (`v3-09` not met)"), `test-plan.md:260` (the dated series records,
    the last reading "`v3-09` stays not met on the 2026-10-07 series' record"); `obs-plan.md:221` names no verdict
    (`grep -c 'v3-09'`: architecture 1 · test-plan 1 · obs-plan 1 · security-plan 0 · the other three masters 0 ·
    registries 0; the obs-plan hit, read, is the fifth series' chunk marker inside `:221` and states no standing).
  - `COMMITTED_CAPTURES` 23 → 26 (`capture_population.rs:9`); `GRADING_MODULES` 13 → 14
    (`real_model_harvest.rs:1258`); the harvest target 125 → 133 tests; the workspace nextest total reads 1233 on
    this tree. Docs stating any of these as a literal: none — verified (`grep -rn -oE "GRADING_MODULES|1219
    tests|1225|125 tests|132 tests|133 tests|23 (committed )?captures|26 captures"` over the seven masters,
    `.andromeda/registries/`, `CLAUDE.md`, `.claude/rules/`, `.claude/docs/`: 0 hits; security-plan and test-plan
    name the population "pinned by count", never the count).
  - The number of series on the Linux dev host: two → three (the contract section says so). No master states a
    series count (test-plan.md:260: "The series set is the posture contract's dated series sections, never a count
    restated here").
- **Dev-tool versions:** none changed. Re-read on the dev host at 2026-10-08T06:27:55Z: the NVIDIA kernel-module
  major and the userspace major both 610 (the driver; unchanged from the phase's reading). `cargo audit` loaded
  1294 advisories (the advisory database, an external artifact, not a tool version; 1290 at the fifth series'
  wrap).
- **Harness / gate surface:** none. No `agent-run` verb, flag, selector or arm, no CI step, no status or verdict
  shape was added or changed; every drive ran through the existing `bash scripts/agent-run.sh run --live
  real-model`.
- **Cross-project / external claims:**
  - **Pulse (`andromeda-pulse`, a sibling repository, read at committed states and through its own log):** the
    series is pinned to `9bfefb8` (`9bfefb812297bbdea610423a21568b3262bdb7ed`). Both binaries were built on
    2026-10-07 at Pulse HEAD `f18c631`; at the launch Pulse's HEAD was `18a872d0e4496b292f8bc10472da835bec0406c4`
    (a setup commit of 2026-10-08T07:09:46+02:00, 13 paths against `f18c631`: 9 under `.andromeda`, 3 under
    `.claude`, the root `CLAUDE.md`), build-input diff against `9bfefb8` empty, build-input porcelain empty, so
    nothing was rebuilt. `pulse-app` sha256 `44f28608c4e9c33685d8bf1329a47aab1641d017d0516b7e7d2669e72f43428d`
    (differs from the recorded `f70be92` build's `df167647…ba4a`); `andromeda-pulse-mcp` sha256
    `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9` (unmoved by its rebuild). The model file's
    sha256 read `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` before d1, equal to the pin.
    Pulse's own log for the sitting (`agent-latest.jsonl.2026-10-08`, 25988 lines): `inference_mode` `real`,
    `model_identity` `gemma-4-E4B-it-Q4_K_M`, `workspace_root_basename` `rm-sixth-series`, 0 bootstrap-window
    override lines, `prompt_version` `v2.6` on all 36 prompt assemblies of the three leg windows, 36 parses all
    `ok`, no inference error or skip line, no ERROR line.
  - **CI:** `CI#37741509455`, `completed/success`, checks 3/3, on `e3fa847239e075a8d9627fa9496b51c6840b62ae`
    (the pre-CI commit; `ci.py conclusion --sha HEAD --wait 1800` read at 07:19:14Z, recorded in the ledger's
    operator-pass section). This wrap's commit adds to that tree.
  - **The local advisory-db copy (outside this repository):** the agent removed one untracked file from it,
    `crates/wasapi/RUSTSEC-0000-0000.md`, on 2026-10-08. Reason: `cargo audit`'s own fetch had moved the copy and
    left that pre-id-assignment file beside its tracked renamed twin (equal apart from the id), so the first audit
    reading of the pass ran over an unclean copy; the plan's scope item 11 sanctions clearing such residue. The
    reading that stands, taken over a copy whose porcelain read 0 lines before and after with `HEAD` equal to
    `FETCH_HEAD`: `cargo audit` exit 0, 1294 advisories, 562 crate dependencies, 7 allowed warnings (6
    `unmaintained`, 1 `unsound`); `cargo deny check advisories bans licenses sources` ok.
  - **Inputs (`inputs.py verify`, 20 entries — unchanged 12 · drifted 2 · n/a 6 · vanished 0 · broken 0 · altered
    0 · unreachable 0 · uncited 4 before this report · unparsed 0):**
    - I1 · `../additional/pc-overseer/relays/conductor-phase-v309sixth-2026-10-07.md` · copy no-repo · unchanged
    - I2 · message (the operator, the phase's arguments) · copy · n/a
    - I3 · `../andromeda-pulse:.andromeda/master-route.md` · pointer committed@`9bfefb81` · **DRIFT — committed
      since (HEAD `18a872d0`)**: Pulse's wrap commit `f18c631` flipped the remedy chunk's record at `:103` from
      `pending` to `complete` (1 line changed). The phase's reading "pending at `:103`" (inputs#I3) is superseded
      by inputs#I15, the same file at `f18c631`, which reads `complete` and is unchanged.
    - I4 · `../andromeda-pulse:andromeda-pulse-0.3.0/working-route.md` · pointer committed@`9bfefb81` · **DRIFT —
      committed since (HEAD `18a872d0`)**: the same Pulse commit changed 3 lines of that route (the remedy
      chunk's own entry). The entry still names the chunk (3 hits for its marker at HEAD).
    - I5 `retrieval.rs` · I6 `assembler.rs` · I8 `remedy-shipped.md` · I9 `reading-replay.md` · I10
      `product-path-equality.md` · I11 Pulse's root `Cargo.toml` · I12 the sidecar's `Cargo.toml` · I13
      `schema.rs` — each `../andromeda-pulse` · pointer committed@`9bfefb81` · unchanged
    - I7 · `../additional/pc-overseer/l4-env.sh` · copy no-repo · unchanged
    - I14 · I16 · I17 — messages (the operator's P4 answers, P5 review answer, the Part A word) · copy · n/a
    - I15 · `../andromeda-pulse:.andromeda/master-route.md` · pointer committed@`f18c631b` · unchanged
    - I18 · `../additional/pc-overseer/relays/conductor-implement-v309sixth-go-2026-10-08.md` · copy no-repo ·
      unchanged (the go for Part B: inputs#I18)
    - I19 · I20 — messages (the re-entry word, inputs#I19; the operator-pass word, inputs#I20) · copy · n/a
    - Uncited at the read: I17, I18, I19, I20 — the four implement snapped; this report cites them (inputs#I17 in
      Decisions, the other three here).
    - No acceptance criterion rests on a drifted input: the gate's clearing event was the relayed sha, and Pulse's
      record now reads `complete`, the stronger state.
  - **Pulse's replay readings, as this chunk's records cite them** (not re-measured here): on the second captured
    prompt, 11 of 20 is the remedy reading's right-service count under the shipped prompt (inputs#I8), and 9 of 20
    is the replay reading's (11 misses, inputs#I9).
- **Reverted / negative API facts:** none shipped and reverted. Inside the red-before-green read, a verdict test
  named for the not-met arm stood for one pass asserting an empty graded set; it was replaced by the met arm before
  any commit.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. `obs-plan.md:221`: "the canary fires three storms per drive and the capture prints two `canary:` lines, the
     third storm landing at the scenario's emission instant so that its digest's tick falls after it". Measured
     false as a general statement: the captures of d1 and d2 print THREE `canary:` lines, the third reading
     `pipeline-fault` with `parse=none`; d3 prints two. The third storm's digest tick fell 4 ms (d1) and 2 ms (d2)
     BEFORE the emission instant and 11 ms after it in d3. The capture pairs canary digests over Pulse lines
     stamped strictly before the emission instant (`real_model_live.rs`, the canary block, read at `720-730`;
     `real_model_common/mod.rs`, `canary_attempts`, read at `73-151`), so in d1 and d2 it saw a retry-storm tick
     whose prompt assembly (1 ms and 2 ms after the instant) lay outside its window. Pulse's own log shows that
     storm prompted, parsed `ok` and deduped in all three drives. Evidence: `evidence/attempt-ledger.md`, the
     section "The third `canary:` line of d1 and d2"; all nine earlier captures on this host print two lines and
     no `pipeline-fault` token (`grep -cE '^canary: (surfaced|dismissed|pipeline-fault)'` per capture). The same
     sentence stands in the plan's implementation notes; no other master, registry file or leaf states it
     (`grep "two .canary:. lines"` over the masters, `CLAUDE.md`, `.claude/rules`, `.claude/docs`: 1 hit).
  2. `test-plan.md:391`: "real-model formation AND pickup both still UNMEASURED", and `architecture.md:184`: "no
     real-model formation or pickup figure exists". This series measured a pickup on each drive (5813, 12290 and
     6048 ms, the capture's `pickup:` line) and Pulse's own stamps from the storm's Autonomous line to the
     incident's creation (ledger rows). The same sentence of `test-plan.md:391` already says the pickup is
     "recorded as a measurement and never budgeted". The clauses predate this chunk: the three earlier records on
     this host printed pickups beside them and no wrap amended them. Disposition is Validate's; nothing here rests
     on it.
- **Expected amendments (from plan):**
  - architecture §Occupied Resources → On-disk artifacts (the posture-contract entry): the latest per-series pin
    `9bfefb8` replaces `f70be92`, the series' date to the sidecar — **carried** (Counts / qualifiers moved, first
    bullet). Sites: `grep -n f70be92 .andromeda/architecture.md` → 1 hit, `:184`; registries 0 hits. The section
    has 1 byte of headroom (38114 B of 38115 B, `scripts/arch-registry-check.py measure`, read at this wrap), and
    the two shas are the same length.
  - architecture §Established Decisions [Read-Back Dependency Posture]: the one `v3-09` verdict statement — MET
    arm — **carried** (Counts / qualifiers moved, second bullet). Sites: `grep -n 'v3-09' .andromeda/architecture.md`
    → 1 hit, `:70`; registries 0 hits. 47 bytes of headroom (38068 B of 38115 B).
  - security-plan §Input Validation (the real-model capture ingest row) and §Security Anti-Patterns → Data
    Protection: the sixth series' dated record — **carried**. Facts: Pulse's workspace-key derivation is unchanged
    at `9bfefb8` (the product diff `f70be92`..`9bfefb8` names two files under `crates/triage/src/digest` only;
    `crates/workspace-detector` is byte-unchanged, the contract section's own clause); the launch's detection fell
    back to the data dir (Pulse's boot line read the leaf as the workspace basename); the leaf `rm-sixth-series`
    occurs 0 times across the three captures (the plan's key probe: 0 at exit 1; the harvest's key arm green);
    each `## Previously Seen` suffix prints `<redacted>` (d2: 1 entry, d3: 2 entries, d1: none); the rendering
    witness reads `verbatim` on all three; d1, d2 and d3 each carry one report body. Sites: `grep -n -E '2026-10-07
    series|capture run' .andromeda/security-plan.md` → `:121` (the ingest row) and `:338` (Data Protection, the
    per-series inventory); registries 0 hits.
  - test-plan §6 Real-model interpretation leg: the sixth series' dated verdict record, after the fifth series'
    and the capture run's and before the closing sentence — **carried** (the verdict in Outcome). Site:
    `test-plan.md:260` (`grep -n f70be92 .andromeda/test-plan.md` → 1 hit); registries 0 hits for `f70be92` and
    for `v3-09` (the two `unmet` hits in `contracts/test-plan/5-command-implementation.md`, read, are "an unmet
    subject" of the preconditions probe and are not this entry's).
  - obs-plan §4 Real-model posture: a dated live observation at `9bfefb8` — **carried**. Facts: Pulse's own log
    read `model_identity` `gemma-4-E4B-it-Q4_K_M` and `prompt_version` `v2.6`; all three drives' envelopes landed
    `ManualCheck` with `verdict` null and the eleven keys; the chunk added no span name, span attribute, log line
    or allowlist entry. Site: `obs-plan.md:221` (`grep -n f70be92 .andromeda/obs-plan.md` → 1 hit); registries 0
    hits. The same line carries disproved claim 1 above.
- **Coverage of new surfaces:**
  - none — the chunk adds no external surface, hot-path operation or UI element. Its new code is test-tree
    grading over committed files: validation n/a · instrumentation n/a · PII: the three captures entered
    `evidence/` through the existing scrub chain (redacted✓: 0 un-elided `fingerprint_hex=` values, a fixed point
    of the elision, 0 occurrences of the workspace key, 0 host-path hits) · tests: unit (the harvest target, 8 new
    tests) · a11y n/a · tokens n/a.

## Deviations from intent
- **The plan's note said the capture prints two `canary:` lines for three storms; d1 and d2 printed three, the
  third `pipeline-fault`.** Nothing was re-fired: the pre-registered re-fire clause reads the canary from Pulse's
  own log, which shows a parse `ok` and a dedupe for that storm and no inference error or skip. The operator
  verified that reading himself before the operator pass (inputs#I20). The capture's pairing window was not
  touched: it is harness code outside the chunk's lists, and a change between drives would have broken the
  identical-drives term.
- **The first `cargo audit` reading was discarded** and entries for the advisory-db porcelain, `cargo audit` and
  `cargo deny` were fired again after the one residue file was removed (Cross-project / external claims).
- **`gate.py hygiene` was fired once as a read-only pre-check before the report of implement**, not as the plan
  entry's firing; the entry itself was fired in the operator pass.
- **The liveness check after the launch was read at 8 s and again at 14 s** of the process's own elapsed time, not
  once at 10 s; both read alive with `:4317` accepting.
- **Pulse's HEAD moved between Part A and Part B** (`f18c631` → `18a872d`, bookkeeping only); the binaries
  launched are Part A's, re-read by digest.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 6 · listed 6 · excluded 72; base `902d12c8`).

## Decisions & corrections
- **The founder's go** for the GPU sitting: his own pick by dialog on 2026-10-08 at 07:05 local, relayed by the pc
  overseer (inputs#I18) and stated in the operator's re-entry arguments (inputs#I19).
- **The operator pass** was performed by the agent on the operator's explicit word (inputs#I20); the commit and
  the push are the operator's acts.
- **The operator's ruling on the route-line quote** (inputs#I17, 2026-10-07): the one-sentence model-text quote on
  this chunk's frozen route line is outside the scoped exception's letter; at this wrap it is replaced by a pointer
  to the evidence file, and nothing is widened. Found while locating it (`grep -rl -F` on a fragment of the quote,
  outside `target/`): the same sentence also stands in the capture-run chunk's `report.md`, a frozen chunk folder
  this chunk may not edit; it is surfaced to the operator, not acted on.
- **This wrap's directives** (the wrap's arguments and the overseer's relay, kept in the run dir as `relay-1.md`):
  mint no entry; the pairing-window finding is a `CARRY` on the version-close entry, not a residual for 0.4.0; the
  pass is not attributed to the remedy; model text stays in `evidence/`.
- **A printed capture token is a claim about a window, not about Pulse.** The capture's `canary:` classifier reads
  only lines before the emission instant, so a storm whose tick lands a few milliseconds before that instant
  prints `pipeline-fault` whatever Pulse did with it. The re-fire decision was taken from Pulse's log, as the
  contract's clause says.
- **A porcelain entry read just before `cargo audit` does not cover the audit's own fetch.** The copy read clean,
  the audit fetched, and the fetch left the residue the scan then ran over. The porcelain is read again AFTER the
  audit before its figures are trusted.
- **Sweep hazards found:**
  - `grep -cE 'v3-09'` over `obs-plan.md` returns a hit that states no standing (a chunk marker that carries the
    token); the standing lives in architecture and test-plan only. `grep -E 'not met|unmet'` over the registries
    returns the preconditions probe's "unmet subject", not a capability standing.
  - A session probe that split a capture on its first `## Hypotheses` read the rule record's own text (the rule
    quotes that header) and reported 0 ranked entries; the capture block starts after `// ---- rule: end ----`.
  - `sed` on a `## Previously Seen` entry expecting a parenthesised workspace suffix matched nothing, because the
    suffix prints bare `<redacted>`; the fall-through printed the entry's title, which is model text. It reached a
    terminal only, no file.

## Outcome
**The verdict.** By the pre-registered rule, applied mechanically by the harvest over the three pinned captures:
graded drives 3 — d1 `Identified`, d2 `Identified`, d3 `Identified`; the real-model witnesses and the launch-cwd
witness hold on each. **`v3-09` is MET on three drives**, and is read as no more than that
(`v3_09_is_met_by_the_2026_10_07_sixth_series`; the ref test
`v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth` exists). The stop rule's consequence, in the
section's words: "`v3-09` is verified on three drives"; "Either way the next route entry is the version close."

**What the verdict does not say.** The pass is not attributed to the remedy. No prompt was recorded in this
series; the capture's corpus-row line counts candidates before the selection (1, 3 and 6, as in every earlier run
of this design); that the narrowed selection is in the launched binary rests on build provenance and the moved
digest alone. The 2026-10-07 capture run, on the `f70be92` build without the remedy, also read three `Identified`
as observations. Across the four live runs on this host the third position reads 2 misses of 4: a rate, not a
cause.

**Beside the verdict.** Each rank-1 statement names `conductor` as a whole word, carries a retry token and carries
the cue line's `scope_id` value; no ranked hypothesis of any drive (8 in all) and no justification line names the
canary identity, so no grade rests on the rule's known weakness. Further grades: d1 `Pass` / `Pass` / `Blocked`
(no prior same-scope incident to retrieve); d2 and d3 `Pass` / `Pass` / `Pass`. Key rendering `verbatim` on all
three.

**Acceptance criteria, each against the diff and the evidence:**
- (capability) `v3-09` — MET as above; the series ran as its section fixes it (three drives, one fresh dir, one
  sitting 06:28:59Z-06:54:37Z in daytime, the plain leg env, no re-fire, no fourth drive). The ref test exists
  because the condition holds.
- (arch) The section is add-only: `git diff --numstat 902d12c -- contracts/pulse-real-model-leg-posture.md` reads
  115 added, 0 deleted; its digest `01cf94c55845834391edd14c041152ad1c3fad6d4b2b7377221d5d8488f44fb5` is the
  ledger's `pre-registration sha256: ` line, recorded 2026-10-07T21:08:29Z before any launch, and
  `the_2026_10_07_sixth_series_rule_was_fixed_before_d1` holds the three equal — met.
- (process) The stop rule stands inside that digested section; the ledger's verdict states its consequence in the
  section's words — met.
- (arch) The handle census reads 6 before and after; no `agent-run` verb, flag or arm was minted — met.
- (arch, layouts, obs) Every drive ran through `bash scripts/agent-run.sh run --live real-model`; the frozen-path
  diff against `902d12c` is empty — met.
- (arch) The grading lives in `conductor-run`'s test tree; the package count reads 562; no manifest changed — met.
- (arch, security, layouts) Before each drive `conductor preconditions --for real-model-interpretation` exited 0
  with its atom, the sidecar resolved through the firing form's `PATH` (three round listings) — met.
- (security) Both binaries built from clean inputs equal to `9bfefb8`'s and proven before d1 (`pulse-app` by
  provenance and a moved digest, the sidecar by provenance, both digests recorded); the section says no named
  token exists; the model digest equalled the pin before d1 — met.
- (security, tests) Each committed capture matches its pin, holds zero un-elided keyed values, is a fixed point of
  the elision and holds zero occurrences of the leaf; `COMMITTED_CAPTURES` is 23 plus the three committed here; no
  capture text sits in test source (the harvest's source arm green) — met.
- (security, obs) The host-path probe over `evidence/` reads 0 with the home-rooted and temp-rooted shapes; the
  secret-scan gate is green inside the bundled default — met.
- (tests) The rule diff is empty; each capture opens with the rule as recorded before its leg
  (`each_2026_10_07_sixth_drive_recorded_the_current_rule_before_it_fired`) — met.
- (tests) The harvest and full non-doc `cargo test -p conductor-run` are green with no retries;
  `capture_paths_guard` green — met.
- (tests, obs) `bash scripts/agent-run.sh run` exits 0; `cargo fmt --all --check` clean — met.
- (obs) Each drive's envelope carries the eleven keys, `verdict` null, `state` `ManualCheck`
  (`the_2026_10_07_sixth_envelopes_carry_the_eleven_keys`); the verdict test keys on no envelope field — met.
- (security) The advisory-db porcelain is empty, then `cargo audit` and `cargo deny` exit 0 — met on the re-read
  (the first audit reading discarded, above).
- (process) The post-series census reads no matching process and no listener; the ledger's table names each
  process with its final state; the seven frozen real-model chunks are byte-unchanged — met.
- (ci) The operator pass's CI read printed `verdict: green` for the pushed HEAD: `CI#37741509455` — met.

**Gates** (the plan's entries by `run`, in order; implement's last whole pass at 07:03:57Z unless said otherwise):
- The five Pulse git probes (`git -C … merge-base --is-ancestor 9bfefb8 '@{u}'`, `rev-list --count '@{u}..HEAD'`,
  `status --porcelain -- crates …`, `diff --name-only 9bfefb8 HEAD -- crates …`, `rev-parse HEAD`) — `leg =
  'operator'`, driven by hand on 2026-10-07 (Part A): exit 0, 0, no output, no output, `f18c631…`; recorded in the
  ledger. The porcelain and the diff were read again by hand before the launch (no output each).
- The two report-only binary probes (`sha256sum pulse-app andromeda-pulse-mcp && ls -l … && for b in …`) and the
  two builds (`cargo build --release -p pulse-app`; `… -p mcp-server --bin andromeda-pulse-mcp --features
  mcp-server`) — `leg = 'operator'`, Part A: recorded tables; both builds exit 0.
- The asserting binary probe (`… test "$d" != df1676… && … echo "digest moved, both built after the commit"`) —
  `leg = 'operator'`, Part A: exit 0, its atom held.
- `ls -1 "$HOME/.cache/pulse-legs" …; test ! -e … && echo "leaf absent"` — `leg = 'operator'`, Part A: exit 0,
  atom held.
- `test -f "$ANDROMEDA_PULSE_MODEL_PATH" && test -x … && echo "model env present"` · `sha256sum
  "$ANDROMEDA_PULSE_MODEL_PATH" | cut -d" " -f1` · the NVIDIA major probe — `leg = 'operator'`, by hand
  06:27:55Z: exit 0 each; atoms held (`model env present`; the pinned digest; `kernel=610 userspace=610`).
- The round, three firings through `gate.py run --live-legs --entry N`, each `round: COMPLETE · legs fired 1/1`
  (listings in `evidence/`): the census (`ps -eo … | grep -E 'pulse-app|…'; ss -ltn | grep -cE ':4317|:4318'`)
  green ×3; `sleep 180` green ×2; `PATH=… cargo run -q -p conductor-cli --bin conductor -- preconditions --for
  real-model-interpretation` green ×3, its atom held; the drive (`PATH=… bash scripts/agent-run.sh run --live
  real-model && cp runs/live-suite/rm-capture.txt …/rm-capture-d{1,2,3}.txt`) green ×3 — exit 0, both `[live]`
  atoms, the artifact fresh, 362 s each, no survivor, no history move.
- The post-series census — `leg = 'operator'`, by hand 06:55:10Z: exit 1, last line `0`; atoms held.
- `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green (133 of 133).
- `cargo test -p conductor-run` — green (390 passed over 27 result lines).
- `cargo nextest run -p conductor-run --test capture_paths_guard --profile ci` — green (8 of 8).
- `bash scripts/agent-run.sh run` — green (workspace nextest 1233 of 1233, doctests, the lint lines; exit 0).
- `cargo fmt --all --check` — green.
- `grep -c "^name = " Cargo.lock` — green (`last line 562`).
- The rule diff (`diff <(git show 902d12c:… | sed -n '/rule: begin/,/rule: end/p') <(sed -n … )`) — green (no
  output).
- `git diff --numstat 902d12c -- contracts/pulse-real-model-leg-posture.md | awk '{d+=$2} END {print d+0}'` —
  green (`last line 0`).
- `grep -oE '(ANDROMEDA|CONDUCTOR)_[A-Z0-9_]+' contracts/pulse-real-model-leg-posture.md | sort -u | wc -l` —
  green (`last line 6`).
- `git diff --numstat 902d12c -- crates/conductor-verify/ …` (the frozen paths) — green (no output).
- `git diff --numstat 902d12c -- conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/ …` (the seven
  frozen chunks) — green (no output).
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` — green (no output).
- `cargo audit` — green (exit 0; 1294 advisories, 562 dependencies, 7 allowed warnings), on the re-read.
- `cargo deny check advisories bans licenses sources` — green.
- The host-path probe over `evidence/*` — green (exit 1, `last line 0`); the key probe over the captures — green
  (exit 1, `last line 0`). Both re-read at 07:19Z after the ledger's operator-pass section: green.
- `bash scripts/agent-run.sh status <id>` — `leg = 'operator'`, smoke, by hand 06:59:44Z with d3's run id
  `2026-10-08T06-48-35-676`: exit 0, atom held; mint-then-read.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, the operator
  pass 07:07:00Z: exit 0, `hygiene: clean`.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=…"` — `leg =
  'operator'`, the operator pass 07:07:14Z, after the operator pre-CI commit `e3fa847` (77 files): exit 0,
  `PUSHED_SHA=e3fa847239e075a8d9627fa9496b51c6840b62ae`, `902d12c..e3fa847` on `build/conductor-0.3.0`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg =
  'operator'`, the operator pass, ended 07:19:14Z: exit 0, `verdict: green · checks 3/3 · wall 703 s`,
  `CI#37741509455 completed/success` on `e3fa847`.
- Smoke: fired (the `status` entry above). No deferral was invoked; no entry carries `defer`.

**Watches:** none folded.

**Outcome basis.** The operator pass ran: the gate verdicts rest on its final state — the one pass commit
`e3fa847` (Setup's list) and the CI run `CI#37741509455` on it, recorded in `evidence/attempt-ledger.md`'s
operator-pass section. Implement's reports (Part A on 2026-10-07; Parts B and C in this window, the conversation
present) are the basis for what only they hold. Operator directives between implement and this report: the
operator-pass word (inputs#I20) and this wrap's arguments with the overseer's relay. Post-implement artifacts: the
ledger's operator-pass section, the handoff's in-flight note, one gate-trail record (a re-read of the harvest and
the two evidence probes).

**Process hygiene** (implement's census, re-measured at this wrap, 07:25:31Z: 0 matching processes, 0 listeners on
`:4317` or `:4318`):

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` 3816613 and its five WebKit children | implement, on the founder's go | terminated (SIGTERM 06:55:02Z, gone within 2 s) |
| `conductor` and `andromeda-pulse-mcp`, one pair per leg and per probe | the legs | exited with each leg; the round tool reported no survivor on any firing |
| `llama-cli`, one per inference | `pulse-app` | each exited with its inference; absent from the post-census |
| the round tool's firings, the gate runs, `cargo`, the CI poll | implement and the operator pass | exited; absent from the census |
| the two Pulse release builds' `cargo` and `rustc` (2026-10-07) | implement Part A, on the operator's word | exited with the builds (exit 0 each) |
| `code-graph.py refresh` | this wrap | exited (`.refresh-done` present) |
## New text, by line
Generated by `cites.py added` (cites v1.2); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 902d12c8 (the parent of the oldest pre-CI commit e3fa8472) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### contracts/pulse-real-model-leg-posture.md — added 115 line(s) in 1 range(s)
added: 761-875
- 783-787 «- **What changed since `f70be92`.** `select_corpus_matches` (`crates/triage/src/digest/retrieval.rs`) takes…»
- 788-789 «- **For the scenario's storm** the cue carries `scope_id=conductor`, so a `conductor-canary` incident reach…»
- 790-791 «- **The trigger line, the trigger-framing instruction, the cue line and the grounded title** are as §The»
- 792-794 «- **Byte-unchanged from `f70be92`:** everything under `crates/interpretation`, `pulse-app/src`,»
- 827-831 «- `pulse-app` by four facts. Its build inputs were clean and equal to `9bfefb8`'s. Its build command exited…»
- 832-834 «- the sidecar by build provenance: the same clean inputs, its build command at exit 0, and an mtime after»
### crates/conductor-run/tests/real_model_grading/capture_population.rs — added 1 line(s) in 1 range(s)
added: 9
### crates/conductor-run/tests/real_model_grading/mod.rs — added 1 line(s) in 1 range(s)
added: 19
### crates/conductor-run/tests/real_model_grading/series_2026_10_07_sixth.rs — new file · 159 line(s)
- 9-13 «const GRADING: Series = Series {»
- 15-18 @16 «pub(crate) fn capture_2026_10_07_sixth(drive: &Drive) -> String {»
- 20-22 @21 «const SERIES_2026_10_07_SIXTH_RULE_SHA256: &str =»
- 24-31 @25 «fn the_2026_10_07_sixth_series_rule_was_fixed_before_d1() {»
  - 26-30 «pre_registered(»
- 33-36 @34 «fn each_2026_10_07_sixth_capture_matches_its_pinned_digest() {»
- 38-41 @39 «fn each_2026_10_07_sixth_drive_recorded_the_current_rule_before_it_fired() {»
- 46-49 @47 «fn the_2026_10_07_sixth_captures_carry_no_fingerprint_and_no_workspace_key() {»
- 51-91 @53 «fn measured_2026_10_07_sixth(label: &str) -> (Route, Grade, [Outcome; 3], CanaryAttempts) {»
  - 54-58 «let surfaced_twice = CanaryAttempts {»
  - 59-65 @61 «let surfaced_twice_and_a_third_token = CanaryAttempts {»
  - 66-90 «match label {»
- 93-97 @94 «fn each_2026_10_07_sixth_drive_grades_as_the_ledger_records() {»
- 99-116 @100 «fn the_2026_10_07_sixth_envelopes_carry_the_eleven_keys() {»
  - 101-115 «for drive in &SERIES_2026_10_07_SIXTH {»
- 118-141 @119 «fn v3_09_is_met_by_the_2026_10_07_sixth_series() {»
  - 125-132 «assert_eq!(»
  - 133-136 «assert_eq!(»
  - 137-140 «assert!(»
- 143-159 @146 «fn v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth() {»
  - 149-158 «for (label, _) in &graded {»
### crates/conductor-run/tests/real_model_harvest.rs — added 15 line(s) in 5 range(s)
added: 68 · 71-73 · 1258 · 1300-1303 · 1347-1352
  - 1300-1303 «(»
  - 1347-1352 «for drive in &SERIES_2026_10_07_SIXTH {»
### crates/conductor-run/tests/real_model_series/mod.rs — added 25 line(s) in 1 range(s)
added: 169-193
- 170-173 @172 «pub const EVIDENCE_2026_10_07_SIXTH: &str =»
- 175-193 @177 «pub const SERIES_2026_10_07_SIXTH: [Drive; 3] = [»
  - 178-182 «Drive {»
  - 183-187 «Drive {»
  - 188-192 «Drive {»
