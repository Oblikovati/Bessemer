"""Complete library-feature and cross-reference wire graphs (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from owned_name_edit import short_wide_name
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag

CLASSES=('moLibraryFeature_c','moXConfigData_c','moXFeatureData_c','moXRefPlaneData_c','moXRefData_c','moAtomIdToFeatIdNode_c','moAtomIdToFeatIds_c')
VERSIONS=(0x27cc,0x2b5c,0x2b5d,0x2b5f,0x2b60,0x2b61,0x2b7b,0x2b7c,0x2b7e,0x2b7f,0x2bab,0x2bac,0x2bc8,0x2bc9,0x42df,0x42e0,19000,0xffffffff)

class LibraryGraph(EdgeGraph):
    """Allocate real class bodies before shared references and direct members."""
    def number(self, edited: bool=False) -> int:
        return 12345 if edited else WORDS[self.style]

    def feature_record(self, edited: bool=False) -> bytes:
        raw=short_wide_name('Ω feature\0')+struct.pack('<HIHI',65535,0xffffffff,32768,0x80000000)
        return raw+(struct.pack('<i',self.number(edited)) if self.version>0x2bc8 else b'')

    def configuration(self, edited: bool) -> bytes:
        raw=struct.pack('<H',65535)+short_wide_name('Ω configuration')+struct.pack('<Ii',0xffffffff,0 if self.style in (0,4) else 2)
        if self.style not in (0,4):
            for index in range(2):raw+=struct.pack('<i' if self.version>0x2bc8 else '<H',self.number(edited and index==0) if self.version>0x2bc8 else 65535)+self.shared('xfeature',CLASSES[2],self.feature_record)
        return raw+self.array()+struct.pack('<H2I',2,0xffffffff,0x80000000)

    def plane(self, edited: bool) -> bytes:
        return struct.pack('<3i',self.number(edited),-2147483648,2147483647)

    def cross_reference(self, edited: bool) -> bytes:
        pointer=lambda:self.shared('single-face','moSingleFaceRef_w',lambda:strip_tag(geometric('moSingleFaceRef_w',self.version,False,0))) if self.style in (3,5) else self.reference()
        raw=struct.pack('<H',65535)+short_wide_name('Ω reference\0')+pointer()+pointer()+bytes([255])
        return raw+struct.pack('<3Q',*(BITS[(self.style+i)%6] for i in range(3)))

    def atom_ids(self, edited: bool=False) -> bytes:
        n=0 if self.style in (0,4) else 2
        raw=struct.pack('<i',n)+b''.join(struct.pack('<i',self.number(edited and index==0)) for index in range(n))+struct.pack('<i',n)
        for _ in range(n):raw+=struct.pack('<4i',-1,2,-2147483648,2147483647)
        return raw+(struct.pack('<i',n)+bytes(2*n) if self.version>0x2b7e else b'')

    def atom_node(self, edited: bool=False) -> bytes:
        return struct.pack('<i',self.number(edited))+self.shared('atom-ids',CLASSES[6],self.atom_ids)

    def conditional_library(self) -> bytes:
        raw=self.array()+struct.pack('<I',0x80000000)+short_wide_name('Ω library')+struct.pack('<4Hi',65535,32768,0,1,-2147483648)
        raw+=struct.pack('<3Q',*(BITS[(self.style+i)%6] for i in range(3)))+self.array()
        return raw+self.shared('xplane',CLASSES[3],lambda:self.plane(False))

    def library_tail(self) -> bytes:
        raw=short_wide_name('Ω tail')
        if self.version>0x2b5c:
            raw+=struct.pack('<2iH',-2147483648,2147483647,0)
            if self.version>0x2b5f:raw+=struct.pack('<2i',-1,0)
        if self.version>0x2b60:raw+=struct.pack('<7i',2,-1,0,1,-1,0,1)
        if 0x2b7b<self.version<0x2bac:raw+=self.atom_ids()
        if self.version>0x2bab:
            raw+=struct.pack('<i',2)+self.shared('atom-node',CLASSES[5],self.atom_node)+self.shared('atom-node',CLASSES[5],self.atom_node)
        return raw+(bytes([255]) if self.version>0x42df else b'')

    def library(self, edited: bool) -> bytes:
        raw=body(self.mapping,self.version,self.style in (3,5),self.ids['root'])
        mode=(0,0,0,5,65535,32768)[self.style]
        raw+=struct.pack('<H',mode)+self.array()
        if mode==0:raw+=self.conditional_library()
        return raw+self.library_tail()


def library_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose all seven complete readers; e.g. library_object(0,19000,2)."""
    graph=LibraryGraph(version,style,first_slot)
    builders=(graph.library,graph.configuration,graph.feature_record,graph.plane,graph.cross_reference,graph.atom_node,graph.atom_ids)
    raw=graph.shared('root',CLASSES[kind],lambda:builders[kind](edited))
    return raw,graph.mapping.slot+1
