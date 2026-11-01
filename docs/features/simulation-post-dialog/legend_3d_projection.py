"""Complete C852 count/precision observations without independent renderer claims (#166)."""
from collections import Counter
from typing import cast
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_3d_evidence import legend_3d_report, legend_3d_fields


def legend_3d_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Require complete EOF and physical scalar bytes before exposing observations; e.g. legend_3d_observation(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields = legend_3d_fields(cast(list[dict[str, object]], parsed['spans']))
    for field in fields:
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    return fields


def verify_legend_3d_native() -> None:
    """Match all276 accepted contexts,552 existing word observations and eight original failures; e.g. verify_legend_3d_native()."""
    report = legend_3d_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_3d_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['fields'], row
    rejected = [row for row in rows if row['selected_version'] is None]
    assert all(row['fields'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    counts = legend_3d_census([cast(list[dict[str, object]], row['fields']) for row in accepted])
    assert counts['field_counts'] == report['field_counts'], counts
    dialog_baseline('legend-3d-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        **counts, additional_stored_byte_credit=0, native_rendering_executed=False))


def legend_3d_census(views: list[list[dict[str, object]]]) -> dict[str, object]:
    """Preserve label multiplicity and each signed word's raw value distribution; e.g. legend_3d_census(views)."""
    counts: Counter[str] = Counter()
    values: dict[str, Counter[str]] = {}
    for fields in views:
        for field in fields:
            label = cast(str, field['label'])
            counts[label] += 1
            values.setdefault(label, Counter())[str(field['signed_value'])] += 1
    return dict(field_counts=dict(counts), value_counts={label: dict(words) for label, words in values.items()})


def verify_legend_3d_owned() -> None:
    """Keep425 complete parents and actual opaque/absent scalar cases; e.g. verify_legend_3d_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_3d_observation(*context) for context in contexts]
    assert sum(not fields for fields in views) == 20
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), fields=fields)
        for (family, version, raw), fields in zip(contexts, views, strict=True)]
    dialog_baseline('legend-3d-owned-reference.json', dict(contexts=425, opaque_older_contexts=20,
        rows=rows, **legend_3d_census(views), missing_values_defaulted=False, renderer_validity_established=False))
