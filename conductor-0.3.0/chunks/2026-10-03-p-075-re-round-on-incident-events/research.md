# Codebase Research — 2026-10-03-p-075-re-round-on-incident-events

## Scope
- **Depth:** deep. This is a replan of the 2026-10-02 promotion rolled back at P5, at the SAME Conductor HEAD
  `6a9ff7c` (`git log -1 --format=%H HEAD` → `6a9ff7cd…a156`). The cancelled research
  (`.andromeda/runs/2026-10-02T22-27-37-phase/cancelled-research.md`) was the starting point. Every count and site it
  stated was RE-RUN here and never copied (re-derived commands below). Three things are new: Pulse at S2, the Linux
  host, and the code graph being unavailable here.
  - **Reads:** 19.
  - **Globs/Greps:** 16.
  - **Graph queries:** 0 — see §Graph impact (`derived-without-graph`).
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: read in full, its 33 Session Additions included. It auto-loaded whole
    with this phase's read of `crates/**/tests/**`.
  - `.claude/rules/testing.md`: read in full, 49 Session Additions. It auto-loaded whole the same way.
  - `.claude/rules/observability.md`: read in full.
  - `.claude/rules/host-win32.md` and `.claude/rules/security.md`: always loaded.
  - Applied below:
    - the firing-form env block (now Linux-shaped);
    - no `boot` before a preflight-firing leg;
    - 150 s quiet windows;
    - the pre-leg line count;
    - a freeze per leg;
    - the census and stop form;
    - printed-verdict atoms read from a recorded output;
    - the digest pin;
    - the stub item-key fidelity rule;
    - buffered capture output;
    - the 2026-10-02 rule to read a gating predicate's definition.
- **Platform issues consulted:** none. There is no runner-only bullet: Setup 5a read CI#37037859266 green 3/3 on
  `6a9ff7c`.

## Host facts (measured at phase time, 2026-10-03T22:2xZ — the Linux host, `Linux 7.2.5-3-omarchy`)
- **Toolchain:** `rust-toolchain.toml` channel `1.95.0`; `cargo 1.95.0` and `rustc 1.95.0` resolve, satisfying the
  ≥ 1.94.1 floor.
  - Present: `cargo-nextest 0.9.146`, `cargo-deny 0.20.2`, `cargo-llvm-cov`.
  - **`cargo-audit` is ABSENT** (`cargo audit --version` → `error: no such command: audit`). The stack's version is
    `cargo-audit 0.22.2` (`.claude/docs/stack.md:42`).
  - **The local advisory-db is absent** (`~/.cargo/advisory-db` does not exist), so `cargo-audit`'s first run clones it
    cold.
  - **`CARGO_HOME` is unset.** The prior plan's porcelain probe `git -C "$CARGO_HOME/advisory-db" status --porcelain`
    would therefore read `/advisory-db`; the Linux form is `${CARGO_HOME:-$HOME/.cargo}/advisory-db`.
- **No build exists here.** There is no `target/` and no `crates/conductor-tauri/ui/{node_modules,dist}`.
  - `conductor-tauri`'s `generate_context!` resolves `frontendDist` (`ui/dist`, `tauri.conf.json:7`) at COMPILE time.
    Every `--workspace` cargo gate therefore needs `npm ci && npm run build` in `crates/conductor-tauri/ui` first.
    `scripts/agent-run.sh:52-56` (`ensure_frontend`) and `ci.yml:49-55` both do exactly this before their cargo steps.
  - Per-crate gates (`-p conductor-verify`, `-p conductor-run`) do not compile `conductor-tauri`.
  - WebKitGTK system libraries are present (`pkg-config`: `webkit2gtk-4.1` 2.52.6, `gtk+-3.0` 3.24.52,
    `libsoup-3.0` 3.6.6). The Tauri crate should link once the bundle exists. **[unmeasured: no Conductor build has
    run on this host]**
- **Missing pieces of the code graph:** `scip-typescript` is not on `PATH`; python `duckdb` and `protobuf` are not
  importable (health check 11); `.andromeda/cache/` is absent (gitignored, fresh checkout).
