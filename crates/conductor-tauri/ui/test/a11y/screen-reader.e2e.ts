// Screen-reader pass — the SR leg (a11y-plan §3 Screen reader test pattern), agent-driven.
//
// Operator/local only, never a CI gate. Three subjects over the ONE WebdriverIO + tauri-driver stack, chosen
// by the invoked suite (`npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error`) and recorded by wdio.conf.ts in
// runs/sr-leg/subject.txt. A portable NVDA (CONDUCTOR_NVDA) is started by the config BEFORE the app, logging
// every utterance; this spec stamps a row of screen-reader/rows.ts before each action it drives, so the parser
// can assign what NVDA spoke to the row that provoked it. It asserts DOM facts only (the same role/text handles
// the driven arm uses) — what NVDA said is the parser's to grade and the operator's to review.
//
// Two things this leg MEASURES rather than assumes: whether NVDA attaches while WebDriver holds the session
// (a silent log through the focus rows), and whether driver-injected keys reach NVDA's browse-mode commands
// (`h` / `d` / arrows are sent only while focus sits on a non-editable control, so a miss is harmless).
import { browser, $, expect } from '@wdio/globals'
import { spawnSync } from 'node:child_process'
import { appendFileSync, existsSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { rowsFor, type SpecRow, type Subject } from './screen-reader/rows'

const here = dirname(fileURLToPath(import.meta.url))
const repoRoot = join(here, '..', '..', '..', '..', '..')
const legDir = join(repoRoot, 'runs', 'sr-leg')

function readSubject(): Subject {
  const file = join(legDir, 'subject.txt')
  const raw = existsSync(file) ? readFileSync(file, 'utf8').trim() : ''
  return raw === 'empty' || raw === 'error' ? raw : 'live'
}

const subject = readSubject()
const speechLog = join(legDir, `nvda-speech.${subject}.log`)
const actionsPath = join(legDir, `actions.${subject}.jsonl`)
const rows = new Map(rowsFor(subject).map((r) => [r.id, r]))

// The first hold sits behind the in-run preflight canary poll (~50s measured on the driven arm); the second
// follows the second scenario's phases. Long waits are the design — every one is bounded and named.
const HOLD_TIMEOUT_MS = 12 * 60 * 1000
const SECOND_HOLD_TIMEOUT_MS = 6 * 60 * 1000

function row(id: string): SpecRow {
  const r = rows.get(id)
  if (!r) throw new Error(`no ${subject} row ${id} in screen-reader/rows.ts`)
  return r
}

function logSize(): number {
  try {
    return statSync(speechLog).size
  } catch {
    return 0
  }
}

/** The session's own record: the WebView2 version the driver actually attached to (the registry can lag it). */
function beginTimeline(): void {
  const version = browser.capabilities.browserVersion ?? 'unknown'
  writeFileSync(actionsPath, JSON.stringify({ ts: new Date().toISOString(), id: '@session', browserVersion: version }) + '\n')
}

const WINDOW_TITLE = 'Conductor' // tauri.conf.json app.windows[0].title

/**
 * NVDA announces the OS foreground window's focus, and a window launched by a background process never
 * takes the foreground on its own (measured 2026-09-02: NVDA read the Pulse window while the driven app's
 * DOM focus moved unannounced). Activate the app window before the first stamp, then wait for NVDA's log to
 * name it — a readiness signal, bounded; silence past it is recorded by the parser, never hidden.
 */
async function bringToForeground(preFocusRow?: string): Promise<{ activated: boolean; nvdaNamedWindow: boolean }> {
  const script = join(here, 'screen-reader', 'activate-window.ps1')
  const result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', script, '-Title', WINDOW_TITLE], {
    stdio: 'ignore',
    windowsHide: true,
    timeout: 20_000,
  })
  const activated = result.status === 0
  // Start every walk from <body>: with a populated catalog the app's initial focus already sits in the
  // picker input (measured 2026-09-02 — the first Tab landed on Start), and the rows assume the Tab order
  // from the document start.
  const initialFocus = await activeName()
  // Chromium keeps a sequential-focus-navigation starting point apart from activeElement, and cycling Tab
  // to the host-chrome stop (which reads as BODY) is what resets it — the row walk then starts from the
  // document start. That cycling DESTROYS the evidence of where the point sat, so the first Tab is taken
  // and described here, before the loop consumes it. Measured 2026-09-07 on the error subject:
  // firstTabTarget "Minimize window", focusableIndex 0/5, tabsToStart 5 — the point sits at the document
  // start and the walk covers every focusable exactly once. The 2026-09-04 reading that a control "held
  // focus and lost it (the picker input at mount)" left the point mid-list does not reproduce; that
  // subject has no mount-time focus at all (ScenarioPicker has no focus site and cmdk's own .focus() is
  // guarded), and tabsToStart 5 over 5 focusables is a full cycle, not a partial one.
  // A row whose subject fires ON the first focus event (the load-error re-assertion) is stamped HERE,
  // between activation and that first Tab. Reading it from the session start instead would work, but
  // would drag the activation window in with it — where NVDA announces the WebView2 host window's own
  // title, an OS-owned string carrying a host path that the ingest scrub then flags. The narrower
  // window keeps the record's 0-security_finding baseline intact.
  if (preFocusRow) stamp(preFocusRow, 'the first focus event after load (Tab)')
  await tab()
  const firstTabTarget = await activeName()
  const firstTabProbe = await activeProbe()
  let tabsToStart = 1
  if (firstTabTarget !== 'BODY') {
    for (let i = 1; i < 10; i += 1) {
      await tab()
      tabsToStart += 1
      if ((await activeName()) === 'BODY') break
    }
  }
  const sizeBefore = logSize()
  let named = false
  try {
    // NVDA names the foregrounded window when its first FOCUS event arrives, not on activation (measured
    // 2026-09-02: "Conductor · document" was spoken with the first Tab, never before), so this wait is short
    // and the first focus row carries the window announcement in its own window.
    await browser.waitUntil(
      () => {
        if (logSize() <= sizeBefore) return false
        named = existsSync(speechLog) && readFileSync(speechLog, 'utf8').slice(sizeBefore).includes(WINDOW_TITLE)
        return named
      },
      { timeout: 4_000, interval: 200 },
    )
  } catch {
    // NVDA did not name the window within the bound — the parser's attach verdict will say so
  }
  await quiet(6_000)
  appendFileSync(
    actionsPath,
    JSON.stringify({ ts: new Date().toISOString(), id: '@foreground', activated, nvdaNamedWindow: named, initialFocus, firstTabTarget, firstTabProbe, tabsToStart }) + '\n',
  )
  return { activated, nvdaNamedWindow: named }
}

