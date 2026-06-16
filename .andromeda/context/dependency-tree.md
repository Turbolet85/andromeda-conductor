# Dependency Tree

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's planned module structure. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-15T22:05:00Z
**Tooling:** `cargo tree --workspace` (the integrity-protocol fallback + canonical crate dependency graph; cargo-modules 0.26 renamed the `generate tree` subcommand, so `cargo tree` is used)
**Source:** arch.md §Inherited Defaults Workspace crates / §Directory structure — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:dep-tree start -->
conductor-cli v0.1.0 (D:\dev\projects\conductor\crates\conductor-cli)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core)
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
    ├── serde_json v1.0.150
    │   ├── itoa v1.0.18
    │   ├── memchr v2.8.2
    │   ├── serde_core v1.0.228
    │   └── zmij v1.0.21
    ├── thiserror v2.0.18
    │   └── thiserror-impl v2.0.18 (proc-macro)
    │       ├── proc-macro2 v1.0.106 (*)
    │       ├── quote v1.0.45 (*)
    │       └── syn v2.0.117 (*)
    ├── tracing v0.1.44
    │   ├── pin-project-lite v0.2.17
    │   ├── tracing-attributes v0.1.31 (proc-macro)
    │   │   ├── proc-macro2 v1.0.106 (*)
    │   │   ├── quote v1.0.45 (*)
    │   │   └── syn v2.0.117 (*)
    │   └── tracing-core v0.1.36
    │       └── once_cell v1.21.4
    └── tracing-subscriber v0.3.23
        ├── matchers v0.2.0
        │   └── regex-automata v0.4.14
        │       └── regex-syntax v0.8.11
        ├── nu-ansi-term v0.50.3
        │   └── windows-sys v0.61.2
        │       └── windows-link v0.2.1
        ├── once_cell v1.21.4
        ├── regex-automata v0.4.14 (*)
        ├── serde v1.0.228 (*)
        ├── serde_json v1.0.150 (*)
        ├── sharded-slab v0.1.7
        │   └── lazy_static v1.5.0
        ├── smallvec v1.15.2
        ├── thread_local v1.1.9
        │   └── cfg-if v1.0.4
        ├── tracing v0.1.44 (*)
        ├── tracing-core v0.1.36 (*)
        ├── tracing-log v0.2.0
        │   ├── log v0.4.32
        │   ├── once_cell v1.21.4
        │   └── tracing-core v0.1.36 (*)
        └── tracing-serde v0.2.0
            ├── serde v1.0.228 (*)
            └── tracing-core v0.1.36 (*)

conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)

conductor-emit v0.1.0 (D:\dev\projects\conductor\crates\conductor-emit)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)

conductor-faults v0.1.0 (D:\dev\projects\conductor\crates\conductor-faults)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)

conductor-report v0.1.0 (D:\dev\projects\conductor\crates\conductor-report)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)

conductor-tauri v0.1.0 (D:\dev\projects\conductor\crates\conductor-tauri)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)

conductor-timeline v0.1.0 (D:\dev\projects\conductor\crates\conductor-timeline)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)

conductor-verify v0.1.0 (D:\dev\projects\conductor\crates\conductor-verify)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)
<!-- LIVING:dep-tree end -->
