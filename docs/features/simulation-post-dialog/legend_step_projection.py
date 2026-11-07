"""Complete C851 raw FC observations without runtime availability inference (#166)."""
from collections import Counter
from typing import cast
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_step_evidence import legend_step_report, legend_step_fields


def legend_step_observation(family: int, version: int, raw: bytes) -> list[dict[str, object]]:
    """Require complete EOF and exact version-gated FC spans; e.g. legend_step_observation(2, 890103, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    fields = legend_step_fields(cast(list[dict[str, object]], parsed['spans']))
    assert not fields or version >= 890103, (version, fields)
    for field in fields:
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    return fields


def verify_legend_step_native() -> None:
    """Match276 accepted identities,109 FC words and eight original rejected streams; e.g. verify_legend_step_native()."""
    report = legend_step_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_step_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['fields'], row
    rejected = [row for row in rows if row['selected_version'] is None]
    assert all(row['fields'] == [] for row in rejected)
    verify_legend_rejected_identities(rejected)
    counts = legend_step_census([cast(list[dict[str, object]], row['fields']) for row in accepted])
    assert counts == {key: report[key] for key in counts}, counts
    dialog_baseline('legend-step-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        absent_fc_contexts=167, **counts, additional_stored_byte_credit=0, runtime_availability_known=False))


def legend_step_census(views: list[list[dict[str, object]]]) -> dict[str, object]:
    """Preserve stream multiplicity and all raw signed patterns; e.g. legend_step_census(views)."""
    values: Counter[str] = Counter()
    for fields in views:
        for field in fields:
            values[str(field['signed_value'])] += 1
    return dict(field_count=sum(values.values()), value_counts=dict(values))


def verify_legend_step_owned() -> None:
    """Keep425 complete parents with actual version-conditioned FC absence; e.g. verify_legend_step_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_step_observation(*context) for context in contexts]
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), fields=fields)
        for (family, version, raw), fields in zip(contexts, views, strict=True)]
    dialog_baseline('legend-step-owned-reference.json', dict(contexts=425,
        absent_fc_contexts=sum(not fields for fields in views), rows=rows, **legend_step_census(views),
        absent_fc_defaulted=False, universal_visibility_inferred=False, runtime_availability_inferred=False))
