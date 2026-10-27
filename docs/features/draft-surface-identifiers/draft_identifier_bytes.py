"""Complete draft/back-draft identifier graphs with real feature and FR targets (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from body_feature_fixture_bytes import suffix
from draft_fixture_bytes import payload
from draft_reference_bytes import DraftOptions
from sweep_fixture_bytes import shared_object
from radiated_edge_bytes import representation_references

CLASSES = ('moDraftSurfIdRep_c', 'moBackDraftSurfIdRep_c')
WORDS = (-2147483648, 0, 2147483647)


def origin_feature(mapping: WireMap, ids: dict[str, int], kind: int, version: int) -> bytes:
    """Keep the complete observed feature target under its own identity; e.g. origin_feature(mapping, ids, 0, 19000)."""
    name = ('moDraft_c', 'moExtrusion_c')[kind]
    def contents() -> bytes:
        inherited = body(mapping, version, False, mapping.slot)
        derived = payload(mapping, version, ids, DraftOptions()) if kind == 0 else suffix(mapping, version, name, False)
        return inherited+derived
    return shared_object(mapping, ids, 'origin-feature', name, contents)


def identifier_object(kind: int, version: int, style: int, word: int, edited: bool = False, first_slot: int = 0) -> tuple[bytes, int]:
    """Keep equal-but-distinct identifiers and shared inherited references; e.g. identifier_object(0, 19000, 2, 0)."""
    mapping = WireMap();mapping.slot = first_slot;ids: dict[str, int] = {}
    def pointer(key: str) -> bytes:
        def contents() -> bytes:
            feature = origin_feature(mapping, ids, kind, version) if style else bytes(2)
            references = representation_references(mapping, ids, feature, style != 0)
            return references+struct.pack('<i', 12345 if edited and key == 'first' else word)
        return shared_object(mapping, ids, key, CLASSES[kind], contents)
    def owner(key: str) -> bytes:
        return mapping.new('moDerivedEdgeIdRep_c', lambda: bytes(4)+pointer(key)+struct.pack('<i', -32769))
    def array() -> bytes:
        feature = origin_feature(mapping, ids, kind, version)
        first = owner('first');second = owner('second')
        return struct.pack('<H', 4)+feature+first+second+pointer('first')
    raw = mapping.new('suObArray', array) if style == 2 else pointer('first')
    return raw, mapping.slot+1
