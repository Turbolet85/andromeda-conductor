# security extract

## Relevance
Partial. This is a doc-compaction chunk with no expected Rust/TS change, so it adds no new trust boundary. It is still relevant for two reasons. First, the security plan relies on facts registered in the two arch sections being compacted, and cites them by section name. Second, the durable size measurement, if it ships as code, becomes a new reader of a committed artifact.

## Constraints
- Compaction must keep every arch fact that the security plan cites by section name. Each one has to survive as a current-truth statement under the same section label:
  - Occupied Resources: "No inbound listener of Conductor's own"; Environment variables — "no secrets/cloud env vars"; Ports; On-disk artifacts; Interface routes.
  - Established Decisions: Run-History Persistence; Deployment; CI/CD; MCP Read-Back Client.
  - Where they are cited: security-plan §Threat Model Summary (Data classification, Attack surface, Auth model Reason, Infrastructure) and §Security Anti-Patterns → Secrets / Code Patterns.
  - A dropped or renamed sub-label breaks those citations without any warning.
- The plan registers a set of network and port facts: the single deliberate `:4317` port-occupier bind, the harness-only loopback `4444`/`4445`, the unused `:4318`, and the one CI-only non-loopback egress (msedgedriver fetch). §Threat Model Summary → Attack surface / Networking and §Security Anti-Patterns → Universal / API require that no inbound listener is added to shipped binaries. The Occupied Resources → Ports row must keep stating this whole set after compaction. Shortening it must not drop a member. This follows from the scope's "no registered fact lost" rule.
- §Threat Model Summary → Attack surface and §Secret Management → Storage enumerate the env-var handles in the reserved `CONDUCTOR_*` / `ANDROMEDA_PULSE_*` namespace: `CONDUCTOR_RUNS_DIR`, `_SCENARIOS_DIR`, `_CONTRACT_MANIFEST`, `_SEED`, `_MSEDGEDRIVER`, `_NVDA`, `_E2E_SEED_DIR`, `_A11Y_STRICT`, and `ANDROMEDA_PULSE_{MCP_ENABLED,DATA_DIR,L4_DETERMINISTIC}`. Each handle is registered with a reader class (shipped, test-binary or dev-harness). The Occupied Resources → Environment variables row must still register every handle after compaction. Whether the current row already carries all of them is research's question.
- §Input Validation rules that a committed artifact parsed at runtime gets a boundary row, and that the duty binds per READER, test binaries included (2026-09-16 scenario-audit-ledger precedent). If P4 makes the measurement a Rust test that parses `architecture.md`, the plan requires that reader to:
  - resolve from `CARGO_MANIFEST_DIR` plus a hard-coded path;
  - use no `CONDUCTOR_*` override handle;
  - report read faults as `e.kind()` only, never the path.

  Whether a size-only byte count counts as "parsed" for this rule is a P3/P4 call. Name it explicitly; don't leave it implied.
- If the measurement ships as a script or harness step instead, §Security Anti-Patterns → Code Patterns rule (b) applies. The script must run a fixed program with fixed argv, with no shell string, no `eval`-equivalent and no operator-supplied value in argv. A plain read-only `run:` step using a fixed toolchain binary falls in the same class as the existing `cargo build` steps and is not a new governed form (per the 2026-09-09 amendment reasoning).
- §Security Anti-Patterns → Secrets and → Logging apply to text moved into `architecture-amendments.md`, to the compacted body, and to any committed measurement output. None of them may add a secret-shaped string, an absolute host path or a leaked `ANDROMEDA_PULSE_DATA_DIR` value. Moving existing text is fine as long as it introduces nothing new of these kinds.

## Patterns to follow
- Cross-master references use section names ("Occupied Resources: Environment variables", "architecture.md §Occupied Resources — Ports"), not line numbers. The security plan body has no `architecture.md:NNN` citations. Keep the section and sub-label names stable so these references still resolve.
- Current truth goes in the body and history goes in the sidecar. security-plan-amendments.md says of itself that "the body holds only current truth; history lives here + in git", and the chunk's compaction rule mirrors this.
- The cascade step 2 cross-master sweep: after a master-body edit, grep the seven masters, CLAUDE.md, the rule files, `docs/session-learnings.md`, `playbook.md` and `drift-base.md` with a known-positive control, then record each hit as amended or NO-CHANGE. This is how stale security-plan citations of changed arch text have been caught before.
- The committed-artifact test-binary reader shape from the §Input Validation manifests row (`coverage_gate`, `scenario_audit_gate`): workspace root from `CARGO_MANIFEST_DIR`, hard-coded relative path, hard fault on absent or malformed, never defaulted.

