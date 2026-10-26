"""Independent connection/route roots, direct parameters and radial dimensions (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from axis_owner_bytes import geometry_header
from point_fixture_bytes import geometric_pointer, projection_point
from sweep_fixture_bytes import dimension
from toolbox_fixture_bytes import ansi_string
from owned_name_edit import short_wide_name
from display_dimension_fixture_bytes import body as strip_tag
from dimension_fixture_bytes import dimension as geometric_dimension, handle_base

NAMES = ('moConnectionPoint_c', 'moRoutePoint_c')
PARAMETERS = ('Specification', 'MinStraightLength', 'EndLengthAdjustment', 'EndType')


def configurable(rich: bool, label: str = 'Parameter') -> bytes:
    """Emit exactly two direct strings with no hidden tag; e.g. configurable(True, 'Specification')."""
    return ansi_string(label)+short_wide_name('Ω '+label) if rich else bytes(2)


def line(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, style: int) -> bytes:
    """Keep complete line and temporary-axis identities; e.g. line(mapping, 19000, ids, True, 1)."""
    if not rich:
        return bytes(2)
    return geometric_pointer(mapping, version, identities, 'moTempAxisRef_w' if style else 'moLineRef_w')


def display(mapping: WireMap, version: int, identities: dict[str, int], rich: bool) -> bytes:
    """Reuse one complete radial display object at four tags; e.g. display(mapping, 19000, ids, True)."""
    return dimension(mapping, version, identities, 'moDisplayRadialDim_c') if rich else bytes(2)


def compatibility(rich: bool, edited: bool) -> bytes:
    """Retain temporary values even when native loading discards them; e.g. compatibility(True, True)."""
    raw = struct.pack('<QQii', 0x8000000000000000 if rich else 0,
        0x7ff8000000001234 if rich else 0, 2147483647 if rich else 0, -2147483648 if rich else 0)
    if not rich:
        return raw+bytes(4)
    name = 'Bessemer compatibility Ω' if edited else 'compatibility Ω'
    return raw+short_wide_name('stored Ω 820')+short_wide_name(name)+configurable(True, 'Temporary')


def connection_suffix(mapping: WireMap, version: int, identities: dict[str, int], rich: bool,
                      style: int, edited: bool) -> bytes:
    """Construct the native field order without member-offset sorting; e.g. connection_suffix(mapping, 19000, ids, True, 1, False)."""
    raw = line(mapping, version, identities, rich, style)+struct.pack('<i3d', -1 if rich else 0, -0.0, 1 if rich else 0, 2 if rich else 0)
    raw += display(mapping, version, identities, rich)+struct.pack('<i', -2147483648 if rich else 0)
    raw += b''.join(configurable(rich, name) for name in PARAMETERS)+compatibility(rich, edited)
    raw += display(mapping, version, identities, rich)+display(mapping, version, identities, rich)
    raw += struct.pack('<i3d', 2147483647 if rich else 0, 3 if rich else 0, 4 if rich else 0, 5 if rich else 0)
    raw += line(mapping, version, identities, rich, style)+struct.pack('<i', -1 if rich else 0)
    raw += display(mapping, version, identities, rich)+struct.pack('<i', -2147483648 if rich else 0)
    return raw+(short_wide_name('stored Ω 828') if rich else b'\0')


def feature_object(name: str, version: int, rich: bool, style: int = 0,
                   first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Build the complete inherited sketch-point body; e.g. feature_object(NAMES[0], 19000, True, 1, 96)."""
    if name not in NAMES or style not in (0, 1) or first_slot < 0:
        raise ValueError(f'connection fixture {(name, style, first_slot)}; expected known class/style and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot;identities: dict[str, int] = {}
    def contents() -> bytes:
        raw = geometry_header(mapping, version, rich)+projection_point(mapping, version, identities, rich, style)
        if name == NAMES[0]:
            return raw+connection_suffix(mapping, version, identities, rich, style, edited)
        return raw+line(mapping, version, identities, rich, style)+struct.pack('<3d', -0.0, 1 if rich else 0, 2 if rich else 0)
    raw = mapping.new(name, contents)
    return raw, mapping.slot+1


def radial_body(version: int, flags: int) -> bytes:
    """Use the geometric base and a full signed suffix word; e.g. radial_body(19000, 30)."""
    return strip_tag(geometric_dimension('ThreeDRadiusDim_c', version, flags))+struct.pack('<i', -2147483648)


def helper_object(kind: int, version: int, rich: bool = False, flags: int = 0) -> tuple[bytes, int]:
    """Exercise tagged parameters, radial bodies and their existing handle caller; e.g. helper_object(2, 19000, flags=30)."""
    if kind not in (0, 1, 2) or flags not in (0, 2, 4, 8, 16, 30):
        raise ValueError(f'connection helper {(kind, flags)}; expected kind 0..2 and known geometric flags')
    mapping = WireMap()
    def radial() -> bytes:
        return mapping.new('ThreeDRadialOrDiameterDim_c', lambda: radial_body(version, flags))
    if kind == 0:
        raw = mapping.new('moConfigurableParam_c', lambda: configurable(rich))
    elif kind == 1:
        raw = radial()
    else:
        raw = mapping.new('moFeatureDimHandle_c', lambda: handle_base(version, False, 0)+radial())
    return raw, mapping.slot+1
