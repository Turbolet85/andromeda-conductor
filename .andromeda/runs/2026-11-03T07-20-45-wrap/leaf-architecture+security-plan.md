group: architecture+security-plan
read: .claude/docs/stack.md · 56 · «1. Update `architecture.md` §Stack first → 2. update `Cargo.»

- leaf: .claude/docs/stack.md:8   class: stale
  leaf-text: «**tokio 1.52.3** — `current_thread` flavor»
  source: fct-fjnaazvl «| Async runtime | tokio 1.54.1 (`current_thread` flavor) | Deterministic single-threaded scheduler for the timeline engine; core-owned runtime |»
  replace-with: «**tokio 1.54.1** — `current_thread` flavor»

- leaf: .claude/docs/stack.md:43   class: omission
  leaf-text: «**CI:** GitHub Actions — `cargo fmt --all --check`»
  source: fct-4ega3gmt «**CI/CD approach:** GitHub Actions on an `ubuntu-latest` runner runs one job, `rust`: `cargo fmt --all --check`»
  replace-with: «**CI:** GitHub Actions on an `ubuntu-latest` runner (one job, `rust`) — `cargo fmt --all --check`»
