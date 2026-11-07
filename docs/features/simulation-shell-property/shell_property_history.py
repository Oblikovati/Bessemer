"""Independently re-read shell edits and every ordered outer/nested parent (#159)."""
from typing import cast
import struct
from shell_property_projection import shell_property_fields, shell_presence_packet
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows
from sheet_projection import sheet_embedded_fields


def shell_property_history(kind: int, version: int, raw: bytes, before: dict[str, object], presence: list[bool] | None) -> bytes:
    """Edit only the nonstructural base word and preserve all delegated bodies; e.g. shell_property_history(0, 891106, raw, parsed, presence)."""
    from cosworks_shell_property_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    assert before['success'] and before['end'] == len(raw), before
    span = next(item for item in cast(list[dict[str, object]], before['spans']) if item['label'] == 'base.word198')
    offset = cast(int, span['offset'])
    assert span['end'] == offset + 4, span
    replacement = struct.unpack_from('<i', raw, offset)[0] ^ 32
    expected = raw[:offset] + struct.pack('<i', replacement) + raw[offset + 4:]
    after = parse(expected, version, 'Shell', archive_reader=archive_reader, layer_presence=presence)
    assert after['base_end'] == before['base_end'] and after['subtype'] == before['subtype']
    fields = shell_property_fields(expected, version, after)
    oracle = shell_presence_packet(presence) + struct.pack('<IIiII', kind, version, replacement, 1, len(raw)) + raw + expected
    return oracle + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, True, 'property.base.integer') + nested_copy_rows(fields, sheet_embedded_fields)
