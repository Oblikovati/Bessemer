"""Complete deleted-face roots and independently nullable data arrays (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from projected_line_targets import nullable_array
from projected_line_bytes import WORDS
from weld_corner_bytes import counted

CLASSES=('moDelFace_c','moDelFaceData_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)


def data_body(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Keep three independent presence words without inventing an owner field; e.g. data_body(mapping,ids,19000,2,4,False)."""
    raw=b''.join(nullable_array(mapping,ids,version,0 if style==0 else (style+2*column)%6,owner) for column in range(3))
    return raw+struct.pack('<i',12345 if edited else WORDS[style])


def data_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool=False) -> bytes:
    """Reserve data identity before nested arrays and their self references; e.g. data_pointer(mapping,ids,19000,2,'nested')."""
    return shared_object(mapping,ids,key,CLASSES[1],lambda:data_body(mapping,ids,version,style,mapping.slot,edited))


def data_array(mapping: WireMap, ids: dict[str,int], version: int, style: int, count: int) -> bytes:
    """Use direct counted bodies with shared complete data, nulls and duplicates; e.g. data_array(mapping,ids,19000,2,3)."""
    entries=[bytes(2) if index==1 else data_pointer(mapping,ids,version,style,'shared-delete-data') for index in range(count)]
    return counted(entries,style in (4,5))


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Retain non-offset word order and independently sized DWORD/object arrays; e.g. payload(mapping,ids,19000,2,4,False)."""
    raw=nullable_array(mapping,ids,version,style,owner)+struct.pack('<i',12345 if edited else WORDS[style])
    raw+=data_array(mapping,ids,version,style,(0,1,3,2,4,3)[style])
    words=(0xffffffff,0,0x89abcdef,0x80000000)[:(0,3,1,4,0,3)[style]]
    raw+=counted([struct.pack('<I',word) for word in words],style in (4,5))
    raw+=struct.pack('<4i',-2147483648,2147483647,-1,0)
    return raw+data_array(mapping,ids,version,style,(0,0,2,3,1,4)[style])


def deleted_face_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Exercise rich feature owners, standalone data and actual shared roots; e.g. deleted_face_object(0,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        change=edited and key=='first'
        if kind==1:return data_pointer(mapping,ids,version,style,key,change)
        def contents() -> bytes:
            owner=mapping.slot
            return body(mapping,version,style in (3,5),owner)+payload(mapping,ids,version,style,owner,change)
        return shared_object(mapping,ids,key,CLASSES[0],contents)
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
