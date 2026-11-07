"""Independent equality-selected axes and direct arrays for coordinate systems (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from axis_owner_bytes import geometry_header
from point_fixture_bytes import geometric_pointer, projection_point
from sweep_fixture_bytes import shared_object

AXES = ('moCSysAxisData_c', 'moLineCSysAxisData_c', 'moPntCSysAxisData_c')


def axis_body(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, style: int) -> bytes:
    """Use a direct ReadCount array and retain nonunit vector bits; e.g. axis_body(mapping, 19000, ids, True, 1)."""
    raw = bytes(2)
    if rich:
        name = 'moPointRef_w' if style == 2 else 'moLineRef_w'
        first = geometric_pointer(mapping, version, identities, name)
        raw = struct.pack('<H', 3)+first+geometric_pointer(mapping, version, identities, name)+bytes(2)
    return raw+struct.pack('<3Q', 0x8000000000000000, 0x4000000000000000 if rich else 0, 0x7ff8000000001234 if rich else 0)


def selected_axis(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, style: int) -> bytes:
    """Reuse selected-axis identity across distinct member pointers; e.g. selected_axis(mapping, 19000, ids, True, 0)."""
    if not rich:
        return bytes(2)
    return shared_object(mapping, identities, 'coordinate-axis', AXES[style],
        lambda: axis_body(mapping, version, identities, True, style))


def payload(mapping: WireMap, version: int, identities: dict[str, int], rich: bool,
            selectors: tuple[int, int], style: int, edited: bool) -> bytes:
    """Select by equality and emit in numeric order; e.g. payload(mapping, 19000, ids, True, (3, 1), 1, False)."""
    raw = projection_point(mapping, version, identities, rich, 1 if style == 2 else 0)+struct.pack('<2H', *selectors)
    raw += b''.join(selected_axis(mapping, version, identities, rich, style) for number in (1, 2, 3) if number in selectors)
    raw += struct.pack('<3B3d', 255 if rich else 0, 254 if rich else 0, 253 if rich else 0,
        -0.0, 12345 if edited else 1.25, -2.5)
    if version > 0x371a:
        raw += struct.pack('<2i3Q', -2147483648, 2147483647, 0x8000000000000000,
            0x7ff8000000001234, 0x3ff4000000000000)
    return raw


def coordinate_object(version: int, rich: bool, selectors: tuple[int, int], style: int = 0,
                      first_slot: int = 0, edited: bool = False, helper: bool = False) -> tuple[bytes, int]:
    """Build complete coordinate or axis objects in either map context; e.g. coordinate_object(19000, True, (3, 1), 2, 96)."""
    if style not in range(3) or first_slot < 0 or len(selectors) != 2 or any(not 0 <= value <= 65535 for value in selectors):
        raise ValueError(f'coordinate fixture {(style, first_slot, selectors)}; expected style 0..2, nonnegative slot and two u16 selectors')
    mapping = WireMap();mapping.slot = first_slot;identities: dict[str, int] = {}
    def contents() -> bytes:
        if helper:
            return axis_body(mapping, version, identities, rich, style)
        return geometry_header(mapping, version, rich)+payload(mapping, version, identities, rich, selectors, style, edited)
    raw = mapping.new(AXES[style] if helper else 'moCoordSys_c', contents)
    return raw, mapping.slot+1
