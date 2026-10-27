"""Complete ordinary reference curves with independent array branches and strict bodies (#54)."""
import struct
from fillet_fixture_bytes import WireMap,body
from sweep_fixture_bytes import shared_object
from display_dimension_fixture_bytes import body as strip_tag
from freeze_boundary_references import wire as boundary
from freeze_component_references import base as component_base
from freeze_geometric_references import wire as geometric
from body_feature_fixture_bytes import direction
from solid_reference_bytes import body_attributes
from kernel_geometry_fixture import frame
from weld_corner_bytes import counted

VERSIONS=(0x27cc,11000,0x2b87,0x2b88,0x2b94,0x2b95,0x3efb,0x3efc,19000,0xffffffff)
FLAGS=(0,1,1,2,-2147483648,-1)


def face(mapping: WireMap, ids: dict[str,int], version: int) -> bytes:
    """Build a complete shared face reference rather than a loaded-object stub; e.g. face(mapping, ids, 19000)."""
    return shared_object(mapping,ids,'face','moFaceRef_c',lambda:strip_tag(boundary('moFaceRef_c',version,False)))


def references(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Keep null, empty and nonempty array identities separate from branch flags; e.g. references(mapping, ids, 19000, 2)."""
    if style==0:return bytes(2)
    count=2 if style in (2,3) else 0
    return shared_object(mapping,ids,'references','suObArray',lambda:struct.pack('<H',count)+(face(mapping,ids,version)+bytes(2) if count else b''))


def dwords(style: int) -> bytes:
    """Retain DWORD high bits, order and duplicates using packed counts; e.g. dwords(2)."""
    words=(0xffffffff,0,0x80000000,0xffffffff) if style else ()
    return counted([struct.pack('<I',word) for word in words],style!=0)


def plane(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Share the complete component-plane body across both pointer occurrences; e.g. plane(mapping, ids, 19000, 2)."""
    if style==0:return bytes(2)
    return shared_object(mapping,ids,'plane','moCompRefPlane_c',lambda:component_base(version,False)+struct.pack('<iIHi',-2147483648,0xffffffff,0,2147483647))


def suffix(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Keep the raw u16 body count distinct from MFC counts and always write each body's attributes; e.g. suffix(mapping, ids, 19000, 3)."""
    raw=b''
    if version>0x2b87:
        count=(0,0,1,2,0,2)[style];raw+=struct.pack('<H',count)
        for index in range(count):
            raw+=(frame() if style in (3,5) and index==0 else b'\xff')+body_attributes(mapping,ids)
        raw+=struct.pack('<i',-2147483648)+plane(mapping,ids,version,style)
        raw+=shared_object(mapping,ids,'direction','moDirectionSpec_c',lambda:direction(mapping,True)) if style else bytes(2)
    if version>0x3efb:
        raw+=shared_object(mapping,ids,'curve','moGeneralCurveRef_w',lambda:strip_tag(geometric('moGeneralCurveRef_w',version,False,0))) if style else bytes(2)
    return raw


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Select flag, populated-array or gated-plane branches from independently retained inputs; e.g. payload(mapping, ids, 0x2b95, 2, False)."""
    raw=references(mapping,ids,version,style)+(frame() if style in (3,5) else b'\xc3')
    raw+=body_attributes(mapping,ids) if style>=2 else bytes(2)
    raw+=struct.pack('<4i',12345 if edited else -2147483648,2147483647,-1,FLAGS[style])+dwords(style)
    if FLAGS[style]&2:raw+=dwords(style)
    elif style==2:raw+=counted([face(mapping,ids,version),bytes(2),face(mapping,ids,version)],True)
    elif version>0x2b94:raw+=plane(mapping,ids,version,style)
    raw+=dwords(style)+dwords(style)+struct.pack('<iHi',-2147483648,65535,2147483647)
    return raw+suffix(mapping,ids,version,style)


def reference_curve_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Preserve equal feature identities and fresh/cached array facts through containing roots; e.g. reference_curve_object(0, 19000, 2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def contents(key: str) -> bytes:
        owner=mapping.slot
        return body(mapping,version,style in (3,5),owner)+payload(mapping,ids,version,style,edited and key=='first')
    def record(key: str) -> bytes:
        return shared_object(mapping,ids,key,'moRefCurve_c',lambda:contents(key))
    def outer() -> bytes:
        seed=references(mapping,ids,version,style)
        return struct.pack('<H',4)+seed+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,4,5) else record('first')
    return raw,mapping.slot+1
