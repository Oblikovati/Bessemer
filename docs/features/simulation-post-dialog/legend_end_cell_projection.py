"""Complete C853 stored flag/color observations, preserving all four color words (#166)."""
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
from legend_end_cell_evidence import legend_end_cell_report, end_cell_flag_fields


def end_cell_observation(family: int, version: int, raw: bytes) -> dict[str, object]:
    """Require complete EOF and exact version-gated E8/EC physical spans; e.g. end_cell_observation(2, 890103, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields = end_cell_flag_fields(cast(list[dict[str, object]], parsed['spans']))
    helper = importlib.import_module('cosworks_post_color_fields')
    colors = [dict(name='end_cell_extension_color', **color) for color in helper.colors_from_spans(parsed['spans'])
        if color['label'] == 'legend.color_ec.component']
    assert len(fields) == len(colors) <= 1 and (not fields or version >= 890103), (version, fields, colors)
    for view in fields + colors:
        assert raw[cast(int, view['offset']):cast(int, view['end'])].hex() == view['raw_hex']
    return dict(fields=fields, colors=colors)


def verify_legend_end_cell_native() -> None:
    """Match276 accepted contexts,109 inactive pairs and all eight rejected streams; e.g. verify_legend_end_cell_native()."""
    report = legend_end_cell_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): end_cell_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == {key: row[key] for key in ('fields', 'colors')}, row
    rejected = [row for row in rows if row['selected_version'] is None]
    assert all(row['fields'] == [] and row['colors'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    counts = end_cell_census([{key: row[key] for key in ('fields', 'colors')} for row in accepted])
    assert counts == {key: report[key] for key in counts}, counts
    dialog_baseline('legend-end-cell-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        absent_pair_contexts=167, **counts, additional_stored_byte_credit=0, actual_extension_observed=False))


def end_cell_census(views: list[dict[str, object]]) -> dict[str, object]:
    """Preserve raw flags, exact RGBA encodings and original multiplicity; e.g. end_cell_census(views)."""
    flags: Counter[str] = Counter()
    colors: Counter[str] = Counter()
    for view in views:
        for field in cast(list[dict[str, object]], view['fields']):
            flags[str(field['signed_value'])] += 1
        for color in cast(list[dict[str, object]], view['colors']):
            colors[cast(str, color['raw_hex'])] += 1
    return dict(flag_count=sum(flags.values()), flag_values=dict(flags), color_count=sum(colors.values()), color_values=dict(colors))


def verify_legend_end_cell_owned() -> None:
    """Keep425 complete parents and actual paired absence without activation/alpha defaults; e.g. verify_legend_end_cell_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [end_cell_observation(*context) for context in contexts]
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), **view)
        for (family, version, raw), view in zip(contexts, views, strict=True)]
    dialog_baseline('legend-end-cell-owned-reference.json', dict(contexts=425,
        absent_pair_contexts=sum(not view['fields'] for view in views), rows=rows, **end_cell_census(views),
        missing_pairs_defaulted=False, allowed_color_values_inferred=False, alpha_compositing_inferred=False))
