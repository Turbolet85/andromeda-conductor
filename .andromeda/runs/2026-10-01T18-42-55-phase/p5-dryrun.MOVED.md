# p5-dryrun.txt — moved out of the committed tree

`p5-dryrun.txt` was this phase run's P5 transcript of the gate tool's authoring dry-run over the chunk's
`## Test Commands` (`gate.py run --plan … --dry-run`, 45 lines). It was moved 2026-10-01 at the /implement operator
pass because the hygiene gate (`gate.py hygiene`, plan entry 36) refused it — 18 host-path forms (drive and MSYS) —
and the overseer ruled (founder-delegated) to follow the 2026-09-30 precedent: never committed, still traceable.

- sha256 (unchanged by the move): `285bb3cece4d390c73938751bd271cc7225a05fe1a202632d15e8bf607302809`
- Now at: `.andromeda/cache/p5-controls/2026-10-01T18-42-55-phase/p5-dryrun.txt` (gitignored by
  `/.andromeda/cache/`; local to this host, not versioned).
- What carries the forms, described in words: unlike the 2026-09-30 control these are REAL host paths, in the
  tool's own header — the line naming the resolved shell binary, the line naming the repository root, and the line
  naming the gate's per-run log directory under the user's temp directory. The 38 per-entry lines below them echo
  the plan's own commands, whose repo-relative and MSYS-root paths (the sibling Pulse tree) are the plan's text.
