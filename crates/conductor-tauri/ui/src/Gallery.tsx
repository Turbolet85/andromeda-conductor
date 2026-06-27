import { useState } from 'react'
import StatusLamp from './components/StatusLamp'
import OperatorPauseDialog from './components/OperatorPauseDialog'
import OperatorChecklist, { type ChecklistItem } from './components/OperatorChecklist'
import CoverageMatrix, { type CapabilityRow } from './components/CoverageMatrix'
import { LAMP_ORDER, lampForRecord, type Lamp } from './lamp'

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

// DEV-only render-all gallery for visual + type inspection of the component primitives. Mounted from
// main.tsx behind `import.meta.env.DEV` (+ #gallery); tree-shaken from the production bundle.
export default function Gallery() {
  const [dialogOpen, setDialogOpen] = useState(false)
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
        <OperatorPauseDialog
          open={dialogOpen}
          onOpenChange={setDialogOpen}
          title="Operator pause — restart-suppression"
          body="Restart the Pulse process, then confirm the RestartEvent was observed before proceeding."
          onProceed={() => setDialogOpen(false)}
          onAbort={() => setDialogOpen(false)}
        />
      </section>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Operator checklist
        </h2>
        <OperatorChecklist items={items} onToggle={toggle} />
      </section>

      <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-md)' }}>
        <h2 className="type-label" style={{ margin: 0, color: 'var(--text-tertiary)' }}>
          Coverage matrix
        </h2>
        <CoverageMatrix rows={SAMPLE_ROWS} lamps={SAMPLE_LAMPS} />
      </section>
    </div>
  )
}
