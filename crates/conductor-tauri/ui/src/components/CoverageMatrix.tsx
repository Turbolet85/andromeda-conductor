import { useRef, useState, type KeyboardEvent } from 'react'
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
// verdict-first via lampForRecord) is the per-P-ID run outcome where one exists — absent ⇒ "Not yet run".
// App.tsx builds that map from the run report's journal records (worst-lamp-wins where several name one
// P-ID), so the join reads the same envelope the report table does. Reuses StatusLamp; never re-spells
// the lamp set.
export default function CoverageMatrix({
  rows,
  lamps,
  unbacked = 0,
}: {
  rows: CapabilityRow[]
  lamps?: Record<string, Lamp>
  unbacked?: number
}) {
  // Roving focus: the region is ONE tab stop — the current row — and Arrow/Home/End move it (a11y-plan §5
  // run-console-live / idle-with-report). Component-local, so every consumer's props stay unchanged.
  const [current, setCurrent] = useState(0)
  const rowEls = useRef<Array<HTMLTableRowElement | null>>([])

  const onKeyDown = (event: KeyboardEvent<HTMLTableSectionElement>) => {
    const last = rows.length - 1
    let next: number
    switch (event.key) {
      case 'ArrowDown':
        next = Math.min(current + 1, last)
        break
      case 'ArrowUp':
        next = Math.max(current - 1, 0)
        break
      case 'Home':
        next = 0
        break
      case 'End':
        next = last
        break
      default:
        return
    }
    event.preventDefault()
    setCurrent(next)
    // Every row renders (no virtualization), so focusing an off-screen row scrolls it into view natively.
    rowEls.current[next]?.focus()
  }

  return (
    <section className="cov" aria-label="Capability coverage matrix">
      <header className="cov__summary type-data">{tally(rows, unbacked)}</header>
      {/* The scroll container is NOT a tab stop and carries no role="group": a focusable group computes
          the whole table as its accessible content, so focusing it read all 83 rows in one utterance
          (NVDA pass 2026-09-02, S0-09). The focused ROW is the stop, and the table carries the name. */}
      <div className="cov__scroll">
        <table className="cov__table" aria-label="Coverage rows">
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
          <tbody onKeyDown={onKeyDown}>
            {rows.map((r, i) => {
              const lamp = lamps?.[r.p_id]
              return (
                <tr
                  key={r.p_id}
                  ref={(el) => {
                    rowEls.current[i] = el
                  }}
                  className="cov__row"
                  tabIndex={i === current ? 0 : -1}
                  aria-current={i === current ? 'true' : undefined}
                  // Focus by any means (Tab, click) makes the row current, so mouse and keyboard agree.
                  onFocus={() => setCurrent(i)}
                >
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
