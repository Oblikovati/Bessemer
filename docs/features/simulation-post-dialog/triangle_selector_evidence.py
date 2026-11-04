"""C-889 original selector/color evidence replay; helper implementation remains separate (#166)."""
from collections import Counter
from typing import cast
import json
import struct
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_projection import actual_post_record
from dialog_owned import dialog_owned_cases
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_identity, raw_style_native_contexts


def triangle_selector_report() -> dict[str, object]:
    """Retain every original caller/helper and native identity; e.g. triangle_selector_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-triangle-color-selectors-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-889' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 276
    methods = cast(list[dict[str, object]], report['functions'])
    assert [(method['start'], method['end']) for method in methods] == [
        ('0x180100e60', '0x180100fcc'), ('0x1801578e0', '0x180157afe'), ('0x180100fd0', '0x180101175')]
    dialog_baseline('triangle-selector-c889-reference.json', report)
    return report


def verify_triangle_selector_sources() -> None:
    """Require original pins with one exact independently replayed report transition; e.g. verify_triangle_selector_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], triangle_selector_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('0bde69f7e89ffa46838afddb2b31eafb615f7e5a5a1f53b8a4bf16542e67f63d', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1
    dialog_baseline('triangle-selector-source-transitions.json', dict(transitions=transitions,
        reason='Every original276 identity, selector, selector offset, RGBA and conditional view is independently replayed from complete bytes.'))


def triangle_selector_observation(family: int, version: int, raw: bytes) -> dict[str, dict[str, object]]:
    """Derive original raw words and full RGBA after exact EOF; e.g. triangle_selector_observation(0, 890620, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = {span['label']: span for span in cast(list[dict[str, object]], parsed['spans'])}
    observed: dict[str, dict[str, object]] = {}
    for word, color in (('1f0', '1f4'), ('230', '220')):
        selector = spans.get('style.word_' + word)
        if selector is None: continue
        wire = bytes.fromhex(cast(str, selector['raw_hex']))
        assert len(wire) == 4
        offset = cast(int, selector['offset'])
        assert raw[offset:offset + 4] == wire
        unsigned = struct.unpack('<I', wire)[0]
        rgba = ''.join(cast(str, spans[f'style.color_{color}.component_{index}']['raw_hex']) for index in range(4))
        assert len(bytes.fromhex(rgba)) == 16
        observed[word] = dict(raw_hex=wire.hex(), offset=selector['offset'], rgba_raw_hex=rgba,
            exact_one_view=unsigned == 1, hidden_selector_domain_view=unsigned < 2 if word == '1f0' else None)
    return observed


def verify_triangle_selector_native() -> None:
    """Independently replay every original native record without inferring invocation; e.g. verify_triangle_selector_native()."""
    report = triangle_selector_report()
    rows = cast(list[dict[str, object]], report['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2
        for row in cast(list[dict[str, object]], local_render_report()['rows'])}
    assert Counter(raw_style_identity(row) for row in rows) == Counter(identities.keys())
    available = {(family, digest(raw), version): triangle_selector_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        assert row['scope'] == 'stored_input_views_only'
        key = identities[raw_style_identity(row)], row['sha256'], row['selected_version']
        assert available[key] == row['values'], row
    counts = Counter(f'{word}_{struct.unpack("<I", bytes.fromhex(cast(str, value["raw_hex"])))[0]}'
        for row in rows for word, value in cast(dict[str, dict[str, object]], row['values']).items())
    assert dict(counts) == report['selector_counts'] == {'1f0_0': 276, '230_1': 221, '230_0': 55}
    dialog_baseline('triangle-selector-native-reference.json', dict(native_contexts=276,
        selector_counts=dict(counts), complete_eof_required=True, additional_stored_byte_credit=0))


def verify_triangle_selector_owned() -> None:
    """Record actual selector presence over every owned version without initialization guesses; e.g. verify_triangle_selector_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    counts: Counter[str] = Counter()
    versions: dict[str, list[int]] = {'1f0': [], '230': []}
    for family, version, raw in contexts:
        observed = triangle_selector_observation(family, version, raw)
        for word in versions:
            counts[word + ('_present' if word in observed else '_missing')] += 1
            if word in observed: versions[word].append(version)
    dialog_baseline('triangle-selector-owned-reference.json', dict(owned_contexts=425,
        presence=dict(counts), observed_versions={word: sorted(set(values)) for word, values in versions.items()},
        complete_eof_required=True, missing_values_defaulted=False, additional_stored_byte_credit=0))