/** Stamp the row BEFORE its action; the parser owns everything NVDA speaks from here to the next stamp. */
function stamp(id: string, action: string): number {
  const r = row(id)
  appendFileSync(actionsPath, JSON.stringify({ ts: new Date().toISOString(), id: r.id, cls: r.cls, action }) + '\n')
  return logSize()
}

/** Wait for the speech log to stop growing (NVDA has said what it will say), bounded. */
async function quiet(maxMs: number): Promise<void> {
  let last = logSize()
  let stableSince = Date.now()
  try {
    await browser.waitUntil(
      () => {
        const now = logSize()
        if (now !== last) {
          last = now
          stableSince = Date.now()
        }
        return Date.now() - stableSince >= 700
      },
      { timeout: maxMs, interval: 100 },
    )
  } catch {
    // still talking past the bound — the next stamp closes this row's window regardless
  }
}

/**
 * Wait for NVDA to have logged something since the stamp, then for it to go quiet, so the next stamp does
 * not cut a row's own speech into its successor. Silence is a legitimate outcome, not a failure.
 */
async function settle(sizeAtStamp: number): Promise<void> {
  try {
    await browser.waitUntil(() => logSize() > sizeAtStamp, { timeout: 4_000, interval: 100 })
  } catch {
    // no speech followed within the window — the parser records the row as not-announced
    return
  }
  await quiet(4_000)
}

/** Close the timeline: whatever NVDA says after this (window teardown, the shell regaining focus) belongs to no row. */
function endTimeline(): void {
  appendFileSync(actionsPath, JSON.stringify({ ts: new Date().toISOString(), id: '@end' }) + '\n')
}

async function act(id: string, action: string, drive: () => Promise<void>): Promise<void> {
  const before = stamp(id, action)
  await drive()
  await settle(before)
}

/** The focused element's accessible name — the driven arm's helper, bounded the same way. */
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

/**
 * A BOUNDED structural projection of the focused element — tag, id, role, label, testid and its index
 * among the document's focusables. Never `textContent`: a BODY fallback would dump the whole rendered
 * document into a diagnostic line. Used to identify the sequential-focus starting point's holder.
 */
