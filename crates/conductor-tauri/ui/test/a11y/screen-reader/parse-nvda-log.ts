// NVDA speech log + the SR leg's action timeline → the pass record (nvda-pass.json).
//
// NVDA's own log is the driver: started with `-l 12` (input/output) it records every utterance sent to the
// synthesizer as an `IO - speech.… (HH:MM:SS.mmm)` header followed by `Speaking [...]`. The leg stamps a row
// before each action; the utterances timestamped between one stamp and the next belong to that row. Grading is
// on REQUIRED TOKENS (name · state word · text), never on NVDA's connective phrasing, which is version-bound —
// the operator's review is the judgment over the graded rows (a11y-plan §3 Screen reader test pattern).
//
// Node built-ins only. The NVDA log itself is a third-party ephemeral artifact under the gitignored runs/ tree;
// the record written here is Conductor's own, so host-path tokens are scrubbed from utterances before they are
// written (obs-plan §11 Logs).
import { execFileSync } from 'node:child_process'
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { ROWS, rowsFor, type RowClass, type Subject, type SpecRow } from './rows'

export type Outcome =
  | 'announced-as-expected'
  | 'announced-differently'
  | 'not-announced'
  | 'not-run-here'
  | 'subject-absent'
export type Arm = 'agent' | 'operator'
/** The input path of the keys the leg sent inside a row's window. */
export type InputPath = 'os' | 'webdriver' | 'mixed' | 'none'

export interface PassRow {
  id: string
  subject: Subject
  state: string
  class: RowClass
  item: string
  node: string
  sc: string
  expected: string
  tokens: readonly string[]
  arm: Arm
  outcome: Outcome
  input: InputPath
  heard: string[]
  action: string | null
  action_ts: string | null
  note?: string
  security_finding?: string
  /** Set by the operator's review transcription, never by the leg. */
  review_grade?: ReviewGrade
}

export interface SubjectRecord {
  recorded_at: string
  speech_utterances: number
  pre_session_utterances: number
  post_session_utterances: number
  attach_observed: boolean | null
  /**
   * `initial_focus` is the accessible name focused when the app came up, before the leg blurred to body.
   * `tabs_to_start` is the DISCRIMINATOR for the initial-focus finding: `initial_focus` reads `BODY` on
   * every subject because it samples `activeElement`, while the defect lives in Chromium's separate
   * sequential-focus starting point — the count of Tabs the leg needed to reach the host-chrome stop is
   * what exposes it (1 ⇒ the walk starts at the document; more ⇒ a control held focus at mount).
   */
  foreground: {
    activated: boolean
    nvda_named_window: boolean
    initial_focus: string | null
    tabs_to_start: number | null
  } | null
  process_census: { before: string[]; after: string[] }
  /** NVDA's log clock against the leg's, calibrated from the paired OS keys. */
  clock?: ClockCalibration
}

/** The operator's judgment over the graded rows — transcribed from the review, never generated. */
export interface OperatorReview {
  reviewed_at: string
  reviewer: string
  verdict: string
  /** The grading rules the operator stated, verbatim. */
  basis: string[]
  grades: {
    'accepted-as-heard': string[]
    finding: Record<string, string>
    'accepted-with-reason': string[]
  }
  notes: string
}

export type ReviewGrade = 'accepted-as-heard' | 'finding' | 'accepted-with-reason'

export interface PassFile {
  spec: string
  recorded_at: string
  nvda_version: string
  /** The WebView2 version the driver session attached to (the registry key can lag a background update). */
  webview2_runtime: string
  webview2_runtime_registry: string
  build_commit: string
  bundle_build: string
  stimulus_runs: { fixture_run_id: string; live_run_id: string | null }
  attach_observed: Record<Subject, boolean | null>
  subjects: Partial<Record<Subject, SubjectRecord>>
  rows: PassRow[]
  findings: string[]
  operator_review: OperatorReview | null
}

export interface ParseOptions {
  subject: Subject
  repoRoot: string
  speechLog: string
  actions: string
  out: string
  specMd: string
  censusBefore: string[]
  censusAfter: string[]
  liveRunsDir?: string
}

interface Stamp {
  ms: number
  ts: string
  id: string
  action: string
}

