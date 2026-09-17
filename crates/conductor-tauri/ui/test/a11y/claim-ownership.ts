// The a11y-plan §5 keyboard/focus claim population and the suite that owns each one.
//
// Claims are keyed by stable subject identity, never by an a11y-plan line coordinate: §3 Configuration
// requires naming by identity because coordinates move as the document grows, and this file's own chunk
// inherited a CARRY whose offsets had already moved once.
//
// Three states, exhaustive and mutually exclusive. `unasserted` is a recorded finding, never a pass —
// test-plan §1 Untestable zones is the in-plan precedent for stating a gap in place rather than
// inventing a driver for it.

export type Suite = 'routine' | 'driven' | 'sr'

/** The registered suite families over the one WebdriverIO + tauri-driver stack (wdio.conf.ts, test-plan §6). */
export const SUITE_SPEC: Record<Suite, string> = {
  routine: 'accessibility.e2e.ts',
  driven: 'operator-hold.e2e.ts',
  sr: 'screen-reader.e2e.ts',
}

/** How an operator invokes each suite — the handle a carve-out names instead of a line coordinate. */
export const SUITE_INVOCATION: Record<Suite, string> = {
  routine: 'agent-run run --e2e',
  driven: 'npm run a11y:driven',
  sr: 'npm run a11y:sr',
}

/** Suites continuous integration runs. The driven and sr arms need a live Pulse / NVDA and stay operator-local. */
export const CI_SUITES: readonly Suite[] = ['routine']

export type Claim =
  | { claim: string; sc: string; state: 'owned'; owner: Suite; assertedBy: string }
  | { claim: string; sc: string; state: 'n/a-by-construction'; basis: string }
  | { claim: string; sc: string; state: 'unasserted'; gap: string }

export const CLAIMS: readonly Claim[] = [
  {
    claim: 'run-console-idle keyboard reachability',
    sc: 'SC 2.1.1',
    state: 'owned',
    owner: 'routine',
    assertedBy: 'every idle-console control is keyboard-reachable by Tab alone (SC 2.1.1)',
  },
  {
    claim: 'run-console-idle focus order',
    sc: 'SC 2.4.3',
    state: 'owned',
    owner: 'routine',
    assertedBy:
      'idle focus order follows the run-console-idle layout: window controls, then the picker (SC 2.4.3)',
  },
  {
    claim: 'run-console-live coverage-matrix row navigation',
    sc: 'SC 2.1.1',
    state: 'unasserted',
    gap: 'no suite drives Arrow keys against the coverage matrix; the only Arrow assertions in the tree are on the picker listbox, in the sr leg',
  },
  {
    claim: 'run-console-HOLD focus order',
    sc: 'SC 2.4.3',
    state: 'owned',
    owner: 'driven',
    assertedBy:
      'the real HOLD dialog traps focus, Space toggles a row, Escape resolves NoGo and restores focus',
  },
  {
    claim: 'idle-with-report coverage-matrix row navigation',
    sc: 'SC 2.4.3',
    state: 'unasserted',
    gap: 'the report-site row navigation and its aria-selected/aria-current marking are asserted by no suite; the report-site render is itself a route-owned gap',
  },
  {
    claim: 'skip links',
    sc: 'SC 2.4.1',
    state: 'n/a-by-construction',
    basis:
      'single frameless window, no router and no repeated nav blocks to bypass, so SC 2.4.1 has no target on this surface',
  },
  {
    claim: 'run-console-HOLD focus trap',
    sc: 'SC 2.1.2',
    state: 'owned',
    owner: 'driven',
    assertedBy:
      'the real HOLD dialog traps focus, Space toggles a row, Escape resolves NoGo and restores focus',
  },
  {
    claim: 'focus restoration to the triggering control',
    sc: 'SC 2.4.3',
    state: 'owned',
    owner: 'driven',
    assertedBy:
      'the real HOLD dialog traps focus, Space toggles a row, Escape resolves NoGo and restores focus',
  },
  {
    claim: 'no route-change focus surface',
    sc: 'SC 2.4.3',
    state: 'n/a-by-construction',
    basis:
      'the four run-STATES are in-place transitions, not navigations, so no route change exists to move focus on',
  },
  {
    claim: 'per-surface keyboard shortcuts',
    sc: 'SC 2.1.1',
    state: 'unasserted',
    gap: 'no suite drives the first-class start / stop / proceed / abort shortcuts; every Enter and Space in the tree activates an already-focused control',
  },
]

/** The claims that have no target at all — asserted as a SET, so a new construction claim cannot slip in silently. */
export const NA_CLAIMS: readonly string[] = ['skip links', 'no route-change focus surface']
