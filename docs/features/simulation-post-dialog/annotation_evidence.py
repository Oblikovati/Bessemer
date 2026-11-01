"""C-887 exact annotation provenance and current semantic roles (#166)."""
import importlib
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline


def annotation_report() -> dict[str, object]:
    """Retain original functions, literal references and every native control pair; e.g. annotation_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-extrema-annotation-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-887' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 276
    assert len(cast(list[object], report['functions'])) == 7
    dialog_baseline('annotation-c887-reference.json', report)
    return report


def verify_annotation_sources() -> None:
    """Require every source/decompilation pin or the one exact historical report transition; e.g. verify_annotation_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], annotation_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('12a07b5b61debc27b8f104d843e9c0a6d13a80fb2fc256362b01a3af4ce15164', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    dialog_baseline('annotation-source-transitions.json', dict(transitions=transitions,
        reason='All original identities, offsets, raw control pairs and enabled views are independently replayed.'))


def verify_annotation_catalog() -> None:
    """Require exact Min/Max identities from the copied current Source catalog; e.g. verify_annotation_catalog()."""
    catalog = importlib.import_module('cosworks_post_style_field_catalog')
    for member, purpose in (('234', 'minimum_annotation_enabled_raw'), ('238', 'maximum_annotation_enabled_raw')):
        assert catalog.describe('style.word_' + member)['purpose'] == purpose