interface Utterance {
  ms: number
  text: string
}

const SPEC_REL = 'crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md'
const BUNDLE_BUILD = 'cargo build --release -p conductor-tauri --features tauri/custom-protocol'
const FIXTURE_RUN_ID = 'lamps-fixture'
const CENSUS_NAMES = new Set([
  'nvda.exe',
  'conductor-tauri.exe',
  'msedgedriver.exe',
  'node.exe',
  'andromeda-pulse-mcp.exe',
  'pulse-app.exe',
])

// obs-plan §11 Logs / security-plan §Error Handling anchors — a drive-letter path, an expanded profile
// path, or a toolchain path in a heard utterance is a finding, and never lands in the record verbatim.
const HOST_PATH = /[A-Za-z]:\\[^\s'"]*|%APPDATA%[^\s'"]*|\/Users\/[^\s'"]*|\/home\/[^\s'"]*|~\/\.cargo[^\s'"]*|\.rustup[^\s'"]*/g

const HEADER = /^(IO|INFO|DEBUG|DEBUGWARNING|WARNING|ERROR|CRITICAL) - (\S+) \((\d{2}):(\d{2}):(\d{2})\.(\d{1,6})\)/
const SPEECH_COMMAND = /\b\w+Command\s*\([^)]*\)/g
const PY_STRING = /'((?:[^'\\]|\\.)*)'|"((?:[^"\\]|\\.)*)"/g

function msOfDay(h: number, m: number, s: number, fraction: string): number {
  const millis = Number((fraction + '000').slice(0, 3))
  return h * 3_600_000 + m * 60_000 + s * 1_000 + millis
}

function stampMs(iso: string): number {
  const d = new Date(iso)
  return d.getHours() * 3_600_000 + d.getMinutes() * 60_000 + d.getSeconds() * 1_000 + d.getMilliseconds()
}

/** A keyboard gesture NVDA's own hook logged (`Input: kb(desktop):<gesture>`), at NVDA's clock. */
interface GestureIn {
  ms: number
  gesture: string
}

/** The utterances and input gestures in an NVDA `-l 12` log, in file order, plus the version line when present. */
export function readSpeechLog(path: string): { utterances: Utterance[]; inputs: GestureIn[]; nvdaVersion: string } {
  if (!existsSync(path)) return { utterances: [], inputs: [], nvdaVersion: 'unknown' }
  const text = readFileSync(path, 'utf8')
  const version = /Starting NVDA version (\S+)/.exec(text)?.[1] ?? 'unknown'
  const utterances: Utterance[] = []
  const inputs: GestureIn[] = []
  let block: { codepath: string; ms: number; body: string[] } | undefined
  const flush = (): void => {
    if (!block) return
    const input = block.body.map((l) => /^Input: kb\([^)]*\):(\S+)/.exec(l)?.[1]).find(Boolean)
    if (input) inputs.push({ ms: block.ms, gesture: input })
    if (!block.codepath.startsWith('speech.')) return
    const speaking = block.body.find((l) => l.startsWith('Speaking'))
    if (!speaking) return
    const parts: string[] = []
    for (const m of speaking.replace(SPEECH_COMMAND, '').matchAll(PY_STRING)) {
      // Python repr: embedded newlines arrive as the two characters `\n`; they separate lines, not words.
      const s = (m[1] ?? m[2] ?? '').replace(/\\n/g, ' ').replace(/\s+/g, ' ').trim()
      if (s) parts.push(s)
    }
    if (parts.length > 0) utterances.push({ ms: block.ms, text: parts.join(' ') })
  }
  for (const line of text.split(/\r?\n/)) {
    const h = HEADER.exec(line)
    if (h) {
      flush()
      block = {
        codepath: h[2] ?? '',
        ms: msOfDay(Number(h[3]), Number(h[4]), Number(h[5]), h[6] ?? '0'),
        body: [],
      }
    } else if (block) {
      block.body.push(line)
    }
  }
  flush()
  return { utterances, inputs, nvdaVersion: version }
}

const SESSION_STAMP = '@session'
const FOREGROUND_STAMP = '@foreground'
const END_STAMP = '@end'
const KEY_STAMP = '@key'

