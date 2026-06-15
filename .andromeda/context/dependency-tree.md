# Dependency Tree

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's planned module structure. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout each wrap (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-14T23:46:47Z
**Tooling:** `cargo tree --workspace` (the integrity-protocol fallback + canonical crate dependency graph; cargo-modules 0.26 renamed the `generate tree` subcommand, so `cargo tree` is used)
**Source:** arch.md §Inherited Defaults Workspace crates / §Directory structure — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:dep-tree start -->
conductor-cli v0.1.0 (crates/conductor-cli)
└── conductor-core v0.1.0 (crates/conductor-core)

conductor-core v0.1.0 (crates/conductor-core)

conductor-emit v0.1.0 (crates/conductor-emit)
└── conductor-core v0.1.0 (crates/conductor-core)

conductor-faults v0.1.0 (crates/conductor-faults)
└── conductor-core v0.1.0 (crates/conductor-core)

conductor-report v0.1.0 (crates/conductor-report)
└── conductor-core v0.1.0 (crates/conductor-core)

conductor-tauri v0.1.0 (crates/conductor-tauri)
└── conductor-core v0.1.0 (crates/conductor-core)

conductor-timeline v0.1.0 (crates/conductor-timeline)
└── conductor-core v0.1.0 (crates/conductor-core)

conductor-verify v0.1.0 (crates/conductor-verify)
└── conductor-core v0.1.0 (crates/conductor-core)
<!-- LIVING:dep-tree end -->
