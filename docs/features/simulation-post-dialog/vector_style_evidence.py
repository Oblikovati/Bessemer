"""Complete C-886 setter-chain evidence without expanding its local side-effect scope (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline


def vector_style_report() -> dict[str, object]:
    """Retain all original native words, colors, function bodies and import bindings; e.g. vector_style_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-vector-style-fields-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-886' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 224
    assert len(cast(list[object], report['functions'])) == 7
    assert report['object_bases'] == dict(dialog='0x160', style_relative_to_dialog='0x240')
    dialog_baseline('vector-style-c886-reference.json', report)
    return report


def verify_vector_style_sources() -> None:
    """Require every original pin except the exact later color-report transition; e.g. verify_vector_style_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], vector_style_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('b96cf68b3341ea4bfa28d077b96e7e6c849fcb9dcf07d86d8e988c392ea0e58d', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1, transitions
    dialog_baseline('vector-style-source-transitions.json', dict(transitions=transitions,
        reason='All224 original identities, complete stored word/color sets and exact wire offsets are independently replayed.'))
