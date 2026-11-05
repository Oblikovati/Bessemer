"""Current node readers and complete historical evidence without regrading unknown bytes (#178)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import sys

FEATURE = Path(__file__).resolve().parent
SOURCE = FEATURE.parents[3] / 'inventor-interface/solidworks'


def node_state_baseline(name: str, value: object) -> None:
    """Require exact evidence outside explicit initialization; e.g. node_state_baseline('native.json', rows)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged node-state evidence'


def node_state_reports() -> list[dict[str, object]]:
    """Preserve all nine original reports with strict hashes and no new credit; e.g. node_state_reports()."""
    pins = json.loads((FEATURE / 'report-pins.json').read_bytes())
    reports: list[dict[str, object]] = []
    for name, expected in pins.items():
        raw = (SOURCE / 'research' / name).read_bytes()
        actual = hashlib.sha256(raw).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected historical report {expected}'
        destination = FEATURE / 'reports' / name
        if '--write' in sys.argv:
            destination.parent.mkdir(exist_ok=True)
            destination.write_bytes(raw)
        assert destination.read_bytes() == raw, f'{destination}; expected complete original report bytes'
        report = cast(dict[str, object], json.loads(raw))
        assert report['target_executed'] is False, name
        reports.append(report)
    assert len(reports) == 9, f'{len(reports)}; expected all nine original reports'
    return reports


def verify_node_state_sources() -> None:
    """Pin current readers separately from historical replay source revisions; e.g. verify_node_state_sources()."""
    for name, expected in json.loads((FEATURE / 'reader-sources.json').read_bytes()).items():
        actual = hashlib.sha256((SOURCE / name).read_bytes()).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected approved current reader {expected}'
    for report in node_state_reports():
        if 'module' not in report:
            continue
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/'), f'{original}; expected an approved source path'
            actual = hashlib.sha256((SOURCE.parent / relative).read_bytes()).hexdigest()
            assert actual == expected, f'{relative}/{actual}; expected original native dependency {expected}'


def verify_node_state_schema() -> None:
    """Retain complete masks, class scopes and effective-predicate caveats; e.g. verify_node_state_schema()."""
    text = (SOURCE.parent / 'docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    for section in ('11.435', '11.448', '11.449', '11.572'):
        start = text.index('### ' + section + ' ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv:
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected the complete unchanged node-state clause'


def verify_node_state_reports() -> None:
    """Keep all historical replay identities and zero credit without adopting old counters; e.g. verify_node_state_reports()."""
    reports = node_state_reports()
    replays = [report for report in reports if 'files' in report]
    assert len(replays) == 4, f'{len(replays)}; expected four original replay reports'
    for report in replays:
        assert len(cast(list[object], report['files'])) == 906, report['evidence']
    global_replay = next(report for report in reports if report['evidence'] == 'C-2192')
    assert global_replay['parameter_counts'] == dict(records=39487, global_variable_set=0), global_replay['evidence']
