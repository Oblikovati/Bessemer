"""Independent complete signed/text fields and isolated direct-header history (#148)."""
import struct
from typing import cast
from thermal_projection import FieldShape, projected_fields
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows


def solid_shape(label: str, size: int) -> list[FieldShape]:
    """Keep each original list ordinal and raw frame atomic; e.g. solid_shape('listb0.0', 4)."""
    prefix, _, ordinal = label.partition('.')
    if prefix in ('listb0', 'liste8') and ordinal.isdecimal(): return [(label, 3, size)]
    assert label in ('member48', 'member4c', 'member70', 'member180', 'listb0.count', 'liste8.count') and size == 4, (label, size)
    return [(label, 0, 4)]


def solid_fields(raw: bytes, parsed: dict[str, object]) -> list[bytes]:
    """Project only completely accepted original primitive calls; e.g. solid_fields(raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), solid_shape)


def projection_case(version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Retain all raw field values, paths and body-relative spans; e.g. projection_case(870008, raw, parsed)."""
    fields = solid_fields(raw, parsed)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def history_case(version: int, raw: bytes) -> bytes:
    """Independently edit the first word and derive every ordered parent; e.g. history_case(870008, raw)."""
    from cosworks_solid_manager_layout import parse
    before = parse(raw, version)
    assert before['success'] and before['end'] == len(raw)
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    expected = struct.pack('<i', replacement) + raw[4:]
    after = parse(expected, version)
    assert before['lists'] == after['lists']
    fields = solid_fields(expected, after)
    return (struct.pack('<IIiII', 0, version, replacement, 1, len(raw)) + raw + expected
            + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, True, 'member48') + nested_copy_rows(fields))
