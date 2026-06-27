// WebdriverIO + @crabnebula/tauri-driver config for the desktop a11y sweep (Epoch 9 ch9).
//
// DISPLAY-GATED — this runs ONLY on Linux + xvfb against a live Pulse (a11y-plan §3.5, a11y.md
// §Testing); it does NOT run on the Windows dev host (no display) nor in the always-on CI gate. The
// always-on a11y CI gate (axe/contrast/keyboard PASS/FAIL → obs envelope) is the Epoch-10 route chunk
// "A11y CI gate + violation JSON". Run locally with `npm run a11y` after `cargo build --release`.
//
// ONE webview-automation stack: WebdriverIO + @crabnebula/tauri-driver (never a 2nd puppeteer/CDP
// stack — a11y.md §Testing). axe-core is injected via @axe-core/webdriverio into THIS session.
import { spawn, type ChildProcess } from 'node:child_process'
import { join } from 'node:path'

let tauriDriver: ChildProcess | undefined

// The built bundle the driver attaches to (release profile; ensure `cargo build --release` first).
const application = join(
  __dirname,
  '..',
  '..',
  '..',
  'target',
  'release',
  process.platform === 'win32' ? 'conductor-tauri.exe' : 'conductor-tauri',
)

export const config: WebdriverIO.Config = {
  hostname: '127.0.0.1',
  port: 4444,
  specs: ['./test/a11y/**/*.e2e.ts'],
  maxInstances: 1,
  capabilities: [
    // tauri-driver reads the vendor-prefixed `tauri:options` capability — a WebDriver vendor extension
    // (colon-prefixed) outside wdio's typed capability surface, so it is asserted through `unknown`
    // (the documented @crabnebula/tauri-driver shape).
    { browserName: 'wry', 'tauri:options': { application } } as unknown as WebdriverIO.Capabilities,
  ],
  reporters: ['spec'],
  framework: 'mocha',
  mochaOpts: { ui: 'bdd', timeout: 60_000 },

  // Bracket the session with the tauri-driver bridge process (Linux+xvfb).
  onPrepare: () => {
    tauriDriver = spawn('tauri-driver', [], { stdio: [null, process.stdout, process.stderr] })
  },
  onComplete: () => {
    tauriDriver?.kill()
  },
}
