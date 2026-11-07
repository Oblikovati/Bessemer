"""Complete draft feature payloads with distinct count, presence and stride conditions (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from sheet_fixture_bytes import dynamic_array, component_reference
from draft_reference_bytes import DraftOptions, face, nullable_array, draft_dimension, draft_direction, draft_words, draft_plane, keeper

CLASSES = ('moDraft_c', 'moDraftSIdKeeper_c', 'moEdgeFaceSIdKeeper_c')


def references(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Keep every declared reference and nullable array in native order; e.g. references(mapping, 19000, ids, options)."""
    raw = draft_dimension(mapping, version, ids, options)+struct.pack('<i', 12345 if options.edited else -2147483648)
    raw += draft_direction(mapping, ids, options)+draft_words(mapping, ids, options)
    raw += nullable_array(mapping, version, ids, options)+struct.pack('<3i', -1, 2147483647, 0)
    raw += nullable_array(mapping, version, ids, options)+struct.pack('<i', -32769)
    raw += component_reference(mapping, version, ids, 'moCompSurfaceBody_c') if options.style else bytes(2)
    return raw+draft_plane(mapping, version, ids, options)


def groups(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Preserve absent, present-empty and repeated face groups separately; e.g. groups(mapping, 19000, ids, DraftOptions(2))."""
    presence = (0, -1, -2147483648, 0, 1, 2147483647)[options.style]
    if not presence:
        return bytes(4)
    widths = {1: (0, 2, 1, 1), 2: (), 4: (0,), 5: (1, 1)}[options.style]
    raw = struct.pack('<3i', presence, -2147483648, len(widths))
    for width in widths:
        raw += struct.pack('<i', width)
        raw += b''.join(face(mapping, version, ids) if index % 2 == 0 else bytes(2) for index in range(width))
    return raw


def points(options: DraftOptions) -> bytes:
    """Keep direct point values without object framing; e.g. points(DraftOptions(1))."""
    if options.style in (0, 2, 4):
        return bytes(4)
    return struct.pack('<i9Q', 3, 1 << 63, 0x7ff8000000001234, 1, 2, 3, 4, 5, 6, 7)


def payload(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Use independent modern presence and version gates; e.g. payload(mapping, 19000, ids, options)."""
    raw = references(mapping, version, ids, options)
    presence = (0, -1, 1, -2147483648, 0, 2)[options.style]
    raw += struct.pack('<i', presence)
    if presence:
        size, count = {1: (16, 2), 2: (0, 0), 3: (8, 0), 5: (24, 2)}[options.style]
        raw += dynamic_array(size, count, 7)
    raw += struct.pack('<i', 65536)+groups(mapping, version, ids, options)+points(options)
    if version > 0x3746:
        raw += draft_dimension(mapping, version, ids, options)+struct.pack('<2i', -32769, 65536)
    return raw+keeper(mapping, version, ids, options)


def draft_object(kind: int, version: int, options: DraftOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Count complete inherited body and direct keeper allocations; e.g. draft_object(0, 19000, DraftOptions(4))."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def contents() -> bytes:
        if kind != 0:
            return keeper(mapping, version, ids, options)
        inherited = body(mapping, version, options.style == 4, mapping.slot)
        return inherited+payload(mapping, version, ids, options)
    raw = mapping.new(CLASSES[kind], contents)
    return raw, mapping.slot+1
