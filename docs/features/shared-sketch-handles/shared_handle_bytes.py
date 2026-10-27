"""Independent shared-handle identities using the existing complete entity-handle grammar (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_component_references import base as component_base
from sketch_reference_fixture_bytes import handle
from display_dimension_fixture_bytes import body as strip_tag

CLASSES = ('sg3DParametricSurfaceHandle', 'sgParabolaHandle')
INDICES = ((0, False), (0x777e, False), (0x777f, True), (0x7780, False),
           (65535, False), (-2147483648, True), (2147483647, True), (7, True))


def handle_body(name: str, index: int, escaped: bool, edited: bool) -> bytes:
    """Retain even a noncanonical escaped small index; e.g. handle_body(CLASSES[0], 7, True, False)."""
    raw = strip_tag(handle(name, index, 12345 if edited else -2147483648))
    if escaped and 0 <= index <= 65535 and index != 0x777f:
        return struct.pack('<Hi', 0x777f, index)+raw[2:]
    return raw


def shared_handle_object(kind: int, version: int, index_case: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Use fresh/cached typed owners without extra handle fields or allocations; e.g. shared_handle_object(1, 19000, 7, 2)."""
    name = CLASSES[kind]
    return named_handle_object(name, version, index_case, style, edited, first_slot)


def named_handle_object(name: str, version: int, index_case: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Share the complete owned handle grammar across native-bound aliases; e.g. named_handle_object('sgConicHandle', 19000, 7, 2)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    index, escaped = INDICES[index_case]
    def pointer() -> bytes:
        return shared_object(mapping, ids, 'entity-handle', name, lambda: handle_body(name, index, escaped, edited))
    def component() -> bytes:
        return component_base(version, False)+struct.pack('<iI', -2147483648, 0xffffffff)+pointer()+struct.pack('<I', 0x89abcdef)
    def array() -> bytes:
        first = pointer()
        return struct.pack('<H', 3)+first+bytes(2)+pointer()
    raw = pointer() if style == 0 else (mapping.new('suObArray', array) if style == 1 else mapping.new('moCompSketchEntHandle_c', component))
    return raw, mapping.slot+1
