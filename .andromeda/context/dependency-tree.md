# Dependency Tree

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's planned module structure. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-15T15:45:15Z
**Tooling:** `cargo tree --workspace` (the integrity-protocol fallback + canonical crate dependency graph; cargo-modules 0.26 renamed the `generate tree` subcommand, so `cargo tree` is used)
**Source:** arch.md §Inherited Defaults Workspace crates / §Directory structure — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:dep-tree start -->
conductor-cli v0.1.0 (crates/conductor-cli)
└── conductor-core v0.1.0 (crates/conductor-core)
    ├── garde v0.22.1
    │   ├── compact_str v0.8.2
    │   │   ├── castaway v0.2.4
    │   │   │   └── rustversion v1.0.22 (proc-macro)
    │   │   ├── cfg-if v1.0.4
    │   │   ├── itoa v1.0.18
    │   │   ├── rustversion v1.0.22 (proc-macro)
    │   │   ├── ryu v1.0.23
    │   │   └── static_assertions v1.1.0
    │   ├── garde_derive v0.22.1 (proc-macro)
    │   │   ├── proc-macro2 v1.0.106
    │   │   │   └── unicode-ident v1.0.24
    │   │   ├── quote v1.0.45
    │   │   │   └── proc-macro2 v1.0.106 (*)
    │   │   └── syn v2.0.117
    │   │       ├── proc-macro2 v1.0.106 (*)
    │   │       ├── quote v1.0.45 (*)
    │   │       └── unicode-ident v1.0.24
    │   └── smallvec v1.15.2
    ├── serde v1.0.228
    │   ├── serde_core v1.0.228
    │   └── serde_derive v1.0.228 (proc-macro)
    │       ├── proc-macro2 v1.0.106 (*)
    │       ├── quote v1.0.45 (*)
    │       └── syn v2.0.117 (*)
    └── thiserror v2.0.18
        └── thiserror-impl v2.0.18 (proc-macro)
            ├── proc-macro2 v1.0.106 (*)
            ├── quote v1.0.45 (*)
            └── syn v2.0.117 (*)
    [dev-dependencies]
    └── serde_json v1.0.150
        ├── itoa v1.0.18
        ├── memchr v2.8.2
        ├── serde_core v1.0.228
        └── zmij v1.0.21

conductor-core v0.1.0 (crates/conductor-core) (*)

conductor-emit v0.1.0 (crates/conductor-emit)
└── conductor-core v0.1.0 (crates/conductor-core) (*)

conductor-faults v0.1.0 (crates/conductor-faults)
└── conductor-core v0.1.0 (crates/conductor-core) (*)

conductor-report v0.1.0 (crates/conductor-report)
└── conductor-core v0.1.0 (crates/conductor-core) (*)

conductor-tauri v0.1.0 (crates/conductor-tauri)
└── conductor-core v0.1.0 (crates/conductor-core) (*)

conductor-timeline v0.1.0 (crates/conductor-timeline)
└── conductor-core v0.1.0 (crates/conductor-core) (*)

conductor-verify v0.1.0 (crates/conductor-verify)
└── conductor-core v0.1.0 (crates/conductor-core) (*)
<!-- LIVING:dep-tree end -->
