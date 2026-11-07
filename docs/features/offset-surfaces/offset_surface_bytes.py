"""Complete offset surfaces, signed nullable-array data and empty native keepers (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from axis_owner_bytes import geometry_header
from body_feature_fixture_bytes import suffix as body_suffix
from sweep_fixture_bytes import shared_object
from display_dimension_fixture_bytes import body as strip_tag
from freeze_geometric_references import wire as geometric
from revolved_bytes import surface_wire
from thicken_bytes import dimension_body
from weld_corner_bytes import counted
from freeze_component_references import base as component_base

CLASSES=('moOffsetRefSurface_c','moOffsetSurfData_c','moOffsetRefSurfIdKeeper_c')
VERSIONS=(0x27cc,0x2bcc,0x2bcd,0x2f85,0x2f86,19000,0xffffffff)
PRESENCE=(0,1,-1,-2147483648,2147483647,1)
COUNTS=(0,0,3,1,2,4)


def face(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Use a complete single-face reference with strict selected body and direct attributes; e.g. face(mapping, ids, 19000, 5)."""
    return shared_object(mapping,ids,'face','moSingleFaceRef_w',lambda:strip_tag(geometric('moSingleFaceRef_w',version,False,2 if style==5 else 1)))


def offset_data(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Distinguish an absent array from present empty and populated direct arrays; e.g. offset_data(mapping, ids, 19000, 2, False)."""
    raw=struct.pack('<i',PRESENCE[style])
    if PRESENCE[style]:
        items=[face(mapping,ids,version,style) if index!=1 else bytes(2) for index in range(COUNTS[style])]
        raw+=counted(items,style>=2)
    raw+=shared_object(mapping,ids,'dimension','moDisplayDistanceDim_c',lambda:dimension_body(mapping,version)) if style>=2 else bytes(2)
    return raw+struct.pack('<i',12345 if edited else -2147483648)


def data_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool) -> bytes:
    """Reserve distinct data identities while retaining shared face and dimension targets; e.g. data_pointer(mapping, ids, 19000, 2, 'first', False)."""
    return shared_object(mapping,ids,key,CLASSES[1],lambda:offset_data(mapping,ids,version,style,edited and key=='first'))


def surface(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Compose native-bound inline surface framing and nonnull data with optional empty keeper; e.g. surface(mapping, ids, 19000, 3)."""
    keeper=lambda:shared_object(mapping,ids,'keeper',CLASSES[2],lambda:b'')
    raw=geometry_header(mapping,version,style in (3,5),keeper=keeper if style in (3,5) else None)
    raw+=surface_wire(mapping,ids,version,4 if style==5 else style)
    return raw+data_pointer(mapping,ids,version,style,'surface-data',False)


def component_surface(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str) -> bytes:
    """Exercise the existing component-surface reader with its native-bound offset geometry; e.g. component_surface(mapping, ids, 19000, 4, 'first')."""
    raw=component_base(version,False)+struct.pack('<iI',-2147483648,0xffffffff)
    raw+=shared_object(mapping,ids,key,CLASSES[0],lambda:surface(mapping,ids,version,style))
    return raw+struct.pack('<i',2147483647)


def offset_surface_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep complete prelude scope, distinct roots, shared dependencies and repeated aliases; e.g. offset_surface_object(0, 19000, 2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        if kind==1:return data_pointer(mapping,ids,version,style,key,edited)
        if kind==0 and style==4:
            return shared_object(mapping,ids,'component-'+key,'moCompRefSurface_c',lambda:component_surface(mapping,ids,version,style,key))
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:surface(mapping,ids,version,style) if kind==0 else b'')
    def outer() -> bytes:
        seed=mapping.new('moRib_c',lambda:body(mapping,version,False,mapping.slot)+body_suffix(mapping,version,'moRib_c',False)) if kind!=2 else b''
        items=[record('first'),record('second'),record('first')] if style in (2,5) else [record('first')]
        return struct.pack('<H',len(items)+int(bool(seed)))+seed+b''.join(items)
    raw=mapping.new('suObArray',outer) if (kind!=2 and style>=2) or style in (2,5) else record('first')
    return raw,mapping.slot+1


def native_minimum(version: int) -> tuple[bytes,int]:
    """Independently pin the documented null-base minimum with cached required data; e.g. native_minimum(0x2f86)."""
    mapping=WireMap()
    def outer() -> bytes:
        data=mapping.new(CLASSES[1],lambda:bytes(10));data_slot=mapping.slot
        prefix=287+4*int(version>=0x2bcd)+4*int(version>0x2f85)
        surface=mapping.new(CLASSES[0],lambda:bytes(prefix)+struct.pack('<H',data_slot))
        return struct.pack('<H',3)+data+surface+struct.pack('<H',mapping.slot)
    raw=mapping.new('suObArray',outer)
    return raw,mapping.slot+1
