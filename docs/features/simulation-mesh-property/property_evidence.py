"""Complete original Mesh Property source and schema bindings (#163)."""
from pathlib import Path
from typing import cast
import json
import sys
from native_carriers import SOURCE, RESEARCH, digest
from freeze_body import verify_source_hash

FEATURE = Path(__file__).resolve().parent
REPORT_NAMES = ('cosworks-mesh-storage-static-evidence.json', 'cosworks-mesh-native-evidence.json',
                'cosworks-mesh-nested-native-evidence.json', 'cosworks-mesh-final-native-evidence.json')
READER = 'solidworks/research/cosworks_mesh_layout.py'
ADOPTED = '7cae57ea0eabb96c9105ca4229cc22f973fefcf0fef2f62865f0e7f7e633f8eb'
HISTORICAL = ('a83791daafb6af1f22e37b38833e2bf64259604fde3c59e59333b98713f69f94',
              '55632d983a2603b409d593319bdd6a16c670030dede00378a6326b3bad5afb0e')


def property_baseline(name: str, value: object) -> None:
    """Initialize only explicitly, then require complete unchanged evidence; e.g. property_baseline('native.json', rows)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged Mesh Property evidence'


def property_reports() -> list[dict[str, object]]:
    """Retain static, initial, nested and final evidence without replacing older reports; e.g. property_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in REPORT_NAMES]
    for name, report, identity in zip(REPORT_NAMES, reports, ('C-758', 'C-759', 'C-760', 'C-761'), strict=True):
        assert report['evidence'] == identity and not report['target_executed'], report['evidence']
        property_baseline(name, report)
    return reports


def property_relative_pin(original: str) -> str:
    """Normalize both historical relative and absolute source pins once; e.g. property_relative_pin('solidworks/research/file.py')."""
    relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
    assert relative.startswith('solidworks/'), f'{original}; expected an inventor-interface solidworks source pin'
    return relative


def require_property_refinement(relative: str, original: str, current: str) -> None:
    """Adopt only the two documented parent-reader refinements; e.g. require_property_refinement(READER, HISTORICAL[0], ADOPTED)."""
    assert relative == READER, f'{relative}; expected {READER}'
    assert original in HISTORICAL, f'{original}; expected a documented C-759/C-760 reader hash'
    assert current == ADOPTED, f'{current}; expected C-761 reader hash {ADOPTED}'


def verify_property_sources() -> None:
    """Verify every historical dependency and the complete adopted source set; e.g. verify_property_sources()."""
    for report in property_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = property_relative_pin(original)
            current = digest((SOURCE / relative).read_bytes())
            if relative == READER and current != expected:
                require_property_refinement(relative, expected, current)
                continue
            verify_source_hash(relative, expected)
    latest = cast(dict[str, str], property_reports()[-1]['sources'])
    reader_pins = [expected for original, expected in latest.items() if property_relative_pin(original) == READER]
    assert reader_pins == [ADOPTED], f'{reader_pins}; expected exactly the adopted complete reader pin'


def verify_property_refinement_guards() -> None:
    """Reject an alternate path, historical hash or adopted hash independently; e.g. verify_property_refinement_guards()."""
    for original in HISTORICAL:
        valid = (READER, original, ADOPTED)
        require_property_refinement(*valid)
        for index in range(3):
            altered = list(valid)
            altered[index] = 'unreviewed'
            try: require_property_refinement(*altered)
            except AssertionError: continue
            raise AssertionError(f'{altered}; expected rejection outside the exact documented refinement')


def verify_property_schema() -> None:
    """Freeze complete clauses, including superseded map wording and its correction; e.g. verify_property_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.378', '14.379', '14.380', '14.381'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected the complete documented Mesh Property clause'
