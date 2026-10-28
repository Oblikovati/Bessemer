"""Exact shared non-default plane body and real typed plane callers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from reference_plane_fixture_bytes import base
from axis_owner_bytes import geometry_header

VERSIONS=(0x27cc,0x2b83,0x2b84,0x2ba5,0x2ba6,0x2f85,0x2f86,19000,0xffffffff)


def absolute_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep the proven common body without default-point insertion; e.g. absolute_graph(1,19000,2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def values(key: str) -> bytes:
        raw=base(version,False)
        return struct.pack('<d',42.0)+raw[8:] if edited and key=='first' else raw
    def plane_data(key: str) -> bytes:
        return shared_object(mapping,ids,'data-'+key,'moAbsoluteRefPlnData_c',lambda:values(key))
    def record(key: str) -> bytes:
        if kind==0:return plane_data(key)
        return shared_object(mapping,ids,'plane-'+key,'moRefPlane_c',lambda:geometry_header(mapping,version,style in (3,5))+(bytes(2) if style==4 else plane_data(key)))
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,mapping.slot+1
