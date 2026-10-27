"""Complete direct net-curve/centerline framing and native thin-dimension branches (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from display_dimension_fixture_bytes import shared
from freeze_model_references import wrapper
from net_component_bytes import WORDS, contact, group, multi_edge

CLASSES=('moNetBlend_c','moNetBlendCut_c','moContactData_c','moCompEntGroup_c','moCompMultiEdge_c')
VERSIONS=(0x27cc,0x2b60,0x2b61,0x2b72,0x2b73,0x2bb8,0x2bb9,0x2f4b,0x2f4c,0x2f55,0x2f56,0x2faf,0x2fb0,0x3b01,0x3b02,0x3ef5,0x3ef6,19000,0xffffffff)
CONTACTS=(0,0,2,1,3,2)
SECOND=(0,2,-2147483648,3,-1,1)
THIN=(0,0,1,2,128,255)
MODES=(3,3,2,3,3,-2147483648)
POINT=struct.pack('<3Q',0x8000000000000000,0x7ff800000000002a,0x7ff0000000000000)


def dwords(style: int, column: int) -> bytes:
    """Keep independent ReadCount framing and raw high bits; e.g. dwords(4, 2)."""
    count=(0,2,3,1,2,4)[style]
    prefix=struct.pack('<HI',65535,count) if style in (4,5) else struct.pack('<H',count)
    words=(0xffffffff,0,0x89abcdef,0x80000000)
    return prefix+b''.join(struct.pack('<I',words[(i+column)%4]) for i in range(count))


def curve_options(style: int, centerline: bool) -> bytes:
    """Preserve raw options instead of performing native runtime migration; e.g. curve_options(2, False)."""
    if not centerline:return struct.pack('<BBiii',255,128 if style==2 else 254,-2147483648,WORDS[style],2147483647)
    return struct.pack('<ii4B3Q2BQi',WORDS[style],WORDS[5-style],0,1,128,255,0x8000000000000000,0x7ff800000000002a,1,254,255,0x7ff000000000002a,-2147483648)


def curve_set(mapping: WireMap, ids: dict[str,int], version: int, style: int, centerline: bool) -> bytes:
    """Keep N nested arrays and N points separate from independently signed M; e.g. curve_set(mapping, ids, 19000, 1, False)."""
    count=CONTACTS[style]
    contacts=b''.join(contact(mapping,ids,version,style) for _ in range(count))
    arrays=b''
    for index in range(count):
        items=[contact(mapping,ids,version,style),bytes(2)] if index%2==0 else []
        arrays+=struct.pack('<H',len(items))+b''.join(items)
    points=POINT*count+struct.pack('<i',SECOND[style])+POINT*max(0,SECOND[style])
    words=b''.join(dwords(style,column) for column in range(3))+struct.pack('<4I',0xffffffff,0x80000000,0x89abcdef,0)
    return struct.pack('<H',count)+contacts+arrays+points+words+curve_options(style,centerline)


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Keep direct data and degree gate separate from tagged conditional dimensions; e.g. payload(mapping, ids, 19000, 4, False)."""
    raw=b''.join(curve_set(mapping,ids,version,style,center) for center in (False,False,True))
    raw+=struct.pack('<BBBiB2Qi',255,128,254,WORDS[style],THIN[style],0x8000000000000000,0x7ff000000000002a,MODES[style])
    raw+=struct.pack('<i',12345 if edited else -2147483648)
    if version>0x2f4b:raw+=struct.pack('<3i',0x12345678,-2147483648,2147483647)
    def dimension() -> bytes:
        return shared_object(mapping,ids,'dimension','moDisplayDim_c',lambda:shared(mapping,version,'moDisplayDim_c',False,0))
    if THIN[style]:raw+=dimension()
    if THIN[style] and MODES[style]==3:raw+=dimension()
    return raw


def net_blend_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve true owners before complete inherited bodies and cyclic dependencies; e.g. net_blend_object(0, 19000, 5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        change=edited and key=='first'
        if kind==2:return contact(mapping,ids,version,style,key)
        if kind==3:return group(mapping,ids,version,style,key,change)
        if kind==4:return multi_edge(mapping,ids,version,style,key,change)
        def contents() -> bytes:
            owner=mapping.slot
            return body(mapping,version,style in (3,5),owner)+payload(mapping,ids,version,style,change)
        return shared_object(mapping,ids,key,CLASSES[kind],contents)
    def wrapped(key: str) -> bytes:
        return shared_object(mapping,ids,'wrapped-'+key,'moRefWrapper_c',lambda:record(key)+wrapper(version,False)[2:])
    def outer() -> bytes:
        first,second=(wrapped('first'),wrapped('second')) if kind>=3 and style==4 else (record('first'),record('second'))
        return struct.pack('<H',3)+first+second+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,4,5) else record('first')
    return raw,mapping.slot+1
