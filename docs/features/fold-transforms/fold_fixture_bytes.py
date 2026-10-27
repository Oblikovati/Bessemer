"""Complete fold folders, transform collections and typed property records (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap
from freeze_environment import feature
from freeze_views import transform
from owned_name_edit import short_wide_name
from sheet_fixture_bytes import component_reference
from sweep_fixture_bytes import shared_object

CLASSES = ('moFoldUpDownSketchFolder_c', 'moFoldUpDownSketchXformData_c',
           'moFoldUpDownFeatXformData_c', 'moFoldUpDownSketchXformPropertyData_c')


@dataclass(frozen=True)
class FoldFixtureOptions:
    rich: bool = False
    matrix: bool = False
    flags: int = 0
    edited: bool = False
    transformed: bool = False


def property_wire(options: FoldFixtureOptions) -> bytes:
    """Keep signed/unsigned domains and both independent CStrings; e.g. property_wire(options)."""
    raw = struct.pack('<3iI', -2147483648, 2147483647, -1, 0xfedcba98) if options.rich else bytes(16)
    raw += short_wide_name('Fold Ω') if options.rich else bytes(1)
    return raw+(short_wide_name('Bessemer Ω') if options.edited else short_wide_name('Property Δ') if options.rich else bytes(1))


def property_pointer(mapping: WireMap, ids: dict[str, int], options: FoldFixtureOptions) -> bytes:
    """Reserve property identity before its first payload; e.g. property_pointer(mapping, ids, options)."""
    return shared_object(mapping, ids, 'fold-property', CLASSES[3], lambda: property_wire(options))


def direct_list(mapping: WireMap, version: int, ids: dict[str, int], rich: bool) -> bytes:
    """Retain nulls and shared feature targets in direct lists; e.g. direct_list(mapping, 19000, ids, True)."""
    if not rich:
        return bytes(2)
    first = component_reference(mapping, version, ids, 'moCompFeature_c')
    return struct.pack('<H', 3)+first+bytes(2)+component_reference(mapping, version, ids, 'moCompFeature_c')


def transform_list(options: FoldFixtureOptions, edited: bool = False) -> bytes:
    """Exercise a selected matrix/perspective and an independent minimal transform; e.g. transform_list(options)."""
    if not options.rich:
        return bytes(4)
    first = bytearray(transform(options.matrix, options.flags))
    if edited:
        struct.pack_into('<d', first, 73 if options.matrix else 1, 123.5)
    return struct.pack('<i', 2)+first+transform(not options.matrix, 0)


def sketch_wire(mapping: WireMap, version: int, ids: dict[str, int], options: FoldFixtureOptions) -> bytes:
    """Keep lists, transforms, property pairs and byte array in native wire order; e.g. sketch_wire(mapping, 19000, ids, options)."""
    raw = direct_list(mapping, version, ids, options.rich)
    raw += struct.pack('<H', 1)+property_pointer(mapping, ids, options) if options.rich else bytes(2)
    raw += transform_list(options, options.transformed)+struct.pack('<i', 3 if options.rich else 0)
    if options.rich:
        pair = component_reference(mapping, version, ids, 'moCompFeature_c')+property_pointer(mapping, ids, options)
        raw += pair+pair+bytes(4)
    count = struct.pack('<HI', 65535, 4) if options.flags & 1 else struct.pack('<H', 4)
    raw += count+b'\x00\xff\x80\x7f' if options.rich else bytes(2)
    return raw+direct_list(mapping, version, ids, options.rich)


def feature_wire(mapping: WireMap, version: int, ids: dict[str, int], options: FoldFixtureOptions) -> bytes:
    """Feature transforms omit sketch property and byte-array fields; e.g. feature_wire(mapping, 19000, ids, options)."""
    return direct_list(mapping, version, ids, options.rich)+transform_list(options)+direct_list(mapping, version, ids, options.rich)


def folder_wire(mapping: WireMap, version: int, ids: dict[str, int], options: FoldFixtureOptions) -> bytes:
    """Use the complete feature-folder base and two fresh tagged transform records; e.g. folder_wire(mapping, 19000, ids, options)."""
    mapping.slot += 2
    raw = feature(version, options.rich)+struct.pack('<2i', -2147483648, 2147483647)
    raw += shared_object(mapping, ids, 'fold-sketch', CLASSES[1], lambda: sketch_wire(mapping, version, ids, options))
    return raw+shared_object(mapping, ids, 'fold-feature', CLASSES[2], lambda: feature_wire(mapping, version, ids, options))


def fold_object(kind: int, version: int, options: FoldFixtureOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Construct a whole class before computing inherited-map size; e.g. fold_object(0, 19000, options, 96)."""
    if kind not in range(4) or first_slot < 0 or not 0 <= options.flags <= 255:
        raise ValueError(f'fold fixture {(kind, first_slot, options.flags)}; expected kind 0..3, nonnegative slot and u8 flags')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    readers = (lambda: folder_wire(mapping, version, ids, options), lambda: sketch_wire(mapping, version, ids, options),
               lambda: feature_wire(mapping, version, ids, options), lambda: property_wire(options))
    raw = mapping.new(CLASSES[kind], readers[kind])
    return raw, mapping.slot+1
