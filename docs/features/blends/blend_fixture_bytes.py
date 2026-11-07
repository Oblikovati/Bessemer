"""Complete blend/cut fixtures with independent column counts and branch controls (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap, body
from sheet_fixture_bytes import component_reference
from sweep_fixture_bytes import shared_object, dimension
from body_feature_fixture_bytes import direction

CLASSES = ('moBlend_c', 'moBlendCut_c')


@dataclass(frozen=True)
class BlendFixtureOptions:
    rich: bool = False
    count: int = 0
    declared: int = 0
    first: int = 0
    second: int = 0
    dimensions: int = 0
    selector: int = 0
    edited: bool = False


def pointer_array(mapping: WireMap, version: int, ids: dict[str, int], count: int, rich: bool) -> bytes:
    """Keep each array's own count and repeated/null object occurrences; e.g. pointer_array(mapping, 19000, ids, 3, True)."""
    raw = struct.pack('<H', count)
    for index in range(count):
        raw += component_reference(mapping, version, ids, 'moCompFeature_c') if rich and index != 1 else bytes(2)
    return raw


def numeric_columns(options: BlendFixtureOptions) -> bytes:
    """Emit four complete columns with distinct independent values; e.g. numeric_columns(options)."""
    first = (0xffffffff, 0x80000000, 0x12345678)
    second = (0x89abcdef, 0, 0xfedcba98)
    points = (0x8000000000000000, 0x7ff8000000001234, 0x3ff0000000000000,
              0x4000000000000000, 0x4008000000000000, 0x4010000000000000,
              0x4014000000000000, 0x4018000000000000, 0x401c000000000000)
    raw = b''.join(struct.pack('<I', value) for value in first[:options.count])
    raw += b''.join(struct.pack('<I', value) for value in second[:options.count])
    raw += b''.join(struct.pack('<Q', value) for value in points[:3*options.count])
    values = (-2147483648, 12345 if options.edited else 2147483647, -1)
    return raw+b''.join(struct.pack('<i', value) for value in values[:options.count])


def blend_direction(mapping: WireMap, ids: dict[str, int], rich: bool) -> bytes:
    """Share a complete direction-spec object between the two selected slots; e.g. blend_direction(mapping, ids, True)."""
    if not rich:
        return bytes(2)
    return shared_object(mapping, ids, 'blend-direction', 'moDirectionSpec_c', lambda: direction(mapping, True))


def controls_wire(mapping: WireMap, version: int, ids: dict[str, int], options: BlendFixtureOptions) -> bytes:
    """Keep low-byte direction conditions separate from full-word dimension equality; e.g. controls_wire(mapping, 19000, ids, options)."""
    raw = struct.pack('<2i2d3Bd2i', -2147483648, 2147483647, -0.0, 1.25, 255, 128, 254, -2.5, options.first, options.second)
    for word in (options.first, options.second):
        if word & 255 == 2:
            raw += blend_direction(mapping, ids, options.rich)
    raw += pointer_array(mapping, version, ids, 3 if options.rich else 0, options.rich)*2
    raw += struct.pack('<i2dB2d2i', -1, -0.0, 3.75, options.dimensions, -4.5, 5.25, options.selector, -2147483648)
    for _ in range(4):
        raw += pointer_array(mapping, version, ids, 3 if options.rich else 0, options.rich)
    return raw+struct.pack('<4i', -2147483648, 2147483647, -1, 0)


def suffix_wire(mapping: WireMap, version: int, ids: dict[str, int], options: BlendFixtureOptions) -> bytes:
    """Retain independently selected display dimensions and all five bytes; e.g. suffix_wire(mapping, 19000, ids, options)."""
    raw = struct.pack('<i4B', -2147483648, 0, 128, 254, 255)
    if options.dimensions:
        raw += dimension(mapping, version, ids, 'moDisplayDistanceDim_c') if options.rich else bytes(2)
        if options.selector == 3:
            raw += dimension(mapping, version, ids, 'moDisplayDistanceDim_c') if options.rich else bytes(2)
    return raw+b'\xfe'


def blend_data(mapping: WireMap, version: int, ids: dict[str, int], options: BlendFixtureOptions) -> bytes:
    """Use the loaded primary array count for columns, never the allocation word; e.g. blend_payload(mapping, 19000, ids, options)."""
    raw = struct.pack('<i', options.declared)+pointer_array(mapping, version, ids, options.count, options.rich)
    for _ in range(2):
        raw += pointer_array(mapping, version, ids, 3 if options.rich else 0, options.rich)
    return raw+numeric_columns(options)+controls_wire(mapping, version, ids, options)


def blend_payload(mapping: WireMap, version: int, ids: dict[str, int], options: BlendFixtureOptions) -> bytes:
    """Keep the old blend/cut suffix after its shared direct data; e.g. blend_payload(mapping, 19000, ids, options)."""
    return blend_data(mapping,version,ids,options)+suffix_wire(mapping,version,ids,options)


def blend_object(kind: int, version: int, options: BlendFixtureOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Compose both complete classes in detached and inherited maps; e.g. blend_object(1, 19000, options, 96)."""
    if kind not in range(2) or first_slot < 0 or not 0 <= options.count <= 3 or not 0 <= options.declared <= 2147483647 or not 0 <= options.dimensions <= 255:
        raise ValueError(f'blend fixture {(kind, first_slot, options)}; expected kind 0..1, nonnegative slot/count 0..3/declared i32 and u8 dimensions')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        owner = mapping.slot
        return body(mapping, version, options.rich, owner)+blend_payload(mapping, version, ids, options)
    raw = mapping.new(CLASSES[kind], contents)
    return raw, mapping.slot+1
