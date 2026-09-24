"""P5 known-positive control for the plan's `cat …` gate: every `contains` atom must hold over HEAD's two sections
(the facts other masters cite INTO them exist today), and at least one must fail over a text lacking them."""
import re
import subprocess

head = subprocess.run(['git', 'show', 'HEAD:.andromeda/architecture.md'], capture_output=True, check=True).stdout.decode('utf-8')


def section(name: str) -> str:
    m = re.search(rf'^## {re.escape(name)}\n.*?(?=^## |\Z)', head, re.S | re.M)
    assert m, name
    return m.group(0)


body = section('Established Decisions') + section('Occupied Resources')
atoms = ['windows-2022', 'CONDUCTOR_A11Y_STRICT', 'journal_conformance', 'runs/a11y', 'operator-local', 'grounded',
         'coverage_gate', 'scenario_audit_gate', 'run --live real-model', 'No inbound listener', 'No `DATABASE_URL`',
         'no secrets/cloud env vars', ':4318']
for a in atoms:
    print(f'{body.count(a):3d}  {a}')
missing = [a for a in atoms if a not in body]
negative = [a for a in atoms if a in section('Design Philosophy')]
print(f'positive: {len(atoms) - len(missing)}/{len(atoms)} present over HEAD sections; missing={missing}')
print(f'negative control (Design Philosophy section): {len(atoms) - len(negative)}/{len(atoms)} atoms fail there')
