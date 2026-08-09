import StatusLamp from './StatusLamp'
import type { Lamp } from '../lamp'
import './CoverageMatrix.css'

// Mirrors conductor-core CoverageMode's serde wire spellings (coverage.rs) — the command returns these.
export type CoverageMode = 'auto' | 'drive+observe' | 'static-only' | 'not-conductors'

// Mirrors conductor-core CapabilityRow (the `coverage_matrix` command's row shape).
export interface CapabilityRow {
  p_id: string
  title: string
  category: string
  mode: CoverageMode
}

// Every mode is counted, so the per-mode counts always sum to rows.length (a11y-plan §4, SC 4.1.3 —
// no capability silently uncounted). Mirrors conductor-core CoverageMode::ALL's order.
const MODES: CoverageMode[] = ['auto', 'drive+observe', 'static-only', 'not-conductors']

const OUT_OF_SCOPE: CoverageMode = 'not-conductors'

// The roll-up: the full row count, the in-scope subtotal with its per-mode breakdown, then the
// out-of-scope count as its own token. Rows outside Conductor's remit were never in play, so folding
// them into an undifferentiated denominator would read as unmeasured work. Mirrors the Markdown +
// cli roll-up shape.
//
// `unbacked` qualifies the auto term — auto claims no scenario yet backs. It comes from the
// `unbacked_auto` command (conductor-core's ledger, held to the catalog by check_scenario_backing),
// never a TS copy. A qualifier, never a fifth summand: those rows are still classified auto.
function tally(rows: CapabilityRow[], unbacked: number): string {
  const by = (m: CoverageMode) => rows.filter((r) => r.mode === m).length
  const inScope = MODES.filter((m) => m !== OUT_OF_SCOPE)
    .map((m) => (m === 'auto' && unbacked > 0 ? `${by(m)} ${m} (${unbacked} unbacked)` : `${by(m)} ${m}`))
    .join(' · ')
  const out = by(OUT_OF_SCOPE)
  return `${rows.length} capabilities · ${rows.length - out} in scope (${inScope}) · ${out} ${OUT_OF_SCOPE}`
}

// The dense single-row-per-P-ID coverage wall (design-system §Component Patterns §3 — instrument-panel
// density, never a card grid). Presentational: `rows` is the classification; `lamps` (p_id → Lamp,
// verdict-first via lampForRecord) is the per-P-ID run outcome where one exists — absent ⇒ "Not yet run"
// (the live per-P-ID runs.db join lands in Epoch 10). Reuses StatusLamp; never re-spells the lamp set.
export default function CoverageMatrix({
  rows,
  lamps,
  unbacked = 0,
}: {
  rows: CapabilityRow[]
  lamps?: Record<string, Lamp>
  unbacked?: number
}) {
  return (
    <section className="cov" aria-label="Capability coverage matrix">
      <header className="cov__summary type-data">{tally(rows, unbacked)}</header>
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
                  <td
                    className={
                      r.mode === OUT_OF_SCOPE
                        ? 'cov__mode cov__mode--out-of-scope type-data'
                        : 'cov__mode type-data'
                    }
                  >
                    {r.mode}
                  </td>
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