- **Environment:** `TEMP` and `TMPDIR` are both unset; `XDG_CONFIG_HOME=~/.config`. Wayland (`wayland-1`) and
  XWayland (`:0`) are present. `grep` in the Bash tool's shell is a function: `ugrep`, which refused one complex
  alternation with "exceeds complexity limits".
- **`scripts/agent-run.sh` is mode 100644** (`git ls-files -s`), so `test -x` fails. Invoke it as
  `bash scripts/agent-run.sh`, which the prior round already did.
- **CI is Windows-only:** `.github/workflows/ci.yml:18` and `:245` are `runs-on: windows-latest`; `:289` is
  `windows-2022`.

## Files inspected
- `contracts/mcp-contract.toml` (`:11-16`) — `required_tools` holds 4 names (re-read).
- `crates/conductor-verify/src/manifest.rs` (`:17`, `:53-64`, `:83-110`):
  - `READBACK_TOOLS: [&str; 4]`;
  - `validate()` rejects an empty entry and any pinned tool missing from the manifest (superset check);
  - unit tests read the committed file and reject a dropped pinned tool.
- `crates/conductor-verify/src/preflight.rs` (`:171-226`):
  - every manifest name is mapped to `Present`/`Absent` against `tools/list`;
  - any absent name sets the existing `required tool(s) absent` precondition, ahead of the run-contract and canary
    arms.
  - Pinning a fifth name therefore blocks a sidecar lacking it on that EXISTING arm; the named-precondition count
    stays five.
- `crates/conductor-verify/src/client.rs` (`:22-26`, `:137-189`):
  - four tool-name consts;
  - `call_tool` is `#[tracing::instrument(name = "verify.readback.call_tool", …, fields(mcp_tool = %name))]`;
  - `resolve_incident(i64)` is the one-field convenience precedent.
- `crates/conductor-verify/src/lib.rs` (`:44`, `:50`) — re-exports the consts and `READBACK_TOOLS`.
- `crates/conductor-verify/src/spawn.rs` (`:33-159`), the Linux behaviour of the shipped spawn path:
  - `build_command` applies `CREATE_NO_WINDOW` only under `#[cfg(windows)]` (`:99-100`);
  - `platform_default` takes `$XDG_CONFIG_HOME/andromeda-pulse` or `~/.andromeda-pulse` off Windows (`:67-78`);
  - `executable_extensions` uses the bare name always and adds `PATHEXT` only under `cfg!(windows)` (`:144-159`).
  - So `sidecar_resolves_on_path` resolves an extensionless Linux `andromeda-pulse-mcp`. **No code change is needed
    for Linux spawn or resolution.**
- `crates/conductor-verify/src/bin/stub_pulse_mcp.rs` (`:7`, `:52`) — the CHILD stub's `tools/list` is built from
  `READBACK_TOOLS`.
- `crates/conductor-verify/tests/common/mod.rs` (`:107`, `:118`, `:135`, `:202-210`, `:279`) — the IN-PROCESS stub:
  - a literal default tool list (`"mark_incident_resolved"` at `:118`, not `READBACK_TOOLS`);
  - per-tool `tools/call` arms with a `resolve_declines` error knob;
  - `{"ok": true}` for an unknown tool.
- `crates/conductor-run/src/lifecycle.rs` (full to `:135`) — `LifecycleObservation { before, resolved, after }`.
  `probe_resolve_lifecycle` (`:114-129`) reads the active set, logs the id on `message`, resolves, then reads again.
  **It stamps no time.**
- `crates/conductor-run/tests/p075_round_live.rs` (full):
  - the firing-form doc (`:8-18`) names a Windows MSYS `PATH`;
  - `ANDROMEDA_PULSE_DATA_DIR` reaches `ReadbackClient::connect` only (`:102-105`);
  - the `std::time` emission stamp (`:116`);
  - one buffered `p075-round:` block (`:110`, `:220`).
- `crates/conductor-run/tests/lifecycle_harvest.rs` (`:223`, `:230`, `:426-457`) — `RoundGrade`, `round_fields`, the
  assertion 1/2 ids and the tamper arms.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (`:378-470`) — `ROUND_EVIDENCE`, the assertion 3–6 ids and
  the tamper arms.
