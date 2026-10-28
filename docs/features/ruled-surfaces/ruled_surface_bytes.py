"""Complete ruled surfaces and tagged/direct feature data with real owners (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from axis_owner_bytes import geometry_header
from revolved_bytes import surface_wire
from sweep_fixture_bytes import dimension, shared_object
from body_feature_fixture_bytes import direction
from weld_corner_bytes import counted

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2bcc,0x2bcd,0x2f85,0x2f86,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,-1,2147483647,0,-32769,65535)
BITS=(0x8000000000000000,0x7ff8000000001234,0x7ff0000000000000,0x7ff0000000004321,1)


def objects(mapping: WireMap, ids: dict[str,int], owner: int, style: int, column: int) -> bytes:
    """Keep independent direct counts, nulls and cached enclosing owners; e.g. objects(mapping,ids,2,5,1)."""
    count=0 if style==0 else (style+column)%4
    entries=[]
    for index in range(count):
        if index==1:entries.append(bytes(2));continue
        if index==2:entries.append(struct.pack('<H',owner));continue
        entries.append(shared_object(mapping,ids,'marker','moPMarkRecord_c',lambda:struct.pack('<i',-2147483648)))
    return counted(entries,style in (4,5))


def display(mapping: WireMap, ids: dict[str,int], version: int, style: int, name: str) -> bytes:
    """Share complete dimensions without assigning meanings to their values; e.g. display(mapping,ids,19000,2,'moDisplayDistanceDim_c')."""
    return bytes(2) if style<2 else dimension(mapping,version,ids,name)


def data_tail(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Retain direct WORD widths and native nonmonotonic scalar order; e.g. data_tail(mapping,ids,19000,5,2,False)."""
    raw=objects(mapping,ids,owner,style,1)
    words=(0xffff,0,0x8000,0x1234)[:(0,3,1,4,0,2)[style]]
    raw+=counted([struct.pack('<H',word) for word in words],style in (4,5))
    raw+=objects(mapping,ids,owner,style,2)
    for name in ('moDisplayDistanceDim_c','moDisplayAngularDim_c'):raw+=display(mapping,ids,version,style,name)
    raw+=struct.pack('<Q4i',BITS[style%5],12345 if edited else WORDS[style],-2147483648,2147483647,-1)
    raw+=bytes(2) if style<2 else shared_object(mapping,ids,'direction','moDirectionSpec_c',lambda:direction(mapping,style in (3,5)))
    return raw+struct.pack('<Qi3Qi',BITS[(style+1)%5],WORDS[(style+1)%6],*(BITS[(style+2+i)%5] for i in range(3)),2147483647)


def surface(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool) -> bytes:
    """Write owned data directly after a complete inline surface base; e.g. surface(mapping,ids,19000,5,'first',False)."""
    def contents() -> bytes:
        owner=mapping.slot
        raw=geometry_header(mapping,version,style in (3,5))+surface_wire(mapping,ids,version,style)
        raw+=objects(mapping,ids,owner,style,0)+struct.pack('<H',owner)
        return raw+data_tail(mapping,ids,version,style,owner,edited and key=='first')
    return shared_object(mapping,ids,key,'moRuledSrfFromEdge_c',contents)


def ruled_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Use complete feature owners for standalone data and keep repeated root identities; e.g. ruled_object(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        if kind==0:return surface(mapping,ids,version,style,key,edited)
        def contents() -> bytes:
            owner_wire=surface(mapping,ids,version,style,'owner',False)
            return owner_wire+data_tail(mapping,ids,version,style,ids['owner'],edited and key=='first')
        return shared_object(mapping,ids,key,'moRuledSrfFromEdgeData_c',contents)
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
