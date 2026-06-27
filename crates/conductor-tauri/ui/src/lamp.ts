// Mirrors conductor-core/src/lamp.rs — the single lamp-truth; labels/prefixes/tokens must not diverge
// (the cli, Markdown report, and this desktop surface all project the same six-way status).
export type Lamp = 'Pass' | 'Fail' | 'Hold' | 'Manual' | 'Residual' | 'Blocked'

export interface LampMeta {
  label: string
  prefix: string
  token: string
  glyph: string
}

export const LAMP_META: Record<Lamp, LampMeta> = {
  Pass: { label: 'Pass', prefix: '[PASS]', token: '--count-nominal', glyph: '●' },
  Fail: { label: 'Fail', prefix: '[FAIL]', token: '--status-fail', glyph: '●' },
  Hold: { label: 'HOLD', prefix: '[HOLD]', token: '--count-hold', glyph: '●' },
  Manual: { label: 'Manual', prefix: '[MANUAL]', token: '--status-manual', glyph: '☐' },
  Residual: { label: 'Residual', prefix: '[RESIDUAL]', token: '--status-residual', glyph: '◌' },
  Blocked: { label: 'Blocked', prefix: '[BLOCKED]', token: '--count-blocked', glyph: '○' },
}

export const LAMP_ORDER: Lamp[] = ['Pass', 'Fail', 'Hold', 'Manual', 'Residual', 'Blocked']
