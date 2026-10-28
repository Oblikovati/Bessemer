"""Exact top-surface and end-cap identifier graphs (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from trim_edge_bytes import inherited

CLASSES=('moFromSktTopSurfIdRep_c','moEndCapChamferSurfIdRep_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)


def surface_identifier(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep distinct representations, shared inherited targets and typed entity pointers; e.g. surface_identifier(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def representation(key: str) -> bytes:
        def contents() -> bytes:
            first=12345 if edited and key=='first' else WORDS[style]
            words=(first,) if kind==0 else (first,WORDS[(style+1)%len(WORDS)])
            return inherited(mapping,ids,version,style)+struct.pack('<'+'i'*len(words),*words)
        return shared_object(mapping,ids,'representation-'+key,CLASSES[kind],contents)
    def entity(key: str) -> bytes:
        return shared_object(mapping,ids,'entity-'+key,'moEntityId_c',lambda:bytes(2) if style==4 else representation(key))
    def array() -> bytes:
        if style==2:items=[representation('first'),representation('second'),representation('first')]
        elif style==3:items=[representation('first'),entity('first')]
        else:items=[entity('first'),entity('second'),entity('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,3,5) else entity('first') if style in (1,4) else representation('first')
    return raw,mapping.slot+1
