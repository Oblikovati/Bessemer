"""Complete independent sew surfaces, direct arrays and actual empty keeper callers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from body_feature_fixture_bytes import header, atom_fields, body_suffix
from display_dimension_fixture_bytes import body as strip_tag
from freeze_geometric_references import wire as geometric
from reference_plane_fixture_bytes import plane_reference
from sweep_fixture_bytes import shared_object

CLASSES=('moSewRefSurface_c','moSewRefSurfIdKeeper_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,-1,0,1,2,2147483647)
NUMBERS=((0x8000000000000000,0x7ff800000000002a,0x7ff0000000000000),
    (0x7ff800000000002b,0x7ff000000000002c,0xfff0000000000000),(1,0x8000000000000000,4),
    (0x3ff4000000000000,0x8000000000000000,0x7ff8000000000043),
    (0xfff0000000000000,0x7ff8000000000099,0x4000000000000000),
    (0x7ff80000000000aa,0x7ff00000000000bb,0x8000000000000000))


def keeper(mapping: WireMap, ids: dict[str,int], key: str='keeper') -> bytes:
    """Reserve only the real tagged keeper, whose body is empty; e.g. keeper(mapping, ids)."""
    return shared_object(mapping,ids,key,CLASSES[1],lambda:b'')


def reference(mapping: WireMap, ids: dict[str,int], version: int, index: int) -> bytes:
    """Use complete existing surface/plane references across direct arrays; e.g. reference(mapping, ids, 19000, 1)."""
    if index%3==1:return bytes(2)
    if index%3==2:return shared_object(mapping,ids,'plane','moPlaneRef_w',lambda:plane_reference(version))
    return shared_object(mapping,ids,'face','moSingleFaceRef_w',lambda:strip_tag(geometric('moSingleFaceRef_w',version,False,0)))


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Keep five independent counts and preserve raw flags and nonmonotonic double order; e.g. payload(mapping, ids, 19000, 4, False)."""
    raw=b''
    for column in range(5):
        count=0 if style==0 else (column+style)%4
        prefix=struct.pack('<HI',65535,count) if style in (4,5) else struct.pack('<H',count)
        items=bytes(2)*count if style==1 else b''.join(reference(mapping,ids,version,i) for i in range(count))
        raw+=prefix+items
    flags=[WORDS[(index+style)%6] for index in range(6)]
    if edited:flags[0]=12345
    return raw+struct.pack('<6i3Q',*flags,*NUMBERS[style])


def surface(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool) -> bytes:
    """Capture the owner before allocating its header name and optional keeper; e.g. surface(mapping, ids, 19000, 3, 'first', False)."""
    def contents() -> bytes:
        owner=mapping.slot;rich=style in (3,5);prefix=header(mapping,version,rich)
        if style>=3:
            if prefix[-2:]!=bytes(2):raise ValueError(f'body header suffix {prefix[-2:].hex()}; expected absent u16 keeper flag')
            prefix=prefix[:-2]+struct.pack('<H',65535)+keeper(mapping,ids)
        return prefix+atom_fields(mapping,rich,owner)+body_suffix(rich)+payload(mapping,ids,version,style,edited and key=='first')
    return shared_object(mapping,ids,key,CLASSES[0],contents)


def sew_surface_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Retain distinct roots, real keeper owners and shared targets; e.g. sew_surface_object(1, 19000, 5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        return surface(mapping,ids,version,style,key,edited) if kind==0 else keeper(mapping,ids,key)
    def outer() -> bytes:
        if kind==1 and style>=3:
            first=keeper(mapping,ids);owner=surface(mapping,ids,version,style,'owner',False)
            items=[first,owner,keeper(mapping,ids)]
        else:items=[record('first'),record('second'),record('first')]
        return struct.pack('<H',len(items))+b''.join(items)
    raw=mapping.new('suObArray',outer) if style in (2,4,5) or (kind==1 and style>=1) else record('first')
    return raw,mapping.slot+1
