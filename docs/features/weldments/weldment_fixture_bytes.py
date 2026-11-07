"""Complete weldment model-feature bases and direct/tagged property identities (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_environment import feature
from freeze_custom_properties import property_bytes, property_record
from sweep_fixture_bytes import shared_object

CLASSES = ('moWeldmentFeature_c', 'moCustomPropertyList', 'moCustomProperty', 'suObArray')
EXPRESSION = 'A@B\nC\r\n材料'


def custom_record(style: int, edited: bool = False) -> bytes:
    """Reuse the existing complete field builder without adding tags; e.g. custom_record(2)."""
    if style in (0, 3):
        return bytes(24)
    return property_record('Bessemer Ω' if edited else 'Density@\n材料', EXPRESSION)


def custom_list(style: int, edited: bool = False) -> bytes:
    """Keep direct duplicate records separate without consuming map slots; e.g. custom_list(2)."""
    if style == 0:
        return bytes(4)
    if style == 1:
        return property_bytes('Bessemer Ω' if edited else 'original')
    if style == 3:
        return struct.pack('<i', 1)+bytes(24)
    first = custom_record(style, edited)
    return struct.pack('<i', 3)+first+custom_record(style)*2


def property_graph(mapping: WireMap, ids: dict[str, int], style: int, edited: bool) -> bytes:
    """Mix tagged lists/records with their cached aliases; e.g. property_graph(mapping, ids, 2, False)."""
    listing = shared_object(mapping, ids, 'custom-list', CLASSES[1], lambda: custom_list(style, edited))
    raw = struct.pack('<H', 4)+listing+shared_object(mapping, ids, 'custom-list', CLASSES[1], lambda: custom_list(style, edited))
    record = shared_object(mapping, ids, 'custom-record', CLASSES[2], lambda: custom_record(style, edited))
    return raw+record+shared_object(mapping, ids, 'custom-record', CLASSES[2], lambda: custom_record(style, edited))


def weldment_body(mapping: WireMap, version: int, style: int, edited: bool) -> bytes:
    """Stop after the model-feature base, not the atom/body feature layers; e.g. weldment_body(mapping, 19000, 2, False)."""
    mapping.slot += 2
    prefix = feature(version, style != 0)+struct.pack('<i', -2147483648)
    word = (0, 1, -2147483648, 2147483647)[style]
    return prefix+custom_list(style, edited)+struct.pack('<i', word)


def weldment_object(kind: int, version: int, style: int, first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Compose complete features and property objects in either map context; e.g. weldment_object(0, 19000, 2, 96)."""
    if kind not in range(4) or style not in range(4) or first_slot < 0:
        raise ValueError(f'weldment fixture {(kind, style, first_slot)}; expected kind/style 0..3 and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    readers = (lambda: weldment_body(mapping, version, style, edited), lambda: custom_list(style, edited),
               lambda: custom_record(style, edited), lambda: property_graph(mapping, ids, style, edited))
    raw = mapping.new(CLASSES[kind], readers[kind])
    return raw, mapping.slot+1
