# Release build + bundle — recorded artifacts

All figures measured this chunk, 2026-09-10, on the Windows dev host. Every gate ran from the
workspace root with its exit read from the bare command, never through a pipe.

## Supply chain — run BEFORE the release build, in this order

| # | gate | exit | reading |
|---|---|---|---|
| 1 | `git -C $CARGO_HOME/advisory-db status --porcelain` | 0 | **no output** — the local checkout is clean |
| 2 | `cargo audit` (bare) | 0 | **1243 advisories · 562 packages · 7 allowed warnings** |
| 3 | `cargo deny check advisories bans licenses sources` | 0 | `advisories ok, bans ok, licenses ok, sources ok` |
| 4 | `cargo metadata --locked` | 0 | the lock agrees with the bumped manifests |

The currency probe read `D:\dev\rust\cargo\advisory-db` — **the same copy `cargo audit` then loaded
from** (its own line: `Loaded 1243 security advisories (from D:\dev\rust\cargo\advisory-db)`), which
is what makes the clean-porcelain reading evidence about the audit rather than about some other copy.
`cargo deny` reports its now-inert ignore entries as `no crate matched advisory criteria`; that is the
accepted-exception set shrinking, not a gate change.

Figures are **re-read, not carried forward**: 1243 · 562 · 7 matches the 2026-09-10 reading recorded
in `security.md`, and the count is read from the log's own summary line rather than a prior date's
clause.

## Version stamp — 0.1.0 → 0.2.0 (operator decision, P4 review)

Moved in both places together, as the plan required:

- `Cargo.toml` `[workspace.package] version`
- `crates/conductor-tauri/tauri.conf.json` `version`

`Cargo.lock` regenerated: **exactly 9 lines changed**, all `version = "0.1.0"` → `"0.2.0"`, one per
workspace member. **Package count 562 → 562, unchanged** — no package added or removed, which is the
basis the dependency-delta rule asks for (never lockfile byte-identity). The three `0.1.0` lines that
remain are third-party (`leb128fmt`, `vswhom`, `windows-threading`) and are not Conductor's.

Observable at all three predicted sites:

- `./target/release/conductor --version` → `conductor 0.2.0`
- every self-obs line → `"service.version":"0.2.0"` (`obs.rs:51`, `env!("CARGO_PKG_VERSION")`)
- both installer filenames → `Conductor_0.2.0_…`

## Build

| gate | exit | reading |
|---|---|---|
| `( cd ui && npm run build )` | 0 | `dist/` rebuilt this run; fonts vendored locally (WOFF2, no CDN) |
| `cargo build --release --workspace` | 0 | `Finished release [optimized] in 42.42s`; compiled `conductor-tauri v0.2.0` |
| `cargo build --release -p conductor-tauri --features tauri/custom-protocol` | 0 | `Finished release [optimized] in 23.85s` — the FEATURE, not the profile, is what serves the bundle |

## Bundle

Producing tool: **`tauri-cli 2.11.4`**, installed this chunk via `cargo install tauri-cli --locked`
(a host dev-tool FLOOR, the cargo-audit / cargo-deny / cargo-llvm-cov class — not a lockfile pin).
Recorded so the floor stays auditable later; architecture registers the bundler line at 2.11.3, and
2.11.4 satisfies it as a floor.

`cargo tauri build` run bare from the workspace root — the CLI resolved the workspace member's
`tauri.conf.json` itself, and ran `beforeBuildCommand` (`npm run build`), which is the behaviour
`architecture.md:204` describes as belonging to the Tauri CLI alone. No cwd change was needed.

| artifact | bytes | built |
|---|---|---|
| `target/release/bundle/msi/Conductor_0.2.0_x64_en-US.msi` | 6 152 192 (5.87 MB) | 2026-09-10 21:31 |
| `target/release/bundle/nsis/Conductor_0.2.0_x64-setup.exe` | 4 418 544 (4.21 MB) | 2026-09-10 21:31 |


**Second reading, taken by the wrap's light gate (2026-09-10 23:19 local) — the nsis byte count is NOT reproducible.** The gate re-ran the same `cargo tauri build` over unchanged source and emitted nsis **4 414 280 B** (4.21 MB), 4 264 bytes below the row above; the msi came out byte-identical at 6 152 192 B. Both nsis readings round to the same 4.21 MB, so the SCALE reproduces and the byte count does not. The table above is left as the record of the /implement build rather than corrected — it is a true measurement of that build, and the point of this note is precisely that a single byte-exact figure is the wrong shape for this artifact. Consequence carried into the specs at the same wrap: `architecture.md:206` now states the MB pair and names the variance, instead of the byte pair it briefly carried.

`bundle.targets = "all"` on this Windows-only host yields the Windows installer set (msi + nsis),
which is what the plan anticipated. Both are under the gitignored `target/`, so neither rides a commit.

**Minor spec-vs-reality observation (not blocking, no gate reads it):** `architecture.md:206` and
§Deployment model describe "an optional **~3 MB** GUI installer". The measured artifacts are 4.21 MB
and 5.87 MB. The descriptor is approximate and governs nothing, but it is now measurably low —
surfaced for wrap rather than corrected here, since implement authors no spec change.
