"""Complete field-specific sketch wrapper regressions from existing body grammars (#46)."""
import struct
from fillet_fixture_bytes import WireMap
from display_dimension_fixture_bytes import body as strip_tag
from freeze_geometric_references import CLASSES as GEOMETRIC, surface, wire as geometric
from freeze_model_references import wrapper
from freeze_views import transform
from reference_plane_fixture_bytes import plane_reference
from sketch_reference_fixture_bytes import external_reference, temporary_axis
from sweep_fixture_bytes import shared_object, endpoint_body, synchronization_body
from solid_reference_bytes import solid_pointer
from owned_name_edit import short_wide_name

CLASSES=('moRefWrapper_c','moGeneralSurfRef_w','moGeneralCurveRef_w','moSingleFaceRef_w',
    'moCircleRef_w','moCylinderRef_w','moSphereRef_w','moPlaneRef_w','moPointRef_w',
    'moEndPointRef_w','moLoftSynchRef_c','moLineRef_w','moLoopRef_w','moSolidRef_w',
    'moConstSurfRef_w','moSketchExtRef_w','moTempAxisRef_w','moCoordSysRef_w')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x371d,0x371e,0x3b01,0x3b02,19000,0xffffffff)


def payload(mapping: WireMap, ids: dict[str,int], version: int, name: str, style: int) -> bytes:
    """Reuse exact complete bodies instead of renaming unrelated wire layouts; e.g. payload(mapping, ids, 19000, 'moPlaneRef_w', 0)."""
    if name in GEOMETRIC:return strip_tag(geometric(name,version,False,0,65535 if style==5 else 0))
    choices={
        'moRefWrapper_c':lambda:wrapper(version,False),
        'moGeneralSurfRef_w':lambda:surface(version,False),
        'moPlaneRef_w':lambda:plane_reference(version),
        'moEndPointRef_w':lambda:endpoint_body(mapping,version,ids,False),
        'moLoftSynchRef_c':lambda:synchronization_body(mapping,version,ids,False),
        'moSketchExtRef_w':lambda:strip_tag(external_reference(version,-1,False)),
        'moTempAxisRef_w':lambda:strip_tag(temporary_axis(version,False)),
        'moCoordSysRef_w':lambda:wrapper(version,False)+transform(False,7),
    }
    return choices[name]()


def target(mapping: WireMap, ids: dict[str,int], version: int, kind: int, style: int, key: str, edited: bool) -> bytes:
    """Preserve each actual class and independently change only its common wrapper word; e.g. target(mapping, ids, 19000, 3, 2, 'first', True)."""
    name=CLASSES[kind];fresh=key not in ids
    if name in ('moSolidRef_w','moConstSurfRef_w'):
        raw=solid_pointer(mapping,ids,version,0,0,0,key,False,name)
    else:
        raw=shared_object(mapping,ids,key,name,lambda:payload(mapping,ids,version,name,style))
    if not edited or not fresh:return raw
    offset=6+len(name.encode('ascii'))+6
    if raw[offset:offset+4]!=struct.pack('<i',-5):
        raise ValueError(f'wrapper {name} word {raw[offset:offset+4].hex()}; expected complete null-pointer prefix and i32 -5')
    return raw[:offset]+struct.pack('<i',12345)+raw[offset+4:]


def external_entity(mapping: WireMap, ids: dict[str,int], version: int, kind: int, style: int, key: str, edited: bool) -> bytes:
    """Keep four pointer positions and the axis CString intact; e.g. external_entity(mapping, ids, 19000, 3, 0, 'first', False)."""
    def contents() -> bytes:
        reference=bytes(2) if style==4 else target(mapping,ids,version,kind,style,'target-'+key,edited and key=='first')
        return reference+bytes(6)+short_wide_name('Ω axis')
    return shared_object(mapping,ids,'external-'+key,'sgExtEnt_c',contents)


def sketch_wrapper_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Exercise fresh, cached, shared and distinct references in the inherited map; e.g. sketch_wrapper_object(3, 19000, 2)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def entity(key: str) -> bytes:
        return external_entity(mapping,ids,version,kind,style,key,edited)
    def seed(key: str) -> bytes:
        return target(mapping,ids,version,kind,style,'target-'+key,edited and key=='first')
    def outer() -> bytes:
        if style==1:items=[seed('first'),entity('first')]
        elif style==2:items=[entity('first'),entity('second'),entity('first')]
        elif style==3:items=[seed('first'),entity('first'),seed('second'),entity('second')]
        else:items=[seed('first'),entity('first'),entity('second'),entity('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',outer) if style in (1,2,3,5) else entity('first')
    return raw,mapping.slot+1


def unrelated_fresh_object() -> tuple[bytes,int]:
    """Use a physically complete unrelated body at the typed field; e.g. unrelated_fresh_object()."""
    mapping=WireMap()
    def contents() -> bytes:
        foreign=mapping.new('moNodeName_c',lambda:short_wide_name('unrelated complete object'))
        return foreign+bytes(6)+short_wide_name('Ω axis')
    raw=mapping.new('sgExtEnt_c',contents)
    return raw,mapping.slot+1
