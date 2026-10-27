"""Complete bend-group, direction and counted-handle wire construction (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap, body
from profile_fixture_bytes import contents as profile_contents
from sketch_tail_fixture_bytes import ordinary as ordinary_sketch
from freeze_model_references import wrapper
from sheet_fixture_bytes import SheetFixtureOptions, common_bend, component_reference
from sweep_fixture_bytes import shared_object, dimension


@dataclass(frozen=True)
class FlangeFixtureOptions:
    extended: bool = False
    rich: bool = False
    mode: int = 1
    first_type: int = 0
    second_type: int = 6
    reference_byte: int = 0
    handles: bool = False
    edited: bool = False


def flange_profile(mapping: WireMap, version: int, ids: dict[str, int], rich: bool) -> bytes:
    """Use the complete profile body without a guessed suffix; e.g. flange_profile(mapping, 19000, ids, True)."""
    if not rich:
        return bytes(2)
    return shared_object(mapping, ids, 'flange-profile', 'moProfileFeature_c',
        lambda: profile_contents(mapping, version, False, 0, False, mapping.slot))


def flange_reference(mapping: WireMap, version: int, ids: dict[str, int], rich: bool) -> bytes:
    """Preserve the wrapper occurrence and its own map identity; e.g. flange_reference(mapping, 19000, ids, True)."""
    if not rich:
        return bytes(2)
    return shared_object(mapping, ids, 'flange-wrapper', 'moRefWrapper_c', lambda: wrapper(version, False))


def direction_wire(mapping: WireMap, version: int, ids: dict[str, int], kind: int, options: FlangeFixtureOptions) -> bytes:
    """Separate untyped-reference and conditional-wrapper branches; e.g. direction_wire(mapping, 19000, ids, 3, options)."""
    raw = struct.pack('<i', kind)
    if kind in (0, 6):
        raw += struct.pack('<Q', 0x7ff8000000001234 if kind == 6 else 0x8000000000000000)
    elif kind == 3:
        raw += component_reference(mapping, version, ids, 'moCompFeature_c') if options.rich else bytes(2)
    elif kind in (4, 5) and options.reference_byte:
        raw += flange_reference(mapping, version, ids, options.rich)
    raw += dimension(mapping, version, ids, 'moDisplayDistanceDim_c') if options.rich else bytes(2)
    return raw+struct.pack('<d', 123.5 if options.edited else -0.0)


def handle_collection(mapping: WireMap, version: int, ids: dict[str, int], count: int) -> bytes:
    """Store one sketch tag for each nonempty collection and share all targets; e.g. handle_collection(mapping, 19000, ids, 3)."""
    raw = struct.pack('<i', count)
    if count == 0:
        return raw
    raw += shared_object(mapping, ids, 'flange-sketch', 'sgSketch', lambda: ordinary_sketch(mapping, version, False))
    first = shared_object(mapping, ids, 'flange-handle', 'sgPointHandle', lambda: struct.pack('<Hiii', 0x777f, -1, -19, 2147483647))
    remaining = (struct.pack('<H', ids['flange-handle'])+bytes(2)) if count == 3 else (bytes(2) if count == 2 else b'')
    return raw+first+remaining


def group_wire(mapping: WireMap, version: int, ids: dict[str, int], options: FlangeFixtureOptions) -> bytes:
    """Retain serialized null/duplicate entries before native filtering; e.g. group_wire(mapping, 19000, ids, options)."""
    raw = common_bend(mapping, version, ids, SheetFixtureOptions(options.extended, options.rich, 2 if options.rich else 1))
    raw += struct.pack('<3i', -2147483648, 2147483647, -1)
    if options.rich:
        raw += struct.pack('<H', 3)+component_reference(mapping, version, ids, 'moCompFeature_c')+bytes(2)
        raw += component_reference(mapping, version, ids, 'moCompFeature_c')
    else:
        raw += bytes(2)
    return raw+struct.pack('<I', 0xfedcba98)


def payload_wire(mapping: WireMap, version: int, ids: dict[str, int], options: FlangeFixtureOptions) -> bytes:
    """Follow selected slots in numeric order without subtracting from extreme modes; e.g. payload_wire(mapping, 19000, ids, options)."""
    raw = flange_profile(mapping, version, ids, options.rich)
    raw += struct.pack('<3iBi', -2147483648, 2147483647, options.mode, options.reference_byte, -1)
    for index, kind in enumerate((options.first_type, options.second_type)):
        if options.mode == 0 or options.mode == index+1:
            raw += direction_wire(mapping, version, ids, kind, options)
    raw += b''.join(handle_collection(mapping, version, ids, count) for count in ((3, 0, 1, 2) if options.handles else (0, 0, 0, 0)))
    if version > 0x3b0c:
        raw += struct.pack('<i', -2147483648)
    if version > 0x46b9:
        raw += from_end(mapping, version, ids, options.rich)
    return raw


def from_end(mapping: WireMap, version: int, ids: dict[str, int], rich: bool) -> bytes:
    """Reuse the documented from-end body with real MFC framing; e.g. from_end(mapping, 19000, ids, True)."""
    if not rich:
        return bytes(2)
    from body_feature_fixture_bytes import from_spec
    return shared_object(mapping, ids, 'flange-from-end', 'moFromEndSpec_c', lambda: from_spec(version, 0))


def flange_object(version: int, options: FlangeFixtureOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Build a complete inherited feature before its bend group; e.g. flange_object(19000, options, 96)."""
    if first_slot < 0 or not 0 <= options.reference_byte <= 255:
        raise ValueError(f'flange fixture {(first_slot, options.reference_byte)}; expected nonnegative slot and u8 reference byte')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        owner = mapping.slot
        return body(mapping, version, options.rich, owner)+group_wire(mapping, version, ids, options)+payload_wire(mapping, version, ids, options)
    raw = mapping.new('moSMBaseFlange_c', contents)
    return raw, mapping.slot+1
