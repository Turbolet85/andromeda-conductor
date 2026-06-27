import { useEffect, useState } from 'react'
import { invoke, Channel } from '@tauri-apps/api/core'
import Titlebar, { type RunState } from './components/Titlebar'
import ScenarioPicker, { type ScenarioSummary } from './components/ScenarioPicker'
import RunControls from './components/RunControls'
import CoverageMatrix, { type CapabilityRow } from './components/CoverageMatrix'

type RunStage = 'progress' | 'blocked' | 'done' | 'aborted'
interface RunEvent {
  stage: RunStage
  count: number
}

// The live Channel stage drives the titlebar run-state: a scenario completing keeps it `live`, the
// terminal stages (`blocked`/`done`) settle to `idle`, and `aborted` dims it.
const STATE_FOR_STAGE: Record<RunStage, RunState> = {
  progress: 'live',
  blocked: 'idle',
  done: 'idle',
  aborted: 'aborted',
}

export default function App() {
  const [runState, setRunState] = useState<RunState>('idle')
  const [count, setCount] = useState('00:00:00')
  const [scenarios, setScenarios] = useState<ScenarioSummary[]>([])
  const [selection, setSelection] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [loadError, setLoadError] = useState<string | null>(null)
  const [actionError, setActionError] = useState<string | null>(null)
  const [coverage, setCoverage] = useState<CapabilityRow[]>([])
  const [coverageError, setCoverageError] = useState<string | null>(null)
  const [coverageLoading, setCoverageLoading] = useState(true)

  useEffect(() => {
    invoke<ScenarioSummary[]>('list_scenarios')
      .then(setScenarios)
      .catch((e) => setLoadError(String(e)))
      .finally(() => setLoading(false))
  }, [])

  useEffect(() => {
    invoke<CapabilityRow[]>('coverage_matrix')
      .then(setCoverage)
      .catch((e) => setCoverageError(String(e)))
      .finally(() => setCoverageLoading(false))
  }, [])

  const running = runState === 'live'

  const start = async () => {
    if (!selection) return
    setActionError(null)
    const channel = new Channel<RunEvent>()
    channel.onmessage = (event) => {
      setCount(String(event.count))
      setRunState(STATE_FOR_STAGE[event.stage])
    }
    try {
      setRunState('live')
      setCount('0')
      await invoke('start_run', { selection, onEvent: channel })
    } catch (e) {
      setRunState('idle')
      setActionError(String(e))
    }
  }

  const stop = async () => {
    setActionError(null)
    try {
      await invoke('stop_run')
      setRunState('aborted')
    } catch (e) {
      setActionError(String(e))
    }
  }

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
      <Titlebar runState={runState} count={count} />

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
        <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-sm)' }}>
          <h2 className="type-heading" style={{ margin: 0 }}>
            Scenario / suite
          </h2>
          {loading ? (
            <p className="type-body" style={{ color: 'var(--text-tertiary)' }}>
              Loading scenarios…
            </p>
          ) : loadError ? (
            <p className="type-body" role="alert" style={{ color: 'var(--status-fail)' }}>
              Could not load scenarios: {loadError}
            </p>
          ) : scenarios.length === 0 ? (
            <p className="type-body" style={{ color: 'var(--text-tertiary)' }}>
              No scenarios found.
            </p>
          ) : (
            <ScenarioPicker scenarios={scenarios} selection={selection} onSelect={setSelection} />
          )}
        </section>

        <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-sm)' }}>
          <RunControls
            canStart={selection !== null}
            running={running}
            onStart={start}
            onStop={stop}
          />
          {actionError ? (
            <p
              className="type-body"
              role="alert"
              style={{ color: 'var(--status-fail)', margin: 0 }}
            >
              {actionError}
            </p>
          ) : null}
        </section>

        <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-sm)' }}>
          <h2 className="type-heading" style={{ margin: 0 }}>
            Coverage matrix
          </h2>
          {coverageLoading ? (
            <p className="type-body" style={{ color: 'var(--text-tertiary)' }}>
              Loading coverage…
            </p>
          ) : coverageError ? (
            <p className="type-body" role="alert" style={{ color: 'var(--status-fail)' }}>
              Could not load coverage: {coverageError}
            </p>
          ) : coverage.length === 0 ? (
            <p className="type-body" style={{ color: 'var(--text-tertiary)' }}>
              No coverage data.
            </p>
          ) : (
            <CoverageMatrix rows={coverage} />
          )}
        </section>
      </main>
    </div>
  )
}