- `crates/conductor-run/tests/evidence_pin/mod.rs` — `sha256_hex` / `check_digest` / `committed` / `pinned`.
- The prior round's chunk:
  - `evidence/round-ledger.md` (full), `evidence/operator-pass.md`, `evidence/p075-leg.txt`;
  - the `[[gate]]` index of `plan.md` (`:280-507`).
- `scripts/agent-run.sh` (`:52-56`, `:141`, `:153`, `:192`, `:295-318`) — `ensure_frontend` precedes the cargo arms.
  The only Windows-shaped lines are two operator hints (`taskkill`, `conductor.exe`), not executed paths.

## Pulse at S2 (`cdb6c1e`) — read directly, no agent
Paths are relative to the Pulse repo; HEAD = S2 (re-read).
- **What changed S → S2** (`git log S..S2` = one commit; `git diff --stat S S2 -- crates pulse-app ui` = 7 files):
  - `coerce_event_kind` now admits `triage::contract::incident_event_kinds()` = `created` / `active` /
    `acknowledged` / `resolved` (`crates/mcp-server/src/tools.rs:491`; `crates/triage/src/contract.rs`, new
    `INCIDENT_EVENT_CREATED` + `incident_event_kinds`);
  - the producer writes `INCIDENT_EVENT_CREATED` (`pulse-app/src/inference_runtime.rs:922`);
  - `tools/list`'s description is corrected (`jsonrpc.rs`);
  - tests were added.
  - The S-time FAIL-by-construction (`created` → `unknown`) is gone at S2, pinned by S2's
    `retrieve_incident_events_reads_the_producers_created_event_as_created`.
- **The tool:**
  - constant `tools.rs:52`, in the tool list `:63`, dispatched at `:180`;
  - `dispatch_retrieve_incident_events` `:498-…`; read limit `:485` = 256;
  - unknown id → `ToolDispatchFailed { reason: "incident not found" }` (`:545`);
  - events read `ORDER BY id ASC LIMIT ?2` (`crates/corpus/src/contract.rs:856-866`).
- **The resolve stamp, re-read at S2** (`tools.rs:451-481`; `contract.rs:645-693`):
  - `let now = current_unix_nanos()` (`:459`) = `chrono::Utc::now().timestamp_nanos_opt()` (`:567-568`), the same
    host clock Conductor's `SystemTime` reads;
  - `update_incident_status(id, "resolved", now, Some(now), …)` runs in ONE transaction. It reads the prior status,
    applies a guarded `UPDATE … WHERE updated_unix_nano <= now`, and inserts a `resolved` event at
    `occurred_unix_nano = now` ONLY when the prior status differs.
  - A stale guard → `DeclinedStale` → JSON-RPC error "incident changed concurrently; resolution not applied".
  - **So the `resolved` event's time lies inside Conductor's request-sent → response-received window, inclusive at
    both ends.** An incident already resolved gains no new event.
- **Since the prior round** (`git diff --stat 03ec944 S2 -- crates pulse-app ui` = 9 files): only
  `crates/corpus`, `crates/mcp-server`, `crates/triage/src/contract.rs` (+24, additive) and
  `pulse-app/src/inference_runtime.rs` (+3/−3, the const swap).
  - Nothing under `ui/`, no telemetry leaf, no auto-resolve or observer file moved.
  - The prior round's source readings for assertions 1–6 (fingerprint union, `degraded_mode`, the four delegated
    leaves, auto-resolve 120 s + 30 s tick, post-resolve re-emission ≤ 60 s) therefore carry to S2, and the P-025
    contract's `03ec944` coordinates need no re-pin.
- **Linux launch history:** Pulse's S2 operator pass (`…/evidence/operator-pass.md:54-61`, committed copy at S2)
  records the pass on this Linux host with **"no pulse-app process was launched; 4317/4318 stayed closed"**.
  - No record shows `pulse-app` ever launched on Linux, so the UI-timed leaves behind assertions 3–6 are UNMEASURED
    on this platform.
  - Pulse's own cross-process test (`pulse-app/tests/e2e_p3_mcp_incident_events_content.rs`, added at S2) exercises
    the producer → sidecar path, not the UI.
