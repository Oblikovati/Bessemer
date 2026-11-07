"""Native member order after the complete reference-curve body (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from helix_bytes import HelixOptions, reference_curve

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2f85,0x2f86,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)


def suffix(style: int, edited: bool) -> bytes:
    """Keep five distinguishable signed values in serializer order; e.g. suffix(2,False)."""
    values=[WORDS[(style+index)%6] for index in range(5)]
    if edited:values[0]=12345
    return struct.pack('<5i',*values)


def cut_list_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Retain real shared inherited objects and repeated roots; e.g. cut_list_graph(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    options=HelixOptions(wire=kind==1,rich=style in (3,5))
    def record(key: str) -> bytes:
        def contents() -> bytes:
            return reference_curve(mapping,ids,version,options)+suffix(style,edited and key=='first')
        return shared_object(mapping,ids,key,'moCosmeticWeldCutList_c',contents)
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,mapping.slot+1