interface KeySent {
  ms: number
  ts: string
  input: 'os' | 'webdriver'
  key: string
}

/** Utterances longer than this are stored truncated in the record; grading always reads the full text. */
const HEARD_MAX = 400

export interface Timeline {
  stamps: Stamp[]
  browserVersion: string | undefined
  /** The leg's own record of bringing the app window to the OS foreground before its first row. */
  foreground:
    | { activated: boolean; nvdaNamedWindow: boolean; initialFocus: string | undefined; tabsToStart: number | undefined }
    | undefined
  /** When the leg closed its timeline; speech after it (teardown, the shell regaining focus) belongs to no row. */
  endMs: number | undefined
  /** Every key the leg sent, with the input path it rode. */
  keys: KeySent[]
}

/** The leg's action timeline: row stamps in file order, plus the session, foreground, key and end records. */
export function readStamps(path: string): Timeline {
  if (!existsSync(path)) return { stamps: [], browserVersion: undefined, foreground: undefined, endMs: undefined, keys: [] }
  const stamps: Stamp[] = []
  const keys: KeySent[] = []
  let browserVersion: string | undefined
  let foreground: Timeline['foreground']
  let endMs: number | undefined
  for (const line of readFileSync(path, 'utf8').split(/\r?\n/)) {
    if (!line.trim()) continue
    const parsed: unknown = JSON.parse(line)
    if (typeof parsed !== 'object' || parsed === null) continue
    // one JSON line the leg wrote — the shape is this module's own contract with screen-reader.e2e.ts
    const rec = parsed as {
      ts?: unknown
      id?: unknown
      action?: unknown
      browserVersion?: unknown
      activated?: unknown
      nvdaNamedWindow?: unknown
      initialFocus?: unknown
      tabsToStart?: unknown
      input?: unknown
      key?: unknown
    }
    if (typeof rec.ts !== 'string' || typeof rec.id !== 'string') continue
    if (rec.id === KEY_STAMP) {
      if (rec.input === 'os' || rec.input === 'webdriver')
        keys.push({ ms: stampMs(rec.ts), ts: rec.ts, input: rec.input, key: typeof rec.key === 'string' ? rec.key : '' })
      continue
    }
    if (rec.id === SESSION_STAMP) {
      if (typeof rec.browserVersion === 'string') browserVersion = rec.browserVersion
      continue
    }
    if (rec.id === FOREGROUND_STAMP) {
      foreground = {
        activated: rec.activated === true,
        nvdaNamedWindow: rec.nvdaNamedWindow === true,
        initialFocus: typeof rec.initialFocus === 'string' ? rec.initialFocus : undefined,
        tabsToStart: typeof rec.tabsToStart === 'number' ? rec.tabsToStart : undefined,
      }
      continue
    }
    if (rec.id === END_STAMP) {
      endMs = stampMs(rec.ts)
      continue
    }
    // any other `@…` record is the leg's own diagnostic (e.g. a Tab-landing probe), never a row window
    if (rec.id.startsWith('@')) continue
    stamps.push({ ts: rec.ts, ms: stampMs(rec.ts), id: rec.id, action: typeof rec.action === 'string' ? rec.action : '' })
  }
  return { stamps, browserVersion, foreground, endMs, keys }
}

function inputPathOf(keys: KeySent[], start: number, end: number): InputPath {
  const paths = new Set(keys.filter((k) => k.ms >= start && k.ms < end).map((k) => k.input))
  if (paths.size === 0) return 'none'
  if (paths.size > 1) return 'mixed'
  return paths.has('os') ? 'os' : 'webdriver'
}

/** The gesture name NVDA's hook logs for each key the leg sends on the OS path (send-keys.ps1's closed set). */
const OS_GESTURE: Record<string, string> = {
  Tab: 'tab',
  ShiftTab: 'shift+tab',
  h: 'h',
  d: 'd',
  ArrowDown: 'downArrow',
}

/** Past this, an offset is a clock step rather than send-latency jitter (the aligned sessions spread ≤ 110 ms). */
const STEP_MS = 300

