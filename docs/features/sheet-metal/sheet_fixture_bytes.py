"""Complete ordinary sheet-metal wire assembly, including strided allowance bytes (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap, body
from freeze_component_references import base as component_base, wire as component_wire
from display_dimension_fixture_bytes import body as strip_tag
from sweep_fixture_bytes import dimension, shared_object
from owned_name_edit import short_wide_name

NAMES = ('moSheetMetal_c', 'moTemplateSheetMetal_c', 'moFixedFaceInFlat_c', 'moCompSheetmetalFeat_c')


@dataclass(frozen=True)
class SheetFixtureOptions:
    extended: bool
    rich: bool
    allowance: int
    edited: bool = False


def raw_words(count: int, seed: int) -> bytes:
    """Keep payload bits unrelated to float interpretation; e.g. raw_words(4, 17)."""
    values = (0x8000000000000000, 0x7ff8000000001234, 0xffffffffffffffff, 0x0102030405060708)
    return b''.join(struct.pack('<Q', values[index % 4] ^ (seed << 32)) for index in range(count))


def dynamic_array(size: int, count: int, seed: int) -> bytes:
    """Distinguish absent descriptors and zero-length allocated arrays; e.g. dynamic_array(16, 2, 7)."""
    if size == 0:
        return bytes(4)
    return struct.pack('<4I', size, count+3, 255 if count == 0 else 2, count)+raw_words(size//8*count, seed)


def allowance_arrays(style: int) -> tuple[bytes, int]:
    """Use counts rather than payload-word counts for grid extents; e.g. allowance_arrays(2)."""
    if style == 2:
        return dynamic_array(16, 2, 1)+dynamic_array(8, 3, 2)+dynamic_array(24, 1, 3), 6
    if style == 3:
        return dynamic_array(8, 2, 1)+dynamic_array(0, 0, 2)+dynamic_array(16, 1, 3), 0
    if style == 4:
        return dynamic_array(24, 0, 1)+dynamic_array(8, 1, 2)+dynamic_array(16, 2, 3), 0
    return bytes(12), 0


def optional_dimension(mapping: WireMap, version: int, ids: dict[str, int], rich: bool) -> bytes:
    """Share the complete distance dimension across independent tags; e.g. optional_dimension(mapping, 19000, ids, True)."""
    return dimension(mapping, version, ids, 'moDisplayDistanceDim_c') if rich else bytes(2)


def component_reference(mapping: WireMap, version: int, ids: dict[str, int], name: str) -> bytes:
    """Use the correct ordinary component layer for each concrete class; e.g. component_reference(mapping, 19000, ids, NAMES[3])."""
    def contents() -> bytes:
        if name == NAMES[3]:
            return component_base(version, False)+struct.pack('<i', -2147483648)
        if name == 'moCompFeature_c':
            return component_base(version, False)+struct.pack('<iI', -7, 0xfedcba98)
        return strip_tag(component_wire(name, version, False, 0))
    return shared_object(mapping, ids, name, name, contents)


def string_array(values: list[str], extended: bool = False) -> bytes:
    """Keep independent direct string-array counts; e.g. string_array(['Ω'], True)."""
    count = struct.pack('<HI', 65535, len(values)) if extended else struct.pack('<H', len(values))
    return count+b''.join(short_wide_name(value) for value in values)


def allowance_tail(version: int, rich: bool) -> bytes:
    """Keep compatibility words and material pointers separate; e.g. allowance_tail(0x2baf, True)."""
    raw = b''.join(short_wide_name(value) if rich else b'\0' for value in ('110 Ω', '100 Ω', 'f8 Ω', '108 Ω'))
    if version > 0x2bae:
        raw += (struct.pack('<i', -2147483648) if version < 0x2bba else b'')+bytes(2)
    raw += string_array(['first Ω', 'second Ω'] if rich else [], rich)
    raw += string_array([])+string_array(['last Ω'] if rich else [])
    return raw


def allowance_body(mapping: WireMap, version: int, ids: dict[str, int], options: SheetFixtureOptions) -> bytes:
    """Build the full direct body without extra slots; e.g. allowance_body(mapping, 19000, ids, options)."""
    raw, count = allowance_arrays(options.allowance)
    raw += short_wide_name('allowance Ω') if options.rich else b'\0'
    raw += struct.pack('<iB', -2147483648, 255 if options.rich else 0)
    raw += optional_dimension(mapping, version, ids, options.rich)
    raw += optional_dimension(mapping, version, ids, options.rich)+struct.pack('<2i', -1, 2147483647)
    for index in range(count):
        bits = (0x8000000000000000, 0x7ff8000000001234, 0x3ff4000000000000)[index % 3]
        raw += struct.pack('<Q', struct.unpack('<Q', struct.pack('<d', 123.5))[0] if options.edited and index == 3 else bits)
    return raw+allowance_tail(version, options.rich)


def common_bend(mapping: WireMap, version: int, ids: dict[str, int], options: SheetFixtureOptions) -> bytes:
    """Preserve discarded words and helper-selected tagged occurrences; e.g. common_bend(mapping, 19000, ids, options)."""
    raw = optional_dimension(mapping, version, ids, options.rich)
    raw += component_reference(mapping, version, ids, 'moCompFeature_c') if options.rich else bytes(2)
    raw += bytes([255 if options.allowance else 0])
    if options.allowance:
        raw += allowance_body(mapping, version, ids, options)
    raw += struct.pack('<4i', -2147483648, 2147483647, -1, 0)
    if options.extended:
        raw += component_reference(mapping, version, ids, NAMES[3]) if options.rich else bytes(2)
    return raw


def fixed_face(mapping: WireMap, version: int, ids: dict[str, int], options: SheetFixtureOptions) -> bytes:
    """The extended bend ID is a full i32 rather than an object tag; e.g. fixed_face(mapping, 19000, ids, options)."""
    raw = struct.pack('<i', -2147483648) if options.extended else bytes(2)
    raw += struct.pack('<i', 2147483647)
    raw += component_reference(mapping, version, ids, 'moCompFace_c') if options.rich else bytes(2)
    raw += struct.pack('<2Q', 0x8000000000000000, 0x7ff8000000001234)
    return raw+(component_reference(mapping, version, ids, 'moCompEdge_c') if options.rich else bytes(2))


def sheet_payload(mapping: WireMap, version: int, ids: dict[str, int], options: SheetFixtureOptions) -> bytes:
    """Follow the native field order and independent 0x2BC5 gate; e.g. sheet_payload(mapping, 19000, ids, options)."""
    raw = common_bend(mapping, version, ids, options)+struct.pack('<3iQ', -1, 2147483647, -2147483648, 0x8000000000000000)
    raw += optional_dimension(mapping, version, ids, options.rich)+struct.pack('<i', -2147483648)
    raw += optional_dimension(mapping, version, ids, options.rich)
    raw += shared_object(mapping, ids, 'fixed-face', NAMES[2], lambda: fixed_face(mapping, version, ids, options)) if options.rich else bytes(2)
    raw += struct.pack('<2i', -1, 2147483647)+(short_wide_name('sheet Ω') if options.rich else b'\0')
    raw += struct.pack('<4i', -2147483648, 2147483647, -1, 0)
    if options.extended:
        raw += struct.pack('<2i', -2147483648, 2147483647)
        if version >= 0x2bc5:
            raw += struct.pack('<i', -7)
    return raw


def sheet_object(kind: int, version: int, options: SheetFixtureOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Own map allocation before every nested body; e.g. sheet_object(1, 19000, SheetFixtureOptions(True, True, 2), 96)."""
    if kind not in range(4) or options.allowance not in range(5) or first_slot < 0:
        raise ValueError(f'sheet fixture {(kind, options.allowance, first_slot)}; expected kind 0..3, allowance 0..4 and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        if kind == 2:
            return fixed_face(mapping, version, ids, options)
        if kind == 3:
            return component_base(version, options.rich)+struct.pack('<i', -2147483648)
        owner = mapping.slot
        return body(mapping, version, options.rich, owner)+sheet_payload(mapping, version, ids, options)
    raw = mapping.new(NAMES[kind], contents)
    return raw, mapping.slot+1
