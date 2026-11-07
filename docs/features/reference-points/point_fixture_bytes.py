"""Independent reference-point roots, empty bases and complete data variants (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from axis_owner_bytes import geometry_header
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from profile_fixture_bytes import contents as profile_contents
from reference_plane_fixture_bytes import plane_reference as plane_body
from freeze_component_references import base as component_base
from sweep_fixture_bytes import shared_object, dimension, endpoint, synchronization

KINDS = ('moRefPointData_c', 'moFaceCenterRefPointData_c', 'moCenterEdgeRefPointData_c',
         'moFaceEdgePointData_c', 'moFaceVertexProjectionPointData_c',
         'moTwoEdgeRefPointData_c', 'moAlongCurveRefPointData_c')
CURVES = ('moGeneralCurveRef_w', 'moCircleRef_w', 'moLineRef_w', 'moTempAxisRef_w')


def geometric_pointer(mapping: WireMap, version: int, identities: dict[str, int], name: str) -> bytes:
    """Preserve actual curve/wrapper classes and shared identity; e.g. geometric_pointer(mapping, 19000, ids, CURVES[2])."""
    source = 'moLineRef_w' if name == 'moTempAxisRef_w' else name
    def contents() -> bytes:
        if name == 'moPlaneRef_w':
            return plane_body(version)
        raw = strip_tag(geometric(source, version, False, 0))
        if name == 'moTempAxisRef_w' and version > 0x371d:
            raw += struct.pack('<6di', 1, 2, 3, -0.0, 5, 6, -2147483648)
        return raw
    return shared_object(mapping, identities, name, name,
        contents)


def data_body(mapping: WireMap, version: int, identities: dict[str, int], kind: int,
              rich: bool, style: int, point_style: int, edited: bool) -> bytes:
    """Keep direct base absence and native derived field order; e.g. data_body(mapping, 19000, ids, 6, True, 2, 0, False)."""
    def pointer(name: str) -> bytes:
        return geometric_pointer(mapping, version, identities, name) if rich else bytes(2)
    curve = CURVES[style]
    if kind == 0:
        return b''
    if kind == 1:
        return pointer('moSingleFaceRef_w')
    if kind == 2:
        return pointer(curve)
    if kind == 3:
        return pointer(curve)+struct.pack('<i', -2147483648)
    if kind == 4:
        return pointer('moSingleFaceRef_w')+projection_point(mapping, version, identities, rich, point_style)+pointer('moPlaneRef_w')
    if kind == 5:
        return pointer(curve)+pointer(curve)+struct.pack('<ii', -2147483648, 2147483647)
    raw = pointer(curve)+struct.pack('<2d2i', -0.0, 1.25, -2147483648, 2147483647)
    raw += dimension(mapping, version, identities, 'moDisplayDistanceDim_c') if rich else bytes(2)
    return raw+struct.pack('<2i', -1, 12345 if edited else -2147483648)


def projection_point(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, style: int) -> bytes:
    """Cover point, endpoint and synchronization inheritance; e.g. projection_point(mapping, 19000, ids, True, 1)."""
    if not rich:
        return bytes(2)
    if style == 1:
        return endpoint(mapping, version, identities, True)
    if style == 2:
        return synchronization(mapping, version, identities, True)
    return geometric_pointer(mapping, version, identities, 'moPointRef_w')


def point_record(mapping: WireMap, version: int, kind: int, rich: bool, style: int,
                 point_style: int, edited: bool) -> bytes:
    """Compose the full geometry base before three real tags; e.g. point_record(mapping, 19000, 6, True, 2, 0, False)."""
    identities: dict[str, int] = {}
    def contents() -> bytes:
        raw = geometry_header(mapping, version, rich)
        raw += mapping.new(KINDS[kind], lambda: data_body(mapping, version, identities, kind, rich, style, point_style, edited))
        raw += shared_object(mapping, identities, 'point-profile-feature', 'moProfileFeature_c',
            lambda: profile_contents(mapping, version, False, 0, False, mapping.slot)) if rich else bytes(2)
        return raw+(geometric_pointer(mapping, version, identities, 'moSingleFaceRef_w') if rich else bytes(2))
    return mapping.new('moRefPoint_c', contents)


def point_object(version: int, kind: int, rich: bool, standalone: bool = False,
                 style: int = 0, point_style: int = 0, first_slot: int = 0,
                 edited: bool = False, component: bool = False) -> tuple[bytes, int]:
    """Build roots, data helpers and real component geometry callers; e.g. point_object(19000, 6, True, style=2, first_slot=96)."""
    if kind not in range(7) or style not in range(4) or point_style not in range(3) or first_slot < 0 or (standalone and component):
        raise ValueError(f'point fixture {(kind, style, point_style, first_slot, standalone, component)}; expected known variants, nonnegative slots and distinct root modes')
    mapping = WireMap();mapping.slot = first_slot;identities: dict[str, int] = {}
    if standalone:
        raw = mapping.new(KINDS[kind], lambda: data_body(mapping, version, identities, kind, rich, style, point_style, edited))
    elif component:
        raw = mapping.new('moCompRefPoint_c', lambda: component_base(version, False)
            +struct.pack('<iI', -2147483648, 0xfedcba98)+point_record(mapping, version, kind, rich, style, point_style, edited))
    else:
        raw = point_record(mapping, version, kind, rich, style, point_style, edited)
    return raw, mapping.slot+1
