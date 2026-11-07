"""Complete C-392 edge representations with retained direct-member identity (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from trim_edge_bytes import inherited, source, WORDS

CLASSES = ('moTrimWireExtEdgeIdRep_c', 'moPttnBodyEdgeIdRep_c')


def frontier_edge_object(kind: int, version: int, style: int, edited: bool = False,
                         first_slot: int = 0, name: str | None = None) -> tuple[bytes, int]:
    """Retain signed extremes, complete parents and repeated aliases; e.g. frontier_edge_object(1, 19000, 5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def contents(key: str) -> bytes:
        prefix=inherited(mapping,ids,version,style)
        direct=source(mapping,ids,version,style) if kind else b''
        return prefix+direct+struct.pack('<2i',12345 if edited and key=='first' else WORDS[style],WORDS[5-style])
    def record(key: str) -> bytes:
        return shared_object(mapping,ids,key,name if name is not None else CLASSES[kind],lambda: contents(key))
    def wrapper(key: str) -> bytes:
        return shared_object(mapping,ids,'entity-'+key,'moEntityId_c',lambda: record(key))
    def outer() -> bytes:
        first,second=(wrapper('first'),wrapper('second')) if style==5 else (record('first'),record('second'))
        return struct.pack('<H',3)+first+second+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else wrapper('first') if style==4 else record('first')
    return raw,mapping.slot+1
