"""P2 apply, second half — verify S1 landed once; AR4 + AR5 on architecture.md."""
import pathlib
A = pathlib.Path('D:/dev/projects/conductor/.andromeda')
sec = (A / 'security-plan.md').read_text(encoding='utf-8')
print('security S1 occurrences', sec.count('Two founder-ratified session-level crossings on the dev host'),
      'count-stays-seven', sec.count('the count stays seven'))

def edit(name, old, new):
    p = A / name
    s = p.read_bytes().decode('utf-8')
    assert s.count(old) == 1, (name, s.count(old))
    p.write_bytes(s.replace(old, new).encode('utf-8'))
    back = p.read_bytes().decode('utf-8')
    assert back.count(new) == 1, name
    print(name, 'ok', len(s), '->', len(back))

edit('architecture.md',
     'Read ONLY by `crates/conductor-tauri/ui/wdio.conf.ts` (never by a shipped binary)',
     'Its only COMMITTED reader is `crates/conductor-tauri/ui/wdio.conf.ts` (never a shipped binary)')
edit('architecture.md',
     '`crates/conductor-tauri/ui/wdio.conf.ts`, the ONLY reader — never a shipped binary)',
     '`crates/conductor-tauri/ui/wdio.conf.ts`, the only COMMITTED reader — never a shipped binary)')
