"""Complete solid references with independent signed presence and strict kernel bodies (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_model_references import wrapper
from freeze_component_references import wire as component_wire
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import shared_object
from kernel_geometry_fixture import frame

VERSIONS = (0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
BRANCHES = ((0,0),)+tuple((presence, mode) for presence in (-2147483648,-1,1,2147483647) for mode in (1,2))


def component_target(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Share a complete supported solid-body component across distinct references; e.g. component_target(mapping, ids, 19000, 1)."""
    if style == 0:
        return bytes(2)
    return shared_object(mapping, ids, 'solid-component', 'moCompSolidBody_c', lambda: strip_tag(component_wire('moCompSolidBody_c', version, False, 0)))


def body_attributes(mapping: WireMap, ids: dict[str, int]) -> bytes:
    """Retain an independent direct attribute list after either selected body state; e.g. body_attributes(mapping, ids)."""
    first = shared_object(mapping, ids, 'attribute', 'moAttribute_c', lambda: bytes(137)+struct.pack('<4Hi', 0, 0, 0, 0, -123))
    return struct.pack('<HI', 65535, 3)+first+bytes(2)+struct.pack('<H', ids['attribute'])


def solid_pointer(mapping: WireMap, ids: dict[str, int], version: int, style: int, presence: int, mode: int, key: str, edited: bool, name: str = 'moSolidRef_w') -> bytes:
    """Keep signed presence independent of suppression and preserve both attribute occurrences; e.g. solid_pointer(mapping, ids, 19000, 2, -1, 1, 'first', False)."""
    def contents() -> bytes:
        raw = component_target(mapping, ids, version, style)+wrapper(version, False)[2:]
        word = 0x12345678 if edited and key == 'first' else (0, 0xffffffff, 0x80000000)[style]
        raw += struct.pack('<Ii', word, presence)
        if presence:
            raw += (b'\xff' if mode == 1 else frame())+body_attributes(mapping, ids)
        return raw
    return shared_object(mapping, ids, key, name, contents)


def solid_object(version: int, style: int, presence: int, mode: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep equal references distinct while sharing nested components/attributes; e.g. solid_object(19000, 2, -1, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def pointer(key: str) -> bytes:
        return solid_pointer(mapping, ids, version, style, presence, mode, key, edited)
    def array() -> bytes:
        return struct.pack('<H', 3)+pointer('first')+pointer('second')+pointer('first')
    raw = mapping.new('suObArray', array) if style == 2 else pointer('first')
    return raw, mapping.slot+1
