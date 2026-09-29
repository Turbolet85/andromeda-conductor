# obs extract

## Relevance
partial — the chunk adds no instrumentation, span, metric or log, but it edits root `Cargo.toml` `[workspace.package]` and the UI `package.json`, which are the compile-time and manifest sources of self-obs service identity. So obs contributes only a non-regression guard.

## Constraints
- The obs tier is Minimal (0), per obs-plan §1 Obs Scope Summary. A license-only chunk owes no new spans, metrics or log events, and none should be added to "cover" it.
- obs-plan §3 Service identity requires `service.version` to be compile-time `env!("CARGO_PKG_VERSION")` from root `Cargo.toml`. The members inherit it through `[workspace.package]`, the same table this chunk edits. Adding `license` there must leave `version` (and the members' `version.workspace = true` inheritance) byte-unchanged. Whether the edit leaves them intact is a diff check for P3 research and implement, not something this extract can attest.
- obs-plan §3 Service identity and §11 Universal ("NEVER hardcode service identity") require `service.name` via `env!("CARGO_PKG_NAME")` plus the `$CONDUCTOR_SERVICE_NAME` override. The `[package] name` keys in the nine member manifests stay untouched when `license.workspace = true` is added.
- obs-plan §2 Telemetry Strategy (Frontend row) says the `conductor-ui` version comes from a "window global or manifest". Editing `crates/conductor-tauri/ui/package.json` must not alter its `name`/`version` fields. Only the `license` key is in scope. Whether the UI actually reads its version from `package.json` is research's question.
- obs-plan §9 CI Integration (Log conformance check) requires every `logs/agent-latest.jsonl` line to carry `service.name` / `service.version` / `deployment.environment`. The chunk must leave that gate's input unchanged, so no identity field may move.

## Patterns to follow
- Workspace inheritance (`*.workspace = true`) is the existing single-source pattern behind `service.version` (per obs-plan §3 Service identity: "from root `Cargo.toml`"). Setting `license` the same way keeps one source of truth for the workspace's package metadata.
- A repository-hygiene result is reported in the job log only and never written into a telemetry artifact (per obs-plan §9 CI Integration, Repository-hygiene gates row). Any `cargo deny check licenses` output this chunk relies on stays a CI or stderr signal, never a `logs/`/`runs/` record.

## Anti-patterns to avoid
- Hardcoding a version or name string into a manifest or source file as a side edit while touching manifests (per obs-plan §11 Universal, "NEVER hardcode service identity").
- Writing supply-chain or license-gate output into `logs/agent-latest.jsonl` or `runs/**` (per obs-plan §9 CI Integration, the fmt-row / repository-hygiene discipline: job log only).

## Contract bindings
- obs ↔ tests: the §9 log-conformance gate asserts `service.version` on every self-obs line (per obs-plan §9 CI Integration, Log conformance check). A manifest edit that disturbed version inheritance would surface there, so the existing gate is the binding and no new one is needed.
- obs ↔ security: `cargo deny check licenses` is security's supply-chain gate. Obs only requires that its output stay in the job log (per obs-plan §9 CI Integration, Supply-chain audit report row: "inline in CI logs").

## Acceptance criteria contributions
- (obs) Root `Cargo.toml` `[workspace.package]` `version` and every member's `version.workspace = true` are unchanged by the diff, so `service.version` resolves as before (per obs-plan §3 Service identity).
- (obs) No member `[package] name` and no `package.json` `name`/`version` field changes. Only `license` keys are added (per obs-plan §3 Service identity; obs-plan §2 Telemetry Strategy).
- (obs) The chunk adds no `tracing` span, event or field, and no metric (per obs-plan §1 Obs Scope Summary, Minimal tier).
