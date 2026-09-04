import { getCurrentWindow } from '@tauri-apps/api/window'
import './Titlebar.css'

export type RunState = 'idle' | 'live' | 'hold' | 'aborted'

/** The pre-run placeholder the count shows before any scenario has completed. */
export const IDLE_COUNT = '00:00:00'

const STATE_LABEL: Record<RunState, string> = {
  idle: 'Conductor · idle',
  live: 'Conductor · live',
  hold: 'Conductor · HOLD — operator pause',
  aborted: 'Conductor · aborted',
}

export default function Titlebar({
  runState,
  count = IDLE_COUNT,
}: {
  runState: RunState
  count?: string
}) {
  // design-system §Component Patterns 1 mandates an assertive HOLD announcement, but an assertive
  // region preempts speech on EVERY flip — which cancelled the dialog's focus-restore announcement
  // when a hold resolved (NVDA pass 2026-09-02, S2-07/S3-04). Assertive is scoped to the flip that
  // needs it; the other three states queue politely.
  const phaseLiveness = runState === 'hold' ? 'assertive' : 'polite'
  const countLabel =
    count === IDLE_COUNT ? 'Scenario count: no run yet' : `Scenarios completed: ${count}`
  return (
    <header className="titlebar" data-tauri-drag-region>
      <span
        className="type-heading titlebar__label"
        data-tauri-drag-region
        aria-live={phaseLiveness}
      >
        {STATE_LABEL[runState]}
      </span>
      <span
        className={`type-data titlebar__count titlebar__count--${runState}`}
        data-tauri-drag-region
        // a11y-plan §4 asks for role=status OR aria-live on a live counter. `aria-live` is the one
        // taken: `role="status"` is reserved here for the operator-checklist roll-up, and the a11y
        // suite's subject-absent guard reads a bare `[role="status"]` as "a hold is raised".
        aria-live="polite"
        aria-label={countLabel}
      >
        {count}
      </span>
      <div className="titlebar__controls">
        <button
          type="button"
          className="titlebar-btn"
          aria-label="Minimize window"
          onClick={() => void getCurrentWindow().minimize()}
        >
          &#8211;
        </button>
        <button
          type="button"
          className="titlebar-btn titlebar-btn--close"
          aria-label="Close window"
          onClick={() => void getCurrentWindow().close()}
        >
          &#10005;
        </button>
      </div>
    </header>
  )
}
