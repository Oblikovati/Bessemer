"""Independent complete edited images using actual reference-parser field boundaries (#134)."""
import struct
from collections.abc import Callable
from typing import cast
from owned_thermal import THERMAL_KINDS, owned_thermal_cases
from native_carriers import ThermalCarrier


def thermal_parser(raw: bytes, version: int, kind: str) -> dict[str, object]:
    """Invoke the actual complete thermal callback; e.g. thermal_parser(raw, 890501, 'Temperature')."""
    from cosworks_binary_thermal_transform_audit import parse_native, archive_reader
    return cast(dict[str, object], parse_native(raw, version, kind, archive_reader))


def scalar_edit_case(kind: int, version: int, raw: bytes, names: tuple[str, ...] = THERMAL_KINDS, parser: Callable[[bytes, int, str], dict[str, object]] = thermal_parser) -> bytes:
    """Change the actual reported first subtype word and replay the complete image; e.g. scalar_edit_case(0, 890501, raw)."""
    before = parser(raw, version, names[kind])
    assert before['success']
    span = next((span for span in cast(list[dict[str, object]], before['spans']) if span['label'] == 'record0.base.integer'), None)
    start = cast(int, span['offset']) if span is not None else 0
    if span is not None: assert span['end'] == start + 4
    old = struct.unpack_from('<i', raw, start)[0]
    replacement = old ^ 32 if span is not None else old
    expected = raw[:start] + struct.pack('<i', replacement) + raw[start + 4:]
    after = parser(expected, version, names[kind])
    assert after['success'] and after['end'] == len(expected)
    assert after['archive_records'] == before['archive_records']
    if span is not None: assert cast(list[dict[str, object]], after['complete_records'])[0]['subtype'] == replacement
    header = struct.pack('<IIiII', kind, version, replacement, int(span is not None), len(raw))
    return header + raw + expected


def owned_scalar_edit_packet() -> bytes:
    """Use independent literal inputs and actual reported boundaries; e.g. owned_scalar_edit_packet()."""
    return b''.join(scalar_edit_case(kind, version, raw) for kind, version, raw in owned_thermal_cases())


def native_scalar_edit_packet(carriers: list[tuple[ThermalCarrier, bytes]]) -> bytes:
    """Retain each original successful native version context without guessing; e.g. native_scalar_edit_packet(carriers)."""
    packet = bytearray()
    for row, raw in carriers:
        kind = THERMAL_KINDS.index(row['stream'].split('/')[-1])
        selected = next(attempt for attempt in row['attempts'] if attempt['success'])
        packet.extend(scalar_edit_case(kind, cast(int, selected['version']), raw))
    return bytes(packet)
