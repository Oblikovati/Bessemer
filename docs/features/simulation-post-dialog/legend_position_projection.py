"""Exact C847 field offsets/raw words and supplementary finite views from complete bodies (#166)."""
from collections import Counter
from typing import cast
import importlib
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_position_evidence import legend_position_report


def legend_position_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Require complete EOF and byte-consistent spans before exposing factors; e.g. legend_position_observation(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    helper = importlib.import_module('cosworks_legend_position_fields')
    fields = cast(list[dict[str, object]], helper.decode_fields(parsed['spans']))
    for field in fields:
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    return fields


def verify_legend_position_native() -> None:
    """Match all276 accepted identities,552 original factor annotations and eight rejections; e.g. verify_legend_position_native()."""
    report = legend_position_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_position_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['fields'], row
    rejected = [row for row in rows if row['selected_version'] is None]
    assert all(row['fields'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    counts = legend_position_census([cast(list[dict[str, object]], row['fields']) for row in accepted])
    assert counts == {key: report[key] for key in counts}, counts
    dialog_baseline('legend-position-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        **counts, additional_stored_byte_credit=0, runtime_dimensions_known=False, native_pixel_results_emulated=False))


def legend_position_census(views: list[list[dict[str, object]]]) -> dict[str, object]:
    """Preserve stream multiplicity and distinguish every nearby raw float encoding; e.g. legend_position_census(views)."""
    counts: Counter[str] = Counter()
    values: dict[str, Counter[str]] = {}
    for fields in views:
        for field in fields:
            name = cast(str, field['name'])
            counts[name] += 1
            values.setdefault(name, Counter())[cast(str, field['raw_hex'])] += 1
    return dict(field_counts=dict(counts), raw_value_counts={name: dict(words) for name, words in values.items()})


def verify_legend_position_owned() -> None:
    """Keep425 complete contexts, twenty opaque styles and all actual wire spans; e.g. verify_legend_position_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_position_observation(*context) for context in contexts]
    assert sum(not fields for fields in views) == 20
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), fields=fields)
        for (family, version, raw), fields in zip(contexts, views, strict=True)]
    dialog_baseline('legend-position-owned-reference.json', dict(contexts=425, opaque_legacy=20,
        rows=rows, **legend_position_census(views), missing_fields_defaulted=False, native_pixel_results_emulated=False))
