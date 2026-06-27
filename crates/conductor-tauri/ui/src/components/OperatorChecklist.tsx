import './OperatorChecklist.css'

export interface ChecklistItem {
  id: string
  induced: string
  observation: string
  checked: boolean
}

export default function OperatorChecklist({
  items,
  onToggle,
}: {
  items: ChecklistItem[]
  onToggle: (id: string, checked: boolean) => void
}) {
  return (
    <ul className="checklist">
      {items.map((item) => (
        <li key={item.id} className="checklist__row">
          <label className="checklist__label">
            <input
              type="checkbox"
              className="checklist__checkbox"
              checked={item.checked}
              onChange={(e) => onToggle(item.id, e.target.checked)}
            />
            <span className="checklist__text">
              <span className="checklist__induced type-label">{item.induced}</span>
              <span className="checklist__observation type-body">{item.observation}</span>
            </span>
          </label>
        </li>
      ))}
    </ul>
  )
}
