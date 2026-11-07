"""Complete move/copy records composed from owned native-layout fixtures (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap,body
from sweep_fixture_bytes import shared_object,dimension
from freeze_model_references import wrapper
from reference_plane_fixture_bytes import plane_reference
from mirror_solid_bytes import matrix_body

CLASSES=('moMoveCopyBody_c','moMoveCopyBodyData_c','moPSMeshToleranceHandler_c')
VERSIONS=(0x27cc,0x2f69,0x2f6a,0x3b0b,0x3b0c,0x42cc,0x42cd,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)


def mesh_bytes(style: int, edited: bool=False) -> bytes:
    """Preserve native nonmonotonic mesh ordering; e.g. mesh_bytes(2)."""
    return struct.pack('<i3QiQ',12345 if edited else WORDS[style],*BITS[:3],WORDS[(style+1)%6],BITS[style])


def conformal_bytes(style: int) -> bytes:
    """Retain negative zero and every documented flag branch; e.g. conformal_bytes(3)."""
    flags=(0,0x200,0xfffffcff,0xfffffeff,0x200,1)[style]
    raw=struct.pack('<QI',0x8000000000000000 if style%2 else 0,flags)
    return raw+(struct.pack('<2Q',0,0x8000000000000000) if flags&0x200 else b'')


class MoveCopyGraph:
    """Reserve identities before nested bodies and retain shared reference targets."""
    def __init__(self, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.version,self.style,self.edited=version,style,edited
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}

    def obj(self, key: str, name: str, build: Callable[[],bytes]) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,build)

    def reference(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        return self.obj('reference','moRefWrapper_c',lambda:wrapper(self.version,False))

    def array(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        first=self.obj('attribute','moEntityIdRep_c',lambda:bytes(4))
        return struct.pack('<H',3)+first+struct.pack('<2H',self.ids['attribute'],self.ids['payload'])

    def vector(self) -> bytes:
        return struct.pack('<3Q',BITS[self.style],BITS[(self.style+1)%6],BITS[(self.style+2)%6])

    def leading(self) -> bytes:
        raw=self.reference()+self.reference()+self.reference()+struct.pack('<Q',BITS[self.style])+self.array()
        words=[0xffffffff-i for i in range(30)];words[28]&=~0xf0
        return raw+struct.pack('<30I',*words)+self.vector()+struct.pack('<i',WORDS[self.style])+self.vector()*2+struct.pack('<2i',-1,2147483647)

    def payload(self) -> bytes:
        raw=self.leading()+bytes(2)+struct.pack('<i',-1)+self.vector()*3
        raw+=self.reference()+struct.pack('<i',-2147483648)
        raw+=self.obj('plane','moPlaneRef_w',lambda:plane_reference(self.version)) if self.style not in (0,4) else bytes(2)
        raw+=struct.pack('<3i',-1,0,2147483647)+mesh_bytes(self.style)
        if self.version>0x2f69:raw+=conformal_bytes(self.style)
        if self.version>0x3b0b:raw+=struct.pack('<7i',*([WORDS[self.style]]*7))
        if self.version>0x42cc:raw+=struct.pack('<2i',-1,2147483647)
        return raw

    def payload_pointer(self) -> bytes:
        return self.obj('payload','moMoveCopyBodyData_c',self.payload)

    def display(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        return dimension(self.mapping,self.version,self.ids,'moDisplayDistanceDim_c')

    def feature(self) -> bytes:
        raw=body(self.mapping,self.version,self.style%2==1,self.ids['root'])+self.payload_pointer()
        raw+=self.obj('matrix','moMatrixIndexHelper_c',lambda:matrix_body(self.mapping,self.ids,self.style))
        raw+=struct.pack('<i',12345 if self.edited else WORDS[self.style])
        if self.version>0x3b0b:raw+=b''.join(self.display() for _ in range(6))+struct.pack('<6i',*WORDS)
        if self.version>0x42cc:raw+=self.display()+struct.pack('<i',-7)+self.display()+struct.pack('<i',9)
        return raw

    def root(self, kind: int) -> bytes:
        if kind==0:return self.obj('root',CLASSES[0],self.feature)
        if kind==1:return self.payload_pointer()
        return self.obj('mesh',CLASSES[2],lambda:mesh_bytes(self.style,self.edited))


def move_copy_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete tagged objects with exact map size; e.g. move_copy_object(0,19000,2)."""
    graph=MoveCopyGraph(version,style,edited,first_slot)
    raw=graph.root(kind)
    return raw,graph.mapping.slot+1
