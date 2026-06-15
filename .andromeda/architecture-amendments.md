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

## 2026-06-15-conductor-core-shared-types — serde_json 1.0 registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "Serialization (JSON) | serde_json 1.0" row.
**Why:** the conductor-core-shared-types chunk added `serde_json = "1.0"` to `[workspace.dependencies]` (canonical-name round-trip tests now; the run-report envelope + per-run JSONL journal consume it at runtime in Epoch 6); arch's §Stack registry did not list it. Cleared by the security drift-detector (audit-green, allowed by §Dependency Security). Cascaded to `.claude/docs/stack.md`. The design proposal (record an implementation note in the Decisions Log) was rejected as non-drift — design-system.md §cli already prescribes the implemented prefixes.

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Stack and Technologies (Validation row) · §Established Decisions [Validation Library] · §Inherited Defaults (Validation)
**Change:** garde version 0.23.0 → 0.22.1 across all three; §Established Decisions now records that garde 0.22.1's `#[garde(custom)]` is field-level only (no container-level custom) — cross-field invariants spanning distinct fields use garde's `Context` pattern.
**Why:** the config-validation-surface chunk needed garde's `derive` feature, but `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + `derive` is unbuildable; user authorized the downgrade to 0.22.1. The no-duplicate-P-IDs validator consequently landed field-level (not struct-level). Cascaded to `.claude/docs/stack.md`. Cleared by the security + arch drift-detectors (§Dependency Security carried no garde pin).
