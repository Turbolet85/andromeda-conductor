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
import { spawn, spawnSync, type ChildProcess } from 'node:child_process'
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { census, writeNvdaPass } from './test/a11y/screen-reader/parse-nvda-log'
import type { Subject } from './test/a11y/screen-reader/rows'

let tauriDriver: ChildProcess | undefined
let nvdaExePath: string | undefined
let srSubject: Subject | undefined

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

// CI inverts the skip: a gate that can pass by skipping is banned (a11y-plan §11 Anti-Patterns → CI),
// and this leg's exit code cannot otherwise separate a full pass from a total skip — measured
// 2026-09-02, recorded at .claude/rules/verification-harness.md §Session Additions. The GUARD above is
// untouched: existence + isFile + metacharacter rejection still decide whether a handle RESOLVES, and
// strict mode only changes what an unresolved handle COSTS (security-plan §Anti-Patterns → Input
// forbids relaxing the guard to obtain redness). Truthiness mirrors conductor_core::flag_declared.
function strictMode(): boolean {
  const raw = process.env.CONDUCTOR_A11Y_STRICT
  return raw === 'true' || raw === '1'
}

// Exit rather than throw: wdio has no "skip the whole run" result, and a throw would surface an
// unconfigured dev host as a failed gate. Under strict mode the non-zero IS the intended verdict.
function exitUnresolvedHandle(): never {
  if (strictMode()) {
    console.log('error: CONDUCTOR_A11Y_STRICT is set, so an unresolved handle is a GATE FAILURE, not a skip.')
    process.exit(1)
  }
  process.exit(0)
}

// The a11y violation record is JOURNAL-shaped and lands in its own dir: /runs/ is gitignored, and the
// harness's status/report readers glob "$RUNS_DIR"/*.jsonl non-recursively (scripts/agent-run.sh), so a
// record here can never be mistaken for a scenario run. NOT under logs/ — that path is the Tauri
// backend's tracing sink (obs-plan §3 Log file location), and this is a journal line.
const A11Y_RUNS_DIR = 'runs/a11y'
const VIOLATION_SIDECAR = join(repoRoot, A11Y_RUNS_DIR, '.violations.jsonl')

let a11yRunId: string | undefined
let a11yStartedAt: number | undefined
let a11ySuite = 'routine'

// The routine arm has no live Pulse and /runs/ is gitignored, so on a clean checkout the app launches
// with zero run records and every coverage row reads "Not yet run" — the populated-lamp half of the
// assertion would have no subject, and on a dev host it would silently read whatever local run residue
// happened to be lying about (the artifact-freshness trap verification-harness.md records). So the leg
// seeds its own subject: a COMMITTED fixture journal (reviewable as an artifact, and pinned for its
// semantics by conductor-run's lamps_fixture.rs) copied into a dedicated runs dir.
//
// CONDUCTOR_RUNS_DIR must stay REPO-RELATIVE — resolve_under rejects absolute handles by design — and
// lives under the gitignored /runs/ tree so the seeded copy never rides a commit.
const FIXTURE_RUNS_DIR = 'runs/e2e-fixture'
const FIXTURE_RUN_ID = 'lamps-fixture'

// The fixture dir is re-created CLEAN before the seed: the spawn below used to force this dir on EVERY
// suite, so a driven or SR session that persisted a run journal here left the routine arm reading a run
// that was not the committed journal (a latent flake, found 2026-09-02). Each suite now persists into its
// own runs dir, and the routine subject only ever holds what this seed writes. The clean re-create is
// also what keeps the envelope INSERT below unique-keyed — a surviving runs.db would reject it.
//
// The subject is ONE run carrying BOTH assertion subjects, because run_report and run_envelope each
// default to latest_run_id: a second seeded run would take the latest slot and the coverage rows would
// stop reading the committed journal. So the seeder persists the lamps journal's records UNDER the lamps
// run-id WITH an ENVIRONMENT-SUSPECT envelope standing, giving the banner its first subject without
// moving the lamps one.
//
// It runs through conductor-run's committed seeder rather than copying files: the run_envelope row lives
// in runs.db, which a journal copy cannot create, and test-plan §7 bans a developer-populated database —
// so the row is written by conductor_run::persist, the same production writer both shells call. The
// seeder is a no-op unless CONDUCTOR_E2E_SEED_DIR is set, so an ordinary suite run never writes here.
function seedFixtureRuns(): void {
  const target = join(repoRoot, FIXTURE_RUNS_DIR)
  rmSync(target, { recursive: true, force: true })
  mkdirSync(target, { recursive: true })

  const seed = spawnSync(
    'cargo',
    ['test', '-q', '-p', 'conductor-run', '--test', 'envelope_fixture', '--', '--nocapture'],
    {
      cwd: repoRoot,
      env: { ...process.env, CONDUCTOR_E2E_SEED_DIR: FIXTURE_RUNS_DIR },
      encoding: 'utf8',
      stdio: 'pipe',
    },
  )
  if (seed.status !== 0) {
    // A failed seed must be loud: a silent one leaves the banner spec with no subject, and a
    // subject-absent skip there is exactly the green-over-nothing this fixture exists to remove.
    throw new Error(
      `fixture seed failed (cargo test exit ${seed.status}). The routine arm cannot assert without ` +
        `its subject.\n${seed.stderr ?? ''}`,
    )
  }
  if (!existsSync(join(target, `${FIXTURE_RUN_ID}.jsonl`))) {
    throw new Error(`fixture seed reported success but wrote no ${FIXTURE_RUN_ID}.jsonl into ${FIXTURE_RUNS_DIR}`)
  }
}

