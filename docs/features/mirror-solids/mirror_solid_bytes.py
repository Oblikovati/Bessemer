"""Independent complete mirrored solids, matrix helpers and unsigned arrays (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from freeze_component_references import wire as component
from reference_plane_fixture_bytes import plane_reference
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import shared_object
from weld_corner_bytes import counted

CLASSES = ('moMirrorSolid_c', 'moMatrixIndexHelper_c', 'suUIntArray')
VERSIONS = (0x27cc, 0x2f86, 0x2f87, 0x2f93, 0x2f94, 0x3745, 0x3746, 0x3760, 0x3761, 19000, 0xffffffff)


def unsigned_body(style: int) -> bytes:
    """Retain fixed signed count/growth and unsigned high bits; e.g. unsigned_body(2)."""
    words = ((), (0xffffffff,), (0xffffffff,0,0x80000000), (0x80000000,0x80000000), (0,), (0xffffffff,0,0xffffffff))[style]
    return struct.pack('<2i', len(words), (0,-1,-2147483648,2147483647,-7,17)[style])+b''.join(struct.pack('<I', word) for word in words)


def unsigned_pointer(mapping: WireMap, ids: dict[str, int], style: int) -> bytes:
    """Share complete unsigned arrays across separately framed helper members; e.g. unsigned_pointer(mapping, ids, 2)."""
    return shared_object(mapping, ids, 'unsigned', 'suUIntArray', lambda: unsigned_body(style))


def matrix_body(mapping: WireMap, ids: dict[str, int], style: int) -> bytes:
    """Keep different direct array sizes and shared unsigned-array identities; e.g. matrix_body(mapping, ids, 2)."""
    raw = b''
    for count in ((0,0),(1,2),(3,1),(0,3),(2,0),(2,2))[style]:
        items = [unsigned_pointer(mapping, ids, style) if index != 1 else bytes(2) for index in range(count)]
        raw += counted(items, style != 0)
    return raw+struct.pack('<i', -2147483648 if style else 0)


def surface_pointer(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Use one complete shared surface wrapper, with no invented derived suffix; e.g. surface_pointer(mapping, ids, 19000, 2)."""
    if style == 0:
        return bytes(2)
    return shared_object(mapping, ids, 'surface', 'moPlaneRef_w', lambda: plane_reference(version))


def plane(style: int, second: bool) -> bytes:
    """Separate both full-width plane selectors and raw floating bits; e.g. plane(2, True)."""
    presence = (0,-1,1,-2147483648,2147483647,0)[(style+int(second))%6] if style else 0
    return struct.pack('<i', presence)+(struct.pack('<6Q', 0x8000000000000000,0x7ff8000000001234,0x3ff0000000000000,0,0,0x3ff0000000000000) if presence else b'')


def payload(mapping: WireMap, ids: dict[str, int], version: int, style: int, edited: bool) -> bytes:
    """Emit direct members in native order before gated fields; e.g. payload(mapping, ids, 19000, 2, False)."""
    face = shared_object(mapping, ids, 'face', 'moCompFace_c', lambda: strip_tag(component('moCompFace_c', version, False, 0))) if style else bytes(2)
    raw = face+struct.pack('<i', 12345 if edited else -2147483648)+plane(style, False)
    for count in ((0,0),(1,0),(0,3),(3,1),(2,2),(1,3))[style]:
        raw += counted([struct.pack('<H', ids['face']) if index != 1 else bytes(2) for index in range(count)], style != 0)
    raw += struct.pack('<i', 2147483647)+surface_pointer(mapping, ids, version, style)
    raw += shared_object(mapping, ids, 'matrix', CLASSES[1], lambda: matrix_body(mapping, ids, style)) if style else bytes(2)
    raw += struct.pack('<Bi3B', 255,-1,254,128,2)
    return raw+continuation(mapping, ids, version, style)


def continuation(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Preserve strict greater-than gates and fixed-width signed vectors; e.g. continuation(mapping, ids, 0x3761, 2)."""
    raw = b''
    if version > 0x2f86:
        raw += counted([unsigned_pointer(mapping, ids, style)] if style else [], style != 0)
    if version > 0x2f93:
        words = (-1,-2147483648,-1) if style else ()
        raw += struct.pack('<I', len(words))+b''.join(struct.pack('<i', word) for word in words)
    if version > 0x3745:
        raw += b'\xfb'+surface_pointer(mapping, ids, version, style)
    if version > 0x3760:
        raw += plane(style, True)
    return raw


def mirror_solid_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Build distinct equal roots with shared nested targets and repeated first identity; e.g. mirror_solid_object(0, 19000, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents(key: str) -> bytes:
        if kind == 2:
            return unsigned_body(style)
        if kind == 1:
            return matrix_body(mapping, ids, style)
        owner = mapping.slot
        return body(mapping, version, style >= 3, owner)+payload(mapping, ids, version, style, edited and key == 'first')
    def record(key: str) -> bytes:
        return shared_object(mapping, ids, key, CLASSES[kind], lambda: contents(key))
    def array() -> bytes:
        return struct.pack('<H', 3)+record('first')+record('second')+record('first')
    raw = mapping.new('suObArray', array) if style in (2,5) else record('first')
    return raw, mapping.slot+1
