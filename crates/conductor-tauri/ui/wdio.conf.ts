// WebdriverIO + @crabnebula/tauri-driver config for the desktop a11y sweep (Epoch 9 ch9).
//
// DRIVER-GATED, not display-gated. The earlier "Linux + xvfb only; does NOT run on the Windows dev
// host (no display)" framing was a CI-runner assumption, measured false on 2026-09-01: the
// win32-x64-msvc tauri-driver prebuilt ships in node_modules and runs here, and Windows support is
// the driver's own --native-driver option (a11y-plan §3 Configuration names msedgedriver as that
// backend). What the leg actually needs is (1) a native WebDriver named by CONDUCTOR_MSEDGEDRIVER and
// (2) a RELEASE build of conductor-tauri — a debug build loads tauri.conf.json's devUrl, never the
// bundled frontendDist. Without (1) the leg SKIPS at exit 0 with a recipe, so an unconfigured host
// stays distinguishable from a real defect. The always-on a11y CI gate remains a separate route chunk.
//
// ONE webview-automation stack: WebdriverIO + @crabnebula/tauri-driver (never a 2nd puppeteer/CDP
// stack — a11y.md §Testing). axe-core is injected via @axe-core/webdriverio into THIS session.
import { spawn, type ChildProcess } from 'node:child_process'
import { existsSync, statSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

let tauriDriver: ChildProcess | undefined

// The package is "type": "module", so the CJS globals are absent at load time — `__dirname` here was
// unreachable for as long as this config existed, which the display gate hid by never running it.
const here = dirname(fileURLToPath(import.meta.url))
const require = createRequire(import.meta.url)

// The workspace root. The app resolves its CONDUCTOR_* artifact handles (scenarios / runs / contracts)
// RELATIVE TO ITS CWD through resolve_under, which rejects absolute paths and `..` by design — so the
// launch cwd, not an env handle, is what decides whether the driven app can see the scenario catalog.
// Left at the ui package (npm's cwd) the picker resolves an ui/scenarios that does not exist and
// list_scenarios errors before it logs (measured 2026-09-01).
const repoRoot = join(here, '..', '..', '..')

// The built bundle the driver attaches to (release profile; ensure `cargo build --release` first).
const application = join(
  repoRoot,
  'target',
  'release',
  process.platform === 'win32' ? 'conductor-tauri.exe' : 'conductor-tauri',
)

// Resolved from the installed package rather than the bare name `tauri-driver`: the npm shim is not on
// PATH on this host, and security-plan §Security Anti-Patterns requires a fixed name or a repo-derived
// constant — never an operator-supplied command string.
const driverCli = require.resolve('@crabnebula/tauri-driver/cli.js')

// The native WebDriver is host-owned and sits outside both audited dependency trees, so it arrives by
// env handle and is never committed. Metacharacters are rejected even though the spawn below is
// array-form and reaches no shell — the hardened .env(...) sidecar precedent (security-plan §Input).
const UNSAFE_PATH = /[;&|`$<>\r\n"']/

function nativeDriver(): string | undefined {
  const raw = process.env.CONDUCTOR_MSEDGEDRIVER
  if (!raw || UNSAFE_PATH.test(raw)) return undefined
  return existsSync(raw) && statSync(raw).isFile() ? raw : undefined
}

// Host paths never reach this message (obs-plan §11 Logs); it names the handle, not its value.
function reportSkip(): void {
  console.log('error: webview self-verify skipped — no native WebDriver resolved')
  console.log('       CONDUCTOR_MSEDGEDRIVER is unset, or does not name an existing file.')
  console.log('hint:  setx CONDUCTOR_MSEDGEDRIVER "<dir>\\msedgedriver.exe"   (then reopen the shell)')
  console.log('hint:  to fetch one, the installed edgedriver devDep can download it — but match the')
  console.log('       WEBVIEW2 RUNTIME version, NOT the Edge browser version. edgedriver keys its')
  console.log('       download on Edge, and the two differ by a major on this host (measured')
  console.log('       2026-09-01: Edge 152.x against WebView2 Runtime 151.x).')
}

export const config: WebdriverIO.Config = {
  hostname: '127.0.0.1',
  port: 4444,
  // Default = the ROUTINE arm only: what an idle console proves without a live Pulse, so `--e2e` stays
  // unattended. The driven arm needs a preflight-ready Pulse and raises a real operator hold, so it is a
  // named suite an operator invokes (`npm run a11y:driven`) — one session either way, never a second stack.
  specs: ['./test/a11y/accessibility.e2e.ts'],
  suites: {
    driven: ['./test/a11y/operator-hold.e2e.ts'],
  },
  maxInstances: 1,
  capabilities: [
    // tauri-driver reads the vendor-prefixed `tauri:options` capability — a WebDriver vendor extension
    // (colon-prefixed) outside wdio's typed capability surface, so it is asserted through `unknown`
    // (the documented @crabnebula/tauri-driver shape).
    // enforceWebDriverClassic: wdio v9 prefers BiDi when the driver advertises it, but script
    // evaluation over BiDi against the wry/WebView2 context answers "Page/Frame is not ready"
    // indefinitely, while the SAME calls over classic WebDriver return normally (measured
    // 2026-09-01 against both transports on one session).
    {
      browserName: 'wry',
      'wdio:enforceWebDriverClassic': true,
      'tauri:options': { application },
    } as unknown as WebdriverIO.Capabilities,
  ],
  reporters: ['spec'],
  framework: 'mocha',
  // The ceiling must clear the driven arm: its hold sits behind preflight's canary poll, which by
  // contract outlasts Pulse's L3 digest cadence (~50s observed). wdio enforces this value itself, so an
  // in-test this.timeout() does not raise it. It is only a ceiling — every wait in both specs carries its
  // own bounded timeout and a named message, so a real stall still fails fast and says what stalled.
  mochaOpts: { ui: 'bdd', timeout: 15 * 60_000 },

  // Bracket the session with the tauri-driver bridge process.
  onPrepare: () => {
    const native = nativeDriver()
    if (!native) {
      // Exit rather than throw: wdio has no "skip the whole run" result, and a throw here would
      // surface an unconfigured host as a failed gate — the one reading the guard must prevent.
      reportSkip()
      process.exit(0)
    }
    // cwd = the workspace root: tauri-driver's child (the app under test) inherits it, which is the
    // only lever that points the app's cwd-relative artifact handles at the real catalog.
    tauriDriver = spawn(process.execPath, [driverCli, '--native-driver', native], {
      cwd: repoRoot,
      stdio: [null, process.stdout, process.stderr],
    })
  },
  // The specs assert against a rendered app, but a fresh session attaches before the webview has
  // mounted — without this every spec fails on an empty document ("Page/Frame is not ready", tokens
  // reading ""), which says nothing about the app. Wait on an explicit signal, never a sleep
  // (test-plan §11 E2E).
  before: async () => {
    await browser.waitUntil(
      async () => {
        try {
          return await browser.execute(
            () =>
              document.readyState === 'complete' && !!document.querySelector('#root')?.firstChild,
          )
        } catch {
          // The webview rejects script evaluation ("Page/Frame is not ready") for the first moments
          // after the session attaches. waitUntil treats a THROWING condition as fatal, so swallow it
          // and let the poll retry — otherwise the wait aborts instantly instead of waiting at all.
          return false
        }
      },
      { timeout: 30_000, interval: 500, timeoutMsg: 'webview did not mount #root within 30s' },
    )
  },
  onComplete: () => {
    tauriDriver?.kill()
  },
}
