"""Complete sculpt payloads, source edge identifiers and bounding centers (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from weld_corner_bytes import counted
from display_dimension_fixture_bytes import shared as display_shared

CLASSES=('moSculpt_c','moRightSculptEdgeIdRep_c','moLeftSculptEdgeIdRep_c','moBBoxCenterData2_c','moDisplayScalarDim_c')
VERSIONS=(0x27cc,0x2b68,0x2b69,0x2b72,0x2b73,0x2faf,0x2fb0,0x3335,0x3336,0x373b,0x373c,0x3b01,0x3b02,19000,0xffffffff)
GROUPS=(0,-1,2,1,-2147483648,3)

class SculptGraph(EdgeGraph):
    """Reserve actual owner and representation slots before recursively referenced bodies."""
    def representation(self, key: str, edited: bool=False) -> bytes:
        identity=self.ids[key] if self.style not in (0,4) else 0
        owner=self.ids.get('root',0) if self.style not in (0,4) else 0
        return struct.pack('<3H2i',owner,0,identity,12345 if edited else WORDS[self.style],WORDS[(self.style+1)%6])

    def center(self, edited: bool=False) -> bytes:
        return struct.pack('<i4Q',12345 if edited else WORDS[self.style],*(BITS[(self.style+i)%6] for i in range(4)))

    def trim_items(self) -> bytes:
        if self.style in (0,4):return bytes(2)
        first=self.shared('right',CLASSES[1],lambda:self.representation('right'))
        second=self.shared('left',CLASSES[2],lambda:self.representation('left'))
        center=self.shared('center',CLASSES[3],self.center)
        return counted([first,second,center,self.shared('right',CLASSES[1],lambda:b''),bytes(2)],self.style==5)

    def chooser(self) -> bytes:
        owner=self.ids['root'] if self.style not in (0,4) else 0
        return self.array()+struct.pack('<H',owner)

    def sculpt(self, edited: bool) -> bytes:
        raw=body(self.mapping,self.version,self.style in (3,5),self.ids['root'])
        raw+=self.trim_items()+self.array()+self.chooser()+self.chooser()+struct.pack('<i',GROUPS[self.style])
        for group in range(max(0,GROUPS[self.style])):
            values=[struct.pack('<I',v) for v in (0xffffffff,0x80000000,group)] if group%2==0 else []
            raw+=counted(values,self.style==5)
        return raw+struct.pack('<5i',12345 if edited else WORDS[self.style],2147483647,-1,0,-2147483648)


def sculpt_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose all four fully decoded records; e.g. sculpt_object(0,19000,2)."""
    graph=SculptGraph(version,style,first_slot)
    build=(lambda:graph.sculpt(edited)) if kind==0 else (lambda:graph.representation('root',edited)) if kind in (1,2) else (lambda:graph.center(edited)) if kind==3 else (lambda:display_shared(graph.mapping,version,'moDisplayDim_c',style in (1,3,5),graph.ids['root']))
    raw=graph.shared('root',CLASSES[kind],build)
    return raw,graph.mapping.slot+1
