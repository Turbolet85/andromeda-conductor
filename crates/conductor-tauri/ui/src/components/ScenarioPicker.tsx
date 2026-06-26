import { Command } from 'cmdk'
import './ScenarioPicker.css'

// Must match conductor_core::scenario_catalog::SUITE_SELECTION (the backend validates against it).
export const SUITE_SELECTION = '__suite__'

export type ScenarioSummary = {
  name: string
  p_ids: string[]
  slo_tier: string
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
      <Command.List className="picker__list">
        <Command.Empty className="picker__empty type-body">No scenarios match.</Command.Empty>
        <Command.Item
          value={SUITE_SELECTION}
          keywords={['suite', 'all', 'catalog']}
          onSelect={() => onSelect(SUITE_SELECTION)}
          className="picker__item"
          aria-current={selection === SUITE_SELECTION ? 'true' : undefined}
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
