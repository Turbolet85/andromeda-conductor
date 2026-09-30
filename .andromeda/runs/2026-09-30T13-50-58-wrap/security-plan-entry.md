
## 2026-09-30-the-sr-cause-isolated-on-this-host — two ratified session-level controls outside rule (b)'s loci
**Section:** §Security Anti-Patterns → Code Patterns rule (b), the outside-the-loci record
**Change:** Rule (b) now records two founder-ratified session-level crossings on the dev host, 2026-09-30, that sit outside the three loci and add no governed form:
- arm W — a gitignored session script that validated `CONDUCTOR_MSEDGEDRIVER` the wdio way, started that driver with the fixed argv `--port=4445` on the registered dev-only port, drove ONE WebDriver session to the Edge browser and tore both down;
- arm 153 — the existing `sr-empty` leg with an operator-supplied msedgedriver 153.0.4234.48, admitted by a pre-execution `Get-AuthenticodeSignature` check (`Valid`, an `O=Microsoft Corporation` signer) plus a SHA-256 and version match BEFORE the binary executed at all.
It states the standing rule "a binary is never executed before its signature verdict is read". Both are CONTROLS only; no committed form moved; the count stays seven. The founder's ratification is quoted in the body.
**Why:** Boundary widening, always a human's call: ratified live by the founder at P4 as controls, relayed by the overseer, and its record's placement ruled at this wrap by the overseer (founder-delegated). The pre-execution rule answers a plan defect — the plan's admission entry ran the driver's `--version` in the same command as its signature check, whatever the signature said.
**Kept:** The `4444`/`4445` binder lists (the committed suite families) are unchanged: W was a one-off session crossing, recorded here.
**Ref:** .andromeda/runs/2026-09-30T13-50-58-wrap/
