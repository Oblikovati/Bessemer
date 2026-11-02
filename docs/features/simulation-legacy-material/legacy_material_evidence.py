"""Strict historical MaterialList source and noncompletion evidence pins (#175)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import sys

FEATURE = Path(__file__).resolve().parent
SOURCE = FEATURE.parents[3] / 'inventor-interface'
RESEARCH = SOURCE / 'solidworks/research'
REPORTS = (
    ('cosworks-legacy-material-list-native-evidence.json', 'C-838', '341028e0e9e46ff6307a348d5efd7afcc3d87da2cc2234ea1884f20f1f349602'),
    ('cosworks-legacy-material-candidate-evidence.json', 'C-894', '9ac9be42081cd9e60848457b2bce47e4adfc52aa0df175b403b9b0a47550e027'),
    ('cosworks-legacy-material-identifier-evidence.json', 'C-895', '77a85b766855f129a6bc0776a49fd265e99019189aeabdfb5258cb8b9854d803'),
)


def legacy_material_reports() -> list[dict[str, object]]:
    """Require all three original reports without adding schema credit; e.g. legacy_material_reports()."""
    reports: list[dict[str, object]] = []
    for name, evidence, expected in REPORTS:
        raw = (RESEARCH / name).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == expected, f'{name}; expected {expected}'
        report = cast(dict[str, object], json.loads(raw))
        assert report['evidence'] == evidence and report['target_executed'] is False, (name, evidence)
        assert report['coverage_credit'] == 0, f'{name}; expected zero schema credit'
        assert len(cast(list[object], report['rows'])) == 70, f'{name}; expected all70 original rows'
        reports.append(report)
    assert reports[0]['complete_streams'] == 0, 'C838; expected all original complete-reader failures'
    assert all(report['complete_schema_records'] == 0 for report in reports[1:]), 'C894/C895; expected noncompletion'
    return reports


def verify_legacy_material_sources() -> None:
    """Reject every changed recorded dependency without hash exceptions; e.g. verify_legacy_material_sources()."""
    for report in legacy_material_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/'), f'{original}; expected SolidWorks source path'
            actual = hashlib.sha256((SOURCE / relative).read_bytes()).hexdigest()
            assert actual == expected, f'{relative}/{actual}; expected {expected}'


def verify_legacy_material_schema() -> None:
    """Retain complete verified-prefix and unbound-candidate clauses; e.g. verify_legacy_material_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.458', '14.512', '14.513'):
        start = text.index('### ' + section + ' ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected the complete unchanged source clause'