function activeProbe(): Promise<string> {
  return browser.execute(() => {
    const el = document.activeElement as HTMLElement | null
    if (!el) return 'none'
    const focusables = Array.from(
      document.querySelectorAll<HTMLElement>(
        'a[href],button,input,select,textarea,[tabindex]:not([tabindex="-1"])',
      ),
    )
    const parts = [
      el.tagName,
      el.id ? `#${el.id}` : '',
      el.getAttribute('role') ? `role=${el.getAttribute('role')}` : '',
      el.getAttribute('aria-label') ? `label=${el.getAttribute('aria-label')}` : '',
      el.getAttribute('data-testid') ? `testid=${el.getAttribute('data-testid')}` : '',
      `focusableIndex=${focusables.indexOf(el)}/${focusables.length}`,
    ]
    return parts.filter(Boolean).join(' ').slice(0, 200)
  })
}

function activeAriaDisabled(): Promise<string | null> {
  return browser.execute(() => (document.activeElement as HTMLElement | null)?.getAttribute('aria-disabled') ?? null)
}

function activeIsCombobox(): Promise<boolean> {
  return browser.execute(() => {
    const el = document.activeElement as HTMLElement | null
    return !!el && el.tagName === 'INPUT' && el.getAttribute('role') === 'combobox'
  })
}

function activeDescendantText(): Promise<string> {
  return browser.execute(() => {
    const id = (document.activeElement as HTMLElement | null)?.getAttribute('aria-activedescendant')
    return id ? (document.getElementById(id)?.textContent ?? '').trim() : ''
  })
}

function activeIsCheckbox(): Promise<boolean> {
  return browser.execute(() => {
    const el = document.activeElement as HTMLInputElement | null
    return !!el && el.tagName === 'INPUT' && el.type === 'checkbox'
  })
}

function focusInsideDialog(): Promise<boolean> {
  return browser.execute(
    () => !!(document.activeElement as HTMLElement | null)?.closest('[role="alertdialog"]'),
  )
}

async function expectActive(name: string): Promise<void> {
  await browser.waitUntil(async () => (await activeName()) === name, {
    timeout: 5_000,
    interval: 100,
    timeoutMsg: `focus did not land on "${name}" (on "${await activeName()}")`,
  })
}

/**
 * The coverage / run-report scroll regions are focusable so the keyboard can scroll them, but they
 * carry no name of their own: a focusable `role="group"` computed the whole table as its accessible
 * content and read every row in one utterance (S0-09, E0-05/E0-06). The name now sits on the table
 * they contain. This asserts BOTH halves — focus landed on the scroll container, AND that container's
 * table is the named one — so it is stricter than the name-only check it replaces.
 */
async function expectActiveScrollRegion(name: string): Promise<void> {
  const landed = (): Promise<boolean> =>
    browser.execute((expected: string) => {
      const el = document.activeElement as HTMLElement | null
      if (!el || el.getAttribute('tabindex') !== '0') return false
      return el.querySelector('table')?.getAttribute('aria-label') === expected
    }, name)
  await browser.waitUntil(landed, {
    timeout: 5_000,
    interval: 100,
    timeoutMsg: `focus did not land on the "${name}" scroll region (on "${await activeName()}")`,
  })
}

/** The first Tab of a walk must reach the first control; on a miss, record where the next Tabs go and fail. */
async function expectFirstTabLanding(name: string): Promise<void> {
  const landed = await activeName()
  if (landed === name) return
  const landings = [landed]
  for (let i = 0; i < 6; i += 1) {
    await tab()
    landings.push(await activeName())
  }
  appendFileSync(actionsPath, JSON.stringify({ ts: new Date().toISOString(), id: '@probe', tabLandings: landings }) + '\n')
  throw new Error(`the first Tab landed on "${landed}", not "${name}"; the next Tabs went ${landings.slice(1).join(' → ')}`)
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
    interval: 1_000,
    timeoutMsg:
      `no operator hold was raised within ${timeout / 1000}s — the live subject needs a preflight-ready ` +
      'Pulse (MCP enabled, deterministic L4, shared data dir) and the sidecar on PATH',
  })
  await browser.waitUntil(focusInsideDialog, {
    timeout: 10_000,
    interval: 200,
    timeoutMsg: 'focus never entered the alertdialog on open',
  })
}

async function waitForDialogGone(): Promise<void> {
  const dialog = await $('[role="alertdialog"]')
  await browser.waitUntil(async () => !(await dialog.isDisplayed().catch(() => false)), {
    timeout: 10_000,
    interval: 200,
    timeoutMsg: 'the alertdialog did not close',
  })
}

async function tab(): Promise<void> {
  await browser.keys('Tab')
}

async function shiftTab(): Promise<void> {
  await browser.keys(['Shift', 'Tab'])
}

async function browseKey(key: string): Promise<void> {
  // Browse-mode commands are attempted only from a non-editable control, so a key that never reaches NVDA
  // is harmless to the page (it would type into an input otherwise).
  if (await activeIsCombobox()) return
  await browser.keys(key)
}

if (subject === 'live') {
  describe('screen-reader pass — live subject (idle → live → hold → aborted)', () => {
    before(() => {
      beginTimeline()
    })
    after(() => {
      endTimeline()
    })

    it('drives the four shipped run states while NVDA logs what it would speak', async () => {
      await bringToForeground()
      // S0 — idle on the trimmed catalog with an empty runs dir
      await act('S0-11', 'none (browse row)', async () => {
        expect(await phaseLine()).toBe('Conductor · idle')
      })
      await act('S0-12', 'none (browse row)', async () => {
        expect(await count()).toBe('00:00:00')
      })
      await act('S0-01', 'Tab', async () => {
        await tab()
        await expectFirstTabLanding('Minimize window')
      })
      await act('S0-02', 'Tab', async () => {
        await tab()
        await expectActive('Close window')
      })
      await act('S0-03', 'Tab', async () => {
        await tab()
        await browser.waitUntil(activeIsCombobox, { timeout: 5_000, timeoutMsg: 'Tab did not reach the picker input' })
      })
      await act('S0-16', 'type "zzz" (filter miss), then Backspace ×3', async () => {
        await browser.keys('zzz')
        const empty = await $('[class~="picker__empty"]')
        await browser.waitUntil(async () => empty.isDisplayed().catch(() => false), {
          timeout: 5_000,
          timeoutMsg: 'the picker never showed its filter-miss prose',
        })
        for (let i = 0; i < 3; i += 1) await browser.keys('Backspace')
        await browser.waitUntil(async () => !(await empty.isDisplayed().catch(() => false)), {
          timeout: 5_000,
          timeoutMsg: 'the filter-miss prose did not clear',
        })
      })
      await act('S0-04', 'ArrowDown', async () => {
        await browser.keys('ArrowDown')
        await browser.waitUntil(async () => (await activeDescendantText()).includes('halo-breathing-encoding'), {
          timeout: 5_000,
          timeoutMsg: 'ArrowDown did not move the active option to halo-breathing-encoding',
        })
      })
      await act('S0-05', 'ArrowDown', async () => {
        await browser.keys('ArrowDown')
        await browser.waitUntil(async () => (await activeDescendantText()).includes('halo-hue-encoding'), {
          timeout: 5_000,
          timeoutMsg: 'ArrowDown did not move the active option to halo-hue-encoding',
        })
      })
      await act('S0-06', 'ArrowUp ×2 to "Suite — all scenarios", Enter', async () => {
        await browser.keys('ArrowUp')
        await browser.keys('ArrowUp')
        await browser.waitUntil(async () => (await activeDescendantText()).includes('Suite'), {
          timeout: 5_000,
          timeoutMsg: 'ArrowUp did not return to the suite option',
        })
        await browser.keys('Enter')
        const chosen = await $('[role="option"][aria-current="true"]')
        await browser.waitUntil(async () => (await chosen.getText().catch(() => '')).includes('selected'), {
          timeout: 5_000,
          timeoutMsg: 'Enter did not commit the suite selection',
        })
      })
      await act('S0-07', 'Tab', async () => {
        await tab()
        await expectActive('Start')
        expect(await activeAriaDisabled()).not.toBe('true')
      })
      await act('S0-08', 'none (browse row — Stop is natively disabled)', async () => {
        await expect(await $('button=Stop')).toBeExisting()
      })
      await act('S0-09', 'Tab', async () => {
        await tab()
        await expectActiveScrollRegion('Coverage rows')
      })
      await act('S0-13', 'h (browse-mode next heading)', () => browseKey('h'))
      await act('S0-14', 'd (browse-mode next landmark)', () => browseKey('d'))
      await act('S0-15', 'ArrowDown (browse-mode next line)', () => browseKey('ArrowDown'))
      await act('S0-10', 'none (browse row)', async () => {
        await expect(await $('p=No run yet')).toBeExisting()
      })

      // S1 — Start
      await act('S1-01', 'Shift+Tab to Start, Enter', async () => {
        await shiftTab()
        await expectActive('Start')
        await browser.keys('Enter')
        await expectPhaseLine('Conductor · live', 15_000)
      })
      // No re-activation here: the sidecar spawn now suppresses its console, so the run start no longer
      // hands the OS foreground away (measured 2026-09-04 — the S1-01 window lost the host-path, terminal
      // and pane utterances the prior record carried). Re-activating anyway was actively harmful once the
      // pane was gone: the script's synthetic ALT landed on the already-foreground app and opened its
      // System menu, whose modal message loop froze the webview (NVDA spoke "System subMenu"; every later
      // WebDriver command timed out).
      await act('S1-02', 'none (browse row — the count reads "0")', async () => {
        expect(await count()).toBe('0')
      })
      await act('S1-05', 'none (subject absent)', async () => {})
      await act('S1-03', 'Shift+Tab, Tab (refocus Start)', async () => {
        await shiftTab()
        await tab()
        await expectActive('Start')
        expect(await activeAriaDisabled()).toBe('true')
      })
      await act('S1-04', 'Tab', async () => {
        await tab()
        await expectActive('Stop')
      })

      // S2 — the first hold (halo-hue-encoding)
      {
        const before = stamp('S2-01', 'Shift+Tab back to Start, then wait for the hold (the in-run preflight canary poll)')
        stamp('S2-02', 'the dialog opens in the same instant (shared window)')
        // The hold restores focus to whatever was focused when it arrived; the rows expect Start.
        await shiftTab()
        await expectActive('Start')
        await waitForHold(HOLD_TIMEOUT_MS)
        await expectPhaseLine('Conductor · HOLD — operator pause', 5_000)
        await settle(before)
      }
      await act('S2-06', 'none (browse row)', async () => {
        expect((await $('[role="status"]').getText()).trim()).toBe('1 of 1 unconfirmed')
      })
      await act('S2-03', 'Tab', async () => {
        await tab()
        await expectActive('Proceed')
      })
      await act('S2-04', 'Tab', async () => {
        await tab()
        await browser.waitUntil(activeIsCheckbox, { timeout: 5_000, timeoutMsg: 'Tab did not reach the checklist row' })
      })
      await act('S2-05', 'Space', async () => {
        const checkbox = await $('[role="alertdialog"] input[type="checkbox"]')
        const rollup = await $('[role="status"]')
        await browser.keys('Space')
        await browser.waitUntil(async () => checkbox.isSelected(), {
          timeout: 5_000,
          timeoutMsg: 'Space did not toggle the focused checklist row',
        })
        await browser.waitUntil(async () => (await rollup.getText()).trim() === 'All observations confirmed', {
          timeout: 5_000,
          timeoutMsg: 'the role=status roll-up did not read "All observations confirmed"',
        })
      })
      {
        const before = stamp('S2-07', 'Shift+Tab to Proceed, Enter (Go); focus restores to Start')
        stamp('S2-08', 'the phase line returns to live in the same instant (shared window)')
        await shiftTab()
        await expectActive('Proceed')
        await browser.keys('Enter')
        await waitForDialogGone()
        await expectActive('Start')
        await expectPhaseLine('Conductor · live', 10_000)
        await settle(before)
      }

      // S3 — Stop during the second scenario, then the second hold, then the Aborted stage
      await browser.waitUntil(async () => (await count()) === '1', {
        timeout: 120_000,
        interval: 250,
        timeoutMsg: 'the first scenario never settled (count did not reach 1)',
      })
      await act('S3-01', 'Tab to Stop, Enter', async () => {
        await tab()
        await expectActive('Stop')
        await browser.keys('Enter')
        await expectPhaseLine('Conductor · aborted', 10_000)
      })
      await act('S3-02', 'Shift+Tab', async () => {
        await shiftTab()
        await expectActive('Start')
        expect(await activeAriaDisabled()).not.toBe('true')
      })
      await act('S3-03', 'wait for the second hold (halo-breathing-encoding)', async () => {
        await waitForHold(SECOND_HOLD_TIMEOUT_MS)
      })
      {
        // The Aborted stage fires as the Escape resolution returns (measured 2026-09-02: "Conductor · live"
        // then "Conductor · aborted" inside one second), so the two rows share a window.
        const before = stamp('S3-04', 'Escape (NoGo); focus restores to Start')
        stamp('S3-05', 'the Aborted stage and the report reload follow in the same instant (shared window)')
        await browser.keys('Escape')
        await waitForDialogGone()
        await expectActive('Start')
        const summary = await $('[class~="report__summary"]')
        await browser.waitUntil(
          async () =>
            (await summary.getText().catch(() => '')).includes('2 scenarios') &&
            (await phaseLine()) === 'Conductor · aborted',
          {
            timeout: 90_000,
            interval: 500,
            timeoutMsg: 'the run never settled to aborted with a two-scenario report',
          },
        )
        await settle(before)
      }
      await act('S3-06', 'ArrowDown (browse-mode next line)', () => browseKey('ArrowDown'))
      await act('S3-07', 'ArrowDown (browse-mode next line)', () => browseKey('ArrowDown'))
      stamp('T-01', 'not run (the live subject is stopped by design)')
    })
  })
}

