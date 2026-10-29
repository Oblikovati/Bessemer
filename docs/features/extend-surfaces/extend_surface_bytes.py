"""Complete inline extended surfaces and independent data selectors (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS
from axis_owner_bytes import geometry_header
from revolved_bytes import surface_wire

CLASSES=('moExtendRefSurface_c','moExtendRefSurfaceData_c','moExtendRefSurfIdKeeper_c')
VERSIONS=(0x27cc,0x2bcc,0x2bcd,0x2f85,0x2f86,19000,0xffffffff)
MODES=(0,1,2,3,-1,0)


class ExtendSurfaceGraph(EdgeGraph):
    """Keep three direct lists and all inline surface members outside container tags."""
    def data(self, edited: bool) -> bytes:
        raw=self.array()+self.array()+self.array()+struct.pack('<2i',WORDS[self.style],MODES[self.style])
        if MODES[self.style]==0:raw+=self.display()
        elif MODES[self.style] in (1,2):raw+=self.reference()
        return raw+struct.pack('<i',12345 if edited else WORDS[(self.style+1)%6])

    def surface(self, edited: bool) -> bytes:
        keeper=lambda:self.shared('keeper',CLASSES[2],lambda:b'')
        raw=geometry_header(self.mapping,self.version,self.style in (3,5),keeper=keeper if self.style in (3,5) else None)
        raw+=surface_wire(self.mapping,self.ids,self.version,4 if self.style==5 else self.style)
        return raw+(bytes(2) if self.style==4 else self.shared('data',CLASSES[1],lambda:self.data(edited)))


def extend_surface_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete surface/data/keeper records; e.g. extend_surface_object(0,19000,3)."""
    if kind not in range(3):raise ValueError(f'extended-surface fixture kind {kind}; expected 0..2')
    graph=ExtendSurfaceGraph(version,style,first_slot)
    contents=(lambda:graph.surface(edited)) if kind==0 else (lambda:graph.data(edited)) if kind==1 else (lambda:b'')
    raw=graph.shared('root',CLASSES[kind],contents)
    return raw,graph.mapping.slot+1
