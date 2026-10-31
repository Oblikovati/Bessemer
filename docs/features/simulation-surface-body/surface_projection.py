"""Independent complete SurfaceBody primitives and isolated word-edit history (#153)."""
import struct
from typing import cast
from thermal_projection import FieldShape, projected_fields, repeated_shape
from child_projection import history_packet


def surface_shape(label: str, size: int) -> list[FieldShape]:
    """Expand only declared primitive groups, preserving physical order; e.g. surface_shape('words100_94', 8)."""
    if label == 'text68' or label.startswith('list108.') and label != 'list108.count': return [(label, 3, size)]
    if label in ('words84_8c', 'words100_94'):
        assert size == 8
        return repeated_shape(label, 0, 4, 2)
    if label == 'words98_a4_doublesa8_c8':
        assert size == 56
        return repeated_shape('words98_a4', 0, 4, 4) + repeated_shape('doublesa8_c8', 2, 8, 5)
    assert label in ('word70', 'word80', 'word88', 'word90', 'word144', 'list108.count'), label
    assert size == 4, (label, size)
    return [(label, 0, 4)]


def surface_fields(raw: bytes, version: int) -> list[bytes]:
    """Require complete actual framing, never project a failed prefix as a body; e.g. surface_fields(raw, 870022)."""
    from cosworks_surface_body_layout import parse
    parsed = parse(raw, version)
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), surface_shape)


def surface_projection(version: int, raw: bytes) -> bytes:
    """Retain every independent raw value/tag/path/span; e.g. surface_projection(870022, raw)."""
    fields = surface_fields(raw, version)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def surface_history(version: int, raw: bytes) -> bytes:
    """Reparse the word70 edit and retain every ordered source/replacement parent; e.g. surface_history(870022, raw)."""
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    expected = struct.pack('<i', replacement) + raw[4:]
    return history_packet(0, version, raw, 0, surface_fields(expected, version), 'word70')
