"""C858 wire grouping controls remain distinct from older reset events (#166)."""
from collections import Counter
from typing import cast
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_grouping_evidence import legend_grouping_report


def grouping_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Inspect only actual C0 wire spans after a complete body read; e.g. grouping_observation(2, 840008, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields: list[dict[str, object]] = []
    for span in cast(list[dict[str, object]], parsed['spans']):
        if span['label'] != 'legend.word_c0': continue
        start, end = cast(int, span['offset']), cast(int, span['end'])
        stored = raw[start:end]; assert len(stored) == 4 and stored.hex() == span['raw_hex']
        word = int.from_bytes(stored, 'little', signed=True)
        fields.append(dict(name='locale_thousands_grouping_enabled_raw', offset=start, end=end,
            raw_hex=stored.hex(), signed_value=word, consumer_nonzero=word != 0))
    assert len(fields) == int(version >= 840008), (version, fields)
    return fields


def verify_grouping_native() -> None:
    """Keep all276 native identities,160 stored controls and eight original failures; e.g. verify_grouping_native()."""
    rows = accepted_grouping_rows()
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, rows)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): grouping_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['fields'], row
    counts = Counter(str(field['signed_value']) for row in rows for field in cast(list[dict[str, object]], row['fields']))
    assert dict(counts) == {'1': 84, '0': 76}, counts
    dialog_baseline('legend-grouping-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        stored_controls=160, absent_controls=116, value_counts=dict(counts), additional_byte_credit=0,
        locale_characters_inferred=False, full_locale_renderer_emulated=False))


def accepted_grouping_rows() -> list[dict[str, object]]:
    """Keep all eight original unselected identities separate from276 complete accepted contexts; e.g. accepted_grouping_rows()."""
    rows = cast(list[dict[str, object]], legend_grouping_report()['rows'])
    rejected = [row for row in rows if row['selected_version'] is None]
    assert len(rejected) == 8 and all(row['fields'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    accepted = [row for row in rows if row['selected_version'] is not None]
    assert len(accepted) == 276
    return accepted


def verify_grouping_owned() -> None:
    """Preserve optional C0 and arbitrary raw signed patterns across425 complete parents; e.g. verify_grouping_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw),
        fields=grouping_observation(family, version, raw)) for family, version, raw in contexts]
    counts = Counter(str(field['signed_value']) for row in rows for field in cast(list[dict[str, object]], row['fields']))
    assert any(word not in ('0', '1') for word in counts), counts
    dialog_baseline('legend-grouping-owned-reference.json', dict(contexts=425, rows=rows, value_counts=dict(counts),
        absence_defaulted=False, nonzero_restricted_to_one=False, unconditional_separator_insertion=False))
