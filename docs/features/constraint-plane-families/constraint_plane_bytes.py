"""Complete native-bound constrained plane, interface and geometry fixture graphs (#54, #46)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from reference_plane_fixture_bytes import base
from axis_owner_bytes import geometry_header
from constraint_plane_targets import FLAGS, BITS, reference, required_surface, required_point, angular, external_surface

CLASSES=('moPntSurfRefPlnData_c','moConstraintRefplaneData_c','moConstraintMidPlaneRefplaneData_c',
 'moConstraintCoincLineAtAnglePlaneRefplaneData_c','moConstraintCoincLineParallelPlaneRefplaneData_c',
 'moConstraintCoincTwoPointsPerpPlnRefplaneData_c','moConstraintPrllPlnTanOneCylinderRefplaneData_c',
 'moConstraintCoincTwoLinesRefplaneData_c','moConstraintCoincLinePerpPlaneRefplaneData_c',
 'moConstraintRefplaneInterfaceData_c','moRefPlaneGeometry_c','moRefPlaneCoincidentGeom_c',
 'moRefPlaneParallelGeom_c','moRefPlaneMidPlaneGeom_c','moRefPlanePerpendicularGeom_c',
 'moRefPlaneTangentGeom_c','moRefPlaneAngleGeom_c','sgExtEnt_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2b83,0x2b84,0x2ba5,0x2ba6,0x2faf,0x2fb0,0x2f85,0x2f86,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-2147483648)


def geometry_body(mapping: WireMap, ids: dict[str,int], version: int, style: int, kind: int, edited: bool) -> bytes:
    """Retain parameter/point bits and separate raw Boolean bytes; e.g. geometry_body(mapping,ids,19000,2,13,True)."""
    bits=[BITS[style],BITS[5-style],0x8000000000000000,0x7ff8000000001234,0x3ff0000000000000]
    if edited and kind in (10,11,12):bits[0]=0x4045000000000000
    raw=reference(mapping,ids,version,style)+struct.pack('<5Q',*bits)
    if kind==16:raw+=struct.pack('<Q',BITS[style])
    return raw+(bytes([1 if edited else FLAGS[style]]) if kind in (13,14,15,16) else b'')


def geometry_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int, kind: int, key: str, edited: bool=False) -> bytes:
    """Share real geometry instances through tagged interface slots; e.g. geometry_pointer(mapping,ids,19000,2,13,'first')."""
    return shared_object(mapping,ids,key,CLASSES[kind],lambda:geometry_body(mapping,ids,version,style,kind,edited))


def interface_body(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Write exactly three geometry tags, including null/shared/distinct variants; e.g. interface_body(mapping,ids,19000,2,True)."""
    if style==0:return bytes(6)
    first_kind=13 if style==2 else (11,10,13,16,15,14)[style]
    first=geometry_pointer(mapping,ids,version,style,first_kind,'interface-first-geometry',edited)
    second=geometry_pointer(mapping,ids,version,style,12 if style==1 else 16,'interface-second-geometry') if style in (3,5) else bytes(2)
    return first+second+geometry_pointer(mapping,ids,version,style,first_kind,'interface-first-geometry')


def interface_pointer(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Keep nullable interface pointers distinct from direct interface bodies; e.g. interface_pointer(mapping,ids,19000,2)."""
    if style==0:return bytes(2)
    return shared_object(mapping,ids,'constraint-interface',CLASSES[9],lambda:interface_body(mapping,ids,version,style,False))


def data_body(mapping: WireMap, ids: dict[str,int], version: int, style: int, kind: int, edited: bool) -> bytes:
    """Compose the real common plane body without default-point insertion or phantom runtime words; e.g. data_body(mapping,ids,19000,2,0,True)."""
    raw=base(version,False)
    if edited and kind in (1,2,4,5,6):
        offset=len(raw)-23-(2 if version>0x2b83 else 0)-(4 if version>0x2ba5 else 0)
        if raw[offset:offset+4]!=struct.pack('<i',-2147483648):raise ValueError(f'plane word_50 {raw[offset:offset+4].hex()}; expected i32 minimum')
        raw=raw[:offset]+struct.pack('<i',12345)+raw[offset+4:]
    if kind==0:
        raw+=struct.pack('<2i',12345 if edited else WORDS[style],WORDS[5-style])
        raw+=required_surface(mapping,ids,version)+required_point(mapping,ids,version,style)
        return raw+struct.pack('<3i',-2147483648,0,2147483647)
    raw+=interface_pointer(mapping,ids,version,style)
    if kind==3:return raw+struct.pack('<i',12345 if edited else WORDS[style])+angular(mapping,ids,version,style)
    return raw+(struct.pack('<i',12345 if edited else WORDS[style]) if kind in (7,8) else b'')


def plane_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Exercise complete plane parents, tagged bodies and the common-wrapper caller; e.g. plane_object(0,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        change=edited and key=='first'
        if kind==17:return external_surface(mapping,ids,version,key,change)
        def contents() -> bytes:
            if kind<9:return data_body(mapping,ids,version,style,kind,change)
            if kind==9:return interface_body(mapping,ids,version,style,change)
            return geometry_body(mapping,ids,version,style,kind,change)
        return shared_object(mapping,ids,key,CLASSES[kind],contents)
    def parent(key: str) -> bytes:
        return shared_object(mapping,ids,'plane-'+key,'moRefPlane_c',lambda:geometry_header(mapping,version,False)+record(key))
    def outer() -> bytes:
        first,second=(parent('first'),parent('second')) if kind<9 and style==5 else (record('first'),record('second'))
        return struct.pack('<H',3)+first+second+(parent('first') if kind<9 and style==5 else record('first'))
    raw=parent('first') if kind<9 and style==4 else mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