const DRIVEN_RUNS_DIR = 'runs/driven/runs'

// The screen-reader leg: three subjects over this same stack, one per suite. `runs` is the app's
// CONDUCTOR_RUNS_DIR for that subject (repo-relative — resolve_under rejects absolute handles);
// `scenarios` overrides the catalog only where the subject needs an empty or malformed one. The live subject
// takes its trimmed catalog from the shell (CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios).
const SR_LEG_DIR = 'runs/sr-leg'
const SR_SUITES: Readonly<Record<string, { subject: Subject; runs: string; scenarios?: string }>> = {
  sr: { subject: 'live', runs: `${SR_LEG_DIR}/runs` },
  'sr-empty': { subject: 'empty', runs: FIXTURE_RUNS_DIR, scenarios: `${SR_LEG_DIR}/empty` },
  'sr-error': { subject: 'error', runs: `${SR_LEG_DIR}/runs`, scenarios: `${SR_LEG_DIR}/bad` },
}

// The suites this run was invoked with. `--suite` is a CLI option the launcher merges onto the runtime config
// object, but wdio's typed Testrunner surface does not declare it — so the field is read reflectively off
// the hook's config, and the launcher's own argv is consulted as well, so an SR suite can never run
// undetected (that would start no NVDA and drive the wrong subject).
function invokedSuites(config: object): Set<string> {
  const names = new Set<string>()
  const merged: unknown = Reflect.get(config, 'suite')
  if (Array.isArray(merged)) for (const s of merged) if (typeof s === 'string') names.add(s)
  const argv = process.argv
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i] ?? ''
    const value = arg === '--suite' ? argv[i + 1] : arg.startsWith('--suite=') ? arg.slice('--suite='.length) : undefined
    if (value) for (const s of value.split(',')) if (s.trim()) names.add(s.trim())
  }
  return names
}

// The portable NVDA is host-owned like the native WebDriver: it arrives by env handle, is validated the same
// way, and is passed as separate argv elements of an array-form spawn — never through a shell.
function nvdaExe(): string | undefined {
  const raw = process.env.CONDUCTOR_NVDA
  if (!raw || UNSAFE_PATH.test(raw)) return undefined
  return existsSync(raw) && statSync(raw).isFile() ? raw : undefined
}

function reportNvdaSkip(): void {
  console.log('error: screen-reader leg skipped — no NVDA resolved')
  console.log('       CONDUCTOR_NVDA is unset, or does not name an existing file.')
  console.log('hint:  setx CONDUCTOR_NVDA "<dir>\\nvda.exe"   (a PORTABLE NVDA copy, no admin needed;')
  console.log('       then reopen the shell). The leg starts it as `nvda -m --no-sr-flag -l 12 -f <log>`')
  console.log('       and quits it with `nvda -q`; log level 12 records every utterance the parser grades.')
}

