# Scope — 2026-08-08-dependency-advisory-remediation

**Working-route entry (verbatim):**
> Dependency advisory remediation — crossbeam-epoch ≥0.9.20 and quick-xml ≥0.41 through the Tauri tree,
> restoring cargo-audit and cargo-deny green

No `PREREQ:` / `CARRY:` annotations on the entry — nothing deferred to absorb.

## Intent

Restore the supply-chain gate to green. The gate has been RED since the 41-day pause: the advisories were
published 2026-06-29 → 2026-07-06 against a `Cargo.lock` that is byte-identical to HEAD, so **the gate decayed
with wall-clock time, not with any diff**. Both `.claude/rules/security.md` and CLAUDE.md's universal
invariants forbid a release build or merge while it is red, so this blocks every downstream chunk that ends
in a release gate.

## What this chunk builds

Four advisories cleared by **lock-only dependency bumps**, then re-proven across the full quality gate.

| Advisory | Package | Class | Fix |
|---|---|---|---|
| RUSTSEC-2026-0194 | quick-xml 0.39.4 | vulnerability (CVSS 7.5, A:H) | → 0.41.0 via `cargo update -p plist` |
| RUSTSEC-2026-0195 | quick-xml 0.39.4 | vulnerability (CVSS 7.5, A:H) | → 0.41.0 via `cargo update -p plist` |
| RUSTSEC-2026-0204 | crossbeam-epoch 0.9.18 | vulnerability | → 0.9.20 via `cargo update -p crossbeam-epoch` |
| RUSTSEC-2026-0190 | anyhow 1.0.102 | unsound (deny-only) | → 1.0.104 via `cargo update -p anyhow` |

Definition of done: `cargo audit` exit 0 **and** `cargo deny check advisories bans sources licenses` exit 0,
with the workspace still building and every existing test green, and `Cargo.lock` committed + un-drifted.

## Three route-entry premises corrected during take-up

The entry's mechanism was written from the prior session's reading; grounding it against the live tooling
(evidence: `.andromeda/runs/2026-08-08T19-29-55-phase/advisory-evidence.md`) corrected three points. The
**intent is unchanged** — only the route to it.

1. **RUSTSEC-2026-0190 is `anyhow`, not `crossbeam-epoch`,** and it is an `unsound` advisory: `cargo audit`
   reports it as an allowed warning (exit 0) while `cargo deny` denies it. It is invisible to an audit-only
   check — a live instance of the 2026-06-23 session learning that both tools must run.
2. **crossbeam-epoch is not in the Tauri tree.** It enters only via `assert_fs` → globwalk → ignore →
   crossbeam-deque — a **dev-dependency**, never compiled into the release binary. Severity in context is
   test-fixture-only; it is fixed anyway because the gate denies it.
3. **quick-xml needs no Tauri bump.** `tauri-utils` is already at its latest (2.9.3); the semver-major
   0.39 → 0.41 jump is absorbed inside `plist`, whose caret-compatible 1.9.0 → 1.10.0 minor bump
   `tauri-utils` already accepts. `cargo update -p quick-xml` alone moves nothing — **plist is the lever**.

Consequence: this is a **lock-only** change. No `Cargo.toml` dependency edit is required, no `deny.toml`
exception is added, and the Tauri version is untouched. These advisories are actionable-with-fixes, so
`deny.toml` remains the wrong instrument for them (it is reserved for the unmaintained-transitive /
non-actionable class).

## Boundaries

**In scope**
- `Cargo.lock` — the three `cargo update -p {plist,crossbeam-epoch,anyhow}` bumps.
- Re-proving the full gate: workspace build, `cargo nextest run --workspace --profile ci` (428 baseline),
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --doc`, `scripts/agent-run.sh run`,
  `cargo audit`, `cargo deny check advisories bans sources licenses`.
- Any **compile or test breakage caused by these bumps** — quick-xml 0.41 is a major bump consumed by
  `tauri-build`'s build script (Info.plist handling), so `conductor-tauri` compilation is the real risk
  surface and must be exercised, not assumed.
- Resolving the RUSTSEC-2024-0429 (glib 0.18.5, unsound) open item below, so the gate cannot come back
  red on a latent fifth error.

**Out of scope**
- Any `Cargo.toml` version-floor edit, unless the P4 review explicitly elects one (see Decisions).
- New `deny.toml` entries — none of these four qualifies.
- A Tauri minor/patch bump (2.11.3 → 2.11.5 exists) — unnecessary for the gate, and a GUI-tree change
  with its own verification cost. Only pull it in if a bump above forces it.
- The 17 `unmaintained` + remaining `unsound` warnings already justified in `deny.toml` — untouched.
- The `npm audit --omit=dev` frontend gate — a separate tree, not advisory-decayed here.
- The stale `rmcp = "1.7.0"` workspace-dependency entry (see Decisions) — flagged, not assumed in.

## Surfaces / contracts touched

- **`Cargo.lock`** — the committed, un-drifted lock is itself the contract (`security.md` §Dependencies:
  it is what makes the audit deterministic).
- **CI** (`.github/workflows/ci.yml`) — the existing separate `cargo audit` + `cargo deny check` steps are
  the gate being restored; no new step is expected.
- **No engine, seam, scenario, report-envelope, or GUI model change.** Zero Rust source change is the
  expected outcome; any source edit would be bump-forced fallout, and is a signal to re-examine scope.

## Decisions to settle in P4

1. **Lock-only, or also raise the manifest floors?** `anyhow = "1.0.102"` in `[workspace.dependencies]`
   would keep naming a version with a known unsound advisory even after the lock moves. Raising it to
   `1.0.103`+ encodes the security floor durably; leaving it relies on the committed lock alone.
2. **The orphan `rmcp = "1.7.0"`.** It sits in `[workspace.dependencies]`, no member references it, and it
   is **absent from `Cargo.lock`** — while the architecture records rmcp as removed on 2026-06-27. It is
   dead manifest cruft contradicting a documented decision, and a zero-risk one-line deletion, but it is
   dependency *hygiene*, not advisory remediation. Include or leave to a later chunk.

## Open item carried into P3

`cargo audit` lists RUSTSEC-2024-0429 (glib 0.18.5, `unsound`) which did **not** appear among cargo-deny's
four errors and is **not** in `deny.toml`'s ignore list. Establish why before declaring the gate green.
