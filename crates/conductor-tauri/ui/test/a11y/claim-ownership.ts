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

/**
 * `inPlace` states what CI gates in place of an owner CI does not run (a11y-plan §11 Strategy). The checker
 * requires it for every such carve-out, so the obligation cannot be lost by leaving the field out.
 */
export type Claim =
  | { claim: string; sc: string; state: 'owned'; owner: Suite; assertedBy: string; inPlace?: string }
  | { claim: string; sc: string; state: 'n/a-by-construction'; basis: string }
  | { claim: string; sc: string; state: 'unasserted'; gap: string }

// The driven arm's one live run carries every hold-dependent and live-state claim, so they share its title.
const DRIVEN_LIVE_RUN =
  'one live run: Ctrl+Enter starts it, its coverage matrix row-navigates while live and the count never takes focus, the HOLD dialog traps focus, Space toggles a row, Escape resolves No-Go and restores focus, Ctrl+. stops it and Ctrl+Enter proceeds the next hold'

const ROUTINE_ROW_NAVIGATION =
  'coverage-matrix rows navigate by Arrow keys and Home/End through one tab stop, the current row marked aria-current with a --border-emphasis edge (idle-with-report)'

const ROUTINE_KEY_MAP =
  'the console declares its shortcut map: aria-keyshortcuts on Start and Stop and a visible hint line'

const HOLD_IN_PLACE =
  "a11y-plan §11 Strategy: the routine arm's expected-skip SET of two (agent-run run --e2e), this enumeration and its checker (npm run a11y:ownership)"

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
    state: 'owned',
    owner: 'driven',
    assertedBy: DRIVEN_LIVE_RUN,
    inPlace: `the routine arm's row-navigation spec ("${ROUTINE_ROW_NAVIGATION}") over the SAME CoverageMatrix component, which App.tsx renders in every run state`,
  },
  {
    claim: 'run-console-HOLD focus order',
    sc: 'SC 2.4.3',
    state: 'owned',
    owner: 'driven',
    assertedBy: DRIVEN_LIVE_RUN,
    inPlace: HOLD_IN_PLACE,
  },
  {
    claim: 'idle-with-report coverage-matrix row navigation',
    sc: 'SC 2.4.3',
    state: 'owned',
    owner: 'routine',
    assertedBy: ROUTINE_ROW_NAVIGATION,
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
    assertedBy: DRIVEN_LIVE_RUN,
    inPlace: HOLD_IN_PLACE,
  },
  {
    claim: 'focus restoration to the triggering control',
    sc: 'SC 2.4.3',
    state: 'owned',
    owner: 'driven',
    assertedBy: DRIVEN_LIVE_RUN,
    inPlace: HOLD_IN_PLACE,
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
    state: 'owned',
    owner: 'driven',
    assertedBy: DRIVEN_LIVE_RUN,
    inPlace: `the routine arm's key-map declaration spec ("${ROUTINE_KEY_MAP}"): the declared map and its visible hint, since a Start keypress there would start a real run`,
  },
  {
    claim: 'visible focus ring on the active element',
    sc: 'SC 2.4.7',
    state: 'owned',
    owner: 'routine',
    assertedBy:
      'the focused control shows a visible --color-focus ring and the control it left shows none (SC 2.4.7)',
  },
]

/** The claims that have no target at all — asserted as a SET, so a new construction claim cannot slip in silently. */
export const NA_CLAIMS: readonly string[] = ['skip links', 'no route-change focus surface']