// NVDA must be up BEFORE the app window exists, so it sees the window and every focus event from the first.
// Readiness is the log's own startup line, never a sleep; a previous session's log is removed first so the
// parser reads only this session's speech.
async function startNvda(exe: string, logPath: string): Promise<void> {
  rmSync(logPath, { force: true })
  // The leg's own NVDA profile (say-all off, silent synth) — copied under the gitignored leg dir, because
  // NVDA writes its profile tree beside the ini it is pointed at.
  const configDir = join(repoRoot, SR_LEG_DIR, 'nvda-config')
  mkdirSync(configDir, { recursive: true })
  copyFileSync(join(here, 'test', 'a11y', 'screen-reader', 'nvda-config', 'nvda.ini'), join(configDir, 'nvda.ini'))
  const child = spawn(exe, ['-m', '--no-sr-flag', '-c', configDir, '-l', '12', '-f', logPath], {
    detached: true,
    stdio: 'ignore',
  })
  child.unref()
  // Readiness is NVDA's own "NVDA initialized" line, not the earlier "Starting NVDA": a window created in
  // the half-second between the two was never bound (measured 2026-09-02 — a whole subject went silent).
  // Then let the log go quiet so the hook threads it starts after that line are up before the app exists.
  const deadline = Date.now() + 30_000
  let ready = false
  while (Date.now() < deadline) {
    if (existsSync(logPath) && readFileSync(logPath, 'utf8').includes('NVDA initialized')) {
      ready = true
      break
    }
    await new Promise((resolve) => setTimeout(resolve, 250))
  }
  if (!ready) throw new Error('NVDA did not report "NVDA initialized" in its speech log within 30s')
  let size = statSync(logPath).size
  let stableSince = Date.now()
  const settleDeadline = Date.now() + 8_000
  while (Date.now() < settleDeadline && Date.now() - stableSince < 1_500) {
    await new Promise((resolve) => setTimeout(resolve, 250))
    const now = statSync(logPath).size
    if (now !== size) {
      size = now
      stableSince = Date.now()
    }
  }
}

