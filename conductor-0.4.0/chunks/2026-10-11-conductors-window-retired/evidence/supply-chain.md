# Supply chain — 2026-10-11-conductors-window-retired

Run dir `.andromeda/runs/2026-10-11T07-59-46-implement`, gate entries 20-23, second firing (after the
`Apache-2.0 WITH LLVM-exception` comment was re-worded; the first firing read the same verdicts). Read over the
pruned `Cargo.lock` (268 packages, `evidence/lock-delta.md`).

## Advisory-db copy (`$CARGO_HOME/advisory-db`)
- Porcelain before `cargo audit` (entry 20): empty, exit 0.
- Porcelain after `cargo audit` (entry 22): empty, exit 0.
- Currency, read live: local `HEAD` `7eebec69c352c7191b1f13eb95dd510eeca5d1de` equals
  `git ls-remote https://github.com/RustSec/advisory-db.git main` → `7eebec69c352c7191b1f13eb95dd510eeca5d1de`
  (commit dated 2026-10-09T10:12:02+02:00).

## `cargo audit` (entry 21)
- Exit 0. `Loaded 1296 security advisories` · `Scanning Cargo.lock for vulnerabilities (268 crate dependencies)`.
- No vulnerability and no allowed warning printed: the sixteen `unmaintained` advisories the retired ignores
  covered named crates that are no longer in the lock.

## `cargo deny check advisories bans licenses sources` (entry 23)
- Exit 0. Summary line: `advisories ok, bans ok, licenses ok, sources ok`.
- No `advisory-not-detected` (the ignore list is empty) and no `license-not-encountered`.
- Warnings: 2, both `warning[duplicate]` (`hashbrown`, `winnow`), from `[bans] multiple-versions = "warn"`.
  Neither is an advisory or license finding.
- **`Apache-2.0 WITH LLVM-exception`: still encountered, so the allow stays** (plan step 5). `cargo deny` printed
  no `license-not-encountered` for it, and `cargo metadata --locked --offline` over the new tree lists 16
  packages carrying it, 15 names, each an OR-expression: `blake3`, `linux-raw-sys`, `rustix`, `wasi`, `wasip2`,
  `wasip3`, `wasm-encoder`, `wasm-metadata`, `wasmparser`, `wit-bindgen` (two versions), `wit-bindgen-core`,
  `wit-bindgen-rust`, `wit-bindgen-rust-macro`, `wit-component`, `wit-parser`. Its comment, which named the Tauri
  tree, now names these holders.
- `MPL-2.0`: 0 holders in the new graph (same read); the allow was removed with a dated retirement comment.
