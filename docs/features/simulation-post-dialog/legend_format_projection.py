"""Complete-body format inputs without initialization or display claims (#166)."""
from collections import Counter
from typing import cast
import importlib
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from legend_word_sources import verify_legend_rejected_identities
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_format_evidence import legend_format_report


def legend_format_observation(family: int, version: int, raw: bytes) -> dict[str, object]:
    """Require complete EOF before projecting present words; e.g. legend_format_observation(2, 700006, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    helper = importlib.import_module('cosworks_legend_number_format')
    view = cast(dict[str, object], helper.decode_fields(parsed['spans']))
    for field in cast(list[dict[str, object]], view['fields']):
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    return view


def verify_legend_format_native() -> None:
    """Retain276 accepted identities, eight failures and prefix qualifications; e.g. verify_legend_format_native()."""
    report = legend_format_report()
    rows = cast(list[dict[str, object]], report['rows'])
    accepted = [row for row in rows if row['selected_version'] is not None]
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, accepted)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_format_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in accepted:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])] == row['view'], row
    verify_legend_format_rejected_rows(rows)
    counts = legend_format_census([cast(dict[str, object], row['view']) for row in accepted])
    assert counts == {key: report[key] for key in counts}, counts
    dialog_baseline('legend-format-native-reference.json', dict(accepted_contexts=len(accepted), rejected_contexts=8,
        **counts, additional_stored_byte_credit=0, runtime_value_known=False, native_rendered=False))


def verify_legend_format_rejected_rows(rows: list[dict[str, object]]) -> None:
    """Match each rejected identity to its retained failed attempts; e.g. verify_legend_format_rejected_rows(rows)."""
    rejected = [row for row in rows if row['selected_version'] is None]
    assert len(rejected) == 8
    for row in rejected:
        assert row['view'] == dict(fields=[]), row
    verify_legend_rejected_identities(rejected)


def legend_format_census(views: list[dict[str, object]]) -> dict[str, object]:
    """Preserve stream multiplicity in role/value/string counts; e.g. legend_format_census(views)."""
    counts: Counter[str] = Counter()
    values: dict[str, Counter[str]] = {}
    formats: Counter[str] = Counter()
    for view in views:
        for field in cast(list[dict[str, object]], view['fields']):
            name = cast(str, field['name'])
            counts[name] += 1
            values.setdefault(name, Counter())[str(field['signed_value'])] += 1
        if 'derived_format_string' in view: formats[cast(str, view['derived_format_string'])] += 1
    return dict(field_counts=dict(counts), value_counts={name: dict(words) for name, words in values.items()}, format_counts=dict(formats))


def verify_legend_format_owned() -> None:
    """Retain425 contexts and older-field absences; e.g. verify_legend_format_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_format_observation(*context) for context in contexts]
    assert sum(not view['fields'] for view in views) == 20
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), view=view)
        for (family, version, raw), view in zip(contexts, views, strict=True)]
    dialog_baseline('legend-format-owned-reference.json', dict(contexts=len(contexts), opaque_legacy=20,
        rows=rows, **legend_format_census(views), missing_fields_defaulted=False, native_rendered=False))
