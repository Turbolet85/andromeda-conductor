- pattern: a chunk adds a POSTURE-CONDITIONAL grading arm to an existing `ANDROMEDA_PULSE_*` READ-SET handle — the
    same declaration-only read of Conductor's own environment, gaining a requirement that ONLY NARROWS what passes
    (an absence requirement under a named posture) — and a security detector proposes updating the §Input Validation
    READ SET row and its dependent restatements
  verdict: routine
  note: apply — nothing new crosses the boundary. The value still never becomes a path, an argv element or a log
    value, the probe still renders handle NAMES only, and the arm can only ADD refusals (a union truthiness rule over
    an absence arm widens what BLOCKS, never what passes). Check rule `Boundary widening` FIRST and defer to it when
    the arm would let anything new cross — a value leaving the read, a relaxed guard, a new crossing. First
    occurrence: the L4 handle read for ABSENCE under the real-model posture (`handle_declared_for` over
    `flag_declared_on_either_side`, 2026-09-22-interpretation-proven-live), applied under the plan's P5-approved
    direction with no governing rule; minted on the operator's approval at that wrap (2026-09-23).
- pattern: a chunk lands a NEW declarative key inside an already-registered scenario-config artifact
    (`scenarios/*.toml`) that is NOT garde-validated but VALIDATED AT LOAD by a closed unit-enum parse (no
    `#[serde(other)]` catch-all) raising `CoreError::Config` on an unknown value — the field marked `#[garde(skip)]`
    because the parse IS the validation — and D-security-input (or D-arch-resources) proposes registering it as a
    §Input Validation boundary / §Conventions config-key
  verdict: routine
  note: apply — the sibling of the scenario new-key rule above, whose "garde-validated at load" qualifier this class
    fails by construction. A closed unit enum is not the nested-spec `skip` the dive rule bans: garde has nothing to
    descend into, and serde rejects every value outside the set at load. The no-catch-all clause is the whole
    discriminator — a `#[serde(other)]` (or any fallback) variant makes the enum OPEN while the words still match, so
    an enum carrying one is NO MATCH and escalates (operator's sharpening, 2026-09-23). The row names the closed set
    and the load-time fault. First occurrence: `l4_posture` (`L4Posture` {`deterministic`, `real-model`},
    2026-09-22-interpretation-proven-live); minted on the operator's approval at that wrap (2026-09-23).
- pattern: a chunk adds a CLI flag or a harness selector whose VALUE is an EXISTING validated input class — a
    scenario NAME resolved through the catalog's validated `resolve_under` load, or a token from a closed allowlist
    that exits non-zero on anything else — and a security detector proposes the §Input Validation CLI-arguments row
    update
  verdict: routine
  note: apply ONLY when the flag's EFFECT adds none of rule `Boundary widening`'s crossings — no write on a read-only
    command, no new subprocess or IPC crossing, no new value reaching an argv, a shell or a path. Where it does,
    `Boundary widening` governs and it escalates: this rule's precondition constrains the flag's VALUE, and the value
    alone never settles it (operator's guard, 2026-09-23). First occurrence: `conductor preconditions --for
    <SCENARIO>` (a scenario name; a `P-NNN` value refused at parse, exit 2) and the harness's `run --live
    [real-model]` selector (2026-09-22-interpretation-proven-live), applied under the plan's P5-approved direction with
    no governing rule; minted on the operator's approval at that wrap (2026-09-23).
