"""Complete composite curves with independently counted direct containers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from helix_bytes import HelixOptions, reference_curve
from weld_corner_bytes import counted

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2f85,0x2f86,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,-1,2147483647,0,-32769,65535)
COUNTS=((0,0,0,0),(3,0,1,2),(1,4,0,3),(4,2,3,0),(0,1,4,2),(3,0,2,4))


def object_array(mapping: WireMap, ids: dict[str,int], owner: int, style: int) -> bytes:
    """Keep a complete shared marker, null and enclosing-owner cycle; e.g. object_array(mapping,ids,2,5)."""
    if style==0:return counted([],False)
    marker=shared_object(mapping,ids,'marker','moPMarkRecord_c',lambda:struct.pack('<i',-2147483648))
    entries=[marker,bytes(2),struct.pack('<H',owner),struct.pack('<H',ids['marker'])]
    return counted(entries[:(0,1,3,2,4,4)[style]],style in (4,5))


def payload(mapping: WireMap, ids: dict[str,int], owner: int, style: int, edited: bool) -> bytes:
    """Retain one tagged array followed by five direct bodies; e.g. payload(mapping,ids,2,5,False)."""
    raw=struct.pack('<i',12345 if edited else WORDS[style])
    raw+=bytes(2) if style==0 else shared_object(mapping,ids,'array','suObArray',lambda:object_array(mapping,ids,owner,style))
    raw+=object_array(mapping,ids,owner,(style+1)%6)
    for column,count in enumerate(COUNTS[style]):
        words=(0xffffffff,0x80000000,0,0x89abcdef)
        raw+=counted([struct.pack('<I',words[(column+index)%4]) for index in range(count)],style in (4,5))
    return raw


def composite_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve complete curve owners before their shared and cyclic references; e.g. composite_object(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    options=HelixOptions(wire=kind==1,rich=style in (3,5))
    def record(key: str) -> bytes:
        def contents() -> bytes:
            owner=mapping.slot
            return reference_curve(mapping,ids,version,options)+payload(mapping,ids,owner,style,edited and key=='first')
        return shared_object(mapping,ids,key,'moCompositeCurve_c',contents)
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
