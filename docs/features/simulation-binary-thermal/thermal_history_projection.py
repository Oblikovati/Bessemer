"""Complete independent outer field and scalar-edit history exports (#134)."""
import struct
from typing import cast
from collections.abc import Callable
from owned_thermal import THERMAL_KINDS, owned_thermal_cases
from native_carriers import ThermalCarrier
from thermal_projection import projected_fields, span_shape
from thermal_edit_oracle import scalar_edit_case, thermal_parser


def outer_history(fields: list[bytes], changed: bool) -> bytes:
    """Keep every source and replacement parent in physical order; e.g. outer_history(fields, True)."""
    rows = bytearray(struct.pack('<I', len(fields)))
    for index, field in enumerate(fields):
        length = struct.unpack_from('<I', field)[0]
        path = field[4:4 + length].decode('utf-8')
        derived = changed and path == 'record0.base.integer'
        rows.extend(struct.pack('<IIIII', index, int(derived), 2 if derived else 1, 0, index))
        if derived: rows.extend(struct.pack('<II', 1, 0xffffffff))
    return bytes(rows)


def history_projection_case(kind: int, version: int, raw: bytes, names: tuple[str, ...] = THERMAL_KINDS, parser: Callable[[bytes, int, str], dict[str, object]] = thermal_parser, shape: Callable[[str], tuple[int, list[str]]] = span_shape) -> bytes:
    """Replay complete independent edited fields and literal provenance; e.g. history_projection_case(0, 890501, raw)."""
    oracle = scalar_edit_case(kind, version, raw, names, parser)
    changed, size = struct.unpack_from('<II', oracle, 12)
    expected = oracle[20 + size:]
    parsed = parser(expected, version, names[kind])
    assert parsed['success'] and len(expected) == size
    fields = projected_fields(expected, cast(list[dict[str, object]], parsed['spans']), shape)
    return oracle + struct.pack('<I', len(fields)) + b''.join(fields) + outer_history(fields, bool(changed))


def owned_history_projection() -> bytes:
    """Cover all complete owned gate contexts; e.g. owned_history_projection()."""
    return b''.join(history_projection_case(kind, version, raw) for kind, version, raw in owned_thermal_cases())


def native_history_projection(carriers: list[tuple[ThermalCarrier, bytes]]) -> bytes:
    """Preserve all original matching native contexts; e.g. native_history_projection(carriers)."""
    packet = bytearray()
    for row, raw in carriers:
        kind = THERMAL_KINDS.index(row['stream'].split('/')[-1])
        selected = next(attempt for attempt in row['attempts'] if attempt['success'])
        packet.extend(history_projection_case(kind, cast(int, selected['version']), raw))
    return bytes(packet)
