"""Distinct attachment-data and inherited parametric-point graphs (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from detail_reference_bytes import detail_fields
from dimension_fixture_bytes import attachment
from display_dimension_fixture_bytes import body as strip_tag
from owned_name_edit import short_wide_name

CLASSES=('moAttachPointData_c','moParametricPoint_c')
VERSIONS=(0x27cc,0x335c,0x335d,19000,0xffffffff)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)


def data_body(style: int, edited: bool) -> bytes:
    """Keep the independent two-point body exactly sixty-six bytes; e.g. data_body(2,False)."""
    words=[BITS[(style+i)%6] for i in range(8)]
    if edited:words[6]=0x4045000000000000
    return struct.pack('<6Q',*words[:6])+bytes(2)+struct.pack('<2Q',*words[6:])


def reference_array(mapping: WireMap, style: int, seed: Callable[[],bytes], point: Callable[[str],bytes]) -> bytes:
    """Allocate each child in wire order before tagging the array; e.g. a cached point remains cached."""
    def contents() -> bytes:
        if style==3:items=[seed(),point('first')]
        elif style==5:items=[seed(),point('first'),point('second'),point('first')]
        else:items=[point('first'),point('second'),point('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    return mapping.new('suObArray',contents)


def attachment_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep actual generic reference identities and typed attachment parents; e.g. attachment_graph(1,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def seed() -> bytes:
        return shared_object(mapping,ids,'seed','moNodeName_c',lambda:short_wide_name('Ω point reference'))
    def contents(key: str) -> bytes:
        if kind==0:
            return data_body(style,edited and key=='first')
        return strip_tag(attachment())+struct.pack('<2Q',BITS[style],BITS[(style+1)%6])+(bytes(2) if style in (0,4) else seed())+struct.pack('<Hi',65535-style,12345 if edited and key=='first' else WORDS[style])
    def point(key: str) -> bytes:
        return shared_object(mapping,ids,'point-'+key,CLASSES[kind],lambda:contents(key))
    def parent() -> bytes:
        return mapping.new('moDetailItem_c',lambda:detail_fields(mapping,ids,1,mapping.slot,False,lambda:bytes(2) if style==4 else point('first')))
    raw=parent() if kind==1 and style in (1,4) else reference_array(mapping,style,seed,point) if style in (1,2,3,5) else point('first')
    return raw,mapping.slot+1
