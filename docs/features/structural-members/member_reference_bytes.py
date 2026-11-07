"""Shared complete curve, point, profile and property targets for member fixtures (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object, profile
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from freeze_model_references import wrapper
from reference_plane_fixture_bytes import plane_reference
from plane_point_fixture_bytes import point_body, POINTS
from weldment_fixture_bytes import custom_list, custom_record


@dataclass(frozen=True)
class MemberFixtureOptions:
    rich: bool = False
    mode: int = 0
    point_style: int = 0
    curve_style: int = 0
    edited: bool = False


def member_curve(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Share one complete curve without introducing a kernel-body guess; e.g. member_curve(mapping, 19000, ids, options)."""
    if not options.rich:
        return bytes(2)
    name = ('moGeneralCurveRef_w', 'moLineRef_w')[options.curve_style]
    return shared_object(mapping, ids, 'member-curve', name, lambda: strip_tag(geometric(name, version, False, 0)))


def member_point(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Keep each documented point subtype's full existing body; e.g. member_point(mapping, 19000, ids, options)."""
    if not options.rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-point', POINTS[options.point_style],
        lambda: point_body(mapping, version, ids, options.point_style))


def member_profile(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Use the real component-profile wrapper before its independent raw word; e.g. member_profile(mapping, 19000, ids, options)."""
    if not options.rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-profile', 'moProfileRef_w', lambda: profile_body(mapping, version, ids, options))


def profile_body(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Use the full shared wrapper followed by one u32, with no inferred profile tag; e.g. profile_body(mapping, 19000, ids, options)."""
    component = profile(mapping, version, ids) if options.rich else bytes(2)
    return component+wrapper(version, False)[2:]+struct.pack('<I', 0xfedcba98)


def member_plane(mapping: WireMap, version: int, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Reuse the complete direct plane-reference layout under its own tag; e.g. member_plane(mapping, 19000, ids, options)."""
    if not options.rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-plane', 'moPlaneRef_w', lambda: plane_reference(version))


def member_properties(mapping: WireMap, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """The member's property list is tagged, unlike the weldment feature's direct list; e.g. member_properties(mapping, ids, options)."""
    if not options.rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-properties', 'moCustomPropertyList', lambda: custom_list(2))


def advanced_body(mapping: WireMap, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Keep tagged property aliases inside a packed direct list; e.g. advanced_body(mapping, ids, options)."""
    if not options.rich:
        return bytes(2)
    first = shared_object(mapping, ids, 'member-property', 'moCustomProperty', lambda: custom_record(2))
    second = shared_object(mapping, ids, 'member-property', 'moCustomProperty', lambda: custom_record(2))
    return struct.pack('<H', 3)+first+bytes(2)+second


def member_advanced(mapping: WireMap, ids: dict[str, int], options: MemberFixtureOptions) -> bytes:
    """Retain the advanced-list object's own identity; e.g. member_advanced(mapping, ids, options)."""
    if not options.rich:
        return bytes(2)
    return shared_object(mapping, ids, 'member-advanced', 'moAdvCusPropList_c', lambda: advanced_body(mapping, ids, options))
