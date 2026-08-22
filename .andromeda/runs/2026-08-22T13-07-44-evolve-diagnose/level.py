"""Stage 4 Pass A — hand-assigned note-theme clusters + Pass B signature hits."""
import json

RUN = '.andromeda/runs/2026-08-22T13-07-44-evolve-diagnose/'
raw = json.load(open(RUN + 'q-level-raw.json', encoding='utf-8'))
facts = raw['epoch_facts']

# hand-assigned themes, by 0-based index into q-level-raw.json epoch_facts
THEMES = {
    'T1-prescribed-shell-write-idiom': {
        'idx': [3, 6, 7, 8, 9, 11, 16, 19, 21, 22, 31, 37],
        'obstacle': ('the shell-write idiom the references prescribe (quoted heredoc / '
                     '.tmp-then-rename / cat > $TMPDIR) cannot carry a Markdown or code '
                     'payload on this host: parse-time unexpected-EOF, collapsed '
                     'backslashes, or an unset TMPDIR resolving to a root path'),
        'routed_to': 'the Write/Edit tool, or a scratchpad file invoked by path',
        'signature': 'band-aid',
    },
    'T2-code-graph-answers-grep-decides': {
        'idx': [4, 20, 25],
        'obstacle': ('the code-graph, named as the first instrument for impact, returned a '
                     'wrong or absent answer (a closure call site, an unindexed symbol, a '
                     'false leaf on an indexed one)'),
        'routed_to': 'grep, with the basis relabelled grep-derived in research.md',
        'signature': 'band-aid',
    },
    'T7-evidence-twin-convention-vs-purpose': {
        'idx': [12, 23, 24],
        'obstacle': ('an evidence-trace convention applied to the letter would have produced '
                     'a misleading or empty artifact (near-duplicate raw twins, a '
                     're-encoded file wearing the name of a raw record, an unstageable '
                     'empty run dir)'),
        'routed_to': 'a substituted single-artifact form authored by hand that run',
        'signature': 'band-aid',
    },
    'T4-named-contract-not-read-schema-probed': {
        'idx': [0, 14],
        'obstacle': ('the skill body names a contract reference as the basis for a read; the '
                     'artifact was opened directly and its schema inferred instead'),
        'routed_to': 'a fallback key chain / a top-level lookup of a nested field',
        'signature': 'below-threshold-in-passA (n=2); see proposal P3 for the 4-record view',
    },
    'T6-plan-file-list-forced-wider': {
        'idx': [18, 34],
        'obstacle': ("the plan's Files-to-modify set omitted companions the change class "
                     'forces mechanically (guard tests data-pinning the retired shape; '
                     'every literal of a struct that gained a field)'),
        'routed_to': 'continuation past the declared set, recorded rather than soft-exited',
        'signature': 'below-threshold-in-passA (n=2); see chain X1',
    },
    'T5-literal-form-check-vs-compliant-content': {
        'idx': [15],
        'obstacle': ('a documented check keyed on a literal form (the trailing per-{plan} § '
                     'anchor) flags content that satisfies it in substance'),
        'routed_to': 'accepted substance over letter rather than respawning the distiller',
        'signature': ('below-threshold-in-passA (n=1); the same mechanism carries the '
                      'untyped cluster U1 (n=3) — counted there, cited here'),
    },
    'T8-host-resource-ceiling': {
        'idx': [1, 10],
        'obstacle': ('a host/tool ceiling made the direct route unusable (tool-result size '
                     'cap on a whole-file read; a ~4-minute gate over the foreground Bash '
                     'ceiling)'),
        'routed_to': 'a structural filter / a backgrounded run redirected to the scratchpad',
        'signature': 'below-threshold (n=2)',
    },
}

UNCLUSTERED = [2, 17, 30, 33]

