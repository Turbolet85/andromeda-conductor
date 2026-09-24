"""P5 review re-derivation: how much of each HEAD registry section's backtick-span set arm (d) would count as
conserved by the HEAD sidecar ALONE (i.e. a body passage could leave and (d) still pass)."""
import re
import subprocess


def show(path: str) -> str:
    return subprocess.run(['git', 'show', f'HEAD:{path}'], capture_output=True, check=True).stdout.decode('utf-8')


arch = show('.andromeda/architecture.md')
side = show('.andromeda/architecture-amendments.md')
side_spans = set(re.findall(r'`([^`\n]+)`', side))
print('control: conductor-core in sidecar (substring):', 'conductor-core' in side)


def section(name: str) -> str:
    m = re.search(rf'^## {re.escape(name)}\n.*?(?=^## |\Z)', arch, re.S | re.M)
    assert m, name
    return m.group(0)


for name in ['Established Decisions', 'Occupied Resources']:
    s = section(name)
    spans = set(re.findall(r'`([^`\n]+)`', s))
    as_sub = sum(1 for x in spans if x in side)
    as_span = sum(1 for x in spans if x in side_spans)
    lines = [l for l in s.split('\n')[1:] if l.strip()]
    tickless = [l for l in lines if '`' not in l]
    sentences = re.split(r'(?<=[.;:])\s+', s)
    sent_all_side = [x for x in sentences if re.findall(r'`([^`\n]+)`', x) and all(t in side for t in re.findall(r'`([^`\n]+)`', x))]
    sent_no_tick = [x for x in sentences if x.strip() and '`' not in x]
    print(f'== {name}: distinct spans {len(spans)} · in sidecar as substring {as_sub} ({as_sub / len(spans):.0%}) · as a backtick span {as_span}')
    print(f'   non-blank lines {len(lines)} · lines with no backtick {len(tickless)}')
    print(f'   sentence split {len(sentences)} · sentences whose every span is sidecar-present {len(sent_all_side)} · sentences with no span {len(sent_no_tick)}')
