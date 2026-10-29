"""Complete jog/break-corner graphs with direct keepers and stored presence (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body,keeper
from flange_fixture_bytes import FlangeFixtureOptions,group_wire,flange_profile
from sheet_fixture_bytes import component_reference

CLASSES=('moJog_c','moBreakCornerFeat_c')
VERSIONS=(0x27cc,0x335a,0x3afd,0x3afe,0x42cb,0x46ca,0x46cb,19000,0xffffffff)
PRESENCES=(0,1,255,128,254,0)
OPTIONS=(0,1,2,3,-2147483648,2147483647)


class JogGraph(EdgeGraph):
    """Preserve direct/tagged ownership without applying native runtime normalization."""
    def base(self, key: str) -> bytes:
        """Use the established body and bend-group serializers; e.g. graph.base('first')."""
        raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
        return raw+group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions(extended=self.extended))

    def boundary(self) -> bytes:
        """Share a complete typed component-face subtype; e.g. graph.boundary()."""
        return component_reference(self.mapping,self.version,self.ids,'moCompFace_c') if self.style not in (0,4) else bytes(2)

    def edge(self) -> bytes:
        """Share a complete typed component-edge body; e.g. graph.edge()."""
        return component_reference(self.mapping,self.version,self.ids,'moCompEdge_c') if self.style not in (0,4) else bytes(2)

    def optional_references(self) -> bytes:
        """Exercise nonzero presence with independent null/full pointers; e.g. graph.optional_references()."""
        presence=PRESENCES[self.style]
        if not presence:return bytes(1)
        point=self.point() if self.style in (2,3) else bytes(2)
        reference=self.reference() if self.style==2 else bytes(2)
        return bytes([presence])+point+reference

    def jog_leading(self, edited: bool) -> bytes:
        """Keep each nonmonotonic scalar and pointer offset in native order; e.g. graph.jog_leading(False)."""
        raw=flange_profile(self.mapping,self.version,self.ids,self.style not in (0,4))
        raw+=struct.pack('<QiQi',BITS[self.style],WORDS[self.style],BITS[(self.style+1)%6],12345 if edited else OPTIONS[self.style])
        return raw+self.boundary()+struct.pack('<i',-1)+self.display()+self.display()+struct.pack('<i',-2147483648)

    def jog_tail(self) -> bytes:
        """Retain point/list/vector fields before the final boundary gate; e.g. graph.jog_tail()."""
        raw=struct.pack('<3Q',*BITS[:3])+self.array()+struct.pack('<3i',*WORDS[:3])+self.component()+struct.pack('<i',-1)
        raw+=struct.pack('<3Q',*BITS[3:6])
        return raw+(self.boundary() if self.version>=0x3afe else b'')

    def break_leading(self, key: str, edited: bool) -> bytes:
        """Serialize an owned keeper directly after the first dimension; e.g. graph.break_leading('first',False)."""
        raw=self.listing()+self.listing()+struct.pack('<iQ',12345 if edited else WORDS[self.style],BITS[self.style])+self.display()
        return raw+keeper(self.mapping,self.style not in (0,4),self.ids[key])

    def break_tail(self) -> bytes:
        """The final array has a real tag independently of the keeper's direct list; e.g. graph.break_tail()."""
        raw=self.edge()+self.display()+struct.pack('<i',-1)+self.listing()+struct.pack('<iQ',-2147483648,BITS[(self.style+1)%6])+self.display()
        raw+=struct.pack('<2iQ',2147483647,-1,BITS[(self.style+2)%6])
        raw+=self.shared('tagged-array','suObArray',self.array) if self.style not in (0,4) else bytes(2)
        return raw+(struct.pack('<i',-1) if self.version>=0x46cb else b'')

    def record(self, kind: int, key: str, edited: bool) -> bytes:
        """Build one complete native feature before computing map size; e.g. graph.record(0,'first',False)."""
        def contents() -> bytes:
            raw=self.base(key)
            if kind==0:return raw+self.jog_leading(edited)+self.optional_references()+self.jog_tail()
            return raw+self.break_leading(key,edited)+self.break_tail()
        return self.shared(key,CLASSES[kind],contents)


def jog_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, extended: bool=False) -> tuple[bytes,int]:
    """Keep fresh, distinct and repeated roots across both feature gates; e.g. jog_graph(1,19000,5)."""
    graph=JogGraph(version,style,first_slot,extended)
    def record(key: str) -> bytes:
        return graph.record(kind,key,edited and key=='first')
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1
