import StatusLamp from './StatusLamp'
import type { Lamp } from '../lamp'
import './CoverageMatrix.css'

// Mirrors conductor-core CoverageMode's serde wire spellings (coverage.rs) — the command returns these.
export type CoverageMode = 'auto' | 'drive+observe' | 'static-only'

// Mirrors conductor-core CapabilityRow (the `coverage_matrix` command's row shape).
export interface CapabilityRow {
  p_id: string
  title: string
  category: string
  mode: CoverageMode
}

function tally(rows: CapabilityRow[]): string {
  const by = (m: CoverageMode) => rows.filter((r) => r.mode === m).length
  return `${rows.length} capabilities · ${by('auto')} auto · ${by('drive+observe')} drive+observe · ${by('static-only')} static-only`
}

// The dense single-row-per-P-ID coverage wall (design-system §Component Patterns §3 — instrument-panel
// density, never a card grid). Presentational: `rows` is the classification; `lamps` (p_id → Lamp,
// verdict-first via lampForRecord) is the per-P-ID run outcome where one exists — absent ⇒ "Not yet run"
// (the live per-P-ID runs.db join lands in Epoch 10). Reuses StatusLamp; never re-spells the lamp set.
export default function CoverageMatrix({
  rows,
  lamps,
}: {
  rows: CapabilityRow[]
  lamps?: Record<string, Lamp>
}) {
  return (
    <section className="cov" aria-label="Capability coverage matrix">
      <header className="cov__summary type-data">{tally(rows)}</header>
      <div className="cov__scroll" tabIndex={0} role="group" aria-label="Coverage rows">
        <table className="cov__table">
          <thead>
            <tr>
              <th scope="col" className="type-label">
                P-ID
              </th>
              <th scope="col" className="type-label">
                Capability
              </th>
              <th scope="col" className="type-label">
                Mode
              </th>
              <th scope="col" className="type-label">
                Status
              </th>
            </tr>
          </thead>
          <tbody>
            {rows.map((r) => {
              const lamp = lamps?.[r.p_id]
              return (
                <tr key={r.p_id} className="cov__row">
                  <td className="cov__pid type-data">{r.p_id}</td>
                  <td className="cov__cap">
                    <span className="cov__title type-label">{r.title}</span>
                    <span className="cov__cat type-label">{r.category}</span>
                  </td>
                  <td className="cov__mode type-data">{r.mode}</td>
                  <td className="cov__status">
                    {lamp ? (
                      <StatusLamp lamp={lamp} size="sm" />
                    ) : (
                      <span className="cov__unrun type-body">Not yet run</span>
                    )}
                  </td>
                </tr>
              )
            })}
          </tbody>
        </table>
      </div>
    </section>
  )
}
