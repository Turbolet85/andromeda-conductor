# Scope — Read-back seam survivors closed

**Marker:** `2026-08-20-read-back-seam-survivors-closed`
**Version:** conductor-0.2.0 · **Epoch:** Epoch 4 — Lifecycle & delegated timing
**Working entry:** "Read-back seam survivors closed — the five out-of-family audit survivors gain killing
assertions (code-audit baseline 2026-08-20, §B2 remainder)"

## What this chunk builds

Killing assertions for the **five mutation survivors in `conductor-verify` that sit outside the four
families `2026-08-20-verifier-self-hardening` named**. After this chunk, every survivor the 2026-08-20
code audit surfaced is either **killed** or **classified accepted-deliberate** — the precondition for the
next boundary's trend measurement to mean anything.

This is a **test-integrity chunk**: it adds assertions and, where an observation surface is missing, the
harness that makes an existing behaviour observable. It claims no capability and drives no live Pulse.

## The five survivors (coordinates VERIFIED, not assumed)

The working entry's coordinates were dictated and fold here as hypotheses. Each was re-verified against
`.andromeda/runs/2026-08-20T18-06-29-code-audit/c-mutation-conductor-verify.json` (`survivors` array) **and**
against the source line itself. All five match exactly:

| Site | Mutation | Verified |
|---|---|---|
| `jsonrpc.rs:41:22` | `replace += with -= in JsonRpcSession::request` | line 41 is `self.next_id += 1;` |
| `jsonrpc.rs:41:22` | `replace += with *= in JsonRpcSession::request` | same site |
| `jsonrpc.rs:65:9` | `replace JsonRpcSession::notify -> Result<(), VerifyError> with Ok(())` | line 65 is `notify`'s first body line |
| `preflight.rs:357:9` | `replace ShapeWitness::list with ()` | line 357 is `if !std::mem::replace(&mut self.list, true) {` |
| `preflight.rs:357:12` | `delete ! in ShapeWitness::list` | col 12 is that `!` |

**The "still-alive" claim reconciles arithmetically** [VERIFIED at P3 — the entry asserted it, this is the check]:
the baseline JSON lists **15** survivors across these two files; `2026-08-20-verifier-self-hardening` killed
**10** of them (its `MAX_LINE_BYTES` x5 at `jsonrpc.rs:18`/`:79` and canary-boundary x5 at
`preflight.rs:385`/`:335` families), leaving exactly these **5**. That matches the prior chunk's measured
scoped re-run: **33 caught / 5 missed / 11 unviable / 2 timeout, zero missed among targeted sites**
(`chunks/2026-08-20-verifier-self-hardening/report.md:26`).

## Why each one survives (the observation gap each must close)

1. **id-counter x2** — the entry states the kill must break **correlation**, not arithmetic. `-=` yields the
   id sequence `1,0,-1...`; `*=` pins it at `1` forever ([VERIFIED] `next_id: 1` at `jsonrpc.rs:33`). A test
   that only checks a call succeeds cannot see either. **[VERIFIED + SHARPENED at P3 — this is the
   structural reason both survive]:** the shared stub **echoes the id it received**
   (`tests/common/mod.rs:145`, `json!({"jsonrpc":"2.0","id": id, ...})`), so it answers a mutated sequence
   exactly as agreeably as the correct one. The kill therefore needs the stub to **record the observed id
   sequence** (or to answer with a fixed/decoy id so a mutated expectation mis-pairs) — merely driving two
   requests through the existing stub kills neither.
2. **`notify`** — its ONLY call site is `client.rs:93`, `let _ = session.notify("notifications/initialized", ...)`,
   whose result is **discarded** by design ("Pulse tolerates its absence"). [VERIFIED — the graph shows
   exactly one caller.] No return-value assertion can ever kill this mutant: **only the wire witnesses it**.
   **[SHARPENED at P3]** the shared stub `continue`s on any line with no `id` (`tests/common/mod.rs:82-84`),
   and `jsonrpc_line_bound.rs:37` explicitly **drains** "the best-effort `notifications/initialized`" — so
   the line already reaches the wire in tests and is simply never asserted on.
3. **`ShapeWitness` x2** — `list()`'s only observable effect is one `tracing::info!` line emitted through
   `extract::log_observed_keys` (`extract.rs:214-216`). [VERIFIED] `-> ()` emits **zero** lines; the
   **`!`-deleted** form emits on every answering call **except the first**. At **three** answering attempts
   the counts separate cleanly — correct **1**, `!`-deleted **2**, `-> ()` **0** — so one `assert_eq!(count, 1)`
   kills both. **[SHARPENED at P3]** `log_observed_keys` has **four** call sites (`extract.rs:83`/`:95`/`:110`
   + `preflight.rs:358`), so the count must be scoped by the `query_incident_list` tool name for the
   assertion to mean what it says.

## Boundaries

- **In:** `crates/conductor-verify/` tests (plus whatever test-support the three gaps above require) and, if
  a witness needs it, that crate's `[dev-dependencies]`.
- **Not in:** any production-behaviour change. Widening `pub(crate)` visibility is explicitly **not** needed —
  the entry pins the public `ReadbackClient::connect_transport` duplex as the seam, VERIFIED public at
  `client.rs:71` and already the worked precedent in `tests/jsonrpc_line_bound.rs`, `tests/preflight.rs`
  (`drive_with`) and `tests/readback.rs`.
