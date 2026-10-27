"""Complete shared draft references and direct keeper lists, independently assembled (#54)."""
from dataclasses import dataclass
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object, dimension
from sheet_fixture_bytes import component_reference
from body_feature_fixture_bytes import direction
from reference_plane_fixture_bytes import plane_reference


@dataclass(frozen=True)
class DraftOptions:
    style: int = 0
    edited: bool = False


def face(mapping: WireMap, version: int, ids: dict[str, int]) -> bytes:
    """Share the same complete face across arrays and groups; e.g. face(mapping, 19000, ids)."""
    return component_reference(mapping, version, ids, 'moCompFace_c')


def nullable_array(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Keep original signed presence distinct from the packed body; e.g. nullable_array(mapping, 19000, ids, DraftOptions(2))."""
    presence = (0, -1, -2147483648, 0, 1, 2147483647)[options.style]
    if not presence:
        return bytes(4)
    if options.style == 4:
        return struct.pack('<iH', presence, 0)
    first = face(mapping, version, ids)
    count = struct.pack('<HI', 65535, 3) if options.style == 5 else struct.pack('<H', 3)
    return struct.pack('<i', presence)+count+first+bytes(2)+face(mapping, version, ids)


def draft_dimension(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Reuse the complete display dimension and its alias across both gates; e.g. draft_dimension(mapping, 19000, ids, options)."""
    return dimension(mapping, version, ids, 'moDisplayAngularDim_c') if options.style else bytes(2)


def draft_direction(mapping: WireMap, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Retain complete independently present direction vectors; e.g. draft_direction(mapping, ids, options)."""
    if options.style == 0:
        return bytes(2)
    return shared_object(mapping, ids, 'draft-direction', 'moDirectionSpec_c', lambda: direction(mapping, True))


def draft_words(mapping: WireMap, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Preserve full unsigned words and escaped direct counts; e.g. draft_words(mapping, ids, DraftOptions(5))."""
    if options.style == 0:
        return bytes(2)
    count = struct.pack('<HI', 65535, 3) if options.style == 5 else struct.pack('<H', 3)
    return shared_object(mapping, ids, 'draft-words', 'su_CWordArray', lambda: count+struct.pack('<3H', 0, 65535, 32768))


def draft_plane(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Use the established complete plane-reference body; e.g. draft_plane(mapping, 19000, ids, options)."""
    if options.style == 0:
        return bytes(2)
    return shared_object(mapping, ids, 'draft-plane', 'moPlaneRef_w', lambda: plane_reference(version))


def entity_ids(mapping: WireMap, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Use signed counts around direct entity-ID pointer records; e.g. entity_ids(mapping, ids, options)."""
    if options.style == 0:
        return bytes(4)
    first = shared_object(mapping, ids, 'draft-representation', 'moSurfaceIdRep_c', lambda: bytes(4))
    repeated = shared_object(mapping, ids, 'draft-representation', 'moSurfaceIdRep_c', lambda: bytes(4))
    return struct.pack('<i', 3)+first+bytes(2)+repeated


def keeper(mapping: WireMap, version: int, ids: dict[str, int], options: DraftOptions) -> bytes:
    """Do not insert a keeper tag into a feature's direct member; e.g. keeper(mapping, 19000, ids, options)."""
    raw = entity_ids(mapping, ids, options)
    if options.style:
        raw += struct.pack('<H', 3)+face(mapping, version, ids)+bytes(2)+face(mapping, version, ids)
    else:
        raw += bytes(2)
    return raw+(entity_ids(mapping, ids, options) if version > 0x3746 else b'')
