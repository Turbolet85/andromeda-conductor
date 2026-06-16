# Dependency Tree

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's planned module structure. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-16T19:50:40Z
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
        │       ├── aho-corasick v1.1.4
        │       │   └── memchr v2.8.2
        │       ├── memchr v2.8.2
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
    [dev-dependencies]
    ├── insta v1.48.0
    │   ├── console v0.16.3
    │   │   ├── encode_unicode v1.0.0
    │   │   ├── libc v0.2.186
    │   │   └── windows-sys v0.61.2 (*)
    │   ├── once_cell v1.21.4
    │   ├── similar v2.7.0
    │   └── tempfile v3.27.0
    │       ├── fastrand v2.4.1
    │       ├── getrandom v0.4.2
    │       │   └── cfg-if v1.0.4
    │       ├── once_cell v1.21.4
    │       └── windows-sys v0.61.2 (*)
    ├── proptest v1.11.0
    │   ├── bit-set v0.8.0
    │   │   └── bit-vec v0.8.0
    │   ├── bit-vec v0.8.0
    │   ├── bitflags v2.13.0
    │   ├── num-traits v0.2.19
    │   │   [build-dependencies]
    │   │   └── autocfg v1.5.1
    │   ├── rand v0.9.4
    │   │   └── rand_core v0.9.5
    │   │       └── getrandom v0.3.4
    │   │           └── cfg-if v1.0.4
    │   ├── rand_chacha v0.9.0
    │   │   ├── ppv-lite86 v0.2.21
    │   │   │   └── zerocopy v0.8.52
    │   │   └── rand_core v0.9.5 (*)
    │   ├── rand_xorshift v0.4.0
    │   │   └── rand_core v0.9.5 (*)
    │   ├── regex-syntax v0.8.11
    │   ├── rusty-fork v0.3.1
    │   │   ├── fnv v1.0.7
    │   │   ├── quick-error v1.2.3
    │   │   ├── tempfile v3.27.0 (*)
    │   │   └── wait-timeout v0.2.1
    │   ├── tempfile v3.27.0 (*)
    │   └── unarray v0.1.4
    └── rstest v0.26.1
        ├── futures-timer v3.0.4
        ├── futures-util v0.3.32
        │   ├── futures-core v0.3.32
        │   ├── futures-macro v0.3.32 (proc-macro)
        │   │   ├── proc-macro2 v1.0.106 (*)
        │   │   ├── quote v1.0.45 (*)
        │   │   └── syn v2.0.117 (*)
        │   ├── futures-task v0.3.32
        │   ├── pin-project-lite v0.2.17
        │   └── slab v0.4.12
        └── rstest_macros v0.26.1 (proc-macro)
            ├── cfg-if v1.0.4
            ├── glob v0.3.3
            ├── proc-macro-crate v3.5.0
            │   └── toml_edit v0.25.12+spec-1.1.0
            │       ├── indexmap v2.14.0
            │       │   ├── equivalent v1.0.2
            │       │   └── hashbrown v0.17.1
            │       ├── toml_datetime v1.1.1+spec-1.1.0
            │       ├── toml_parser v1.1.2+spec-1.1.0
            │       │   └── winnow v1.0.3
            │       └── winnow v1.0.3
            ├── proc-macro2 v1.0.106 (*)
            ├── quote v1.0.45 (*)
            ├── regex v1.12.4
            │   ├── aho-corasick v1.1.4 (*)
            │   ├── memchr v2.8.2
            │   ├── regex-automata v0.4.14 (*)
            │   └── regex-syntax v0.8.11
            ├── relative-path v1.9.3
            ├── syn v2.0.117 (*)
            └── unicode-ident v1.0.24
            [build-dependencies]
            └── rustc_version v0.4.1
                └── semver v1.0.28
[dev-dependencies]
├── assert_cmd v2.2.2
│   ├── anstyle v1.0.14
│   ├── bstr v1.12.1
│   │   ├── memchr v2.8.2
│   │   └── regex-automata v0.4.14 (*)
│   ├── predicates v3.1.4
│   │   ├── anstyle v1.0.14
│   │   ├── difflib v0.4.0
│   │   ├── float-cmp v0.10.0
│   │   │   └── num-traits v0.2.19 (*)
│   │   ├── normalize-line-endings v0.3.0
│   │   ├── predicates-core v1.0.10
│   │   └── regex v1.12.4 (*)
│   ├── predicates-core v1.0.10
│   ├── predicates-tree v1.0.13
│   │   ├── predicates-core v1.0.10
│   │   └── termtree v0.5.1
│   └── wait-timeout v0.2.1
├── assert_fs v1.1.4
│   ├── anstyle v1.0.14
│   ├── globwalk v0.9.1
│   │   ├── bitflags v2.13.0
│   │   ├── ignore v0.4.26
│   │   │   ├── crossbeam-deque v0.8.6
│   │   │   │   ├── crossbeam-epoch v0.9.18
│   │   │   │   │   └── crossbeam-utils v0.8.21
│   │   │   │   └── crossbeam-utils v0.8.21
│   │   │   ├── globset v0.4.18
│   │   │   │   ├── aho-corasick v1.1.4 (*)
│   │   │   │   ├── bstr v1.12.1 (*)
│   │   │   │   ├── log v0.4.32
│   │   │   │   ├── regex-automata v0.4.14 (*)
│   │   │   │   └── regex-syntax v0.8.11
│   │   │   ├── log v0.4.32
│   │   │   ├── memchr v2.8.2
│   │   │   ├── regex-automata v0.4.14 (*)
│   │   │   ├── same-file v1.0.6
│   │   │   │   └── winapi-util v0.1.11
│   │   │   │       └── windows-sys v0.61.2 (*)
│   │   │   ├── walkdir v2.5.0
│   │   │   │   ├── same-file v1.0.6 (*)
│   │   │   │   └── winapi-util v0.1.11 (*)
│   │   │   └── winapi-util v0.1.11 (*)
│   │   └── walkdir v2.5.0 (*)
│   ├── predicates v3.1.4 (*)
│   ├── predicates-core v1.0.10
│   ├── predicates-tree v1.0.13 (*)
│   └── tempfile v3.27.0 (*)
└── predicates v3.1.4 (*)

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
├── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)
├── rand_chacha v0.9.0 (*)
├── rand_core v0.9.5 (*)
├── thiserror v2.0.18 (*)
├── tokio v1.52.3
│   ├── pin-project-lite v0.2.17
│   └── tokio-macros v2.7.0 (proc-macro)
│       ├── proc-macro2 v1.0.106 (*)
│       ├── quote v1.0.45 (*)
│       └── syn v2.0.117 (*)
└── tracing v0.1.44 (*)
[dev-dependencies]
├── insta v1.48.0 (*)
├── proptest v1.11.0 (*)
├── rstest v0.26.1 (*)
└── tokio v1.52.3 (*)

conductor-verify v0.1.0 (D:\dev\projects\conductor\crates\conductor-verify)
└── conductor-core v0.1.0 (D:\dev\projects\conductor\crates\conductor-core) (*)
<!-- LIVING:dep-tree end -->
