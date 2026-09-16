- pattern: a chunk lands a NEW committed, runtime-parsed artifact under `contracts/` whose ONLY reader is a
    crate-local TEST binary resolving the workspace root from `CARGO_MANIFEST_DIR` — no shipped reader at all,
    `default_path()` a hard-coded relative constant, `load()` taking an ALREADY-RESOLVED path so the module never
    calls `resolve_under`, no `CONDUCTOR_*` override handle, absent/malformed a hard `CoreError` — and a security
    detector proposes registering it as a §Input Validation boundary row
  verdict: routine
  note: apply BOTH halves — the boundary row (what to validate + how) AND the row's per-READER record naming the
    test binary. §Input Validation is an EXHAUSTIVE enumeration of external-input surfaces, and a committed artifact
    parsed at runtime earns its row whether or not a SHIPPED binary is the one parsing it; the duty binds per READER,
    so an unrecorded reader is the drift. This is the NARROWER sibling of the 2026-08-10 rule above, which governs the
    same class only when the artifact is "read at a fixed `default_path()` through `resolve_under`" — a clause that
    FAILS here, because there is no shipped reader to resolve it and the not-operator-steerable property holds by the
    hard-coded constant instead. That clause failing is what sent both occurrences of this shape to an escalation.
    Second occurrence with an identical resolution: `contracts/pulse-capabilities.toml`'s `conductor-report --test
    coverage_gate` reader (2026-09-06-coverage-completeness-gate, escalated, operator-resolved as "record the new
    reader", no rule minted — which is why the second escalated too) and `contracts/scenario-audit-ledger.toml`'s
    `conductor-core --test scenario_audit_gate` reader (2026-09-16-scenario-assertion-audit-gate). Confirmed with the
    user on 2026-09-16. The dismissal never covers a SHIPPED reader or an operator-steerable path — those stay the
    2026-08-10 rule's and the escalating boundary class's respectively.
