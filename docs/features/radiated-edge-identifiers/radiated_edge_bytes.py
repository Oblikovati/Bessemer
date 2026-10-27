"""Complete independently assembled radiated-edge identifiers and declared-role owners (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object

RADIATED = 'moRadiateEdgeIdRep_c'


def representation(mapping: WireMap, ids: dict[str, int], rich: bool, edited: bool) -> bytes:
    """Reserve self identity before complete inherited references; e.g. representation(mapping, ids, True, False)."""
    def contents() -> bytes:
        feature = struct.pack('<H', ids['radiated']) if rich else bytes(2)
        external = mapping.new('moFR_c', lambda: struct.pack('<HiI', 0, -2147483648, 0xffffffff)) if rich else bytes(2)
        return feature+external+struct.pack('<2i', 12345 if edited else -2147483648, 2147483647)
    return shared_object(mapping, ids, 'radiated', RADIATED, contents)


def identifier_object(style: int, rich: bool, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Exercise fresh and cached objects through the finite representation role; e.g. identifier_object(2, True)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def owner() -> bytes:
        return bytes(4)+representation(mapping, ids, rich, edited)+struct.pack('<i', -32769)
    def array() -> bytes:
        first = mapping.new('moDerivedEdgeIdRep_c', owner)
        second = mapping.new('moDerivedEdgeIdRep_c', owner)
        return struct.pack('<H', 3)+first+second+representation(mapping, ids, rich, edited)
    if style == 0:
        raw = representation(mapping, ids, rich, edited)
    elif style == 1:
        raw = mapping.new('moDerivedEdgeIdRep_c', owner)
    else:
        raw = mapping.new('suObArray', array)
    return raw, mapping.slot+1
