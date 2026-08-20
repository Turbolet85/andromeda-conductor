# Codebase Research — 2026-08-20-read-back-seam-survivors-closed

## Scope
- **Depth:** deep · **Reads:** 15 · **Globs/Greps:** ~10 · **Code-graph queries:** 2 (42 rows impact, 3 rows crate-edges)

## Files inspected
- `crates/conductor-verify/src/jsonrpc.rs` (full, 1-110) — `JsonRpcSession { writer, reader, next_id }`; `next_id: 1` at `:33`; `request` takes `id = next_id` then `next_id += 1` (`:40-41`); the read loop **skips messages whose `id` differs** (`:49-51`, comment "Skip notifications / unrelated ids"); `notify` writes an id-less line (`:64-66`); `read_line` returns a typed `VerifyError::Transport(UnexpectedEof)` on stream close (`:104-107`) — **a correlation failure ends as a typed error, not a hang, once the peer closes**.
- `crates/conductor-verify/src/client.rs` (60-132) — `connect_transport` is `pub` (`:71`) and splits the transport into a `JsonRpcSession`; `initialize` runs `request("initialize")` (`:88`) then `let _ = session.notify("notifications/initialized", json!({}))` (`:93`) — **result discarded by design**; `list_tools` (`:109`) and `call_tool` (`:127`) are the other two `request` sites.
- `crates/conductor-verify/src/preflight.rs` (76-96, 150-175, 318-400) — `CanaryPoll { attempts, interval }` has **public fields** (`:80-84`) plus `immediate()`; `run_preflight` is `pub` (`:155`); `poll_canary` builds one `ShapeWitness` **outside** the attempt loop (`:328`) and calls `assert_canary` per attempt (`:330`); `assert_canary` calls `witness.list(&list)` (`:381`) **before** the empty/stale branches, so every answering call witnesses.
- `crates/conductor-verify/src/extract.rs` (214-226) — `log_observed_keys` is a single `tracing::info!("read-back shape: {tool} returned keys [{}]", …)`; the payload rides the format string, i.e. the `message` field.
- `crates/conductor-core/src/redact.rs` (15-70) — `ALLOWLISTED_FIELDS` **contains `message`, `target`, `level`, `timestamp_ms`, `run_id`** and the service triple; a non-listed field name is dropped at the processor stage.
- `crates/conductor-core/src/obs.rs` (63-83) — `ObsSink::{Stderr, File(PathBuf)}`; `init_observability` builds an `EnvFilter` whose **default directive is INFO** and installs via `tracing::subscriber::set_global_default` — process-global, first-install-wins, infallible.
- `crates/conductor-verify/tests/common/mod.rs` (full, 156) — `StubConfig` (11 knobs, `Default`) + `serve_stub`; file carries `#![allow(dead_code)]`, so additive knobs/helpers are the established extension shape. **Two structural facts decide this chunk** (below).
- `crates/conductor-verify/tests/preflight.rs` (1-70, 228-270, + test index) — `drive_with` (duplex + `serve_stub` + `connect_transport` + `run_preflight`); 15 existing `#[tokio::test(flavor = "current_thread")]` legs; `the_poll_loop_sleeps_between_attempts_but_not_after_the_last` (`:228-266`) already drives `CanaryPoll { attempts: 3, interval }` under `start_paused`.
- `crates/conductor-verify/tests/jsonrpc_line_bound.rs` (1-50) — the duplex seam precedent; its `serve_line` comment names the drained "best-effort `notifications/initialized`".
- `crates/conductor-run/tests/canary_obs_witness.rs` (full) — the worked self-obs witness: `init_observability(..., ObsSink::File(tmp))`, read the file, filter lines by a test-owned `run_id`, assert count + per-line base fields. Its module doc states the process-global reason it is alone in its binary.
- `crates/conductor-verify/Cargo.toml` · `crates/conductor-run/Cargo.toml` · root `Cargo.toml` — dev-dep surfaces (below).
- `.andromeda/runs/2026-08-20T18-06-29-code-audit/c-mutation-conductor-verify.json` — 134 mutants / 81 caught / 28 missed / 2 timeout; the five target coordinates confirmed verbatim.
- `conductor-0.2.0/chunks/2026-08-20-verifier-self-hardening/report.md` — the scoped re-run tally (`:26`) reconciling the five.

