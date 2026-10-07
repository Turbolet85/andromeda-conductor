# Scope — 2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive

**Working entry (`working-route.md:96`):** A capture run records the prompt the model received in each drive — the
fifth series' three-drive design, read by Pulse's builder; not a `v3-09` verdict.

**Matrix target:** none. `v3-09` (*Real-model interpretation leg*) stays `planned` and un-claimed: this run is a
capture, its grades are observations, and the verdict stays the sixth series (the entry's CONTEXT; inputs#I3 §3;
inputs#I1 §2). The chunk links no capability.

**Chunk base:** `6987764` (`69877645c135265378358bcbb5a84de2111c2199`), HEAD at take-up. Every diff-shaped gate probe
names it explicitly, because the operator pre-CI commit moves HEAD before the wrap.

**Host:** the Linux dev host (Omarchy), as the fourth and fifth series. Host-shaped terms are read from the fifth
series' contract section, plan and ledger, re-verified at P3.

## The vehicle (the entry's first CARRY, resolved)

- The CARRY held the vehicle open between (A) and (B) and barred a launch, and any plan fixing the vehicle, until
  the founder's own word. That word arrived: **the founder, 2026-10-07 13:49 local, by dialog, his own pick — the
  option "the operator's"** (inputs#I1 §1, relayed by the pc overseer; inputs#I2). The question as put to him, in the
  relay's summary: the real prompt must be written once in the clear on this host; whose wrapper around the model
  does it — the operator's, a Conductor feature with two clauses amended on his word, or no recording.
- So vehicle **(A)** binds, and nothing of Conductor's boundary moves:
  - `.andromeda/security-plan.md:338` stands unamended (re-read at take-up: corpus content is never persisted into a
    Conductor artifact, save the one scoped, scrubbed MCP re-read exception). The recorded prompts never enter a
    Conductor artifact.
  - `.andromeda/architecture.md:204` stands unamended (re-read at take-up: "Conductor neither SETS nor READS
    either" of `ANDROMEDA_PULSE_MODEL_PATH` · `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`). No Conductor file names the
    pass-through as a program, sets the handle or reads its value. Measured at P3: `grep -rn -E
    'LLAMA_CUDA_BIN_PATH|LLAMA_CPU_BIN_PATH|PULSE_MODEL_PATH|PC_L4_' crates scripts .github` prints nothing.
  - No boundary widening is asked of this chunk, so none needs ratifying on Conductor's side. A leg that set the
    handle, or read what the pass-through recorded, would be the widening the founder declined.
- The operator's vehicle, read from the files themselves (never from a capture). The three files stay in the
  overseer's folder and are not snapshotted into this chunk: the repository is public, and the overseer ruled at
  the P5 review that each is recorded by its sha256 and byte count instead (inputs#I8; the table is research.md
  §The operator's files).
  - The pass-through (`l4-shim/llama-cli` in that folder; as re-read at 2026-10-07 14:20 local, sha256
    `66dc9cc4…c742`, 1230 B, mode `rwxr-xr-x`) is a `/bin/sh` script. It writes its own name and its whole argv
    NUL-separated, a UTC stamp, and the operand that follows `-p`, then runs `exec` on the real `llama-cli` with
    the same argv. Two things the relay does not say: a capture that cannot be written is silent, and the real
    binary runs regardless; and `exec` keeps the process id, so Pulse's kill-on-drop still reaches the real
    binary. The overseer checks after the launch that a capture directory appears (inputs#I8).
    - [premise-corrected: at P3 the script set `umask 077` and left it, so the real binary inherited it. The
      overseer changed it on that finding: the script now saves the caller's umask and restores it before `exec`
      (re-read at 14:20 local; the overseer's re-test of that instant, "prompt sha in = out", inputs#I8). The
      figures above are of the changed file.]
    - The overseer's self-tests are theirs and were not re-run here: running the script writes under the capture
      folder.
  - The capture env file (`l4-env-capture.sh`; sha256 `59d167a3…b2c7`, 1016 B) sources the plain leg env
    (`l4-env.sh`; sha256 `0bc0706d…be2b`, 1080 B, byte-equal to the copy the fifth series holds), keeps the real
    binary in the operator's own variable `PC_L4_REAL_LLAMA_BIN`, and points BOTH
    `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` and `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` at the pass-through. The model
    path and the tokenizer path are unchanged. In the plain env both binary handles already named one `llama-cli`
    build, and that build is the pass-through's `exec` target (measured at P3 and again after the change: the
    script's target line equals the operator variable, count 1).
  - Captures land in one directory per invocation, named by a UTC stamp to the second, the nanoseconds and the
    process id, under a label directory of the operator's capture folder; each holds `argv.nul`, `started-utc`
    (UTC, nanoseconds) and `prompt.txt`. The label is read from a file inside that folder which the overseer
    writes before the first drive. That the label is `capture-20261007` is the relay's word (inputs#I1 §1); it
    sits under the folder this chunk does not read.

## What this chunk builds

1. **A pre-registered capture-run record**, fixed BEFORE d1: a new section `## The 2026-10-07 capture run` in
   `contracts/pulse-real-model-leg-posture.md`, add-only, between `## The 2026-10-07 series` and `## The quiet
   window and serialization`. It fixes: three drives, one fresh data dir, the fifth series' design, the launch
   posture at Pulse `f70be92`, that the run is a CAPTURE whose grades are observations and never a `v3-09` verdict,
   that no drive is replaced or added, and the posture note of item 2. Its sha256 is recorded in this chunk's
   `evidence/attempt-ledger.md` before d1.
   - Verified at P3: the pre-registration mechanics carry over unchanged. `pre_registered(evidence, heading,
     sha256)` reads the contract at a constant path, cuts the section at its exact heading line and holds its
     digest against the ledger's `pre-registration sha256: ` line and a pin
     (`crates/conductor-run/tests/real_model_grading/mod.rs:153`). The heading differs from the fifth series' by
     its last two words, so the two sections never resolve to each other.
2. **The disclosure — the leg's whole part in the vehicle** (inputs#I1 §1 "Your leg's part is disclosure, nothing
   more"). The run's record carries a dated posture note: the launching shell's model-binary handle was the
   operator's recording pass-through, and the leg neither set it nor read what it recorded. It stands in the
   pre-registered record, the attempt ledger and the report.
3. **What the leg must not touch** (inputs#I1 §1; inputs#I2): the files under
   `~/dev/projects/additional/pc-overseer/l4-capture/` are not opened, copied, hashed or counted, by the leg or by
   the agent running it. Their reader is the Pulse builder.
   - [premise-corrected: the entry says committed evidence "carries derived facts only (byte counts, a sha256 per
     prompt, line counts per digest section, which sections differ between drives)". That was the overseer's
     provisional shape while the vehicle was open (inputs#I3 §3). Under (A) every one of those facts is a read of
     the recorded prompts, which inputs#I1 §1 forbids. This chunk commits none of them. What Conductor commits is
     its own evidence, as every series did: its scrubbed MCP re-read captures and the facts of item 5.]
   - [premise-corrected: the entry says the run "delivers, for every model invocation of every drive … the exact
     prompt bytes and the full argv, tied to the drive, the digest it belongs to … and the generation's grade".
     Under (A) the operator's pass-through delivers the bytes and the argv; this chunk delivers the TIE and the
     grade. The tie is time (item 5).]
4. **The three drives**, d1..d3, the fifth series' design: each through `bash scripts/agent-run.sh run --live
   real-model` against a `pulse-app` launched per the posture contract, one fresh home-rooted data dir under
   `~/.cache/pulse-legs/` with a NEW leaf. At take-up that directory holds `rm-trigger-series` and
   `rm-fifth-series`; both stay untouched as evidence. The leaf is model input (the fifth series' P3 finding: the
   launch cwd's basename reaches the prompt's `PROJECT:` line), so it carries no digit, no `@`, no vendor prefix,
   no keyed scrubber word and neither word of the grading rule; P4 names it and its absence before creation is
   recorded.
   - The only term of the launch that differs from the fifth series is the env file the launching shell sources
     (the vehicle above). Verified at P3 from the fifth series' ledger (`:132-142`): the agent launched `pulse-app`
     from a scratchpad script with one env block carrying the plain leg env's four handles as sourced, and stopped
     it after d3, on the overseer's go.
   - Each drive's Conductor capture is committed under this chunk's `evidence/` through the standing scrub chain
     (`mask_workspace_key` + `redact_value` + `mask_host_paths` + `elide_fingerprints`), digest-pinned, no capture
     text in test source. Verified at P3, and it is not optional: the harvest walks every chunk's `evidence/` for
     `rm-capture*.txt` and holds the population to an exact count (`COMMITTED_CAPTURES = 20`,
     `real_model_grading/capture_population.rs:9`), so three more files turn it red until the count moves with
     them; and the capture-text arm holds the grading modules' directory to an exact set (`GRADING_MODULES`,
     `real_model_harvest.rs:1255`). So the run takes its own grading module, registered like a series', with no
     verdict test.
   - Each drive's grade is recorded as an OBSERVATION in the attempt ledger beside its capture. Three of three does
     not make `v3-09` met; a miss does not count against it; no ref test is written and no verdict is stated. If no
     drive misses, that is a finding, reported as such, and the run is still delivered (inputs#I1 §2; inputs#I3 §3).
5. **The tie between a drive and the operator's captures is time** (inputs#I1 §1). The report names the path the
   Pulse builder reads, as relayed (inputs#I1 §1) and never as a reading the leg took. Verified at P3:
   - Each drive's start and end are the first and the last `timestamp_ms` of its frozen self-obs stream
     `runs/live-suite/rm.jsonl` (epoch milliseconds from `std::time`; the first line is the leg's `observability
     initialized`, written before the preflight canary, and the last is `report.generate` closing). Measured on the
     stream the fifth series' d3 left: 08:04:42.989Z and 08:10:44.458Z, against that ledger's "fired 08:04:42Z,
     ended 08:10:45Z". The next invocation deletes that file, so the two stamps are read after each drive.
   - Each committed capture already prints every `interpretation.prompt.assemble` line of its drive's window with a
     millisecond UTC stamp and a `token_count`, and every `digest.assemble.request` line with its mode and cue kind
     (the fifth series' `rm-capture-d1.txt:449-459`, `:505-510`). The prompt is an operand of the spawn, so its
     assembly precedes the pass-through's own stamp; the size of that gap is not measured here.
   - The pass-through stamps in UTC on the same host clock (read from its source), so the two sides compare
    directly.
   - Pulse assembles a cadence digest about once a minute whether or not a drive is running, so the operator's
     folder also holds invocations that fall in no drive's bracket (the quiet windows, the launch). The report says
     so; it gives brackets and never an invocation count.
6. **The binaries.** Pulse is pinned at `f70be92`, the sha the fifth series ran. Read at take-up in the Pulse repo:
   HEAD `48714f0`, `git diff --stat f70be92 HEAD -- crates pulse-app xtask` empty; the working tree carries an
   uncommitted `pulse-app/examples/l4_decision_probe.rs` and route bookkeeping. Verified at P3 by digest:
   `target/release/pulse-app` hashes `df167647…ba4a` and `andromeda-pulse-mcp` `6175fc36…d2a9`, the two digests
   the fifth series' ledger records for its rebuilt `f70be92` binaries (`:61-62`). They are used as they are; no
   build is planned. A digest that no longer matches before d1 stops the chunk and is reported: a rebuild would be
   from the sha, never from the tree, and needs its own grant.
7. **The pre-leg checks**, recorded in the ledger before d1: the model's sha256 equals the pinned digest, the
   NVIDIA driver major equals the runtime's, the data dir leaf is absent.
   - [premise-corrected: the relay says the env file exports the real binary "for a pre-leg check that inspects
     the runtime itself". The fifth series ran no such check: its runtime check compares the NVIDIA kernel-module
     major with the userspace major and touches no handle (that plan's entry 12). Its handle check tests that the
     model handle names a file and the binary handle an executable (entry 10); under the capture env that tests
     the pass-through.]
   - This run adds a posture check, session-level and printing no value: the two binary handles name one file,
     that file is not the real runtime, the real runtime is executable, and the pass-through's `exec` target is the
     real runtime. It is what tells a shell that sourced the capture env from one that sourced the plain env.
   - `ANDROMEDA_PULSE_L4_ALLOW_ROOT` read unset at P3 in a shell that had sourced the capture env (the Bash tool's,
     not the one that will launch); the launch re-reads it.
8. **The hypothesis, and what proves it for this run.** `hypothesis:` the pass-through changes nothing the model
   sees (inputs#I1 §2; inputs#I3 §2: "same binary, same argv, same environment").
   - Re-derived at P3 (inputs#I6, Pulse `pulse-app/src/llamacli_inference.rs`, unchanged from `f70be92` to HEAD):
     the prompt is the last argv element, after `-p` (`:491`); the binary is admitted when it is a regular file
     (`:285`, `validate_path_input` at `:606`); the child is spawned with that argv, stdin null and both output
     streams piped, and the file sets no environment and no working directory on it (0 hits for `.env(`,
     `env_clear`, `env_remove`, `current_dir`), so it inherits `pulse-app`'s. The two `interpretation.model.load`
     lines carry the tier, the load status and a model identity taken from the model FILE's stem, so the binary's
     path reaches no line Pulse logs there and none Conductor's capture ingests.
   - What this run shows from Conductor's side: the `pulse-app` binary is the fifth series' by digest, so the argv
     builder and the embedded grammar are the same bytes; Pulse's boot log reads the model `loaded` with the
     pass-through as its binary; and in every drive's capture the prompt version reads as in the fifth series,
     every generation parses under the grammar, and no inference error or skip is logged.
   - [premise-corrected: the entry's control — "the recorded argv replayed by hand yields a well-formed
     generation" — reads the recorded argv, which is under the capture folder. It is not this chunk's to run under
     (A).] Not proved here: that the real binary received the argv byte for byte (the operator's self-test and the
     script's `exec` line carry that). One difference in `pulse-app`'s own environment stays and is recorded: the
     env file exports one more name, the operator's variable (inputs#I8).
9. **The sitting.** The GPU is open in daytime today on the founder's 07:42 word and closed at night; Pulse's
   builder shares it. **The go is asked of the overseer before the launch** (inputs#I2; inputs#I1 §3), and the
   sitting is planned to end in daylight: implement reaches d1 early, only what the leg reads precedes it, and a
   sitting that would run into the night is not started. The fifth series' sitting ran 07:45:11Z to 08:11:04Z,
   launch to stop. The bound, ruled by the overseer at the P5 review (inputs#I8): the sitting ends by 19:00
   local, and no go is asked after 18:25.
10. **Architecture §Occupied Resources** (the entry's second CARRY): it stands at its byte threshold, 38115 B
    (`scripts/arch-registry-check.py measure --file .andromeda/architecture.md`, re-run at P3), so an amendment
    that grows it first frees bytes there. Under (A) the `:204` edit the CARRY anticipated does not happen. The
    chunk registers nothing there: the contract record names only handles the contract already names, and the
    operator's variable and the CPU binary handle are named in this chunk's plan and ledger alone (the standing
    rule: only a handle a SHIPPED artifact names is registered).
11. **The supply-chain gate**, as every chunk: clear advisory-db residue or pass a fresh `--db` before reading
    `cargo audit`; `cargo deny` beside it.

## Folded freight (`working-route.md:96`, three blocks, per `route.py pins`)

- **CONTEXT** (1992 chars, folded whole) → the matrix target line, items 1, 3, 4, 5, 6 and 9. Its founder ruling
  of 13:13 (the builder must be able to read the digest itself; the corpus-key read stays refused) is the END this
  run serves. Its provisional terms — a capture and never a verdict, the prompts stay on this host and are never
  committed or pasted into a report, a card or a relay — stand, the second now by construction: the prompts are in
  the operator's folder and Conductor never reads them.
- **CARRY** (1686 chars, folded whole) → §The vehicle and item 8. Resolved by the founder's 13:49 pick.
- **CARRY** (368 chars, folded whole) → item 10.

## Causal claims carried in (closed at P3, marker kept)

- "Measured at this wrap: Pulse's reproduction reading of 2026-10-07 read not reproduced — 199 of 200 generations
  `identifies` = `both` over 10 shapes × 20 on the shipped prompt … its chunk stands `pending` and uncommitted in
  Pulse, no remedy exists, and the prompt d3 received is unread" (the CONTEXT block). Context for why the run
  exists; nothing this chunk builds turns on the count, and it was not re-counted here.
- "Pulse hands the prompt to the model binary on argv (`-p`, measured at Pulse
  `pulse-app/src/llamacli_inference.rs:491`, the file unchanged since `f70be92`) and admits any canonicalized
  regular file as that binary (`:285`, `validate_path_input` at `:606`; `ANDROMEDA_PULSE_L4_ALLOW_ROOT` unset in
  this wrap's shell)" (the first CARRY) — VERIFIED at P3, item 8.
- "hypothesis: the pass-through changes nothing the model sees" (the first CARRY; inputs#I1 §2) — closed as far as
  item 8 states, with what stays unproved named there.
- "`48714f0` differs from it in no file under `crates pulse-app xtask`, while Pulse's working tree carries an
  uncommitted `pulse-app/examples/l4_decision_probe.rs`" (the CONTEXT block) — VERIFIED at take-up and again at P3
  (`git status --porcelain -- crates pulse-app Cargo.toml Cargo.lock` prints that one path).

## Boundaries (out of scope)

- No file under the operator's `l4-capture/` is opened, copied, hashed or counted. No Conductor file sets or reads
  a model-binary handle. No recorded prompt, and no fact derived from one, enters a Conductor artifact, a report,
  a card or a relay.
- No `v3-09` claim, verdict, ref test or matrix write. No change to the series design or the grading rule.
- No edit to Pulse's repository and no Pulse build; Pulse is read at its committed state, never its working tree.
  No edit to the operator's vehicle.
- No edit to any frozen chunk's `evidence/`, no rewrite of the posture contract's dated records, and neither
  earlier data dir is touched.
- No drive before the pre-registered record's digest is in the ledger; no drive past three; none replaced.
- No launch before the overseer's go; no GPU run at night.
- The seven spec masters stay read-only.

## CI (Setup 5a)

Three commits from the last wrap's flip through HEAD:

- `6987764` (HEAD, the 0-pending route adaptation): `verdict: in progress` · checks 3/3 · CI#37617071200 — verdict
  not yet available at take-up. Re-read at P5 (2026-10-07 12:18Z), once the run settled: `verdict: green` ·
  checks 3/3 · wall 693 s · completed/success.
- `3afe4f6` (session 182's bookkeeping commit): `verdict: green` · checks 3/3 · wall 588 s · CI#37606657057.
- `b55f346` (the fifth series' wrap, the flip commit): `verdict: not green` · checks 3/3 · wall 656 s ·
  CI#37605669051 completed/cancelled. Read at take-up through `gh run view`: the Rust and Frontend jobs concluded
  `success`; the A11y job concluded `cancelled` with no failed step. The workflow sets `concurrency:
  cancel-in-progress: true`, and the run on `3afe4f6` was created at 10:19:45Z, inside this run's window (10:10:50Z
  to 10:21:50Z). `ci.yml` holds no diff-scoped step (0 hits for `git diff`, `HEAD~`, `event.before`), so the green
  run on `3afe4f6` exercised a tree that contains everything `b55f346` shipped. Nothing failed, so there is no
  failing subject to intersect with this chunk or to hand to another; recorded here as read and named on the
  review card.
