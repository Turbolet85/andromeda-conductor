// Desktop a11y sweep — the DRIVEN arm (a11y-plan §3 Focus management · §5 Focus trap / Focus restoration).
//
// Operator/local only, never a CI gate: it needs a live preflight-ready Pulse, because the operator hold
// fires only on a READY gate with a non-Blocked read-back and a declare-only scenario
// (conductor-run/src/lib.rs). `halo-hue-encoding` is the cheapest scenario declaring a [[checklist]].
//
// Run it as: npm run a11y:driven  (with ANDROMEDA_PULSE_MCP_ENABLED / _L4_DETERMINISTIC / _DATA_DIR set).
//
// This arm exists because a skip is not a pass: v2-22's keyboard clauses are affordance claims, and
// verification-matrix-contract §Affordance honesty requires the REAL control be exercised — so these
// assertions drive actual keypresses against the real dialog, never a programmatic proxy.
import { browser, $, expect } from '@wdio/globals'

const SCENARIO = 'halo-hue-encoding'

// The hold sits behind preflight's canary poll, which by contract outlasts Pulse's L3 digest cadence
// (pulse-run-contract [incident_formation].min_canary_poll_seconds). A long wait is the design, not a hang.
const HOLD_TIMEOUT_MS = 12 * 60 * 1000

/**
 * The focused element's accessible name — role/text handles only; ui/src carries no data-testid.
 * Bounded: a fallback to textContent on a container (BODY, say) would otherwise return the entire
 * rendered document, which drowns any failure message that reports it.
 */
function activeName(): Promise<string> {
  return browser.execute(() => {
    const el = document.activeElement as HTMLElement | null
    if (!el) return ''
    const label = el.getAttribute('aria-label')
    if (label) return label.trim()
    const interactive = ['BUTTON', 'INPUT', 'A', 'SELECT', 'TEXTAREA'].includes(el.tagName)
    return interactive ? (el.textContent ?? '').trim().slice(0, 60) : el.tagName
  })
}

/** Is focus currently inside the hold dialog? (SC 2.1.2 trap containment.) */
function focusInsideDialog(): Promise<boolean> {
  return browser.execute(
    () => !!(document.activeElement as HTMLElement | null)?.closest('[role="alertdialog"]'),
  )
}

describe('desktop a11y — driven arm (live Pulse raises a real operator hold)', () => {
  it('the real HOLD dialog traps focus, Space toggles a row, Escape resolves NoGo and restores focus', async () => {
    const option = await $(`[role="option"]*=${SCENARIO}`)
    await expect(option).toBeExisting()
    await option.click()

    // Start signals availability with aria-disabled, not the native attribute (it must stay focusable
    // to be a focus-restore target), so isEnabled() would pass vacuously — assert the real signal.
    const start = await $('button=Start')
    await expect(start).toBeExisting()
    expect(await start.getAttribute('aria-disabled')).not.toBe('true')
    const invokerName = (await start.getText()).trim()
    await start.click()

    const dialog = await $('[role="alertdialog"]')
    await browser.waitUntil(async () => dialog.isDisplayed().catch(() => false), {
      timeout: HOLD_TIMEOUT_MS,
      interval: 1000,
      timeoutMsg:
        `no operator hold was raised within ${HOLD_TIMEOUT_MS / 1000}s — the driven arm requires a live ` +
        'preflight-ready Pulse (MCP enabled, deterministic L4, shared data dir); a Blocked preflight ' +
        'returns before the hold fires',
    })

    // Trap entry (SC 2.1.2): focus moves into the dialog when it opens.
    await browser.waitUntil(focusInsideDialog, {
      timeout: 10_000,
      interval: 200,
      timeoutMsg: 'focus never entered the alertdialog on open',
    })

    // Tab must CYCLE within the trap, never escape it. Walk further than the control count so a leak
    // would show up as focus landing outside.
    const visited: string[] = []
    for (let i = 0; i < 8; i += 1) {
      await browser.keys('Tab')
      expect(await focusInsideDialog()).toBe(true)
      visited.push(await activeName())
    }
    await browser.keys(['Shift', 'Tab'])
    expect(await focusInsideDialog()).toBe(true)

    // Space toggles the focused checklist row, and the unticked roll-up (role=status) reflects it.
    const checkbox = await $('[role="alertdialog"] input[type="checkbox"]')
    await expect(checkbox).toBeExisting()
    const rollup = await $('[role="status"]')
    await expect(rollup).toBeExisting()

    // Tab to the row rather than clicking it: a click on a checkbox IS a toggle, so clicking to
    // "focus" it and then pressing Space toggles twice and lands back where it started — which reads
    // as "Space did nothing". The acceptance is about Space on a FOCUSED row, so reach it by keyboard.
    let reached = false
    for (let i = 0; i < 12 && !reached; i += 1) {
      reached = await browser.execute(() => {
        const el = document.activeElement as HTMLInputElement | null
        return !!el && el.tagName === 'INPUT' && el.type === 'checkbox'
      })
      if (!reached) await browser.keys('Tab')
    }
    expect(reached).toBe(true)

    const before = await checkbox.isSelected()
    const rollupBefore = (await rollup.getText()).trim()
    await browser.keys('Space')
    await browser.waitUntil(async () => (await checkbox.isSelected()) !== before, {
      timeout: 5_000,
      interval: 100,
      timeoutMsg: 'Space did not toggle the focused checklist row',
    })
    await browser.waitUntil(async () => (await rollup.getText()).trim() !== rollupBefore, {
      timeout: 5_000,
      interval: 100,
      timeoutMsg: 'the role=status unticked roll-up did not announce the toggle',
    })

    // Escape escapes the trap (SC 2.1.2) and resolves NoGo.
    await browser.keys('Escape')
    await browser.waitUntil(async () => !(await dialog.isDisplayed().catch(() => false)), {
      timeout: 10_000,
      interval: 200,
      timeoutMsg: 'Escape did not dismiss the alertdialog',
    })

    // Focus restoration (SC 2.4.3): back to the control that was focused when the hold opened.
    let restored = ''
    let landed = { tag: '', name: '', disabled: false }
    try {
      await browser.waitUntil(
        async () => {
          restored = await activeName()
          return restored === invokerName
        },
        { timeout: 10_000, interval: 200 },
      )
    } catch {
      landed = await browser.execute(() => {
        const el = document.activeElement as HTMLElement | null
        return {
          tag: el?.tagName ?? 'none',
          name: (el?.getAttribute('aria-label') ?? el?.textContent ?? '').trim().slice(0, 40),
          disabled: !!(el as HTMLButtonElement | null)?.disabled,
        }
      })
    }
    expect(
      `restored=${JSON.stringify(restored)} landed_on=${landed.tag}` +
        `${landed.name ? `(${landed.name})` : ''}${landed.disabled ? ' [disabled]' : ''}`,
    ).toBe(`restored=${JSON.stringify(invokerName)} landed_on=`)

    expect(visited.filter((n) => n.length > 0).length).toBeGreaterThan(0)
  })
})
