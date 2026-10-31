"""Independent declared-entry and archive budgets for complete Mesh Property operations (#163)."""
from typing import cast
import re
import struct
from owned_property import property_direct_spans
from property_literal_values import PropertyLiteralSpan
from property_projection import property_fields
from shell_property_metrics import shell_text_maximum, shell_archive_counts


def property_declared_entries(raw: bytes, version: int, spans: list[dict[str, object]]) -> int:
    """Count physical declarations, excluding blob byte lengths and implicit duration words; e.g. property_declared_entries(raw, version, spans)."""
    entries = 0
    counts = (r'mesh\.(?:(?:text_list|settings\.references|direct_records|array_objects|map|double_array|newest_set)\.count|'
              r'duration_count|record\d+\.extension(?:2c|38)\.count|array_object\d+\.count[01]|map\d+\.child_count)')
    for span in spans:
        label, start = cast(str, span['label']), cast(int, span['offset'])
        if re.fullmatch(counts, label): entries += struct.unpack_from('<i', raw, start)[0]
        if re.fullmatch(r'mesh\.record\d+\.extension(?:2c|38)\.nested\d+', label):
            entries += nested_property_entries(raw, version, start, cast(int, span['end']))
    return entries


def nested_property_entries(raw: bytes, version: int, start: int, end: int) -> int:
    """Observe both nested array declarations from the actual bounded prefix parser; e.g. nested_property_entries(raw, version, start, end)."""
    from cosworks_mesh_nested_record import parse
    parsed = parse(raw, version, offset=start, require_eof=False)
    assert parsed['success'] and parsed['end'] == end, parsed
    return sum(struct.unpack_from('<i', raw, row['offset'])[0]
               for row in parsed['spans'] if row['label'] in ('array0.count', 'array1.count'))


def property_budget_packet(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain seven independent exact resource axes and every physical coverage statistic; e.g. property_budget_packet(version, raw, parsed)."""
    spans = cast(list[dict[str, object]], parsed['spans'])
    archives = cast(list[dict[str, object]], parsed['archive_records'])
    fields = property_fields(raw, version, parsed)
    nested = shell_archive_counts(raw, archives)
    root = max((cast(int, item['end']) - cast(int, item['offset']) for item in archives), default=0)
    objects = max((sum(cast(dict[str, int], item['classes']).values()) for item in archives), default=0)
    entries = property_declared_entries(raw, version, spans)
    limits = struct.pack('<6I', entries, shell_text_maximum(fields), root, objects, len(fields), max(nested, default=0))
    records = property_direct_spans(cast(list[PropertyLiteralSpan], spans))
    nested_count = sum(bool(re.fullmatch(r'mesh\.record\d+\.extension(?:2c|38)\.nested\d+', cast(str, span['label']))) for span in spans)
    statistics = struct.pack('<4I', len(records), nested_count, len(nested), sum(nested))
    extents = b''.join(struct.pack('<3I', record['index'], record['start'], record['end']) for record in records)
    return struct.pack('<III', 0, version, len(raw)) + raw + limits + statistics + extents
