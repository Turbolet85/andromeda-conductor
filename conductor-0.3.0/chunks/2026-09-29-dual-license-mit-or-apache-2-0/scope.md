# Scope — 2026-09-29-dual-license-mit-or-apache-2-0

**Working entry (`working-route.md:52`):** Dual license MIT OR Apache-2.0 — `LICENSE-MIT` and `LICENSE-APACHE`
committed and every manifest's `license` field set, so the public repository carries a license.

## What this chunk builds
- `LICENSE-MIT` at the repository root: the standard MIT text, with the copyright line `Copyright (c) 2026 Turbolet85`.
  The name comes from the operator's take-up directive (2026-09-29): "Copyright line: Turbolet85 unless the founder says
  otherwise (he has not)".
- `LICENSE-APACHE` at the repository root: the Apache License 2.0 terms. [premise-corrected: "as published by the ASF,
  unmodified" cannot be byte-verified offline.]
  - The file is the Rust-ecosystem standard `LICENSE-APACHE`, sha256 `a60eea81…`: 472 of the 1091 copies in the
    local cargo registry, shipped by rust-lang's `log` / `cfg-if` / `once_cell`.
  - It is word-identical to the ASF text as `clap` ships it (research.md §Conventions).
- The `license` field set to the SPDX expression `MIT OR Apache-2.0` on every manifest:
  - the Cargo workspace root `Cargo.toml` — `[workspace.package] license`. Verified: all nine members already inherit
    `version` / `edition` / `rust-version` / `publish` via `*.workspace = true`, and `cargo metadata` reports
    `license: null` for all nine.
  - each of the nine member crates' `Cargo.toml` — `license.workspace = true`;
  - `crates/conductor-tauri/ui/package.json` — `"license"`. The root package's mirrored `license` field in
    `package-lock.json` (`packages[""]`) follows through a real npm regeneration, never a hand edit, so the lockfile
    stays committed and un-drifted (security rule). Verified in a scratch copy:
    - `npm pkg set` + `npm install --package-lock-only --ignore-scripts --offline` exits 0;
    - the lock diff is exactly one added line under `packages[""]`.
- `cargo deny check licenses` stays green for Conductor's own crates. This is the operator's take-up directive
  (2026-09-29): "cargo deny licenses still green for our own crates".
  - Verified: `deny.toml:68` carries `private = { ignore = true }`, and every member is `publish = false`, so today
    the tool never license-checks our own crates (`cargo deny check licenses` → `licenses ok`, exit 0).
  - Measured with that one setting flipped in a scratch config: exit 4, `error[unlicensed]` on exactly the nine
    `conductor-*` crates.
  - Whether "green for our own crates" means *still skipped* or *now actually checked* is a fork for P4.
  - Either way, `MIT` and `Apache-2.0` both already sit in `deny.toml`'s `allow` list (`:55-56`).

## Boundaries (out of scope)
- This is not a relicensing of dependencies, and there is no third-party NOTICE aggregation.
- No SPDX headers in source files. This follows the operator's "keep it small".
- No README license section. Verified: there is no root README. The only tracked README is
  `crates/conductor-tauri/ui/test/README.md`, a harness doc (`git ls-files | grep -i readme`).
- No change to the `publish = false` posture.
- No Tauri bundle `license` / installer license page. Verified: `tauri.conf.json` carries no license or copyright
  key. The operator enumerated the manifests as Cargo workspace + member crates + `package.json`.

## Folded freight
- CONTEXT (`:52`), folded whole: founder direction 2026-09-29, relayed by the overseer (relay
  `conductor-wrap-50-2026-09-29` §2.1, first in its order after the diagnostic-quality chunk): «надо будет добавить
  лицензии апачи мит» ("we need to add Apache and MIT licenses").
  - Measured at that wrap: no `LICENSE*` file, no tracked license file, no `license` key in any workspace
    `Cargo.toml` or in `crates/conductor-tauri/ui/package.json`.
  - The repository has been public since 2026-09-29.
  - Re-verified at take-up: `ls LICENSE*` returns nothing. There are ten Cargo manifests (root + 9 members) and
    one `package.json`. The `[package]` blocks carry no `license` key.
- CI (Setup 5a): `c97f697` (the 2026-09-29 wrap commit).
  - At take-up it was `in progress`: run CI#36611592537, oldest check the Rust gate at 215 s, 3/3 checks started.
  - Re-read at P4 through the same tool: **`verdict: green` · checks 3/3 · wall 530 s**, CI#36611592537
    completed/success. Nothing to fold.
