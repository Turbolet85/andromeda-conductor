
## 2026-09-30-the-sr-cause-isolated-on-this-host — host-tool handles: the only COMMITTED reader
**Section:** §Occupied Resources → Environment variables — the `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` rows
**Change:** Was "Read ONLY by `crates/conductor-tauri/ui/wdio.conf.ts`" and "`wdio.conf.ts`, the ONLY reader"; now "Its only COMMITTED reader is `crates/conductor-tauri/ui/wdio.conf.ts` (never a shipped binary)" and "the only COMMITTED reader". Every other clause of both rows stands: validation at the harness edge, array-form spawn, skip at exit 0 when unset.
**Why:** Gitignored, uncommitted session scripts also read both handles (guarded the wdio way) for the 2026-09-30 SR controls, so "only reader" was literally false. Narrowed by the overseer's ruling so the claim stays true going forward with no dated clutter. The arch registry carries STANDING committed readers and binders: a one-off session script is recorded in security rule (b) and chunk evidence, never here.
**Kept:** No arch row for `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` and no Ports or trust-boundary edit for the one-off `:4445` binder — rejected by the overseer (founder-delegated); a successor that COMMITS a reader or binder registers it then.
**Ref:** .andromeda/runs/2026-09-30T13-50-58-wrap/
