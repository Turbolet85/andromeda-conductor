# security extract

## Relevance
partial — the change is test-only (`#[cfg(test)]` `InvokeRequest` origin fixtures), but the value it changes is the IPC ORIGIN, which the plan treats as the security-relevant input of the Tauri IPC trust boundary; there are no new inputs, dependencies, secrets or artifacts.

## Constraints
- The Tauri IPC vector is an in-process boundary between the backend and the bundled webview (per security-plan §Threat Model Summary → Attack surface, "Tauri IPC"). The tests here pin how that boundary admits a dispatch. Their origin must be the webview's REAL origin on each host. A fix that stops the origin check from mattering removes the evidence that the boundary rejects a wrong origin. Scope §Boundaries already bans that, and this domain grounds the ban.
- The deny-by-default capabilities file must allow ONLY the actual commands plus the one live-counter `Channel` (per security-plan §Security Anti-Patterns → Code Patterns, "NEVER ship Tauri commands without a minimal capabilities file"). Fixing the six tests by widening a capability, a `remote`/URL scope or an allowed-origin list to admit both hosts' origins is out of bounds. `capabilities/*.json` and `tauri.conf.json` stay byte-unchanged.
- The `tauri` ≥ 2.10.3 floor exists for origin confusion (CVE-2026-42184, a Windows custom-protocol origin issue) (per security-plan §Dependency Security → Update policy; §Security Anti-Patterns → Universal, standing floors). The per-host origin the fix selects must be the origin Tauri's own runtime uses on that platform. It must not be a value that happens to pass the mock. Which origin the Linux mock webview actually carries, and whether the literal is the only host-dependent input, are research's questions (P3).
- No dependency delta: `Cargo.lock` stays committed and un-drifted (per security-plan §Security Anti-Patterns → Universal, "NEVER let `Cargo.lock` drift"). If the fix reaches for a helper crate or a new feature flag to derive the origin, that is a dependency change and needs the full audit/deny gate. Scope already excludes it.
- The change must not alter how the error edge sanitizes output (per security-plan §Error Handling, External responses). Only `#[cfg(test)]` code moves, so no `#[tauri::command]` return path, `anyhow` edge or run-report artifact should change. Anything that does is outside scope.
- No secret-shaped string may enter the tree (per security-plan §Secret Management, "Secret scanning in CI"; §Security Anti-Patterns → Secrets). A URL-scheme origin literal is not secret-shaped, but the `rust` job's `Secret-scan gate` still has to stay green over the change.

## Patterns to follow
- Use the established per-OS split. Host-specific behaviour is selected at compile time with `#[cfg(windows)]` / `cfg!` (per security-plan §Security Anti-Patterns → Code Patterns, the `console_suppressing_flags()` / `#[cfg(windows)]` precedent in `build_command`). One host-correct source selects the origin, and neither host's test is skipped or excluded.
- App-defined `#[tauri::command]`s are not gated by the capability ACL; the real boundary is input validation inside the command (per `.claude/rules/security.md` Session Additions 2026-06-26). So the IPC failure on the wrong origin should be read as Tauri's runtime origin check, not as the capability file. Whether that is the actual mechanism behind `Plugin not found` is P3's question.
- Keep the doc comment at `commands.rs:410` truthful per host. It is the in-tree record of the origin invariant the IPC boundary relies on (per security-plan §Threat Model Summary → Tauri IPC trust boundary).

## Anti-patterns to avoid
- NEVER widen the deny-by-default capabilities, or admit an arbitrary/remote origin, to make the IPC tests pass (per security-plan §Security Anti-Patterns → Code Patterns, minimal capabilities file; remote-origin iframe / CVE-2026-42184 origin-confusion ban).
- NEVER let a Cargo.lock or dependency change ride in with a test-fixture fix (per security-plan §Security Anti-Patterns → Universal, "NEVER let `Cargo.lock` drift").

## Contract bindings
- security ↔ tests: the origin-rejection property is a security invariant, and the mock-runtime tests are its only executable witness (per security-plan §Threat Model Summary → Tauri IPC). The tests plan owns the test bodies. The `.claude/rules/testing.md` mock-origin rule, which says Windows-only, moves at wrap curation (scope.md Folded freight).
- security ↔ tests §CI: CI stays `windows-latest`. The supply-chain and secret-scan gates there must stay green over the change (per security-plan §Threat Model Summary → Infrastructure, CI/CD).

## Acceptance criteria contributions
- `git diff --stat` over the chunk touches no `crates/conductor-tauri/capabilities/*.json`, no `tauri.conf.json`, no `Cargo.toml` and no `Cargo.lock` (per security-plan §Security Anti-Patterns → Code Patterns, minimal capabilities file; §Universal, Cargo.lock un-drifted).
- After the fix, at least one existing or added assertion still shows a dispatch with a non-matching origin FAILS on the host running it, so the origin check is not made irrelevant (per security-plan §Threat Model Summary → Attack surface, Tauri IPC trust boundary).
- `cargo audit` and `cargo deny check` each exit 0, with the exit status captured before any pipe, over the unchanged lock (per security-plan §Security Anti-Patterns → Universal, "NEVER … merge … without `cargo-audit` (and recommended `cargo-deny`) green").
- The `secret_scan_gate` target (`crates/conductor-core/tests/secret_scan_gate.rs`) passes in the workspace nextest run (per security-plan §Secret Management, "Secret scanning in CI").
