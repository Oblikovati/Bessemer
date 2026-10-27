"""Complete native-bound parabola/cylinder backup graphs and their shared handles (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from sketch_reference_fixture_bytes import point
from freeze_model_references import wrapper
from display_dimension_fixture_bytes import body as strip_tag
from shared_handle_bytes import named_handle_object

CLASSES = ('moParabolaBackedUpData_c', 'mo3DCylinderBackedUpData_c', 'sgConicHandle', 'sg3DCylinderHandle')


def backup_body(mapping: WireMap, ids: dict[str, int], kind: int, rich: bool, edited: bool) -> bytes:
    """Preserve the segment prefix and all six point occurrences; e.g. backup_body(mapping, ids, 0, True, False)."""
    if kind == 1:
        raw = struct.pack('<6Q', 1 << 63, 0x7ff8000000001234, 1, 2, 3, 4)
        return raw+(struct.pack('<d', 123.5) if edited else struct.pack('<Q', 0xfff0000000000000))
    def pointer(key: str) -> bytes:
        return shared_object(mapping, ids, key, 'moPointBackedUpData_c', lambda: strip_tag(point())) if rich else bytes(2)
    raw = pointer('first-point')+pointer('first-point')+struct.pack('<2i', -2147483648, 2147483647)
    raw += pointer('second-point')+bytes(2)+pointer('first-point')+pointer('second-point')
    return raw+(struct.pack('<d', 123.5) if edited else struct.pack('<Q', 1 << 63))+struct.pack('<Q', 0x7ff8000000001234)


def backup_object(kind: int, version: int, style: int, mode: int = 0, edited: bool = False, first_slot: int = 0, index_case: int = 0) -> tuple[bytes, int]:
    """Exercise standalone, aliased and complete declared-role owners; e.g. backup_object(0, 19000, 2, -1)."""
    if kind >= 2:
        return named_handle_object(CLASSES[kind], version, index_case, style, edited, first_slot)
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def pointer() -> bytes:
        return shared_object(mapping, ids, 'backup', CLASSES[kind], lambda: backup_body(mapping, ids, kind, style != 0, edited))
    def array() -> bytes:
        first = pointer()
        return struct.pack('<H', 3)+first+bytes(2)+pointer()
    def reference() -> bytes:
        return wrapper(version, False)+struct.pack('<h', mode)+pointer()
    raw = pointer() if style == 0 else (mapping.new('suObArray', array) if style == 1 else mapping.new('moSketchExtRef_w', reference))
    return raw, mapping.slot+1
