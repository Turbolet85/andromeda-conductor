# security extract

## Relevance
Partial — the posture decision itself is not a security surface, but the folded-in `cargo audit` PREREQ is squarely §Dependency Security, and Branch A (real-model live leg) crosses the sidecar-spawn, MCP read-back, and run-report-sanitization boundaries.

## Constraints
1. **Audit-tool versions are floors, not pins.** If `cargo audit` is still red at this chunk's gates for the same external RustSec-DB parse fault, the resolution is raising the cargo-audit **floor** in the plan body to the fixed release — not a `deny.toml` entry, not a CI edit (security plan §Dependency Security → Audit tool + Accepted exceptions; `D:/dev/projects/conductor/.claude/rules/security.md:46`).
2. **The release/merge gate is audit + deny both green, and the overlap must be *verified*, not assumed** — `cargo deny check` running green is what keeps coverage while `cargo audit` is externally broken (security plan §Security Anti-Patterns → Universal, "NEVER run `cargo build --release` … without cargo-audit (and recommended cargo-deny) green"; §Dependency Security → CI integration).
3. **If this chunk lets Conductor observe `ANDROMEDA_PULSE_L4_DETERMINISTIC`**, it is a presence/boolean non-path handle → no garde boundary needed, but it MUST NOT reach the sidecar via argv or a shell string; strictly the `.env(...)` builder after metacharacter rejection (security plan §Input Validation → env-handle rows + non-path note; §Security Anti-Patterns → Input, Code Patterns).
4. **Branch A hypothesis evidence comes from MCP read-back ONLY.** Conductor production code never opens, copies, or exfiltrates Pulse's plaintext `corpus.db`, and never persists its content into Conductor artifacts (security plan §Data Protection; §Security Anti-Patterns → Data Protection).
5. **Whatever carries the posture (caveat line, deferral record, report field) must stay sanitized** — run-report `<run_id>.md`, `runs.db` rows, and JSONL journals carry verdict/state/identity fields only: no absolute host paths (canonicalized `CONDUCTOR_*`, `ANDROMEDA_PULSE_DATA_DIR`) and no internal seam-crate struct/field names (security plan §Error Handling → Run-report artifact sanitization; §Security Anti-Patterns → Logging).
6. **Any new `runs.db` column/row for the posture uses rusqlite bound parameters** — never `format!`/concatenated SQL, even for self-generated synthetic content (security plan §Input Validation → runs.db row; §Security Anti-Patterns → Input).
7. **A real-model (non-deterministic) leg must not blur the verdict/error wall.** Transport/JSON-RPC faults stay typed values routed to `Ok(Verdict/ReportState)` or harness-fault `Err`, never panics; a failed preflight or empty read-back is `blocked` with its named precondition string, never a false pass (security plan §Error Handling; §Security Anti-Patterns → Universal).

## Patterns to follow
- **Hardened sidecar spawn** — `D:/dev/projects/conductor/crates/conductor-verify/src/spawn.rs` (fixed hard-coded program path, injection-metachar reject, propagation via `command.env(DATA_DIR_ENV, data_dir)`). Any additional SUT-mode env this chunk propagates rides that same builder.
- **`resolve_under` traversal guard** for artifact paths derived from `CONDUCTOR_RUNS_DIR` — `D:/dev/projects/conductor/crates/conductor-cli/src/paths.rs`, `D:/dev/projects/conductor/crates/conductor-core/src/config_path.rs` (already used by the report/journal/db writers).
- **Presence-only non-path env handle** documented rather than validated — the `CONDUCTOR_AGENT_MODE` precedent (security plan §Input Validation, final paragraph) is the template if L4-determinism observation lands.
- **Bounded read-back decoding** — `D:/dev/projects/conductor/crates/conductor-verify/src/jsonrpc.rs` parsing to `serde_json::Value` with faults mapping to `VerifyError::{JsonRpc,Decode,Transport}` (security plan §Input Validation → MCP read-back child stdout).
- **Bounded-deferral discipline for external supply-chain decay** — re-check, then raise the floor; never suppress (`D:/dev/projects/conductor/.claude/rules/security.md:46`; `D:/dev/projects/conductor/.claude/session-handoff.md:19`).