- **The binary record:** `round-binary.md` as committed at S2 still names S's Windows artifacts. The S2 Linux values
  live in Pulse's uncommitted working copy (`git -C andromeda-pulse status --short`: ` M …/round-binary.md`,
  ` M …/operator-pass.md`). Both sha256 values were re-measured equal to that copy (scope §The binary under test).

## Graph impact
**derived-without-graph** (rust plane): there is no `.andromeda/cache/` on this fresh Linux checkout and no python
`duckdb`, so `scripts/code-graph.py` cannot run. Call and reference sets are re-derived by grep over `crates/` WHOLE
(`grep -rn --include=*.rs "\b{name}\b" crates`). The counts below include each definition, so they are line-hit
counts, not caller counts:
- **`READBACK_TOOLS`** — 9 hits:
  - the def `manifest.rs:17`; uses `manifest.rs:58,83,101`;
  - the re-export `lib.rs:50`;
  - the child stub `stub_pulse_mcp.rs:7,52`;
  - `tests/preflight.rs:12,103`.

  Every use iterates, so `[&str; 4]` → `[&str; 5]` breaks no call site, and the child stub follows automatically.
- **`probe_resolve_lifecycle`** — 7 hits: the def `lifecycle.rs:114`, a doc mention `:4`, the re-export `lib.rs:38`,
  and calls at `tests/lifecycle_live.rs:155` and `tests/p075_round_live.rs:199` (+ their imports). There is NO
  production caller.
- **`LifecycleObservation`** — 16 hits. The struct literals are at `lifecycle.rs:124`, `src/testkit.rs:125` and
  `tests/lifecycle_harvest.rs:83,94`; there are also type refs (`composition_root.rs:24,41`).
  - A new field would thread through all four literals.
  - An additive sibling function threads through none.
- **`resolve_incident`** — 4 hits: the def `client.rs:186`, `lifecycle.rs:122`, `tests/readback.rs:283,291`.
- **`MARK_INCIDENT_RESOLVED`** — 8 hits: `client.rs:26,179`, `lib.rs:44`, `manifest.rs:11,21`,
  `tests/preflight.rs:11,131,137`. This is the sibling-const precedent for `RETRIEVE_INCIDENT_EVENTS`.
- **`retrieve_incident_events`** — 0 hits (`grep -rn --include=*.rs retrieve_incident_events crates/`).
- **Companion sweep (data pins, not calls):**
  - `grep -rln '"mark_incident_resolved"' crates contracts scripts` → 3 files: `client.rs`, `tests/common/mod.rs`, the
    manifest;
  - `grep -rln mcp-contract.toml crates scripts` → 10 readers, all dynamic (no count literal);
  - `grep -rn -E '\[&str; 4\]|tools [0-9]/[0-9]' crates scripts contracts` → 2 hits. `manifest.rs:17` changes.
    `operator_pause_harvest.rs:231` `WITNESSES` is unrelated, so no change.

## Patterns detected
- **The live leg prints and the harvest grades** (`p075_round_live.rs:20-23`; `lifecycle_harvest.rs:426-457`).
- **Absence is UNGRADED, never met** (`RoundGrade`, `lifecycle_harvest.rs:223`).
- **Digest-pinned evidence graded from file, with a tamper arm** (`evidence_pin/mod.rs`; `lifecycle_harvest.rs:446-457`;
  `delegated_timing_harvest.rs:456-470`).
- **A JSON-RPC-error knob on the stub** (`common/mod.rs:107,210`, `resolve_declines`).
- **A one-field typed convenience over the raw-`Value` call** (`client.rs:186-189`).
- **Stdout carries integers, booleans and closed words only.** The prior hex probe `grep -cE "[0-9a-f]{16,}"` reads 0
  on the capture. A raw 19-digit `occurred_unix_nano` would match it, so the leg prints window-RELATIVE offsets.

## Conventions to follow
- A new tool name joins the client as a `pub const` beside the four (`client.rs:22-26`), is re-exported from `lib.rs`,
  and enters `READBACK_TOOLS`.