if (subject === 'empty') {
  describe('screen-reader pass — empty subject (empty catalog, seeded fixture report)', () => {
    before(() => {
      beginTimeline()
    })
    after(() => {
      endTimeline()
    })

    it('reads the empty-catalog prose and the fixture report', async () => {
      await bringToForeground()
      await act('E0-01', 'none (browse row)', async () => {
        await expect(await $('p=No scenarios found.')).toBeExisting()
      })
      await act('E0-02', 'Tab', async () => {
        await tab()
        await expectFirstTabLanding('Minimize window')
      })
      await act('E0-03', 'Tab', async () => {
        await tab()
        await expectActive('Close window')
      })
      await act('E0-04', 'Tab', async () => {
        await tab()
        await expectActive('Start')
        expect(await activeAriaDisabled()).toBe('true')
      })
      await act('E0-05', 'Tab', async () => {
        await tab()
        await expectActiveScrollRegion('Coverage rows')
      })
      await act('E0-06', 'Tab', async () => {
        await tab()
        await expectActiveScrollRegion('Run report rows')
      })
      await act('E0-07', 'ArrowDown (browse-mode next line)', () => browseKey('ArrowDown'))
      await act('E0-08', 'ArrowDown (browse-mode next line)', () => browseKey('ArrowDown'))
      await act('E0-09', 'Shift+Tab, ArrowDown (browse-mode next line)', async () => {
        await shiftTab()
        await expectActiveScrollRegion('Coverage rows')
        await browseKey('ArrowDown')
      })
      stamp('E0-10', 'none (subject absent)')
    })
  })
}

