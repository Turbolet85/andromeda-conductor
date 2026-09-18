- pattern: a chunk lands a NEW committed artifact under `contracts/` with NO Rust reader AT ALL — no `default_path()`,
    no `resolve_under` load path, no bounds check, no `CONDUCTOR_*` override handle; addressed outward to the SUT or
    forward to a later chunk rather than parsed at runtime — and D-arch-resources proposes registering it in arch
    §Occupied Resources — On-disk artifacts
  verdict: routine
  note: apply. §Occupied Resources registers each `contracts/` member INDIVIDUALLY, unlike `scenarios/`, which arch
    tracks at directory grain — so the per-file over-reach dismissal (the scenario-config rule) does not govern here,
    and a genuinely new on-disk artifact earns its own row stating the regime it takes and what its `provenance`
    claims. The two adjacent runtime-parsed rules do NOT match either: both carry a "read at a fixed `default_path()`
    through `resolve_under`" qualifier that a reader-less member fails, and a rule governs only when every qualifying
    clause holds. The registration carries a DEPENDENT edit wherever the existing members' prose states a UNIQUENESS
    claim about the reader-less regime ("the one member no Rust code reads") — retire it by naming the SET or the new
    count's basis, never by substituting a fresh literal that re-stales; the claim restates itself in the §Infrastructure
    directory tree and in the CLAUDE.md / `docs/conventions.md` distillations the cascade re-derives. A historical
    ORDINAL ("the FIRST member with no Rust reader") is not a uniqueness claim and stays as written. Confirmed with the
    user on 2026-09-18 (2026-09-18-real-model-leg-posture-and-grading-rule wrap). n=2 for the class:
    `contracts/pulse-p025-measurement-contract.md` (2026-09-13, escalated with no rule minted) and
    `contracts/pulse-real-model-leg-posture.md` (this chunk) — the recurrence condition for minting.