- **Not in — the 2 TIMEOUTs.** [VERIFIED — they sit in the JSON's separate `timeouts` array, not `survivors`.]
  `jsonrpc.rs:49:30` (`!=` to `==` in the id-match loop) and
  `jsonrpc.rs:70:9` (`write_message` to `Ok(())`) sit in the same file and persisted through the prior
  chunk's scoped re-run, but they are `timeouts`, a **distinct outcome class** from `survivors` — the entry
  scopes this chunk to survivors. Surfaced here so the boundary is a decision, not an omission.
  **[RATIFIED at P4 — operator decision]** five only; the scoped re-run **reports** both timeouts' status as
  an observation. A stub that closes its side after its scripted responses can turn a correlation failure
  into a typed `VerifyError::Transport(UnexpectedEof)` (`jsonrpc.rs:104-107`) rather than a hang, so
  `:49:30` may convert to CAUGHT for free — **any such conversion is a bonus, never an acceptance
  criterion**, so the chunk cannot fail on it.
- **Not in:** the other 13 baseline survivors outside these two files (e.g. `stub_pulse_mcp.rs`), already
  dispositioned or excluded by the prior chunk.

## Contracts / invariants this chunk must respect

- **test-plan §11 process-global-singleton ban.** `init_observability` installs a **process-global**
  subscriber (first install wins), so any test capturing the self-obs artifact must be **alone in its own
  test binary** — the ban the prior chunk added after measuring the failure. `tests/canary_obs_witness.rs`
  (conductor-run) is the worked precedent. [VERIFIED — `init_observability` calls
  `tracing::subscriber::set_global_default`, `conductor-core/src/obs.rs:83`.] This binds the ShapeWitness
  witness specifically.
- **Zero-retry determinism.** nextest stays zero-retry and no real-clock dependency is introduced —
  `CanaryPoll { attempts, interval }` has public fields ([VERIFIED] `preflight.rs:80-84`), so a
  multi-attempt poll is constructible with `interval: Duration::ZERO`. **[SHARPENED at P3]** a
  multi-attempt drive already exists — `the_poll_loop_sleeps_between_attempts_but_not_after_the_last`
  (`tests/preflight.rs:228-266`) — so the witness reuses a shipped shape rather than inventing one.
- **Verdict/error wall** unchanged — this chunk asserts on existing behaviour, it does not reclassify any
  outcome.
- **Mutation-tooling traps** (test-plan §4): `-f` resolves from the **workspace root**;
  `--test-tool=nextest` is REQUIRED; `Found 0 mutants to test` **exits 0** — a silent no-op that looks
  exactly like a clean pass, so the scoped re-run must confirm a non-zero mutant count.

## Acceptance (from the working entry)

A scoped `cargo mutants -p conductor-verify --test-tool=nextest -f crates/conductor-verify/src/jsonrpc.rs
-f crates/conductor-verify/src/preflight.rs` shows **these five killed** with the **previously-killed set
still dead**; **nextest stays zero-retry**.

## Carried PREREQ — `cargo audit`, 34th consecutive (PROBE-AUTO-SATISFY)

Standing deferral since `2026-08-08-sut-capability-manifest`; operator-ratified at the
`2026-08-10-workspace-key-divergence-probe` wrap; basis RE-DERIVED at `2026-08-20-verifier-self-hardening`
(the lock moved for the first time in the chain — one removed dev-edge line, **zero package-set change**,
`cargo deny check advisories bans licenses sources` observed true exit 0 over the POST-removal lock).

**PROBE-AUTO-SATISFY signature:** `cargo audit` true exit 1 whose first diagnostic line is
`duplicate advisory ID: RUSTSEC-2026-0244` AND `cargo deny` true exit 0. Reproduce byte-identically and the
pin is satisfied by the one-line record `probe unchanged, 34th consecutive`, no basis re-authoring. ANY
deviation (changed diagnostic, moved exit code, overlap shift, or a dependency delta that ADMITS a package)
restores the FULL form. Remedy stays the bounded wait — no floor raise, no `deny.toml` ignore, no CI edit.

**This chunk may itself move the lock** if a dev-dependency is added (see Boundaries). A dev-dep already
present in the workspace adds no package and keeps the compact form; anything that ADMITS a new package
restores the FULL form by the signature's own terms. **[VERIFIED at P3]** the only dev-dep this chunk is
likely to want — `assert_fs`, for the self-obs temp sink the precedent uses — is already
`[workspace.dependencies]` (`Cargo.toml:83`) and already consumed by `conductor-run`, so adding
`assert_fs.workspace = true` moves ONE lock EDGE line with **zero package-set change**: the compact
PROBE-AUTO-SATISFY form holds. The plan should prefer it over any package that would admit a new node.
**[RATIFIED at P4 — operator decision]** take the `assert_fs` dev-dep, matching the worked precedent and
its drop-cleanup (incl. on panic). The zero-package-delta claim is **to be verified by a `Cargo.lock` diff
at implement**, not assumed — that diff is what entitles the compact PROBE-AUTO-SATISFY form; a package
node appearing restores the FULL form.

## Audit-ledger item carried forward (verified first-hand at this take-up)

`c-mutation-conductor-verify.json`'s `command` field reads
`cargo mutants -p conductor-verify --output {run_dir}/mutants-conductor-verify --jobs 2` — it **omits
`--test-tool=nextest`**, while code-audit §B3 claims the flag is recorded there. Confirmed by reading the
artifact. Belongs to the next boundary's audit, not to this chunk's work.
