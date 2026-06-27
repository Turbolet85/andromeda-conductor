import OperatorChecklist, { type ChecklistItem } from './OperatorChecklist'
import './OperatorChecklistView.css'

// The ManualCheck operator-checklist surface — the OperatorChecklist primitive plus an unticked-count
// footer roll-up (design-system §Component Patterns §7), announced via role="status"/aria-live without
// stealing focus (a11y-plan §4). Presentational: `items` is controlled by the caller.
export default function OperatorChecklistView({
  items,
  onToggle,
}: {
  items: ChecklistItem[]
  onToggle: (id: string, checked: boolean) => void
}) {
  const unticked = items.filter((it) => !it.checked).length
  return (
    <section className="checklist-view" aria-label="Operator checklist">
      <OperatorChecklist items={items} onToggle={onToggle} />
      <p className="checklist-view__footer type-label" role="status" aria-live="polite">
        {unticked === 0 ? 'All observations confirmed' : `${unticked} of ${items.length} unconfirmed`}
      </p>
    </section>
  )
}
