import { useCallback, useEffect, useRef, useState } from 'react'
import { invoke, Channel } from '@tauri-apps/api/core'
import Titlebar, { type RunState } from './components/Titlebar'
import ScenarioPicker, { type ScenarioSummary } from './components/ScenarioPicker'
import RunControls from './components/RunControls'
import CoverageMatrix, { type CapabilityRow } from './components/CoverageMatrix'
import RunReport from './components/RunReport'
import OperatorPauseDialog from './components/OperatorPauseDialog'
import { type ChecklistItem } from './components/OperatorChecklist'
import { type RunRecord } from './lamp'

type RunStage = 'progress' | 'blocked' | 'done' | 'aborted'
interface RunEvent {
  stage: RunStage
  count: number
}

// The backend HoldPrompt projection (conductor-tauri pause.rs) — the operator-pause dialog renders
// these fields; the decision posts back via the resolve_operator_hold command.
interface HoldPrompt {
  title: string
  body: string
  allow_no_go: boolean
  checklist: HoldChecklistItem[]
}

// One declared operator-checklist item as the backend projects it (conductor-core ChecklistItem):
// what Conductor drove, and the observation to confirm. The tick state is the webview's own — the
// backend declares the pair, the operator records the answer.
interface HoldChecklistItem {
  induced: string
  observation: string
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
  const [unbacked, setUnbacked] = useState(0)
  const [report, setReport] = useState<RunRecord[]>([])
  const [reportError, setReportError] = useState<string | null>(null)
  const [reportLoading, setReportLoading] = useState(true)
  const [holdPrompt, setHoldPrompt] = useState<HoldPrompt | null>(null)
  const [tickedItems, setTickedItems] = useState<string[]>([])
  const pendingHold = useRef(false)
  const holdInvoker = useRef<HTMLElement | null>(null)

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

  // The unbacked-auto ledger qualifies the roll-up's auto term. A failure leaves it at 0 — the
  // qualifier disappears rather than the matrix, which is the honest degrade for a roll-up annotation.
  useEffect(() => {
    invoke<string[]>('unbacked_auto')
      .then((ids) => setUnbacked(ids.length))
      .catch(() => setUnbacked(0))
  }, [])

  // The run report is re-read whenever a run settles (a terminal Channel stage) so the GUI closes the
  // run→report loop; `initial` gates the one-shot loading prose so refreshes don't flash it.
  const loadReport = useCallback((initial = false) => {
    invoke<RunRecord[]>('run_report')
      .then(setReport)
      .catch((e) => setReportError(String(e)))
      .finally(() => {
        if (initial) setReportLoading(false)
      })
  }, [])

  useEffect(() => {
    loadReport(true)
  }, [loadReport])

  const running = runState === 'live'

  // The declared pair comes from the backend; `checked` is the webview's own record of what the
  // operator has confirmed. Index-keyed because a declared item carries no identity of its own.
  const checklistItems: ChecklistItem[] = (holdPrompt?.checklist ?? []).map((item, index) => ({
    id: String(index),
    induced: item.induced,
    observation: item.observation,
    checked: tickedItems.includes(String(index)),
  }))

  const toggleChecklistItem = (id: string, checked: boolean) =>
    setTickedItems((prev) => (checked ? [...prev, id] : prev.filter((x) => x !== id)))

  // Deliver the operator's go/no-go to the awaiting backend hold. Idempotent via a ref: the controlled
  // Radix dialog fires onProceed/onAbort AND onOpenChange(false) for a single action.
  const resolveHold = async (decision: 'Go' | 'NoGo') => {
    if (!pendingHold.current) return
    pendingHold.current = false
    setHoldPrompt(null)
    setRunState('live')
    try {
      await invoke('resolve_operator_hold', { decision })
    } catch (e) {
      setActionError(String(e))
    }
  }

  const start = async () => {
    if (!selection) return
    setActionError(null)
    const channel = new Channel<RunEvent>()
    channel.onmessage = (event) => {
      setCount(String(event.count))
      setRunState(STATE_FOR_STAGE[event.stage])
      if (event.stage !== 'progress') loadReport()
    }
    const holdChannel = new Channel<HoldPrompt>()
    holdChannel.onmessage = (prompt) => {
      // The hold arrives on a Channel, so the dialog has no Radix Trigger to restore focus to on close
      // — capture the control the operator was on and hand it back explicitly (SC 2.4.3).
      holdInvoker.current = document.activeElement as HTMLElement | null
      pendingHold.current = true
      setHoldPrompt(prompt)
      setTickedItems([])
      setRunState('hold')
    }
    try {
      setRunState('live')
      setCount('0')
      await invoke('start_run', { selection, onEvent: channel, onHold: holdChannel })
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
            <CoverageMatrix rows={coverage} unbacked={unbacked} />
          )}
        </section>

        <section style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-sm)' }}>
          <h2 className="type-heading" style={{ margin: 0 }}>
            Run report
          </h2>
          {reportLoading ? (
            <p className="type-body" style={{ color: 'var(--text-tertiary)' }}>
              Loading run report…
            </p>
          ) : reportError ? (
            <p className="type-body" role="alert" style={{ color: 'var(--status-fail)' }}>
              Could not load run report: {reportError}
            </p>
          ) : report.length === 0 ? (
            <p className="type-body" style={{ color: 'var(--text-tertiary)' }}>
              No run yet
            </p>
          ) : (
            <RunReport records={report} />
          )}
        </section>
      </main>

      <OperatorPauseDialog
        open={holdPrompt !== null}
        onOpenChange={(open) => {
          if (!open) void resolveHold('NoGo')
        }}
        title={holdPrompt?.title ?? ''}
        body={holdPrompt?.body ?? ''}
        checklist={checklistItems}
        onChecklistToggle={toggleChecklistItem}
        allowNoGo={holdPrompt?.allow_no_go ?? true}
        restoreFocusTo={() => holdInvoker.current}
        onProceed={() => void resolveHold('Go')}
        onAbort={() => void resolveHold('NoGo')}
      />
    </div>
  )
}
