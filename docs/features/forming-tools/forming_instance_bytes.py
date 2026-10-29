"""Independent forming-tool instances with direct keepers and ordered body/transform lists (#54)."""
from pathlib import Path
import struct
import sys

FEATURE = Path(__file__).resolve().parent
sys.path.insert(0, str(FEATURE.parent / 'trim-edge-identifiers'))
from freeze_trim_edges import REFERENCE
from body_feature_fixture_bytes import WireMap, header, atom_fields, body_suffix
from profile_fixture_bytes import contents as profile_contents
from owned_name_edit import short_wide_name as text
from freeze_views import transform
from embedded_part_bytes import blank_tool, observed_read
from manager_tag_observation import relocated_tail
from object_fixture_observer import object_cursor


def kept_ids(wire: WireMap, rich: bool) -> bytes:
    """Keep direct IDs and the direct reference array on one map; e.g. kept_ids(wire,True)."""
    if not rich:
        return bytes(6)
    raw = wire.new('moFormFeatureSurfIdRep_c', lambda: struct.pack('<HHii', 2, 0, -2**31, 2**31-1))
    identity = wire.slot
    return struct.pack('<i', 2) + raw + struct.pack('<HHIHH', identity, 65535, 2, identity, 0)


def embedded_tool(wire: WireMap, reference: Path, edited: bool = False) -> bytes:
    """Relocate only observed references when placing a real tool inside an instance."""
    raw = blank_tool(edited=edited)
    cursor = object_cursor(reference, raw, 19000, None)
    _, events = observed_read(raw, cursor.object)
    assert cursor.p == len(raw)
    result = relocated_tail(raw, events, -1, 1, wire.slot)
    wire.slot += len(cursor.map) - 1
    return result


def body_collection(style: int) -> bytes:
    """Retain nonpositive counts and suppressed-body attribute framing; e.g. body_collection(1)."""
    count = [-1, 2, 0, 1, -2**31, 2][style % 6]
    raw = struct.pack('<i', count)
    for index in range(max(0, count)):
        raw += bytes([255 - index]) + struct.pack('<HHH', 2, 2, 0)
    return raw


def transform_entries(style: int) -> bytes:
    """Retain repeated null keys in their wire order; e.g. transform_entries(1)."""
    count = 2 if style % 2 else -1
    raw = struct.pack('<i', count)
    for index in range(max(0, count)):
        raw += bytes(2) + transform(index == 0, [0, 16, 32, 64][(style + index) % 4])
    return raw


def instance_payload(wire: WireMap, version: int, style: int, reference: Path, final_word: int, edited_part: bool) -> bytes:
    """Compose the documented derived sequence after the complete body base."""
    rich = style % 2 == 1
    profile = wire.slot + 2
    raw = wire.new('moProfileFeature_c', lambda: profile_contents(wire, version, False, 0, False, 2))
    raw += struct.pack('<i', -2**31) + transform(rich, [0, 16, 32, 64][style % 4])
    raw += embedded_tool(wire, reference, edited_part) if style == 6 else (wire.new('moFormTool_c', lambda: struct.pack('<Hii', 0, -7, 8)) if rich else bytes(2))
    raw += bytes(2) + text('Ω first') + struct.pack('<iH', -1, 0) + text('Ω second')
    raw += struct.pack('<7i3ddiHii', -1, 2, -3, 4, -5, 6, -7, -0., 1.25, -2.5, 3.75, -8, profile, -9, 10)
    raw += struct.pack('<HI', 65535, 2) + text('Ω duplicate') * 2
    raw += kept_ids(wire, rich) + bytes(2) + body_collection(style) + body_collection(style + 1)
    return raw + transform_entries(style) + struct.pack('<3i', -11, 12, final_word)


def instance_object(version: int, style: int, reference: Path, final_word: int = -13, edited_part: bool = False) -> tuple[bytes, int]:
    """Build a complete fresh instance with no preloaded map entries."""
    wire = WireMap()
    raw = wire.new('moFormToolInstance_c', lambda: header(wire, version, style % 2 == 1)
                   + atom_fields(wire, style % 2 == 1, 2) + body_suffix(style % 2 == 1)
                   + instance_payload(wire, version, style, reference, final_word, edited_part))
    return raw, wire.slot + 1


def representation_object(wrapped: bool) -> tuple[bytes, int]:
    """Verify both the concrete two-word body and its declared entity-ID caller."""
    wire = WireMap()
    def representation() -> bytes:
        return wire.new('moFormFeatureSurfIdRep_c', lambda: struct.pack('<HHii', 0, 0, -2**31, 2**31-1))
    raw = wire.new('moEntityId_c', representation) if wrapped else representation()
    return raw, wire.slot + 1


def owned_instance(kind: int, version: int, style: int, edited: bool, first_slot: int, reference: Path) -> tuple[bytes, int]:
    """Relocate complete actual graphs into the real inherited configuration map."""
    raw, slots = instance_object(version, 6 if kind else style, reference,
                                  123456 if edited and not kind else -13, edited_part=edited and bool(kind))
    cursor = object_cursor(reference, raw, version, None)
    _, events = observed_read(raw, cursor.object)
    assert cursor.p == len(raw) and len(cursor.map) == slots
    return relocated_tail(raw, events, -1, 1, first_slot), slots + first_slot
