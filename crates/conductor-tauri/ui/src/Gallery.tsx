import { useState } from 'react'
import StatusLamp from './components/StatusLamp'
import OperatorPauseDialog from './components/OperatorPauseDialog'
import OperatorChecklistView from './components/OperatorChecklistView'
import { type ChecklistItem } from './components/OperatorChecklist'
import CoverageMatrix, { type CapabilityRow } from './components/CoverageMatrix'
import RunReport from './components/RunReport'
import { LAMP_ORDER, lampForRecord, type Lamp, type RunRecord } from './lamp'

const SAMPLE_ROWS: CapabilityRow[] = [
  { p_id: 'P-005', title: 'Span Status Error Detection', category: 'Hard Signal Detection', mode: 'auto' },
  {
    p_id: 'P-001',
    title: 'Receiver Lifecycle State',
    category: 'Connection & Health Awareness',
    mode: 'drive+observe',
  },
  { p_id: 'P-049', title: 'Encryption at Rest', category: 'Privacy & Trust', mode: 'static-only' },
]

// Exercises lampForRecord (verdict-first): P-005 measured Pass, P-001 Blocked; P-049 omitted ⇒ "Not yet run".
const SAMPLE_LAMPS: Record<string, Lamp> = {
  'P-005': lampForRecord('Pass', 'Pass'),
  'P-001': lampForRecord('Blocked', null),
}

// Spans all six verdict-first lamps: Pass, Fail, Hold (CalibrationRegion beats the ManualCheck state),
// Residual (KnownResidual state precedes the measured verdict), Manual (ManualCheck, no verdict), and
// the Blocked null-row (— in every measurement column).
const SAMPLE_RECORDS: RunRecord[] = [
  {
    journal_emitted_at: '2026-06-16T21:10:06Z',
    read_back_observed_at: '2026-06-16T21:10:07Z',
    run_id: '2026-06-16T21-10-06-demo',
    seed: 424242,
    scenario: 'error-baseline-spike',
    p_ids: ['P-009', 'P-010'],
    verdict: 'Pass',
    state: 'Pass',
    latency_ms: 1840,
    slo_tier: '<5s',
    fingerprints: ['fp-1'],
  },
  {
    journal_emitted_at: '2026-06-16T21:10:08Z',
    read_back_observed_at: '2026-06-16T21:10:11Z',
    run_id: '2026-06-16T21-10-06-demo',
    seed: 424242,
    scenario: 'severity-boundary',
    p_ids: ['P-007'],
    verdict: 'Fail',
    state: 'Fail',
    latency_ms: 2900,
    slo_tier: '<5s',
    fingerprints: [],
  },
  {
    journal_emitted_at: '2026-06-16T21:10:12Z',
    read_back_observed_at: '2026-06-16T21:10:36Z',
    run_id: '2026-06-16T21-10-06-demo',
    seed: 424242,
    scenario: 'severity-choice',
    p_ids: ['P-019'],
    verdict: 'CalibrationRegion',
    state: 'ManualCheck',
    latency_ms: 24000,
    slo_tier: '<90s',
    fingerprints: [],
  },
  {
    journal_emitted_at: '2026-06-16T21:10:37Z',
    read_back_observed_at: '2026-06-16T21:10:39Z',
    run_id: '2026-06-16T21-10-06-demo',
    seed: 424242,
    scenario: 'context-grounding',
    p_ids: ['P-032'],
    verdict: 'Pass',
    state: 'KnownResidual',
    latency_ms: 2100,
    slo_tier: '<20s',
    fingerprints: ['fp-resid'],
  },
  {
    journal_emitted_at: '2026-06-16T21:10:40Z',
    read_back_observed_at: null,
    run_id: '2026-06-16T21-10-06-demo',
    seed: 424242,
    scenario: 'constellation-hue',
    p_ids: ['P-025'],
    verdict: null,
    state: 'ManualCheck',
    latency_ms: null,
    slo_tier: '<90s',
    fingerprints: null,
  },
  {
    journal_emitted_at: null,
    read_back_observed_at: null,
    run_id: '2026-06-16T21-10-06-demo',
    seed: 424242,
    scenario: 'port-occupier',
    p_ids: ['P-003'],
    verdict: null,
    state: 'Blocked',
    latency_ms: null,
    slo_tier: '<20s',
    fingerprints: null,
  },
]

// DEV-only render-all gallery for visual + type inspection of the component primitives. Mounted from
// main.tsx behind `import.meta.env.DEV` (+ #gallery); tree-shaken from the production bundle.
export default function Gallery() {
  const [dialogOpen, setDialogOpen] = useState(false)
  const [lastDecision, setLastDecision] = useState<string | null>(null)
  const [items, setItems] = useState<ChecklistItem[]>([
    {
      id: 'restart',
      induced: 'Restart the Pulse process',
      observation: 'A RestartEvent surfaces within 20s',
      checked: false,
    },
    {
      id: 'breathing',
      induced: 'Drive halo-breathing at peak load',
      observation: 'The constellation breathes amber',
      checked: true,
    },
  ])

  const toggle = (id: string, checked: boolean) =>
    setItems((prev) => prev.map((it) => (it.id === id ? { ...it, checked } : it)))

  return (
    <div
      style={{
        minHeight: '100vh',
        background: 'var(--color-base)',
        color: 'var(--text-primary)',
        fontFamily: 'var(--font-sans)',
        padding: 'var(--space-xl)',
        display: 'flex',
        flexDirection: 'column',
        gap: 'var(--space-xl)',
      }}
    >
      <h1 className="type-heading" style={{ margin: 0 }}>
        Component primitives gallery
      </h1>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Status lamps
        </h2>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 'var(--space-lg)' }}>
          {LAMP_ORDER.map((lamp) => (
            <StatusLamp key={lamp} lamp={lamp} />
          ))}
        </div>
      </section>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Operator-pause dialog
        </h2>
        <button
          type="button"
          className="dialog__btn dialog__btn--proceed"
          style={{ alignSelf: 'flex-start' }}
          onClick={() => setDialogOpen(true)}
        >
          Open dialog
        </button>
        {lastDecision ? (
          <p className="type-body" role="status" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
            Last decision: {lastDecision}
          </p>
        ) : null}
        <OperatorPauseDialog
          open={dialogOpen}
          onOpenChange={setDialogOpen}
          title="P-025 — observe-hue"
          body="Observe the constellation hue for this scenario, then proceed or abort."
          onProceed={() => {
            setLastDecision('Go')
            setDialogOpen(false)
          }}
          onAbort={() => {
            setLastDecision('No-Go')
            setDialogOpen(false)
          }}
        />
      </section>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Operator checklist
        </h2>
        <OperatorChecklistView items={items} onToggle={toggle} />
      </section>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Coverage matrix
        </h2>
        <CoverageMatrix rows={SAMPLE_ROWS} lamps={SAMPLE_LAMPS} />
      </section>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Run report
        </h2>
        <RunReport records={SAMPLE_RECORDS} />
      </section>
    </div>
  )
}
