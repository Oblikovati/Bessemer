"""Complete structural-member feature bodies with independent counted records (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from owned_name_edit import short_wide_name
from member_reference_bytes import (MemberFixtureOptions, member_curve, member_point, member_profile, member_plane,
                                    member_properties, member_advanced, advanced_body, profile_body)
from member_contour_bytes import (index_body, index_array, indices_pointer, orientation_body, single_body,
                                  segment_body, segment_array, contours_body, contours_pointer)

CLASSES = ('moWeldMemberFeat_c', 'moProfileRef_w', 'moWeldmentContours_c', 'moAdvCusPropList_c',
           'moWeldSegSkIndices_c', 'moWeldSegSkIndex_c', 'moContourOrientation_c', 'moWeldSingleContour_c', 'moWeldSegment_c')


def curve_collection(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Keep each signed count and repeated/null curve occurrence; e.g. curve_collection(mapping, 19000, ids, options)."""
    raw = struct.pack('<i', 3 if options.rich else 0)
    if not options.rich:
        return raw
    return raw+member_curve(mapping, version, ids, options)+bytes(2)+member_curve(mapping, version, ids, options)


def point_collection(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Store full i32 metadata before the later native u16 truncation; e.g. point_collection(mapping, 19000, ids, options)."""
    raw = struct.pack('<i', 3 if options.rich else 0)
    if not options.rich:
        return raw
    words = ((12345 if options.edited else -2147483648, 2147483647), (-1, 65536), (-32769, 32768))
    for index, pair in enumerate(words):
        raw += (member_point(mapping, version, ids, options) if index != 2 else bytes(2))+struct.pack('<2i', *pair)
    return raw


def member_payload(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Keep collection grammar separate from the later contour mode; e.g. member_payload(mapping, 19000, ids, options)."""
    raw = member_profile(mapping, version, ids, options)+member_plane(mapping, version, ids, options)
    raw += struct.pack('<2i', -2147483648, 2147483647)+curve_collection(mapping, version, ids, options)
    raw += point_collection(mapping, version, ids, options)+indices_pointer(mapping, ids, options.rich)
    raw += struct.pack('<d', -0.0)+short_wide_name('Member Ω\n@original')
    raw += member_point(mapping, version, ids, options)+member_properties(mapping, ids, options)
    raw += struct.pack('<5i', -2147483648, 2147483647, -1, 0, 65536)+curve_collection(mapping, version, ids, options)
    raw += struct.pack('<di', -1.25, options.mode)
    if options.mode > 0:
        raw += contours_pointer(mapping, version, ids, options)
    return raw+member_tail(mapping, version, ids, options)


def member_tail(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Preserve the direct-array and non-offset string/word continuation; e.g. member_tail(mapping, 19000, ids, options)."""
    raw = segment_array(mapping, version, ids, options)
    raw += short_wide_name('960 Ω')+short_wide_name('968 Δ')+struct.pack('<i', -2147483648)+short_wide_name('970 @')
    raw += struct.pack('<3i', 2147483647, -1, 0)+short_wide_name('980\r\nretained')+struct.pack('<i', -19)
    return raw+member_advanced(mapping, ids, options)


def structural_object(kind: int, version: int, options: MemberFixtureOptions, first_slot: int = 0) -> tuple[bytes, int]:
    """Construct one complete supported class in a detached or inherited map; e.g. structural_object(0, 19000, options, 96)."""
    if kind not in range(9) or first_slot < 0 or options.point_style not in range(3) or options.curve_style not in range(2):
        raise ValueError(f'member fixture {(kind, first_slot, options)}; expected kind 0..8, nonnegative slot, point 0..2 and curve 0..1')
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def complete_member() -> bytes:
        owner = mapping.slot
        return body(mapping, version, options.rich, owner)+member_payload(mapping, version, ids, options)
    readers = (complete_member, lambda: profile_body(mapping, version, ids, options),
        lambda: contours_body(mapping, version, ids, options), lambda: advanced_body(mapping, ids, options),
        lambda: index_array(mapping, ids, options.rich), index_body,
        lambda: orientation_body(mapping, version, ids, options), lambda: single_body(mapping, version, ids, options),
        lambda: segment_body(mapping, version, ids, options))
    raw = mapping.new(CLASSES[kind], readers[kind])
    return raw, mapping.slot+1
