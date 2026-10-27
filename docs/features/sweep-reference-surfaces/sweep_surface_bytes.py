"""Independent complete native-bound sweep surfaces and edge helpers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import SweepOptions, sweep_data, shared_object, dimension, endpoint, synchronization
from axis_owner_bytes import geometry_header
from revolved_bytes import surface_wire
from freeze_component_references import base as component_base

CLASSES=('moSweepRefSurface_c','moEdgeData_c','moSweepRefSurfIdKeeper_c')
VERSIONS=(0x27cc,0x27d9,0x27da,0x2bcc,0x2bcd,0x2f85,0x2f86,19000,0xffffffff)
COUNTS=(0,-1,3,-2147483648,4,2)


def edge_body(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Retain three endpoint occurrences and native subclass identity; e.g. edge_body(mapping,ids,19000,5)."""
    if style<2:
        return bytes(6)
    first=endpoint(mapping,version,ids,True)
    second=synchronization(mapping,version,ids,True) if style in (3,5) else bytes(2)
    return first+second+endpoint(mapping,version,ids,True)


def edge_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Share complete edge-data targets across consumed pointer occurrences; e.g. edge_pointer(mapping,ids,19000,2)."""
    return shared_object(mapping,ids,'edge-data',CLASSES[1],lambda:edge_body(mapping,ids,version,style))


def direct_data(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Reuse the complete sweep body and isolate its versioned final word; e.g. direct_data(mapping,ids,19000,2,True)."""
    options=SweepOptions(enabled=(0,1,128,255,2,254)[style],radial=(0,-1,1,-2147483648,0,2147483647)[style],
                         angular=(7,8,9,8,-1,9)[style],first_direction=style>=2,second_direction=style==5)
    raw=sweep_data(mapping,version,ids,style>=2,options)
    if not edited:
        return raw
    if version<=0x27d9 or raw[-4:]!=struct.pack('<i',-6):
        raise ValueError(f'sweep data version/tail {(version,raw[-4:])}; expected modern final i32 -6')
    return raw[:-4]+struct.pack('<i',12345)


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Keep surface suffix ordering independent of ordinary sweep suffixes; e.g. payload(mapping,ids,19000,2,False)."""
    raw=direct_data(mapping,ids,version,style,edited)+bytes([0,1,128,254,255])
    if style in (1,2,3,5):
        raw+=dimension(mapping,version,ids,'moDisplayAngularDim_c')+dimension(mapping,version,ids,'moDisplayAngularDim_c')
    count=COUNTS[style];raw+=struct.pack('<i',count)
    for index in range(max(0,count)):
        raw+=bytes(2) if index==1 else edge_pointer(mapping,ids,version,style)
    if style in (1,2,3,5):
        raw+=dimension(mapping,version,ids,'moDisplayRadialDim_c')
    return raw


def surface(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Use the complete reference geometry, inline surface and actual inherited keeper; e.g. surface(mapping,ids,19000,3,False)."""
    keeper=lambda:shared_object(mapping,ids,'keeper',CLASSES[2],lambda:b'')
    raw=geometry_header(mapping,version,style in (3,5),keeper=keeper if style in (3,5) else None)
    raw+=surface_wire(mapping,ids,version,3 if style in (3,5) else 0)
    return raw+payload(mapping,ids,version,style,edited)


def sweep_surface_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep distinct equal roots, repeated aliases and typed component owners; e.g. sweep_surface_object(0,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def contents(key: str) -> bytes:
        if kind==0:return surface(mapping,ids,version,style,edited and key=='first')
        if kind==1:
            raw=edge_body(mapping,ids,version,style)
            return raw
        return b''
    def record(key: str) -> bytes:
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:contents(key))
    def owner() -> bytes:
        return component_base(version,False)+struct.pack('<iI',-2147483648,0xffffffff)+record('first')+struct.pack('<i',2147483647)
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('moCompRefSurface_c',owner) if kind==0 and style==4 else mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
