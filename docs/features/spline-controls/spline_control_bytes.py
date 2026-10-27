"""Independent complete spline controls with direct handle records and real sketch owners (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from sketch_tail_fixture_bytes import ordinary

CLASSES = ('moSplineTangencyControl_c', 'moSplineControlPolygonControl_c', 'moSplineCurvatureControl_c')


def direct_handle(style: int, second: bool, edited: bool) -> bytes:
    """Vary short and escaped indices independently, without object tags; e.g. direct_handle(2, True, False)."""
    escaped = (style == 1 and not second) or (style == 2 and second) or style == 3
    index = struct.pack('<Hi', 0x777f, -2147483648 if second else 2147483647) if escaped else struct.pack('<H', 65535 if second else 32768)
    return index+struct.pack('<2i', 12345 if edited and not second else -2147483648, 2147483647)


def control_body(mapping: WireMap, ids: dict[str, int], kind: int, version: int, style: int, edited: bool) -> bytes:
    """Use the complete declared sketch target and retained raw geometry; e.g. control_body(mapping, ids, 0, 19000, 1, False)."""
    sketch = bytes(2) if style == 0 else shared_object(mapping, ids, 'sketch', 'sgSketch', lambda: ordinary(mapping, version, False))
    raw = sketch+direct_handle(style, False, edited)
    if kind == 1:
        return raw
    raw += direct_handle(style, True, edited)
    geometry = struct.pack('<10Q', 1 << 63, 0x7ff8000000001234, 1, 2, 3, 4, 5, 6, 7, 0xfff0000000000000)
    return raw+geometry[:48 if kind == 0 else 80]


def control_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep actual self/cached sketch and control identities; e.g. control_object(2, 19000, 3)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def control() -> bytes:
        return shared_object(mapping, ids, 'control', CLASSES[kind], lambda: control_body(mapping, ids, kind, version, style, edited))
    def sketch() -> bytes:
        def users() -> bytes:
            first = control()
            return struct.pack('<i', 3)+first+bytes(2)+control()
        return shared_object(mapping, ids, 'sketch', 'sgSketch', lambda: ordinary(mapping, version, False, user_objects=users))
    def array() -> bytes:
        first = shared_object(mapping, ids, 'sketch', 'sgSketch', lambda: ordinary(mapping, version, False))
        second = control()
        return struct.pack('<H', 3)+first+second+control()
    raw = sketch() if style == 3 else (mapping.new('suObArray', array) if style == 2 else control())
    return raw, mapping.slot+1
