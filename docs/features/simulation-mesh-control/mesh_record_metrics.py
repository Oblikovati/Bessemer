"""Independent counted-record resource limits and exact operation coverage (#161)."""
from typing import cast
import struct
from mesh_record_projection import mesh_record_fields
from shell_property_metrics import shell_entry_count, shell_text_maximum, shell_archive_counts


def mesh_record_budget_packet(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain seven resource axes and every declared/physical record extent; e.g. mesh_record_budget_packet(version, raw, parsed)."""
    spans = cast(list[dict[str, object]], parsed['spans'])
    archives = cast(list[dict[str, object]], parsed['archive_records'])
    records = cast(list[dict[str, int]], parsed['records'])
    fields = mesh_record_fields(raw, parsed)
    entries = struct.unpack_from('<i', raw)[0] + shell_entry_count(raw, version, spans)
    nested = shell_archive_counts(raw, archives)
    root = max((cast(int, item['end']) - cast(int, item['offset']) for item in archives), default=0)
    objects = max((sum(cast(dict[str, int], item['classes']).values()) for item in archives), default=0)
    limits = struct.pack('<6I', entries, shell_text_maximum(fields), root, objects, len(fields), max(nested, default=0))
    statistics = struct.pack('<3I', len(records), len(nested), sum(nested))
    extents = b''.join(struct.pack('<4I', record['index'], record['start'], record['base_end'], record['end']) for record in records)
    return struct.pack('<III', 0, version, len(raw)) + raw + limits + statistics + extents
