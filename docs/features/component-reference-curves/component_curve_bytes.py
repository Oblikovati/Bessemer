"""Complete component-reference curves, unit targets and shared wrapper identities (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from freeze_component_references import base
from freeze_model_references import wrapper
from sweep_fixture_bytes import shared_object

VERSIONS = (0x27cc, 0x2b60, 0x2b61, 0x2bb8, 0x2bb9, 0x2f55, 0x2f56, 0x3ef5, 0x3ef6, 19000, 0xffffffff)
WORDS = ((-2147483648, 0xffffffff), (0, 0), (2147483647, 0x80000000))


def curve_pointer(mapping: WireMap, ids: dict[str, int], version: int, rich: bool, words: tuple[int, int], key: str, edited: bool) -> bytes:
    """Share a complete unit component while retaining distinct feature ID/time pairs; e.g. curve_pointer(mapping, ids, 19000, True, (0, 0), 'curve-first', False)."""
    def contents() -> bytes:
        component = shared_object(mapping, ids, 'unit', 'moUnitComponent_c', lambda: bytes(3)) if rich else bytes(2)
        inherited = component+base(version, rich)[2:]
        return inherited+struct.pack('<iI', 12345 if edited and key == 'curve-first' else words[0], words[1])
    return shared_object(mapping, ids, key, 'moCompReferenceCurve_c', contents)


def component_curve_object(version: int, style: int, rich: bool, words: tuple[int, int], edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep wrapper admission separate from concrete component-feature ancestry; e.g. component_curve_object(19000, 2, True, (0, 0))."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def pointer(key: str) -> bytes:
        return curve_pointer(mapping, ids, version, rich, words, key, edited)
    def owner() -> bytes:
        return mapping.new('moRefWrapper_c', lambda: pointer('curve-first')+wrapper(version, False)[2:])
    def array() -> bytes:
        return struct.pack('<H', 4)+pointer('curve-first')+owner()+pointer('curve-second')+pointer('curve-first')
    raw = mapping.new('suObArray', array) if style == 2 else owner() if style == 1 else pointer('curve-first')
    return raw, mapping.slot+1