out = {'pass_a_themes': [], 'unclustered_passA_facts': [], 'pass_b_signatures': []}
for name, t in THEMES.items():
    out['pass_a_themes'].append({
        'theme': name, 'n': len(t['idx']), 'signature': t['signature'],
        'obstacle': t['obstacle'], 'routed_to': t['routed_to'],
        'natures': sorted({facts[i]['nature'] for i in t['idx']}),
        'solutions': sorted({facts[i]['solution'] for i in t['idx']}),
        'steps': sorted({facts[i]['step'] for i in t['idx']}),
        'chunks': sorted({str(facts[i]['chunk']) for i in t['idx']}),
        'facts': [facts[i] for i in t['idx']],
    })
out['unclustered_passA_facts'] = [facts[i] for i in UNCLUSTERED]

out['pass_b_signatures'] = [
    {'signature': 'band-aid', 'theme': 'T1-prescribed-shell-write-idiom',
     'in_epoch_facts': 12, 'prior_epoch_facts': 16,
     'prior_epochs': ['Epoch 1', 'Epoch 2', 'Epoch 3'],
     'hypothesis': ('the cause is a reference-vs-host conflict in the pipeline (and, at '
                    'wrap/curation, a conflict between two DOCUMENTED mechanisms); the fixes '
                    'so far are 28 per-chunk local re-routings')},
    {'signature': 'band-aid', 'theme': 'T2-code-graph-answers-grep-decides',
     'in_epoch_facts': 3, 'prior_epoch_facts': 2, 'prior_epochs': ['Epoch 1'],
     'hypothesis': ('the cause is indexer/query coverage in the code-graph pipeline; the '
                    'fixes are per-chunk grep substitutions plus one plan-level prohibition')},
    {'signature': 'band-aid', 'theme': 'T7-evidence-twin-convention-vs-purpose',
     'in_epoch_facts': 3, 'prior_epoch_facts': 0,
     'hypothesis': ('the cause is a convention stated in artifact-count terms rather than '
                    'evidence-purpose terms; each run re-derives the substitution by hand')},
    {'signature': 'chronic-degrade', 'theme': 'tooling.host-shell typed frictions',
     'counts': {'Epoch 3': 14, 'Epoch 4': 13}, 'halts': 0,
     'hypothesis': ('a two-epoch-stable rate with zero halts: the halt policy structurally '
                    'never surfaces it, and every instance is absorbed inside its own step')},
    {'signature': 'chronic-degrade', 'theme': 'a written learning did not prevent its own recurrence',
     'in_epoch_records': 3,
     'records': ['2026-08-21T17:57:38Z-b', '2026-08-22T09:46:47Z-b',
                 'wrap-session/curation untyped @ 2026-08-20-read-back-seam-survivors-closed'],
     'hypothesis': ('the corrective that was actually applied (a session-learnings entry) '
                    'lives in a corpus that is authored but not consulted at the moment of '
                    'risk; the cause lives in the transport / the shell')},
    {'signature': 'deferred-forever', 'theme': 'unattended-host live-leg and display gate',
     'in_epoch_facts': 4,
     'facts_idx': [26, 27, 28, 36],
     'headless_skip_by_epoch': {'Epoch 1': 3, 'Epoch 3': 1, 'Epoch 4': 2},
     'closure': ('partial: the delegated-timing deferral closed only by an unsanctioned '
                 'wrap-time reorder (halted 1, dialogue_rounds 2); the operator-pause one is '
                 'an open standing Epoch-5 CARRY')},
    {'signature': 'override', 'theme': 'operator overrides',
     'in_epoch_facts': 2, 'facts_idx': [5, 32],
     'below_threshold': True,
     'note': 'two different rules, not the same rule recurring — no miscalibration signal'},
]
json.dump(out, open(RUN + 'q-level.json', 'w', encoding='utf-8'), indent=1)

covered = sum(len(t['idx']) for t in THEMES.values()) + len(UNCLUSTERED)
passa = [f for f in facts if f['solution'] in ('workaround', 'prohibition', 'removed-cause')]
print('pass-A eligible facts:', len(passa), '| assigned+unclustered:', covered)
for t in out['pass_a_themes']:
    print(f"  n={t['n']:2} {t['theme']:42} {t['signature']}")
print('unclustered:', UNCLUSTERED)
