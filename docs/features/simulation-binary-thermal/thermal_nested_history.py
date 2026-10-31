"""Independent physical nested occurrence parents for complete scalar edits (#134)."""
import struct
from typing import cast
from collections.abc import Callable
from owned_thermal import THERMAL_KINDS, owned_thermal_cases
from native_carriers import ThermalCarrier
from thermal_projection import FieldShape, projected_fields, span_shape
from thermal_edit_oracle import thermal_parser
from thermal_history_projection import history_projection_case
from embedded_field_projection import embedded_field_case


def nested_copy_rows(fields: list[bytes], project: Callable[[bytes], bytes] = embedded_field_case) -> bytes:
    """Derive every nested occurrence from independently observed source fields; e.g. nested_copy_rows(fields)."""
    rows = bytearray()
    count = 0
    for outer, field in enumerate(fields):
        length = struct.unpack_from('<I', field)[0]
        tag = struct.unpack_from('<I', field, 4 + length)[0]
        if tag != 5: continue
        raw = field[16 + length:]
        observed = project(raw)
        field_count = struct.unpack_from('<I', observed, 4 + len(raw))[0]
        for index in range(field_count):
            rows.extend(struct.pack('<IIIIIII', outer, outer, index, 0, 1, 0, index))
            count += 1
    return struct.pack('<I', count) + rows


def nested_history_case(kind: int, version: int, raw: bytes, names: tuple[str, ...] = THERMAL_KINDS, parser: Callable[[bytes, int, str], dict[str, object]] = thermal_parser, shape: Callable[[str, int], list[FieldShape]] = span_shape) -> bytes:
    """Compose complete independently verified scalar edits and nested provenance; e.g. nested_history_case(0, 890501, raw)."""
    parsed = parser(raw, version, names[kind])
    assert parsed['success'] and parsed['end'] == len(raw)
    fields = projected_fields(raw, cast(list[dict[str, object]], parsed['spans']), shape)
    return history_projection_case(kind, version, raw, names, parser, shape) + nested_copy_rows(fields)


def owned_nested_history_packet() -> bytes:
    """Retain every owned historical context and physical parent; e.g. owned_nested_history_packet()."""
    return b''.join(nested_history_case(kind, version, raw) for kind, version, raw in owned_thermal_cases())


def native_nested_history_packet(carriers: list[tuple[ThermalCarrier, bytes]]) -> bytes:
    """Retain all matching native scalar-edit contexts and nested parents; e.g. native_nested_history_packet(carriers)."""
    packet = bytearray()
    for row, raw in carriers:
        kind = THERMAL_KINDS.index(row['stream'].split('/')[-1])
        selected = next(attempt for attempt in row['attempts'] if attempt['success'])
        packet.extend(nested_history_case(kind, cast(int, selected['version']), raw))
    return bytes(packet)
