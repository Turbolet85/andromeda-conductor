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

// Mirrors conductor-core's Verdict / ReportState (the run-report envelope's two independent fields).
export type Verdict = 'Pass' | 'Fail' | 'CalibrationRegion'
export type ReportState = 'Pass' | 'Fail' | 'ManualCheck' | 'KnownResidual' | 'Blocked'

// Verdict-first lamp projection — byte-mirror of conductor-core/src/lamp.rs `Lamp::for_record`,
// arm-for-arm. Blocked / KnownResidual are state-driven and precede the verdict (a measured residual
// still carries a verdict, which must not override the accepted-residual signal); otherwise the
// verdict drives (CalibrationRegion → Hold), with a verdict-less check falling back to its state.
export function lampForRecord(state: ReportState, verdict: Verdict | null): Lamp {
  if (state === 'Blocked') return 'Blocked'
  if (state === 'KnownResidual') return 'Residual'
  if (verdict === 'Pass') return 'Pass'
  if (verdict === 'Fail') return 'Fail'
  if (verdict === 'CalibrationRegion') return 'Hold'
  if (state === 'ManualCheck') return 'Manual'
  if (state === 'Pass') return 'Pass'
  return 'Fail'
}
