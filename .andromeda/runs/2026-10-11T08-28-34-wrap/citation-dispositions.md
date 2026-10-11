sweep: base: 355a6788 — the previous sweep (2026-10-08T10-43-15-wrap) · re-pointed 1 · changed 4 · stretched 0

Written by the tool: `.andromeda/test-plan.md:426` `.github/workflows/ci.yml:157 → 143` (moved; the `--lcov` report line,
read at `ci.yml:143` in the work tree).

held .andromeda/architecture.md:20 Cargo.toml:154 — refused at 2026-10-08T07-22-11-wrap, not touched since → holds — stands. The citing sentence reads "Version-matched to the SUT's own pin (`andromeda-pulse Cargo.toml:154`)": it cites Pulse's manifest, another repository's file, never this tree's `Cargo.toml` (84 lines here). Not re-read in the sibling at this wrap — this chunk touched neither the blake3 pin nor the citing row; the 2026-10-08T10-43-15-wrap disposition's live read stands.
moved .andromeda/test-plan.md:426 .github/workflows/ci.yml:158 → 144 — listed, not written: naked → holds — re-pointed by hand to 144 (read: `ci.yml:144` is the `--cobertura` report line carrying `--ignore-filename-regex '[\\/]tests[\\/]'`).
moved .andromeda/test-plan.md:426 .github/workflows/ci.yml:161 → 147 — listed, not written: naked → holds — re-pointed by hand to 147 (read: `ci.yml:147` is the `--fail-under-lines 60` line-floor gate carrying the same regex).
changed .claude/rules/verification-harness.md:58@c4442 accessibility.e2e.ts:295 → ? — the file is gone «return {» → leaf — re-derived
changed .claude/rules/verification-harness.md:58@c4469 accessibility.e2e.ts:301 → ? — the file is gone · naked: bound by position → leaf — re-derived
changed .claude/rules/verification-harness.md:58@c4513 accessibility.e2e.ts:239 → ? — the file is gone · naked: bound by position → leaf — re-derived
changed .claude/rules/verification-harness.md:58@c4521 accessibility.e2e.ts:251 → ? — the file is gone · naked: bound by position → leaf — re-derived

The leaf `.claude/rules/verification-harness.md` joins cascade step 3's set at this wrap (its line 58 cites the deleted
`accessibility.e2e.ts`). Unresolved 48: another repository's files — no disposition owed.
