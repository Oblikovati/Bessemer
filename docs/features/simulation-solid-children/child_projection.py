"""Complete component-reader projections and separately static folder histories (#149)."""
import struct
from typing import cast
from thermal_projection import FieldShape, projected_fields
from thermal_history_projection import outer_history
from owned_children import folder_fields


def component_shape(label: str, size: int) -> list[FieldShape]:
    """Keep each actual reader call atomic; e.g. component_shape('0x70', 4)."""
    if label == 'text68': return [('component.text68', 3, size)]
    assert label in ('0x70', '0x74', '0x78', '0x7c', '0xc0') and size == 4, (label, size)
    return [('component.word' + label[2:], 0, 4)]


def component_fields(raw: bytes, version: int) -> list[bytes]:
    """Require complete actual-reader acceptance before projecting; e.g. component_fields(raw, 840013)."""
    from cosworks_solid_component_layout import parse
    parsed = parse(raw, version)
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), component_shape)


def component_projection(version: int, raw: bytes) -> bytes:
    """Retain every independent path, tag, offset and raw value; e.g. component_projection(840013, raw)."""
    fields = component_fields(raw, version)
    return struct.pack('<II', version, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def history_packet(kind: int, version: int, raw: bytes, offset: int, fields: list[bytes], path: str) -> bytes:
    """Derive every ordered parent independently of Rust; e.g. history_packet(0, version, raw, offset, fields, path)."""
    replacement = struct.unpack_from('<i', raw, offset)[0] ^ 32
    expected = raw[:offset] + struct.pack('<i', replacement) + raw[offset + 4:]
    return (struct.pack('<IIiII', kind, version, replacement, 1, len(raw)) + raw + expected
            + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, True, path) + struct.pack('<I', 0))


def component_history(version: int, raw: bytes) -> bytes:
    """Reparse an isolated first-word edit with the actual reader; e.g. component_history(840013, raw)."""
    from cosworkspersist import read_text
    offset = read_text(raw).end
    replacement = struct.unpack_from('<i', raw, offset)[0] ^ 32
    expected = raw[:offset] + struct.pack('<i', replacement) + raw[offset + 4:]
    return history_packet(0, version, raw, offset, component_fields(expected, version), 'component.word70')


def folder_history(version: int, profile: int) -> bytes:
    """Use literal vectors only, without claiming a folder reader; e.g. folder_history(840014, 1)."""
    fields = folder_fields(version, profile)
    raw = b''.join(value for _, _, value in fields)
    fields[0] = (fields[0][0], 0, struct.pack('<i', struct.unpack_from('<i', raw)[0] ^ 32))
    projected: list[bytes] = []
    offset = 0
    for label, tag, value in fields:
        name = label.encode()
        projected.append(struct.pack('<I', len(name)) + name + struct.pack('<III', tag, offset, offset + len(value)) + value)
        offset += len(value)
    return history_packet(1, version, raw, 0, projected, 'folder.word58')
