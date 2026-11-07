"""Complete shared surface targets, list bodies and ordinary feature owners (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_geometric_references import surface
from weld_corner_bytes import counted


def surface_pointer(mapping: WireMap, ids: dict[str,int], version: int, key: str, edited: bool=False) -> bytes:
    """Retain the actual base surface body and an isolated common-word edit; e.g. surface_pointer(mapping,ids,19000,'first',True)."""
    def contents() -> bytes:
        raw=surface(version,False)
        if not edited:return raw
        if raw[6:10]!=struct.pack('<i',-5):
            raise ValueError(f'surface wrapper word {raw[6:10].hex()}; expected complete common prefix and i32 -5')
        return raw[:6]+struct.pack('<i',12345)+raw[10:]
    return shared_object(mapping,ids,key,'moSurfaceRef_w',contents)


def direct_array(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, count: int) -> bytes:
    """Keep owner cycles and complete surface aliases in direct arrays; e.g. direct_array(mapping,ids,19000,2,4,3)."""
    items=[]
    for index in range(count):
        items.append(struct.pack('<H',owner) if index==1 else surface_pointer(mapping,ids,version,'surface-target'))
    return counted(items,style in (4,5))


def nullable_array(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int) -> bytes:
    """Preserve noncanonical signed presence and present-empty arrays; e.g. nullable_array(mapping,ids,19000,1,4)."""
    presence=(0,-2147483648,1,-1,2147483647,255)[style]
    raw=struct.pack('<i',presence)
    return raw+(direct_array(mapping,ids,version,style,owner,(0,0,3,1,2,3)[style]) if presence else b'')


def object_list(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str) -> bytes:
    """Allocate tagged lists while sharing their complete surface entries; e.g. object_list(mapping,ids,19000,5,'list-first')."""
    def contents() -> bytes:
        count=(0,0,2,1,3,2)[style]
        items=[bytes(2) if index==1 else surface_pointer(mapping,ids,version,'surface-target') for index in range(count)]
        return counted(items,style in (4,5))
    return shared_object(mapping,ids,key,'suObList',contents)
