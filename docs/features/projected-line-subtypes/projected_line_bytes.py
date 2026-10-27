"""Complete projected-line parents, subtype bodies and typed surface callers (#54, #46)."""
import struct
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from body_feature_fixture_bytes import direction
from axis_owner_bytes import geometry_header
from freeze_fixtures import BASE as AXIS_BASE
from weld_corner_bytes import counted
from projected_line_targets import surface_pointer, direct_array, nullable_array, object_list

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x371d,0x371e,0x3b01,0x3b02,0x3ef5,0x3ef6,19000,0xffffffff)
BITS=(0,0x8000000000000000,0x7ff8000000000042,0x7ff0000000000000,0xfff0000000000000,0xfff8000000001234)
WORDS=(-2147483648,2147483647,-1,0,1,-2147483648)


def keeper(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int) -> bytes:
    """Retain direct entity IDs, independent arrays and full-width WORD/DWORD values; e.g. keeper(mapping,ids,19000,2,4)."""
    count=2 if style>=2 else 0;raw=struct.pack('<i',count)
    for _ in range(count):
        raw+=shared_object(mapping,ids,'surface-id','moFromSktEntSurfIdRep_c',lambda:struct.pack('<HHi',owner,0,-2147483648))
    raw+=direct_array(mapping,ids,version,style,owner,(0,0,2,1,3,2)[style])
    raw+=direct_array(mapping,ids,version,style,owner,(0,0,1,3,2,1)[style])
    raw+=counted([struct.pack('<H',n) for n in (65535,32768,0)] if style else [],style in (4,5))
    raw+=direct_array(mapping,ids,version,style,owner,2 if style>=2 else 0)
    raw+=counted([struct.pack('<I',n) for n in (0xffffffff,0x80000000,0)] if style else [],style in (4,5))
    return raw+struct.pack('<2i',-2147483648,2147483647)


def subtype(mapping: WireMap, ids: dict[str,int], version: int, style: int, intersect: bool, owner: int | None, edited: bool) -> bytes:
    """Keep owner identity, nullable presence and the concrete suffix distinct; e.g. subtype(mapping,ids,19000,2,False,4,True)."""
    raw=struct.pack('<H',owner) if owner is not None else pline_owner(mapping,ids,version)
    owner_index=owner if owner is not None else ids['feature-owner']
    raw+=nullable_array(mapping,ids,version,style,owner_index)
    if not intersect:
        raw+=struct.pack('<Q',0x4045000000000000 if edited else BITS[style])
        return raw+(shared_object(mapping,ids,'direction','moDirectionSpec_c',lambda:direction(mapping,True)) if style>=2 else bytes(2))
    raw+=struct.pack('<i',12345 if edited else WORDS[style])
    if style==0:return raw+bytes(6)
    first=object_list(mapping,ids,version,style,'first-list')
    second=object_list(mapping,ids,version,style,'second-list') if style in (3,5) else bytes(2)
    return raw+first+second+object_list(mapping,ids,version,style,'first-list')


def parent(mapping: WireMap, ids: dict[str,int], version: int, style: int, intersect: bool | None, edited: bool) -> bytes:
    """Expose the complete subtype through its required concrete PLine owner; e.g. parent(mapping,ids,19000,2,False,True)."""
    owner=mapping.slot;raw=body(mapping,version,style in (3,5),owner)
    raw+=keeper(mapping,ids,version,style,owner)+struct.pack('<i',-2147483648)
    if intersect is None:return raw+mapping.new('moPLineData_c',lambda:struct.pack('<H',owner))
    name='moPLineIntersect_c' if intersect else 'moPLineDraft_c'
    return raw+mapping.new(name,lambda:subtype(mapping,ids,version,style,intersect,owner,edited))


def pline_owner(mapping: WireMap, ids: dict[str,int], version: int) -> bytes:
    """Share a complete existing PLine owner and its self-owned base data; e.g. pline_owner(mapping,ids,19000)."""
    return shared_object(mapping,ids,'feature-owner','moPLine_c',lambda:parent(mapping,ids,version,0,None,False))


def projected_line_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Exercise complete parents, standalone owned data and an existing typed surface role; e.g. projected_line_object(0,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        change=edited and key=='first'
        if kind==4:return surface_pointer(mapping,ids,version,key,change)
        if kind<2:return shared_object(mapping,ids,key,'moPLine_c',lambda:parent(mapping,ids,version,style,kind==1,change))
        name='moPLineDraft_c' if kind==2 else 'moPLineIntersect_c'
        return shared_object(mapping,ids,key,name,lambda:subtype(mapping,ids,version,style,kind==3,None,change))
    def axis() -> bytes:
        return geometry_header(mapping,version,False)+mapping.new('moSurfaceAxisData_c',lambda:AXIS_BASE+record('first'))
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('moRefAxis_c',axis) if kind==4 and style==4 else mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
