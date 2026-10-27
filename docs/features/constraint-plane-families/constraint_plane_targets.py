"""Complete plane-family wrapper, point and angular-dimension dependencies (#54, #46)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object, endpoint_body, synchronization_body, dimension
from reference_plane_fixture_bytes import plane_reference
from freeze_model_references import wrapper
from projected_line_targets import surface_pointer
from plane_point_fixture_bytes import point_body

FLAGS=(0,1,2,128,254,255)
BITS=(0x8000000000000000,0,0x7ff8000000000042,0x7ff0000000000000,0xfff0000000000000,0xfff8000000001234)


def reference(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Use actual complete wrapper bodies through the common geometry role; e.g. reference(mapping,ids,19000,2)."""
    if style==0:return bytes(2)
    if style==2:return surface_pointer(mapping,ids,version,'geometry-surface')
    choices={1:('moRefWrapper_c',lambda:wrapper(version,False)),
        3:('moEndPointRef_w',lambda:endpoint_body(mapping,version,ids,False)),
        4:('moPlaneRef_w',lambda:plane_reference(version)),
        5:('moLoftSynchRef_c',lambda:synchronization_body(mapping,version,ids,False))}
    name,contents=choices[style]
    return shared_object(mapping,ids,'geometry-reference',name,contents)


def required_surface(mapping: WireMap, ids: dict[str,int], version: int) -> bytes:
    """Keep the nonnull surface reference complete; e.g. required_surface(mapping,ids,19000)."""
    return surface_pointer(mapping,ids,version,'required-surface')


def required_point(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Reuse the verified base, endpoint and loft-sync point family; e.g. required_point(mapping,ids,19000,2)."""
    kind=style%3;names=('moPointRef_w','moEndPointRef_w','moLoftSynchRef_c')
    return shared_object(mapping,ids,'required-point',names[kind],lambda:point_body(mapping,version,ids,kind))


def angular(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Keep null and shared complete angular dimensions; e.g. angular(mapping,ids,19000,3)."""
    return dimension(mapping,version,ids,'moDisplayAngularDim_c') if style else bytes(2)


def external_surface(mapping: WireMap, ids: dict[str,int], version: int, key: str, edited: bool) -> bytes:
    """Exercise the complete surface base through the existing sketch-wrapper field; e.g. external_surface(mapping,ids,19000,'first',True)."""
    return shared_object(mapping,ids,key,'sgExtEnt_c',lambda:surface_pointer(mapping,ids,version,'external-surface-'+key,edited)+bytes(7))
