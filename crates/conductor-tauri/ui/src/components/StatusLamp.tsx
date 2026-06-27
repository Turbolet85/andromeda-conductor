import { LAMP_META, type Lamp } from '../lamp'
import './StatusLamp.css'

export default function StatusLamp({
  lamp,
  size = 'md',
}: {
  lamp: Lamp
  size?: 'sm' | 'md'
}) {
  const meta = LAMP_META[lamp]
  return (
    <span className={`lamp lamp--${size}`}>
      <span className="lamp__glyph" aria-hidden="true" style={{ color: `var(${meta.token})` }}>
        {meta.glyph}
      </span>
      <span className="lamp__label type-label">{meta.label}</span>
    </span>
  )
}
