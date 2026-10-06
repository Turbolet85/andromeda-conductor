Answers to the P4 fork round (AskUserQuestion), 2026-10-06, verbatim.

Question 1 — "Where should the series' fresh data dir live on this Linux host? …"
Answer: "Home-rooted (Recommended)"
Notes: overseer (founder-delegated). Measured 2026-10-06: redact.rs:130 matches /home/ and mask_host_paths covers /home/; neither carries a temp-rooted form. Fix the temp-rooted gap in this chunk after the drives, as you propose.

Question 2 — "The Pulse binaries on disk predate 5f77859 … Does your grant cover the two release builds in the Pulse tree, and who launches pulse-app?"
Answer: "Agent builds and launches (Recommended)"
Notes: overseer (founder-delegated). The grant covers both release builds in the Pulse tree and the launch; pulse-builder stays idle, Pulse is at 5f77859 with only .andromeda bookkeeping dirty. Correction to my directive, from the founder tonight: ONLY GPU runs are barred at night (only the GPU heats his water loop). Cargo builds, tests and pre-push may run at any hour, so defer nothing but the model drives.
