"""Complete component-curve, blend-reference surface and ordinary keeper graphs (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from component_curve_bytes import curve_pointer
from freeze_model_references import wrapper
from freeze_component_references import base as component_base
from axis_owner_bytes import geometry_header
from revolved_bytes import surface_wire
from blend_fixture_bytes import BlendFixtureOptions,blend_data

CLASSES=('moCompRefCurve_c','moBlendRefSurface_c','moBlendRefSurfIdKeeper_c')
VERSIONS=(0x27cc,0x2b60,0x2b61,0x2bb8,0x2bb9,0x2bcc,0x2bcd,0x2f55,0x2f56,0x2f85,0x2f86,0x3ef5,0x3ef6,19000,0xffffffff)
FLAGS=(0,1,2,128,254,255)


def surface_data(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Keep the direct shared blend data without the blend/cut dimension-pointer suffix; e.g. surface_data(mapping, ids, 19000, 2, False)."""
    options=BlendFixtureOptions(rich=style>=2,count=(0,1,2,3,2,1)[style],declared=(0,2,2147483647,1,0,3)[style],first=258 if style>=2 else 0,second=-254 if style>=2 else 0,dimensions=255 if style>=2 else 0,selector=3)
    raw=blend_data(mapping,version,ids,options)
    return raw+bytes([123 if edited else FLAGS[style],1,128,254,255])


def surface(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Compose native inline framing, an actual empty keeper and direct blend members; e.g. surface(mapping, ids, 19000, 3, False)."""
    keeper=lambda:shared_object(mapping,ids,'surface-keeper',CLASSES[2],lambda:b'')
    raw=geometry_header(mapping,version,style in (3,5),keeper=keeper if style in (3,5) else None)
    raw+=surface_wire(mapping,ids,version,4 if style==5 else style)
    return raw+surface_data(mapping,ids,version,style,edited)


def geometry_owner(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str) -> bytes:
    """Use the existing component-surface role with its proven blend-reference descendant; e.g. geometry_owner(mapping, ids, 19000, 4, 'first')."""
    raw=component_base(version,False)+struct.pack('<iI',-2147483648,0xffffffff)
    raw+=shared_object(mapping,ids,key,CLASSES[1],lambda:surface(mapping,ids,version,style,False))
    return raw+struct.pack('<i',2147483647)


def blend_reference_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Preserve exact native spellings, distinct roots and typed wrapper/geometry callers; e.g. blend_reference_object(1, 19000, 2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        if kind==0:
            words=(2147483647,0x80000000) if style==1 else (-2147483648,0xffffffff)
            return curve_pointer(mapping,ids,version,style>=2,words,'curve-'+key,edited,CLASSES[0])
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:surface(mapping,ids,version,style,edited and key=='first') if kind==1 else b'')
    def wrapped() -> bytes:
        if kind==0:return record('first')+wrapper(version,False)[2:]
        return geometry_owner(mapping,ids,version,style,'surface-first')
    def outer() -> bytes:
        if kind==0 and style==5:
            first=record('first');owner=mapping.new('moRefWrapper_c',wrapped)
            return struct.pack('<H',4)+first+owner+record('second')+record('first')
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    if style==4 and kind!=2:
        raw=mapping.new('moRefWrapper_c' if kind==0 else 'moCompRefSurface_c',wrapped)
    else:raw=mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
