"""Exact weld-symbol bodies with directly owned text and native subtype callers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from detail_reference_bytes import detail_fields,WORDS
from freeze_component_references import base
from display_dimension_fixture_bytes import body as strip_tag
from dimension_fixture_bytes import attachment
from freeze_sketch_text import wire as text_wire
from freeze_detail_prefix import character
from owned_name_edit import short_wide_name

CLASSES=('moWeldSymbol_c','moCosmeticWeldBeadSymbol_c','moSFSymbol_c','moCosmeticWeldBeadAttachPoint_c')
VERSIONS=(0x27cc,0x3eec,0x3eed,0x42de,0x42df,0x46bf,0x46c0,19000,0xffffffff)
BITS=(0x8000000000000000,0x7ff0000000000042,0x7ff0000000000000,0xfff0000000000000)


def strings(count: int, rich: bool) -> bytes:
    """Retain ordered empty, ANSI and Unicode strings; e.g. strings(18,True)."""
    return b''.join(short_wide_name(f'Ω symbol {i}\0') if rich and i%2 else b'\x03abc' if rich else bytes(1) for i in range(count))


def direct_text(mapping: WireMap, ids: dict[str,int], version: int, style: int, ordinal: int) -> bytes:
    """Only a real shared font gets map slots inside the direct text; e.g. direct_text(mapping,ids,19000,3,1)."""
    if style==0:return bytes(129)
    raw=strip_tag(text_wire(version,(style+ordinal)%4,(style+ordinal)%4,False))
    if style%2==0:return raw
    offset=len(short_wide_name('<RTL>Ω sketch text'))+28
    font=shared_object(mapping,ids,'font','utCharFormat_c',lambda:character(version))
    return raw[:offset]+font+raw[offset+2:]


def feature_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Use a complete native component-feature suffix for the cosmetic attachment; e.g. feature_pointer(mapping,ids,19000,1)."""
    if style==0:return bytes(2)
    return shared_object(mapping,ids,'feature','moCompFeature_c',lambda:base(version,False)+struct.pack('<iI',-7,0xffffffff))


def surface_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Share one complete finish symbol between both weld pointers; e.g. surface_pointer(mapping,ids,19000,3)."""
    if style==0:return bytes(2)
    return shared_object(mapping,ids,'surface','moSFSymbol_c',lambda:surface_fields(mapping,ids,version,style,ids['surface'],False))


def surface_fields(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Retain strict greater-than finish suffix gates around one direct text; e.g. surface_fields(mapping,ids,0x42df,1,2,False)."""
    raw=bytes(35) if style==0 else detail_fields(mapping,ids,style,owner)
    raw+=struct.pack('<2i',12345 if edited else WORDS[style],-1)+strings(10,style!=0)+struct.pack('<i',2147483647)
    raw+=direct_text(mapping,ids,version,style,0)+struct.pack('<i',-2147483648)
    if version>0x42de:raw+=struct.pack('<iQ',-32769,BITS[style%4])
    if version>0x46bf:raw+=strings(2,style!=0)+struct.pack('<2i',-1,2147483647)
    return raw


def weld_fields(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Compose the complete common weld without outer tags on three texts; e.g. weld_fields(mapping,ids,19000,1,2,False)."""
    raw=bytes(35) if style==0 else detail_fields(mapping,ids,style,owner)
    raw+=strings(18,style!=0)+struct.pack('<4QH',BITS[style%4],*BITS[:3],65535)
    raw+=struct.pack('<4i',12345 if edited else WORDS[style],2147483647,-1,0)+strings(5,style!=0)+struct.pack('<2H',32768,65535)
    raw+=surface_pointer(mapping,ids,version,style)+surface_pointer(mapping,ids,version,style)
    return raw+b''.join(direct_text(mapping,ids,version,style,i) for i in range(3))


def symbol_fields(mapping: WireMap, ids: dict[str,int], version: int, kind: int, style: int, owner: int, edited: bool) -> bytes:
    """Keep each native class's actual inherited body and suffix; e.g. symbol_fields(mapping,ids,19000,3,1,2,False)."""
    if kind==2:return surface_fields(mapping,ids,version,style,owner,edited)
    if kind==3:
        raw=strip_tag(attachment()) if style else bytes(103)
        return raw+feature_pointer(mapping,ids,version,style)+struct.pack('<Q',0x4045000000000000 if edited else BITS[style%4])
    raw=weld_fields(mapping,ids,version,style,owner,edited)
    return raw+(struct.pack('<4i2Q2i',-2147483648,2147483647,-1,0,*BITS[:2],1,-32769) if kind==1 else b'')


def symbol_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Retain fresh/cached/null subtype callers and actual repeated roots; e.g. symbol_graph(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def symbol(key: str) -> bytes:
        return shared_object(mapping,ids,'symbol-'+key,CLASSES[kind],lambda:symbol_fields(mapping,ids,version,kind,style,ids['symbol-'+key],edited and key=='first'))
    def parent(key: str) -> bytes:
        def contents() -> bytes:
            if kind==3:return detail_fields(mapping,ids,1,ids['parent-'+key],False,lambda:bytes(2) if style==4 else symbol(key))
            first=bytes(2) if style==4 else symbol(key)
            return base(version,False)+first+(bytes(2) if style==4 else symbol(key))+struct.pack('<i',-7)
        return shared_object(mapping,ids,'parent-'+key,'moDetailItem_c' if kind==3 else 'moCompDetailItem_c',contents)
    def seed() -> bytes:
        return feature_pointer(mapping,ids,version,style) if kind==3 else surface_pointer(mapping,ids,version,style)
    def array() -> bytes:
        if style==2:items=[symbol('first'),symbol('second'),symbol('first')]
        elif style==3:items=[seed(),symbol('first')]
        else:items=[seed(),parent('first'),parent('second'),parent('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,3,5) else parent('first') if style in (1,4) else symbol('first')
    return raw,mapping.slot+1
