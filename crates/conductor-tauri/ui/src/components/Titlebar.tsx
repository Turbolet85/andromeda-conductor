import { getCurrentWindow } from '@tauri-apps/api/window'
import './Titlebar.css'

export type RunState = 'idle' | 'live' | 'hold' | 'aborted'

const STATE_LABEL: Record<RunState, string> = {
  idle: 'Conductor · idle',
  live: 'Conductor · live',
  hold: 'Conductor · HOLD — operator pause',
  aborted: 'Conductor · aborted',
}

export default function Titlebar({
  runState,
  count = '00:00:00',
}: {
  runState: RunState
  count?: string
}) {
  return (
    <header className="titlebar" data-tauri-drag-region>
      <span
        className="type-heading titlebar__label"
        data-tauri-drag-region
        aria-live="assertive"
      >
        {STATE_LABEL[runState]}
      </span>
      <span
        className={`type-data titlebar__count titlebar__count--${runState}`}
        data-tauri-drag-region
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
