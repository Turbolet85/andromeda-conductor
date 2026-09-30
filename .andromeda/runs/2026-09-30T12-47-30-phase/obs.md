# obs extract

## Relevance
partial — the chunk is a diagnostic control leg plus an evidence record and two PREREQ gates, with no `crates/*/src` delta expected. So obs binds through artifact hygiene, the gate readings and how a control arm gets recorded, not through new instrumentation.

## Constraints
- Committed artifacts must carry no absolute host paths. This covers the control's evidence record, any speech-log extract and the configuration table (runtime, driver, NVDA, OS build, KB list, `allowInChromium`). The path forms to reject are the alternation in obs-plan §9 *Log conformance check*, and obs-plan §11 *Logs* holds the ban. Whether the chunk's evidence-hygiene gate uses exactly this alternation is research's question.
- The screen-reader speech log is untrusted third-party text. Its scrub (`<host-path>` placeholder + a `security_finding` flag) is a second line of defence, and obs-plan §11 *PII Scrubbing* (the last bullet) says the named redaction sites do not bound every host-path channel. A new host (Edge 154, or another WebView2 app) speaking a window title or an address is exactly such an unenumerated channel.
- The PREREQ has two gates. `cargo clippy … -D warnings` is the lint stage of obs-plan §9 *Pipeline integration*. `agent-run.sh run --unit` is the unit-test stage whose self-obs stream must meet the §3 base-line schema (obs-plan §9 *Log conformance check*) and hold zero unlogged panics (obs-plan §9 *Zero-unlogged-panics gate*; §10 always-required invariant). Which of those checks `--unit` actually runs is research's question.
- The Tauri backend's self-obs sink location depends on the suite (obs-plan §3 *Log file location*). The file name is fixed and the directory follows `CONDUCTOR_RUNS_DIR`: `runs/logs/` for `sr-empty` and `runs/sr-leg/logs/` for `sr`/`sr-error`. A control arm that drives Edge or a third-party WebView2 app instead of `conductor-tauri` produces no `conductor-tauri.jsonl` at all. The plan must state that absence as expected for that arm, never read it as a harness fault.
- Every signal needs an agent-readable consumption path (obs-plan §11 *Universal (agent-driven specific)*). Each control arm's heard/silent reading must come from a machine-parseable record, the NVDA speech log with its timestamps, even when an operator presses Tab at the physical keyboard.
- If any Rust delta does arise, the bounded span-name set in obs-plan §11 *Spans / Traces* holds and no self-obs OTLP is allowed (obs-plan §11 *Universal*; §3 *OTel SDK init*). No new span or metric is expected for a leg with zero `src` delta.

## Patterns to follow
- The a11y leg's existing speech-log ingest is the pattern: scrub to `<host-path>` and flag a `security_finding` before the record is committed, then count both in the evidence. The prior reading of 0 `security_finding` rows and 0 placeholders is cited as a measured baseline in obs-plan §11 *PII Scrubbing*.
- Name a Tauri-arm leg's self-obs file by its per-suite directory, not the project-root `logs/`, as in obs-plan §3 *Log file location*.
- Report a hygiene hit by `path:line` and rule, never by quoting the matched text. This is the discipline for the repository-hygiene gates in obs-plan §9 *Pipeline integration*, and it keeps the gate's own output out of committed artifacts.

## Anti-patterns to avoid
- Never let an absolute host path or internal struct name into logs or committed evidence (obs-plan §11 *Logs*, *Error Reporting*). This includes a host path spoken by NVDA in a new host's window, and a raw capture pasted without the scrub.
- Never use human-review-gated analysis without a machine-parseable export (obs-plan §11 *CI* and *Universal*). A control whose verdict rests on an operator's recollection alone does not count as a reading.
- Never add a retry that masks a real failure (obs-plan §11 *SLO*; the plan states this for panic handling, parallel to tests' zero-flakiness). The scope's "never a rerun-until-announced" is the same principle for the three owned regrade reds.

## Contract bindings
- obs ↔ a11y: the SR speech-log scrub (`<host-path>` + `security_finding`) is the a11y leg's mechanism, and obs-plan §11 *PII Scrubbing* records it as the second defence line. It must extend to whatever host control (1) points NVDA at.
- obs ↔ tests harness: `agent-run.sh run --unit` consumes the §3 self-obs base-line schema (obs-plan §9 *Log conformance check*). Test-plan §3 owns the command form and research confirms it.
- obs ↔ security: the host-path alternation in obs-plan §9 is also the evidence-hygiene gate's anchor. The configuration record will name registry-sourced facts (OS build, KB updates, the runtime version), so how those are rendered is security's and research's question, not obs's.

## Acceptance criteria contributions
- Every committed file the chunk adds under its `evidence/` returns zero absolute-host-path matches over the obs-plan §9 alternation. Any scrubbed speech-log utterance appears only as `<host-path>` with a matching `security_finding` row, and both counts are recorded per arm (per obs-plan §9 *Log conformance check*; §11 *PII Scrubbing*).
- The PREREQ entries (`agent-run.sh run --unit` and `cargo clippy --workspace --all-targets -- -D warnings`) both RUN in this chunk and read green. Each exit code is captured from the bare command, and the `--unit` output holds no unstructured `^thread.*panicked` line (per obs-plan §9 *Zero-unlogged-panics gate*; §10).
- Each control arm's heard/silent reading cites a timestamped NVDA speech-log span, a machine-parseable source, alongside its recorded configuration. A row graded with no log span behind it fails (per obs-plan §11 *Universal (agent-driven specific)*).
- For a non-Conductor arm, the record states that no `conductor-tauri.jsonl` is expected. For a Conductor arm, it names the per-suite sink directory the leg wrote to (per obs-plan §3 *Log file location*).
