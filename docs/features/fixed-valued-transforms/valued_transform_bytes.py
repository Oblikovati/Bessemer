"""Complete valued-dimension bodies with independently retained fixed words (#86)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from sketch_dimension_fixture_bytes import valued_body, base

CLASSES=('sgValG1Dim','sgValG2Dim','sgValG3Dim','sgOffsetDim')
VERSIONS=(0x27cc,0x2f79,0x2f7a,0x2f9f,0x2fa0,0x42ee,0x42ef,19000,0xffffffff)
PRESENCE=(0,1,2,65535,32768,255)


def fixed_words(style: int, edited: bool=False) -> tuple[int,...]:
    """Keep unsigned high bits and allowed raw flag bits independent; e.g. fixed_words(2)."""
    words=[(index*0x1020304 ^ style*0x87654321)&0xffffffff for index in range(30)]
    words[28]&=0xffffff0f
    if edited:words[0]=12345
    return tuple(words)


def contents(mapping: WireMap, version: int, kind: int, style: int, edited: bool) -> bytes:
    """Reuse the complete suffix and count only its actual new handle objects; e.g. contents(mapping,19000,1,2,False)."""
    rich=style!=0;raw=valued_body(CLASSES[kind],version,rich,mapping.slot)
    if rich and kind!=3:mapping.slot+=4 if kind==1 else 2
    offset=len(base(version));raw=raw[:offset]+struct.pack('<H',PRESENCE[style])+raw[offset+2:]
    if not rich:return raw
    return raw[:offset+2]+struct.pack('<30I',*fixed_words(style,edited))+raw[offset+122:]


def transform_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep shared roots distinct from their contained fixed operators; e.g. transform_object(1,19000,2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:contents(mapping,version,kind,style,edited and key=='first'))
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,mapping.slot+1
