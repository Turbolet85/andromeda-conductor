// The screen-reader pass rows — the one source the SR leg (../screen-reader.e2e.ts) drives and the parser
// (./parse-nvda-log.ts) grades. nvda-pass-spec.md mirrors these ids, and the parser refuses to write
// evidence when an id here is missing from that spec, so the two cannot drift apart silently
// (a11y-plan §3 Screen reader test pattern).
export type Subject = 'live' | 'empty' | 'error'
export type RowClass = 'focus' | 'live' | 'browse'
export type RowState = 'idle' | 'idle-report' | 'live' | 'hold' | 'aborted' | 'terminal'

export interface SpecRow {
  readonly id: string
  readonly subject: Subject
  readonly state: RowState
  readonly cls: RowClass
  readonly item: string
  readonly node: string
  readonly sc: string
  readonly expected: string
  /** Every token must appear (case-insensitive) in the row's heard utterances for announced-as-expected. */
  readonly tokens: readonly string[]
  /** Any of these heard turns the row announced-differently (e.g. `unavailable` on an enabled control). */
  readonly forbidden?: readonly string[]
  /** The node does not exist in the release bundle: recorded subject-absent, no action taken. */
  readonly absent?: string
  /** Deliberately not driven by the leg: recorded not-run-here with this reason. */
  readonly notRun?: string
  /** No token can grade the value (a bare number); heard text is captured for the operator's review. */
  readonly review?: true
  /** The first row of a session: whatever NVDA spoke before the first stamp belongs to it. */
  readonly fromSessionStart?: true
}

const NVDA = 'NVDA default verbosity'

