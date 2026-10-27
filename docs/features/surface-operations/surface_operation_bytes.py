"""Independent complete surface-cut/trim records with direct chooser ownership (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from display_dimension_fixture_bytes import body as strip_tag
from freeze_boundary_references import wire as boundary
from reference_plane_fixture_bytes import plane_reference
from solid_reference_bytes import solid_pointer
from body_feature_fixture_bytes import direction
from profile_fixture_bytes import contents as profile_contents
from sweep_fixture_bytes import shared_object
from weld_corner_bytes import counted

CLASSES = ('moSurfCut_c','moTrimRefSurface_c')
VERSIONS = (0x27cc,0x2b91,0x2b92,0x332c,0x332d,19000,0xffffffff)
COUNTS = ((0,0,0,0,0),(1,0,1,0,1),(0,3,2,3,0),(3,1,0,2,1),(2,2,3,1,2),(1,3,1,2,3))


def reference(mapping: WireMap, ids: dict[str, int], version: int, style: int, solid: bool) -> bytes:
    """Retain absent, suppressed and serialized selected bodies through complete reference aliases; e.g. reference(mapping, ids, 19000, 3, True)."""
    name = 'moSolidRef_w' if solid else 'moConstSurfRef_w'
    presence = (0,0,-2147483648,-1,2147483647,1)[style]
    return solid_pointer(mapping, ids, version, min(style,2), presence, 2 if style in (3,5) else 1, name, False, name)


def face(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Use a complete face reference with its direct entity-ID pairs; e.g. face(mapping, ids, 19000)."""
    return shared_object(mapping, ids, 'face', 'moFaceRef_c', lambda: strip_tag(boundary('moFaceRef_c', version, False)))


def array(mapping: WireMap, ids: dict[str, int], version: int, style: int, count: int) -> bytes:
    """Keep independently sized direct arrays, null entries and shared targets; e.g. array(mapping, ids, 19000, 2, 3)."""
    items = [face(mapping, ids, version) if index != 1 else bytes(2) for index in range(count)]
    return counted(items, style != 0)


def chooser(mapping: WireMap, ids: dict[str, int], version: int, style: int, owner: int, count: int) -> bytes:
    """Write the direct chooser body without an optional presence or outer object tag; e.g. chooser(mapping, ids, 19000, 2, 4, 3)."""
    return array(mapping, ids, version, style, count)+struct.pack('<H', owner if style >= 2 else 0)


def cut_payload(mapping: WireMap, ids: dict[str, int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Keep atom, face, constant-surface and plane roles in the native order; e.g. cut_payload(mapping, ids, 19000, 2, 4, False)."""
    raw = struct.pack('<H2i', owner if style else 0, 12345 if edited else -2147483648,2147483647)
    raw += face(mapping, ids, version) if style else bytes(2)
    raw += reference(mapping, ids, version, style, False) if style else bytes(2)
    raw += shared_object(mapping, ids, 'plane', 'moPlaneRef_w', lambda: plane_reference(version)) if style else bytes(2)
    first,second,third,_,_ = COUNTS[style]
    raw += array(mapping, ids, version, style, first)+array(mapping, ids, version, style, second)
    return raw+chooser(mapping, ids, version, style, owner, third)


def profile(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Use the complete ordinary profile with suppressed geometry and no reused source; e.g. profile(mapping, ids, 19000)."""
    return shared_object(mapping, ids, 'profile', 'moProfileFeature_c', lambda: profile_contents(mapping, version, False, 0, False, mapping.slot))


def trim_payload(mapping: WireMap, ids: dict[str, int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Serialize allocated data members directly, omitting constructed but unwritten array +80; e.g. trim_payload(mapping, ids, 19000, 2, 4, False)."""
    first,second,third,fourth,fifth = COUNTS[style]
    raw = array(mapping, ids, version, style, first)+array(mapping, ids, version, style, second)
    raw += profile(mapping, ids, version) if style else bytes(2)
    raw += shared_object(mapping, ids, 'direction', 'moDirectionSpec_c', lambda: direction(mapping, True)) if style else bytes(2)
    raw += struct.pack('<I', 0xffffffff if style else 0)
    raw += chooser(mapping, ids, version, style, owner, third)
    raw += reference(mapping, ids, version, style, True) if style else bytes(2)
    raw += array(mapping, ids, version, style, fourth)+array(mapping, ids, version, style, fifth)
    return raw+struct.pack('<2i', 12345 if edited else -2147483648,2147483647)


def surface_operation_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep equal features distinct, cached shared targets and self-owned direct choosers; e.g. surface_operation_object(0, 19000, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents(key: str) -> bytes:
        owner = mapping.slot
        prefix = body(mapping, version, style >= 3, owner)
        payload = cut_payload if kind == 0 else trim_payload
        return prefix+payload(mapping, ids, version, style, owner, edited and key == 'first')
    def record(key: str) -> bytes:
        return shared_object(mapping, ids, key, CLASSES[kind], lambda: contents(key))
    def outer() -> bytes:
        return struct.pack('<H', 3)+record('first')+record('second')+record('first')
    raw = mapping.new('suObArray', outer) if style in (2,5) else record('first')
    return raw, mapping.slot+1