## Anti-patterns to avoid
- **Do not silence `cargo audit` with a `deny.toml` `[advisories] ignore` or a CI-workflow edit.** The justified-ignore mechanism is for a *new dep* pulling a non-actionable advisory — a disjoint case from a transient external DB fault (security plan §Dependency Security → Accepted exceptions).
- **Do not read Pulse's `corpus.db` directly to check whether the top hypothesis names the injected root cause** — the tempting shortcut is exactly the banned one (security plan §Security Anti-Patterns → Data Protection).
- **Do not let the "interpretation not covered" caveat become a softening channel** — a real preflight/read-back failure still surfaces as `blocked`/`Err`, not as narrative in the report (security plan §Security Anti-Patterns → Universal, no silent preflight downgrade).

## Contract bindings
- **Supply-chain gate ↔ tests §CI Integration** — the audit/deny gate is a job in the single `.github/workflows/ci.yml`; this chunk resolves the deferral in the plan body only, and CI's own `cargo audit` step keeps hitting the same wall until upstream heals (scope: "not a CI edit").
- **Posture caveat ↔ obs (run-report envelope / journal field shape) + design (CLI + webview rendering)** — field naming and placement are theirs; the no-absolute-paths / no-internal-struct-names sanitization rule on the emitted artifact is security's.
- **Branch A leg ↔ tests** — non-deterministic by construction, so it stays outside the zero-retry CI test set; the security-side CI control remains the audit/deny job, not the live leg.

## Acceptance criteria contributions
1. `cargo audit` re-run at this chunk's gates and its outcome recorded: exit 0 → deferral explicitly closed; still exit 1 on the same advisory-DB parse error → cargo-audit floor raised to the fixed release in security-plan §Dependency Security, with **no** `deny.toml` change and **no** CI change (diff verifies).
2. `cargo deny check advisories bans sources licenses` exits 0 and is shown green in the chunk's recorded gates (the overlapping signal, verified rather than assumed).
3. If any posture text/field reaches `<run_id>.md`, a `runs.db` row, or the JSONL journal: grep of the emitted artifact for absolute host paths and seam-crate struct/field names returns zero hits.
4. If any new env handle or sidecar env propagation lands: it flows through `spawn.rs`'s `.env(...)` after metachar rejection, with zero occurrences in argv or shell strings (grep verifies), and no direct `corpus.db` open appears in production code.

## Relevant amendment history
- **2026-06-15-dependency-audit-gate** — reframed cargo-audit/cargo-deny pins as **minimum floors** (external CLI tools, unlockable in `Cargo.lock`; advisory DB fetched fresh each run). This is the amendment that makes "raise the floor" the sanctioned PREREQ resolution and keeps it non-escalating.
- **2026-06-23-line-oriented-output-rendering** — recorded the two justified `deny.toml` exceptions (RUSTSEC-2025-0119 ignore, `Zlib` license allow) and established that mechanism's scope; it is the *disjoint* case the PREREQ must not borrow, and it is the origin of the deliberate audit↔deny overlap now doing the covering.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — added `CONDUCTOR_AGENT_MODE` to the non-path, presence-only, no-validation env-handle note; the precedent for documenting (not validating) an L4-determinism observation handle without opening a new input boundary.
- **2026-06-27-mcp-read-back-result-shape-adapter** — reconciled the read-back boundary to the hand-rolled bounded JSON-RPC client with `VerifyError::{JsonRpc,Decode,Transport}`; that is the exact path a Branch A hypothesis assertion would consume.
- **2026-06-27-live-pulse-e2e-proof** — corrected `corpus.db` to plaintext (P-049 encryption assumption wrong) and sharpened the MCP-read-back-only posture. This is the prior live chunk whose OPEN posture decision (`.claude/rules/verification-harness.md:47`) this chunk closes, so its corpus-access rule is the live constraint on Branch A.
- **2026-08-08-sut-capability-manifest** — registered `contracts/pulse-capabilities.toml` as a validated-at-load boundary with read faults carrying `e.kind()` only, never the path; `v2-05` is a row in that manifest, so any manifest-side touch inherits those rules.
