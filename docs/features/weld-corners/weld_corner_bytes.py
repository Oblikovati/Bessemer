"""Complete weld corners and their nested trimmed-body/tool data (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import shared_object
from weldment_fixture_bytes import weldment_body

CLASSES = ('moWeldCornerFeat_c', 'moWeldBodyData_c', 'moWeldTrimmedBodyData_c', 'moWeldTrimToolData_c')
VERSIONS = (0x27cc, 0x334d, 0x334e, 19000, 0xffffffff)
COUNTS = ((0,0,0), (2,0,1), (0,2,3), (3,1,0), (1,3,2), (2,2,2))


def counted(items: list[bytes], escaped: bool) -> bytes:
    """Keep each direct array independent without allocating list map slots; e.g. counted([bytes(2)], True)."""
    count = struct.pack('<HI', 65535, len(items)) if escaped else struct.pack('<H', len(items))
    return count+b''.join(items)


def weldment_pointer(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Use a complete shared generic feature target rather than a loaded-object stub; e.g. weldment_pointer(mapping, ids, 19000, 2)."""
    if style == 0:
        return bytes(2)
    return shared_object(mapping, ids, 'weldment', 'moWeldmentFeature_c', lambda: weldment_body(mapping, version, 2, False))


def data_body(mapping: WireMap, ids: dict[str, int], version: int, kind: int, style: int) -> bytes:
    """Preserve complete base words, direct dwords and raw tool scalars; e.g. data_body(mapping, ids, 19000, 2, 1)."""
    count = (0,2,1,3,0,2)[style]
    items = [weldment_pointer(mapping, ids, version, style) if index != 1 else bytes(2) for index in range(count)]
    raw = counted(items, style != 0)+struct.pack('<2i', -2147483648, 2147483647)
    if kind == 2:
        count = (0,3,1,2,0,3)[style]
        words = (0xffffffff, 0, 0x80000000)[:count]
        raw += counted([struct.pack('<I', value) for value in words], style != 0)
    if kind == 3:
        raw += struct.pack('<2Q', 0x8000000000000000, 0x7ff8000000001234)
    return raw


def body_pointer(mapping: WireMap, ids: dict[str, int], version: int, kind: int, style: int, key: str) -> bytes:
    """Reserve each data identity before its own complete nested references; e.g. body_pointer(mapping, ids, 19000, 2, 1, 'trimmed')."""
    return shared_object(mapping, ids, key, CLASSES[kind], lambda: data_body(mapping, ids, version, kind, style))


def corner_array(mapping: WireMap, ids: dict[str, int], version: int, kind: int, count: int, style: int) -> bytes:
    """Keep empty and differently sized feature arrays with repeated/null data pointers; e.g. corner_array(mapping, ids, 19000, 2, 3, 1)."""
    items = [body_pointer(mapping, ids, version, kind, style, CLASSES[kind]) if index != 1 else bytes(2) for index in range(count)]
    return counted(items, style != 0)


def corner_payload(mapping: WireMap, ids: dict[str, int], version: int, style: int, edited: bool) -> bytes:
    """Keep the generic pointer between its two independent array boundaries; e.g. corner_payload(mapping, ids, 0x334e, 2, False)."""
    counts = COUNTS[style]
    raw = corner_array(mapping, ids, version, 2, counts[0], style)+corner_array(mapping, ids, version, 3, counts[1], style)
    raw += struct.pack('<i', 12345 if edited else -2147483648)+weldment_pointer(mapping, ids, version, style)
    raw += corner_array(mapping, ids, version, 1, counts[2], style)+struct.pack('<3i', -1, 0, 2147483647)
    raw += shared_object(mapping, ids, 'point', 'moPointRef_w', lambda: strip_tag(geometric('moPointRef_w', version, False, 0))) if style else bytes(2)
    return raw+(struct.pack('<i', -2147483648) if version > 0x334d else b'')


def weld_corner_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Retain distinct equal records and shared references through complete containing roots; e.g. weld_corner_object(0, 19000, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents(key: str) -> bytes:
        if kind != 0:
            return data_body(mapping, ids, version, kind, style)
        owner = mapping.slot
        return body(mapping, version, style >= 3, owner)+corner_payload(mapping, ids, version, style, edited and key == 'first')
    def record(key: str) -> bytes:
        return shared_object(mapping, ids, key, CLASSES[kind], lambda: contents(key))
    def array() -> bytes:
        return struct.pack('<H', 3)+record('first')+record('second')+record('first')
    raw = mapping.new('suObArray', array) if style in (2, 5) else record('first')
    return raw, mapping.slot+1
