"""Compose complete stock records using existing inherited and kernel fixtures (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper
from freeze_component_references import base as component_base, wire as component_wire
from display_dimension_fixture_bytes import body as strip_tag
from freeze_views import transform
from kernel_geometry_fixture import frame
from solid_reference_bytes import body_attributes
from owned_name_edit import short_wide_name
from reference_plane_fixture_bytes import plane_reference

CLASSES=('moStock_c','moMirrorStock_c','moBodyRef_w','moCompBody_c')
VERSIONS=(0x27cc,0x2b68,0x2b69,0x3335,0x3336,0x373b,0x373c,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)


def pairs(unsigned: bool, rich: bool) -> bytes:
    """Keep duplicate signed pairs under their distinct count overloads; e.g. pairs(True,True)."""
    return struct.pack('<I' if unsigned else '<i',2 if rich else 0)+(struct.pack('<4i',-1,7,-1,7) if rich else b'')


def hole_records(rich: bool) -> bytes:
    """Keep each nullable hole pointer after its own raw pair; e.g. hole_records(True)."""
    return struct.pack('<i',2 if rich else 0)+(struct.pack('<2iH2iH',-7,8,0,-7,8,0) if rich else b'')


class StockGraphBuilder:
    """Own a single fixture's ordered class/object map."""
    def __init__(self, kind: int, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}
        self.kind,self.version,self.style,self.edited=kind,version,style,edited
        self.rich=style%2==1

    def obj(self, key: str, name: str, build: Callable[[],bytes]) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,build)

    def word(self, kind: int) -> int:
        return 12345 if self.edited and self.kind==kind else WORDS[self.style]

    def configuration(self) -> bytes:
        return short_wide_name('Ω source configuration')+struct.pack('<i',self.word(4))

    def component(self) -> bytes:
        return self.obj('component','moCompBody_c',self.component_body)

    def component_body(self) -> bytes:
        raw=component_base(self.version,False)+bytes([255 if self.rich else 0])
        if self.rich:raw+=short_wide_name('Ω alias')
        raw+=struct.pack('<iB',self.word(3),1 if self.rich else 0)
        if self.rich:raw+=transform(True,0x2a)
        raw+=struct.pack('<I3i10dB',0xffffffff,-1,0,2147483647,-0.,1.,-2.,3.,-4.,5.,6.,-7.,8.,9.,128 if self.style>=2 else 0)
        if self.style>=2:raw+=transform(False,0x0a)
        return raw

    def body_reference(self) -> bytes:
        return self.obj('body-reference','moBodyRef_w',self.body_reference_body)

    def body_reference_body(self) -> bytes:
        component=self.component() if self.style else bytes(2)
        control=(0,-1,1,-2147483648,0,2147483647)[self.style]
        if self.edited and self.kind==2:control=12345
        raw=component+wrapper(self.version,False)[2:]+struct.pack('<i',control)
        if control==0:return raw
        raw+=struct.pack('<i',2)
        for index in range(2):
            kernel=frame() if self.style in (2,5) and index==0 else b'\xff'
            raw+=kernel+body_attributes(self.mapping,self.ids)
        return raw+bytes(2)

    def derived(self) -> bytes:
        raw=b''
        if self.version>0x2b68:
            raw=pairs(True,self.rich)*2+struct.pack('<I',3 if self.rich else 0)
            if self.rich:raw+=struct.pack('<3i',-2147483648,0,2147483647)
        if self.version>=0x373c:raw+=hole_records(self.rich)
        return raw

    def direct_array(self) -> bytes:
        if not self.rich:return bytes(2)
        first=self.component()
        return struct.pack('<H',3)+first+struct.pack('<HH',self.ids['component'],0)

    def fixed_transform(self) -> bytes:
        words=[(0x80000000+i*31)&0xffffffff for i in range(30)]
        words[28]=(0,1,2,7,8,15)[self.style]
        return struct.pack('<30I',*words)

    def dynamic_words(self) -> bytes:
        if self.style==0:return bytes(4)
        width=(0,4,8,12,20,0xfffffffc)[self.style]
        count=0 if self.style==5 else 2
        raw=struct.pack('<4I',width,0x12345678,0x80000000,count)
        return raw+b''.join(struct.pack('<I',(0x80000000+i)&0xffffffff) for i in range(count*(width//4)))

    def edge(self) -> bytes:
        return self.obj('edge','moCompEdge_c',lambda:strip_tag(component_wire('moCompEdge_c',self.version,False,0)))

    def late(self) -> bytes:
        raw=struct.pack('<Ii',0xffffffff,-2)+self.dynamic_words()+pairs(False,self.rich)
        raw+=struct.pack('<i',2 if self.rich else 0)
        if self.rich:raw+=self.edge()+bytes(2)+self.edge()+bytes(2)
        raw+=struct.pack('<ii',-3,1 if self.rich else 0)
        if self.rich:raw+=self.edge()+struct.pack('<i',3)+self.edge()+self.edge()+bytes(2)
        raw+=struct.pack('<i',-4)+pairs(False,self.rich)*2
        if 0x3335<self.version<0x373c:raw+=hole_records(self.rich)
        return raw

    def stock_body(self) -> bytes:
        raw=body(self.mapping,self.version,self.rich,self.ids['root'])+self.derived()+self.body_reference()
        mode=(0,1,2,3,0xffffffff,2)[self.style]
        raw+=struct.pack('<I',mode)
        if mode in (1,2):raw+=frame() if self.style==2 else b'\xff'
        raw+=self.configuration()+self.direct_array()+self.direct_array()+struct.pack('<i',12345 if self.edited else WORDS[self.style])
        raw+=self.direct_array()+struct.pack('<3di',-0.,1.25,-2.5,-1)+pairs(False,self.rich)+self.direct_array()
        raw+=self.fixed_transform()+self.late()
        if self.kind==1:
            raw+=self.obj('plane','moPlaneRef_w',lambda:plane_reference(self.version)) if self.rich else bytes(2)
        return raw

    def root(self) -> bytes:
        if self.kind in (0,1):return self.obj('root',CLASSES[self.kind],self.stock_body)
        if self.kind==2:return self.body_reference()
        return self.component()


def stock_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete inherited stock graphs; e.g. stock_graph(1,19000,3)."""
    builder=StockGraphBuilder(kind,version,style,edited,first_slot)
    raw=builder.root()
    return raw,builder.mapping.slot+1
