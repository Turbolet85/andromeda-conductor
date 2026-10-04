"""C1 summarizer (collectors.md C1 + §Host-excluded mutants, pinned recipe verbatim).
Usage: summarize_c.py {run_dir} {unit} [shard-label]  -> writes c-mutation-{unit}.json only for a COMPLETE invocation."""
import json, os, re, subprocess, sys

RD, UNIT = sys.argv[1], sys.argv[2]
SHARD = sys.argv[3] if len(sys.argv) > 3 else None
REPO = os.getcwd()


# ---------------- pinned recipe (collectors.md §Host-excluded mutants) ----------------
def host_cfg(repo):
    out = subprocess.run(['rustc', '--print', 'cfg'], cwd=repo, capture_output=True, text=True, check=True).stdout
    return {(k, v.strip('"') if v else None) for k, _, v in (l.partition('=') for l in out.splitlines())}

def ev(p, cfg):
    p = p.strip()
    m = re.fullmatch(r'(all|any|not)\s*\((.*)\)', p, re.S)
    if m:
        args, depth, cur = [], 0, ''
        for ch in m.group(2):
            depth += (ch == '(') - (ch == ')')
            if ch == ',' and depth == 0: args.append(cur); cur = ''
            else: cur += ch
        vals = [ev(a, cfg) for a in args + [cur] if a.strip()]
        if m.group(1) == 'not': return None if len(vals) != 1 or vals[0] is None else not vals[0]
        if m.group(1) == 'all': return False if False in vals else None if None in vals else True
        return True if True in vals else None if None in vals else False
    m = re.fullmatch(r'([A-Za-z_]\w*)\s*(?:=\s*"([^"]*)")?', p)
    if not m: return None
    if (m.group(1), m.group(2)) in cfg: return True
    return False if m.group(1) in ('unix', 'windows') or m.group(1).startswith('target_') else None

def mask(src):
    out, i, n = list(src), 0, len(src)
    def blank(a, b):
        for k in range(a, b): out[k] = out[k] if src[k] == '\n' else ' '
    while i < n:
        if src.startswith('//', i): j = src.find('\n', i); j = n if j < 0 else j; blank(i, j); i = j
        elif src.startswith('/*', i):
            d, j = 1, i + 2
            while j < n and d: d += src.startswith('/*', j) - src.startswith('*/', j); j += 2 if src[j:j+2] in ('/*', '*/') else 1
            blank(i, j); i = j
        elif m := re.match(r'b?r(#*)"', src[i:i+300]) if (i == 0 or not (src[i-1].isalnum() or src[i-1] == '_')) else None:
            j = src.find('"' + m.group(1), i + m.end()); j = n if j < 0 else j + 1 + len(m.group(1)); blank(i, j); i = j
        elif src[i] == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == '\\' else 1
            blank(i, j + 1); i = j + 1
        elif src[i] == "'" and (m := re.match(r"'(\\.[^']*|[^\\'])'", src[i:i+12])): blank(i, i + m.end()); i += m.end()
        else: i += 1
    return ''.join(out)

ITEM = {'pub', 'fn', 'impl', 'mod', 'struct', 'enum', 'union', 'trait', 'unsafe', 'async', 'const', 'static', 'type',
        'use', 'extern', 'macro_rules'}

def extents(src):
    mk, res = mask(src), []
    for a in re.finditer(r'#(!?)\[\s*cfg\s*\(', mk):
        d, j = 1, a.end()
        while j < len(mk) and d: d += (mk[j] == '(') - (mk[j] == ')'); j += 1
        pred, j = src[a.end():j-1], mk.find(']', j) + 1
        if a.group(1): res.append((0, len(src), pred)); continue
        k = re.match(r'(\s*#\[[^\]]*\])*\s*(\w+)', mk[j:])
        item = bool(k) and k.group(2) in ITEM
        d = 0
        while j < len(mk):
            c = mk[j]
            if c in '([': d += 1
            elif c in ')]': d -= 1
            elif d == 0 and c == '{':
                b = 1; j += 1
                while j < len(mk) and b: b += (mk[j] == '{') - (mk[j] == '}'); j += 1
                break
            elif d == 0 and (c == ';' or c == ',' and not item): j += 1; break
            elif d == 0 and c == '}': break
            j += 1
        res.append((a.start(), j, pred))
    return res

def off(src, line, col):
    starts = [0] + [k + 1 for k, c in enumerate(src) if c == '\n']
    return starts[line - 1] + col - 1

def file_excluded(repo, rel, cfg, seen=()):
    d, stem = os.path.split(rel)
    name = os.path.basename(d) if stem == 'mod.rs' else stem[:-3]
    if name in ('lib', 'main') or rel in seen: return None
    parent_dir = os.path.dirname(d) if stem == 'mod.rs' else d
    cands = [os.path.join(parent_dir, x) for x in ('lib.rs', 'main.rs', 'mod.rs')] + [parent_dir + '.rs']
    for c in cands:
        try: src = open(os.path.join(repo, c), encoding='utf-8').read()
        except OSError: continue
        m = re.search(r'\bmod\s+' + re.escape(name) + r'\s*;', mask(src))
        if not m: continue
        for s, e, p in extents(src):
            if s <= m.start() and m.end() <= e and ev(p, cfg) is False: return p
        return file_excluded(repo, c.replace('\\', '/'), cfg, seen + (rel,))
    return None

