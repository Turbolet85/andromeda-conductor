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

/** Every violation, named — a bare `violations.length` failure reports a count and no rule. */
async function axeFindings(): Promise<string[]> {
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
 * The focused element's accessible name — a SHORT projection, never a non-interactive node's text.
 * An `<input>` is named by its label/placeholder, never by textContent: an input's textContent is
 * ALWAYS empty, which would make the picker's filter box indistinguishable from "no element" and
 * silently truncate any Tab walk that passes through it.
 */
function activeName(): Promise<string> {
  return browser.execute(() => {
    const el = document.activeElement as HTMLElement | null
    if (!el) return ''
    const label = el.getAttribute('aria-label')
    if (label) return label.trim()
    if (el.tagName === 'INPUT') {
      return (el.getAttribute('placeholder') ?? el.getAttribute('role') ?? 'INPUT').trim().slice(0, 60)
    }
    const interactive = ['BUTTON', 'A', 'SELECT', 'TEXTAREA'].includes(el.tagName)
    return interactive ? (el.textContent ?? '').trim().slice(0, 60) : el.tagName
  })
}

/**
 * The accessible names the Tab order visits, in order, from the document start. Chromium keeps a
 * sequential-focus-navigation starting point apart from `activeElement`, so the walk first cycles to
 * the host-chrome stop (which reads as BODY) — otherwise the order depends on whatever held focus
 * last and the spec would measure the previous test's leftovers.
 */
async function tabCycleNames(): Promise<string[]> {
  for (let i = 0; i < 20; i += 1) {
    await browser.keys('Tab')
    if ((await activeName()) === 'BODY') break
  }
  const names: string[] = []
  for (let i = 0; i < 20; i += 1) {
    await browser.keys('Tab')
    const name = await activeName()
    if (name === 'BODY') break
    names.push(name)
  }
  return names
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
    const names = await tabCycleNames()
    // The cycle must cover the DOM's own focusable set: a control it never reaches is unreachable by
    // keyboard, so a newly added one cannot silently fall out of the Tab order. The observed names ride
    // INTO the asserted value — a bare boolean names nothing in the diff (frontend.md 2026-09-02).
    const focusableCount = await browser.execute(
      () =>
        document.querySelectorAll(
          'a[href],button:not([disabled]),input:not([disabled]),select,textarea,[tabindex]:not([tabindex="-1"])',
        ).length,
    )
    expect(`${names.length} reached [${names.join(' | ')}]`).toBe(
      `${focusableCount} reached [${names.join(' | ')}]`,
    )
    const missing = ['Minimize window', 'Close window'].filter((n) => !names.includes(n))
    expect(`missing: ${missing.join(', ') || 'none'} · order: ${names.join(' | ')}`).toBe(
      `missing: none · order: ${names.join(' | ')}`,
    )
  })

  it('idle focus order follows the run-console-idle layout: window controls, then the picker (SC 2.4.3)', async () => {
    const names = await tabCycleNames()
    // a11y-plan §5 run-console-idle fixes the visible order, and layout-templates §Component — Header
    // renders minimize before close; focus order must match what is seen, not merely contain both.
    expect(`${names[0]} then ${names[1]} · full: ${names.join(' | ')}`).toBe(
      `Minimize window then Close window · full: ${names.join(' | ')}`,
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

  // --- subject-absent on an idle console: skip with a reason, never fail, never vacuously pass ---

  it('operator-pause dialog: alertdialog role, focus trap, Escape resolves NoGo, focus restores', async function () {
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
