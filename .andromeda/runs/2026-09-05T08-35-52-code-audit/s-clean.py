"""Scripted delete of this run's OWN transients — sanctioned only for paths inside {run_dir} (the host's shell-rm policy
denies the shell form). Raw tool output is deleted after summarization; the c-*.json twins + the reproducibility triple stay."""
import os, shutil, sys
RUN = os.path.abspath(sys.argv[1])
assert os.sep + 'runs' + os.sep in RUN and RUN.endswith('-code-audit'), 'refusing: not a code-audit run dir'
targets = sys.argv[2:]
for t in targets:
    p = os.path.abspath(os.path.join(RUN, t))
    assert p.startswith(RUN + os.sep), f'refusing: {p} is outside the run dir'
    if os.path.isdir(p): shutil.rmtree(p); print('removed dir ', t)
    elif os.path.isfile(p): os.remove(p); print('removed file', t)
    else: print('absent      ', t)
