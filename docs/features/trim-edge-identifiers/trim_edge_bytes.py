"""Complete native-bound left/right trim-edge representations and typed entity owners (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from surface_operation_bytes import trim_payload
from radiated_edge_bytes import representation_references

CLASSES = ('moLeftTrimEdgeIdRep_c','moRightTrimEdgeIdRep_c')
VERSIONS = (0x27cc,0x2b73,0x2fb0,19000,0xffffffff)
WORDS = (-2147483648,2147483647,-1,0,1,-2147483648)


def feature(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Share a complete trim feature while retaining its own chooser owner; e.g. feature(mapping, ids, 19000, 3)."""
    if style < 2:
        return bytes(2)
    def contents() -> bytes:
        owner = mapping.slot;level = 2 if style in (3,5) else 0
        return body(mapping, version, False, owner)+trim_payload(mapping, ids, version, level, owner, False)
    return shared_object(mapping, ids, 'trim-feature', 'moTrimRefSurface_c', contents)


def inherited(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Preserve shared feature and full external-reference identities; e.g. inherited(mapping, ids, 19000, 2)."""
    target = feature(mapping, ids, version, style)
    return representation_references(mapping, ids, target, style != 0)


def source(mapping: WireMap, ids: dict[str, int], version: int, style: int) -> bytes:
    """Keep direct entity wrappers out of the map while allocating their real representations; e.g. source(mapping, ids, 19000, 3)."""
    if style < 2:
        return bytes(2)
    simple: Callable[[],bytes] = lambda: shared_object(mapping, ids, 'source-simple', 'moFromSktEntSurfIdRep_c',
        lambda: inherited(mapping, ids, version, style)+struct.pack('<i', 2147483647))
    if style not in (3,5):
        return simple()
    return shared_object(mapping, ids, 'source-derived', 'moDerivedSurfIdRep_c',
        lambda: inherited(mapping, ids, version, style)+simple()+struct.pack('<i', -2147483648))


def trim_edge_object(kind: int, version: int, style: int, edited: bool = False, first_slot: int = 0,
                     name: str | None = None, source_member: bool = True) -> tuple[bytes, int]:
    """Keep equal roots distinct, shared complete sources and typed entity-ID pointer occurrences; e.g. trim_edge_object(0, 19000, 2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def contents(key: str) -> bytes:
        prefix=inherited(mapping, ids, version, style)
        direct=source(mapping, ids, version, style) if source_member else b''
        return prefix+direct+struct.pack('<i',12345 if edited and key=='first' else WORDS[style])
    def record(key: str) -> bytes:
        return shared_object(mapping, ids, key, name if name is not None else CLASSES[kind], lambda: contents(key))
    def wrapper(key: str) -> bytes:
        return shared_object(mapping, ids, 'entity-'+key, 'moEntityId_c', lambda: record(key))
    def outer() -> bytes:
        first,second=(wrapper('first'),wrapper('second')) if style==5 else (record('first'),record('second'))
        return struct.pack('<H',3)+first+second+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else wrapper('first') if style==4 else record('first')
    return raw,mapping.slot+1
