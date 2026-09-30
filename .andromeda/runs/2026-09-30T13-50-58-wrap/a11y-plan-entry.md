
## 2026-09-30-the-sr-cause-isolated-on-this-host — focus silence bound to the injected, driver-launched path
**Section:** §3 Harness Contract → Per-surface test spec (desktop-webview → NVDA), the focus-verdict clause
**Change:** The focus verdict is now CONFIGURATION- and INPUT-PATH-BOUND. Was: on WebView2 154.0.4258.37 / Windows 26200.9457 / NVDA 2026.2 the agent arm hears focus only in the first burst, "so an NVDA user on that configuration hears no focus change in Conductor after the first burst", with "the runtime/driver pair, the Windows cumulative updates and the desktop" as candidates. Now:
- the silence after the first burst is stated for the agent arm alone — WebDriver-injected keys into the app launched under tauri-driver + msedgedriver — at 154/154 and equally at the 153.0.4234.48 / 153.0.4234.48 pair;
- an NVDA user on that 154 configuration pressing Tab through the OS keyboard path hears every focus change (OS-level `SendInput` keys, handled by NVDA as physical, into the bundle launched by path with no driver); Edge 154 is heard under both input paths; a physical keyboard is unmeasured;
- the cause stays recorded, not established: the object model and the runtime/driver pair are ruled out; injected keys versus the driver launch stay confounded; the cumulative updates and the desktop stay candidates against the 2026-09-07 heard run on 152.0.4191.66.
**Why:** Four no-boundary controls measured it: OS-level keys heard in Edge and in Conductor, injected keys heard in Edge, and the injected leg silent on the 153 runtime. The user sentence overstated what the agent arm's path measured. Directed by the operator's wrap relay. Every SR verdict now names its input path beside runtime × driver × NVDA × OS build.
**Kept:** "browse-mode rows stay pending OS-level key injection" — OS keys reaching NVDA's hook makes those rows possibly agent-reachable, a route item, not a disproof.
**Ref:** .andromeda/runs/2026-09-30T13-50-58-wrap/
