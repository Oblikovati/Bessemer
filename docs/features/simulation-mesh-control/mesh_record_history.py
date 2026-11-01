"""Independent nonstructural edit and ordered provenance expectations for counted records (#161)."""
from typing import cast
import struct
from mesh_record_projection import mesh_record_fields
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows
from sheet_projection import sheet_embedded_fields


def mesh_record_history(version: int, raw: bytes, before: dict[str, object]) -> bytes:
    """Preserve completed-empty no-ops and every physical record/reference parent; e.g. mesh_record_history(version, raw, parsed)."""
    from cosworks_mesh_control_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    records = cast(list[dict[str, int]], before['records'])
    changed = bool(records)
    offset = mesh_history_target(before) if changed else 0
    replacement = struct.unpack_from('<i', raw, offset)[0] ^ (32 if changed else 0)
    expected = raw[:offset] + struct.pack('<i', replacement) + raw[offset + 4:]
    after = parse(expected, version, 'MeshControl', archive_reader=archive_reader)
    assert after['success'] and after['end'] == len(raw) and after['records'] == records, after
    fields = mesh_record_fields(expected, after)
    target = 'record0.base.integer' if changed else 'mesh_control.count'
    packet = struct.pack('<IIiII', 0, version, replacement, int(changed), len(raw)) + raw + expected
    return packet + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, changed, target) + nested_copy_rows(fields, sheet_embedded_fields)


def mesh_history_target(parsed: dict[str, object]) -> int:
    """Select a documented nonstructural word within the first physical record; e.g. mesh_history_target(parsed)."""
    records = cast(list[dict[str, int]], parsed['records'])
    spans = cast(list[dict[str, object]], parsed['spans'])
    matches = [span for span in spans if span['label'] == 'base.word198'
               and records[0]['start'] <= cast(int, span['offset']) < records[0]['base_end']]
    assert len(matches) == 1, f'{matches}; expected one first-record base integer'
    span = matches[0]
    assert span['end'] == cast(int, span['offset']) + 4, span
    return cast(int, span['offset'])