export const ROWS: readonly SpecRow[] = [
  // ── live subject · S0 idle (real trimmed catalog, empty runs dir) ────────────────────────────────
  {
    id: 'S0-11', subject: 'live', state: 'idle', cls: 'browse',
    item: 'titlebar phase line at rest',
    node: 'banner › span[aria-live="polite"] "Conductor · idle"',
    sc: 'SC 4.1.3', expected: `"Conductor · idle" read by navigation (the region announces changes, not initial content)`,
    tokens: ['idle'],
  },
  {
    id: 'S0-12', subject: 'live', state: 'idle', cls: 'browse',
    item: 'titlebar count placeholder',
    node: 'banner › span[aria-live="polite"][aria-label="Scenario count: no run yet"] "00:00:00"',
    sc: 'SC 4.1.2', expected: 'the count is NAMED — "Scenario count: no run yet" — never bare digits',
    tokens: ['Scenario count'],
  },
  {
    id: 'S0-01', subject: 'live', state: 'idle', cls: 'focus',
    item: 'Minimize window control', node: 'button[aria-label="Minimize window"]',
    sc: 'SC 4.1.2', expected: `"Minimize window" + button (${NVDA})`, tokens: ['Minimize window'],
  },
  {
    id: 'S0-02', subject: 'live', state: 'idle', cls: 'focus',
    item: 'Close window control', node: 'button[aria-label="Close window"]',
    sc: 'SC 4.1.2', expected: `"Close window" + button`, tokens: ['Close window'],
  },
  {
    id: 'S0-03', subject: 'live', state: 'idle', cls: 'focus',
    item: 'scenario / suite picker', node: 'input[role="combobox"] named "Scenario or suite picker" (cmdk)',
    sc: 'SC 4.1.2', expected: `"Scenario or suite picker" + combo box, editable, with the first option active`,
    tokens: ['Scenario or suite picker'],
  },
  {
    id: 'S0-16', subject: 'live', state: 'idle', cls: 'browse',
    item: 'picker filter-miss prose',
    node: 'div[aria-live="polite"] › p "No scenarios match." (cmdk Empty replaced — its role="presentation" is set after the prop spread and cannot be overridden)',
    sc: 'SC 4.1.3',
    expected: 'ANNOUNCED when the filter stops matching — the status region is mounted before the text arrives, so the prose is a change and not a mount',
    tokens: ['No scenarios match'],
  },
  {
    id: 'S0-04', subject: 'live', state: 'idle', cls: 'focus',
    item: 'picker option (Arrow Down)', node: '[role="option"] "halo-breathing-encoding · P-026 · <20s"',
    sc: 'SC 4.1.2', expected: 'the option text as the active descendant changes', tokens: ['halo-breathing-encoding'],
  },
  {
    id: 'S0-05', subject: 'live', state: 'idle', cls: 'focus',
    item: 'picker option (Arrow Down)', node: '[role="option"] "halo-hue-encoding · P-025 · <5s"',
    sc: 'SC 4.1.2', expected: 'the option text as the active descendant changes', tokens: ['halo-hue-encoding'],
  },
  {
    id: 'S0-06', subject: 'live', state: 'idle', cls: 'focus',
    item: 'select the suite (Enter on "Suite — all scenarios")', node: '[role="option"][aria-current="true"] with " · selected"',
    sc: 'SC 4.1.2',
    expected: 'the committed choice announced WITH its selected state as it becomes current — the option\'s aria-label carries " · selected", so it no longer waits for a re-read',
    tokens: ['selected'],
  },
  {
    id: 'S0-07', subject: 'live', state: 'idle', cls: 'focus',
    item: 'Start control after a selection', node: 'button "Start" (aria-disabled=false)',
    sc: 'SC 4.1.2', expected: `"Start" + button, NOT unavailable`, tokens: ['Start'], forbidden: ['unavailable'],
  },
  {
    id: 'S0-08', subject: 'live', state: 'idle', cls: 'browse',
    item: 'Stop control while idle', node: 'button "Stop" (native disabled — unfocusable)',
    sc: 'SC 4.1.2', expected: `"Stop" + button + unavailable, by navigation only`, tokens: ['Stop'],
  },
  {
    id: 'S0-09', subject: 'live', state: 'idle', cls: 'focus',
    item: 'coverage rows scroll region', node: 'div[tabindex=0] › table[aria-label="Coverage rows"]',
    sc: 'SC 1.3.1',
    expected: 'the region named "Coverage rows" only — focusing it must NOT read the table\'s rows as one utterance (the role="group" that did is gone)',
    tokens: ['Coverage rows'],
  },
  {
    id: 'S0-13', subject: 'live', state: 'idle', cls: 'browse',
    item: 'heading list (three h2, no h1 — finding)', node: 'h2 "Scenario / suite" · "Coverage matrix" · "Run report"',
    sc: 'SC 1.3.1', expected: 'next-heading navigation reads "Scenario / suite heading level 2"', tokens: ['Scenario / suite'],
  },
  {
    id: 'S0-14', subject: 'live', state: 'idle', cls: 'browse',
    item: 'landmark list (banner · main · two regions; contentinfo absent — finding)', node: 'header (banner) · main · section[aria-label]',
    sc: 'SC 1.3.1', expected: 'next-landmark navigation reads "main landmark"', tokens: ['main'],
  },
  {
    id: 'S0-15', subject: 'live', state: 'idle', cls: 'browse',
    item: 'coverage matrix not-yet-run cell', node: 'td.cov__status "Not yet run" (text, never a tint)',
    sc: 'SC 1.4.1', expected: '"Not yet run" read as a table cell', tokens: ['Not yet run'],
  },
  {
    id: 'S0-10', subject: 'live', state: 'idle', cls: 'browse',
    item: 'run report empty prose', node: 'p "No run yet" (plan prose "No run yet — pick a scenario/suite to begin" — divergence)',
    sc: 'SC 4.1.3', expected: '"No run yet" by navigation', tokens: ['No run yet'],
  },
  // ── live subject · S1 live ────────────────────────────────────────────────────────────────────────
  {
    id: 'S1-01', subject: 'live', state: 'live', cls: 'live',
    item: 'phase line flip on Start', node: 'span[aria-live="polite"] "Conductor · live"',
    sc: 'SC 4.1.3', expected: `"Conductor · live" announced (polite — assertive is reserved for the HOLD flip, so this no longer preempts a focus announcement)`, tokens: ['live'],
  },
  {
    id: 'S1-02', subject: 'live', state: 'live', cls: 'browse',
    item: 'count after Start (a scenario counter, not a clock)',
    node: 'span.titlebar__count[aria-live="polite"][aria-label="Scenarios completed: 0"] "0"',
    sc: 'SC 4.1.3',
    expected: 'ANNOUNCED as it advances — the count is an aria-live region named "Scenarios completed"',
    tokens: ['Scenarios completed'], review: true,
  },
  {
    id: 'S1-05', subject: 'live', state: 'live', cls: 'browse',
    item: 'in-progress prose', node: '(none) — the plan\'s "Run in progress" prose does not ship',
    sc: 'SC 4.1.3', expected: 'subject absent', tokens: ['Run in progress'],
    absent: 'no in-progress prose ships; the live state rides the titlebar label + count only',
  },
  {
    id: 'S1-03', subject: 'live', state: 'live', cls: 'focus',
    item: 'Start control while running', node: 'button "Start" (aria-disabled=true)',
    sc: 'SC 4.1.2', expected: `"Start" + button + unavailable`, tokens: ['Start', 'unavailable'],
  },
  {
    id: 'S1-04', subject: 'live', state: 'live', cls: 'focus',
    item: 'Stop control while running', node: 'button "Stop" (enabled)',
    sc: 'SC 4.1.2', expected: `"Stop" + button, NOT unavailable`, tokens: ['Stop'], forbidden: ['unavailable'],
  },
  // ── live subject · S2 hold ────────────────────────────────────────────────────────────────────────
  {
    id: 'S2-01', subject: 'live', state: 'hold', cls: 'live',
    item: 'phase line flip to HOLD (focus returned to Start first, so the restore target is Start)', node: 'span[aria-live="assertive"] "Conductor · HOLD — operator pause"',
    sc: 'SC 4.1.3', expected: `"HOLD — operator pause" announced (assertive; may interleave with the dialog)`, tokens: ['HOLD'],
  },
  {
    id: 'S2-02', subject: 'live', state: 'hold', cls: 'focus',
    item: 'operator-pause dialog opens, focus on Abort', node: '[role="alertdialog"] labelled "P-026 — operator-checklist" (halo-breathing-encoding holds first in sorted-filename order), described "Observe the operator-checklist claim for this scenario"; Abort focused',
    sc: 'SC 4.1.2 · SC 2.4.3', expected: 'dialog + title + description, then "Abort" + button', tokens: ['operator-checklist', 'Abort'],
  },
  {
    id: 'S2-06', subject: 'live', state: 'hold', cls: 'browse',
    item: 'unticked roll-up at open', node: 'p[role="status"][aria-live="polite"] "1 of 1 unconfirmed"',
    sc: 'SC 4.1.3', expected: 'initial text by navigation (polite region announces changes)', tokens: ['1 of 1 unconfirmed'],
  },
  {
    id: 'S2-03', subject: 'live', state: 'hold', cls: 'focus',
    item: 'Proceed action (Tab)', node: 'button "Proceed"', sc: 'SC 4.1.2', expected: `"Proceed" + button`, tokens: ['Proceed'],
  },
  {
    id: 'S2-04', subject: 'live', state: 'hold', cls: 'focus',
    item: 'checklist row (Tab)', node: 'label › input[type="checkbox"] + induced + observation text',
    sc: 'SC 4.1.2', expected: 'the row text ("… halo breathing rate tracks throughput?") + check box + not checked',
    tokens: ['throughput', 'not checked'],
  },
  {
    id: 'S2-05', subject: 'live', state: 'hold', cls: 'live',
    item: 'Space toggles the row; roll-up updates', node: 'checkbox checked + p[role="status"] "All observations confirmed"',
    sc: 'SC 4.1.3', expected: '"checked" then "All observations confirmed" (polite)', tokens: ['checked', 'All observations confirmed'],
  },
  {
    id: 'S2-07', subject: 'live', state: 'hold', cls: 'focus',
    item: 'Proceed (Enter) closes the dialog; focus restored', node: 'button "Start" regains focus (onCloseAutoFocus → restoreFocusTo)',
    sc: 'SC 2.4.3',
    expected: '"Start" + button (+ unavailable while running) — the restore announcement completes; the phase line leaving hold is polite now and no longer preempts it',
    tokens: ['Start'],
  },
  {
    id: 'S2-08', subject: 'live', state: 'hold', cls: 'live',
    item: 'phase line back to live', node: 'span[aria-live="polite"] "Conductor · live"',
    sc: 'SC 4.1.3', expected: `"Conductor · live" announced`, tokens: ['live'],
  },
  // ── live subject · S3 aborted ─────────────────────────────────────────────────────────────────────
  {
    id: 'S3-01', subject: 'live', state: 'aborted', cls: 'live',
    item: 'Stop during the second scenario (a third follows, so the backend abort check fires)', node: 'span[aria-live="polite"] "Conductor · aborted"',
    sc: 'SC 4.1.3', expected: `"Conductor · aborted" announced`, tokens: ['aborted'],
  },
  {
    id: 'S3-02', subject: 'live', state: 'aborted', cls: 'focus',
    item: 'Start control after Stop', node: 'button "Start" (aria-disabled=false again)',
    sc: 'SC 4.1.2', expected: `"Start" + button, NOT unavailable`, tokens: ['Start'], forbidden: ['unavailable'],
  },
  {
    id: 'S3-03', subject: 'live', state: 'aborted', cls: 'live',
    item: 'second hold still opens (abort is polled between scenarios)', node: 'HOLD flip + [role="alertdialog"] "P-025 — operator-checklist" (halo-hue-encoding, second in sorted order)',
    sc: 'SC 4.1.3', expected: '"HOLD" then the P-025 dialog', tokens: ['HOLD', 'P-025'],
  },
  {
    id: 'S3-04', subject: 'live', state: 'aborted', cls: 'focus',
    item: 'Escape resolves NoGo; focus restored', node: 'button "Start" regains focus',
    sc: 'SC 2.1.2 · SC 2.4.3',
    expected: '"Start" + button — spoken every session; the assertive phase-line flip that cancelled it is scoped to entering hold',
    tokens: ['Start'],
  },
  {
    id: 'S3-05', subject: 'live', state: 'aborted', cls: 'live',
    item: 'terminal Aborted stage settles the phase line', node: 'span[aria-live="polite"] "Conductor · aborted"; report reloads',
    sc: 'SC 4.1.3',
    expected: '"Conductor · aborted" announced and STAYS aborted — a stop during the last scenario now reports the Aborted stage, so the line no longer settles to idle',
    tokens: ['aborted'],
  },
  {
    id: 'S3-06', subject: 'live', state: 'aborted', cls: 'browse',
    item: 'run report rows after the stopped run', node: 'table rows with the "Manual" lamp label',
    sc: 'SC 1.4.1', expected: '"Manual" read as the status cell text', tokens: ['Manual'],
  },
  {
    id: 'S3-07', subject: 'live', state: 'aborted', cls: 'browse',
    item: 'coverage matrix lamp for P-025', node: 'td.cov__status › lamp label "Manual" beside P-025',
    sc: 'SC 1.4.1', expected: '"P-025" … "Manual" by navigation', tokens: ['P-025'],
  },
  {
    id: 'T-01', subject: 'live', state: 'terminal', cls: 'live',
    item: 'un-stopped run settles to idle', node: 'span[aria-live="polite"] "Conductor · idle"',
    sc: 'SC 4.1.3', expected: `"Conductor · idle" announced on the Done stage`, tokens: ['idle'],
    notRun: 'the live subject is stopped by design (aborted needs a Stop); a second un-stopped session is optional',
  },
  // ── empty subject · idle with an empty catalog and the seeded fixture report ──────────────────────
  {
    id: 'E0-01', subject: 'empty', state: 'idle-report', cls: 'browse',
    item: 'empty-catalog prose', node: 'p "No scenarios found." (plan prose "No scenarios loaded" — divergence)',
    sc: 'SC 4.1.3', expected: '"No scenarios found." by navigation', tokens: ['No scenarios found'],
  },
  {
    id: 'E0-02', subject: 'empty', state: 'idle-report', cls: 'focus',
    item: 'Minimize window control', node: 'button[aria-label="Minimize window"]', sc: 'SC 4.1.2',
    expected: `"Minimize window" + button`, tokens: ['Minimize window'],
  },
  {
    id: 'E0-03', subject: 'empty', state: 'idle-report', cls: 'focus',
    item: 'Close window control', node: 'button[aria-label="Close window"]', sc: 'SC 4.1.2',
    expected: `"Close window" + button`, tokens: ['Close window'],
  },
  {
    id: 'E0-04', subject: 'empty', state: 'idle-report', cls: 'focus',
    item: 'Start control with no selection possible', node: 'button "Start" (aria-disabled=true)',
    sc: 'SC 4.1.2', expected: `"Start" + button + unavailable`, tokens: ['Start', 'unavailable'],
  },
  {
    id: 'E0-05', subject: 'empty', state: 'idle-report', cls: 'focus',
    item: 'coverage rows scroll region', node: 'div[role="group"][aria-label="Coverage rows"]',
    sc: 'SC 1.3.1', expected: `"Coverage rows" + grouping`, tokens: ['Coverage rows'],
  },
  {
    id: 'E0-06', subject: 'empty', state: 'idle-report', cls: 'focus',
    item: 'run report rows scroll region (fixture present)', node: 'div[tabindex=0] › table[aria-label="Run report rows"]',
    sc: 'SC 1.3.1',
    expected: 'the region named "Run report rows" only — focusing it must NOT read the table\'s rows as one utterance',
    tokens: ['Run report rows'],
  },
  {
    id: 'E0-07', subject: 'empty', state: 'idle-report', cls: 'browse',
    item: 'run report header', node: 'header "3 scenarios · run lamps-fixture"', sc: 'SC 1.3.1',
    expected: 'the header text by navigation', tokens: ['lamps-fixture'],
  },
  {
    id: 'E0-08', subject: 'empty', state: 'idle-report', cls: 'browse',
    item: 'run report status cells', node: 'td.report__status › lamp labels "Pass" · "Blocked" · "Fail" (glyph aria-hidden)',
    sc: 'SC 1.4.1', expected: '"Blocked" read as text, never a colour', tokens: ['Blocked'],
  },
  {
    id: 'E0-09', subject: 'empty', state: 'idle-report', cls: 'browse',
    item: 'coverage lamp for the collided P-ID', node: 'row P-019 › lamp "Blocked" (worst-lamp-wins over Pass)',
    sc: 'SC 1.4.1', expected: '"P-019" … "Blocked" by navigation', tokens: ['P-019'],
  },
  {
    id: 'E0-10', subject: 'empty', state: 'idle-report', cls: 'browse',
    item: 'run-level load-envelope banner', node: 'p.report__envelope (not rendered — the fixture records no run_envelope row)',
    sc: 'SC 1.4.1', expected: 'subject absent', tokens: ['ENVIRONMENT-SUSPECT'],
    absent: 'the seeded fixture records no run_envelope row; the rendered-DOM proof is v2-25\'s',
  },
  // ── error subject · a malformed catalog ──────────────────────────────────────────────────────────
  {
    id: 'R0-01', subject: 'error', state: 'idle', cls: 'live',
    item: 'scenario load error, re-announced on the first focus event',
    node: 'div[role="alert"] › p (sr-only) "Could not load scenarios: …" — the region mounts EMPTY and the visible copy sits outside it; the re-assertion is inserted once, on the first focusin',
    sc: 'SC 4.1.3',
    expected: '"Could not load scenarios" announced ONCE; the text must carry no host path. NVDA binds a window on its first focus event, which necessarily follows a load-time paint, so this row is stamped between activation and that first Tab — the narrowest window containing the announcement',
    tokens: ['Could not load scenarios'],
  },
  {
    id: 'R0-02', subject: 'error', state: 'idle', cls: 'focus',
    item: 'Minimize window control', node: 'button[aria-label="Minimize window"]', sc: 'SC 4.1.2',
    expected: `"Minimize window" + button`, tokens: ['Minimize window'],
  },
  {
    id: 'R0-03', subject: 'error', state: 'idle', cls: 'focus',
    item: 'Close window control', node: 'button[aria-label="Close window"]', sc: 'SC 4.1.2',
    expected: `"Close window" + button`, tokens: ['Close window'],
  },
  {
    id: 'R0-04', subject: 'error', state: 'idle', cls: 'focus',
    item: 'Start control with nothing loaded', node: 'button "Start" (aria-disabled=true)', sc: 'SC 4.1.2',
    expected: `"Start" + button + unavailable`, tokens: ['Start', 'unavailable'],
  },
]

export function rowsFor(subject: Subject): readonly SpecRow[] {
  return ROWS.filter((r) => r.subject === subject)
}
