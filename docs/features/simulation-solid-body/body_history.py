"""Independent complete scalar edits and nested copy provenance for solid bodies (#150)."""
import struct
from collections.abc import Callable
from typing import cast
from thermal_projection import projected_fields
from thermal_history_projection import outer_history
from thermal_nested_history import nested_copy_rows
from embedded_field_projection import embedded_field_case
from body_projection import body_shape


def body_embedded_case(raw: bytes) -> bytes:
    """Observe actual C-734 callback calls without replacing readers; e.g. body_embedded_case(raw)."""
    from cosworks_embedded_body_ids import archive_reader
    return embedded_field_case(raw, archive_reader)


def edited_body_fields(version: int, raw: bytes) -> list[bytes]:
    """Reparse every edited field through the unchanged source grammar; e.g. edited_body_fields(880004, raw)."""
    from cosworks_solid_body_layout import parse
    from cosworks_embedded_body_ids import archive_reader
    parsed = parse(raw, version, archive_reader=archive_reader)
    assert parsed['success'] and parsed['end'] == len(raw)
    return projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), body_shape)


def body_history(version: int, raw: bytes, project_fields: Callable[[int, bytes], list[bytes]] = edited_body_fields, project_embedded: Callable[[bytes], bytes] = body_embedded_case) -> bytes:
    """Derive all outer and nested occurrence parents independently; e.g. body_history(880004, raw)."""
    replacement = struct.unpack_from('<i', raw)[0] ^ 32
    expected = struct.pack('<i', replacement) + raw[4:]
    fields = project_fields(version, expected)
    return (struct.pack('<IIiII', 0, version, replacement, 1, len(raw)) + raw + expected
            + struct.pack('<I', len(fields)) + b''.join(fields)
            + outer_history(fields, True, 'persistid.words[0]') + nested_copy_rows(fields, project_embedded))
