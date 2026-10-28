"""Complete modern isocurve information and actual external-entity callers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from owned_name_edit import short_wide_name

RATIOS=(0,0x8000000000000000,0x7ff8000000001234,0x7ff0000000000042,0x7ff0000000000000,
        0xfff0000000000000,0x3ff0000000000000,0xbff0000000000000,0x3fc5555555555555)
WORDS=(0,65535,32768,1,65534,32767,2,255,128)
IDS=(-2147483648,2147483647,-1,0,1,-32769,65536,-12345,0x12345678)
VERSIONS=(0x27cc,19000,0xffffffff)


def body(kind: int, edited: bool) -> bytes:
    """Keep the signed ID at unaligned wire offset 14 and the hole word last; e.g. body(2,False)."""
    words=[WORDS[(kind+column)%len(WORDS)] for column in range(4)]
    return struct.pack('<Q3HiH',RATIOS[kind],*words[:3],12345 if edited else IDS[kind],words[3])


def isocurve_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep fresh/cached/null parent fields and real repeated objects; e.g. isocurve_object(3,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def information(key: str) -> bytes:
        return shared_object(mapping,ids,'info-'+key,'moSkIsoCurveInfo_c',lambda:body(kind,edited and key=='first'))
    def parent(key: str) -> bytes:
        def contents() -> bytes:
            return bytes(6)+(bytes(2) if style==4 else information(key))+short_wide_name('Ω isocurve axis')
        return shared_object(mapping,ids,'parent-'+key,'sgExtEnt_c',contents)
    def array() -> bytes:
        if style==2:items=[information('first'),information('second'),information('first')]
        elif style==3:items=[information('first'),parent('first')]
        else:items=[information('first'),parent('first'),information('second'),parent('second'),parent('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,3,5) else parent('first') if style in (1,4) else information('first')
    return raw,mapping.slot+1
