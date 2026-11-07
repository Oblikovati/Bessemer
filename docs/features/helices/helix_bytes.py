"""Complete independently assembled helices and wire-edge representation graphs (#54)."""
from dataclasses import dataclass
import math
import struct
from fillet_fixture_bytes import WireMap
from axis_owner_bytes import geometry_header
from kernel_geometry_fixture import frame
from sweep_fixture_bytes import dimension, shared_object
from radiated_edge_bytes import representation_references

VERSIONS = (0x27cc, 19000, 0xffffffff)
WORDS = (-2147483648, 0, 2147483647)


@dataclass(frozen=True)
class HelixOptions:
    pitch_bits: int = 0
    variable_presence: int = 0
    array_style: int = 0
    table_presence: int = 0
    rows: int = 0
    dimensions: bool = False
    wire: bool = False
    rich: bool = False


def double_bits(value: float) -> int:
    """Select adjacent stored doubles without decimal-rounding ambiguity; e.g. double_bits(1e-6)."""
    return struct.unpack('<Q', struct.pack('<d', value))[0]


THRESHOLDS = tuple(double_bits(value) for value in (
    math.nextafter(-1e-6, -math.inf), -1e-6, math.nextafter(-1e-6, 0), -0.0, 0.0,
    math.nextafter(1e-6, 0), 1e-6, math.nextafter(1e-6, math.inf), math.ulp(0.0))) + (
    0x7ff8000000001234, 0xfff8000000001234, 0x7ff0000000001234, 0x7ff0000000000000, 0xfff0000000000000)


def parameter_wire(index: int, style: int) -> bytes:
    """Keep five arrays' independent descriptors, metadata and raw values; e.g. parameter_wire(2, 3)."""
    if style == 0 or (style == 1 and index % 2 == 1):
        return bytes(4)
    count = ((0, 0, 0, 0, 0), (0, 0, 0, 0, 0), (2, 0, 3, 1, 0), (0, 2, 0, 3, 1))[style][index]
    values = (0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000000)
    return struct.pack('<4I', 8, 0xffffffff-index, 0x80000000+index, count)+b''.join(struct.pack('<Q', values[i]) for i in range(count))


def helix_data(options: HelixOptions) -> bytes:
    """Keep the direct 89-byte prefix and any nonzero variable byte; e.g. helix_data(HelixOptions(variable_presence=255))."""
    raw = struct.pack('<2i3Q', -2147483648, 2147483647, 0x8000000000000000, 0x7ff8000000001234, 1)
    raw += struct.pack('<Q2d2i3dB', options.pitch_bits, -2.5, 3.75, -1, 0, 4.5, -0.0, 6.25, options.variable_presence)
    if options.variable_presence:
        raw += struct.pack('<i', -2147483648)+b''.join(parameter_wire(index, options.array_style) for index in range(5))
    return raw


def display_pointer(mapping: WireMap, ids: dict[str, int], version: int, options: HelixOptions, angular: bool = False) -> bytes:
    """Share complete dimension bodies across main and table occurrences; e.g. display_pointer(mapping, ids, 19000, options)."""
    name = 'moDisplayAngularDim_c' if angular else 'moDisplayDistanceDim_c'
    return dimension(mapping, version, ids, name) if options.dimensions else bytes(2)


def table_wire(mapping: WireMap, ids: dict[str, int], version: int, options: HelixOptions) -> bytes:
    """Only an exact-one flag writes rows, with a terminal pointer only after nonempty rows; e.g. table_wire(mapping, ids, 19000, options)."""
    if options.table_presence != 1:
        return b''
    raw = struct.pack('<i', options.rows)
    for _ in range(options.rows):
        for column in range(4):
            raw += bytes(2) if column == 1 else display_pointer(mapping, ids, version, options, column == 3)
    return raw+(display_pointer(mapping, ids, version, options, True) if options.rows else b'')


def helix_payload(mapping: WireMap, ids: dict[str, int], version: int, options: HelixOptions, edited: bool) -> bytes:
    """Use the native strict threshold on a retained copy of the stored bits; e.g. helix_payload(mapping, ids, 19000, options, False)."""
    raw = struct.pack('<iQ', 12345 if edited else -2147483648, 0xfff8000000004321)+helix_data(options)
    for index in range(3):
        raw += bytes(2) if index == 1 else display_pointer(mapping, ids, version, options)
    value = struct.unpack('<d', struct.pack('<Q', options.pitch_bits))[0]
    if abs(value) > 1e-6:
        raw += display_pointer(mapping, ids, version, options)
    raw += struct.pack('<i', 2147483647)+display_pointer(mapping, ids, version, options)+bytes([options.table_presence])
    return raw+table_wire(mapping, ids, version, options)


def reference_curve(mapping: WireMap, ids: dict[str, int], version: int, options: HelixOptions) -> bytes:
    """A direct attribute list adds no class/object slot even around a suppressed wire; e.g. reference_curve(mapping, ids, 19000, options)."""
    raw = geometry_header(mapping, version, options.rich)+(frame() if options.wire else b'\xff')+struct.pack('<i', -2147483648)
    if not options.rich:
        return raw+bytes(2)
    marker = shared_object(mapping, ids, 'wire-attribute', 'moPMarkRecord_c', lambda: struct.pack('<i', 2147483647))
    return raw+struct.pack('<HI', 65535, 3)+marker+bytes(2)+struct.pack('<H', ids['wire-attribute'])


def helix_record(mapping: WireMap, ids: dict[str, int], version: int, options: HelixOptions, key: str, edited: bool) -> bytes:
    """Reserve the enclosing helix before its inherited chooser references; e.g. helix_record(mapping, ids, 19000, options, 'helix-first', False)."""
    return shared_object(mapping, ids, key, 'moHelix_c', lambda: reference_curve(mapping, ids, version, options)
        +helix_payload(mapping, ids, version, options, edited and key == 'helix-first'))


def helix_object(version: int, options: HelixOptions, alias: bool = False, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep two equal helices distinct while sharing dimensions and repeated occurrences; e.g. helix_object(19000, options, True)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def record(key: str) -> bytes:
        return helix_record(mapping, ids, version, options, key, edited)
    def array() -> bytes:
        return struct.pack('<H', 3)+record('helix-first')+record('helix-second')+record('helix-first')
    raw = mapping.new('suObArray', array) if alias else record('helix-first')
    return raw, mapping.slot+1


def wire_identifier_object(version: int, style: int, word: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Compose complete inherited references and declared-role owners; e.g. wire_identifier_object(19000, 2, -2147483648)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def feature() -> bytes:
        return helix_record(mapping, ids, version, HelixOptions(), 'helix-first', False) if style else bytes(2)
    def pointer(key: str) -> bytes:
        def contents() -> bytes:
            base = representation_references(mapping, ids, feature(), style != 0)
            return base+struct.pack('<i', 12345 if edited and key == 'first' else word)
        return shared_object(mapping, ids, key, 'moWireEdgeIdRep_c', contents)
    def owner(key: str) -> bytes:
        return mapping.new('moDerivedEdgeIdRep_c', lambda: bytes(4)+pointer(key)+struct.pack('<i', -32769))
    def array() -> bytes:
        return struct.pack('<H', 4)+feature()+owner('first')+owner('second')+pointer('first')
    raw = mapping.new('suObArray', array) if style == 2 else pointer('first')
    return raw, mapping.slot+1
