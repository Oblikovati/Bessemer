"""Independent reference-geometry bases and populated axis owners (#46)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from body_feature_fixture_bytes import header, atom_fields
from freeze_fixtures import BASE


def geometry_header(mapping: WireMap, version: int, rich: bool, edited: bool = False, keeper: Callable[[], bytes] | None = None) -> bytes:
    """Retain the common base and its real owner index; e.g. geometry_header(mapping, 19000, True)."""
    owner = mapping.slot
    raw = header(mapping, version, rich)
    if keeper is not None:
        raw = raw[:-2]+struct.pack('<H', 65535)+keeper()
    raw += atom_fields(mapping, rich, owner)
    flags = 0x87654321 if edited else 0x89abcdef
    raw += struct.pack('<3dIiH', 1, -0.0, 3, flags, -2147483648, 0)
    return raw+(struct.pack('<i', 2147483647) if version > 0x2f85 else b'')


def axis_record(mapping: WireMap, version: int, rich: bool, edited: bool = False) -> bytes:
    """Compose the existing complete axis-data body; e.g. axis_record(mapping, 19000, True)."""
    return mapping.new('moRefAxis_c', lambda: geometry_header(mapping, version, rich, edited)
        +mapping.new('moRefAxisData_c', lambda: BASE))


def axis_object(version: int, rich: bool, first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Build complete axes in bare or imported maps; e.g. axis_object(19000, True, 96)."""
    if first_slot < 0:
        raise ValueError(f'axis initial slot {first_slot}; expected a nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot
    raw = axis_record(mapping, version, rich, edited)
    return raw, mapping.slot+1
