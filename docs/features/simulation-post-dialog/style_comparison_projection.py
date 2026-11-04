"""C863 diagnostic signed views do not replace arbitrary raw control words or loaded state (#166)."""
from collections import Counter
from typing import cast
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from style_comparison_evidence import style_comparison_report, STYLE_COMPARISON_WORDS, STYLE_COMPARISON_NATIVE_COUNTS


def style_comparison_word_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Retain exact present spans without filling constructor or migration values; e.g. style_comparison_word_observation(2, 840008, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields: list[dict[str, object]] = []
    for span in cast(list[dict[str, object]], parsed['spans']):
        if span['label'] not in STYLE_COMPARISON_WORDS: continue
        start, end = cast(int, span['offset']), cast(int, span['end'])
        wire = raw[start:end]; assert len(wire) == 4 and wire.hex() == span['raw_hex']
        fields.append(dict(label=span['label'], offset=start, end=end, raw_hex=wire.hex(),
            diagnostic_signed_view=int.from_bytes(wire, 'little', signed=True)))
    return fields


def accepted_style_comparison_rows() -> list[dict[str, object]]:
    """Keep all eight original failed identities and their empty observations outside accepted replay; e.g. accepted_style_comparison_rows()."""
    rows = cast(list[dict[str, object]], style_comparison_report()['rows'])
    rejected = [row for row in rows if row['selected_version'] is None]
    assert len(rejected) == 8 and all(row['fields'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    accepted = [row for row in rows if row['selected_version'] is not None]
    assert len(accepted) == 276
    return accepted


def verify_style_comparison_native() -> None:
    """Match276 accepted identities,all2136 original words and eight rejected candidates; e.g. verify_style_comparison_native()."""
    rows = accepted_style_comparison_rows()
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, rows)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): style_comparison_word_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['fields'], row
    counts = style_comparison_counts([field for row in rows for field in cast(list[dict[str, object]], row['fields'])])
    assert counts == STYLE_COMPARISON_NATIVE_COUNTS and sum(sum(values.values()) for values in counts.values()) == 2136
    dialog_baseline('style-comparison-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        original_words=2136, original_bytes=8544, word_value_counts=counts, additional_byte_credit=0,
        wire_words_as_runtime_state=False, inactive_payloads_discarded=False))


def style_comparison_counts(fields: list[dict[str, object]]) -> dict[str, dict[str, int]]:
    """Count signed diagnostic views while retaining raw spans as authoritative; e.g. style_comparison_counts(fields)."""
    return {label: dict(Counter(str(field['diagnostic_signed_view']) for field in fields if field['label'] == label))
        for label in STYLE_COMPARISON_WORDS}


def verify_style_comparison_owned() -> None:
    """Retain425 complete parents,twenty raw-class absences and unrestricted wire controls; e.g. verify_style_comparison_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw),
        fields=style_comparison_word_observation(family, version, raw)) for family, version, raw in contexts]
    assert sum(not row['fields'] for row in rows) == 20
    fields = [field for row in rows for field in cast(list[dict[str, object]], row['fields'])]
    counts = style_comparison_counts(fields)
    assert all(any(word not in ('0', '1') for word in values) for values in counts.values()), counts
    dialog_baseline('style-comparison-owned-reference.json', dict(contexts=425, raw_class_absences=20, rows=rows,
        word_value_counts=counts, missing_fields_defaulted=False, runtime_migrations_applied=False))
