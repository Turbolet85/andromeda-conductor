# security extract

## Relevance
partial — Logging infrastructure adds log emission (Security Plan §Logging & Monitoring is SKIP at Minimal tier), but the chunk does not itself handle sensitive data or auth; however, it establishes the foundation for future redaction (next chunk), and it introduces new dependencies requiring audit-gate compliance.

## Constraints
1. All new dependencies (`tracing` 0.1.44, `tracing-subscriber` 0.3.23) must pass `cargo-audit` ≥0.22 / `cargo-deny` ≥0.19 gates before merge (per §Dependency Security; `Cargo.lock` must remain committed and un-drifted).
2. Panic hook MUST route ALL panics into the structured log stream, never allowing an unlogged crash (Obs invariant: zero-unlogged-panics). `std::panic::set_hook` capture is mandatory (scope.md Definition of Done).
3. Service-identity fields (`service.name`, version) MUST be stamped on every JSON log line; `run_id` MUST be present on every line as the cross-cutting correlation key (Obs-plan §3/§6; scope.md Service-identity fields).
4. JSON log output is STDOUT/FILE ONLY — no OTel SDK integration (scope.md: "Self-observation ONLY … uses NO OTel SDK"). The product OTLP fault stream to Pulse `127.0.0.1:4317` remains separate (Epoch 3).
5. Init surface MUST be callable identically from `conductor-cli` (release gate) and optional `conductor-tauri` (scope.md: runtime-agnostic entrypoint); exact crate/module home TBD by codebase research.

## Patterns to follow
1. Use `tracing-subscriber` JSON feature for machine-parseable self-obs ground truth (scope.md: agent-parseable log stream, obs-plan JSON schema).
2. Thread `run_id` as a context field through the tracing span/event so all lines for a run carry it without manual injection (observability cross-cutting).
3. Keep redaction (field-allowlist, path/struct-name sanitization) in the NEXT chunk — this chunk builds the seam for redaction to attach to, no redaction logic here.

## Anti-patterns to avoid
1. NEVER ship Tauri commands without deny-by-default capabilities (§Security Anti-Patterns § Code Patterns) — applies to any IPC surface the logging stack may observe.
2. NEVER log secrets, tokens, or `Authorization` headers (§Logging ban); this chunk establishes the non-secret path; redaction enforcement is next chunk.
3. NEVER use tokio's virtual clock for journal/report timestamps — use `std::time::SystemTime`/`Instant` so ground-truth artifact math stays correct (§Logging ban; scope.md notes this is distinct from self-obs timestamps).

## Contract bindings
- **obs ↔ logging:** Obs-plan §3/§6 define the JSON schema; this chunk materializes it. Redaction (PII/path scrubbing) binds to the next chunk (Log + error-boundary redaction).
- **tests ↔ CI gate:** cargo-audit/deny must pass before merge (§Dependency Security § CI integration); existing GitHub Actions pipeline extended with `cargo install cargo-audit && cargo audit` + optional `cargo deny check`.

## Acceptance criteria contributions
- (security) `cargo audit` passes on the new `tracing` + `tracing-subscriber` 0.1.44 / 0.3.23 dependencies (no high/critical advisories).
- (security) `cargo deny check advisories bans sources licenses` passes (optional, but recommended for supply-chain completeness).
- (logging) All panics are captured via `std::panic::set_hook` and routed into the JSON stream, never unlogged (zero-unlogged-panics invariant).
- (logging) Service-identity fields + `run_id` present on every JSON line (obs-plan §3/§6 contract).

## Relevant amendment history
**2026-06-15-dependency-audit-gate** (§Dependency Security): audit-tool versions (cargo-audit / cargo-deny) reframed as **minimum floors** rather than strict pins, since they are external CLI tools (not `Cargo.lock`-pinnable) and the advisory DB is fetched fresh each run. Toolchain confirmed bumped to ≥1.94.1 (tar-rs CVE cleared). **Rationale:** mirror the architecture's downgrade flexibility while keeping the gate firm. This chunk's new tracing dependencies must clear the same floor; tauri ≥2.10.3 (origin-confusion CVE-2026-42184) remains a forward "required bump" deferred to the Tauri GUI chunk (Epoch 9).