"""Native-bound cone suffix and shared surface-axis callers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
BITS=(0,0x8000000000000000,0x7ff8000000001234,0x7ff0000000000042,
      0x7ff0000000000000,0xfff0000000000000,0x3ff0000000000000,0xbff0000000000000)


def cone_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Share actual cone pointers across distinct axis data; e.g. cone_object(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def cone(key: str) -> bytes:
        bits=[BITS[(index+kind)%8] for index in range(8)]
        if edited and key=='first':bits[7]=0x4004000000000000
        contents=wrapper(version,False)+struct.pack('<4d',*range(4))+struct.pack('<8Q',*bits)
        return shared_object(mapping,ids,'cone-'+key,'moConeRef_w',lambda:contents)
    def axis(key: str) -> bytes:
        return shared_object(mapping,ids,'axis-'+key,'moSurfaceAxisData_c',lambda:struct.pack('<14d',*range(14))+(bytes(2) if style==4 else cone(key)))
    def array() -> bytes:
        if style==2:items=[cone('first'),cone('second'),cone('first')]
        elif style==3:items=[cone('first'),axis('first')]
        else:items=[cone('first'),axis('first'),cone('second'),axis('second'),axis('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,3,5) else axis('first') if style in (1,4) else cone('first')
    return raw,mapping.slot+1
