"""Original MeshControl evidence and an explicit first-public-behavior refinement (#161)."""
from pathlib import Path
from typing import cast
import json
import sys
from native_carriers import SOURCE, RESEARCH, digest
from freeze_body import verify_source_hash
from shell_property_evidence import verify_shell_sources

FEATURE = Path(__file__).resolve().parent
REPORT_NAMES = ('cosworks-mesh-control-header-native-evidence.json', 'cosworks-mesh-control-native-evidence.json')
RECORD_READER = 'solidworks/research/cosworks_mesh_control_layout.py'
ORIGINAL_RECORD_HASH = 'c3a4b38d836ce236c5591e91d7277f2596218549372b12196db917139a01e89e'
REFINED_RECORD_HASH = '7181f48e5d14d24770baf78c13c5454236861c7f4611dfd2735c686a6960d155'


def mesh_baseline(name: str, value: object) -> None:
    """Require an explicit first freeze, then complete unchanged evidence; e.g. mesh_baseline('native-reference.json', {})."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged evidence'


def mesh_reports() -> list[dict[str, object]]:
    """Retain both original denominators and all failed alternatives; e.g. mesh_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in REPORT_NAMES]
    for name, report, expected in zip(REPORT_NAMES, reports, ('C-748', 'C-749'), strict=True):
        assert report['evidence'] == expected and not report['target_executed'], report['evidence']
        mesh_baseline(name, report)
    return reports


def require_mesh_refinement(relative: str, original: str, current: str) -> None:
    """Accept only the recorded original-to-C-757 Mesh reader pair; e.g. require_mesh_refinement(path, old, current)."""
    assert relative == RECORD_READER, f'{relative}; expected {RECORD_READER}'
    assert original == ORIGINAL_RECORD_HASH, f'{original}; expected original C-749 hash {ORIGINAL_RECORD_HASH}'
    assert current == REFINED_RECORD_HASH, f'{current}; expected adopted C-757 hash {REFINED_RECORD_HASH}'


def verify_mesh_source_bindings() -> None:
    """Keep historical pins intact and bind the adopted refinement to its complete reviewed report; e.g. verify_mesh_source_bindings()."""
    for report in mesh_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
            current = digest((SOURCE / relative).read_bytes())
            if relative == RECORD_READER and current != expected:
                require_mesh_refinement(relative, expected, current)
                continue
            verify_source_hash(relative, expected)
    verify_shell_sources()
    mesh_baseline('source-refinement-reference.json', dict(file=RECORD_READER, original_c749_sha256=ORIGINAL_RECORD_HASH,
                  adopted_c757_sha256=REFINED_RECORD_HASH, behavior_profile=1,
                  scope='First public MeshControl behavior adopts documented base virtual/curve refinements; every original native outcome remains required.'))


def verify_mesh_refinement_rejections() -> None:
    """Reject substitutions along every refinement boundary; e.g. verify_mesh_refinement_rejections()."""
    original = (RECORD_READER, ORIGINAL_RECORD_HASH, REFINED_RECORD_HASH)
    require_mesh_refinement(*original)
    for index in range(3):
        altered = list(original)
        altered[index] = 'unreviewed'
        try: require_mesh_refinement(*altered)
        except AssertionError: continue
        raise AssertionError(f'{altered}; expected rejection outside the exact adopted refinement')


def verify_mesh_schema_clauses() -> None:
    """Retain complete header/record clauses rather than excerpted threshold tables; e.g. verify_mesh_schema_clauses()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.369', '14.370'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete schema clause'
