import re, pathlib, sys
root = pathlib.Path('D:/dev/projects/conductor')
rd = root / '.andromeda/runs/2026-09-30T13-50-58-wrap'
flow = pathlib.Path(sys.argv[1]).read_text(encoding='utf-8')  # argv[1]: the wrap skill's references/amendment-flow.md (host path elided from the record)
tpl = re.search(r'```\n(You are the drift-detector.*?)\n```', flow, re.S).group(1)
base = (root / '.andromeda/drift-base.md').read_text(encoding='utf-8')
entries = re.split(r'(?m)^(?=- id: )', base)
docs = {'architecture': 'arch', 'security-plan': 'security-plan', 'design-system': 'design-system',
        'layout-templates': 'layout-templates', 'test-plan': 'test-plan', 'obs-plan': 'obs-plan', 'a11y-plan': 'a11y-plan'}
report = 'D:/dev/projects/conductor/conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/report.md'
for doc, key in docs.items():
    picked = []
    for e in entries[1:]:
        m = re.search(r'(?m)^  doc: (.*)$', e)
        vals = [v.strip() for v in m.group(1).split('|')]
        if key in vals:
            lines = [l for l in e.rstrip('\n').split('\n') if not l.lstrip().startswith('#')]
            picked.append('\n'.join(lines))
    body = tpl.replace('{contracts_line}\n', '')
    body = body.replace('{doc_path}', f'D:/dev/projects/conductor/.andromeda/{doc}.md')
    body = body.replace('{report_path}', report)
    body = body.replace('{detectors_yaml}', '\n'.join(picked))
    body = body.replace('{doc}', doc)
    left = re.findall(r'\{(?:doc|doc_path|report_path|detectors_yaml|contracts_line)\}', body)
    (rd / f'.prompt-{doc}.txt').write_text(body, encoding='utf-8')
    print(doc, 'detectors', len(picked), 'unsubstituted', left, 'bytes', len(body.encode()))
