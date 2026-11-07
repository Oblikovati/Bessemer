"""Complete ordinary display-weld graphs with exact raw suffixes (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from display_item_fixture_bytes import object_base,item
from owned_name_edit import short_wide_name

VERSIONS=(0x27cc,0x3371,0x3372,0x3373,0x3374,0x371d,0x371e,19000,0xffffffff)
BITS=((0x8000000000000000,0x3ff4000000000000,0xc004000000000000,0x8000000000000000),
      (0x7ff0000000000042,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000001234),
      (1,0x7fefffffffffffff,0x8000000000000001,0x3fc5555555555555))
WORDS=(-2147483648,2147483647,-1)
STRINGS=(b'\x03abc',short_wide_name('Ω weld\0 😀'),b'\xff\xfe\xff\xff'+struct.pack('<H',260)+'Ω'.encode('utf-16-le')*260)


def suffix(kind: int, edited: bool, pointer: bytes) -> bytes:
    """Retain IEEE bits, signed words and original CString framing; e.g. suffix(1,False,bytes(2))."""
    return struct.pack('<4Qi',*BITS[kind],12345 if edited else WORDS[kind])+STRINGS[kind]+pointer


def display_weld_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep complete inherited bodies and actual shared/self payload pointers; e.g. display_weld_object(2,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def seed() -> bytes:
        return shared_object(mapping,ids,'payload','moNodeName_c',lambda:short_wide_name('Ω shared payload'))
    def contents(key: str) -> bytes:
        base=object_base(mapping,style%2==1,ids['weld-'+key])+item(mapping,version,style%2==1)
        pointer=bytes(2) if style in (0,4) else struct.pack('<H',ids['weld-'+key]) if style==1 else seed()
        return base+suffix(kind,edited and key=='first',pointer)
    def weld(key: str) -> bytes:
        return shared_object(mapping,ids,'weld-'+key,'moDisplayWeld_c',lambda:contents(key))
    def array() -> bytes:
        if style==2:items=[weld('first'),weld('second'),weld('first')]
        elif style==3:items=[seed(),weld('first')]
        else:items=[seed(),weld('first'),weld('second'),weld('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,3,5) else weld('first')
    return raw,mapping.slot+1
