"""Complete flat-pattern and direct/tagged bounding-box fixture wires (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap, body
from flange_fixture_bytes import FlangeFixtureOptions, group_wire, flange_reference
from sheet_fixture_bytes import component_reference
from sweep_fixture_bytes import shared_object
from kernel_geometry_fixture import frame
from owned_name_edit import short_wide_name


@dataclass(frozen=True)
class FlatFixtureOptions:
    extended: bool = False
    rich: bool = False
    present: bool = False
    edited: bool = False


def box_wire(rich: bool, edited: bool = False) -> bytes:
    """Preserve vector order and the final unaligned double; e.g. box_wire(True)."""
    values = (-0.0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11) if rich else (0,)*12
    return struct.pack('<12di', *values, -2147483648 if rich else 0)+struct.pack('<d', 123.5 if edited else -0.0)


def direct_array(mapping: WireMap, version: int, ids: dict[str, int], rich: bool) -> bytes:
    """Retain shared/null/shared occurrences without adding a container slot; e.g. direct_array(mapping, 19000, ids, True)."""
    if not rich:
        return bytes(2)
    first = component_reference(mapping, version, ids, 'moCompFeature_c')
    return struct.pack('<H', 3)+first+bytes(2)+component_reference(mapping, version, ids, 'moCompFeature_c')


def flat_prefix(mapping: WireMap, version: int, ids: dict[str, int], options: FlatFixtureOptions) -> bytes:
    """Keep independent array/reference fields and original raw-word order; e.g. flat_prefix(mapping, 19000, ids, options)."""
    raw = direct_array(mapping, version, ids, options.rich)
    for _ in range(3):
        raw += component_reference(mapping, version, ids, 'moCompFace_c') if options.rich else bytes(2)
    raw += struct.pack('<i', -1)
    if version > 0x372b:
        raw += struct.pack('<i', -2147483648)
    raw += direct_array(mapping, version, ids, options.rich)+struct.pack('<4idi d4i', -1, 2, -3, 4, -0.0, -5, 6.25, 7, -8, 9, -10)
    raw += component_reference(mapping, version, ids, 'moCompFeature_c') if options.rich else bytes(2)
    raw += shared_object(mapping, ids, 'flat-list', 'suObList', lambda: direct_array(mapping, version, ids, True)) if options.rich else bytes(2)
    raw += struct.pack('<i', -2147483648 if options.present else 0)
    return raw+(frame()[1:] if options.present else b'')


def flat_tail(mapping: WireMap, version: int, ids: dict[str, int], options: FlatFixtureOptions) -> bytes:
    """Keep helper, CString, direct box and duplicate pair keys separate; e.g. flat_tail(mapping, 19000, ids, options)."""
    raw = struct.pack('<i', 1)+direct_array(mapping, version, ids, options.rich)
    if version > 0x3364:
        raw += component_reference(mapping, version, ids, 'moCompFace_c') if options.rich else bytes(2)
    if version > 0x3761:
        raw += struct.pack('<i', -1 if options.rich else 0)+(short_wide_name('Flat Ω') if options.rich else b'')
    raw += flange_reference(mapping, version, ids, options.rich)+box_wire(options.rich, options.edited)
    raw += struct.pack('<4i', -2147483648, 2147483647, -1, 0)
    if options.extended:
        raw += struct.pack('<i', -23)
    return raw+(struct.pack('<7i', 3, 7, 8, 7, -9, -2147483648, 2147483647) if options.rich else bytes(4))


def flat_object(version: int, options: FlatFixtureOptions, first_slot: int = 0, name: str = 'moFlatPattern_c') -> tuple[bytes, int]:
    """Build complete inherited objects in detached or imported maps; e.g. flat_object(19000, options, 96)."""
    if first_slot < 0 or name not in ('moFlatPattern_c', 'moFlatPatBoundBox_c', 'moBoundBoxData_c'):
        raise ValueError(f'flat fixture {(first_slot, name)}; expected nonnegative slot and flat-pattern/box class')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        if name != 'moFlatPattern_c':
            return box_wire(options.rich, options.edited)
        owner = mapping.slot
        inherited = body(mapping, version, options.rich, owner)
        inherited += group_wire(mapping, version, ids, FlangeFixtureOptions(extended=options.extended, rich=options.rich))
        return inherited+flat_prefix(mapping, version, ids, options)+flat_tail(mapping, version, ids, options)
    raw = mapping.new(name, contents)
    return raw, mapping.slot+1
