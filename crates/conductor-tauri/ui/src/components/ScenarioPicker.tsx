import { Command, useCommandState } from 'cmdk'
import './ScenarioPicker.css'

// Must match conductor_core::scenario_catalog::SUITE_SELECTION (the backend validates against it).
export const SUITE_SELECTION = '__suite__'

export type ScenarioSummary = {
  name: string
  p_ids: string[]
  slo_tier: string
}

// `Command.Empty` hard-codes role="presentation" AFTER spreading props (cmdk 1.1.1), so the prose is
// unnameable and never announced (NVDA pass 2026-09-02, S0-16) and a passed `role` cannot win. The
// region is instead always mounted and empty, so the message arrives as a CHANGE a live region reports
// — a region that mounts together with its text announces nothing.
// It renders OUTSIDE Command.List: the list is a `listbox`, whose only permitted children are options,
// so a region nested there is an `aria-required-children` violation. `aria-live` rather than
// `role="status"` — see the note in Titlebar.tsx.
function FilterMiss() {
  const matches = useCommandState((state) => state.filtered.count)
  return (
    <div aria-live="polite">
      {matches === 0 ? <p className="picker__empty type-body">No scenarios match.</p> : null}
    </div>
  )
}

export default function ScenarioPicker({
  scenarios,
  selection,
  onSelect,
}: {
  scenarios: ScenarioSummary[]
  selection: string | null
  onSelect: (value: string) => void
}) {
  return (
    <Command label="Scenario or suite picker" className="picker">
      <Command.Input
        className="picker__input type-body"
        placeholder="Filter scenarios by name or P-ID…"
      />
      <FilterMiss />
      <Command.List className="picker__list">
        <Command.Item
          value={SUITE_SELECTION}
          keywords={['suite', 'all', 'catalog']}
          onSelect={() => onSelect(SUITE_SELECTION)}
          className="picker__item"
          aria-current={selection === SUITE_SELECTION ? 'true' : undefined}
          // aria-current alone speaks as the bare word "current" and the ' · selected' text was reached
          // only on re-navigation (S0-06/S1-03); folding it into the name announces the committed
          // choice when the option becomes current.
          aria-label={`Suite — all scenarios · ${scenarios.length} scenarios${
            selection === SUITE_SELECTION ? ' · selected' : ''
          }`}
        >
          <span className="type-label">Suite — all scenarios</span>
          <span className="picker__meta type-data">
            {scenarios.length} scenarios{selection === SUITE_SELECTION ? ' · selected' : ''}
          </span>
        </Command.Item>
        {scenarios.map((s) => (
          <Command.Item
            key={s.name}
            value={s.name}
            keywords={s.p_ids}
            onSelect={() => onSelect(s.name)}
            className="picker__item"
            aria-current={selection === s.name ? 'true' : undefined}
            aria-label={`${s.name} · ${s.p_ids.join(' · ')} · ${s.slo_tier}${
              selection === s.name ? ' · selected' : ''
            }`}
          >
            <span className="type-label">{s.name}</span>
            <span className="picker__meta type-data">
              {s.p_ids.join(' · ')} · {s.slo_tier}
              {selection === s.name ? ' · selected' : ''}
            </span>
          </Command.Item>
        ))}
      </Command.List>
    </Command>
  )
}