def cover(repo, mutant, cfg):
    try: src = open(os.path.join(repo, mutant['file']), encoding='utf-8').read()
    except OSError: return None
    fp = file_excluded(repo, mutant['file'], cfg)
    if fp: return fp
    sp = mutant['span']
    a = off(src, sp['start']['line'], sp['start']['column'])
    b = off(src, sp['end']['line'], sp['end']['column'])
    for s, e, p in extents(src):
        if s <= a and b <= e and ev(p, cfg) is False: return p
    return None
# ---------------------------------------------------------------------------------------

mo = os.path.join(RD, f'mutants-{UNIT}', 'mutants.out')
oc = json.load(open(os.path.join(mo, 'outcomes.json'), encoding='utf-8'))
planned = json.load(open(os.path.join(mo, 'mutants.json'), encoding='utf-8')) if os.path.isfile(os.path.join(mo, 'mutants.json')) else None
outs = oc['outcomes']
base = outs[0]
if oc.get('end_time') and base.get('summary') != 'Success' and oc.get('total_mutants', 0) == 0:
    json.dump({'unit': UNIT, 'state': 'baseline-test-failure', 'shard': SHARD, 'planned': len(planned) if planned else None,
               'baseline_summary': base.get('summary'), 'end_time': oc.get('end_time')},
              open(os.path.join(RD, f'c-mutation-{UNIT}.json'), 'w'), indent=1)
    sys.exit(f'{UNIT}: baseline-test-failure')
complete = bool(oc.get('end_time')) and (planned is None or oc.get('total_mutants') == len(planned))
if not complete:
    sys.exit(f'{UNIT}: INCOMPLETE (end_time={oc.get("end_time")!r}, total={oc.get("total_mutants")}, planned={len(planned) if planned else None})')


def line_text(fname):  # "file:line:col: description" lines
    p = os.path.join(mo, fname)
    rows = []
    for l in open(p, encoding='utf-8') if os.path.isfile(p) else []:
        m = re.match(r'^(.+?):(\d+):(\d+): (.*)$', l.rstrip('\n'))
        if m: rows.append([f'{m.group(1)}:{m.group(2)}:{m.group(3)}', m.group(4)])
    return rows

missed_rows = line_text('missed.txt')
cfg = host_cfg(REPO)
spans = {}
for o in outs[1:]:
    sc = o.get('scenario')
    mu = sc.get('Mutant') if isinstance(sc, dict) else None
    if mu:
        k = f"{mu['file']}:{mu['span']['start']['line']}:{mu['span']['start']['column']}"
        spans.setdefault(k, []).append(mu)
survivors, not_measured = [], []
for site, text in missed_rows:
    cands = spans.get(site, [])
    pred = None
    if cands:
        preds = {cover(REPO, m, cfg) for m in cands}
        pred = preds.pop() if len(preds) == 1 else None  # ambiguous site -> uncertainty never exempts
    (not_measured.append([site, text, f'cfg({pred})']) if pred else survivors.append([site, text]))
cnt = {'mutants': oc.get('total_mutants'), 'caught': oc.get('caught', 0), 'missed': len(survivors),
       'not_measured': len(not_measured), 'timeout': oc.get('timeout', 0), 'unviable': oc.get('unviable', 0)}
assert len(missed_rows) == oc.get('missed', 0), (len(missed_rows), oc.get('missed'))
assert len({tuple(s) for s in survivors}) == len(survivors), 'duplicate survivor key'
tested_other = cnt['caught'] + cnt['missed'] + cnt['not_measured'] + cnt['timeout']
state = 'unviable-dominant' if cnt['unviable'] > tested_other else 'complete'
score = None if state != 'complete' or (cnt['caught'] + cnt['missed']) == 0 else round(100 * cnt['caught'] / (cnt['caught'] + cnt['missed']), 2)
json.dump({'unit': UNIT, 'state': state, 'shard': SHARD,
           'unit_state': (f"complete {oc.get('total_mutants')}/{len(planned) if planned else '?'}" + (f' of shard {SHARD}' if SHARD else '')) if state == 'complete' else f"unviable-dominant {cnt['unviable']}/{tested_other + cnt['unviable']}",
           'counts': cnt, 'score': score, 'score_formula': 'caught/(caught+missed)',
           'survivors': survivors, 'not_measured': not_measured, 'timeouts': line_text('timeout.txt'),
           'baseline_summary': base.get('summary'), 'start_time': oc.get('start_time'), 'end_time': oc.get('end_time')},
          open(os.path.join(RD, f'c-mutation-{UNIT}.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print(UNIT, state, cnt, score)
