"""C860 original A8 words and derived views retain separate complete-parent provenance (#166)."""
from collections import Counter
from typing import cast
import importlib
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from dialog_field_evidence import dialog_field_reports
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_inverse_evidence import legend_inverse_report


def inverse_observation(family: int, version: int, raw: bytes) -> dict[str, object] | None:
    """Keep absent raw-class A8 unknown and actual canonical words unchanged; e.g. inverse_observation(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = [span for span in cast(list[dict[str, object]], parsed['spans']) if span['label'] == 'legend.word_a8']
    assert len(spans) == int(version >= 700006), (version, spans)
    if not spans: return None
    span = spans[0]; offset, end = cast(int, span['offset']), cast(int, span['end'])
    wire = raw[offset:end]; assert len(wire) == 4 and wire.hex() == span['raw_hex']
    word = int.from_bytes(wire, 'little', signed=True)
    view = importlib.import_module('cosworks_legend_inverse_preference').zero_test(word)
    assert view == int(word == 0)
    return dict(offset=offset, end=end, raw_hex=wire.hex(), stored_a8=word, preference_word_0_view=view)


def verify_inverse_native() -> None:
    """Require276 complete native matches, original raw offsets and all eight failed identities; e.g. verify_inverse_native()."""
    report = legend_inverse_report(); rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, rows)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): inverse_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        expected = {key: row[key] for key in ('offset', 'end', 'raw_hex', 'stored_a8', 'preference_word_0_view')}
        assert available[(family, row['sha256'], row['selected_version'])] == expected, row
    current = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    verify_legend_rejected_identities([row for row in current if not any(attempt['success'] for attempt in row['attempts'])])
    counts = Counter(str(row['stored_a8']) for row in rows)
    assert dict(counts) == report['stored_values']
    dialog_baseline('legend-inverse-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        stored_values=dict(counts), additional_byte_credit=0, wire_words_rewritten=False,
        universal_preference_setter_inferred=False))


def verify_inverse_owned() -> None:
    """Retain425 complete parents, twenty raw-class absences and arbitrary signed A8 patterns; e.g. verify_inverse_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw),
        observation=inverse_observation(family, version, raw)) for family, version, raw in contexts]
    observations = [cast(dict[str, object], row['observation']) for row in rows if row['observation'] is not None]
    assert len(observations) == 405
    counts = Counter(str(row['stored_a8']) for row in observations)
    assert any(word not in ('0', '1') for word in counts), counts
    dialog_baseline('legend-inverse-owned-reference.json', dict(contexts=425, raw_class_absences=20,
        stored_values=dict(counts), rows=rows, absent_words_defaulted=False, original_words_normalized=False))
