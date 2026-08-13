# design extract

## No domain coverage

This chunk is entirely backend Rust (`conductor-run` dispatch layer over `conductor-emit` primitives, `phase_spec.rs` model, scenario TOMLs) with no rendering surface — no webview or CLI output is produced here (`crates/conductor-run` contains zero `println!`/`eprintln!`/`owo-colors`/`indicatif` call sites), so no tokens, typography, motion, iconography, or component patterns apply; per the focus guide, backend/API/IPC is explicitly out of design scope.