export interface ClockCalibration {
  pairs: number
  /** The aligned offset: the median of the pairs within STEP_MS of the lowest. It is send latency plus any skew. */
  baseline_ms: number | null
  max_excess_ms: number
  /** Runs of consecutive keys whose offset exceeds the baseline by more than STEP_MS; their speech is shifted back. */
  shifted_stretches: Array<{ from_key: number; to_key: number; excess_ms: number }>
  /** Set when the session cannot be graded: an unpaired OS key, or a shifted stretch that is not one step. */
  void: string | null
  /** The per-key pair table: the leg's stamp, the key, and NVDA's offset from it. */
  pair_table: Array<{ key: string; leg_ts: string; offset_ms: number }>
}

function median(values: number[]): number {
  const s = [...values].sort((a, b) => a - b)
  return s[Math.floor(s.length / 2)] ?? 0
}

/**
 * NVDA's log clock can step against the leg's within one session (measured 2026-09-30: ~2.5 s ahead for 21 of
 * 35 keys, then realigned), which moves every utterance into the NEXT row's window. The only anchor is the
 * stimulus itself: each OS-path key the leg records, paired in order with the gesture NVDA's hook logs for it.
 * The offset of an aligned pair is send latency (~370 ms) plus skew, inseparable, so nothing is corrected
 * there; only a stretch that exceeds the baseline by more than STEP_MS is shifted back, by its own median
 * excess. Graded content is never consulted.
 */
export function calibrateClock(keys: KeySent[], inputs: GestureIn[]): { record: ClockCalibration; anchors: number[] } {
  const osKeys = keys.filter((k) => k.input === 'os')
  const table: ClockCalibration['pair_table'] = []
  const anchors: number[] = []
  const unshifted = (voidReason: string | null) => ({
    record: { pairs: table.length, baseline_ms: null, max_excess_ms: 0, shifted_stretches: [], void: voidReason, pair_table: table },
    anchors,
  })
  let j = 0
  for (const [i, k] of osKeys.entries()) {
    const gesture = OS_GESTURE[k.key]
    while (j < inputs.length && inputs[j]?.gesture !== gesture) j += 1
    const input = inputs[j]
    if (gesture === undefined || input === undefined)
      return unshifted(`OS key ${i + 1} of ${osKeys.length} (${k.key}) has no matching NVDA input gesture`)
    table.push({ key: k.key, leg_ts: k.ts, offset_ms: input.ms - k.ms })
    anchors.push(input.ms)
    j += 1
  }
  if (table.length === 0) return unshifted(null)
  const offsets = table.map((p) => p.offset_ms)
  const lowest = Math.min(...offsets)
  const baseline = median(offsets.filter((o) => o - lowest <= STEP_MS))
  const stretches: ClockCalibration['shifted_stretches'] = []
  let voidReason: string | null = null
  for (let i = 0; i < offsets.length; ) {
    if ((offsets[i] ?? 0) - baseline <= STEP_MS) {
      i += 1
      continue
    }
    let end = i
    while (end + 1 < offsets.length && (offsets[end + 1] ?? 0) - baseline > STEP_MS) end += 1
    const run = offsets.slice(i, end + 1)
    const spread = Math.max(...run) - Math.min(...run)
    if (spread > STEP_MS && voidReason === null)
      voidReason = `keys ${i + 1}-${end + 1} spread ${spread} ms, over ${STEP_MS} ms: not one clock step`
    stretches.push({ from_key: i + 1, to_key: end + 1, excess_ms: median(run) - baseline })
    i = end + 1
  }
  const record: ClockCalibration = {
    pairs: table.length,
    baseline_ms: baseline,
    max_excess_ms: Math.max(0, ...offsets.map((o) => o - baseline)),
    shifted_stretches: stretches,
    void: voidReason,
    pair_table: table,
  }
  return { record, anchors }
}

/**
 * An utterance's time on the leg's clock: shifted back only when the nearest earlier paired key (by NVDA's
 * clock, `anchors`) falls in a shifted stretch.
 */
function onLegClock(calibration: ClockCalibration, anchors: number[], ms: number): number {
  let at = 0
  while (at < anchors.length && (anchors[at] ?? Infinity) <= ms) at += 1
  const stretch = calibration.shifted_stretches.find((s) => at >= s.from_key && at <= s.to_key)
  return stretch ? ms - stretch.excess_ms : ms
}

