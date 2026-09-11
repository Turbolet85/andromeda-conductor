# Conductor 0.3.0 — vision

_Derived from `conductor-0.3.0-incubator/intent.md` (operator-authored 2026-09-11). That file is the
authoritative version intent; this is its framing, not a second source._

## The problem this version advances

0.2.0 closed complete — done-test 32/32 at `b54e6ec`, 31 verified plus one deferred, unclaimed 0. That
ledger is honest about what it measured, and precisely there lies the problem: **four of its closures rest
on a narrowing or a pin rather than on a measurement.** A capability deferred on a candidate list, a
delegated budget routed forward because its instrument could not resolve it, scenario checks that cannot
pass by construction, and a live-proof corpus that exercises plumbing while the interpretation it is meant
to verify stays pinned in a drift set.

None of these is a regression. Each is a compromise taken deliberately, recorded, and left standing —
which is exactly why nothing downstream will surface them again. 0.3.0 exists to retire them.

## Who this is for, and why now

Conductor's single operator, immediately before attention moves to Pulse. The version's whole reason for
existing is that a harness left in this state would keep reporting green over claims it never measured, and
the cost of that is paid later and by someone reading the ledger rather than the evidence. The intent states
the bar directly: switching attention to Pulse must leave **nothing here that is true only by wording**.

## What is in

- **The a11y CI gate reaches an honest terminal** (`v2-24`, the one deferred capability). The hosted-runner
  cause is *measured* rather than listed as candidates — and the capability then either runs green in CI or
  is closed as a ratified permanent exclusion resting on that measured cause. Separately, the requirement
  and the suites are made to agree about where keyboard and focus-order coverage actually lives.
- **Committed scenarios stop carrying checks that cannot pass.** The structurally-dead assertion class is
  retired *as a class in one pass*, pinning tests updated in the same change; every scenario's SLO tier
  either fits its own phase duration or states why it does not; and the result is auditable by a mechanical
  check a fresh reader can re-run.
- **The fourth delegated budget gets an instrument.** Conductor states the measurement contract P-025 needs
  in a form Pulse can implement, and re-drives the grade once Pulse emits it.
- **One live leg proves interpretation, not plumbing.** Deterministic L4 off, a known root cause injected,
  the top hypothesis asserted to identify it — with the non-determinism handled as a stated property of the
  leg, never by retrying until it passes.

## What is out

Named in the intent so the intake does not mint work for them: **`incident_events`-through-MCP** (a
PULSE-side capability gap — no MCP tool surfaces that table at any width, already routed to Pulse's 0.4.0
incubator; it stays a residual here), **Pulse's own `P-075` bookkeeping** (a decision in Pulse's ledger, not
Conductor work), and **release/bundle work** (done in 0.2.0; owed only if a 0.3.0 change moves it).

## What "0.3.0 done" means

Every capability below is `verified` or explicitly `deferred` with a named owner and a measured basis — and
for this version specifically, **no capability closes on a narrowing that a later reader would have to
re-derive.** Where a bound genuinely cannot be measured, the record says what would measure it and who owns
that; where an assertion cannot pass, it is retired rather than left standing as decoration. The blocked
half of the hue-budget work is expected to remain blocked on Pulse and is routed as such — a version that
ends with that entry still `BLOCKED-ON` its named clearing event is a version that closed honestly.
