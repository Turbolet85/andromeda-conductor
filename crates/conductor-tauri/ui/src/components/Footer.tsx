import { Fragment, type ReactNode } from 'react'
import StatusLamp from './StatusLamp'
import type { RunState } from './Titlebar'
import { LAMP_ORDER, lampForRecord, type RunRecord } from '../lamp'
import './Footer.css'

const PHASE_WORD: Record<RunState, string> = {
  idle: 'idle',
  live: 'live',
  hold: 'HOLD',
  aborted: 'aborted',
}

// The window's status strip — the contentinfo landmark (layout-templates §Component — Footer (status
// strip)). Presentational over state App already holds: the seed rides the loaded report's records, the
// roll-up counts each lamp through lampForRecord and renders its label via StatusLamp, never a tint
// alone. Not focusable and not a live region: the titlebar's phase line is the one announced state.
export default function Footer({ runState, records }: { runState: RunState; records: RunRecord[] }) {
  const seed = records.length > 0 ? String(records[0].seed) : '—'
  const tally = LAMP_ORDER.map((lamp) => ({
    lamp,
    n: records.filter((r) => lampForRecord(r.state, r.verdict) === lamp).length,
  })).filter((t) => t.n > 0)

  const segments: Array<{ key: string; node: ReactNode }> = [
    { key: 'app', node: 'conductor' },
    { key: 'seed', node: `seed ${seed}` },
    {
      key: 'phase',
      node: (
        <>
          {/* The HOLD accent rides a glyph beside the word: --count-hold is under 4.5:1 as small text on
              the light --color-base, and the word itself is the signal. */}
          {runState === 'hold' ? (
            <span className="footer__hold-glyph" aria-hidden="true">
              ●{' '}
            </span>
          ) : null}
          {PHASE_WORD[runState]}
        </>
      ),
    },
    ...tally.map(({ lamp, n }) => ({
      key: lamp,
      node: (
        <>
          {n} <StatusLamp lamp={lamp} size="sm" />
        </>
      ),
    })),
  ]

  // One text line with plain-text separators: separate flex items carried no whitespace between them, and
  // NVDA read the strip as "conductorseed —idle" (measured 2026-09-30).
  return (
    <footer className="footer type-label">
      <span className="footer__line">
        {segments.map(({ key, node }, i) => (
          <Fragment key={key}>
            {i > 0 ? ' · ' : null}
            {node}
          </Fragment>
        ))}
      </span>
    </footer>
  )
}
