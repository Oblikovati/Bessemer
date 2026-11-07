"""Complete sketch-bitmap feature and explicit byte-plane fixtures (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_environment import feature
from freeze_model_references import wrapper
from freeze_views import transform
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from owned_name_edit import short_wide_name

CLASSES=('moSketchBitmap_c','moSketchBitmapData_c')
VERSIONS=(0x27cc,0x2bd2,0x2bd3,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)
REFERENCE_PRESENCE=(0,1,0xffffffff,0x80000000,0,7)
PLANE_PRESENCE=(0,1,255,128,1,0)


class BitmapGraph:
    """Allocate complete targets and preserve independent presence words."""
    def __init__(self, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.version,self.style,self.edited=version,style,edited
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}

    def obj(self, key: str, name: str, contents: Callable[[],bytes]) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,contents)

    def references(self) -> bytes:
        presence=REFERENCE_PRESENCE[self.style]
        raw=struct.pack('<I',presence)
        if not presence:return raw
        raw+=self.obj('line','moLineRef_w',lambda:strip_tag(geometric('moLineRef_w',self.version,False,0)))
        return raw+self.obj('dimension','moDimRefWrapper_c',lambda:struct.pack('<Q',BITS[self.style])+wrapper(self.version,False))

    def payload(self) -> bytes:
        raw=struct.pack('<6Q',*BITS)
        width,height=((2,3),(2,3),(3,2),(0,2147483647),(0,0),(2147483647,-1))[self.style]
        raw+=struct.pack('<3i',width,height,12345 if self.edited else WORDS[self.style])+short_wide_name('Ω bitmap source')
        raw+=self.references()+struct.pack('<3iIi',-7,2147483647,-2147483648,0xffffffff,-8)
        presence=PLANE_PRESENCE[self.style];raw+=bytes([presence])
        if presence:raw+=bytes((i*51)&255 for i in range(width*height))
        if self.version>=0x2bd3:raw+=struct.pack('<i',-9)
        return raw

    def feature(self) -> bytes:
        self.mapping.slot+=2
        raw=feature(self.version,False)+struct.pack('<i',-2)
        raw+=self.obj('payload',CLASSES[1],self.payload)
        presence=(0,1,-1,-2147483648,0,2147483647)[self.style]
        raw+=struct.pack('<i',presence)
        return raw+(transform(self.style%2==1,0x2b) if presence else b'')

    def root(self, kind: int) -> bytes:
        return self.obj('root',CLASSES[kind],self.feature if kind==0 else self.payload)


def bitmap_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete feature/data bodies and map size; e.g. bitmap_object(0,19000,2)."""
    graph=BitmapGraph(version,style,edited,first_slot)
    raw=graph.root(kind)
    return raw,graph.mapping.slot+1