- The incident id rides `message`, never a span attribute (`lifecycle.rs:119-121`).
- Readers accept `incident_id` and `id`. The new tool's stub mirrors S2's keys: `incident_id`,
  `events[].event_kind`, `events[].occurred_unix_nano`, `total`, `truncated`.
- Window stamps are `std::time::SystemTime` epoch nanos, taken immediately around the `resolve_incident` await.
- A `live-pulse`-gated target owes `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`.
- Capture output is buffered and written once, leading with a newline.
- Name every scenario by its stem: P-025/027/037/045 are each named by several scenarios.
- No `boot` before a preflight-firing leg. Use 150 s quiet windows between legs on one launch.
- Prior-round Windows-only recipe elements a Linux leg must replace (from its `plan.md` gate index):
  - the `powershell.exe … Win32_Process` census (`:360`), replaced by `ps`;
  - the `/d/…` MSYS `PATH` and `.exe` artifact names (`:368-456`), replaced by the Pulse checkout's `target/release`
    and extensionless names;
  - `cygpath -u "$ANDROMEDA_PULSE_DATA_DIR"` (`:384`), which has no Linux counterpart and is unneeded;
  - `C:/Users/…/gate.py` / `ci.py` (`:489`, `:505`), replaced by the Linux skill path;
  - the `Stop-Process -Force` teardown;
  - the `%TEMP%/pulse-legs/<leaf>` data dir. `TEMP` is unset here.
- Evidence file names stay outside the `rm-capture*.txt` population glob. Committed evidence and gate text carry no
  `/home/` token (the host-path anchor) — the Linux checkout paths are exactly that anchor.

## New files to create
- `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/` — the round's committed evidence: the P-075 leg capture, each delegated leg's frozen self-obs and target-filtered Pulse slice, and the round ledger with the binary sha256 values and the censuses

## Files to modify
- `contracts/mcp-contract.toml` — `required_tools` gains `retrieve_incident_events`
- `crates/conductor-verify/src/client.rs` — the `RETRIEVE_INCIDENT_EVENTS` const and a typed by-id read through `call_tool`
- `crates/conductor-verify/src/lib.rs` — re-export the new const
- `crates/conductor-verify/src/manifest.rs` — `READBACK_TOOLS` grows to five
- `crates/conductor-verify/tests/common/mod.rs` — the default tool list gains the fifth name and the stub answers the new tool with S2's raw shape plus an unknown-id error knob
- `crates/conductor-verify/tests/readback.rs` — stub-tier arms for the new read
- `crates/conductor-verify/tests/preflight.rs` — a sidecar lacking the fifth tool blocks on the existing tool-absence precondition
- `crates/conductor-run/src/lifecycle.rs` — the additive timed sibling of the resolve probe
- `crates/conductor-run/src/lib.rs` — re-export the new lifecycle item
- `crates/conductor-run/tests/p075_round_live.rs` — events read before and after the resolve, window-relative output, a host-neutral firing-form doc
- `crates/conductor-run/tests/lifecycle_harvest.rs` — this round's assertion 1, 2 and 7 grades over its pinned capture
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — this round's assertion 3 to 6 grades over its pinned slices

## Open questions
- **Where the live round runs** (the operator's directive: a FOUNDER decision at P4, never decided here). The facts
  the fork needs, all measured above:
  - the relayed binary is a Linux build, and no Windows S2 artifact exists;
  - `pulse-app` has no recorded launch on Linux, so assertions 3–6's UI leaves are unmeasured on this platform;
  - Conductor's CI is Windows-only, with no Pulse checkout and no live-Pulse job. A CI arm would also be a second CI
    egress, a network-arriving program and a NINTH rule-(b) crossing, each an escalation (security extract §Contract
    bindings);
  - Conductor itself has never been built on this host (the bundle prerequisite, `cargo-audit` absent).

  → blocks: plan-decision.
- **`cargo-audit` on this host:** the supply-chain gate needs it, at the stack's `0.22.2` (≥ as a floor), and installing
  it is a host act outside the repo. → blocks: implementation-scope (a PREREQ the plan names; no repo file changes).

## Scope premise closure
Each `[inferred]` bullet of `scope.md` is closed against the findings above. `scope.md` is amended in place.
