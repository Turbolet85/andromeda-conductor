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
]

// The closed six-label lamp set (a11y-plan §6 State color tokens · lamp.ts LAMP_ORDER). The
// --status-residual coverage Mode cell is a coverage-mode classification, NOT a seventh lamp.
const LAMP_LABELS = ['Pass', 'Fail', 'HOLD', 'Manual', 'Residual', 'Blocked']

/** Every violation, named — a bare `violations.length` failure reports a count and no rule. */
async function axeFindings(): Promise<string[]> {
  const results = await new AxeBuilder({ client: browser }).withTags(WCAG_TAGS).analyze()
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

  // --- subject-absent on an idle console: skip with a reason, never fail, never vacuously pass ---

  it('operator-pause dialog: alertdialog role, focus trap, Escape resolves NoGo, focus restores', async function () {
    if (!(await $('[role="alertdialog"]').isExisting())) {
      this.skip() // subject absent: no hold is raised without a live preflight-ready Pulse
    }
  })

  it('operator-checklist: unticked count is role=status and a row toggles on Space', async function () {
    if (!(await $('[role="status"]').isExisting())) {
      this.skip() // subject absent: checklist rows render only inside a raised hold
    }
    expect((await $$('input[type="checkbox"]')).length).toBeGreaterThan(0)
  })
})
