# security extract

## Relevance
partial — the chunk ships no code path, but it commits a close record under `evidence/`, touches two governed harness scripts (comments only), states the one known model-text exception, and carries a boundary question about security-plan's fixed-path-manifests row.

## Constraints
- security-plan §Security Anti-Patterns → Data Protection requires that corpus-rendered model text enter a committed file only inside a chunk's `evidence/` tree AND only through the capture's scrub chain. The close record sits under `evidence/` but is not produced by that chain, so it must carry no model text at all: the rank-1 statement is named as an exception and quoted nowhere (close record, `report.md`, plan, commit message).
- security-plan §Security Anti-Patterns → Data Protection states the exception's scope as the `evidence/` tree and says the ban binds unchanged anywhere else; as read, the plan holds no record of the frozen capture-run `report.md` exception the founder ruled stays. Whether the plan owes that ruling a dated record is a boundary-clause question for the plan review — nothing is written on this extract's reading.
- security-plan §Input Validation (the committed SUT-facing manifests row) requires the recording duty to bind per READER, and holds that a committed artifact parsed at runtime earns its row whether or not a shipped binary parses it. The row names four manifests; `contracts/pulse-real-model-leg-posture.md` is named nowhere in the plan (0 mentions, counted by file name). Whether `pre_registered` (opens the file, checks one section's sha256) falls in that row's class — "SUT-facing manifest", "parsed at runtime", bounds-checked at load — is research's question; the answer goes to the review, never into the plan.
- security-plan §Security Anti-Patterns → Code Patterns (rule (b)) governs `scripts/agent-run.{sh,ps1}` as the routine harness locus and `scripts/a11y-token-witness.ps1` as the seventh form's leg entry point. The two re-points must leave every spawn form, argv element, guard and exit path of both files unchanged; a change beyond comment lines is a governed-form change and escalates.
- security-plan §Input Validation (the committed SUT-facing manifests row) requires `scripts/agent-run.{sh,ps1}`'s read of the run contract to stay never-defaulted with its clamp-up rule; the `agent-run.ps1` edit must not move that read. Whether line 230 sits near it is research's question.
- security-plan §Security Anti-Patterns → Logging requires committed evidence records to carry no absolute host path and no internal seam-crate struct name. The close record names data dirs, captures and helpers; it must name them host-path-free. Whether naming a test helper by its function name trips the chunk's hygiene gate is research's question.
- security-plan §Dependency Security requires `Cargo.lock` committed and un-drifted, and §Security Anti-Patterns → Universal bans a merge or release build without `cargo audit` and `cargo deny` green. The chunk moves no lockfile and no manifest; a pipeline step that would merge to `main` is a stop, and would owe both gates first.

## Patterns to follow
- The recorded shape of a test-binary reader of a committed `contracts/` file — root resolved from `CARGO_MANIFEST_DIR`, no `CONDUCTOR_*` handle, read faults by error kind only, absent or malformed a hard fault — per security-plan §Input Validation (the committed SUT-facing manifests row); it is the form a row for the posture reader would take if one is owed.
- The nearest recorded precedent for a test helper that opens a committed file by hard-coded path and holds it by sha256 is the harvest reader, per security-plan §Input Validation (the real-model capture ingest row); research can read `pre_registered` against it.
- Stating a thing without holding it: naming a file by sha256 and byte count and copying none of it, per security-plan §Security Anti-Patterns → Data Protection (the capture run's operator-owned recording) — the form for stating the `report.md` exception.
- A changed boundary reading is recorded as a dated correction that keeps the earlier statement as history, never a silent rewrite, per security-plan §Security Anti-Patterns → Code Patterns (the seventh form's scope correction) — the form any founder-ratified security-plan note would take.

## Anti-patterns to avoid
- Never let model text reach a committed file outside the ratified route, and never treat a ruled exception as a widening of the clause: the plan's own precedent records an out-of-scope occurrence as remedied or stated, never ratified by a delegate (security-plan §Security Anti-Patterns → Data Protection).
- Never let a committed evidence record carry an absolute host path (security-plan §Security Anti-Patterns → Logging).
- Never move a governed spawn form under cover of a comment edit (security-plan §Security Anti-Patterns → Code Patterns).

## Contract bindings
- security §Input Validation (committed SUT-facing manifests row) ↔ architecture §Occupied Resources registry row and its directory-structure key file: the corrected "no Rust reader" sentence and any security-side row must describe the same reader in the same terms; the plan review settles both together.
- security §Input Validation (real-model capture ingest row) ↔ tests: the harvest walks every `rm-capture*.txt` under `conductor-0.3.0/chunks/*/evidence/` with its population pinned by count. This chunk's `evidence/` tree sits under that walk; a file named in that pattern would move the pin. Whether the close record's file names stay clear of it is research's question.
- security §Secret Management (Secret-scan gate shape) ↔ tests CI (`rust` job): every file this chunk commits is in the gate's listing. The close record may cite sha256 digests; whether a digest reads as secret-shaped to the gate's rules is research's question.
- security §Security Anti-Patterns → Logging ↔ obs redaction boundary: committed evidence is held to the same host-path rule as run artifacts.

## Acceptance criteria contributions
- The rank-1 model statement, and any other corpus-rendered model text, occurs in no file this chunk writes or amends; the exception is stated by location only (per security-plan §Security Anti-Patterns → Data Protection).
- The committed close record holds zero absolute host paths (per security-plan §Security Anti-Patterns → Logging).
- The diff of `scripts/agent-run.ps1` and `scripts/a11y-token-witness.ps1` touches comment lines only — one line each — and no executable line of either file moves (per security-plan §Security Anti-Patterns → Code Patterns).
- The secret-scan gate target passes over the tree with this chunk's files present, and `.andromeda/security-plan.md` is byte-unchanged by this chunk unless the founder's word, through the operator, says otherwise (per security-plan §Secret Management).
