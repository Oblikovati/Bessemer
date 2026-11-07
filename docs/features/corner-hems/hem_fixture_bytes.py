"""Native corner and hem graphs with directly owned data and chain identities (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from flange_fixture_bytes import FlangeFixtureOptions,group_wire,flange_profile
from sheet_fixture_bytes import component_reference
from weld_corner_bytes import counted

CLASSES=('moCornerFeat_c','moFaceGeom_c','moPlanarFaceGeom_c','moHem_c','moHemData_c',
         'moHemEdgeChainData_c','moHemEdgeChain_c','moHemEdgeChainElement_c','moReliefData_c')
VERSIONS=(0x27cc,0x2f5f,0x335a,0x372f,0x42cb,19000,0xffffffff)
EDITED_KINDS=(0,1,2,3,4,5,8)


class HemGraph(EdgeGraph):
    """Reuse complete dependencies while keeping each collection independently counted."""
    def rich(self) -> bool:
        """Select actual populated dependencies; e.g. graph.rich()."""
        return self.style not in (0,4)

    def edge_pointer(self) -> bytes:
        """Share a complete component-edge reference; e.g. graph.edge_pointer()."""
        return component_reference(self.mapping,self.version,self.ids,'moCompEdge_c') if self.rich() else bytes(2)

    def geometry(self, edited: bool=False) -> bytes:
        """Retain exactly six binary64 bit patterns; e.g. graph.geometry(True)."""
        values=[BITS[(self.style+i)%6] for i in range(6)]
        if edited:values[0]=struct.unpack('<Q',struct.pack('<d',123.5))[0]
        return struct.pack('<6Q',*values)

    def relief(self, edited: bool=False) -> bytes:
        """Keep the exact two-word, three-double, final-word order; e.g. graph.relief(False)."""
        return struct.pack('<2i3Qi',12345 if edited else WORDS[self.style],2147483647,*BITS[:3],-2147483648)

    def chain_element(self) -> bytes:
        """A tagged element has only its edge pointer; e.g. graph.chain_element()."""
        return self.shared('chain-element',CLASSES[7],self.edge_pointer)

    def chain_array(self) -> bytes:
        """The chain array preserves duplicates without a wrapper tag; e.g. graph.chain_array()."""
        entries=[self.chain_element(),bytes(2),self.chain_element()] if self.rich() else []
        return counted(entries,self.style==5)

    def chain_fields(self, edited: bool=False) -> bytes:
        """Store the chain before its profile, option and edge; e.g. graph.chain_fields(False)."""
        chain=self.shared('chain',CLASSES[6],self.chain_array) if self.rich() else bytes(2)
        profile=flange_profile(self.mapping,self.version,self.ids,self.rich())
        return chain+profile+struct.pack('<i',12345 if edited else WORDS[self.style])+self.edge_pointer()

    def chain_pointer(self) -> bytes:
        """Share a complete chain-data record in independently counted arrays; e.g. graph.chain_pointer()."""
        return self.shared('chain-data',CLASSES[5],self.chain_fields)

    def chains(self, count: int) -> bytes:
        """Use different count and presence shapes for the three hem arrays; e.g. graph.chains(2)."""
        if not self.rich():return counted([],self.style==5)
        entries=[self.chain_pointer() if i%2==0 else bytes(2) for i in range(count)]
        return counted(entries,self.style==5)

    def hem_data(self, owner: bytes, edited: bool=False) -> bytes:
        """Write the required owner and both independent data arrays directly; e.g. graph.hem_data(owner)."""
        raw=owner+self.chains(2)+(b'' if self.extended else bytes(2))+self.display()+self.display()
        raw+=struct.pack('<Q3i',BITS[self.style],12345 if edited else WORDS[self.style],-1,2147483647)
        raw+=self.display()+struct.pack('<i',-2147483648)
        raw+=self.shared('relief',CLASSES[8],self.relief) if self.rich() else bytes(2)
        return raw+self.chains(1)

    def base(self, key: str) -> bytes:
        """Use the established complete body and bend-group prefix; e.g. graph.base('first')."""
        raw=body(self.mapping,self.version,self.style in (3,5),self.ids[key])
        return raw+group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions(extended=self.extended))

    def hem(self, key: str, edited: bool=False) -> bytes:
        """The owned data body adds no class/object map slots; e.g. graph.hem('first')."""
        def contents() -> bytes:
            raw=self.base(key)+self.chains(3)
            return raw+self.hem_data(struct.pack('<H',self.ids[key]),edited)+struct.pack('<i',-1)
        return self.shared(key,CLASSES[3],contents)

    def face_list(self) -> bytes:
        """Keep real component-face targets in the tagged lists; e.g. graph.face_list()."""
        def contents() -> bytes:
            first=component_reference(self.mapping,self.version,self.ids,'moCompFace_c')
            return counted([first,bytes(2),component_reference(self.mapping,self.version,self.ids,'moCompFace_c')],self.style==5)
        return self.shared('face-list','suObList',contents) if self.rich() else bytes(2)

    def corner(self, key: str, edited: bool) -> bytes:
        """Preserve stored corner zero independently of its native loaded value; e.g. graph.corner('first',False)."""
        raw=self.base(key)+self.face_list()+self.face_list()
        option=12345 if edited else (0,2,0,-1,-2147483648,2147483647)[self.style]
        raw+=struct.pack('<2i2Q3i',WORDS[self.style],option,*BITS[:2],*WORDS[:3])
        def geometry() -> bytes:
            return self.shared('planar-geometry',CLASSES[2],self.geometry)
        entries=[geometry(),bytes(2),geometry()] if self.rich() else []
        return raw+counted(entries,self.style==5)

    def record(self, kind: int, key: str, edited: bool) -> bytes:
        """Build complete concrete bodies before returning map size; e.g. graph.record(4,'first',False)."""
        if kind==3:return self.hem(key,edited)
        def contents() -> bytes:
            if kind==0:return self.corner(key,edited)
            if kind in (1,2):return self.geometry(edited)
            if kind==4:return self.hem_data(self.hem('hem-owner'),edited)
            if kind==5:return self.chain_fields(edited)
            if kind==6:return self.chain_array()
            return self.edge_pointer() if kind==7 else self.relief(edited)
        return self.shared(key,CLASSES[kind],contents)


def hem_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, extended: bool=False) -> tuple[bytes,int]:
    """Keep distinct and repeated roots across all nine native bindings; e.g. hem_graph(3,19000,5)."""
    graph=HemGraph(version,style,first_slot,extended)
    def record(key: str) -> bytes:
        return graph.record(kind,key,edited and key=='first')
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1
