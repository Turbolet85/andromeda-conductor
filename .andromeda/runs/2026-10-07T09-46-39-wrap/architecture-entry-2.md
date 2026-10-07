
## 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09 — the two Pulse model handles registered
**Section:** §Occupied Resources → Environment variables
**Change:**
- New bullet after `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`: `ANDROMEDA_PULSE_MODEL_PATH` · `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`, Pulse's real-model launch handles, set by Pulse's launching shell. Conductor neither sets nor reads either; the committed `contracts/pulse-real-model-leg-posture.md` names both; neither is a run-contract term or a `conductor preconditions` subject.
- The `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` bullet was "The one registered handle Conductor neither SETS nor READS"; now "A registered handle …" — three registered handles share that class.
- To hold the section at its byte threshold, two redundancies left the body. The bootstrap-window bullet's last sentence left: "Which posture actually booted is read from Pulse's own log at the TARGET `triage.baseline.bootstrap_window.override` (`reason="env_override"`, `resolved_seconds`), never from the emitting function name." The `WEBVIEW2_USER_DATA_FOLDER` · `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`, `TEMP` · `LOCALAPPDATA` and `RUST_LOG` bullets each lost a trailing ", and no `CONDUCTOR_*` namespace claim is made".
- The section measures 38115 B of a 38115 B threshold after this pass: within target, with no headroom.
**Why:** the posture-contract entry says the contract names only already-registered environment handles, and two of its six were absent from the registry; the registration makes that clause true. It rides the playbook's rule for an external handle a shipped artifact names. The dropped log-target sentence is stated in full in test-plan §9, and each dropped clause restated its own bullet's opening "not in the reserved namespace". The next amendment that grows this section must first free bytes.
**Kept:** the section's evidence pointers stay in the body: a mechanism statement keeps its measurement pointer. No handle reader exists in Conductor, so security-plan's handle inventory is unchanged.
**Ref:** .andromeda/runs/2026-10-07T09-46-39-wrap/
