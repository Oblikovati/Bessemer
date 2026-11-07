"""Complete C849 raw word14 observations without font/tick-count inference (#166)."""
from collections import Counter
from typing import cast
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_height_evidence import legend_height_report, legend_height_fields


def legend_height_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Require complete EOF and exact source-byte spans before exposing the count; e.g. legend_height_observation(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields = legend_height_fields(cast(list[dict[str, object]], parsed['spans']))
    for field in fields:
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    return fields


def verify_legend_height_native() -> None:
    """Match276 accepted word14 observations and all eight original rejections; e.g. verify_legend_height_native()."""
    report = legend_height_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_height_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['fields'], row
    rejected = [row for row in rows if row['selected_version'] is None]
    assert all(row['fields'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    counts = legend_height_census([cast(list[dict[str, object]], row['fields']) for row in accepted])
    assert counts == {key: report[key] for key in counts}, counts
    dialog_baseline('legend-height-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        **counts, additional_stored_byte_credit=0, font_metrics_known=False, universal_tick_count_inferred=False))


def legend_height_census(views: list[list[dict[str, object]]]) -> dict[str, object]:
    """Preserve stream multiplicity and every original raw signed value; e.g. legend_height_census(views)."""
    values: Counter[str] = Counter()
    for fields in views:
        for field in fields:
            values[str(field['signed_value'])] += 1
    return dict(field_count=sum(values.values()), value_counts=dict(values))


def verify_legend_height_owned() -> None:
    """Retain425 complete contexts and twenty opaque older count absences; e.g. verify_legend_height_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_height_observation(*context) for context in contexts]
    assert sum(not fields for fields in views) == 20
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), fields=fields)
        for (family, version, raw), fields in zip(contexts, views, strict=True)]
    dialog_baseline('legend-height-owned-reference.json', dict(contexts=425, opaque_legacy=20,
        rows=rows, **legend_height_census(views), absent_counts_defaulted=False, runtime_font_height_inferred=False))