async function stopNvda(exe: string): Promise<void> {
  spawnSync(exe, ['-q'], { stdio: 'ignore' })
  const deadline = Date.now() + 15_000
  while (Date.now() < deadline && census().some((line) => line.startsWith('nvda.exe'))) {
    await new Promise((resolve) => setTimeout(resolve, 250))
  }
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

/**
 * The a11y run's obs envelope — the ELEVEN-field run-report envelope verbatim (a11y-plan §3 Structured
 * violation JSON schema), which is what lets conductor-run's `journal_conformance` test BE the gate
 * instead of a second copy of the schema re-listed in CI (ci.yml states that reasoning for the run
 * journal, and a third copy would drift from the structs and obs-plan §3/§6).
 *
 * Five of the eleven are scenario-shaped and carry no a11y measurement — `seed`, `scenario`, `p_ids`,
 * `slo_tier`, `read_back_observed_at`. They are MAPPED, not measured: each is the defensible member of
 * its closed set, and this note exists so a later reader does not read them as findings. Resource
 * attributes ride alongside as extra keys, which the conformance gate admits by design ("extra keys are
 * allowed — obs-plan §3 keeps the extension point open") and obs-plan §9 requires on a CI artifact.
 */
function writeA11yEnvelope(results: { passed?: number; failed?: number } | undefined): void {
  if (!a11yRunId || a11yStartedAt === undefined) return
  const finishedAt = Date.now()
  const fingerprints = existsSync(VIOLATION_SIDECAR)
    ? readFileSync(VIOLATION_SIDECAR, 'utf8')
        .split('\n')
        .filter(Boolean)
        .map((line) => JSON.parse(line) as string)
    : []
  // A contrast, colour-alone or reduced-motion failure IS an a11y violation that axe never emits, so
  // the verdict is the union — any recorded tuple, or any failed spec. Reading axe's array alone would
  // report Pass over a red suite.
  // wdio types this hook's `results` as `any`, so the compiler cannot vouch for the shape — read it
  // defensively rather than let a missing field silently decide the verdict.
  const failedSpecs = typeof results?.failed === 'number' ? results.failed : 0
  const failed = fingerprints.length > 0 || failedSpecs > 0
  const iso = (ms: number) => new Date(ms).toISOString().replace(/\.\d{3}Z$/, 'Z')

  const envelope: Record<string, unknown> = {
    run_id: a11yRunId,
    seed: 0,
    scenario: a11ySuite,
    p_ids: [],
    verdict: failed ? 'Fail' : 'Pass',
    state: failed ? 'Fail' : 'Pass',
    journal_emitted_at: iso(a11yStartedAt),
    read_back_observed_at: iso(finishedAt),
    latency_ms: finishedAt - a11yStartedAt,
    slo_tier: '<90s',
    fingerprints,
    'service.name': 'conductor-ui',
    'deployment.environment': process.env.CONDUCTOR_ENV ?? 'local',
  }
  if (process.env.GITHUB_ACTIONS === 'true') {
    envelope['ci.run.id'] = process.env.GITHUB_RUN_ID ?? ''
    envelope['git.commit.sha'] = process.env.GITHUB_SHA ?? ''
  }

  writeFileSync(
    join(repoRoot, A11Y_RUNS_DIR, `${a11yRunId}.jsonl`),
    `${JSON.stringify(envelope)}\n`,
    'utf8',
  )
  rmSync(VIOLATION_SIDECAR, { force: true })
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
    // The screen-reader leg (a11y-plan §3 Screen reader test pattern): agent-driven over this same stack
    // with a portable NVDA logging its speech — `sr` needs the live Pulse form, the other two no run at all.
    sr: ['./test/a11y/screen-reader.e2e.ts'],
    'sr-empty': ['./test/a11y/screen-reader.e2e.ts'],
    'sr-error': ['./test/a11y/screen-reader.e2e.ts'],
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

  // Bracket the session with the tauri-driver bridge process (and, for the screen-reader leg, NVDA).
  onPrepare: async (config) => {
    const native = nativeDriver()
    if (!native) {
      reportSkip()
      exitUnresolvedHandle()
    }
    // The run's identity and clock are minted here, before any spec runs, so journal_emitted_at is the
    // suite's true start rather than the moment the last spec happened to finish.
    a11yStartedAt = Date.now()
    a11yRunId = `${new Date(a11yStartedAt).toISOString().replace(/[:.]/g, '-').slice(0, 19)}-a11y`
    mkdirSync(join(repoRoot, A11Y_RUNS_DIR), { recursive: true })
    rmSync(VIOLATION_SIDECAR, { force: true })
    seedFixtureRuns()
    // The subject env is chosen by the invoked suite at this ONE spawn site. The routine arm reads the
    // clean fixture; the driven and SR suites persist their run journals into their own dirs, so nothing
    // a session writes can leak into the routine subject.
    const invoked = invokedSuites(config)
    // The envelope's `scenario` names the arm that produced it — the default (no --suite) IS the
    // routine arm, which is the only one this gate runs.
    a11ySuite = [...invoked][0] ?? 'routine'
    const appEnv: NodeJS.ProcessEnv = { ...process.env, CONDUCTOR_RUNS_DIR: FIXTURE_RUNS_DIR }
    if (invoked.has('driven')) {
      mkdirSync(join(repoRoot, DRIVEN_RUNS_DIR), { recursive: true })
      appEnv.CONDUCTOR_RUNS_DIR = DRIVEN_RUNS_DIR
    }
    const sr = [...invoked]
      .map((name) => (Object.hasOwn(SR_SUITES, name) ? SR_SUITES[name] : undefined))
      .find((entry) => entry !== undefined)
    if (sr) {
      const exe = nvdaExe()
      if (!exe) {
        reportNvdaSkip()
        exitUnresolvedHandle()
      }
      const legDir = join(repoRoot, SR_LEG_DIR)
      mkdirSync(legDir, { recursive: true })
      if (sr.runs !== FIXTURE_RUNS_DIR) {
        rmSync(join(repoRoot, sr.runs), { recursive: true, force: true })
        mkdirSync(join(repoRoot, sr.runs), { recursive: true })
      }
      appEnv.CONDUCTOR_RUNS_DIR = sr.runs
      if (sr.scenarios) appEnv.CONDUCTOR_SCENARIOS_DIR = sr.scenarios
      writeFileSync(join(legDir, 'subject.txt'), sr.subject)
      writeFileSync(join(legDir, `census-before.${sr.subject}.txt`), census().join('\n'))
      nvdaExePath = exe
      srSubject = sr.subject
      await startNvda(exe, join(legDir, `nvda-speech.${sr.subject}.log`))
    }
    // cwd = the workspace root: tauri-driver's child (the app under test) inherits it, which is the
    // only lever that points the app's cwd-relative artifact handles at the real catalog. The app
    // inherits this env too, so CONDUCTOR_RUNS_DIR is what points run_report / run_envelope at the
    // subject's runs dir rather than at whatever the host's own runs/ happens to hold.
    tauriDriver = spawn(process.execPath, [driverCli, '--native-driver', native], {
      cwd: repoRoot,
      env: appEnv,
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
  onComplete: async (_exitCode, _config, _capabilities, results) => {
    // NVDA quits first so the app's teardown is not the last thing it logs; then the driver tree, then
    // the pass record is written from this session's log against its action timeline.
    if (nvdaExePath) await stopNvda(nvdaExePath)
    tauriDriver?.kill()
    writeA11yEnvelope(results)
    if (srSubject) {
      const legDir = join(repoRoot, SR_LEG_DIR)
      const beforeFile = join(legDir, `census-before.${srSubject}.txt`)
      writeNvdaPass({
        subject: srSubject,
        repoRoot,
        speechLog: join(legDir, `nvda-speech.${srSubject}.log`),
        actions: join(legDir, `actions.${srSubject}.jsonl`),
        out: join(legDir, 'nvda-pass.json'),
        specMd: join(here, 'test', 'a11y', 'screen-reader', 'nvda-pass-spec.md'),
        censusBefore: existsSync(beforeFile) ? readFileSync(beforeFile, 'utf8').split('\n').filter(Boolean) : [],
        censusAfter: census(),
        liveRunsDir: srSubject === 'live' ? join(repoRoot, SR_LEG_DIR, 'runs') : undefined,
      })
    }
  },
}
