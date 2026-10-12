
## 2026-10-12-panel-shaped-types-retired — `conductor scenarios`, the suite stop, the stored caption and the hold's checklist lines
**Section:** §Surface: cli → Primary screens (commands) · Output structure — `conductor suite` · Component — Hero (hold, CLI mirror) · Component — Primary content block 1
**Change:**
- `conductor scenarios` is a new verb. It lists the catalogue through `list_scenarios` and the capability manifest as a 3-column table (Scenario, P-IDs, SLO) with no bracket prefix, the one table sized to the terminal width. It ends in a `1 scenario` / `N scenarios` roll-up counted from the catalogue. An empty catalogue prints `No scenarios found.` (exit 0); an absent manifest is a harness fault, exit 1. It mints no token, label or ANSI entry.
- `conductor suite`'s operator stop: after preflight, the first Ctrl-C prints the stop-requested line and the suite stops before its next scenario. The spinner is abandoned in place, the caption and table print over the kept records, `suite stopped: {kept} of {total} scenarios kept` prints on stderr, and the exit is 130. A second Ctrl-C exits 130 at once; a completed suite exits on the hard-Fail rule.
- `conductor report` prints a suspect run's stored `[ENVIRONMENT-SUSPECT]` caption once before the six-column table, with no flag.
- The interactive hold prints each checklist item as two indented plain lines (`  induced:` / `  observe:`) between the HOLD line and the confirm; the headless path prints none.
- Primary content block 1 names three table shapes: results (6 columns), coverage (4) and the catalogue (3) (was "The two must not be conflated"). The results and coverage tables are sized to their content (was "width detected dynamically from the terminal"; the suite mockup's "widths from terminal" now reads "widths from content").
**Why:** the four functions that had no command-line entry once the window left get one at this chunk. The width wording was an older mismatch, brought to the code on the operator's own word at this wrap.
**Ref:** .andromeda/runs/2026-10-12T07-35-57-wrap/
