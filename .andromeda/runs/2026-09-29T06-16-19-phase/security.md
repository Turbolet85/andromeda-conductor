# security extract

## Relevance
partial. The chunk adds no new trust boundary. It does, however, re-drive the real-model capture ingest, which reads untrusted SUT output into committed evidence. It may read corpus-rendered report sections for P-031/P-033/P-034/P-044 for the first time. It may change the preflight canary and the run contract, and it adds scenario config that names P-IDs. Each of those surfaces carries security mandates.

## Constraints
- Anything the real-model capture writes to the committed `evidence/` tree is untrusted SUT output. security-plan §Input Validation (the real-model capture ingest row) requires:
  - the MCP re-read is bounded (a poll bound and an id-sweep bound);
  - every printed line passes `redact_value` and then `mask_host_paths`;
  - Pulse's witnesses are read fields-only, and the workspace basename is never printed;
  - envelope `fingerprints` are elided to counts;
  - the output is buffered and written ONCE.

  Changes to the canary, its digest or its poll budget must keep every one of these properties. Whether the current capture path still satisfies all of them after this chunk's edits is research's question.
- Grading P-031/P-033/P-034/P-044 will likely need the attributed incident's `retrieve_report` sections in committed evidence. That is corpus-rendered model text. It may enter committed evidence only under the ONE ratified, scoped exception in security-plan §Security Anti-Patterns → Data Protection: through `redact_value` + `mask_host_paths`, with fingerprints elided. The plan records that the 2026-09-23 capture carried no report text. So this chunk would be the first to exercise the exception, and the plan should name it explicitly. Anywhere outside that exception the corpus-persistence ban still binds unchanged.
- The canary fix must not silently downgrade the preflight (security-plan §Security Anti-Patterns → Universal; §Input Validation, MCP child stdout row):
  - "no incident opened after the canary storm" stays the distinct named `Blocked` precondition, never the generic corpus-empty string;
  - an absent or unparseable opened-at stamp reads as NOT-fresh, and so `Blocked`, never a false pass;
  - the preconditions stay FIVE. A real-model posture mismatch is `Blocked` at scenario level and never becomes a sixth precondition.
- If the fix touches the poll budget through `contracts/pulse-run-contract.toml`, security-plan §Input Validation (the committed SUT-facing manifests row) requires:
  - the contract is read at a fixed `default_path()` with no `CONDUCTOR_*` override, and is bounds-checked (a non-zero `min_canary_poll_seconds`, no zero-emission warm-up);
  - absent or malformed is a hard harness fault, never defaulted.

  Its shell readers `scripts/agent-run.{sh,ps1}` carry the same rule: a missing term exits 2, and a lowered `CONDUCTOR_PREFLIGHT_TIMEOUT` clamps UP to the contract floor.
- Any new or edited scenario naming the four P-IDs must pass the full load-time trust boundary (security-plan §Input Validation, scenario config row; §Security Anti-Patterns → Input):
  - garde `range`/`custom`, with `dive` and never `skip` on nested specs;
  - the sibling-spanning `Scenario::check_*()` rules invoked from `from_toml_str`, including P-ID membership via `check_capabilities` against the manifest;
  - `l4_posture` as the CLOSED `deterministic`/`real-model` enum, where an unknown value is a load-time `CoreError::Config`.
- The capture's path handles must keep resolving through `conductor-run/tests/capture_paths` (security-plan §Input Validation, path-handle row and capture-ingest row):
  - `CONDUCTOR_RUNS_DIR` through `runs_dir_from` → `resolve_under`, rejecting an absolute or `..` value;
  - `ANDROMEDA_PULSE_DATA_DIR` canonicalized and required to be a directory before `logs/` is joined.

  Every rejection must be a path-free reason.
- Under the real-model posture the L4 handle must be ABSENT or falsy, graded under the union truthiness rule `flag_declared_on_either_side`. Handle names only: no value becomes a path, an argv element or a log value (security-plan §Input Validation, the `ANDROMEDA_PULSE_*` declaration-only row).

