// Holds the committed claim→owner enumeration to what the suites actually assert.
//
// The point is the anti-vacuity check: a spec whose TITLE names a claim and whose body asserts nothing
// must not stand as that claim's owner. accessibility.e2e.ts carried exactly that shape — four claims in
// a title, a bare this.skip() for a body — which is why naming an owner is not enough on its own.
//
// Run: npm run a11y:ownership   (tsx; no browser, no driver, no Pulse)

import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

import {
  CI_SUITES,
  CLAIMS,
  NA_CLAIMS,
  SUITE_INVOCATION,
  SUITE_SPEC,
  type Claim,
} from './claim-ownership'

const HERE = dirname(fileURLToPath(import.meta.url))

const failures: string[] = []

function fail(claim: string, why: string): void {
  failures.push(`${claim}: ${why}`)
}

const sources = new Map<string, string>()
function source(spec: string): string {
  const cached = sources.get(spec)
  if (cached !== undefined) return cached
  const text = readFileSync(join(HERE, spec), 'utf8')
  sources.set(spec, text)
  return text
}

/**
 * The spec body between an `it('<title>'` and the next `it(` — or end of file for the last spec.
 * Returns null when no spec carries that title.
 */
function specBody(spec: string, title: string): string | null {
  const text = source(spec)
  const at = text.indexOf(`it('${title}'`)
  if (at < 0) return null
  const after = text.indexOf("it('", at + 4)
  return after < 0 ? text.slice(at) : text.slice(at, after)
}

for (const entry of CLAIMS as Claim[]) {
  switch (entry.state) {
    case 'owned': {
      const spec = SUITE_SPEC[entry.owner]
      const body = specBody(spec, entry.assertedBy)
      if (body === null) {
        fail(entry.claim, `owner ${entry.owner} (${spec}) carries no spec titled "${entry.assertedBy}"`)
        break
      }
      // The claim is owned only if its spec ASSERTS. A title is not coverage.
      if (!body.includes('expect(')) {
        fail(
          entry.claim,
          `owner ${entry.owner} (${spec}) has the spec but it asserts nothing — 0 expect( in its body; a title is not coverage`,
        )
      }
      break
    }
    case 'n/a-by-construction':
      if (entry.basis.trim() === '') fail(entry.claim, 'n/a-by-construction with no stated basis')
      break
    case 'unasserted':
      if (entry.gap.trim() === '') fail(entry.claim, 'unasserted with no stated gap reason')
      break
  }
}

// Every claim named once, so a duplicate cannot mask a missing one.
const seen = new Set<string>()
for (const entry of CLAIMS) {
  if (seen.has(entry.claim)) failures.push(`${entry.claim}: named more than once`)
  seen.add(entry.claim)
}

// The no-target set is asserted as a SET, not a containment: a new construction claim must be declared here
// deliberately rather than appearing because it happened to be written that way.
const declaredNa = CLAIMS.filter((c) => c.state === 'n/a-by-construction')
  .map((c) => c.claim)
  .sort()
const expectedNa = [...NA_CLAIMS].sort()
if (declaredNa.join(' | ') !== expectedNa.join(' | ')) {
  failures.push(
    `n/a-by-construction set drift: enumeration has [${declaredNa.join(' | ')}], NA_CLAIMS declares [${expectedNa.join(' | ')}]`,
  )
}

const ownedRows = CLAIMS.filter((c) => c.state === 'owned')
const na = declaredNa.length
const gaps = CLAIMS.filter((c) => c.state === 'unasserted')

// The carve-out population: claims whose owner CI does not run. Printing it is the point — these are the
// claims the requirement must say what gates in place of, and a silent list is how that obligation is lost.
const carveOut = ownedRows.filter((c) => c.state === 'owned' && !CI_SUITES.includes(c.owner))
for (const c of carveOut) {
  if (c.state !== 'owned') continue
  console.log(`carve-out: ${c.claim} — owner ${c.owner} (${SUITE_INVOCATION[c.owner]}), not run by CI`)
}

for (const gap of gaps) console.log(`gap (recorded, not a pass): ${gap.claim}`)

if (failures.length > 0) {
  for (const f of failures) console.error(`FAIL ${f}`)
  console.error(`ownership: ${failures.length} claim(s) unresolved`)
  process.exit(1)
}

console.log(
  `${CLAIMS.length} claims · ${ownedRows.length} owned (${carveOut.length} operator-local, carve-out) · ${na} n/a-by-construction · ${gaps.length} recorded gaps`,
)
console.log('ownership: every claim resolved')
