"""Size discipline: drop raw tool output, keep the capped c-*.json twins + record + proposals.

Deletes are sanctioned ONLY inside THIS run's run dir (audit-pass.md §Size discipline). Every
path is asserted to resolve under RUN before removal; anything else is refused, not skipped.
"""
import os, shutil, sys

RUN = os.path.dirname(os.path.abspath(__file__))

RAW_DIRS = ['_rca_head', 'jscpd_rs',
            'mutants-conductor-tauri', 'mutants-conductor-cli', 'mutants-conductor-run',
            'mutants-conductor-verify', 'mutants-conductor-core']
RAW_FILES = ['_tokei.json', '_zeroref.json', '_fanin.json', '_fanout.json', '_churn.txt',
             '_lcov.info', '_cov.log', '_knip.json', '_knip.err', '_mut_progress.txt',
             '_mut_tauri.start',
             '_mut_tauri.log', '_mut_cli.log', '_mut_run.log', '_mut_verify.log', '_mut_core.log',
             'c-_perfile_cx.json', 'c-_commits_per_file.json']


def guard(p):
    full = os.path.realpath(p)
    root = os.path.realpath(RUN)
    if not (full == root or full.startswith(root + os.sep)):
        raise SystemExit(f'REFUSED — outside the run dir: {full}')
    return full


freed = 0
for d in RAW_DIRS:
    p = guard(os.path.join(RUN, d))
    if not os.path.isdir(p):
        continue
    for dp, _, ns in os.walk(p):
        for n in ns:
            try:
                freed += os.path.getsize(os.path.join(dp, n))
            except OSError:
                pass
    shutil.rmtree(p)
    print(f'removed dir  {d}')
for f in RAW_FILES:
    p = guard(os.path.join(RUN, f))
    if not os.path.isfile(p):
        continue
    freed += os.path.getsize(p)
    os.remove(p)
    print(f'removed file {f}')
print(f'\nfreed {freed / 1e6:.1f} MB')
kept = sorted(os.listdir(RUN))
print(f'kept {len(kept)} entries: ' + ' · '.join(kept))