## Graph impact (code-graph query, `rows: 42`, `db_state: fresh`)
- **`JsonRpcSession::request`** — 3 callers, all in-crate: `client.rs:88` (initialize) / `:109` (list_tools) / `:127` (call_tool). Every read-back call funnels through the mutated counter.
- **`JsonRpcSession::notify`** — **exactly one caller**, `client.rs:92` (0-indexed; `:93` in file terms). Confirms the discarded-result reading: there is no second site where a return value could be observed.
- **`log_observed_keys`** — **four** call sites: `extract.rs:83`, `:95`, `:110` and `preflight.rs:357` (0-indexed; `:358` in file terms). A count-based witness assertion **must scope by the `query_incident_list` tool name**, or the three extraction sites can contaminate it.
- **`ShapeWitness::list`** — one call site, `preflight.rs:380` (0-indexed; `:381` in file terms), inside `assert_canary`.
- **`ReadbackClient::connect_transport`** — 7 test call sites across `tests/jsonrpc_line_bound.rs` (2), `tests/preflight.rs` (2), `tests/readback.rs` (3). The seam is established; no widening needed.
- **Crate edges** (`rows: 3`) — outbound `conductor-verify → conductor-core`; inbound from `conductor-cli` and `conductor-run`. Test-only changes here have **zero cross-crate blast radius**.
- Note: graph lines are **0-indexed**; cargo-mutants coordinates are 1-indexed. Both reconcile at every site above.

## Patterns detected
- **Echoing duplex stub** (`tests/common/mod.rs:145`): the stub replies `json!({"jsonrpc":"2.0","id": id, "result": result})` using the id it *received*. **This is why both id-counter mutants survive** — an echo stub answers `1,0,-1…` and `1,1,1…` exactly as agreeably as `1,2,3…`, so correlation never breaks and no existing assertion can see the difference.
- **Notifications silently dropped** (`tests/common/mod.rs:82-84`): `let Some(id) = req.get("id").cloned() else { continue; }`. The `notifications/initialized` line reaches the stub and is discarded unobserved — the reason the `notify` mutant survives despite the line being on the wire.
- **Self-obs witness shape** (`conductor-run/tests/canary_obs_witness.rs`): install `init_observability` with a `File` sink into a temp dir, read the artifact, filter by a test-owned `run_id`, assert line count and base-field presence. One test per binary.
- **Multi-attempt canary drive** (`tests/preflight.rs:228-266`): `StubConfig { canary_in_corpus: false, .. }` never goes fresh, so the loop spends its full attempt budget while `query_incident_list` **answers** every time — exactly the N-answering-attempts shape the ShapeWitness count needs.
- **Additive stub extension** (`tests/common/mod.rs:5`, `#![allow(dead_code)]`): knobs are added to `StubConfig` + `Default` without disturbing existing call sites.

## Conventions to follow
- **Public-seam assertions only** — drive through `ReadbackClient::connect_transport` (`client.rs:71`); `JsonRpcSession` is `pub(crate)` and stays so (`jsonrpc.rs:22`).
- **`#[tokio::test(flavor = "current_thread")]`** for every async leg (all 15 legs in `tests/preflight.rs`); `start_paused` only when virtual time is the assertion (`:228`).
- **Typed-error vocabulary** — `VerifyError::{Transport,Decode,JsonRpc,Protocol}` as values; `jsonrpc_line_bound.rs` is the precedent for asserting on the variant.
- **Mirror-the-constant discipline** (`jsonrpc_line_bound.rs:13-14`): when a test pins a crate-private value, mirror it with a comment saying a drift is the point.
- **Deterministic intervals** — `interval: Duration::ZERO` keeps a multi-attempt poll clock-free (`CanaryPoll` fields are public).

