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
      {/* aria-disabled, not the native attribute: Start is the control that invokes the operator-pause
          hold, and a natively `disabled` button is unfocusable — so when the dialog closes (with the run
          live, hence Start unavailable) the dialog's focus restore has no target and focus falls to
          <body>, losing SC 2.4.3. Kept focusable, the restore lands on the trigger a11y-plan §5 names.
          Activation is guarded here because aria-disabled does not block clicks. */}
      <button
        type="button"
        className="run-btn run-btn--start type-label"
        onClick={() => {
          if (canStart && !running) onStart()
        }}
        aria-disabled={!canStart || running}
        aria-keyshortcuts="Control+Enter"
      >
        Start
      </button>
      <button
        type="button"
        className="run-btn run-btn--stop type-label"
        onClick={onStop}
        disabled={!running}
        aria-keyshortcuts="Control+."
      >
        Stop
      </button>
      {/* The key map made discoverable. Button TEXT stays Start / Stop: selectors and the dialog's focus
          restore key on it, so the hint is a sibling line — not focusable, no live region. */}
      <p className="run-controls__hint type-data">
        {'Ctrl+Enter start · proceed   Ctrl+. stop   Esc abort'}
      </p>
    </div>
  )
}
