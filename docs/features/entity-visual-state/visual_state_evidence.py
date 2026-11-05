"""Current entity visual readers beside unchanged historical native/replay evidence (#181)."""
from pathlib import Path
from typing import cast
import hashlib
import json

FEATURE = Path(__file__).resolve().parent
SOURCE = FEATURE.parents[3] / 'inventor-interface/solidworks'
VISUAL_CLAUSES = ('11.371', '11.382', '11.383', '11.417', '11.419', '11.432',
    '11.433', '11.446', '11.447', '11.481', '11.562')


def verify_visual_readers() -> None:
    """Keep current readers/tests distinct from old replay revisions; e.g. verify_visual_readers()."""
    pins = cast(dict[str, str], json.loads((FEATURE / 'reader-sources.json').read_bytes()))
    assert len(pins) == 19, f'{len(pins)}; expected eight current readers and eleven unchanged test modules'
    for name, expected in pins.items():
        actual = hashlib.sha256((SOURCE / name).read_bytes()).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected current visual reader/test {expected}'


def verify_visual_clauses() -> None:
    """Retain complete stored-role and runtime-behavior qualifications; e.g. verify_visual_clauses()."""
    text = (SOURCE.parent / 'docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    for clause in VISUAL_CLAUSES:
        start = text.index('### ' + clause + ' ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        assert raw == (FEATURE / f'schema-{clause}.md').read_bytes(), f'{clause}; expected the complete approved visual-property clause'


def visual_reports() -> list[dict[str, object]]:
    """Keep every complete original report byte and source pin; e.g. visual_reports()."""
    pins = cast(dict[str, str], json.loads((FEATURE / 'report-pins.json').read_bytes()))
    assert len(pins) == 15, f'{len(pins)}; expected all fifteen original reports'
    reports: list[dict[str, object]] = []
    for name, expected in pins.items():
        raw = (SOURCE / 'research' / name).read_bytes()
        actual = hashlib.sha256(raw).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected original report {expected}'
        assert raw == (FEATURE / 'reports' / name).read_bytes(), f'{name}; expected byte-identical retained report'
        report = cast(dict[str, object], json.loads(raw))
        assert report['target_executed'] is False, f'{name}; expected read-only original provenance'
        reports.append(report)
    return reports


def verify_visual_native_research(report: dict[str, object]) -> None:
    """Check native research dependencies without adopting historical reader versions; e.g. verify_visual_native_research(report)."""
    sources = cast(dict[str, str], report['sources'])
    assert sources, f'{report["evidence"]}; expected explicit original provenance pins'
    for original, expected in sources.items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        if not relative.startswith('solidworks/research/'):
            continue
        actual = hashlib.sha256((SOURCE.parent / relative).read_bytes()).hexdigest()
        assert actual == expected, f'{relative}/{actual}; expected original native research dependency {expected}'


def verify_visual_reports() -> None:
    """Preserve original scopes and conservative accounting; e.g. verify_visual_reports()."""
    reports = visual_reports()
    replay = [report for report in reports if 'files' in report]
    assert len(replay) == 5 and all(len(cast(list[object], report['files'])) == 906 for report in replay), len(replay)
    rgb = next(report for report in reports if report['evidence'] == 'C-1959')
    assert rgb['records'] == 127773 and rgb['unknown_charge_bytes'] == 894411, rgb['records']
    assert rgb['coverage_credit'] == 0 and rgb['remaining_unknown_bytes'] == 16143727, rgb['coverage_credit']
    for report in reports:
        if 'module' in report or 'modules' in report:
            verify_visual_native_research(report)
