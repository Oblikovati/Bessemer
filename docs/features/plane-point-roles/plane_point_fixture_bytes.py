"""Complete plane data reached through fresh or cached point-reference subclasses (#46)."""
import struct
from fillet_fixture_bytes import WireMap
from reference_plane_fixture_bytes import base, plane_reference, header, atom_fields, VECTOR
from freeze_model_references import wrapper
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import shared_object, endpoint_body, synchronization_body

PLANES = ('moFacePtRefPlnData_c', 'moLinePtRefPlnData_c', 'moPerLinePtRefPlnData_c')
POINTS = ('moPointRef_w', 'moEndPointRef_w', 'moLoftSynchRef_c', 'moRefWrapper_c')


def point_body(mapping: WireMap, version: int, ids: dict[str, int], kind: int, edited: bool = False) -> bytes:
    """Reuse only complete established class bodies; e.g. point_body(mapping, 19000, ids, 1)."""
    choices = (lambda: wrapper(version, False)+struct.pack('<3d', 1, 2, -0.0),
               lambda: endpoint_body(mapping, version, ids, False),
               lambda: synchronization_body(mapping, version, ids, False), lambda: wrapper(version, False))
    raw = bytearray(choices[kind]())
    if edited and kind < 3:
        struct.pack_into('<d', raw, len(wrapper(version, False)), 123.5)
    return bytes(raw)


def point_pointer(mapping: WireMap, version: int, ids: dict[str, int], kind: int, edited: bool = False) -> bytes:
    """Capture the point's slot before constructing nested objects; e.g. point_pointer(mapping, 19000, ids, 2)."""
    return shared_object(mapping, ids, 'plane-point', POINTS[kind], lambda: point_body(mapping, version, ids, kind, edited))


def plane_body(mapping: WireMap, version: int, ids: dict[str, int], plane: int, point: int, edited: bool = False) -> bytes:
    """Preserve the complete plane base, reference and distinct suffix; e.g. plane_body(mapping, 19000, ids, 1, 1)."""
    raw = base(version, False)+point_pointer(mapping, version, ids, point, edited)
    if plane == 0:
        raw += plane_reference(version)
    else:
        raw += shared_object(mapping, ids, 'plane-line', 'moLineRef_w',
            lambda: strip_tag(geometric('moLineRef_w', version, False, 0)))
    raw += struct.pack('<2i', -2147483648, 2147483647)
    return raw+(struct.pack('<Hi', 65535, -1) if plane == 2 else b'')


def role_object(plane: int, point: int, version: int, cached: bool) -> tuple[bytes, int]:
    """Use a tagged array to prove the same target through new and loaded tags; e.g. role_object(2, 1, 19000, True)."""
    if plane not in range(3) or point not in range(4):
        raise ValueError(f'plane/point fixture {(plane, point)}; expected plane 0..2 and point 0..3')
    mapping = WireMap();ids: dict[str, int] = {}
    def contents() -> bytes:
        first = point_pointer(mapping, version, ids, point) if cached else bytes(2)
        second = mapping.new(PLANES[plane], lambda: plane_body(mapping, version, ids, plane, point))
        return struct.pack('<H', 2)+first+second
    raw = mapping.new('suObArray', contents)
    return raw, mapping.slot+1


def plane_feature(plane: int, point: int, first_slot: int, edited: bool = False) -> tuple[bytes, int]:
    """Construct a complete owned reference plane before its selected data; e.g. plane_feature(0, 1, 96)."""
    if plane not in range(3) or point not in range(4) or first_slot < 0:
        raise ValueError(f'owned plane/point/slot {(plane, point, first_slot)}; expected plane 0..2, point 0..3 and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        owner = mapping.slot
        raw = header(mapping, 19000, False)+atom_fields(mapping, False, owner)
        raw += VECTOR+struct.pack('<IiHi', 0x89abcdef, -2147483648, 0, 2147483647)
        return raw+mapping.new(PLANES[plane], lambda: plane_body(mapping, 19000, ids, plane, point, edited))
    raw = mapping.new('moRefPlane_c', contents)
    return raw, mapping.slot+1
