"""Complete independently assembled thin extrusion/revolution graphs (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from body_feature_fixture_bytes import suffix
from sweep_fixture_bytes import shared_object, dimension
from projected_line_bytes import BITS, WORDS

CLASSES=('moThinSpec_c','moExtruThin_c','moBossThin_c','moCutThin_c','moRevolutionThin_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b00,0x3b01,0x3b02,0x3f04,0x3f05,19000,0xffffffff)
BENDS=((0xffffffff,0,0x80000000),(0x89abcdef,7,0xffffffff),(0xffffffff,0,0x80000000))


def specification(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Keep all three dimensions independent from raw options; e.g. specification(mapping,ids,19000,2,True)."""
    first=dimension(mapping,version,ids,'moDisplayAngularDim_c' if style==4 else 'moDisplayDistanceDim_c') if style in (1,2,4) else bytes(2)
    second=dimension(mapping,version,ids,'moDisplayDistanceDim_c') if style in (1,2,3,4) else bytes(2)
    raw=first+second+struct.pack('<i2Q2i',12345 if edited else WORDS[style],BITS[style],BITS[5-style],WORDS[5-style],-1)
    radial=dimension(mapping,version,ids,'moDisplayRadialDim_c') if style in (1,2,4,5) else bytes(2)
    return raw+radial+struct.pack('<2i',-2147483648,2147483647)


def bend_bytes(style: int) -> bytes:
    """Retain direct fixed-width triples and duplicates; e.g. bend_bytes(2)."""
    triples=BENDS[:(0,1,3,2,0,3)[style]]
    return struct.pack('<H',len(triples))+b''.join(struct.pack('<3I',*triple) for triple in triples)


def thin_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose real inherited bodies with shared specifications; e.g. thin_object(1,19000,2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def spec(change: bool) -> bytes:
        return shared_object(mapping,ids,'shared-thin-spec',CLASSES[0],lambda:specification(mapping,ids,version,style,change))
    def record(key: str) -> bytes:
        def contents() -> bytes:
            if kind==0:return specification(mapping,ids,version,style,edited and key=='first')
            owner=mapping.slot;rich=style in (3,4,5);name='moRevolution_c' if kind==4 else 'moExtrusion_c'
            return body(mapping,version,rich,owner)+suffix(mapping,version,name,rich)+spec(edited and key=='first')+(b'' if kind==4 else bend_bytes(style))
        return shared_object(mapping,ids,key,CLASSES[kind],contents)
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
