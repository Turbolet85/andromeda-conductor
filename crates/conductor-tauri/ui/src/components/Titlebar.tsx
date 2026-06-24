import { getCurrentWindow } from '@tauri-apps/api/window'
import './Titlebar.css'

export default function Titlebar() {
  return (
    <header className="titlebar" data-tauri-drag-region>
      <span className="type-heading titlebar__label" data-tauri-drag-region>
        Conductor · idle
      </span>
      <span className="type-data titlebar__count" data-tauri-drag-region>
        00:00:00
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
