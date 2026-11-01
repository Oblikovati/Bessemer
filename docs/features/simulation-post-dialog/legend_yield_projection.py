"""Complete raw D8 color observations without forced gray/alpha or runtime-state inference (#166)."""
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
from legend_yield_evidence import legend_yield_report


def legend_yield_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Require complete EOF and exact physical bytes before exposing all four channels; e.g. legend_yield_observation(2, 810019, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    helper = importlib.import_module('cosworks_post_color_fields')
    colors = [dict(name='gray_above_yield_base_color', **color) for color in helper.colors_from_spans(parsed['spans'])
        if color['label'] == 'legend.color_d8.component']
    for color in colors:
        assert raw[cast(int, color['offset']):cast(int, color['end'])].hex() == color['raw_hex']
    return colors


def verify_legend_yield_native() -> None:
    """Match276 accepted identities,212 original colors and all eight rejected streams; e.g. verify_legend_yield_native()."""
    report = legend_yield_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_yield_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['colors'], row
    rejected = [row for row in rows if row['selected_version'] is None]
    assert all(row['colors'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    counts = legend_yield_census([cast(list[dict[str, object]], row['colors']) for row in accepted])
    assert counts == {key: report[key] for key in counts}, counts
    dialog_baseline('legend-yield-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        **counts, additional_stored_byte_credit=0, runtime_split_known=False, actual_gray_path_observed=False))


def legend_yield_census(views: list[list[dict[str, object]]]) -> dict[str, object]:
    """Keep nearby raw encodings, white counterexamples and original stream multiplicity; e.g. legend_yield_census(views)."""
    values: Counter[str] = Counter()
    for colors in views:
        for color in colors:
            values[cast(str, color['raw_hex'])] += 1
    return dict(color_count=sum(values.values()), raw_value_counts=dict(values))


def verify_legend_yield_owned() -> None:
    """Retain425 complete contexts and actual version-conditioned color absence; e.g. verify_legend_yield_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_yield_observation(*context) for context in contexts]
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), colors=colors)
        for (family, version, raw), colors in zip(contexts, views, strict=True)]
    dialog_baseline('legend-yield-owned-reference.json', dict(contexts=425,
        absent_color_contexts=sum(not colors for colors in views), rows=rows, **legend_yield_census(views),
        absent_colors_defaulted=False, equal_channels_required=False, alpha_convention_inferred=False))
