// Desktop a11y sweep — the routine arm (a11y-plan §3/§5/§6/§10).
//
// DRIVER-GATED, not display-gated: the platform SET is Linux CI headless under xvfb, and the Windows
// dev host headful via WebView2 + an operator-supplied msedgedriver named by CONDUCTOR_MSEDGEDRIVER
// (unset ⇒ the leg skips at exit 0 — see ../../wdio.conf.ts). macOS alone has no WebDriver.
//
// This file is the UNATTENDED arm: it asserts what an idle console can prove without a live Pulse and
// must exit 0. Assertions whose SUBJECT only exists in a driven run (the operator-pause alertdialog,
// the checklist rows) skip with a named reason here and are asserted for real in operator-hold.e2e.ts,
// the driven arm — a skip is not a pass (a11y-plan §11 SLO).
//
// Selectors are role/text/aria-live only — never xpath, never hashed classes (test-plan §6/§11).
import { browser, $, $$, expect } from '@wdio/globals'
import AxeBuilder from '@axe-core/webdriverio'
import Color from 'colorjs.io'
import { appendFileSync, mkdirSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

// Minimal tier — wcag2a/2aa/21aa, no wcag22aa (a11y-plan §1/§3.5).
const WCAG_TAGS = ['wcag2a', 'wcag2aa', 'wcag21aa']

// The a11y-plan §6 pair table, verbatim. Names are the contract; raw values live in design-system.md.
const PAIRS: ReadonlyArray<{ fg: string; bg: string; need: number }> = [
  { fg: '--text-primary', bg: '--color-base', need: 4.5 },
  { fg: '--text-secondary', bg: '--color-raised-1', need: 4.5 },
  { fg: '--text-tertiary', bg: '--color-base', need: 4.5 },
  { fg: '--text-muted', bg: '--color-inset', need: 4.5 },
  { fg: '--color-id-cyan', bg: '--color-base', need: 4.5 },
  { fg: '--count-nominal', bg: '--color-base', need: 3 },
  { fg: '--count-hold', bg: '--color-base', need: 3 },
  { fg: '--status-fail', bg: '--color-base', need: 3 },
  { fg: '--color-focus', bg: '--color-base', need: 3 },
]

// --text-tertiary is also rendered on the raised surfaces — measured 2026-09-01, where it was the sole
// foreground in all 18 nodes of the one color-contrast violation (thead on raised-2, rows on raised-1).
// The §6 table names only its --color-base pairing, so these would otherwise go unasserted.
const EXTRA_PAIRS: ReadonlyArray<{ fg: string; bg: string; need: number }> = [
  { fg: '--text-tertiary', bg: '--color-raised-1', need: 4.5 },
  { fg: '--text-tertiary', bg: '--color-raised-2', need: 4.5 },
  // --status-residual carries TEXT on both its non-lamp surfaces — the coverage Mode cell and the
  // run-level envelope banner — so the ratio it owes is 4.5:1 (SC 1.4.3), not the 3:1 a non-text
  // indicator owes. a11y-plan §6's table enumerates no pair for this token while §1 requires the
  // residual pair be asserted like any other, so these stand in for the row the table lacks.
  { fg: '--status-residual', bg: '--color-raised-1', need: 4.5 },
  { fg: '--status-residual', bg: '--color-base', need: 4.5 },
]

// The fixture the routine arm is seeded with — wdio.conf.ts copies the COMMITTED
// crates/conductor-run/tests/fixtures/lamps-journal.jsonl into a dedicated runs dir. Its semantics are
// pinned Rust-side by conductor-run/tests/lamps_fixture.rs, so a fixture that stops carrying the
// collision fails at nextest rather than quietly weakening what these assertions can see.
const COLLIDED_P_ID = 'P-019'
const COLLIDED_WORST_LAMP = 'Blocked' // beats the sibling record's Pass under worst-lamp-wins
const UNRUN_TEXT = 'Not yet run'

/** Each coverage row as rendered: its P-ID, its lamp label (empty when unrun), its status text. */
async function coverageRows(): Promise<Array<{ pId: string; lamp: string; status: string }>> {
  return browser.execute(() =>
    Array.from(document.querySelectorAll('[class~="cov__row"]')).map((row) => ({
      pId: (row.querySelector('[class~="cov__pid"]')?.textContent ?? '').trim(),
      lamp: (row.querySelector('[class~="lamp__label"]')?.textContent ?? '').trim(),
      status: (row.querySelector('[class~="cov__status"]')?.textContent ?? '').trim(),
    })),
  )
}

// The closed six-label lamp set (a11y-plan §6 State color tokens · lamp.ts LAMP_ORDER). The
// --status-residual coverage Mode cell is a coverage-mode classification, NOT a seventh lamp.
const LAMP_LABELS = ['Pass', 'Fail', 'HOLD', 'Manual', 'Residual', 'Blocked']

// The spec runs in a wdio WORKER; the envelope is assembled in the LAUNCHER (wdio.conf.ts onComplete),
// and a module global is not shared across that boundary. So the fingerprint tuples ride a file at a
// path both sides derive the same way — no env handle, no shared state.
const VIOLATION_SIDECAR = join(
  dirname(fileURLToPath(import.meta.url)),
  '..',
  '..',
  '..',
  '..',
  '..',
  'runs',
  'a11y',
  '.violations.jsonl',
)

/**
 * The obs-envelope fingerprint tuple: axe rule-id | WCAG SC tag | node selector (a11y-plan §3
 * Structured violation JSON schema). One line per NODE, not per violation — `target` is per-node, and
 * a single `color-contrast` violation has covered 18 of them, so a per-violation tuple would drop 17
 * selectors. Selectors are DOM-relative by construction; a filesystem path can never appear here.
 */
function recordViolationTuples(
  violations: ReadonlyArray<{ id: string; tags: string[]; nodes: ReadonlyArray<{ target: unknown[] }> }>,
): void {
  if (violations.length === 0) return
  const lines = violations.flatMap((v) => {
    const sc = v.tags.find((t) => /^wcag\d+$/.test(t)) ?? v.tags.find((t) => t.startsWith('wcag')) ?? 'wcag-untagged'
    return v.nodes.map((n) => `${v.id}|${sc}|${n.target.join(' ')}`)
  })
  mkdirSync(dirname(VIOLATION_SIDECAR), { recursive: true })
  appendFileSync(VIOLATION_SIDECAR, lines.map((l) => JSON.stringify(l)).join('\n') + '\n', 'utf8')
}

/**
 * A readiness probe must answer within this to count. @axe-core/webdriverio 4.12.1 opens every analysis
 * with `execute(() => document.readyState === 'complete')` raced against a hard FRAME_LOAD_TIMEOUT of
 * 1 000 ms, and any miss — a late true included — throws "Page/Frame is not ready". A classic-WebDriver
 * execute queues behind the page's synchronous JavaScript, and right after #root first mounts the main
 * thread is busy for ~1-2 s (CI#36635281444: the probe answered true 1 173 ms after it was sent). A
 * quarter of axe's budget; settled round-trips measured 5-25 ms in that run.
 */
const RESPONSIVE_MS = 250

/** Wait until axe's own readiness probe answers true well inside axe's budget. */
async function untilResponsive(): Promise<void> {
  await browser.waitUntil(
    async () => {
      const sent = performance.now()
      try {
        const ready = await browser.execute(() => document.readyState === 'complete')
        return ready && performance.now() - sent < RESPONSIVE_MS
      } catch {
        // A thrown execute is a not-yet, as in wdio.conf.ts's before hook: waitUntil treats a throwing
        // condition as fatal.
        return false
      }
    },
    {
      timeout: 30_000,
      interval: 250,
      timeoutMsg: `readiness probe never answered within ${RESPONSIVE_MS} ms in 30s (axe's readiness budget is 1 000 ms)`,
    },
  )
}

/** Every violation, named — a bare `violations.length` failure reports a count and no rule. */
async function axeFindings(): Promise<string[]> {
  await untilResponsive()
  const results = await new AxeBuilder({ client: browser }).withTags(WCAG_TAGS).analyze()
  recordViolationTuples(results.violations)
  return results.violations.map(
    (v) => `${v.id} [${v.impact}] ${v.nodes.length} node(s) — ${v.help}`,
  )
}

/** Resolve a `:root` token from the RUNNING webview's computed style (the rendered truth). */
async function token(name: string): Promise<string> {
  return browser.execute(
    (n: string) => getComputedStyle(document.documentElement).getPropertyValue(n).trim(),
    name,
  )
}

/**
 * Both themes' DECLARED token values, read out of the compiled stylesheet (a11y-plan §3 sanctions
 * resolving pairs "from the compiled CSS custom properties"). getComputedStyle can only ever report
 * the theme the OS is currently in, so the inactive theme is unreachable without this.
 */
async function declaredThemes(): Promise<{ dark: Record<string, string>; light: Record<string, string> }> {
  return browser.execute(() => {
    const dark: Record<string, string> = {}
    const light: Record<string, string> = {}
    const harvest = (rule: CSSStyleRule, into: Record<string, string>) => {
      for (let i = 0; i < rule.style.length; i += 1) {
        const prop = rule.style.item(i)
        if (prop.startsWith('--')) into[prop] = rule.style.getPropertyValue(prop).trim()
      }
    }
    for (const sheet of Array.from(document.styleSheets)) {
      let rules: CSSRule[]
      try {
        rules = Array.from(sheet.cssRules)
      } catch {
        continue
      }
      for (const rule of rules) {
        if (rule instanceof CSSStyleRule && rule.selectorText === ':root') harvest(rule, dark)
        if (rule instanceof CSSMediaRule && rule.conditionText.includes('prefers-color-scheme: light')) {
          for (const inner of Array.from(rule.cssRules)) {
            if (inner instanceof CSSStyleRule && inner.selectorText === ':root') harvest(inner, light)
          }
        }
      }
    }
    // The light block overrides the dark default; unmentioned tokens inherit.
    return { dark, light: { ...dark, ...light } }
  })
}

/**
 * The focusable set the Tab order must cover. ONE definition, read by the walk and by every assertion
 * over it, so the two sides of a reachability comparison cannot drift apart.
 */
const FOCUSABLE_SELECTOR =
  'a[href],button:not([disabled]),input:not([disabled]),select,textarea,[tabindex]:not([tabindex="-1"])'

interface FocusVisit {
  /** Position in the live focusable set; -1 for a stop outside it (host chrome, BODY). */
  index: number
  name: string
}

/**
 * The focused element's INDEX in the live focusable set, its accessible name, and the set's names in DOM
 * order — read in one page call so the three cannot describe different moments.
 *
 * Identity is the index, never the name: two controls can share a name. The coverage and report scroll
 * regions were both unnamed `<div tabindex="0">` (each carries its label on the inner table, deliberately —
 * a named focusable region made NVDA read the whole table in one utterance), so both projected to "DIV" and
 * a name-keyed set silently collapsed six elements into five (measured 2026-09-17). The coverage stop is now
 * its current ROW (roving focus), which projects to "TR" — still unnamed, so the rule stands.
 *
 * An `<input>` is named by its label/placeholder, never by textContent: an input's textContent is ALWAYS
 * empty, which would make the picker's filter box indistinguishable from "no element".
 */
function focusSnapshot(): Promise<{ index: number; name: string; roster: string[] }> {
  return browser.execute((selector: string) => {
    const nameOf = (node: Element | null): string => {
      if (!node) return ''
      const el = node as HTMLElement
      const label = el.getAttribute('aria-label')
      if (label) return label.trim()
      if (el.tagName === 'INPUT') {
        return (el.getAttribute('placeholder') ?? el.getAttribute('role') ?? 'INPUT').trim().slice(0, 60)
      }
      const interactive = ['BUTTON', 'A', 'SELECT', 'TEXTAREA'].includes(el.tagName)
      return interactive ? (el.textContent ?? '').trim().slice(0, 60) : el.tagName
    }
    const set = Array.from(document.querySelectorAll(selector))
    const active = document.activeElement
    return { index: active ? set.indexOf(active) : -1, name: nameOf(active), roster: set.map(nameOf) }
  }, FOCUSABLE_SELECTOR)
}

/**
 * One full lap of the Tab cycle, delimited by the first REPEATED element identity.
 *
 * It deliberately does not wait for a BODY sentinel. This webview's cycle need not contain one: measured
 * 2026-09-17 on runtime 153, Tab walks the six focusables round and round with no BODY stop at all, so a
 * sentinel walk exhausts its cap, begins collecting from an arbitrary position, and reports a truncated
 * visit tally. Both Operable specs failed that way, and the focus-order one's pass on another runtime was
 * luck about where the walk happened to stop.
 *
 * A stop outside the focusable set is recorded in `visits` for the failure message but anchors no lap and
 * joins no index set — a host-chrome stop is legitimate and says nothing about reachability.
 */
async function tabCycle(): Promise<{ cycle: FocusVisit[]; visits: FocusVisit[]; roster: string[] }> {
  const { roster } = await focusSnapshot()
  const visits: FocusVisit[] = []
  const firstSeenAt = new Map<number, number>()
  let lap: FocusVisit[] = []
  for (let i = 0; i < roster.length * 2 + 2; i += 1) {
    await browser.keys('Tab')
    const { index, name } = await focusSnapshot()
    visits.push({ index, name })
    if (index < 0) continue
    const seen = firstSeenAt.get(index)
    if (seen !== undefined) {
      lap = visits.slice(seen, visits.length - 1).filter((v) => v.index >= 0)
      break
    }
    firstSeenAt.set(index, visits.length - 1)
  }
  return { cycle: lap, visits, roster }
}

/** A colour normalized to sRGB hex, so a token's authored spelling and a computed rgb() compare equal. */
function hex(color: string): string {
  try {
    return new Color(color).to('srgb').toString({ format: 'hex' })
  } catch {
    return JSON.stringify(color)
  }
}

function sameColor(a: string, b: string): boolean {
  return hex(a) === hex(b)
}

/**
 * The focused element's computed outline, plus the outline style of the element at `prevIndex` in the live
 * focusable set — the stop just left — read in one page call so both describe the same moment.
 */
function ringAt(
  prevIndex: number,
): Promise<{ index: number; name: string; style: string; color: string; prevStyle: string }> {
  return browser.execute(
    (selector: string, prevAt: number) => {
      const set = Array.from(document.querySelectorAll(selector))
      const el = document.activeElement as HTMLElement | null
      const style = el ? getComputedStyle(el) : null
      const prev = prevAt >= 0 ? set[prevAt] : undefined
      return {
        index: el ? set.indexOf(el) : -1,
        name: el ? `${el.tagName}${el.getAttribute('aria-label') ? `(${el.getAttribute('aria-label')})` : ''}` : 'none',
        style: style?.outlineStyle ?? 'none',
        color: style?.outlineColor ?? '',
        prevStyle: prev ? getComputedStyle(prev).outlineStyle : 'none',
      }
    },
    FOCUSABLE_SELECTOR,
    prevIndex,
  )
}

/**
 * The coverage matrix as the keyboard sees it, in one page call: the focused row's DOM index (-1 off the
 * rows), the rows carrying aria-current / tabindex=0, whether the last row lies inside the scroll region,
 * and the current row's edge and ring colours beside a non-current row's edge.
 */
function matrixState(): Promise<{
  index: number
  rows: number
  current: number[]
  stops: number[]
  lastRowInside: boolean
  currentEdge: string
  otherEdge: string
  currentRing: string
}> {
  return browser.execute(() => {
    const rows = Array.from(document.querySelectorAll<HTMLElement>('[class~="cov__row"]'))
    const where = (pred: (row: HTMLElement) => boolean) => rows.flatMap((row, i) => (pred(row) ? [i] : []))
    const current = where((row) => row.getAttribute('aria-current') === 'true')
    const edge = (row: HTMLElement | undefined) => {
      const pid = row?.querySelector('[class~="cov__pid"]')
      return pid ? getComputedStyle(pid).borderLeftColor : ''
    }
    const currentRow = rows[current[0] ?? -1]
    const otherRow = rows.find((row) => row !== currentRow)
    const region = rows[0]?.closest('[class~="cov__scroll"]')?.getBoundingClientRect()
    const lastRow = rows[rows.length - 1]?.getBoundingClientRect()
    return {
      index: document.activeElement ? rows.indexOf(document.activeElement as HTMLElement) : -1,
      rows: rows.length,
      current,
      stops: where((row) => row.getAttribute('tabindex') === '0'),
      lastRowInside:
        !!region && !!lastRow && lastRow.top >= region.top - 1 && lastRow.bottom <= region.bottom + 1,
      currentEdge: edge(currentRow),
      otherEdge: edge(otherRow),
      currentRing: currentRow ? getComputedStyle(currentRow).outlineColor : '',
    }
  })
}

/** Which mechanism answered the coverage-row name reads — the driver's own endpoint, or the in-page fallback. */
let nameMechanism: 'webdriver computedlabel' | 'in-page aria-labelledby' = 'webdriver computedlabel'

/**
 * The accessible name of the coverage row at `index`, as the browser computes it. The in-page resolution of
 * `aria-labelledby` stands in ONLY when the driver has no computed-label endpoint; any other error is real.
 */
async function coverageRowName(index: number): Promise<string> {
  const row = (await $$('[class~="cov__row"]'))[index]
  try {
    return (await row.getComputedLabel()).trim()
  } catch (e) {
    if (!/unknown command|unknown method|not implemented/i.test(String(e))) throw e
    nameMechanism = 'in-page aria-labelledby'
    return browser.execute((at: number) => {
      const el = document.querySelectorAll('[class~="cov__row"]')[at]
      const text = (node: Node): string => {
        if (node instanceof Element && node.getAttribute('aria-hidden') === 'true') return ''
        if (node.nodeType === Node.TEXT_NODE) return node.textContent ?? ''
        return Array.from(node.childNodes).map(text).join(' ')
      }
      const ids = (el?.getAttribute('aria-labelledby') ?? '').split(/\s+/).filter(Boolean)
      return ids
        .map((id) => {
          const ref = document.getElementById(id)
          return ref ? text(ref) : ''
        })
        .join(' ')
        .replace(/\s+/g, ' ')
        .trim()
    }, index)
  }
}

/** The computed role of every element `selector` matches, in document order. */
async function computedRoles(selector: string): Promise<string[]> {
  const roles: string[] = []
  for (const el of await $$(selector)) roles.push(await el.getComputedRole())
  return roles
}

function ratio(fg: string, bg: string): number {
  return new Color(fg).contrastWCAG21(new Color(bg))
}

/**
 * The compiled reduced-motion rule, as DECLARATIONS (SC 2.3.3 enforcement is CSS-native, not scripted).
 * Read per-property, never by substring: the CSSOM re-serializes `animation: none` into the expanded
 * shorthand (`animation: auto ease 0s 1 normal none running none`), so a cssText match on the authored
 * spelling fails against a perfectly correct rule.
 */
async function reducedMotionDecls(): Promise<
  Array<{ selector: string; animationName: string; transition: string; important: boolean }>
> {
  return browser.execute(() => {
    const out: Array<{ selector: string; animationName: string; transition: string; important: boolean }> = []
    for (const sheet of Array.from(document.styleSheets)) {
      let rules: CSSRule[]
      try {
        rules = Array.from(sheet.cssRules)
      } catch {
        continue
      }
      for (const rule of rules) {
        if (rule instanceof CSSMediaRule && rule.conditionText.includes('prefers-reduced-motion')) {
          for (const inner of Array.from(rule.cssRules)) {
            if (!(inner instanceof CSSStyleRule)) continue
            out.push({
              selector: inner.selectorText,
              animationName: inner.style.animationName,
              transition: inner.style.transitionProperty || inner.style.transition,
              important:
                inner.style.getPropertyPriority('animation') === 'important' &&
                inner.style.getPropertyPriority('transition') === 'important',
            })
          }
        }
      }
    }
    return out
  })
}

describe('desktop a11y — routine arm (no live Pulse)', () => {
  it('zero axe violations on the Minimal-tier baseline', async () => {
    const findings = await axeFindings()
    expect(findings.join(' | ')).toBe('')
  })

  it('axe completes over a 1.5 s main-thread stall', async () => {
    // The readiness race on demand. The NEXT read of document.readyState — the probe axe opens every
    // analysis with — blocks the main thread for 1.5 s and then removes itself, so that probe answers
    // past axe's 1 000 ms budget. A timer-scheduled stall cannot stand in: its setTimeout fires before
    // the scheduling execute's response is delivered, so that call absorbs the whole stall and axe's
    // probe meets an idle thread (measured: stall 1 500 ms, the next probe's round trip 17.7 ms). It
    // fails with "Page/Frame is not ready" whenever axeFindings() stops waiting a slow probe out.
    await browser.execute(() => {
      Object.defineProperty(document, 'readyState', {
        configurable: true,
        get() {
          delete (document as unknown as Record<string, unknown>).readyState
          const until = performance.now() + 1_500
          while (performance.now() < until) {
            // the stall is the subject
          }
          return document.readyState
        },
      })
    })
    const findings = await axeFindings()
    expect(findings.join(' | ')).toBe('')
  })

  it('a Blocked row is never rendered red (count-blocked, not status-fail)', async () => {
    // Blocked uses --count-blocked / slate-violet, never --status-fail (layouts §Primary content block 2).
    const blocked = await token('--count-blocked')
    const fail = await token('--status-fail')
    expect(blocked).not.toBe(fail)
  })

  it('every a11y-plan §6 token pair meets its ratio in the RENDERED theme', async () => {
    const failures: string[] = []
    for (const { fg, bg, need } of [...PAIRS, ...EXTRA_PAIRS]) {
      const r = ratio(await token(fg), await token(bg))
      if (r < need) failures.push(`${fg}/${bg} ${r.toFixed(2)}:1 (need ${need}:1)`)
    }
    expect(failures.join(' | ')).toBe('')
  })

  it('every a11y-plan §6 token pair meets its ratio in BOTH declared themes', async () => {
    const themes = await declaredThemes()
    const failures: string[] = []
    for (const [name, tokens] of Object.entries(themes)) {
      expect(Object.keys(tokens).length).toBeGreaterThan(0)
      for (const { fg, bg, need } of [...PAIRS, ...EXTRA_PAIRS]) {
        const r = ratio(tokens[fg], tokens[bg])
        if (r < need) failures.push(`${name} ${fg}/${bg} ${r.toFixed(2)}:1 (need ${need}:1)`)
      }
    }
    expect(failures.join(' | ')).toBe('')
  })

  it('no lamp conveys state by colour alone, and none is outside the closed six', async () => {
    // Reads what the idle console actually renders. The all-six gallery is DEV-only and stripped from
    // the release bundle, so completeness across the six is the driven arm's + the unit tier's, not this one's.
    const lamps = await browser.execute(() =>
      Array.from(document.querySelectorAll('[class~="lamp"]')).map((el) => ({
        label: (el.querySelector('[class~="lamp__label"]')?.textContent ?? '').trim(),
        glyph: (el.querySelector('[class~="lamp__glyph"]')?.textContent ?? '').trim(),
        glyphHidden:
          el.querySelector('[class~="lamp__glyph"]')?.getAttribute('aria-hidden') === 'true',
      })),
    )
    const bad = lamps
      .filter((l) => !l.label || !l.glyph || !l.glyphHidden || !LAMP_LABELS.includes(l.label))
      .map((l) => `label=${JSON.stringify(l.label)} glyph=${JSON.stringify(l.glyph)} hidden=${l.glyphHidden}`)
    expect(bad.join(' | ')).toBe('')
  })

  it('reduced-motion drops every transition and animation (SC 2.3.3)', async () => {
    const decls = await reducedMotionDecls()
    const universal = decls.find((d) => d.selector === '*')
    expect(universal).toBeDefined()
    expect(universal?.animationName).toBe('none')
    expect(universal?.transition).toBe('none')
    expect(universal?.important).toBe(true)

    // The drive path a11y-plan §6 prescribes — browser.emulate('prefers-reduced-motion', 'reduce') —
    // does NOT exist: webdriverio 9.x implements exactly six emulate scopes (clock · geolocation ·
    // userAgent · device · colorScheme · onLine), so there is no reduced-motion scope on any platform.
    // This is stronger than the plan's "recorded-not-established under classic WebDriver" caveat, and
    // it is why enforcement is asserted CSS-natively above rather than by emulating the preference.
    // The honest limit: this proves the rule SHIPS and is correctly shaped, not that the OS preference
    // was exercised — SC 2.3.3 conformance is not claimed from it alone.
  })

  it('a coverage row carries its run outcome, worst-lamp-wins where records collide', async function () {
    const rows = await coverageRows()
    expect(rows.length).toBeGreaterThan(0)
    if (!rows.some((r) => r.lamp)) {
      // Subject absent: no run record reached the app, so no row can carry a verdict state. The
      // fixture seed (wdio.conf.ts) is what supplies it; skipping here is never a pass.
      this.skip()
    }
    // Names what it observed on BOTH failure modes — a missing row and a wrong lamp read differently.
    const collided = rows.find((r) => r.pId === COLLIDED_P_ID)
    const observed = collided
      ? `${collided.pId}=${collided.lamp}`
      : `${COLLIDED_P_ID} absent from [${rows.map((r) => r.pId).join(',')}]`
    expect(observed).toBe(`${COLLIDED_P_ID}=${COLLIDED_WORST_LAMP}`)
  })

  it('a capability with no run record reads as prose, distinct from a failure by TEXT', async function () {
    const rows = await coverageRows()
    if (!rows.some((r) => r.lamp)) this.skip() // subject absent: nothing ran, every row is unrun

    const unrun = rows.filter((r) => r.status === UNRUN_TEXT)
    const failed = rows.filter((r) => r.lamp === 'Fail')
    const problems: string[] = []
    if (unrun.length === 0) problems.push(`no row rendered "${UNRUN_TEXT}"`)
    if (failed.length === 0) problems.push('the fixture rendered no Fail lamp to contrast against')
    // The discriminator is the rendered TEXT, not the tint — an unrun row must never read as a failure.
    for (const row of unrun) {
      if (failed.some((f) => f.status === row.status)) {
        problems.push(`${row.pId} unrun status "${row.status}" matches a failed row's`)
      }
      if (LAMP_LABELS.includes(row.status)) {
        problems.push(`${row.pId} unrun status "${row.status}" is a lamp label`)
      }
    }
    expect(problems.join(' | ')).toBe('')
  })

  it('a coverage row is named by its own cells — its P-ID and its status (SR rows S0-09, E0-05)', async () => {
    // The expected halves are the cells' DOM text and the fixture literals, never the name under test.
    const rows = await coverageRows()
    const first = rows[0]
    expect(first?.pId ?? '(no row)').toMatch(/^P-\d{3}$/)
    const firstName = await coverageRowName(0)
    expect(firstName).toContain(first.pId)
    expect(firstName).toContain(first.lamp || first.status)

    const at = rows.findIndex((r) => r.pId === COLLIDED_P_ID)
    expect(at >= 0 ? COLLIDED_P_ID : `${COLLIDED_P_ID} absent`).toBe(COLLIDED_P_ID)
    const collidedName = await coverageRowName(at)
    expect(collidedName).toContain(COLLIDED_P_ID)
    expect(collidedName).toContain(COLLIDED_WORST_LAMP)
    console.log(`[a11y] coverage-row names read via ${nameMechanism}`)
  })

  it('the phase line is the single level-1 heading (a11y-plan §4)', async () => {
    const selector = 'h1,h2,h3,h4,h5,h6,[role="heading"]'
    const roles = await computedRoles(selector)
    const found = await browser.execute(
      (sel: string) =>
        Array.from(document.querySelectorAll<HTMLElement>(sel)).map((el) => ({
          level: Number(el.getAttribute('aria-level') ?? el.tagName.replace(/^H/, '')),
          text: (el.textContent ?? '').trim(),
        })),
      selector,
    )
    const level1 = found.filter((h, i) => roles[i] === 'heading' && h.level === 1).map((h) => h.text)
    const phase = (await $('[class~="titlebar__label"]').getText()).trim()
    expect(`level-1 headings [${level1.join(' | ')}]`).toBe(`level-1 headings [${phase}]`)
  })

  it('the window exposes exactly one main and one contentinfo landmark (a11y-plan §4)', async () => {
    const roles = await computedRoles('header,main,footer,nav,aside,[role]')
    const count = (role: string) => roles.filter((r) => r === role).length
    expect(`main ${count('main')} · contentinfo ${count('contentinfo')}`).toBe('main 1 · contentinfo 1')
  })

  it('an over-envelope run banners its standing with the label in the DOM', async () => {
    // NO subject-absent skip here any more. The fixture seed persists an ENVIRONMENT-SUSPECT envelope
    // row through conductor_run::persist (wdio.conf.ts → conductor-run's envelope_fixture seeder), so
    // an absent banner now means the seed failed rather than that the run was legitimately
    // in-envelope — and a skip would restore exactly the green-over-nothing this subject removes.
    const banner = await $('[class~="report__envelope"]')
    expect(await banner.isExisting()).toBe(true)

    const label = await $('[class~="report__envelope-label"]')
    expect(await label.getText()).toBe('ENVIRONMENT-SUSPECT')
  })

  it('the over-envelope banner state carries no axe violation', async () => {
    // The banner's contrast rode a render-independent token-pair assertion until this subject existed
    // (a11y-plan §6); this is the first time the rendered state itself is measured. Findings are
    // rendered, never counted — a bare length assertion names no rule (testing.md 2026-09-01).
    expect(await $('[class~="report__envelope"]').isExisting()).toBe(true)
    const findings = await axeFindings()
    expect(findings.join('\n')).toBe('')
  })

  // --- Operable, hold-free half (a11y-plan §5 run-console-idle). The trap/restoration half needs a
  // raised hold and stays in the driven suite; these two need only the idle console, which is why they
  // can sit on the CI-gated routine arm at all.

  it('every idle-console control is keyboard-reachable by Tab alone (SC 2.1.1)', async () => {
    const { cycle, roster } = await tabCycle()
    // Both directions over element identity: every focusable is reached, and the reached set holds nothing
    // else. The received half comes from the walk and the expected half from the DOM — never from each
    // other, which is what the predecessor's `${names}` on both sides made impossible to fail.
    // The VISIT count is deliberately unasserted: it varies with where the cycle wraps, which is an
    // environment property and not a reachability one (a11y-plan §11 → CI).
    const reached = [...new Set(cycle.map((v) => v.index))].sort((a, b) => a - b)
    expect(`reached ${reached.length}/${roster.length} [${reached.map((i) => roster[i]).join(' | ')}]`).toBe(
      `reached ${roster.length}/${roster.length} [${roster.join(' | ')}]`,
    )
    const reachedNames = reached.map((i) => roster[i])
    const missing = ['Minimize window', 'Close window'].filter((n) => !reachedNames.includes(n))
    expect(`missing: ${missing.join(', ') || 'none'}`).toBe('missing: none')
  })

  it('idle focus order follows the run-console-idle layout: window controls, then the picker (SC 2.4.3)', async () => {
    const { cycle, roster } = await tabCycle()
    // a11y-plan §5 run-console-idle fixes the visible order, and layout-templates §Component — Header
    // renders minimize before close; focus order must match what is seen, not merely contain both.
    // Compared as a ROTATION: the cycle is a ring, so where a walk ENTERS it is arbitrary while the
    // sequence read from the document's first focusable is not.
    const start = cycle.findIndex((v) => v.index === 0)
    const rotated = start >= 0 ? [...cycle.slice(start), ...cycle.slice(0, start)] : cycle
    expect(`tab order [${rotated.map((v) => v.index).join(',')}]`).toBe(
      `tab order [${roster.map((_, i) => i).join(',')}]`,
    )
    expect(`first two [${roster.slice(0, 2).join(' | ')}]`).toBe(
      'first two [Minimize window | Close window]',
    )
    // No positive tabindex anywhere — the order must come from DOM order (a11y-plan §11 Keyboard).
    const positiveTabindex = await browser.execute(
      () =>
        Array.from(document.querySelectorAll('[tabindex]')).filter(
          (el) => Number(el.getAttribute('tabindex')) > 0,
        ).length,
    )
    expect(positiveTabindex).toBe(0)
  })

  it('the focused control shows a visible --color-focus ring and the control it left shows none (SC 2.4.7)', async () => {
    // One full lap by the same first-repeated-identity rule as tabCycle. The negative witness is the
    // control just LEFT: a ring drawn on every element regardless of focus would pass the first half alone.
    const ring = await token('--color-focus')
    const focus = (color: string) => sameColor(ring, color)
    const { roster, index: startedOn } = await focusSnapshot()
    let prev = startedOn
    const firstSeen = new Set<number>()
    const failures: string[] = []
    let stops = 0
    for (let i = 0; i < roster.length * 2 + 2; i += 1) {
      await browser.keys('Tab')
      const at = await ringAt(prev)
      if (at.index < 0) continue
      if (firstSeen.has(at.index)) break
      firstSeen.add(at.index)
      stops += 1
      if (at.style === 'none' || !focus(at.color)) {
        failures.push(`${at.name}: outline ${at.style} ${hex(at.color)}`)
      }
      if (prev >= 0 && at.prevStyle !== 'none') failures.push(`${roster[prev]} kept outline ${at.prevStyle} after focus left`)
      prev = at.index
    }
    expect(`${stops}/${roster.length} stops · ${failures.join(' | ') || 'no failures'}`).toBe(
      `${roster.length}/${roster.length} stops · no failures`,
    )
  })

  it('coverage-matrix rows navigate by Arrow keys and Home/End through one tab stop, the current row marked aria-current with a --border-emphasis edge (idle-with-report)', async () => {
    const { roster } = await focusSnapshot()
    let at = await matrixState()
    for (let i = 0; i < roster.length + 2 && at.index < 0; i += 1) {
      await browser.keys('Tab')
      at = await matrixState()
    }
    // One tab stop for the region: the first row, and no other row, is in the Tab order.
    expect(`row ${at.index} of ${at.rows} · stops [${at.stops.join(',')}]`).toBe(`row 0 of ${at.rows} · stops [0]`)

    // The expected half is the key's documented move over the DOM row count, never the focus read.
    const last = at.rows - 1
    const moves: ReadonlyArray<[string, (i: number) => number]> = [
      ['ArrowDown', (i) => Math.min(i + 1, last)],
      ['ArrowDown', (i) => Math.min(i + 1, last)],
      ['ArrowUp', (i) => Math.max(i - 1, 0)],
      ['End', () => last],
      ['Home', () => 0],
    ]
    let expected = at.index
    const failures: string[] = []
    for (const [key, move] of moves) {
      if (key === 'End' && at.lastRowInside) failures.push('the last row was already inside the scroll region before End')
      await browser.keys(key)
      expected = move(expected)
      at = await matrixState()
      const read = `${key} → row ${at.index} current [${at.current.join(',')}] stops [${at.stops.join(',')}]`
      if (read !== `${key} → row ${expected} current [${expected}] stops [${expected}]`) failures.push(read)
      if (key === 'End' && !at.lastRowInside) failures.push('End did not bring the last row inside the scroll region')
    }

    // The current row's edge is --border-emphasis, a non-current row's is not, and the ring differs from it.
    const emphasis = await token('--border-emphasis')
    if (!sameColor(emphasis, at.currentEdge)) failures.push(`current edge ${hex(at.currentEdge)} ≠ --border-emphasis ${hex(emphasis)}`)
    if (sameColor(emphasis, at.otherEdge)) failures.push(`a non-current row's edge is --border-emphasis too`)
    if (sameColor(at.currentEdge, at.currentRing)) failures.push(`the ring ${hex(at.currentRing)} is the edge colour`)
    expect(failures.join(' | ')).toBe('')

    const findings = await axeFindings()
    expect(findings.join(' | ')).toBe('')
  })

  it('the console declares its shortcut map: aria-keyshortcuts on Start and Stop and a visible hint line', async () => {
    // The substitute CI gate for the driven arm's shortcut claim (claim-ownership.ts): it proves the map is
    // declared and discoverable, not that a keypress acts — Start would really start a run here.
    const declared = await browser.execute(() =>
      Array.from(document.querySelectorAll('[aria-keyshortcuts]')).map(
        (el) => `${(el.textContent ?? '').trim()}=${el.getAttribute('aria-keyshortcuts')}`,
      ),
    )
    // The whole declared set, so a single-letter or a platform binding (Ctrl+W, Alt+F4) cannot slip in.
    expect(declared.sort().join(' | ')).toBe('Start=Control+Enter | Stop=Control+.')
    const hint = await browser.execute(
      () => document.querySelector('[class~="run-controls__hint"]')?.textContent ?? '(no hint line)',
    )
    expect(hint).toBe('Ctrl+Enter start · proceed   Ctrl+. stop   Esc abort')
  })

  // --- subject-absent on an idle console: skip with a reason, never fail, never vacuously pass ---

  // Its four claims — alertdialog role, focus trap, Escape→NoGo, focus restoration — are owned by the
  // DRIVEN arm (test/a11y/claim-ownership.ts). This marker holds the expected-skip SET at two; it asserts
  // nothing and must not be read as covering them, which is what its previous title implied.
  it('operator-pause dialog is subject-absent on an idle console (owner: driven arm)', async function () {
    if (!(await $('[role="alertdialog"]').isExisting())) {
      this.skip() // subject absent: no hold is raised without a live preflight-ready Pulse
    }
  })

  it('operator-checklist: unticked count is role=status and a row toggles on Space', async function () {
    // Key on the hold marker, not on the roll-up's own role: `role=status` is a shared and growing
    // population (a11y-plan §4 live regions), so a bare `[role="status"]` stops meaning "a hold is
    // raised" the moment anything else adopts it — measured 2026-09-04, when two new live regions
    // satisfied it and this spec asserted checkboxes it found none of.
    if (!(await $('[role="alertdialog"]').isExisting())) {
      this.skip() // subject absent: checklist rows render only inside a raised hold
    }
    expect((await $$('input[type="checkbox"]')).length).toBeGreaterThan(0)
  })
})
