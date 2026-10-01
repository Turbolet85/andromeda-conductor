# p5-dryrun-2.txt — moved out of the committed tree

`p5-dryrun-2.txt` was this phase run's second P5 transcript of the gate tool's authoring dry-run over the chunk's
`## Test Commands` (`gate.py run --plan … --dry-run`, 45 lines, taken after a plan revision). It was moved
2026-10-01 at the /implement operator pass because the hygiene gate (`gate.py hygiene`, plan entry 36) refused it —
18 host-path forms (drive and MSYS) — and the overseer ruled (founder-delegated) to follow the 2026-09-30 precedent:
never committed, still traceable.

- sha256 (unchanged by the move): `96a7d53a7a264bffd8e72e163bdf9e52804f7e19c43d3a65e765bb85979bcd43`
- Now at: `.andromeda/cache/p5-controls/2026-10-01T18-42-55-phase/p5-dryrun-2.txt` (gitignored by
  `/.andromeda/cache/`; local to this host, not versioned).
- What carries the forms, described in words: REAL host paths in the tool's own header — the line naming the
  resolved shell binary, the line naming the repository root, and the line naming the gate's per-run log directory
  under the user's temp directory. The per-entry lines below echo the plan's own commands.
