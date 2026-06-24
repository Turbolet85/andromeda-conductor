import { useEffect, useState } from 'react'
import Titlebar, { type RunState } from './components/Titlebar'

const DEV_CYCLE: readonly RunState[] = ['idle', 'live', 'hold', 'live', 'aborted']

const STATUS_TIERS = [
  { label: 'Pass', token: '--count-nominal' },
  { label: 'HOLD', token: '--count-hold' },
  { label: 'Fail', token: '--status-fail' },
  { label: 'Blocked', token: '--count-blocked' },
  { label: 'Manual', token: '--status-manual' },
  { label: 'Residual', token: '--status-residual' },
] as const

const TYPE_TIERS = [
  { role: 'Display', cls: 'type-display' },
  { role: 'Heading', cls: 'type-heading' },
  { role: 'Body', cls: 'type-body' },
  { role: 'Label', cls: 'type-label' },
  { role: 'Code', cls: 'type-code' },
  { role: 'Data', cls: 'type-data' },
] as const

const SURFACES = [
  '--color-base',
  '--color-raised-1',
  '--color-raised-2',
  '--color-raised-3',
  '--color-inset',
] as const

export default function App() {
  const [runState, setRunState] = useState<RunState>('live')

  // DEV-only: cycle the titlebar run-state to exercise the hold-point signature
  // (idle → live → hold → live → aborted) until the live Channel lands (Epoch 9 ch4).
  useEffect(() => {
    if (!import.meta.env.DEV) return
    let i = 0
    const onKey = (e: KeyboardEvent) => {
      if (e.key !== '`') return
      i = (i + 1) % DEV_CYCLE.length
      setRunState(DEV_CYCLE[i])
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [])

  return (
    <div
      style={{
        height: '100vh',
        display: 'flex',
        flexDirection: 'column',
        background: 'var(--color-base)',
        color: 'var(--text-primary)',
        fontFamily: 'var(--font-sans)',
      }}
    >
      <Titlebar runState={runState} count="00:00:00" />

      <main
        style={{
          flex: 1,
          overflow: 'auto',
          padding: 'var(--space-xl)',
          display: 'flex',
          flexDirection: 'column',
          gap: 'var(--space-lg)',
        }}
      >
        <div
          className="type-display"
          style={{ color: 'var(--count-nominal)', fontVariantNumeric: 'tabular-nums' }}
        >
          00:00:00 · step 0
        </div>

        <section style={{ display: 'flex', gap: 'var(--space-md)', flexWrap: 'wrap' }}>
          {STATUS_TIERS.map(({ label, token }) => (
            <span
              key={label}
              style={{ display: 'inline-flex', alignItems: 'center', gap: 'var(--space-xs)' }}
            >
              <span
                aria-hidden="true"
                style={{
                  width: 12,
                  height: 12,
                  borderRadius: 'var(--radius-full)',
                  background: `var(${token})`,
                }}
              />
              <span className="type-label" style={{ color: `var(${token})` }}>
                {label}
              </span>
            </span>
          ))}
        </section>

        <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-xs)' }}>
          {TYPE_TIERS.map(({ role, cls }) => (
            <div key={role} className={cls}>
              {role} — the quick brown fox · P-001 · 1840ms
            </div>
          ))}
        </section>

        <section style={{ display: 'flex', gap: 'var(--space-sm)' }}>
          {SURFACES.map((token) => (
            <div
              key={token}
              style={{
                width: 64,
                height: 40,
                borderRadius: 'var(--radius-md)',
                background: `var(${token})`,
                border: '1px solid var(--border-subtle)',
              }}
            />
          ))}
        </section>
      </main>
    </div>
  )
}
