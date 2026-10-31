"""Independent physical fields and complete header/pair edit history for C-726 (#144)."""
import struct
from typing import cast
from owned_groups import KINDS
from thermal_projection import FieldShape, projected_fields, repeated_shape, span_shape
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows


def group_shape(label: str, size: int) -> list[FieldShape]:
    """Expand exact mixed group values while reusing reference shapes; e.g. group_shape(label, 20)."""
    if '.references.' in label: return span_shape(label, size)
    if label == 'group.doubleb8_three_words':
        return [('group.doubleb8', 2, 8)] + [(f'group.{word}', 0, 4) for word in ('wordc0', 'worda8', 'wordac')]
    if label in ('group.textc8', 'folder.text28'): return [(label, 3, size)]
    if label in ('group.saved_version_double', 'joint.doublef8'): return [(label, 2, 8)]
    if label == 'joint.wordsf0_f4': return [('joint.wordf0', 0, 4), ('joint.wordf4', 0, 4)]
    if label == 'joint.three_doubles': return repeated_shape(label, 2, 8, 3)
    if label.endswith(('.identity', '.beam_ids', '.pairs')):
        assert size % 4 == 0, (label, size)
        return repeated_shape(label, 0, 4, size // 4)
    assert size == 4, (label, size)
    return [(label, 0, 4)]


def actual_parser(raw: bytes, version: int, kind: int, expected: int | None) -> dict[str, object]:
    """Use the unchanged actual reader and explicit dependencies; e.g. actual_parser(raw, 810030, 1, None)."""
    from cosworks_beam_group_layout import parse
    from cosworks_binary_thermal_transform_audit import archive_reader
    return cast(dict[str, object], parse(raw, version, KINDS[kind], archive_reader=archive_reader, expected_joint_count=expected))


def fields_for(raw: bytes, parsed: dict[str, object]) -> list[bytes]:
    """Require complete independent acceptance before projecting values; e.g. fields_for(raw, parsed)."""
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), group_shape)


def projection_case(kind: int, version: int, expected: int | None, raw: bytes) -> bytes:
    """Keep explicit kind/count context and every typed physical value; e.g. projection_case(2, 891107, 2, raw)."""
    fields = fields_for(raw, actual_parser(raw, version, kind, expected))
    return struct.pack('<IIiI', kind, version, -1 if expected is None else expected, len(raw)) + raw + struct.pack('<I', len(fields)) + b''.join(fields)


def edit_target(kind: int, fields: list[bytes]) -> tuple[str, int, int, bool]:
    """Prefer a stored pair value; lists without pairs have a complete exact no-op; e.g. edit_target(2, fields)."""
    for field in fields:
        length = struct.unpack_from('<I', field)[0]
        path = field[4:4 + length].decode()
        if kind == 2 and '.pairs[' not in path: continue
        start = struct.unpack_from('<I', field, 8 + length)[0]
        value = struct.unpack_from('<i', field, 16 + length)[0]
        return path, start, value ^ 32, True
    assert kind == 2
    value = struct.unpack_from('<i', fields[0], 16 + struct.unpack_from('<I', fields[0])[0])[0]
    return 'joint_list.count', 0, value, False


def history_case(kind: int, version: int, expected: int | None, raw: bytes) -> bytes:
    """Derive complete independent edited fields and ordered outer/nested parents; e.g. history_case(2, 891107, 2, raw)."""
    fields = fields_for(raw, actual_parser(raw, version, kind, expected))
    path, start, replacement, changed = edit_target(kind, fields)
    result = raw[:start] + struct.pack('<i', replacement) + raw[start + 4:]
    after = fields_for(result, actual_parser(result, version, kind, expected))
    name = path.encode()
    return (struct.pack('<iI', -1 if expected is None else expected, len(name)) + name
            + struct.pack('<IIiII', kind, version, replacement, int(changed), len(raw))
            + raw + result + struct.pack('<I', len(after)) + b''.join(after)
            + outer_history(after, changed, path) + nested_copy_rows(fields))
