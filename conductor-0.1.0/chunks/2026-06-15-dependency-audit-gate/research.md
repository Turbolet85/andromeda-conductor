# Codebase Research — 2026-06-15-dependency-audit-gate

## Scope
- **Depth:** moderate · **Reads:** 4 (`Cargo.toml`, `rust-toolchain.toml`, `crates/conductor-tauri/Cargo.toml`, `dependency-tree.md`) · **Globs/Greps:** 3 (`deny.toml`/CI/`.cargo` glob · `Cargo.lock` crate grep · `git ls-files` + tool-presence probe)

## Files inspected
- `Cargo.toml` (full) — workspace manifest. `rust-version = "1.94.1"` (line 17) and `tauri = "2.10.3"` (line 55) already pinned; OTLP/gRPC (`opentelemetry-proto`/`tonic`/`tonic-prost`/`prost`), `rmcp`, `rusqlite`, `tokio`, `tracing*` are declared in `[workspace.dependencies]` (lines 28-55) but only enter the build when a member opts in.
- `rust-toolchain.toml` (full) — channel `1.95.0` ≥ MSRV `1.94.1`; tar-rs CVE-2026-33056 floor already satisfied.
- `crates/conductor-tauri/Cargo.toml` (full) — depends ONLY on `conductor-core`; does NOT reference `tauri` yet (Epoch 9). ⇒ `tauri` is absent from the lock.
- `.andromeda/context/dependency-tree.md` (full; reconciled 2026-06-15) — current real graph: `conductor-core` → `garde 0.22.1` (+ `garde_derive 0.22.1`, `compact_str`, `smallvec`, `serde_derive`…), `serde 1.0.228`, `thiserror 2.0.18`; dev `serde_json 1.0.150`. The other 7 crates depend only on `conductor-core`. No OTLP/gRPC/SQLite/rmcp/tokio/tracing in the graph yet.
- `Cargo.lock` (Grep) — confirms `tauri`/`rusqlite`/`libsqlite3-sys`/`tonic`/`opentelemetry-proto`/`rmcp`/`tokio`/`tracing-subscriber` are **NOT** locked; only the garde tree + serde + thiserror (+ dev serde_json) are present. 253 lines, git-tracked.

## Patterns detected
- **Declared-but-unreferenced workspace deps** (`Cargo.toml:28-55`): versions are centrally pinned, but a crate only pulls one into the lock via `{dep}.workspace = true`. cargo-audit/cargo-deny audit the **lock**, so today they only see the actually-referenced crates (garde/serde/thiserror tree).
- **Version floors pre-satisfied** (`Cargo.toml:17,55` · `rust-toolchain.toml:2`): the security-plan's "bumps required (MSRV 1.88.0 / tauri 2.10.1)" was already applied in `2026-06-14-cargo-workspace-scaffold`; the manifest is at 1.94.1 / tauri 2.10.3. This chunk **confirms via audit**, it does not bump.

## Conventions to follow
- **`Cargo.lock` committed as the reproducibility anchor** (git-tracked); un-drift verified via a `--locked` invocation (arch §Infrastructure Patterns Build system).
- **Central pins in `[workspace.dependencies]`**; `deny.toml` policy *complements* them (bans / licenses / sources), never duplicates the version pins.
- **`cargo audit` already named** in the CLAUDE.md Workflow block + `.claude/docs/commands.md` — align invocation wording, add `cargo deny check`.

## New files to create
- `deny.toml` (repo root) — cargo-deny policy: `[advisories]` (RustSec DB), `[bans]` (duplicate / yanked / disallowed crates), `[licenses]` (allowed-license allowlist for a local-only personal tool), `[sources]` (crates.io as sole registry).

## Files to modify
- `Cargo.lock` — only if a `--locked` run reveals drift (regenerate + commit); currently tracked and expected clean.
- `.claude/docs/commands.md` (minor) — already lists `cargo audit`; add the `cargo deny check` invocation to the documented gate. No crate source changes.

## Open questions
- **"OTLP/gRPC/SQLite tree" vs the actual lock:** those crates are not locked yet (Epochs 3/5/6). Resolution for P4: this chunk establishes the **gate + `deny.toml` + a green baseline over today's lock** (garde/serde/thiserror); the gate's remit extends to OTLP/gRPC/SQLite automatically when those deps land (arch's "baseline at feature lock"). Not a divergence — surface as a **framing note at P5**, keep the title's intent.
- **cargo-deny already on PATH** (contradicts the handoff's "not installed") — so the gate is "confirm installed + version + run green", not "install". Confirm the installed `cargo-deny` version → its `deny.toml` schema (0.18+ uses the v2 `[advisories]`/`[bans]` field shape) at implement time.
- **garde 0.22.1 advisory status** is directly closeable here — the garde tree IS in today's lock, so the green run settles last session's open item immediately.