## Patterns to follow
- The capture ingest's scrub path (`redact_value` → `mask_host_paths`, fields-only witnesses, fingerprint elision, a single buffered write) is the model for any added capture output, such as digest content or model-decision evidence gathered for the canary measurement (security-plan §Input Validation, real-model capture ingest row).
- The harvest reader's shape is the model for any new grading reader: `real_model_harvest` loads the committed capture from `CARGO_MANIFEST_DIR` plus a hard-coded path, with no `CONDUCTOR_*` handle, and asserts pinned literals (security-plan §Input Validation, real-model capture ingest row). A harvest extended to four ids should keep that shape.
- The capture's sidecar spawn goes through the hardened `ReadbackClient::connect` (security-plan §Security Anti-Patterns → Code Patterns rule (a); §Input Validation, spawn-propagation row):
  - the fixed program name `andromeda-pulse-mcp` resolved through `PATH`;
  - `ANDROMEDA_PULSE_DATA_DIR` passed only via `.env(...)`, after metacharacter rejection;
  - the console window suppressed.

  Any added drive tooling must reuse it rather than spawn the sidecar itself.
- The `run --live real-model` selector is a closed `case` allowlist that leads with `conductor preconditions --for real-model-interpretation`. An unmet subject refuses at exit 1 before any leg spawns (security-plan §Input Validation, CLI arguments row; §Security Anti-Patterns → Universal, invoked-preflight clause). A new scenario name is resolved through the validated scenario load, and a `P-NNN`-shaped `--for` value is refused.

## Anti-patterns to avoid
- NEVER open, copy or stage a row in Pulse's `corpus.db` directly to induce a SUT state. P-044 may need prior incidents for retrieval, but seeding the corpus by file is the banned route (security-plan §Security Anti-Patterns → Data Protection). Retrieval history must arise through the SUT's own ingest or the MCP tool surface, or be recorded as unattainable.
- NEVER let committed evidence, the run report, `runs.db` or the journal leak absolute host paths (the `ANDROMEDA_PULSE_DATA_DIR` value, canonicalized `CONDUCTOR_*` dirs) or seam-crate struct names. This applies especially to any new diagnostic output added to tell the three canary causes apart (security-plan §Security Anti-Patterns → Logging; §Error Handling, run-report artifact sanitization).
- NEVER let a malformed or errored MCP response on the capture/read-back path panic, and never read an empty read-back as a pass. JSON-RPC errors and decode faults are typed inputs through the verdict/error wall (security-plan §Security Anti-Patterns → Universal; §Input Validation, MCP child stdout row).

## Contract bindings
- security ↔ tests: the host-path hygiene gates over the committed capture and harvest (security-plan §Input Validation, capture-ingest row) are the chunk's Test Commands. The real-model leg stays operator-gated and is never a CI gate, so its security evidence is a local gate result, not a CI job.
- security ↔ obs: the capture scrub and the tracing field-allowlist share the no-host-path / no-struct-name redaction invariant (security-plan §Security Anti-Patterns → Logging). The scrub binds the evidence record, and the obs allowlist binds self-obs log lines.
- security ↔ arch: the run contract (`pulse-run-contract.toml`), the capability manifest and the scenario catalog feed `check_scenario_backing` / `UNBACKED_AUTO`. security-plan §Input Validation (the committed SUT-facing manifests row) governs how they are read, never what the pin holds.

## Acceptance criteria contributions
- A host-path hygiene grep over the chunk's committed `evidence/` (capture, harvest inputs and any canary-diagnostic record) returns zero real host paths and zero seam-crate struct names. Every drive-letter, `%APPDATA%`, `/Users` or `/home` hit gets an explicit disposition: a real host-path leak fails the check, while a URL scheme, a registry form or the gate's own command text is ruled a non-leak (per security-plan §Security Anti-Patterns → Logging; §Input Validation, real-model capture ingest row).
- Any committed report-section text for P-031/P-033/P-034/P-044 is present only in scrubbed form, with envelope fingerprints elided to counts. The chunk report names the §Data Protection exception as the basis (per security-plan §Security Anti-Patterns → Data Protection).
- A canary drive that forms no fresh incident grades as the distinct named "no incident opened after the canary storm" `Blocked` precondition, never as a pass and never as the generic corpus-empty string. This holds before and after the Conductor-side fix (per security-plan §Security Anti-Patterns → Universal).
- If the dependency delta is non-zero: `cargo audit` is exit 0 over the committed lock, with the advisory-db porcelain probe first, and `cargo deny check advisories bans licenses sources` is exit 0 over the new `Cargo.lock`, both captured before any pipe (per security-plan §Dependency Security).
