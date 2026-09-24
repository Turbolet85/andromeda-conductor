# One-shot mutation controls — the two static gates against the REAL subject

Run at /andromeda-implement P2, 2026-09-24, run dir `.andromeda/runs/2026-09-24T12-36-55-implement`. These
controls mutate the workspace, so none is a listed gate entry: the light gate re-runs entries verbatim, and a
mutating control never belongs among them. The listed entries 1 and 2 guard the property on every re-run; their
in-suite negative arms prove the check logic can fail on in-memory input. These controls prove the same thing
end to end, through the real subject enumeration.

## Setup
Two untracked files were written into the workspace by a Python probe. Each sample was assembled at runtime, so
neither the probe's text nor this record holds a contiguous match:
- `planted-secret-control.txt` — line 2 carries a GitHub-classic-token-shaped string (44 chars).
  `git check-ignore -q` exited 1: the file is NOT ignored, so it falls inside the gate's subject
  (`git ls-files --cached --others --exclude-standard`).
- `.github/workflows/zz-planted-control.yml` — line 7 carries `${{ env.PLANTED_UNDECLARED }}` in a `run:` step,
  and nothing declares that key.

## Reading 1 — both gates RED on the mutated tree
`gate.py run --only 1,2` → `entries 23 · green 0 · red 2 (1,2)`; both exited 100.
- Entry 1: `secret_scan_gate the_workspace_holds_no_secret_shaped_string` FAILED, reporting exactly
  `planted-secret-control.txt:2: secret-shaped string (github-token, 44 chars)`. A fixed-string count of the
  sample's 40-char body in that log returned **0**: the matched text is never echoed.
- Entry 2: `workflow_env_gate the_committed_workflows_read_only_declared_env_keys` FAILED, reporting exactly
  `.github/workflows/zz-planted-control.yml:7: env.PLANTED_UNDECLARED is read but no workflow, job or step
  declares it and no step writes it to GITHUB_ENV`.

## Teardown
Both files were removed in the same command. `ls` then reported "No such file or directory" for each, and
`git status --short` lists neither.

## Reading 2 — both gates GREEN again on the restored tree
`gate.py run --only 1,2` → `entries 23 · green 2 · red 0`.