## Anti-patterns to avoid
- Don't let compaction reword a registered fact that the security plan relies on into something weaker. Examples: "no secrets/cloud env vars", "No inbound listener of Conductor's own", the per-reader registrations. The scope says no fact is re-decided; for security, a quietly lost negative ("no X") is a quiet widening of the attack surface (per security-plan §Security Anti-Patterns → Universal, → Secrets).
- Don't give the measurement reader, if one ships, a `CONDUCTOR_*` override handle, a path in its failure text, or a shell-string spawn (per security-plan §Input Validation manifests row; §Security Anti-Patterns → Code Patterns rule (b), → Logging).

## Contract bindings
- security ↔ architecture: the security plan's section-name citations of Occupied Resources (Environment variables, Ports, On-disk artifacts, Interface routes) and Established Decisions (Run-History Persistence, Deployment, CI/CD, MCP Read-Back Client) must still resolve after compaction. The cascade sweep is the check.
- security ↔ drift-base: the CARRY `D-arch-collision` detector (a second owner of an already-registered resource) would also catch a second claimant on `:4317` / `4444` / `4445`. That supports the "single deliberate port bind" invariant (security-plan §Threat Model Summary → Attack surface; §Security Anti-Patterns → API). The proposal text should include Ports in its subject set.
- security ↔ tests: if the measurement lands as a test binary, its reader record binds to test-plan's gate tier, in the same way `coverage_gate` / `scenario_audit_gate` did.

## Acceptance criteria contributions
- Every section-name reference that security-plan.md makes into arch Occupied Resources / Established Decisions still resolves to a label in the compacted body. Check by grepping each cited sub-label (per security-plan §Threat Model Summary; §Secret Management → Storage).
- The compacted Occupied Resources still registers `:4317` (deliberate fault bind), `:4318` (unused), `4444`/`4445` (harness-only), the CI-only msedgedriver egress, and every `CONDUCTOR_*` / `ANDROMEDA_PULSE_*` handle listed in the security plan. Check with a before/after handle-and-port set diff; the result must be empty (per security-plan §Threat Model Summary → Attack surface / Networking; §Secret Management → Storage).
- Nothing added to `architecture.md`, `architecture-amendments.md` or any committed measurement output contains an absolute host path or a secret-shaped string. Check with a host-path hygiene grep over the diff (per security-plan §Security Anti-Patterns → Logging, → Secrets).
- If a measurement test binary ships, it resolves `architecture.md` from `CARGO_MANIFEST_DIR` with no `CONDUCTOR_*` handle, and its failure text carries no path. The security-plan §Input Validation manifests row either records it or the wrap records why it does not qualify (per security-plan §Input Validation, per-READER clause).

## Relevant amendment history
- 2026-09-16-scenario-assertion-audit-gate: added the second test-binary reader of a committed artifact, and the operator ruled that "a committed artifact parsed at runtime earns its row whether or not a shipped binary is the one parsing it". This is the precedent if the size measurement becomes a Rust test. Its cascade sweep also cited `architecture.md:60`, the [CI/CD] Established Decisions line this chunk targets, as amended in that pass. That line is where security-relevant reader facts have been added before.
- 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate: the orchestrator's cross-master sweep, keyed on `architecture.md:37`, reconciled the security Threat Model CI/CD bullet. It also held that a plain fixed-toolchain `run:` step is not a new rule-(b) spawn form, which settles the class a CI measurement step would fall into.
- 2026-08-10-workspace-key-divergence-probe (keychain half): a security-plan bullet stayed stale because the arch correction it cited was never reconciled across masters. This is the cascade-step-2 lesson that applies here: when arch text moves, sweep the citing masters in the same pass.
- 2026-09-23-real-model-capture-path-handles-guarded-...: the most recent change in the env-handle area, which closed the `CONDUCTOR_RUNS_DIR` / `ANDROMEDA_PULSE_DATA_DIR` test-reader residuals. The matching arch Environment-variables registrations are among the facts compaction must keep in current-truth form, without the dated residual narrative.