if (subject === 'error') {
  describe('screen-reader pass — error subject (malformed catalog)', () => {
    before(() => {
      beginTimeline()
    })
    after(() => {
      endTimeline()
    })

    it('hears the load-error alert and the unavailable Start control', async () => {
      await bringToForeground('R0-01')
      // The row grades the load error REACHING the user. Mounting the region empty was necessary and not
      // sufficient: NVDA binds a window on its first focus event, which necessarily follows a load-time
      // paint, so the app re-asserts the message into the (empty) region on that first focusin and the row
      // is stamped immediately before it. The reload this action used to perform is gone — it could not
      // tell a first-load announcement from a post-reload one, and it silently reset the sequential-focus
      // start point besides. Silence here is a FINDING, never a pass.
      await quiet(4_000)
      // R0-01 was stamped inside bringToForeground(), immediately before the first focus event its
      // subject fires on — so no act() here, which would re-stamp and split the row's own window. The
      // DOM assertion still runs: it proves the subject exists whatever NVDA did or did not say, which
      // is what keeps "was it announced" and "is it there" separate questions.
      const alert = await $('[role="alert"]')
      await expect(alert).toBeExisting()
      const visible = await $('p*=Could not load scenarios')
      await expect(visible).toBeExisting()
      await act('R0-02', 'Tab', async () => {
        await tab()
        await expectFirstTabLanding('Minimize window')
      })
      await act('R0-03', 'Tab', async () => {
        await tab()
        await expectActive('Close window')
      })
      await act('R0-04', 'Tab', async () => {
        await tab()
        await expectActive('Start')
        expect(await activeAriaDisabled()).toBe('true')
      })
    })
  })
}
