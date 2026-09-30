# Driver / runtime coherence — re-read before the webview legs

Plan entry 6, fired 2026-09-30 through `gate.py run --only 6` (run dir `.andromeda/runs/2026-09-30T09-58-22-implement`),
right after the PREREQ pair and before any file edit, `--e2e` or `sr*` leg.

Printed line, verbatim:

    driver 154.0.4258.37 runtime 154.0.4258.37

- driver: `CONDUCTOR_MSEDGEDRIVER --version` (the handle's value is not recorded here).
- runtime: the machine-wide WebView2 EdgeUpdate client key's `pv`, read in the colon-free reg.exe form.
- Exit 0 — the equality `test "$d" = "$r"` held, and the driver read was non-empty.

The pair matches the take-up measurement (154.0.4258.37 on both sides), so both webview legs may fire.
