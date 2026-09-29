# Codebase Research — 2026-09-29-dual-license-mit-or-apache-2-0

## Scope
- **Depth:** minimal (the chunk changes manifest metadata plus two new root text files, and touches no code symbol).
  **Reads:** 9 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/testing.md` and `.claude/rules/verification-harness.md` were read in
  full (they auto-loaded on the `crates/**/tests/**` read). Applied: the 2026-09-09 established-is-not-green rule
  (every gate below is baselined at HEAD), and the nextest-selector rule (no filtered run is planned). There is no
  live leg in this chunk, so no firing form is needed.
- **Platform issues consulted:** none. No runner-only bullet: the one CI verdict folded, `c97f697`, was
  `in progress`, not red.

## Files inspected
- `Cargo.toml` (`[workspace.package]`, `:15-19`) — holds `version` / `edition` / `rust-version` / `publish = false`.
  There is no `license` key.
- `crates/{cli,core,emit,faults,report,run,tauri,timeline,verify}/Cargo.toml` (`[package]` blocks) — all nine carry
  `name` plus `version.workspace` / `edition.workspace` / `rust-version.workspace` / `publish.workspace = true`, and no
  `license`.
  - Derivation: `sed -n '/^\[package\]/,/^\[/p'` over each manifest.
  - Confirmation: `cargo metadata --format-version 1 --no-deps --offline | jq -r '.packages[] | "\(.name) \(.license)"'`
    prints `null` for all 9 packages.
- `crates/conductor-tauri/ui/package.json` — keys are `name, private, version, type, scripts, dependencies,
  devDependencies`, with no `license`. `"private": true` stays.
- `crates/conductor-tauri/ui/package-lock.json` — `lockfileVersion 3`, 782 package entries.
  - `packages[""]` today carries only `name` and `version` (python read of the JSON).
  - The lock already holds `OR`-form license expressions without parentheses elsewhere (`Apache-2.0 OR MIT`), so
    the unparenthesized form has precedent inside this very file.
- `deny.toml` (`:51-68`) — the `[licenses] allow` list includes `MIT` (`:55`) and `Apache-2.0` (`:56`).
  - `:67-68`: `# The workspace's own conductor-* crates are unpublished — skip license checks for them.` /
    `private = { ignore = true }`.
- `crates/conductor-core/tests/secret_scan_gate.rs` (full) — the gate's subject is
  `git ls-files -z --cached --others --exclude-standard`.
  - It has 14 content rules (`:19-46`), the file-name rule (`:49`), and an empty `ALLOWLIST` (`:60`) graded as an
    exact set.
  - `LICENSE-MIT` / `LICENSE-APACHE` match no part of the file-name rule, which keys on `.env*`, `id_*` and
    `*.pem|p12|pfx|key|cer|crt|jks|keystore`.
  - The Apache text's only URLs are `http://www.apache.org/licenses/…`, which carry no `user:pass@`, so they fall
    outside the `url-credentials` shape. It is not a measured pass until the gate runs with the files present.
- `.gitattributes` — `* text=auto eol=lf` is repo-wide (`:2`). Both new root files fall under it.
- `.github/workflows/ci.yml` — the `rust` job runs `npm ci` + `npm run build` in `crates/conductor-tauri/ui` (`:52-55`).
  The `frontend` job (`:243-268`) runs `npm ci`, `npm audit --omit=dev` and `npm run build`. Both consume
  `package.json` + `package-lock.json` as committed.
- `.andromeda/test-plan.md:469` — carries the verbatim "the stage table above is the complete inventory of gates CI
  enforces". The tests-history note about a missing premise is therefore answered: the wording exists. This plan does
  not lean on it either way.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **No symbol in scope** — the graph was not queried. The chunk edits no Rust or TS item, only `[package]` /
  `[workspace.package]` metadata and `package.json`'s `license` key.
- The crate-edge property the arch extract asks to hold (members unchanged, no dependency edge added) is measured
  directly by `cargo metadata` before and after, which is a stronger basis than the graph for a manifest diff.

## Patterns detected
- **Workspace metadata inheritance** (`Cargo.toml:15-19`; every member `[package]`): shared keys live once in
  `[workspace.package]`, and members opt in with `key.workspace = true`. `license` joins the four keys already
  inherited.
- **Justified `deny.toml` entries** (`deny.toml:59-64`): every non-obvious setting carries an inline reason.
  `:67`'s rationale for `private.ignore` ("unpublished — skip license checks") is the one this chunk's fork turns on.
- **Lockfile via the tool, never by hand**: measured in a scratch copy of the two npm files.
  - `npm pkg set license="MIT OR Apache-2.0"` appends the key as `package.json`'s last member.
  - `npm install --package-lock-only --ignore-scripts --offline` then exits 0, printing `found 0 vulnerabilities`.
  - The lock diff is exactly one added line, `"license": "MIT OR Apache-2.0",` under `packages[""]`, after `version`.
  - Offline means no registry round-trip, and no other resolution moves.

## Conventions to follow
- **SPDX expression form `MIT OR Apache-2.0`**: the scope's spelling; the npm lock already uses bare `OR`
  expressions (`package-lock.json`, python read). `cargo deny`'s allow list names both terms (`deny.toml:55-56`).
- **LICENSE-APACHE text source**: measured over the local cargo registry, 1091 `LICENSE-APACHE*` files (`find
  $CARGO_HOME/registry/src/index.crates.io-*/ -maxdepth 2 -name 'LICENSE-APACHE*'`, then `sha256sum`, grouped).
  - The dominant variant is sha256 `a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2`: 472 copies,
    201 lines, 10 847 B, LF, ASCII. It ships with rust-lang-owned crates (`log`, `cfg-if`, `once_cell`).
  - A whitespace-normalized word comparison shows it is word-identical to the 11 357 B variant `c71d239d…` that
    `clap` ships. So the Rust-ecosystem file is the ASF text re-flowed, with the terms unchanged.
  - The `{}`-bracket variant `c6596eb7…` differs in one appendix token, and `futures`' variant embeds its own
    copyright line. Both are rejected.
  - Offline, there is no way to byte-compare against apache.org's published file. So the byte-level claim this plan
    makes is "identical to the Rust-ecosystem standard `LICENSE-APACHE` (`a60eea…`)", never "byte-identical to
    apache.org".
- **LICENSE-MIT text**: the MIT terms as the Rust ecosystem ships them (`log-0.4.28/LICENSE-MIT`, read in full). A
  copyright line, a blank line, then the three MIT paragraphs, with the copyright line set to
  `Copyright (c) 2026 Turbolet85` per the operator's take-up directive.

## New files to create
- `LICENSE-MIT` — the MIT terms with `Copyright (c) 2026 Turbolet85`.
- `LICENSE-APACHE` — a byte copy of the Rust-ecosystem Apache-2.0 text, sha256 `a60eea81…`.

## Files to modify
- `Cargo.toml` — `license = "MIT OR Apache-2.0"` in `[workspace.package]`.
- `crates/conductor-cli/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-core/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-emit/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-faults/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-report/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-run/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-tauri/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-timeline/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-verify/Cargo.toml` — `license.workspace = true`.
- `crates/conductor-tauri/ui/package.json` — the `license` key, set through `npm pkg set`.
- `crates/conductor-tauri/ui/package-lock.json` — the one-line `packages[""].license` mirror, regenerated by npm.
- `deny.toml` — `private = { ignore = true }` removed and the `:67` rationale rewritten (fork arm B, decided at P4).

## Open questions
- `deny.toml` `private.ignore`: keep skipping our own crates (A), or check them now that they carry a license (B)?
  → blocks: plan-decision.
  - Measured at HEAD with a scratch config that differs from `deny.toml` only in `private = { ignore = false }`
    (`cargo deny --config <scratch> check licenses`): **exit 4**, `error[unlicensed]` on exactly the nine
    `conductor-*` crates and on nothing else.
  - So arm B's gate is RED before the manifest edit and green after only if all nine carry an allowed expression.
    It is a discriminating witness.
  - Under arm A, `cargo deny check licenses` exits 0 at HEAD and after, and says nothing about our crates.
