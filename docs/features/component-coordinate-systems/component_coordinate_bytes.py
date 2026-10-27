"""Complete component-coordinate references with the fixed 30-word operator (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_component_references import base
from freeze_model_references import wrapper
from axis_owner_bytes import geometry_header
from coordinate_fixture_bytes import payload


def fixed_words(rich: bool, kind: int, edited: bool = False) -> list[int]:
    """Preserve all fixed words without float normalization; e.g. fixed_words(True, 11)."""
    words = [0xffffffff, 0x80000000]+list(range(2, 30)) if rich else [0]*30
    words[28] = (0x12340000 if rich else 0)+kind
    if edited:
        words[0] = 12345
    return words


def coordinate_target(mapping: WireMap, version: int, rich: bool) -> bytes:
    """Build the complete derived geometry with its own identities; e.g. coordinate_target(mapping, 19000, True)."""
    return mapping.new('moCoordSys_c', lambda: geometry_header(mapping, version, rich)
        +payload(mapping, version, {}, rich, (3, 1), 2, False))


def component_coordinate_body(mapping: WireMap, version: int, rich: bool, mode: int,
                              kind: int, edited: bool, target: bytes | None) -> bytes:
    """Emit inherited fields before the fixed suffix; e.g. component_coordinate_body(mapping, 19000, True, 1, 11, False, None)."""
    raw = base(version, rich)+struct.pack('<iI', -2147483648, 0xfedcba98)
    raw += target if target is not None else (coordinate_target(mapping, version, rich) if mode else bytes(2))
    return raw+(struct.pack('<30I', *fixed_words(rich, kind, edited)) if version > 0x3b27 else b'')


def component_coordinate_container(mapping: WireMap, version: int, rich: bool, mode: int,
                                   kind: int, edited: bool) -> bytes:
    """Keep each containing class's own framing; e.g. component_coordinate_container(mapping, 19000, True, 2, 11, False)."""
    def component(target: bytes | None = None) -> bytes:
        return mapping.new('moCompCoordSys_c', lambda: component_coordinate_body(mapping, version, rich, mode, kind, edited, target))
    def array() -> bytes:
        slot = mapping.slot+2;first = coordinate_target(mapping, version, rich)
        return struct.pack('<H', 2)+first+component(struct.pack('<H', slot))
    if mode == 2:
        return mapping.new('suObArray', array)
    if mode == 3:
        return mapping.new('moRefWrapper_c', lambda: component()+wrapper(version, False)[2:])
    return component()


def component_coordinate_object(version: int, rich: bool, mode: int, first_slot: int = 0,
                                kind: int = 11, edited: bool = False) -> tuple[bytes, int]:
    """Construct null/fresh/cached/wrapped references; e.g. component_coordinate_object(19000, True, 3, 96)."""
    if mode not in range(4) or first_slot < 0 or kind not in range(256):
        raise ValueError(f'component coordinate fixture {(mode, first_slot, kind)}; expected mode 0..3, nonnegative slot and u8 kind')
    mapping = WireMap();mapping.slot = first_slot
    raw = component_coordinate_container(mapping, version, rich, mode, kind, edited)
    return raw, mapping.slot+1
