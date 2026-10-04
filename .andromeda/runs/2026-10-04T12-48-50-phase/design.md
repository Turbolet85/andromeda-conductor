# design extract

## No domain coverage
The chunk is test-surface and lint-policy work with no rendered surface: a behaviour-preserving split of `delegated_timing_harvest.rs`, mutation kills in `conductor-emit/src/identity.rs` and `conductor-run/src/canary.rs` (test additions, no output change), and an obs-plan §10 clippy wording CARRY. It touches no desktop-webview token, no cli surface output (per design-system §Surface: cli), and no component pattern. A design-system.md sweep finds no mention of clippy, mutation, harvest, canary or the span-identity code.
