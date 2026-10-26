"""Independent complete circular patterns with retained map order and dimension aliases (#54)."""
from collections.abc import Callable
from typing import Literal, Protocol
import struct

LineMode = Literal['null', 'line', 'temporary']


class CircularWireMap(Protocol):
    slot: int
    def new(self, name: str, body: Callable[[], bytes]) -> bytes: ...


def dimension(mapping: CircularWireMap, version: int, name: str) -> tuple[bytes, int]:
    """Allocate a complete independent dimension body; e.g. dimension(mapping, 19000, 'moDisplayAngularDim_c')."""
    from display_dimension_fixture_bytes import shared, suffix
    raw = mapping.new(name, lambda: shared(mapping, version, name, False, mapping.slot)
                      +suffix(mapping, version, name, False))
    return raw, mapping.slot


def line_pointer(mapping: CircularWireMap, version: int, mode: LineMode) -> bytes:
    """Exercise complete ordinary and temporary line references; e.g. line_pointer(mapping, 19000, 'line')."""
    from freeze_geometric_references import wire
    from display_dimension_fixture_bytes import body
    from sketch_reference_fixture_bytes import temporary_axis
    if mode == 'null':
        return bytes(2)
    if mode == 'line':
        return mapping.new('moLineRef_w', lambda: body(wire('moLineRef_w', version, False, 0)))
    return mapping.new('moTempAxisRef_w', lambda: body(temporary_axis(version, False)))


def circular_dimensions(mapping: CircularWireMap, version: int, alternate: bool) -> tuple[bytes, int]:
    """Keep primary generic dimensions distinct from angular alternatives; e.g. circular_dimensions(mapping, 19000, True)."""
    first, first_slot = dimension(mapping, version, 'moDisplayDistanceDim_c')
    second, second_slot = (bytes(2), 0) if alternate else dimension(mapping, version, 'moDisplayAngularDim_c')
    return first+second+struct.pack('<HH', first_slot, 0), second_slot


def circular_integers(populated: bool) -> bytes:
    """Retain duplicate keys and signed extrema; e.g. circular_integers(True)."""
    if not populated:
        return bytes(8)
    return struct.pack('<11i', 2, -1, 2, -2147483648, -1, -1, 1, 123, 2, 456, -2)


def circular_suffix(mapping: CircularWireMap, version: int, alternate: bool,
                    populated: bool, line: LineMode, flag: int) -> bytes:
    """Preserve six dimension positions and the unaligned suffix; e.g. circular_suffix(mapping, 19000, True, True, 'null', 255)."""
    prefix, second_slot = circular_dimensions(mapping, version, alternate)
    prefix += mapping.new('suObList', lambda: bytes(2))+line_pointer(mapping, version, line)+bytes([flag])
    other, other_slot = (dimension(mapping, version, 'moDisplayAngularDim_c') if alternate
                         else (struct.pack('<H', second_slot), second_slot))
    prefix += other+bytes([254])+struct.pack('<H', other_slot)+bytes([253, 252, 251, 250, 249])
    geometry = struct.pack('<6d', 1, 2, 3, 0, 0, 1) if version > 0x3efa else b''
    return prefix+circular_integers(populated)+geometry


def circular_object(version: int, rich: bool, alternate: bool, populated: bool = True,
                    line: LineMode = 'null', flag: int = 255, first_slot: int = 0) -> tuple[bytes, int]:
    """Build one complete feature in either map context; e.g. circular_object(19000, True, True, first_slot=96)."""
    from pattern_fixture_bytes import WireMap
    from mirror_fixture_bytes import common_pattern
    if line not in ('null', 'line', 'temporary') or not 0 <= flag <= 255 or first_slot < 0:
        raise ValueError(f'circular line/flag/slot {(line, flag, first_slot)}; expected a known line mode, u8 flag and nonnegative slot')
    mapping = WireMap()
    mapping.slot = first_slot
    raw = mapping.new('moCirPattern_c', lambda:
        common_pattern(mapping, version, rich)+circular_suffix(mapping, version, alternate, populated, line, flag))
    return raw, mapping.slot+1
