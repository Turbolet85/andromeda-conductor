import type { CSSProperties } from 'react'

// Off-screen but in the accessibility tree: the clip-rect idiom, not `display:none`/`aria-hidden`,
// which would remove the node from the tree and announce nothing (a11y-plan §11 Screen Reader). These
// are a11y mechanics, not design values, so they carry no token (design-system §Tokens governs
// palette/space/motion).
export const SR_ONLY: CSSProperties = {
  position: 'absolute',
  width: '1px',
  height: '1px',
  margin: '-1px',
  padding: 0,
  overflow: 'hidden',
  clip: 'rect(0, 0, 0, 0)',
  whiteSpace: 'nowrap',
  border: 0,
}
