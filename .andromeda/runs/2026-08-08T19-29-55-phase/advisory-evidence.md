# Advisory evidence — captured 2026-08-08T19:2x UTC (phase P1 grounding)

Tooling: cargo-audit 0.22.1 · cargo-deny 0.19.4

## `cargo audit` — 3 vulnerabilities, 19 allowed warnings
| Advisory | Package | Title | Patched | Path |
|---|---|---|---|---|
| RUSTSEC-2026-0204 | crossbeam-epoch 0.9.18 | Invalid pointer deref in `fmt::Pointer` for `Atomic`/`Shared` | `>=0.9.20` | crossbeam-deque ← ignore ← globwalk ← **assert_fs (DEV-dep)** |
| RUSTSEC-2026-0194 | quick-xml 0.39.4 | Quadratic runtime on duplicate-attribute check (CVSS 7.5 A:H) | `>=0.41.0` | plist ← tauri-utils ← tauri / tauri-build (build-dep) |
| RUSTSEC-2026-0195 | quick-xml 0.39.4 | Unbounded namespace-declaration alloc in `NsReader` (CVSS 7.5 A:H) | `>=0.41.0` | plist ← tauri-utils ← tauri / tauri-build (build-dep) |

## `cargo deny check advisories` — FAILED, 4 errors
The 3 vulnerabilities above **plus** one deny-only error (audit reports it as a warning, deny denies it):

| Advisory | Package | Class | Title | Patched |
|---|---|---|---|---|
| RUSTSEC-2026-0190 | anyhow 1.0.102 | `unsound` | Unsoundness in `Error::downcast_mut()` | `>=1.0.103` |

## Corrections to the route entry / handoff premises
1. **RUSTSEC-2026-0190 is `anyhow`, not `crossbeam-epoch`** — and it is an `unsound` advisory, invisible to
   `cargo audit`'s exit code, caught only by `cargo deny` (confirms the 2026-06-23 session learning).
2. **crossbeam-epoch is NOT in the Tauri tree** — it reaches the graph only through `assert_fs`, a
   **dev-dependency** (test fixtures). Never compiled into the release binary.
3. **quick-xml 0.39 → 0.41 needs NO Tauri bump.** The major jump is absorbed by `plist` (a caret-compatible
   MINOR bump that `tauri-utils 2.9.3` already accepts). `tauri-utils` is already at its latest (2.9.3).

## Feasibility — verified by `cargo update --dry-run`
| Command | Effect |
|---|---|
| `cargo update -p plist` | plist 1.9.0 → **1.10.0**, quick-xml 0.39.4 → **0.41.0** (2 packages) |
| `cargo update -p crossbeam-epoch` | crossbeam-epoch 0.9.18 → **0.9.20** (1 package) |
| `cargo update -p anyhow` | anyhow 1.0.102 → **1.0.104** (1 package) |

`cargo update -p quick-xml` alone locks **0 packages** — plist 1.9.0 pins `^0.39`, so plist is the lever.
All three are lock-only bumps: no `Cargo.toml` edit required, no `deny.toml` exception, no Tauri major.
plist 1.10.0 declares `rust-version: 1.88.0` (≤ workspace MSRV 1.94.1).

## Open item carried into research (P3)
`cargo audit` lists a SECOND unsound advisory — RUSTSEC-2024-0429 (glib 0.18.5) — which did **not** appear
among cargo-deny's 4 errors and is **not** in `deny.toml`'s ignore list. Confirm why before declaring the
gate green, so it cannot surface as a latent 5th failure.
