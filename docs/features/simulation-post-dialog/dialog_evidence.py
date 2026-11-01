"""Complete original dialog checkpoints and explicit legacy-view source adoption (#166)."""
from pathlib import Path
from typing import cast
import json
import sys
from native_carriers import SOURCE, RESEARCH, digest

FEATURE = Path(__file__).resolve().parent
DIALOG_REPORTS = ('cosworks-post-dialog-prefix-native-evidence.json', 'cosworks-post-dialog-info-native-evidence.json',
    'cosworks-post-dialog-style-native-evidence.json', 'cosworks-post-dialog-planes-native-evidence.json',
    'cosworks-post-dialog-native-evidence.json', 'cosworks-legacy-post-blocks-native-evidence.json')
OLD_STYLE = 'a13ebac8138b855f5740aaf330c9325af93958f15e6bb8be7bd25e473626ad3f'
CURRENT_STYLE = '3bbb7feb95bc490dc8d3c80365a1cae41b3cdccb8013b48ce5f8158a91982145'


def dialog_baseline(name: str, value: object) -> None:
    """Require complete unchanged evidence outside explicit initialization; e.g. dialog_baseline('native.json', rows)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged dialog evidence'


def dialog_reports() -> list[dict[str, object]]:
    """Preserve every complete original prefix/full/legacy report; e.g. dialog_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in DIALOG_REPORTS]
    for name, report, evidence in zip(DIALOG_REPORTS, reports, ('C-766', 'C-767', 'C-768', 'C-769', 'C-770', 'C-840'), strict=True):
        assert report['evidence'] == evidence and not report['target_executed'], report['evidence']
        dialog_baseline(name, report)
    return reports


def approved_style_adoption(relative: str, expected: str, actual: str, report: dict[str, object]) -> bool:
    """Allow only C-867's exact legacy-view extension already pinned by complete C-770; e.g. approved_style_adoption(path, old, new, report)."""
    if (relative, expected, actual) != ('solidworks/research/cosworks_post_style_layout.py', OLD_STYLE, CURRENT_STYLE): return False
    assert report['evidence'] in ('C-768', 'C-769', 'C-840', 'C-841', 'C-843'), report['evidence']
    for name in ('cosworks-post-dialog-native-evidence.json', 'cosworks-post-v7-composed-view-evidence.json'):
        canonical = json.loads((RESEARCH / name).read_bytes())
        matches = [value for key, value in canonical['sources'].items() if key.replace('\\', '/').endswith('/cosworks_post_style_layout.py')]
        assert matches == [CURRENT_STYLE], (name, matches)
    return True


def verify_dialog_sources() -> None:
    """Require every original dependency pin or the single fully bound style adoption; e.g. verify_dialog_sources()."""
    adopted = []
    for report in dialog_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/'), f'{original}; expected approved SolidWorks source path'
            actual = digest((SOURCE / relative).read_bytes())
            if actual == expected: continue
            assert approved_style_adoption(relative, expected, actual, report), (relative, expected, actual)
            adopted.append(dict(evidence=report['evidence'], path=relative, original=expected, adopted=actual))
    assert len(adopted) == 3, adopted
    dialog_baseline('source-adoption-reference.json', dict(reason='C-867 integrated legacy view; complete original native outcomes replay unchanged', bindings=adopted))


def verify_dialog_schema() -> None:
    """Retain every complete initial framing and standalone carrier clause; e.g. verify_dialog_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.386', '14.387', '14.388', '14.389', '14.390', '14.460', '14.461', '14.462', '14.463', '14.479', '14.487'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete dialog/standalone/field-view clause'
