"""C857 derived zeros stay disjoint from complete wire fields and raw legacy contexts (#166)."""
from collections import Counter
from typing import cast
import importlib
import struct
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_field_evidence import dialog_field_reports
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record
from style_preferences_projection import raw_style_native_contexts, raw_style_identity
from local_render_evidence import local_render_report
from legend_word_sources import verify_legend_rejected_identities
from legend_resets_evidence import LEGEND_RESET_LABELS, legend_resets_report


def reset_wire_fields(spans: list[dict[str, object]]) -> list[dict[str, object]]:
    """Retain exact existing C0/C4/C8 words without filling absent fields; e.g. reset_wire_fields(spans)."""
    fields: list[dict[str, object]] = []
    for span in spans:
        if span['label'] not in LEGEND_RESET_LABELS: continue
        raw = bytes.fromhex(cast(str, span['raw_hex']))
        assert len(raw) == cast(int, span['end']) - cast(int, span['offset']) == 4, (span, 'expected four signed32 legend bytes')
        fields.append(dict(label=span['label'], offset=span['offset'], end=span['end'], raw_hex=raw.hex(), signed_value=struct.unpack('<i', raw)[0]))
    return fields


def legend_resets_observation(family: int, version: int, raw: bytes) -> dict[str, object]:
    """Require full EOF and explicit distinction between raw-class unknowns, wire words and derived zeros; e.g. legend_resets_observation(2, 840009, raw)."""
    parsed = actual_post_record(family, version, raw)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, parsed)
    spans = cast(list[dict[str, object]], parsed['spans'])
    fields = reset_wire_fields(spans)
    for field in fields:
        assert raw[cast(int, field['offset']):cast(int, field['end'])].hex() == field['raw_hex']
    assignments: dict[str, int] | None = None
    if version >= 700006:
        assignments = importlib.import_module('cosworks_legend_read_resets').forced_zero_fields(version)
        assert set(assignments) <= set(LEGEND_RESET_LABELS) and all(type(value) is int and value == 0 for value in assignments.values())
        labels = {span['label'] for span in spans}
        assert not labels.intersection(assignments), (version, assignments, fields)
        assert {field['label'] for field in fields} | set(assignments) == set(LEGEND_RESET_LABELS), (version, fields, assignments)
    return dict(fields=fields, zero_assignments=assignments)


def verify_legend_resets_native() -> None:
    """Match all276 original identities,348 derived zeros and eight rejected candidates; e.g. verify_legend_resets_native()."""
    report = legend_resets_report()
    rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    assert Counter(map(raw_style_identity, rows)) == Counter(map(raw_style_identity, original))
    available = {(family, digest(raw), version): legend_resets_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        family = 0 if cast(str, row['stream']).endswith('/cwPostDlgParam') else 2
        assert row['scope'] == ('prefix_only' if family == 0 else 'strict_eof')
        assert available[(family, row['sha256'], row['selected_version'])]['zero_assignments'] == row['read_time_zero_assignments'], row
    current = cast(list[dict[str, object]], dialog_field_reports()[2]['rows'])
    verify_legend_rejected_identities([row for row in current if not any(attempt['success'] for attempt in row['attempts'])])
    counts = Counter(label for row in rows for label in cast(dict[str, int], row['read_time_zero_assignments']))
    assert dict(counts) == report['reset_counts'] and sum(counts.values()) == 348, counts
    dialog_baseline('legend-resets-native-reference.json', dict(accepted_contexts=276, rejected_contexts=8,
        reset_counts=dict(counts), reset_contexts=116, no_reset_contexts=160,
        assignments_without_wire_offsets=True, additional_stored_byte_credit=0, wire_bytes_rewritten=False))


def verify_legend_resets_owned() -> None:
    """Keep425 complete parents, actual interval edges and unsupported raw-class absence; e.g. verify_legend_resets_owned()."""
    contexts = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    assert len(contexts) == 425
    views = [legend_resets_observation(*context) for context in contexts]
    assert sum(view['zero_assignments'] is None for view in views) == 20
    rows = [dict(family=family, version=version, bytes=len(raw), sha256=digest(raw), **view)
        for (family, version, raw), view in zip(contexts, views, strict=True)]
    counts = Counter(str(len(cast(dict[str, int], view['zero_assignments']))) for view in views if view['zero_assignments'] is not None)
    dialog_baseline('legend-resets-owned-reference.json', dict(contexts=425, raw_class_unsupported_contexts=20,
        reset_count_contexts=dict(counts), rows=rows, wire_absence_defaulted=False, constructor_state_inferred=False))
