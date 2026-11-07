"""Complete component-axis/point bodies with real inherited map identities (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_component_references import base
from freeze_model_references import wrapper
from axis_owner_bytes import geometry_header, axis_record

NAMES = ('moCompRefAxis_c', 'moCompRefPoint_c')


def geometry(mapping: WireMap, version: int, rich: bool, kind: int) -> bytes:
    """Construct complete plane or axis bodies rather than guessed payloads; e.g. geometry(mapping, 19000, True, 2)."""
    if kind == 0:
        return bytes(2)
    if kind == 1:
        return mapping.new('moRefPlane_c', lambda: geometry_header(mapping, version, rich)+bytes(2))
    return axis_record(mapping, version, rich)


def component_object(name: str, version: int, rich: bool, kind: int, first_slot: int = 0,
                     wrapped: bool = False, edited: bool = False) -> tuple[bytes, int]:
    """Keep the component-feature suffix independent from plane-only fields; e.g. component_object(NAMES[0], 19000, True, 2, 96)."""
    if name not in NAMES or kind not in (0, 1, 2) or first_slot < 0:
        raise ValueError(f'component geometry {(name, kind, first_slot)}; expected known axis/point class, geometry 0..2 and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot
    def contents() -> bytes:
        return base(version, rich)+struct.pack('<iI', 12345 if edited else -2147483648, 0xfedcba98)+geometry(mapping, version, rich, kind)
    def tagged() -> bytes:
        return mapping.new(name, contents)
    raw = mapping.new('moRefWrapper_c', lambda: tagged()+wrapper(version, False)[2:]) if wrapped else tagged()
    return raw, mapping.slot+1
