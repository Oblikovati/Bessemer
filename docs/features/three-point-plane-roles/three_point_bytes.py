"""Complete three-point planes with actual point-subclass identities (#46)."""
import struct
from functools import partial
from fillet_fixture_bytes import WireMap
from reference_plane_fixture_bytes import base
from plane_point_fixture_bytes import POINTS, point_body
from axis_owner_bytes import geometry_header
from sweep_fixture_bytes import shared_object

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2b83,0x2b84,0x2ba5,0x2ba6,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)


def point(mapping: WireMap, ids: dict[str,int], version: int, kind: int, key: str, edited: bool=False) -> bytes:
    """Retain complete point bodies and real repeated identities; e.g. point(mapping,ids,19000,1,'first')."""
    return shared_object(mapping,ids,key,POINTS[kind],lambda:point_body(mapping,version,ids,kind,edited))


def point_references(mapping: WireMap, ids: dict[str,int], version: int, style: int, first: int, edited: bool) -> bytes:
    """Write three separate occurrences, including null, mixed and shared targets; e.g. point_references(mapping,ids,19000,2,1,False)."""
    if style==0:return bytes(6)
    raw=point(mapping,ids,version,first,'first-point',edited)
    second=point(mapping,ids,version,1 if style in (3,4) else 0,'second-point') if style in (3,4,5) else point(mapping,ids,version,first,'first-point')
    third=point(mapping,ids,version,2,'third-point') if style==4 else point(mapping,ids,version,first,'first-point')
    return raw+second+third


def plane_data(mapping: WireMap, ids: dict[str,int], version: int, style: int, first: int, edited: bool) -> bytes:
    """Reuse the complete existing nondefault plane base; e.g. plane_data(mapping,ids,19000,2,1,False)."""
    return shared_object(mapping,ids,'plane-data','moRefThreePtsData_c',
        lambda:base(version,False)+point_references(mapping,ids,version,style,first,edited))


def plane_parent(mapping: WireMap, ids: dict[str,int], version: int, style: int, first: int, edited: bool) -> bytes:
    """Exercise the unchanged typed plane-data boundary through a complete parent; e.g. plane_parent(mapping,ids,19000,4,0,False)."""
    return shared_object(mapping,ids,'parent-plane','moRefPlane_c',
        lambda:geometry_header(mapping,version,False)+plane_data(mapping,ids,version,style,first,edited))


def three_point_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, point_override: int | None=None) -> tuple[bytes,int]:
    """Use fresh/cached points and complete containing parents; e.g. three_point_object(0,19000,2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    first=point_override if point_override is not None else (0,0,1,2,0,2)[style]
    read=partial(plane_parent if style in (4,5) else plane_data,mapping,ids,version,style,first,edited)
    def outer() -> bytes:
        seed=point(mapping,ids,version,first,'first-point',edited)
        return struct.pack('<H',3)+seed+read()+read()
    raw=mapping.new('suObArray',outer) if style in (2,5) else read()
    return raw,mapping.slot+1