function truncate(text: string): string {
  return text.length <= HEARD_MAX ? text : `${text.slice(0, HEARD_MAX)} … [${text.length - HEARD_MAX} more chars]`
}

function scrub(text: string): { text: string; flagged: boolean } {
  let flagged = false
  const scrubbed = text.replace(HOST_PATH, () => {
    flagged = true
    return '<host-path>'
  })
  return { text: scrubbed, flagged }
}

const BROWSE_NOT_DRIVEN =
  'no browse-mode command was driven through the OS input path in this row window, so NVDA’s keyboard hook ' +
  'never saw one; the operator arm reads this row'

function grade(row: SpecRow, heard: string[], input: InputPath): { outcome: Outcome; arm: Arm; note?: string } {
  if (row.absent) return { outcome: 'subject-absent', arm: 'agent', note: row.absent }
  if (row.notRun) return { outcome: 'not-run-here', arm: 'agent', note: row.notRun }
  // A silent browse row is "not announced" only when an OS key reached NVDA's hook inside its window;
  // otherwise the command never reached the screen reader and the row falls to the operator's manual arm.
  if (heard.length === 0 && row.cls === 'browse' && input !== 'os' && input !== 'mixed')
    return { outcome: 'not-run-here', arm: 'operator', note: BROWSE_NOT_DRIVEN }
  if (heard.length === 0) return { outcome: 'not-announced', arm: 'agent' }
  const joined = heard.join(' ').toLowerCase()
  if (row.review) return { outcome: 'announced-differently', arm: 'agent', note: 'no token grades this value; operator review decides' }
  const missing = row.tokens.filter((t) => !joined.includes(t.toLowerCase()))
  const hit = (row.forbidden ?? []).filter((t) => joined.includes(t.toLowerCase()))
  if (missing.length === 0 && hit.length === 0) return { outcome: 'announced-as-expected', arm: 'agent' }
  const why = [
    missing.length > 0 ? `missing: ${missing.join(', ')}` : '',
    hit.length > 0 ? `forbidden heard: ${hit.join(', ')}` : '',
  ]
    .filter(Boolean)
    .join('; ')
  return { outcome: 'announced-differently', arm: 'agent', note: why }
}

/** Every row id the leg drives must be a row of the spec document — the two are kept from drifting apart. */
function assertSpecCarriesRows(specMd: string): void {
  if (!existsSync(specMd)) throw new Error('nvda-pass-spec.md is missing; the leg writes no evidence without its spec')
  const md = readFileSync(specMd, 'utf8')
  const missing = ROWS.filter((r) => !md.includes(`| ${r.id} |`)).map((r) => r.id)
  if (missing.length > 0) throw new Error(`nvda-pass-spec.md is missing rows: ${missing.join(', ')}`)
}

function runOrUnknown(file: string, args: string[], cwd?: string): string {
  try {
    return execFileSync(file, args, { cwd, encoding: 'utf8', windowsHide: true, timeout: 15_000 }).trim()
  } catch {
    return ''
  }
}

function webview2Runtime(): string {
  const out = runOrUnknown('reg', [
    'query',
    'HKLM\\SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
    '/v',
    'pv',
  ])
  return /pv\s+REG_SZ\s+(\S+)/.exec(out)?.[1] ?? 'unknown'
}

/** The processes the leg cares about, as `name pid` lines (Windows `tasklist`; empty elsewhere). */
export function census(): string[] {
  const out = runOrUnknown('tasklist', ['/FO', 'CSV', '/NH'])
  const lines: string[] = []
  for (const line of out.split(/\r?\n/)) {
    const cells = line.split('","').map((c) => c.replace(/^"|"$/g, ''))
    const name = (cells[0] ?? '').toLowerCase()
    if (CENSUS_NAMES.has(name)) lines.push(`${name} ${cells[1] ?? '?'}`)
  }
  return lines.sort()
}

