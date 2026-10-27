"""Independent complete combine/delete features and their nested body payloads (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from freeze_component_references import wire as component
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import shared_object
from solid_reference_bytes import solid_pointer
from weld_corner_bytes import counted

CLASSES = ('moCombineBodies_c', 'moDeleteBody_c', 'moDeleteBodyData_c')
VERSIONS = (0x27cc,0x3f07,0x3f08,19000,0xffffffff)


def body_component(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Use complete shared solid/surface components without guessing item restrictions; e.g. body_component(mapping, ids, 19000, 2)."""
    name = 'moCompSurfaceBody_c' if style % 2 else 'moCompSolidBody_c'
    return shared_object(mapping, ids, name, name, lambda: strip_tag(component(name, version, False, 0)))


def body_array(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Keep direct counts, nulls, repeated targets and full absent-body solid references; e.g. body_array(mapping, ids, 19000, 2)."""
    items: list[bytes] = []
    for index in range((0,1,3,2,4,3)[style]):
        if index == 1:
            items.append(bytes(2));continue
        if style >= 4:
            items.append(solid_pointer(mapping, ids, version, 2, 0, 0, 'solid-reference', False));continue
        items.append(body_component(mapping, ids, version, style))
    return counted(items, style != 0)


def delete_payload(mapping: WireMap, ids: dict[str, int], version: int, style: int, edited: bool) -> bytes:
    """Preserve the six-byte empty direct payload rather than its runtime allocation; e.g. delete_payload(mapping, ids, 19000, 2, False)."""
    return body_array(mapping, ids, version, style)+struct.pack('<i', 12345 if edited else -2147483648)


def feature_body(mapping: WireMap, ids: dict[str, int], version: int, kind: int, style: int, edited: bool) -> bytes:
    """Keep inherited owner identity before allocating nested component records; e.g. feature_body(mapping, ids, 19000, 0, 2, False)."""
    owner = mapping.slot
    raw = body(mapping, version, style >= 3, owner)
    if kind == 1:
        return raw+(shared_object(mapping, ids, 'delete-data', CLASSES[2], lambda: delete_payload(mapping, ids, version, style, False)) if style else bytes(2))
    raw += shared_object(mapping, ids, 'solid-body', 'moCompSolidBody_c', lambda: strip_tag(component('moCompSolidBody_c', version, False, 0))) if style else bytes(2)
    raw += body_array(mapping, ids, version, style)+struct.pack('<i', 12345 if edited else -2147483648)
    return raw+(bytes([(0,1,2,128,254,255)[style]]) if version > 0x3f07 else b'')


def combine_body_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep equal roots distinct, repeated root aliases and shared nested objects; e.g. combine_body_object(0, 19000, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents(key: str) -> bytes:
        change = edited and key == 'first'
        return delete_payload(mapping, ids, version, style, change) if kind == 2 else feature_body(mapping, ids, version, kind, style, change)
    def record(key: str) -> bytes:
        return shared_object(mapping, ids, key, CLASSES[kind], lambda: contents(key))
    def array() -> bytes:
        return struct.pack('<H', 3)+record('first')+record('second')+record('first')
    raw = mapping.new('suObArray', array) if style in (2,5) else record('first')
    return raw, mapping.slot+1
