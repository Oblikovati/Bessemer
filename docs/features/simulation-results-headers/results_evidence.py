"""Original Results root/category/plot source and hierarchy bindings (#164)."""
from pathlib import Path
from typing import cast
import json
import re
import sys
from native_carriers import SOURCE, RESEARCH, digest

FEATURE = Path(__file__).resolve().parent
REPORTS = ('cosworks-results-header-native-evidence.json', 'cosworks-results-category-native-evidence.json',
           'cosworks-results-plot-header-native-evidence.json')
READERS = ('cosworks_results_header', 'cosworks_results_category', 'cosworks_results_plot_header')


def results_baseline(name: str, value: object) -> None:
    """Initialize only explicitly and require full evidence equality afterward; e.g. results_baseline('native.json', rows)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged Results header evidence'


def results_reports() -> list[dict[str, object]]:
    """Preserve every original header report and failed alternative; e.g. results_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in REPORTS]
    for name, report, identity in zip(REPORTS, reports, ('C-762', 'C-763', 'C-764'), strict=True):
        assert report['evidence'] == identity and not report['target_executed'], report['evidence']
        results_baseline(name, report)
    return reports


def results_level(stream: str) -> int:
    """Assign only complete hierarchy shapes, never a basename; e.g. results_level('Cosworks/StudyManager/StudyNumber-0/Results/Property')."""
    match = re.fullmatch(r'[^/]+/StudyManager/[^/]+/Results/((?:[^/]+/){0,2})Property', stream)
    assert match is not None, f'{stream}; expected Results root/category/plot Property shape'
    return match[1].count('/')


def verify_results_sources() -> None:
    """Require every original dependency pin without source-hash exceptions; e.g. verify_results_sources()."""
    for report in results_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/'), f'{original}; expected an approved SolidWorks source path'
            actual = digest((SOURCE / relative).read_bytes())
            assert actual == expected, f'{relative}/{actual}; expected original source hash {expected}'


def verify_results_schema() -> None:
    """Retain all reader/writer caveats and hierarchy clauses verbatim; e.g. verify_results_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.382', '14.383', '14.384'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete Results hierarchy clause'


def verify_results_level_guards() -> None:
    """Reject unrelated and deeper Property shapes before parser selection; e.g. verify_results_level_guards()."""
    prefix = 'Cosworks/StudyManager/StudyNumber-0/Results/'
    for level, suffix in enumerate(('Property', 'Category/Property', 'Category/Plot/Property')):
        assert results_level(prefix + suffix) == level
    assert results_level(prefix + 'Results/Property') == 1
    assert results_level(prefix + 'Results/Results/Property') == 2
    assert results_level('Cosworks/StudyManager/SimStudy-7/Results/Property') == 0
    for stream in ('Property', 'Cosworks/StudyManager/StudyNumber-0/Mesh/Property', prefix + 'A/B/C/Property', prefix + '/Property'):
        try: results_level(stream)
        except AssertionError: continue
        raise AssertionError(f'{stream}; expected rejection outside the three complete hierarchy shapes')