## Load-bearing mechanism (the equality the design needs)
**ShapeWitness count separation at 3 answering attempts** — correct code emits **1** witness line, `!`-deleted emits **2** (attempts 2 and 3), `-> ()` emits **0**. Verified from `poll_canary` constructing one witness outside the loop (`:328`) and `assert_canary` witnessing before the branch that decides retry (`:381`), with `canary_in_corpus: false` guaranteeing all three attempts answer. A single `assert_eq!(count, 1)` therefore kills both — **at two attempts it would not** (correct = 1, `!`-deleted = 1).

**Id-counter observability** — because the stub echoes, correlation cannot break by itself; the difference between `+=`, `-=` and `*=` is observable **only as the id values on the wire**. So the primary kill is a stub that RECORDS the received ids and a test asserting the sequence is strictly increasing and repeat-free (`-=` gives `1,0,-1`; `*=` gives `1,1,1`; both fail). A **decoy** line (a stale `id:1` response emitted before the real answer to request 2) additionally makes correlation the carrier and exercises the skip-loop at `jsonrpc.rs:49-51` — it kills `*=` but **not** `-=`, so it is a complement to the sequence assertion, never a replacement.

## Seam facts (dependency visibility)
- `conductor-core` is a **normal** dependency of `conductor-verify` — `init_observability` / `ObsSink` / `redact_value` are reachable from tests with no manifest change.
- `conductor-verify` dev-deps today: `tokio` (rt, macros, io-util, time, test-util), `serde_json`, `rstest`. **`assert_fs` is absent.**
- `assert_fs = "1"` **is** in root `[workspace.dependencies]` (`Cargo.toml:83`) and already consumed by `conductor-run`. Adding `assert_fs.workspace = true` is a **dev-edge line with no new package node** — the compact PROBE-AUTO-SATISFY form of the carried audit PREREQ survives.
- No production `pub` surface changes; nothing is re-exported.

## New files to create
- `crates/conductor-verify/tests/readback_shape_witness.rs` — the ShapeWitness one-shot witness, **alone in its binary** (test-plan §11): installs `init_observability` with a `File` sink, drives a 3-attempt `run_preflight` over the duplex stub, asserts exactly one `query_incident_list`-scoped witness line carrying the §3 base fields.

## Files to modify
- `crates/conductor-verify/tests/common/mod.rs` — additive: a recording variant of `serve_stub` (or a recorder handle on `StubConfig`) capturing (a) each request's `id` in arrival order and (b) each id-less notification line's `method`. Additive so the 7 existing `connect_transport` call sites and 15 preflight legs are untouched.
- `crates/conductor-verify/tests/jsonrpc_line_bound.rs` **or** a sibling — the id-sequence + `notifications/initialized` wire assertions (the file already owns the "assert the wire through the public seam" idiom and already drains the notify line).
- `crates/conductor-verify/Cargo.toml` — `[dev-dependencies]` gains `assert_fs.workspace = true` **only if** the witness uses the precedent's temp-dir helper.
- `Cargo.lock` — one dev-edge line if the above lands (no package added).

## Open questions
- Does adding the `assert_fs` dev-edge leave the resolved package set byte-identical (no feature unification pulling a node)? → blocks: **implementation-scope** — it decides whether the carried audit PREREQ stays in compact PROBE-AUTO-SATISFY form or restores the FULL form. Resolvable in one `cargo metadata` / lock diff at implement time; the fallback (no dev-dep, `std::env::temp_dir()` + unique subdir) keeps the compact form unconditionally.
- Do the id-sequence assertions live in the existing `jsonrpc_line_bound.rs` binary or a new sibling binary? → blocks: **implementation-scope** — they do NOT capture the self-obs subscriber, so test-plan §11 does not force isolation; only file cohesion decides, and the file list above is provisional on that call.
