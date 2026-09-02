import StatusLamp from './StatusLamp'
import { lampForRecord, type RunRecord } from '../lamp'
import './RunReport.css'

const latencyCell = (r: RunRecord): string => (r.latency_ms === null ? '—' : `${r.latency_ms} ms`)
const fingerprintCell = (r: RunRecord): string =>
  r.fingerprints === null ? '—' : String(r.fingerprints.length)

// The run's load-envelope standing as the `run_envelope` command projects it. `label` is carried from
// conductor-core's EnvelopeStatus::label, never re-spelled here (the lamp-mirror rule applied to the
// qualifier's text).
export interface EnvelopeStanding {
  label: string
  cause: string | null
  suspect: boolean
}

// Per-scenario verdict lines for one run — the desktop twin of the Markdown run report / the CLI
// results table. Presentational: `records` is the run's JSONL journal (the run-report envelope).
// Reuses StatusLamp via the verdict-first lampForRecord projection; a Blocked row shows — in its
// measurement columns (never the failure red — Blocked is "never measured", not a Fail).
//
// `envelope` is the run-level qualifier the cli caption and the Markdown banner already carry: it
// renders OUTSIDE the lamp column and is never a seventh lamp (the not-conductors Mode-cell
// precedent), and is omitted entirely when the run stayed in-envelope.
export default function RunReport({
  records,
  envelope,
}: {
  records: RunRecord[]
  envelope?: EnvelopeStanding | null
}) {
  return (
    <section className="report" aria-label="Run report">
      {envelope?.suspect ? (
        <p className="report__envelope type-label">
          <span className="report__envelope-label">{envelope.label}</span>
          {envelope.cause ? <span className="report__envelope-cause">{envelope.cause}</span> : null}
        </p>
      ) : null}
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
