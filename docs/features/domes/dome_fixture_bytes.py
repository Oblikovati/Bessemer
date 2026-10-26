"""Independent complete dome bodies and nullable pointer combinations (#54)."""
from fillet_fixture_bytes import WireMap, body
from freeze_model_references import wrapper
from sweep_fixture_bytes import shared_object, dimension
import struct


def reference(mapping: WireMap, version: int, identities: dict[str, int]) -> bytes:
    """Keep repeated wrapper occurrences as cached references; e.g. reference(mapping, 19000, identities)."""
    return shared_object(mapping, identities, 'wrapper', 'moRefWrapper_c', lambda: wrapper(version, False))


def face_list(mapping: WireMap, version: int, identities: dict[str, int], rich: bool, owner: int) -> bytes:
    """Use a tagged list with independent direct count and shared entries; e.g. face_list(mapping, 19000, ids, True, 2)."""
    def contents() -> bytes:
        if not rich:
            return bytes(2)
        return struct.pack('<HI', 65535, 3)+reference(mapping, version, identities)+struct.pack('<HH', owner, 0)
    return mapping.new('suObList', contents)


def payload(mapping: WireMap, version: int, identities: dict[str, int], rich: bool,
            mask: int, owner: int, edited: bool) -> bytes:
    """Construct all four independent nullable tags; e.g. payload(mapping, 19000, ids, True, 15, 2, False)."""
    raw = struct.pack('<ii', -2147483648, 2147483647)
    raw += dimension(mapping, version, identities, 'moDisplayDistanceDim_c') if mask & 1 else bytes(2)
    raw += face_list(mapping, version, identities, rich, owner) if mask & 2 else bytes(2)
    for bit in (4, 8):
        raw += reference(mapping, version, identities) if mask & bit else bytes(2)
    return raw+struct.pack('<i', 12345 if edited else -1)


def dome_object(version: int, rich: bool, mask: int, first_slot: int = 0, edited: bool = False) -> tuple[bytes, int]:
    """Build the full inherited body and suffix independently; e.g. dome_object(19000, True, 15, 96)."""
    if not 0 <= mask <= 15 or first_slot < 0:
        raise ValueError(f'dome mask/slot {(mask, first_slot)}; expected mask 0..15 and nonnegative slot')
    mapping = WireMap();mapping.slot = first_slot;identities: dict[str, int] = {}
    def contents() -> bytes:
        owner = mapping.slot
        return body(mapping, version, rich, owner)+payload(mapping, version, identities, rich, mask, owner, edited)
    raw = mapping.new('moDome_c', contents)
    return raw, mapping.slot+1
