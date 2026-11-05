"""Independent complete ordinary parameter graphs and literal value edits (#202)."""
import struct
from fixture_object_map import WireMap
from owned_name_edit import short_wide_name

PARAMETER_CLASSES = ('moDensityParameter_c', 'moLengthParameter_c', 'moAngleParameter_c',
                     'moIntegerParameter_c', 'moScalerParameter_c', 'moStringParameter_c')
PRESENCE_BYTES = (0, 1, 255, 128)
PRIMARY_BITS = (0x8000000000000000, 0x7ff800000000002a, 0xc023000000000000, 1)
ALTERNATE_BITS = (0x7ff0000000000000, 0xfff0000000000000, 0x7ff0000000000001, 0xfff8000000000042)
PRIMARY_INTEGERS = (-2**31, -1, 0, 2**31 - 1)
ALTERNATE_INTEGERS = (2**31 - 1, -2**31, 42, -42)


def parameter_names(mapping: WireMap, style: int) -> tuple[bytes, bytes]:
    """Keep both serialized name pointers independent; e.g. parameter_names(mapping,1)."""
    first = bytes(2) if style == 0 else mapping.new('moNodeName_c', lambda: short_wide_name('node Ω'))
    identity = mapping.slot
    if style == 1:
        return first, struct.pack('<H', identity)
    second = bytes(2) if style == 3 else mapping.new('moNodeName_c', lambda: short_wide_name('parameter λ'))
    return first, second


def parameter_string(style: int, edited: bool) -> bytes:
    """Use literal legal CString frames independently of Bessemer; e.g. parameter_string(2,False)."""
    if edited:
        return short_wide_name('updated')
    if style == 0:
        return bytes(1)
    if style == 1:
        return b'\xff\xfe\xff\x03' + 'A\0Ω'.encode('utf-16le')
    if style == 2:
        return b'\xff\x03\x00ABC'
    return b'\xff\xfe\xff\x04' + '😀\0z'.encode('utf-16le')


def parameter_value(kind: int, style: int, edited: bool) -> bytes:
    """Preserve exact primary/alternate images and full presence bytes; e.g. parameter_value(0,2,False)."""
    if kind == 5:
        return parameter_string(style, edited)
    primary = struct.pack('<i', 12345 if edited else PRIMARY_INTEGERS[style]) if kind == 3 else struct.pack('<Q', 0x3ff4000000000000 if edited else PRIMARY_BITS[style])
    alternate = struct.pack('<i', ALTERNATE_INTEGERS[style]) if kind == 3 else struct.pack('<Q', ALTERNATE_BITS[style])
    return primary + bytes([PRESENCE_BYTES[style]]) + (alternate if PRESENCE_BYTES[style] else b'')


def parameter_graph(kind: int, style: int, edited: bool = False, first_slot: int = 96) -> tuple[bytes, int]:
    """Reserve map identities before each body; e.g. parameter_graph(4,1,False,96)."""
    mapping = WireMap()
    mapping.slot = first_slot
    def body() -> bytes:
        node_name, parameter_name = parameter_names(mapping, style)
        node = node_name + struct.pack('<iIiI', -2, 0xffffffff, -3, 0x80000000) + b'\x04help'
        return node + parameter_name + parameter_value(kind, style, edited)
    root = mapping.new(PARAMETER_CLASSES[kind], body)
    return struct.pack('<iH', -123, 1) + root, mapping.slot + 1


def inherited_parameter_graph(identity: int, first_slot: int, edited: bool = False) -> tuple[bytes, int]:
    """Use an independently loaded configuration name for both pointers; e.g. inherited_parameter_graph(4,96)."""
    mapping = WireMap()
    mapping.slot = first_slot
    reference = struct.pack('<H', identity)
    node = reference + struct.pack('<iIiI', -2, 0xffffffff, -3, 0x80000000) + b'\x04help'
    value = struct.pack('<QB', 0xc023000000000000 if edited else 0x3ff4000000000000, 0)
    feature = mapping.new('moDensityParameter_c', lambda: node + reference + value)
    return struct.pack('<iH', -123, 1) + feature, mapping.slot + 1
