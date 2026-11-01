"""Complete independent composite primitives and scalar edit/history packets (#157)."""
from typing import cast
import struct
from thermal_projection import FieldShape, projected_fields, repeated_shape
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows
from body_projection import body_shape
from sheet_projection import sheet_embedded_fields


def composite_context(kind: str, profile: str) -> int:
    """Normalize the named stream/profile boundary once; e.g. composite_context('CompositeExtra', 'writer')."""
    assert kind in ('CompositeFlipTurn', 'CompositeExtra'), kind
    assert profile in ('reader', 'writer'), profile
    return 2 * int(kind == 'CompositeExtra') + int(profile == 'writer')


def composite_shape(label: str, size: int) -> list[FieldShape]:
    """Expand only statically documented physical groups; e.g. composite_shape('six_double_bits', 48)."""
    if label == 'four_word_records':
        assert size % 16 == 0, size
        return [(f'record{record}.word{word}', 0, 4) for record in range(size // 16) for word in range(4)]
    if label == 'six_double_bits':
        assert size == 48, size
        return repeated_shape('double400_428', 2, 8, 6)
    if label == 'persistid.byte_count': return [('persistid.blob_count', 0, 4)]
    if label.startswith('persistid.'): return body_shape(label, size)
    assert label in ('count', 'member37c', 'member3f0', 'member3f4', 'member3f8') and size == 4, (label, size)
    return [(label, 0, 4)]


def composite_fields(raw: bytes, parsed: dict[str, object]) -> list[bytes]:
    """Retain complete accepted source calls, including empty blobs and both archive roots; e.g. composite_fields(raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), composite_shape)


def composite_projection(context: int, version: int, raw: bytes, parsed: dict[str, object]) -> bytes:
    """Keep every primitive value/path/tag/span without native dispatch inference; e.g. composite_projection(3, 840002, raw, parsed)."""
    fields = composite_fields(raw, parsed)
    return struct.pack('<III', context, version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def composite_scalar_edit(raw: bytes, kind: str) -> tuple[int, int, str, bytes]:
    """Change a non-structural word or retain an empty-count no-op; e.g. composite_scalar_edit(raw, 'CompositeFlipTurn')."""
    if kind == 'CompositeExtra': offset, path = 0, 'member37c'
    elif struct.unpack_from('<i', raw)[0]: offset, path = 4, 'record0.word0'
    else: return 0, 0, 'count', raw
    replacement = struct.unpack_from('<i', raw, offset)[0] ^ 32
    edited = raw[:offset] + struct.pack('<i', replacement) + raw[offset + 4:]
    return replacement, 1, path, edited


def composite_history(context: int, version: int, raw: bytes, kind: str, profile: str) -> bytes:
    """Re-read a known isolated edit and every ordered outer/nested parent; e.g. composite_history(3, 840002, raw, kind, profile)."""
    from cosworks_composite_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    replacement, changed, path, edited = composite_scalar_edit(raw, kind)
    after = parse(edited, version, kind, wire_profile=profile, archive_reader=archive_reader)
    fields = composite_fields(edited, after)
    return (struct.pack('<IIiII', context, version, replacement, changed, len(raw)) + raw + edited
            + struct.pack('<I', len(fields)) + b''.join(fields)
            + outer_history(fields, bool(changed), path) + nested_copy_rows(fields, sheet_embedded_fields))
