import './RunControls.css'

export default function RunControls({
  canStart,
  running,
  onStart,
  onStop,
}: {
  canStart: boolean
  running: boolean
  onStart: () => void
  onStop: () => void
}) {
  return (
    <div className="run-controls" role="group" aria-label="Run controls">
      <button
        type="button"
        className="run-btn run-btn--start type-label"
        onClick={onStart}
        disabled={!canStart || running}
      >
        Start
      </button>
      <button
        type="button"
        className="run-btn run-btn--stop type-label"
        onClick={onStop}
        disabled={!running}
      >
        Stop
      </button>
    </div>
  )
}
