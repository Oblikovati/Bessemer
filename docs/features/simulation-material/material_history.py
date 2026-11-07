"""Complete independently replayed Material header edits and every physical outer history parent (#140)."""
import struct
from typing import cast
from material_reference import actual_material_parser
from material_projection import material_shape
from thermal_projection import projected_fields
from thermal_history_projection import outer_history


def material_history_case(raw: bytes, version: int, before: dict[str, object]) -> bytes:
    """Locate the real source word and replay the complete edited image; e.g. material_history_case(raw, 830036, before)."""
    assert before['success'] and before['end'] == len(raw)
    span = next(span for span in cast(list[dict[str, object]], before['spans']) if span['label'] == 'word68')
    start = cast(int, span['offset'])
    assert span['end'] == start + 4
    replacement = struct.unpack_from('<i', raw, start)[0] ^ 32
    expected = raw[:start] + struct.pack('<i', replacement) + raw[start + 4:]
    after = actual_material_parser(expected, version)
    assert after['success'] and after['end'] == len(expected)
    assert after['records'] == before['records'] and after['curves'] == before['curves']
    fields = projected_fields(expected, cast(list[dict[str, object]], after['spans']), material_shape)
    oracle = struct.pack('<IIiII', 0, version, replacement, 1, len(raw)) + raw + expected
    return oracle + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, True, 'word68')
