# Scope — Host-portable Tauri IPC tests

**Marker:** `2026-10-04-host-portable-tauri-ipc-tests` · **Version:** conductor-0.3.0 · **Epoch:** Epoch 5 — Polish & ship

## Working entry (verbatim intent)
Host-portable Tauri IPC tests — the six `conductor-tauri` mock-runtime tests pass on the Linux dev host as they do on
Windows.

## What this chunk builds
- The `conductor-tauri` mock-runtime IPC tests dispatch through an origin that is correct on the HOST they run on, so
  the same test source passes on the Linux dev host and on Windows (CI `windows-latest`), with no test skipped,
  `#[ignore]`d or `cfg`-excluded on either host to get there.
- The two literal sites that build an `InvokeRequest` with `url: "http://tauri.localhost"` —
  `crates/conductor-tauri/src/commands.rs:417` (the shared `request()` helper) and `crates/conductor-tauri/src/pause.rs:243`
  (an inline construction in the hold-gate dispatch test) — take their origin from one host-correct source rather
  than a Windows-only literal. Re-verified at HEAD `f5076ad`: `grep -rn 'tauri.localhost' crates/ --include=*.rs`
  returns exactly these two sites plus the doc comment at `commands.rs:410`.
- The error-expecting dispatch helper refuses an ACL denial as the error it expects, so a wrong origin can never make
  its callers pass vacuously (added at P3's premise closure — see Folded freight).
- The doc comment at `commands.rs:410` ("The `url` MUST be the mock webview's real origin (`http://tauri.localhost`) —
  any other value fails dispatch") is corrected to state the host-dependent truth, not the Windows value alone.

## Boundaries
- TEST code only: both sites sit inside `#[cfg(test)]` modules. No production command, no capability file
  (`capabilities/*.json`), no `tauri.conf.json`, no frontend, no dependency change.
- No new CI job: CI stays `windows-latest` (this host has no Linux runner obligation); the Linux leg is this dev host.
- The fix must not weaken what the tests assert: a dispatch that would fail on a wrong origin must still fail — the
  origin is made CORRECT per host, never made irrelevant (e.g. by widening a capability to admit any origin).

## Acceptance outcome
- `cargo nextest run --workspace --profile ci` (the wrap's gate 27) is green on this Linux dev host — the six
  `conductor-tauri` failures gone, nothing else regressed.
- CI on `windows-latest` stays green over the change (the operator pass's push).

## Folded freight (working-route.md:85)
- **EVIDENCE** (folded as hypothesis, coordinates re-verified above): the mock-webview origin `http://tauri.localhost`
  is hardcoded at `crates/conductor-tauri/src/commands.rs:417` and `pause.rs:243`; on the Linux dev host six tests
  fail with `Plugin not found`, identically at base `6a9ff7c` (measured at
  `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/report.md`, gate 27 — CI on Windows stays green).
  Re-verified: that report's lines 41 and 86 carry the measurement (12 failing lines each side of the base control).
- **CONTEXT:** founder ruling 2026-10-04, relayed by the overseer — its own entry, before the Version close.
- Causal claim carried in the freight, kept verbatim: "the mock-webview origin `http://tauri.localhost` is
  hardcoded … on the Linux dev host six tests fail with `Plugin not found`" — i.e. the origin literal is the CAUSE of
  the six failures. VERIFIED at P3 (research.md §The mechanism): `is_local_url` compares the request URL against
  `tauri_protocol_url`, which is `http://tauri.localhost` only under `cfg!(windows) || cfg!(target_os = "android")`
  and `tauri://localhost` otherwise; a non-local origin resolves Remote and the ACL refuses with
  `"<cmd> not allowed. Plugin not found"`. The literal is the ONLY host-dependent input (the mock config carries no
  dev/dist URL).
- [premise-corrected: P3 run on this host — 6 failing + 3 passing vacuously through the same `request()`] The literal
  reaches NINE tests, not six. The six that fail are named in research.md (5 in `commands.rs`, 1 in `pause.rs`). Three
  more — the `invoke_expecting_error` callers (`run_report_errors_…`, `run_envelope_errors_…`,
  `list_scenarios_errors_…`) — assert only a non-empty error, so on Linux the ACL denial satisfies them and they PASS
  without the command running. In scope: the error-expecting path must refuse an ACL denial as its error, so a wrong
  origin can never again make those mutation killers pass vacuously.
- `.claude/rules/testing.md:65` carries the 2026-06-27 rule the doc comment cites, corrected at the last wrap to say
  the literal is right on Windows only; it also still states "any other value (e.g. `tauri://localhost`) fails
  dispatch", which is false on Linux. VERIFIED present; its update is a wrap curation item, not this chunk's code.

## CI verdict read at Setup (5a)
- `f5076ad` (the last wrap's flip, = HEAD): **verdict not yet available** at Setup — CI#37165680435 `in_progress`,
  checks 3/3 open, oldest (A11y gate) at 111 s when read. Not a red, not a green; recorded as read.
- **Updated at P5 (2026-10-04): green.** The overseer measured CI#37165680435 completed success (relayed with the
  founder's approval), and a re-read via `ci.py conclusion --sha f5076ad9…` confirmed it: `verdict: green · checks 3/3 ·
  wall 682 s · CI#37165680435 completed/success`. Nothing to fold.

## Matrix
- The version matrix reads verified 10/11 · deferred 1 · unclaimed 0 — this chunk claims no capability (it repairs
  a host-dependent test fixture). VERIFIED: `matrix.py show --dir conductor-0.3.0 --unclaimed` → `unclaimed 0 of 11`.
