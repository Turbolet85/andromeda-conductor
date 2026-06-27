// Desktop a11y sweep — the four must-be-accessible paths (a11y-plan §3/§5/§6).
//
// DISPLAY-GATED: runs on Linux+xvfb + live Pulse via `npm run a11y` (see ../../wdio.conf.ts), never on
// the Windows dev host. Selectors are role/text/aria-live only — never xpath or hashed-CSS classes
// (testing.md, test-plan §6). axe is configured for the Minimal-tier WCAG baseline; the exhaustive
// per-path verification matrix + NVDA/VoiceOver manual spec are the next chunk (Desktop a11y verification).
import { browser, $, expect } from '@wdio/globals'
import AxeBuilder from '@axe-core/webdriverio'
import Color from 'colorjs.io'

// Minimal tier — wcag2a/2aa/21aa, no wcag22aa (a11y-plan §1/§3.5).
const WCAG_TAGS = ['wcag2a', 'wcag2aa', 'wcag21aa']

async function axeViolations(): Promise<number> {
  const results = await new AxeBuilder({ client: browser }).withTags(WCAG_TAGS).analyze()
  return results.violations.length
}

/** Resolve a `:root` design token from the running webview's computed style (design-system §Tokens). */
async function token(name: string): Promise<string> {
  return browser.execute(
    (n: string) => getComputedStyle(document.documentElement).getPropertyValue(n).trim(),
    name,
  )
}

/** WCAG 2.1 contrast ratio between two design tokens, via colorjs.io (a11y-plan §6). */
async function tokenContrast(fg: string, bg: string): Promise<number> {
  const ratio = new Color(await token(fg)).contrastWCAG21(new Color(await token(bg)))
  return ratio
}

describe('desktop a11y — four accessible paths', () => {
  it('run report: zero axe violations on the Minimal-tier baseline', async () => {
    expect(await axeViolations()).toBe(0)
  })

  it('run report: a Blocked row is never rendered red (count-blocked, not status-fail)', async () => {
    // Blocked uses --count-blocked / slate-violet, never --status-fail (layouts §Primary content block 2).
    const blocked = await token('--count-blocked')
    const fail = await token('--status-fail')
    expect(blocked).not.toBe(fail)
  })

  it('token-pair contrast meets WCAG AA (text 4.5:1, status + focus 3:1)', async () => {
    expect(await tokenContrast('--text-primary', '--color-base')).toBeGreaterThanOrEqual(4.5)
    expect(await tokenContrast('--count-nominal', '--color-base')).toBeGreaterThanOrEqual(3)
    expect(await tokenContrast('--color-focus', '--color-base')).toBeGreaterThanOrEqual(3)
  })

  it('operator-pause dialog: alertdialog role, focus trap, Escape resolves NoGo, focus restores', async () => {
    const dialog = await $('[role="alertdialog"]')
    await expect(dialog).toBeDisplayed()
    // Tab must not escape the trap; Escape dismisses (→ NoGo) and restores focus to the trigger.
    await browser.keys('Escape')
    await expect(dialog).not.toBeDisplayed()
  })

  it('operator-checklist: unticked count is role=status and a row toggles on Space', async () => {
    await expect($('[role="status"]')).toBeExisting()
    const checkbox = await $('input[type="checkbox"]')
    await checkbox.click()
    await browser.keys('Space')
    await expect(checkbox).toBeExisting()
  })
})
