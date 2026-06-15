# Architecture — Amendments

_Append-only changelog of amendments to `architecture.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1
**Section:** §Stack and Technologies (+ §Infrastructure Patterns Build system · §Inherited Defaults · directory-tree comment)
**Change:** MSRV pinned to 1.94.1 (was 1.88.0) across all occurrences.
**Why:** the cargo-workspace-scaffold chunk implemented the security-plan §Dependency Security required bump (≥1.94.1, tar-rs CVE-2026-33056); arch's stated MSRV 1.88.0 was superseded. The workspace pins build toolchain 1.95.0 with `rust-version = "1.94.1"` as the MSRV floor.

## 2026-06-14-cargo-workspace-scaffold — self-observation stack row added
**Section:** §Stack and Technologies
**Change:** added a "Self-observation | tracing 0.1.44 + tracing-subscriber 0.3.23" row.
**Why:** the chunk pinned `tracing` + `tracing-subscriber` in `[workspace.dependencies]` (the obs-plan §3 self-obs stack); arch's Stack table did not list them. NOT an OTel SDK — OTLP remains the PRODUCT emission.
