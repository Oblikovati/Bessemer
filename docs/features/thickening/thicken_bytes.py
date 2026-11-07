"""Complete thickening features and native-bound reference layers (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from body_feature_fixture_bytes import direction, suffix as body_suffix
from freeze_component_references import base
from sweep_fixture_bytes import shared_object
from solid_reference_bytes import solid_pointer
from display_dimension_fixture_bytes import shared, suffix
from weld_corner_bytes import counted

CLASSES = ('moThicken_c','moConstSurfRef_w','moCompRefSurface_c')
VERSIONS = (0x27cc,0x2f73,0x2f74,0x3ef5,0x3ef6,19000,0xffffffff)
PRESENCE = (0,0,-2147483648,-1,2147483647,1)


def constant(mapping: WireMap, ids: dict[str, int], version: int, style: int, key: str) -> bytes:
    """Reuse the independently verified atom-reference grammar for this exact native class; e.g. constant(mapping, ids, 19000, 3, 'constant')."""
    mode = 2 if style in (3,5) else 1
    return solid_pointer(mapping, ids, version, min(style,2), PRESENCE[style], mode, key, False, CLASSES[1])


def component_surface(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Retain the component-feature suffix and a complete reference-geometry target; e.g. component_surface(mapping, ids, 19000, 2)."""
    raw = base(version, False)+struct.pack('<iI', -2147483648, 0xffffffff)
    raw += shared_object(mapping, ids, 'axis', 'moRefAxis_c', lambda: axis_body(mapping, version, style)) if style else bytes(2)
    return raw+struct.pack('<i', 2147483647)


def axis_body(mapping: WireMap, version: int, style: int) -> bytes:
    """Use the existing complete axis builder within the reserved identity; e.g. axis_body(mapping, 19000, 2)."""
    from axis_owner_bytes import geometry_header
    from freeze_fixtures import BASE
    return geometry_header(mapping, version, style >= 3)+mapping.new('moRefAxisData_c', lambda: BASE)


def list_body(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Keep list identity distinct from direct member counts and nested reference aliases; e.g. list_body(mapping, ids, 19000, 2)."""
    items: list[bytes] = []
    for index in range((0,1,3,2,4,3)[style]):
        if index == 1:
            items.append(bytes(2));continue
        items.append(constant(mapping, ids, version, style, 'constant'))
    return counted(items, style != 0)


def dimension_body(mapping: WireMap, version: int) -> bytes:
    """Keep the complete distance-dimension base and derived suffix in one owner context; e.g. dimension_body(mapping, 19000)."""
    owner = mapping.slot;name = 'moDisplayDistanceDim_c'
    return shared(mapping, version, name, False, owner)+suffix(mapping, version, name, False)


def feature(mapping: WireMap, ids: dict[str, int], version: int, style: int, edited: bool) -> bytes:
    """Emit native field widths and the inclusive direction boundary; e.g. feature(mapping, ids, 0x2f74, 2, False)."""
    owner = mapping.slot;raw = body(mapping, version, style >= 3, owner)
    raw += struct.pack('<Q2i', 0x7ff8000000001234 if style else 0x8000000000000000, 12345 if edited else -2147483648, 2147483647)
    raw += shared_object(mapping, ids, 'list', 'suObList', lambda: list_body(mapping, ids, version, style)) if style else bytes(2)
    raw += struct.pack('<i', -1)
    raw += shared_object(mapping, ids, 'dimension', 'moDisplayDistanceDim_c', lambda: dimension_body(mapping, version)) if style else bytes(2)
    raw += struct.pack('<2i', -2147483648, 2147483647)
    if version >= 0x2f74:
        raw += shared_object(mapping, ids, 'direction', 'moDirectionSpec_c', lambda: direction(mapping, True)) if style else bytes(2)
    return raw


def thicken_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Retain distinct equal roots and shared nested list/dimension/direction identities; e.g. thicken_object(0, 19000, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def record(key: str) -> bytes:
        if kind == 1:
            return constant(mapping, ids, version, style, key)
        return shared_object(mapping, ids, key, CLASSES[kind], lambda: feature(mapping, ids, version, style, edited and key == 'first') if kind == 0 else component_surface(mapping, ids, version, style))
    def array() -> bytes:
        if kind == 0:
            seed = mapping.new('moRib_c', lambda: body(mapping, version, False, mapping.slot)+body_suffix(mapping, version, 'moRib_c', False))
            entries = [record('first'),record('second'),record('first')] if style in (2,5) else [record('first')]
            return struct.pack('<H', len(entries)+1)+seed+b''.join(entries)
        return struct.pack('<H', 3)+record('first')+record('second')+record('first')
    raw = mapping.new('suObArray', array) if kind == 0 or style in (2,5) else record('first')
    return raw, mapping.slot+1
