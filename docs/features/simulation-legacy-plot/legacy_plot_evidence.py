"""Keep C839/C908 source, schema and complete historical layout pins strict (#174)."""
from pathlib import Path
from typing import cast
import json
import re
import sys
from native_carriers import RESEARCH, SOURCE, digest

FEATURE = Path(__file__).resolve().parent
REPORTS = ('cosworks-legacy-plot-record-native-evidence.json', 'cosworks-legacy-plot-wide-evidence.json')


def legacy_plot_baseline(name: str, value: object) -> None:
    """Preserve exact evidence outside explicit initialization; e.g. legacy_plot_baseline('native.json', rows)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged historical plot evidence'


def legacy_plot_reports() -> tuple[dict[str, object], dict[str, object]]:
    """Load only the two previously frozen complete reports; e.g. legacy_plot_reports()."""
    foundation = json.loads((FEATURE / 'foundation-verification.json').read_bytes())
    pins = {row['file']: row['sha256'] for row in foundation['upstream_report_inventory']}
    reports: list[dict[str, object]] = []
    for name, evidence in zip(REPORTS, ('C-839', 'C-908'), strict=True):
        raw = (RESEARCH / name).read_bytes()
        assert digest(raw) == pins[name], f'{name}; expected {pins[name]}'
        report = cast(dict[str, object], json.loads(raw))
        assert report['evidence'] == evidence and report['target_executed'] is False
        assert report['coverage_credit'] == 0, report['coverage_credit']
        reports.append(report)
    return reports[0], reports[1]


def verify_legacy_plot_sources() -> None:
    """Reject every changed dependency without source-pin exceptions; e.g. verify_legacy_plot_sources()."""
    for report in legacy_plot_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/'), f'{original}; expected SolidWorks source path'
            actual = digest((SOURCE / relative).read_bytes())
            assert actual == expected, f'{relative}/{actual}; expected {expected}'


def verify_legacy_plot_schema() -> None:
    """Keep both complete source clauses, allowing unrelated schema growth; e.g. verify_legacy_plot_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    manifest = json.loads((FEATURE / 'schema-snapshot.json').read_bytes())
    for clause in manifest['clauses']:
        remaining = text[text.index('### ' + clause['clause'] + ' '):]
        boundaries = [position for marker in ('\n### ', '\n## ') if (position := remaining.find(marker, 1)) >= 0]
        raw = (remaining[:min(boundaries)].rstrip() + '\n').encode()
        assert len(raw) == clause['bytes'] and digest(raw) == clause['sha256'], clause
        assert (FEATURE / clause['file']).read_bytes() == raw, clause['file']


def verify_legacy_fixed_sequence() -> None:
    """Derive every ordered read from the independently hashed decompilation; e.g. verify_legacy_fixed_sequence()."""
    report, _ = legacy_plot_reports()
    source = RESEARCH / 'decomp/cosworks-additional/FUN_181187ea0.c'
    text = source.read_bytes().decode().replace('\r\n', '\n')
    derived: list[dict[str, int]] = []
    pattern = r'\(\*pcVar2\)\(param_2,param_1 \+ (0x[0-9a-f]+|\d+),(0x[0-9a-f]+|\d+)(?:,0)?\);'
    for match in re.finditer(pattern, text):
        line = text[:match.start()].count('\n') + 1
        if 443 <= line <= 581: derived.append(dict(line=line, member=int(match[1], 0), bytes=int(match[2], 0)))
    fixed = json.loads((RESEARCH / 'cosworks-legacy-plot-fixed-reads.json').read_bytes())
    assert derived == fixed['exact700003'] and len(derived) == 46
    assert sum(field['bytes'] for field in derived) == 1344, derived
    functions = cast(list[dict[str, str]], report['functions'])
    expected = next(row['decompilation_sha256'] for row in functions if row['address'] == '0x181187ea0')
    assert digest(source.read_bytes()) == expected, f'{source}; expected {expected}'
