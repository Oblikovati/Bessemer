"""Complete alignment, order-dimension, alias and text-kernel graphs (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from freeze_environment import feature
from dimension_fixture_bytes import dimension,dimension_base,handle_base
from sketch_dimension_fixture_bytes import valued_body
from display_dimension_fixture_bytes import shared as display_shared,body as strip_tag
from freeze_component_references import base as component_base
from sketch_reference_fixture_bytes import handle as sketch_handle
from sketch_geometry_fixture_bytes import text as text_wire
from sketch_tail_fixture_bytes import ordinary
from kernel_geometry_fixture import frame
from owned_name_edit import short_wide_name
from weld_corner_bytes import counted

CLASSES=('moAlignGroup_c','moDisplayOrdDim_c','sgSplineLengthDim','sgBsOrdDim','moScalerLengthDim_c','sgSketch')
VERSIONS=(0x27cc,0x2f4d,0x2f4e,0x2f4f,0x2f50,0x2f55,0x2f56,0x2f79,0x2f7a,0x2f9f,0x2fa0,0x3747,0x3748,0x42ee,0x42ef,19000,0xffffffff)
FLAGS=(0,-1,-2147483648,1,0,2147483647)

class AlignmentGraph(EdgeGraph):
    """Own one real map for inherited bodies, direct members and back references."""
    def vector(self, count: int=3) -> bytes:
        return struct.pack('<'+'Q'*count,*(BITS[(self.style+i)%6] for i in range(count)))

    def length_dimension(self, edited: bool=False) -> bytes:
        raw=strip_tag(dimension('moScalerDim_c',self.version,(0,2,4,6,8,16)[self.style]))
        return struct.pack('<i',12345)+raw[4:] if edited else raw

    def handle(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        def contents() -> bytes:
            return handle_base(self.version,False,0)+self.shared('length',CLASSES[4],self.length_dimension)
        return self.shared('dimension-handle','moFeatureDimHandle_c',contents)

    def handles(self, maximum: bool=False) -> bytes:
        n=(0,1,2,1,0,2)[self.style] if maximum else (0,2,3,1,0,3)[self.style]
        return counted([self.handle() for _ in range(n)],self.style==5)

    def alignment(self, edited: bool) -> bytes:
        self.mapping.slot+=2
        raw=feature(self.version,False)+self.handles()+struct.pack('<i',12345 if edited else WORDS[self.style])
        values=bytes([0,255,65,0,128,254,1,127,255][:(0,3,5,1,0,9)[self.style]])
        raw+=counted([bytes([v]) for v in values],self.style==5)+struct.pack('<IQi',0xffffffff,BITS[self.style],-2147483648)
        if self.version>0x2f4d:raw+=struct.pack('<i',FLAGS[self.style])
        if self.version<0x2f50:return raw
        if self.version<0x2fa0:return raw+self.handle()+self.handle()
        raw+=self.handles(True)
        raw+=self.shared('component','moCompRef_c',lambda:component_base(self.version,False)) if self.style not in (0,4) else bytes(2)
        raw+=self.shared('entity','sgEntHandle',lambda:strip_tag(sketch_handle('sgEntHandle',-2147483648))) if self.style not in (0,4) else bytes(2)
        return raw+self.vector(6)+(self.vector() if FLAGS[self.style] else b'')

    def display_reference(self) -> bytes:
        return bytes(2) if self.style in (0,4) else self.shared('display-reference','moDispObjRef_c',lambda:struct.pack('<4i',*WORDS[:4]))

    def order(self, edited: bool) -> bytes:
        raw=display_shared(self.mapping,self.version,'moDisplayDistanceDim_c',self.style in (1,3,5),self.ids['root'])
        shared=self.ids['root'] if self.style not in (0,4) else 0
        raw+=struct.pack('<H',shared)+self.listing()+struct.pack('<H',65535)+self.vector(2)+struct.pack('<2HB',65534,32768,255)
        raw+=self.component()+struct.pack('<2i',-2147483648,2147483647)+self.display_reference()
        refs=[] if self.style in (0,4) else [self.display_reference(),bytes(2),self.display_reference()]
        return raw+struct.pack('<I',len(refs))+b''.join(refs)+struct.pack('<i',12345 if edited else WORDS[self.style])+self.vector(1)

    def valued(self, kind: int, edited: bool) -> bytes:
        rich=self.style in (1,3,5);raw=valued_body('sgValG1Dim' if kind==2 else 'sgValG2Dim',self.version,rich,self.ids['root'])
        if rich:self.mapping.slot+=2 if kind==2 else 4
        offset=len(dimension_base(self.version,False))
        return raw[:offset]+struct.pack('<i',12345)+raw[offset+4:] if edited else raw

    def text_sketch(self, edited: bool) -> bytes:
        raw=ordinary(self.mapping,self.version,False)
        text=text_wire(self.version,False,curve=self.style!=0);self.mapping.slot+=2
        if self.style:
            if self.style==5:text=text[:-2]+self.shared('curves','suObArray',self.array)
            text+=(frame() if self.style in (2,5) else bytes([(0,1,0,255,128,0)[self.style]]))
            text+=short_wide_name('Ω curve text\0')+struct.pack('<2i',-2147483648,12345 if edited else 2147483647)
        return raw[:8]+struct.pack('<i',1)+text+raw[12:]


def alignment_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose six complete class paths; e.g. alignment_object(0,19000,2)."""
    graph=AlignmentGraph(version,style,first_slot)
    build=(lambda:graph.alignment(edited)) if kind==0 else (lambda:graph.order(edited)) if kind==1 else (lambda:graph.valued(kind,edited)) if kind in (2,3) else (lambda:graph.length_dimension(edited)) if kind==4 else (lambda:graph.text_sketch(edited))
    raw=graph.shared('root',CLASSES[kind],build)
    return raw,graph.mapping.slot+1
