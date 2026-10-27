"""Complete revolved surfaces, shared data records and exact zero-byte keepers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from axis_owner_bytes import geometry_header
from body_feature_fixture_bytes import revolution_end
from display_dimension_fixture_bytes import body as strip_tag, shared, suffix
from freeze_geometric_references import wire as geometric
from sweep_fixture_bytes import shared_object
from kernel_geometry_fixture import frame

CLASSES = ('moRevolvRefSurf_c', 'moRevolvSurfData_c', 'moRevolvRefSurfIdKeeper_c')
VERSIONS = (0x27cc, 0x2bcc, 0x2bcd, 0x2f85, 0x2f86, 19000, 0xffffffff)
WORDS = ((-2147483648, 2147483647), (0, 0), (2147483647, -2147483648))


def angular(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Retain both angular pointer occurrences even for non-two-direction modes; e.g. angular(mapping, ids, 19000)."""
    name = 'moDisplayAngularDim_c'
    return shared_object(mapping, ids, 'angle', name,
        lambda: shared(mapping, version, name, False, mapping.slot)+suffix(mapping, version, name, False))


def revolve_data(mapping: WireMap, ids: dict[str, int], version: int, style: int, words: tuple[int, int], key: str, edited: bool) -> bytes:
    """Separate signed words with their intervening dimension pointer; e.g. revolve_data(mapping, ids, 19000, 1, (0, 2), 'first', False)."""
    def contents() -> bytes:
        first = angular(mapping, ids, version) if style in (1, 2) else bytes(2)
        raw = first+struct.pack('<i', 12345 if edited and key == 'data-first' else words[0])
        raw += (angular(mapping, ids, version) if style else bytes(2))+struct.pack('<i', words[1])
        end = shared_object(mapping, ids, 'end', 'moRevEndSpec_c', lambda: revolution_end(mapping, version, style == 3)) if style else bytes(2)
        return raw+end
    return shared_object(mapping, ids, key, CLASSES[1], contents)


def axis_pointer(mapping: WireMap, ids: dict[str, int], version: int) -> bytes:
    """Keep the complete supported line wrapper under a shared axis identity; e.g. axis_pointer(mapping, ids, 19000)."""
    return shared_object(mapping, ids, 'axis', 'moLineRef_w', lambda: strip_tag(geometric('moLineRef_w', version, False, 0)))


def surface_wire(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Nonzero backend markers do not externalize the native-bound inline body; e.g. surface_wire(mapping, ids, 19000, 3)."""
    marker = lambda: shared_object(mapping, ids, 'marker', 'moPMarkRecord_c', lambda: struct.pack('<i', -2147483648))
    if style == 3:
        first = b'\xff'+struct.pack('<H', 2)+marker()+marker()
        raw = struct.pack('<ii', -2147483648, 2)+first+b'\x01'+bytes(2)+struct.pack('<iB', -1, 255)
    elif style == 4:
        raw = struct.pack('<3i', 1, 0, 1)+frame()+struct.pack('<H', 2)+marker()+marker()
    else:
        raw = struct.pack('<3i', 1, 0, 0)
    if version >= 0x2bcd:
        raw += struct.pack('<i3i', 3, -2147483648, 0, 2147483647) if style >= 3 else bytes(4)
    return raw


def surface_object(mapping: WireMap, ids: dict[str, int], version: int, style: int, words: tuple[int, int], key: str, edited: bool) -> bytes:
    """Use the reference-surface base rather than a body-feature revolution suffix; e.g. surface_object(mapping, ids, 19000, 1, (0, 0), 'surface-first', False)."""
    def contents() -> bytes:
        keeper = lambda: shared_object(mapping, ids, 'keeper-first', CLASSES[2], lambda: b'')
        raw = geometry_header(mapping, version, style == 3, keeper=keeper if style == 3 else None)
        raw += surface_wire(mapping, ids, version, style)
        raw += revolve_data(mapping, ids, version, style, words, 'data-first', edited) if style else bytes(2)
        return raw+(axis_pointer(mapping, ids, version) if style else bytes(2))
    return shared_object(mapping, ids, key, CLASSES[0], contents)


def revolved_object(kind: int, version: int, style: int, words: tuple[int, int], edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Distinguish equal data/keeper objects and preserve shared surface dependencies; e.g. revolved_object(0, 19000, 2, (0, 2))."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def pointer(key: str) -> bytes:
        if kind == 0:
            return surface_object(mapping, ids, version, style, words, key, edited)
        if kind == 1:
            return revolve_data(mapping, ids, version, style, words, key, edited)
        return shared_object(mapping, ids, key, CLASSES[2], lambda: b'')
    def array() -> bytes:
        if kind == 0:
            raw = revolve_data(mapping, ids, version, style, words, 'data-first', edited)+axis_pointer(mapping, ids, version)
            return struct.pack('<H', 5)+raw+pointer('surface-first')+pointer('surface-second')+pointer('surface-first')
        prefix = 'data' if kind == 1 else 'keeper'
        return struct.pack('<H', 3)+pointer(prefix+'-first')+pointer(prefix+'-second')+pointer(prefix+'-first')
    raw = mapping.new('suObArray', array) if style == 2 or (kind == 2 and style != 0) else pointer(('surface', 'data', 'keeper')[kind]+'-first')
    return raw, mapping.slot+1
