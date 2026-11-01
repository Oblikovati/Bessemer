"""C-885 raw initialization and local preference transfers without inferred UI meanings (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline

RAW_STYLE_INITIAL = {0x1f0: 0, 0x204: 0, 0x208: 0, 0x230: 1, 0x234: 0, 0x238: 0,
                     0x23c: 0, 0x240: 1, 0x28c: 200, 0x290: 50, 0x294: 0, 0x298: 1}
RAW_STYLE_TRANSFERS = {0x240: (0x152c, 4), 0x230: (0x1574, 0x4c),
                       0x204: (0x15b4, 0x8c), 0x208: (0x15bc, 0x94)}


def style_preferences_report() -> dict[str, object]:
    """Retain all original assignments, transfers and native observations; e.g. style_preferences_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-style-raw-preferences-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-885' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 276
    assert len(cast(list[object], report['functions'])) == 4
    dialog_baseline('raw-preferences-c885-reference.json', report)
    return report


def verify_style_preferences_sources() -> None:
    """Require all original pins apart from the exact later color-report transition; e.g. verify_style_preferences_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], style_preferences_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('b283e2f8a63a5276fcb77d646ae98358b48a682c7d6b740f91a6de7206381880', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1, transitions
    dialog_baseline('raw-preferences-source-transitions.json', dict(transitions=transitions,
        reason='Every original identity, stored raw-word occurrence and initializer comparison is independently replayed from complete bytes.'))
