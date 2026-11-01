"""Retain original shell evidence and narrowly bound documented source refinements (#159)."""
from pathlib import Path
from typing import cast
import json
import sys
from native_carriers import SOURCE, RESEARCH, digest
from freeze_body import verify_source_hash

FEATURE = Path(__file__).resolve().parent
REPORT_NAMES = ('cosworks-shell-property-native-evidence.json', 'cosworks-midsurf-property-native-evidence.json',
                'cosworks-base-curves-native-evidence.json', 'cosworks-base-virtual-native-evidence.json',
                'cosworks-layer-descriptor-native-evidence.json')
PARSER = 'solidworks/research/cosworks_shell_property_layout.py'
TESTS = 'solidworks/research/test_cosworks_shell_property.py'
REFINEMENTS = {
    PARSER: ('0b8650a4b40214d0d401b0a7050b73482892a62b5baa3c9bf5b09b2f8e45e31b', '30c112c62865e95c10bf3db3a96ac12ab7536f8710ad1f855f2e8741d316cb0e', 'b451b168178f3bdcd3cefd5fc789dd02ac0c693d7040832757d7ec3ee994e7a0'),
    TESTS: ('44e240fc58055b94de94f2cc996a8eb58c58f01cc45ccb1e01813905b621882a',),
    'solidworks/research/cosworks_mesh_control_layout.py': ('e27879bea4d2bf8a9bcc1245a94ec77554949286f7c5f6a20ac18bf3d0df1cd3',),
    'solidworks/research/cosworks_contact_component_layout.py': ('6c501c14450ad417b1fd5b18d50c59d1611699b3f26dc768f2e8b5b36ca09780',),
    'solidworks/research/cosworks_contact_pair_layout.py': ('126705f601832e46fd804e7277a4be0baf322eb269143dabafb23578d4ccc2ba',),
}
TERMINALS = {
    PARSER: 'ac700fe228e0a35fe431c1d1a988092ba32f6cb391c240b33afd903201be48c8',
    TESTS: '703707b838f938638d66a47f9b96b4932d1cb6dbc37e5e5c17891fde4f15cd9c',
    'solidworks/research/cosworks_mesh_control_layout.py': '7181f48e5d14d24770baf78c13c5454236861c7f4611dfd2735c686a6960d155',
    'solidworks/research/cosworks_contact_component_layout.py': '4718debade4f4e6526c7b949ececc0d97007fbfefd3ee282bd4042b5cdea7064',
    'solidworks/research/cosworks_contact_pair_layout.py': '7d925c0ae6436b34fc6ddefdb0bedc67f2ef12f2ffd1db2e761ca55fd5531a6f',
}


def shell_baseline(name: str, value: object) -> None:
    """Keep reviewed evidence unchanged in normal replay; e.g. shell_baseline('native-reference.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def shell_reports() -> list[dict[str, object]]:
    """Retain all original reports instead of replacing their historical pins; e.g. shell_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in REPORT_NAMES]
    for name, report, evidence in zip(REPORT_NAMES, reports, ('C-744', 'C-745', 'C-755', 'C-756', 'C-757'), strict=True):
        assert report['evidence'] == evidence and not report['target_executed']
        shell_baseline(name, report)
    return reports


def relative_pin(original: str) -> str:
    """Normalize source identity once across original Windows reports; e.g. relative_pin(original)."""
    return original.replace('\\', '/').split('/inventor-interface/', 1)[1]


def require_shell_refinement(relative: str, original: str, current: str) -> None:
    """Accept only specific historical pins ending at the exact C-757 source; e.g. require_shell_refinement(path, old, new)."""
    assert relative in REFINEMENTS, relative
    assert original in REFINEMENTS[relative], (relative, original)
    assert current == TERMINALS[relative], (relative, current)


def verify_shell_sources() -> None:
    """Bind every original pin and the unchanged latest refinement source set; e.g. verify_shell_sources()."""
    reports = shell_reports()
    for report in reports:
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = relative_pin(original)
            current = digest((SOURCE / relative).read_bytes())
            if relative in REFINEMENTS and current != expected:
                require_shell_refinement(relative, expected, current)
                continue
            verify_source_hash(relative, expected)
    for original, expected in cast(dict[str, str], reports[-1]['sources']).items():
        assert digest((SOURCE / relative_pin(original)).read_bytes()) == expected
        if relative_pin(original) in TERMINALS: assert expected == TERMINALS[relative_pin(original)]


def verify_shell_refinement_guards() -> None:
    """Reject alternate paths, old pins and latest hashes independently; e.g. verify_shell_refinement_guards()."""
    for path, originals in REFINEMENTS.items():
        for original in originals:
            valid = (path, original, TERMINALS[path])
            require_shell_refinement(*valid)
            for index in range(3):
                altered = list(valid)
                altered[index] = 'unreviewed'
                try: require_shell_refinement(*altered)
                except AssertionError: continue
                raise AssertionError(f'{altered}; expected rejection outside the exact refinement')


def verify_shell_clauses() -> None:
    """Freeze complete base/derived/refinement clauses, including runtime-presence limits; e.g. verify_shell_clauses()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.365', '14.366', '14.367', '14.375', '14.376', '14.377'):
        start = text.index(f'### {section} ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw
