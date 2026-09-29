
## 2026-09-29-dual-license-mit-or-apache-2-0 — the dual license
**Section:** §Infrastructure Patterns → Directory structure · §Infrastructure Patterns → Build system
**Change:**
- Directory structure: the root-file set gains `LICENSE-MIT` and `LICENSE-APACHE`, one tree row naming them as a set — the dual license every Cargo + npm manifest declares, `MIT OR Apache-2.0`.
- Build system: `deny.toml`'s license policy now also covers the workspace's OWN `conductor-*` crates — no `private` exemption. Each passes as `MIT OR Apache-2.0`, inherited from `[workspace.package]` (`license.workspace = true` in every member), so a member without an allowed license fails `cargo deny check licenses`; `publish = false` exempts nothing.
**Why:** The repository went public and was dual-licensed on the founder's direction. The own-crate check was the overseer's founder-delegated ruling: an exempted gate proves nothing about our crates. The member set, every `name@version` and `Cargo.lock` stay unchanged.
**Ref:** .andromeda/runs/2026-09-29T19-16-23-wrap/
