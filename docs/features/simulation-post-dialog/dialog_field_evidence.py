"""Explicit adoption of complete PostInfo/color views and current stored-role catalog (#166)."""
from typing import cast
import json
from native_carriers import SOURCE, RESEARCH, digest
from dialog_evidence import dialog_baseline, approved_style_adoption

FIELD_REPORTS = ('cosworks-post-info-fields-native-evidence.json', 'cosworks-post-contour-policy-native-evidence.json',
                'cosworks-post-color-fields-native-evidence.json', 'cosworks-post-style-field-catalog.json')
OLD_INFO = 'f3c69f50e1427551325a342125a173cf606966cbd4f59cf9f9507944e46a052b'
CURRENT_INFO = '7566a85a2b1322aec278d39f45c43dcb42bc996461f1cb68b13dfa482d0eba7b'
OLD_COLOR_REPORT = '3efb4ca03307aa6f8dc7205c49e435bf3588f9f8d8ed65b66af39d3b90f3546e'
CURRENT_COLOR_REPORT = '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'


def dialog_field_reports() -> list[dict[str, object]]:
    """Preserve original and current field-view evidence without rewriting ancestors; e.g. dialog_field_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in FIELD_REPORTS]
    for name, report, evidence in zip(FIELD_REPORTS, reports, ('C-841', 'C-843', 'C-844', 'C-859'), strict=True):
        assert report['evidence'] == evidence and not report['target_executed'], report['evidence']
        dialog_baseline(name, report)
    return reports


def approved_field_adoption(relative: str, expected: str, actual: str, report: dict[str, object]) -> bool:
    """Require the two exact semantic-view/provenance transitions and their current canonical pins; e.g. approved_field_adoption(path, old, new, report)."""
    if relative == 'solidworks/research/cosworks_post_style_layout.py': return approved_style_adoption(relative, expected, actual, report)
    if (relative, expected, actual, report['evidence']) == ('solidworks/research/cosworks_post_info_fields.py', OLD_INFO, CURRENT_INFO, 'C-841'):
        canonical = json.loads((RESEARCH / FIELD_REPORTS[1]).read_bytes())
        pins = [value for key, value in canonical['sources'].items() if key.replace('\\', '/').endswith('/cosworks_post_info_fields.py')]
        assert pins == [CURRENT_INFO], pins
        return True
    return (relative, expected, actual, report['evidence']) == ('solidworks/research/' + FIELD_REPORTS[2], OLD_COLOR_REPORT, CURRENT_COLOR_REPORT, 'C-859')


def verify_dialog_field_sources() -> None:
    """Verify all consumed field-model pins and exactly four reviewed ancestor transitions; e.g. verify_dialog_field_sources()."""
    adopted = []
    for report in dialog_field_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/'), original
            actual = digest((SOURCE / relative).read_bytes())
            if actual == expected: continue
            assert approved_field_adoption(relative, expected, actual, report), (relative, expected, actual)
            adopted.append(dict(evidence=report['evidence'], path=relative, original=expected, adopted=actual))
    assert len(adopted) == 4, adopted
    dialog_baseline('field-source-adoption-reference.json', dict(reason='Explicit C-843 field annotations and current C-844 native provenance; original view projection preserved', bindings=adopted))


def verify_dialog_catalog_provenance() -> None:
    """Require every role's exact contributing evidence packet and retain unknowns; e.g. verify_dialog_catalog_provenance()."""
    catalog = dialog_field_reports()[3]
    for entry in cast(list[dict[str, str]], catalog['provenance']):
        raw = (RESEARCH / entry['file']).read_bytes()
        assert digest(raw) == entry['sha256'], entry['file']
        assert json.loads(raw)['evidence'] == entry['evidence'], entry['file']
    assert catalog['catalog_status_counts'] == dict(unresolved=14, bound_consumer_role=103, framing_role=1)
    assert catalog['native_accepted_streams'] == 276, catalog['native_accepted_streams']
