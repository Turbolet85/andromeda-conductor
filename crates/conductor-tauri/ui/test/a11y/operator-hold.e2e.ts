// Desktop a11y sweep — the DRIVEN arm (a11y-plan §3 Focus management · §5 run-console-live · Focus trap /
// Focus restoration · Per-surface keyboard shortcuts).
//
// Operator/local only, never a CI gate: it needs a live preflight-ready Pulse, because the operator hold
// fires only on a READY gate with a non-Blocked read-back and a declare-only scenario
// (conductor-run/src/lib.rs). wdio.conf.ts serves this suite a trimmed catalog of the two [[checklist]]
// scenarios, so ONE run reaches two holds behind ONE preflight canary — a second run would dedupe against
// the first run's open canary incident (verification-harness.md 2026-08-16).
//
// Run it as: npm run a11y:driven  (with ANDROMEDA_PULSE_MCP_ENABLED / _L4_DETERMINISTIC / _DATA_DIR set and
// the sidecar on PATH).
//
// This arm exists because a skip is not a pass: the keyboard clauses are affordance claims, and
// verification-matrix-contract §Affordance honesty requires the REAL control be exercised — so these
// assertions drive actual keypresses against the real app, never a programmatic proxy. Hold decisions are
// graded from the backend's own log, because Proceed and Abort both close the dialog.
import { browser, $, expect } from '@wdio/globals'
import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

// The hold sits behind preflight's canary poll, which by contract outlasts Pulse's L3 digest cadence
// (pulse-run-contract [incident_formation].min_canary_poll_seconds). A long wait is the design, not a hang.
const HOLD_TIMEOUT_MS = 12 * 60 * 1000

// The second hold follows halo-hue-encoding's 180 s of emission plus its read-back.
const SECOND_HOLD_TIMEOUT_MS = 6 * 60 * 1000

// The Tauri backend's self-obs sink for this suite: runs_dir.parent()/logs (obs-plan §3 Log file location),
// with CONDUCTOR_RUNS_DIR = runs/driven/runs (wdio.conf.ts). Derived from this file like the routine arm's
// violation sidecar, so no env handle carries it.
const BACKEND_LOG = join(
  dirname(fileURLToPath(import.meta.url)),
  '..',
  '..',
  '..',
  '..',
  '..',
  'runs',
  'driven',
  'logs',
  'conductor-tauri.jsonl',
)

function backendLogLines(): string[] {
  return existsSync(BACKEND_LOG) ? readFileSync(BACKEND_LOG, 'utf8').split('\n').filter(Boolean) : []
}

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

/**
 * Where focus sits relative to the coverage matrix, read in one page call: the focused row's DOM index
 * (-1 when focus is not on a row) and P-ID, the row count, and which rows carry aria-current / tabindex=0.
 * Identity is the DOM index and the P-ID text — every row projects to TR, so a name cannot tell them apart.
 */
function matrixFocus(): Promise<{ index: number; pId: string; rows: number; current: number[]; stops: number[] }> {
  return browser.execute(() => {
    const rows = Array.from(document.querySelectorAll('[class~="cov__row"]'))
    const active = document.activeElement
    const index = active ? rows.indexOf(active) : -1
    const pIdOf = (row: Element | undefined) =>
      (row?.querySelector('[class~="cov__pid"]')?.textContent ?? '').trim()
    const where = (pred: (row: Element) => boolean) =>
      rows.flatMap((row, i) => (pred(row) ? [i] : []))
    return {
      index,
      pId: index >= 0 ? pIdOf(rows[index]) : '',
      rows: rows.length,
      current: where((row) => row.getAttribute('aria-current') === 'true'),
      stops: where((row) => row.getAttribute('tabindex') === '0'),
    }
  })
}

async function phaseLine(): Promise<string> {
  return (await $('[class~="titlebar__label"]').getText()).trim()
}

async function count(): Promise<string> {
  return (await $('[class~="titlebar__count"]').getText()).trim()
}

async function expectPhaseLine(text: string, timeout: number): Promise<void> {
  await browser.waitUntil(async () => (await phaseLine()) === text, {
    timeout,
    interval: 200,
    timeoutMsg: `the phase line never read "${text}" (reads "${await phaseLine()}")`,
  })
}

async function waitForHold(timeout: number): Promise<void> {
  const dialog = await $('[role="alertdialog"]')
  await browser.waitUntil(async () => dialog.isDisplayed().catch(() => false), {
    timeout,
    interval: 1000,
    timeoutMsg:
      `no operator hold was raised within ${timeout / 1000}s — the driven arm requires a live ` +
      'preflight-ready Pulse (MCP enabled, deterministic L4, shared data dir) and the sidecar on PATH; ' +
      'a Blocked preflight returns before the hold fires',
  })
  // Trap entry (SC 2.1.2): focus moves into the dialog when it opens.
  await browser.waitUntil(focusInsideDialog, {
    timeout: 10_000,
    interval: 200,
    timeoutMsg: 'focus never entered the alertdialog on open',
  })
}

async function waitForDialogGone(what: string): Promise<void> {
  const dialog = await $('[role="alertdialog"]')
  await browser.waitUntil(async () => !(await dialog.isDisplayed().catch(() => false)), {
    timeout: 10_000,
    interval: 200,
    timeoutMsg: `${what} did not dismiss the alertdialog`,
  })
}

describe('desktop a11y — driven arm (live Pulse raises a real operator hold)', () => {
  it('one live run: Ctrl+Enter starts it, its coverage matrix row-navigates while live and the count never takes focus, the HOLD dialog traps focus, Space toggles a row, Escape resolves No-Go and restores focus, Ctrl+. stops it and Ctrl+Enter proceeds the next hold', async () => {
    // 1. Baseline the backend log (it persists across runs, so only lines added after this count are read)
    // and select the suite over the trimmed catalog — a full catalog could never start here by accident.
    const logBaseline = backendLogLines().length
    const suite = await $('[role="option"][aria-label^="Suite — all scenarios"]')
    await expect(suite).toBeExisting()
    expect(await suite.getAttribute('aria-label')).toContain('2 scenarios')
    await suite.click()

    // 2. Start by shortcut from a matrix row — a non-activating position (a11y-plan §5). Tab is bounded by
    // the focusable count, so a row that never takes focus fails here instead of walking forever.
    const focusable = await browser.execute(
      () => document.querySelectorAll('a[href],button:not([disabled]),input:not([disabled]),[tabindex]').length,
    )
    let onRow = false
    for (let i = 0; i < focusable + 2 && !onRow; i += 1) {
      await browser.keys('Tab')
      onRow = (await matrixFocus()).index >= 0
    }
    const entry = await matrixFocus()
    expect(`on a coverage row: ${onRow} (${entry.index}/${entry.rows})`).toBe(
      `on a coverage row: true (${entry.index}/${entry.rows})`,
    )
    await browser.keys(['Control', 'Enter'])
    await expectPhaseLine('Conductor · live', 15_000)
    expect(`still on ${(await matrixFocus()).pId}`).toBe(`still on ${entry.pId}`)

    // 3. Navigate rows while live. The expected half is computed from the DOM row count and the key's
    // documented move (±1, clamped), never from the focus read under test.
    let expected = entry.index
    for (const key of ['ArrowDown', 'ArrowDown', 'ArrowUp'] as const) {
      await browser.keys(key)
      expected = key === 'ArrowDown' ? Math.min(expected + 1, entry.rows - 1) : Math.max(expected - 1, 0)
      const at = await matrixFocus()
      expect(`${key} → row ${at.index} current [${at.current.join(',')}] stops [${at.stops.join(',')}]`).toBe(
        `${key} → row ${expected} current [${expected}] stops [${expected}]`,
      )
    }
    const invoker = await matrixFocus()

    // 4. Hold 1 — trap, containment, Space, Escape → No-Go, restoration to the invoking row.
    await waitForHold(HOLD_TIMEOUT_MS)
    const visited: string[] = []
    for (let i = 0; i < 8; i += 1) {
      await browser.keys('Tab')
      expect(await focusInsideDialog()).toBe(true)
      visited.push(await activeName())
    }
    await browser.keys(['Shift', 'Tab'])
    expect(await focusInsideDialog()).toBe(true)
    expect(visited.filter((n) => n.length > 0).length).toBeGreaterThan(0)

    const checkbox = await $('[role="alertdialog"] input[type="checkbox"]')
    await expect(checkbox).toBeExisting()
    const rollup = await $('[role="status"]')
    await expect(rollup).toBeExisting()
    // Tab to the row rather than clicking it: a click on a checkbox IS a toggle, so clicking to "focus" it
    // and then pressing Space toggles twice and reads as "Space did nothing".
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

    await browser.keys('Escape')
    await waitForDialogGone('Escape')
    // Focus restoration (SC 2.4.3), compared by P-ID: the rows all project to TR.
    let restored = await matrixFocus()
    try {
      await browser.waitUntil(
        async () => {
          restored = await matrixFocus()
          return restored.pId === invoker.pId
        },
        { timeout: 10_000, interval: 200 },
      )
    } catch {
      // The rendered comparison below names where focus landed instead.
    }
    expect(`restored to ${restored.pId || `non-row (${await activeName()})`}`).toBe(`restored to ${invoker.pId}`)

    // 5. The count update never takes focus (SC 4.1.3): the first scenario's progress event sets it to 1
    // right after the hold resolves, which is exactly when focus has just been restored.
    await browser.waitUntil(async () => (await count()) === '1', {
      timeout: 120_000,
      interval: 250,
      timeoutMsg: `the first scenario never settled (count reads "${await count()}")`,
    })
    expect(`focus after the count update: ${(await matrixFocus()).pId}`).toBe(
      `focus after the count update: ${invoker.pId}`,
    )

    // 6. Stop by shortcut. The backend polls abort between scenarios, so the second still runs to its hold.
    await browser.keys(['Control', '.'])
    await expectPhaseLine('Conductor · aborted', 10_000)

    // 7. Hold 2 — Proceed by shortcut.
    await waitForHold(SECOND_HOLD_TIMEOUT_MS)
    await browser.keys(['Control', 'Enter'])
    await waitForDialogGone('Ctrl+Enter')

    // 8. Each decision discriminated by the backend's own lines (execute.rs: "operator-checklist hold
    // resolved by tauri-dialog: {label} (…)"; the No-Go label carries its hyphen, so ": Go (" cannot match
    // it), and the abort by drive.rs's "run aborted by the operator".
    await browser.waitUntil(
      async () => backendLogLines().slice(logBaseline).some((l) => l.includes('run aborted by the operator')),
      {
        timeout: 60_000,
        interval: 500,
        timeoutMsg: 'the backend never logged "run aborted by the operator" after Ctrl+.',
      },
    )
    const added = backendLogLines().slice(logBaseline)
    const noGo = added.filter((l) => l.includes(': No-Go (')).length
    const go = added.filter((l) => l.includes(': Go (')).length
    expect(`No-Go ${noGo} · Go ${go}`).toBe('No-Go 1 · Go 1')
  })
})
