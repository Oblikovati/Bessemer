"""Complete C-888 request/helper provenance with inherited version qualifications (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline


def aux_triangle_report() -> dict[str, object]:
    """Preserve every original function, vtable binding and native input; e.g. aux_triangle_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-aux-triangle-fields-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-888' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 160
    assert report['threshold_raw_hex'] == 'ae47e17a14aeef3f'
    assert report['request_counts'] == {'False': 160}
    dialog_baseline('aux-triangle-c888-reference.json', report)
    return report


def verify_aux_triangle_sources() -> None:
    """Require all original pins except the exact later native-color report transition; e.g. verify_aux_triangle_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], aux_triangle_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('97f5b4f1665a07e03abb0384ac4eca88b86a0823a9f8d57e316ef94e775f123b', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    dialog_baseline('aux-triangle-source-transitions.json', dict(transitions=transitions,
        reason='All original160 native control/color inputs and conditional outputs are independently replayed from complete bytes.'))