function newestRunId(dir: string | undefined): string | null {
  if (!dir || !existsSync(dir)) return null
  let best: { id: string; mtime: number } | undefined
  for (const name of readdirSync(dir)) {
    if (!name.endsWith('.jsonl')) continue
    const mtime = statSync(join(dir, name)).mtimeMs
    if (!best || mtime > best.mtime) best = { id: name.slice(0, -'.jsonl'.length), mtime }
  }
  return best?.id ?? null
}

function readExisting(out: string): PassFile | undefined {
  if (!existsSync(out)) return undefined
  const parsed: unknown = JSON.parse(readFileSync(out, 'utf8'))
  if (typeof parsed !== 'object' || parsed === null) return undefined
  return parsed as PassFile // the file this module wrote on a previous subject's session
}

export function writeNvdaPass(opts: ParseOptions): PassFile {
  assertSpecCarriesRows(opts.specMd)
  const speech = readSpeechLog(opts.speechLog)
  const timeline = readStamps(opts.actions)
  const { record: clock, anchors } = calibrateClock(timeline.keys, speech.inputs)
  const utterances = speech.utterances.map((u) => ({ ...u, ms: onLegClock(clock, anchors, u.ms) }))
  const nvdaVersion = speech.nvdaVersion
  const stamps = timeline.stamps.sort((a, b) => a.ms - b.ms)
  const rows = rowsFor(opts.subject)
  const byId = new Map(stamps.map((s) => [s.id, s]))
  const firstMs = stamps[0]?.ms ?? Number.POSITIVE_INFINITY
  const assigned = new Set<number>()

  const endMs = timeline.endMs ?? Number.POSITIVE_INFINITY
  // Rows the leg stamps "in the same instant" (a state flip and the dialog it opens) land a few ms apart;
  // they share one window rather than the first getting an empty slice.
  const SHARED_WINDOW_MS = 50
  const windowOf = (stamp: Stamp): [number, number] => {
    const next = stamps.find((s) => s.ms > stamp.ms + SHARED_WINDOW_MS)
    return [stamp.ms, Math.min(next ? next.ms : Number.POSITIVE_INFINITY, endMs)]
  }

  const findings: string[] = []
  let attachSeen = false
  let attachAttempted = false
  const passRows: PassRow[] = rows.map((row) => {
    const stamp = byId.get(row.id)
    const heardRaw: string[] = []
    let input: InputPath = 'none'
    if (stamp) {
      const [start, end] = windowOf(stamp)
      input = inputPathOf(timeline.keys, start, end)
      utterances.forEach((u, i) => {
        const inWindow = u.ms >= start && u.ms < end
        const preSession = row.fromSessionStart === true && u.ms < firstMs
        if (inWindow || preSession) {
          heardRaw.push(u.text)
          assigned.add(i)
        }
      })
    }
    let security: string | undefined
    const heardFull = heardRaw.map((t) => {
      const s = scrub(t)
      if (s.flagged)
        security =
          'a host path was HEARD in this row window — either a foreign window title NVDA announced (a console, ' +
          'a terminal pane) or text Conductor rendered; only the latter is a security finding against the ' +
          'sanitize_error edge, so the verdict must say which'
      return s.text
    })
    // A session whose clock cannot be calibrated is recorded, never graded.
    const graded: { outcome: Outcome; arm: Arm; note?: string } =
      clock.void !== null && !row.absent
        ? { outcome: 'not-run-here', arm: 'agent', note: `session void — NVDA's clock: ${clock.void}` }
        : grade(row, heardFull, input)
    const heard = heardFull.map(truncate)
    if (row.cls !== 'browse' && !row.absent && !row.notRun && stamp) {
      attachAttempted = true
      if (heard.length > 0) attachSeen = true
    }
    if (!row.absent && !row.notRun && graded.outcome !== 'announced-as-expected') {
      findings.push(`${row.id} ${graded.outcome}${graded.note ? ` (${graded.note})` : ''}: ${row.item}`)
    }
    if (security) findings.push(`${row.id} ${security}`)
    return {
      id: row.id,
      subject: row.subject,
      state: row.state,
      class: row.cls,
      item: row.item,
      node: row.node,
      sc: row.sc,
      expected: row.expected,
      tokens: row.tokens,
      arm: graded.arm,
      outcome: graded.outcome,
      input,
      heard,
      action: stamp?.action ?? null,
      action_ts: stamp?.ts ?? null,
      ...(graded.note ? { note: graded.note } : {}),
      ...(security ? { security_finding: security } : {}),
    }
  })

  const preSession = utterances.filter((u, i) => u.ms < firstMs && !assigned.has(i)).length
  const postSession = utterances.filter((u, i) => u.ms >= endMs && !assigned.has(i)).length
  const now = new Date().toISOString()
  const existing = readExisting(opts.out)
  const attach = attachAttempted ? attachSeen : null
  const file: PassFile = {
    spec: SPEC_REL,
    recorded_at: now,
    nvda_version: nvdaVersion !== 'unknown' ? nvdaVersion : (existing?.nvda_version ?? 'unknown'),
    webview2_runtime: timeline.browserVersion ?? existing?.webview2_runtime ?? webview2Runtime(),
    webview2_runtime_registry: webview2Runtime(),
    build_commit: runOrUnknown('git', ['rev-parse', 'HEAD'], opts.repoRoot) || 'unknown',
    bundle_build: BUNDLE_BUILD,
    stimulus_runs: {
      fixture_run_id: FIXTURE_RUN_ID,
      live_run_id: opts.subject === 'live' ? newestRunId(opts.liveRunsDir) : (existing?.stimulus_runs.live_run_id ?? null),
    },
    attach_observed: { ...(existing?.attach_observed ?? { live: null, empty: null, error: null }), [opts.subject]: attach },
    subjects: {
      ...(existing?.subjects ?? {}),
      [opts.subject]: {
        recorded_at: now,
        speech_utterances: utterances.length,
        pre_session_utterances: preSession,
        post_session_utterances: postSession,
        attach_observed: attach,
        foreground: timeline.foreground
          ? {
              activated: timeline.foreground.activated,
              nvda_named_window: timeline.foreground.nvdaNamedWindow,
              initial_focus: timeline.foreground.initialFocus ?? null,
              tabs_to_start: timeline.foreground.tabsToStart ?? null,
            }
          : null,
        process_census: { before: opts.censusBefore, after: opts.censusAfter },
        clock,
      },
    },
    rows: [...(existing?.rows ?? []).filter((r) => r.subject !== opts.subject), ...passRows],
    findings: [...(existing?.findings ?? []).filter((f) => !rows.some((r) => f.startsWith(`${r.id} `))), ...findings],
    operator_review: existing?.operator_review ?? null,
  }
  writeFileSync(opts.out, JSON.stringify(file, null, 2) + '\n', 'utf8')
  return file
}

// CLI: tsx parse-nvda-log.ts <speech-log> <actions.jsonl> <out.json> [--subject live|empty|error]
const invokedDirectly = process.argv[1] !== undefined && resolve(process.argv[1]) === fileURLToPath(import.meta.url)
if (invokedDirectly) {
  const [speechLog, actions, out] = process.argv.slice(2)
  if (!speechLog || !actions || !out) {
    console.error('usage: tsx parse-nvda-log.ts <speech-log> <actions.jsonl> <out.json> [--subject live|empty|error]')
    process.exit(2)
  }
  const flag = process.argv.indexOf('--subject')
  const raw = flag >= 0 ? process.argv[flag + 1] : undefined
  const subject: Subject = raw === 'empty' || raw === 'error' ? raw : 'live'
  const here = dirname(fileURLToPath(import.meta.url))
  const repoRoot = resolve(here, '..', '..', '..', '..', '..', '..')
  const file = writeNvdaPass({
    subject,
    repoRoot,
    speechLog: resolve(speechLog),
    actions: resolve(actions),
    out: resolve(out),
    specMd: join(here, 'nvda-pass-spec.md'),
    censusBefore: [],
    censusAfter: census(),
    liveRunsDir: subject === 'live' ? join(repoRoot, 'runs', 'sr-leg', 'runs') : undefined,
  })
  const counts = new Map<Outcome, number>()
  for (const r of file.rows) counts.set(r.outcome, (counts.get(r.outcome) ?? 0) + 1)
  console.log(`nvda-pass: ${file.rows.length} rows — ${[...counts].map(([k, v]) => `${k} ${v}`).join(' · ')}`)
}
