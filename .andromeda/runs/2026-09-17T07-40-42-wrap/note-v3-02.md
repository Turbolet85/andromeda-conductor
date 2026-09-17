2026-09-17 (2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm, wrap P7.3): un-claimed, and the EXCLUSION ARM IS NOW DEAD ON MEASUREMENT.

The acceptance stands unchanged and was deliberately NOT refined downward — it is correct as written, and the world is one small fix from meeting it.

Arm 1 (green printed verdict) is not met, but for the first time it is REACHABLE and nearly reached. Run 35192876641 — hosted `windows-2022`, native WebView2 runtime 131.0.2903.86, msedgedriver pinned to 131.0.2903.86, HIGH integrity, no launcher — created the first hosted-runner WebView2 session in this project's history (`DevToolsActivePort` in 1 s, banner `[webview2 131.0.2903.86 windows]`) and ran the routine arm at 11 passing / 1 failing / 2 skipped, the two skips being the expected live-hold set. SC 2.4.3 (`:397`) PASSES on runtime 131.

Arm 2 (ratified permanent exclusion) is now FORECLOSED. The endpoint demonstrably opens on a hosted runner, so a permanent exclusion would have been ratified on a false basis. Recorded here so no future chunk re-opens it: it had a measured basis for roughly one day (the driver-alone isolation at Medium, runs 35145664132 / 35147042449 / 35150449243) and that basis was superseded twice — first by the transport comparison, then by the coherent-pair run. Holding the terminal open was vindicated on both occasions.

The single red is `:384` (SC 2.1.1) and it is a COUNTING-BASIS defect in the assertion, not an accessibility defect: expected and received bracket lists are identical to the character — same six controls, same order, same one wrap of the focus cycle — and only the prefix differs, expected "6 reached" against received "12 reached". The expectation counts DISTINCT controls; the label counts VISITS. Record it as "12 visits / 6 distinct", never as a bare count. The reachability property SC 2.1.1 asserts is satisfied, and the proof is inside the failure message.

Not established: why the focus cycle wraps on the runner and did not on the dev host at runtime 152 (the 2026-09-10 green). The assertion is environment-sensitive and the first successful CI run of this arm is what exposed it.

Owner: the successor entry minted at this wrap's P5, `conductor-0.3.0/working-route.md:37`, which carries the working configuration, the failure's own evidence, and the four unresolved operator escalations.
