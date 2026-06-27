import StatusLamp from './StatusLamp'
import { lampForRecord, type RunRecord } from '../lamp'
import './RunReport.css'

const latencyCell = (r: RunRecord): string => (r.latency_ms === null ? '—' : `${r.latency_ms} ms`)
const fingerprintCell = (r: RunRecord): string =>
  r.fingerprints === null ? '—' : String(r.fingerprints.length)

// Per-scenario verdict lines for one run — the desktop twin of the Markdown run report / the CLI
// results table. Presentational: `records` is the run's JSONL journal (the run-report envelope).
// Reuses StatusLamp via the verdict-first lampForRecord projection; a Blocked row shows — in its
// measurement columns (never the failure red — Blocked is "never measured", not a Fail).
export default function RunReport({ records }: { records: RunRecord[] }) {
  return (
    <section className="report" aria-label="Run report">
      <header className="report__summary type-data">
        {records.length} {records.length === 1 ? 'scenario' : 'scenarios'}
        {records.length > 0 ? ` · run ${records[0].run_id}` : ''}
      </header>
      <div className="report__scroll" tabIndex={0} role="group" aria-label="Run report rows">
        <table className="report__table">
          <thead>
            <tr>
              <th scope="col" className="type-label">
                P-ID
              </th>
              <th scope="col" className="type-label">
                Scenario
              </th>
              <th scope="col" className="type-label">
                Status
              </th>
              <th scope="col" className="type-label">
                Latency
              </th>
              <th scope="col" className="type-label">
                SLO
              </th>
              <th scope="col" className="type-label">
                FP
              </th>
            </tr>
          </thead>
          <tbody>
            {records.map((r) => (
              <tr key={`${r.run_id}:${r.scenario}`} className="report__row">
                <td className="report__pid type-data">{r.p_ids.join(' ')}</td>
                <td className="report__scenario type-body">{r.scenario}</td>
                <td className="report__status">
                  <StatusLamp lamp={lampForRecord(r.state, r.verdict)} size="sm" />
                </td>
                <td className="report__latency type-data">{latencyCell(r)}</td>
                <td className="report__slo type-data">{r.slo_tier}</td>
                <td className="report__fp type-data">{fingerprintCell(r)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  )
}
