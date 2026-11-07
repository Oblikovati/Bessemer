"""Complete face-blend items and pairs with independent cached identities (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_boundary_references import wire as boundary_wire
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import shared_object

CLASSES = ('moFaceBlendItem_c', 'moFaceBlendItemPair_c')
PRESENCES = (0, 1, 128, 255)
VERSIONS = (0x27cc, 0x332c, 0x332d, 19000, 0xffffffff)


def boundary_pointer(mapping: WireMap, ids: dict[str, int], version: int, name: str) -> bytes:
    """Reuse complete boundary references without changing their identity; e.g. boundary_pointer(mapping, ids, 19000, 'moFaceRef_c')."""
    return shared_object(mapping, ids, name, name, lambda: strip_tag(boundary_wire(name, version, False)))


def item_pointer(mapping: WireMap, ids: dict[str, int], version: int, style: int, presence: int, key: str, edited: bool) -> bytes:
    """Preserve noncanonical presence bytes and raw point bits; e.g. item_pointer(mapping, ids, 19000, 1, 255, 'first', False)."""
    def contents() -> bytes:
        raw = b''.join(boundary_pointer(mapping, ids, version, name) if style else bytes(2)
            for name in ('moFaceRef_c', 'moEdgeRef_c'))
        raw += bytes([presence])
        if presence:
            first = struct.unpack('<Q', struct.pack('<d', 123.5))[0] if edited and key == 'first' else 0x8000000000000000
            raw += struct.pack('<3Q', first, 0x7ff8000000001234, 0x7ff0000000000000)
        return raw
    return shared_object(mapping, ids, key, CLASSES[0], contents)


def blend_item_object(kind: int, version: int, style: int, presence: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep equal-but-distinct items and repeated pair pointers; e.g. blend_item_object(1, 19000, 2, 255)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def item(key: str) -> bytes:
        return item_pointer(mapping, ids, version, style, presence, key, edited)
    def pair(key: str) -> bytes:
        def contents() -> bytes:
            if style == 0:
                return bytes(4)
            return item('first')+item('second' if style == 2 else 'first')
        return shared_object(mapping, ids, key, CLASSES[1], contents)
    def array() -> bytes:
        if kind == 0:
            return struct.pack('<H', 3)+item('first')+item('second')+item('first')
        return struct.pack('<H', 3)+pair('pair-first')+pair('pair-second')+pair('pair-first')
    raw = mapping.new('suObArray', array) if style == 2 else item('first') if kind == 0 else pair('pair-first')
    return raw, mapping.slot+1
