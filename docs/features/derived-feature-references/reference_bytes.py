"""Complete derived-feature references and inherited surface-body references (#54)."""
import struct
from collections.abc import Callable
from functools import partial
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper
from reference_plane_fixture_bytes import base as plane_data
from stock_fixture_bytes import StockGraphBuilder
from body_wrapper_bytes import wrapper_caller_object

CLASSES=('moDerivedFeatRef_w','moSurfaceBodyRef_w')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2f85,0x2f86,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)
PRESENCE=(0,1,-1,-2147483648,0,2147483647)
AXIS_BASE=struct.pack('<6dQQ6d',1,2,3,4,5,6,0x8000000000000000,0x7ff8000000000042,0,0,1,7,8,9)


class DerivedGraph:
    """Keep directly owned arrays outside the shared object map."""
    def __init__(self, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}
        self.version,self.style,self.edited=version,style,edited
        self.rich=style not in (0,4)

    def obj(self, key: str, name: str, build: Callable[[],bytes]) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,build)

    def attribute(self) -> bytes:
        return self.obj('attribute','moEntityIdRep_c',lambda:bytes(4))

    def array(self) -> bytes:
        if not self.rich:return bytes(2)
        first=self.attribute()
        return struct.pack('<H',3)+first+self.attribute()+struct.pack('<H',self.ids['root'])

    def references(self) -> bytes:
        feature=self.obj('feature','moFR_c',lambda:struct.pack('<HiI',0,-1,0xffffffff)) if self.rich else bytes(2)
        raw=feature+struct.pack('<I',(0,0xffffffff,0x80000000,1,0x7fffffff,0xdeadbeef)[self.style])
        raw+=self.obj('plane','moRefPlnData_c',lambda:plane_data(self.version,False)) if self.rich else bytes(2)
        raw+=self.obj('axis','moRefAxisData_c',lambda:AXIS_BASE) if self.rich else bytes(2)
        return raw+bytes(2)

    def contents(self) -> bytes:
        raw=wrapper(self.version,False)+self.references()+self.array()
        raw+=struct.pack('<2i',12345 if self.edited else WORDS[self.style],-2)
        if self.version>0x2f85:
            presence=PRESENCE[self.style];raw+=struct.pack('<i',presence)
            if presence:raw+=struct.pack('<3Q',0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234)
        return raw

    def root(self) -> bytes:
        return self.obj('root',CLASSES[0],self.contents)


def reference_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build exact reference subclasses through their real bases; e.g. reference_object(0,19000,3)."""
    if kind==0:
        graph=DerivedGraph(version,style,edited,first_slot)
        raw=graph.root();return raw,graph.mapping.slot+1
    graph=StockGraphBuilder(2,version,style,edited,first_slot)
    raw=graph.obj('body-reference',CLASSES[1],graph.body_reference_body)
    return raw,graph.mapping.slot+1


def caller_pointer(mapping: WireMap, ids: dict[str,int], version: int, key: str, target: int) -> bytes:
    """Use minimal real suffixes at every shared caller; e.g. caller_pointer(w,ids,19000,'first',0)."""
    def contents() -> bytes:
        raw=wrapper(version,False)
        if target==1:return raw+struct.pack('<i',0)
        raw+=bytes(2)+struct.pack('<I',0xffffffff)+bytes(8)+struct.pack('<2i',-1,-2)
        return raw+(struct.pack('<i',0) if version>0x2f85 else b'')
    return shared_object(mapping,ids,key,CLASSES[target],contents)


def wrapper_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep all five complete callers for each new subclass; e.g. wrapper_object(7,19000,5)."""
    return wrapper_caller_object(kind%5,version,style,first_slot,partial(caller_pointer,target=kind//5))
