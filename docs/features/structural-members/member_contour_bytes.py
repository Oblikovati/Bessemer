"""Complete contour/index/orientation/segment fixture bodies and shared targets (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from body_feature_fixture_bytes import direction
from sweep_fixture_bytes import shared_object
from member_reference_bytes import MemberFixtureOptions, member_curve, member_point


def index_body() -> bytes:
    """Keep three raw signed words without an inferred pointer/count; e.g. index_body()."""
    return struct.pack('<3i', -2147483648, 2147483647, -1)


def index_pointer(mapping: WireMap, ids: dict[str, int]) -> bytes:
    """Share a complete segment-index leaf across different arrays; e.g. index_pointer(mapping, ids)."""
    return shared_object(mapping, ids, 'member-index', 'moWeldSegSkIndex_c', index_body)


def index_array(mapping: WireMap, ids: dict[str, int], rich: bool) -> bytes:
    """Direct packed arrays preserve null and repeated index targets; e.g. index_array(mapping, ids, True)."""
    if not rich:
        return bytes(2)
    return struct.pack('<H', 3)+index_pointer(mapping, ids)+bytes(2)+index_pointer(mapping, ids)


def indices_pointer(mapping: WireMap, ids: dict[str, int], rich: bool) -> bytes:
    """Preserve the plural container's tagged identity and direct packed body; e.g. indices_pointer(mapping, ids, True)."""
    if not rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-indices', 'moWeldSegSkIndices_c', lambda: index_array(mapping, ids, rich))


def orientation_body(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Preserve four pointer occurrences and the exact non-offset scalar order; e.g. orientation_body(mapping, 19000, ids, options)."""
    raw = member_curve(mapping, version, ids, options)+member_point(mapping, version, ids, options)
    raw += shared_object(mapping, ids, 'member-direction', 'moDirectionSpec_c', lambda: direction(mapping, True)) if options.rich else bytes(2)
    raw += struct.pack('<2id', -2147483648, 2147483647, -0.0)
    raw += member_point(mapping, version, ids, options)+struct.pack('<3id2i', -1, 0, -2147483648, 1.25, 2147483647, -1)
    return raw


def segment_body(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """The segment owns a direct array after four independent doubles; e.g. segment_body(mapping, 19000, ids, options)."""
    raw = member_curve(mapping, version, ids, options)+struct.pack('<3i4d', -2147483648, 2147483647, -1, -0.0, 1.25, -2.5, 3.75)
    return raw+index_array(mapping, ids, options.rich)+struct.pack('<5i', -1, 0, -2147483648, 2147483647, -19)


def segment_pointer(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Reuse one complete segment rather than repeating newly constructed IDs; e.g. segment_pointer(mapping, 19000, ids, options)."""
    return shared_object(mapping, ids, 'member-segment', 'moWeldSegment_c', lambda: segment_body(mapping, version, ids, options))


def segment_array(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Retain segment/null/segment order in each direct array; e.g. segment_array(mapping, 19000, ids, options)."""
    if not options.rich:
        return bytes(2)
    return struct.pack('<H', 3)+segment_pointer(mapping, version, ids, options)+bytes(2)+segment_pointer(mapping, version, ids, options)


def single_body(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Do not prepend the runtime suPtrArray ancestry as an extra wire array; e.g. single_body(mapping, 19000, ids, options)."""
    raw = shared_object(mapping, ids, 'member-orientation', 'moContourOrientation_c', lambda: orientation_body(mapping, version, ids, options)) if options.rich else bytes(2)
    raw += segment_array(mapping, version, ids, options)
    return raw+struct.pack('<2d4i', -0.0, 6.25, -2147483648, 2147483647, -1, 0)


def single_pointer(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Share each complete contour under its tagged object identity; e.g. single_pointer(mapping, 19000, ids, options)."""
    return shared_object(mapping, ids, 'member-single', 'moWeldSingleContour_c', lambda: single_body(mapping, version, ids, options))


def contours_body(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Use a fixed signed contour count, not packed ReadCount; e.g. contours_body(mapping, 19000, ids, options)."""
    raw = struct.pack('<i', 3 if options.rich else 0)
    if options.rich:
        raw += single_pointer(mapping, version, ids, options)+bytes(2)+single_pointer(mapping, version, ids, options)
    return raw+struct.pack('<2d', -0.0, 7.5)


def contours_pointer(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """The positive mode selects a whole tagged contour container; e.g. contours_pointer(mapping, 19000, ids, options)."""
    if not options.rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-contours', 'moWeldmentContours_c', lambda: contours_body(mapping, version, ids, options))
