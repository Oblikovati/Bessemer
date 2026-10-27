"""Independent C-392 feature layers and complete edge/radius helper graphs (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from body_feature_fixture_bytes import suffix
from freeze_environment import feature
from freeze_component_references import wire as component
from display_dimension_fixture_bytes import shared, body as tagged_body
from sweep_fixture_bytes import shared_object
from owned_name_edit import short_wide_name

CLASSES=('moLockFeature_c','moSMEnvTableFeat_c','moLPatternItemId_c','edgeParamsObject_c','featRadiusObject_c')
VERSIONS=(0x27cc,0x27df,0x27e0,0x2b60,0x2b61,0x2bb8,0x2bb9,0x2f55,0x2f56,0x3ef5,0x3ef6,19000,0xffffffff)
WORDS=(0,2147483647,-2147483648,-1,1,-2147483648)
BITS=(0,0x8000000000000000,0x7ff0000000000000,0xfff0000000000000,0x7ff800000000002a,0x7ff000000000002a)
FLAGS=(0,1,2,128,254,255)


def dimension(mapping: WireMap, ids: dict[str,int], version: int, key: str) -> bytes:
    """Share complete display dimensions rather than fabricated map targets; e.g. dimension(mapping, ids, 19000, 'dimension')."""
    return shared_object(mapping,ids,key,'moDisplayDim_c',lambda:shared(mapping,version,'moDisplayDim_c',False,0))


def edge(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Retain complete component-edge branches and shared identity; e.g. edge(mapping, ids, 19000, 3)."""
    return shared_object(mapping,ids,'edge','moCompEdge_c',lambda:tagged_body(component('moCompEdge_c',version,False,8 if style in (3,5) else 0)))


def arbitrary(mapping: WireMap, ids: dict[str,int], version: int, style: int, outer: int) -> bytes:
    """Keep the radius CObject pointer unqualified, including a cached containing array; e.g. arbitrary(mapping, ids, 19000, 5, 2)."""
    if style==0:return bytes(2)
    if style==5:return struct.pack('<H',outer)
    if style==3:return edge(mapping,ids,version,style)
    if style==4:return dimension(mapping,ids,version,'dimension')
    return shared_object(mapping,ids,'arbitrary','moNodeName_c',lambda:short_wide_name('Ω arbitrary object'))


def helper_contents(mapping: WireMap, ids: dict[str,int], version: int, kind: int,
                    style: int, edited: bool, outer: int) -> bytes:
    """Keep concrete helper order and raw widths independent of Bessemer; e.g. helper_contents(mapping, ids, 19000, 3, 2, False, 2)."""
    dim=dimension(mapping,ids,version,'dimension') if style else bytes(2)
    if kind==4:return dim+arbitrary(mapping,ids,version,style,outer)
    target=edge(mapping,ids,version,style) if style>=2 else bytes(2)
    first=dimension(mapping,ids,version,'second-dimension' if style==4 else 'dimension') if style else bytes(2)
    last=dimension(mapping,ids,version,'dimension') if style else bytes(2)
    return dim+target+struct.pack('<i',12345 if edited else WORDS[style])+first+last


def contents(mapping: WireMap, ids: dict[str,int], version: int, kind: int,
             style: int, edited: bool, outer: int) -> bytes:
    """Use complete inherited feature bytes and the literal 17-byte item; e.g. contents(mapping, ids, 19000, 2, 5, False, 2)."""
    if kind>=3:return helper_contents(mapping,ids,version,kind,style,edited,outer)
    if kind==2:return struct.pack('<iiBQ',12345 if edited else WORDS[style],WORDS[5-style],FLAGS[style],BITS[style])
    mapping.slot+=2
    raw=feature(version,style in (3,5),word_290=12345 if edited and kind==0 else -3)
    return raw+(struct.pack('<i',12345 if edited else WORDS[style]) if kind==1 else b'')


def frontier_helper_object(kind: int, version: int, style: int, edited: bool=False,
                           first_slot: int=0) -> tuple[bytes,int]:
    """Preserve distinct equal roots, shared targets and complete reader-installing prelude; e.g. frontier_helper_object(4, 19000, 5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={};outer=first_slot+2
    def record(key: str) -> bytes:
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:contents(mapping,ids,version,kind,style,edited and key=='first',outer))
    def array() -> bytes:
        seed=mapping.new('moRib_c',lambda:body(mapping,version,False,mapping.slot)+suffix(mapping,version,'moRib_c',False)) if kind>=3 and style else b''
        items=[record('first'),record('second'),record('first')] if style in (2,4,5) else [record('first')]
        return struct.pack('<H',len(items)+int(bool(seed)))+seed+b''.join(items)
    raw=mapping.new('suObArray',array) if style in (2,4,5) or (kind>=3 and style) else record('first')
    return raw,mapping.slot+1
