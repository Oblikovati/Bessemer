"""Complete concentric mates and plane sections with direct owned geometry (#54)."""
import struct
from collections.abc import Callable
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from freeze_environment import feature
from axis_owner_bytes import geometry_header
from reference_plane_fixture_bytes import base as plane_data
from freeze_geometric_references import surface,wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from kernel_geometry_fixture import frame
from owned_name_edit import short_wide_name

CLASSES=('moMateConcentric','moPlaneSection_c','moPlaneSectionEdge_c','moPlaneSectionIdKeeper_c')
VERSIONS=(0x27cc,0x2b83,0x2b84,0x2ba5,0x2ba6,0x2bb4,0x2bb5,0x2f85,0x2f86,0x3afc,0x3afd,19000,0xffffffff)
PRESENCE=(0,1,255,128,0,254)
BODY_COUNTS=(0,1,2,1,0,3)


class MatePlaneGraph(EdgeGraph):
    """Share complete reference targets without introducing tags for direct members."""
    def vector(self, count: int=3) -> bytes:
        return struct.pack('<'+'Q'*count,*(BITS[(self.style+i)%6] for i in range(count)))

    def signed_pointers(self, build: Callable[[],bytes]) -> bytes:
        count=(0,2,3,1,0,2)[self.style]
        return struct.pack('<i',count)+b''.join(build() for _ in range(count))

    def mate_base(self, edited: bool) -> bytes:
        self.mapping.slot+=2
        raw=feature(self.version,False)+struct.pack('<3i',-2,12345 if edited else WORDS[self.style],WORDS[(self.style+1)%6])
        if self.version>0x2bb4:raw+=struct.pack('<i',-2147483648)
        if self.version>0x3afc:
            values=WORDS[:(0,1,3,2,0,3)[self.style]];raw+=struct.pack('<i',len(values))+b''.join(struct.pack('<i',v) for v in values)
        raw+=struct.pack('<2HB',65535,32768,PRESENCE[self.style])
        if PRESENCE[self.style]:raw+=self.vector()
        count=(0,2,2,3,0,1)[self.style];raw+=struct.pack('<i',count)
        for index in range(count):raw+=struct.pack('<i',WORDS[(self.style+index)%6])+self.reference()
        return raw

    def mate(self, edited: bool) -> bytes:
        raw=self.mate_base(edited)
        raw+=self.shared('mate-array','suObArray',self.array) if self.style not in (0,4) else bytes(2)
        raw+=self.signed_pointers(self.reference)+self.signed_pointers(self.listing)+bytes(6)
        raw+=struct.pack('<Hi',65535,-2147483648)+short_wide_name('Ω mate\0')+b'\x03A\0A'
        limit=(-1,0,-2147483648,1,-1,2147483647)[self.style]
        raw+=struct.pack('<2i',WORDS[self.style],limit)
        return raw+(struct.pack('<i',-32769)+self.vector(2)+struct.pack('<2i',-2147483648,2147483647) if limit!=-1 else b'')

    def section_geometry(self) -> bytes:
        count=BODY_COUNTS[self.style];raw=self.vector(16)+struct.pack('<i',count)
        for _ in range(count):raw+=frame()+self.array()
        return raw+self.vector(4)+self.vector()

    def edge(self, edited: bool=False) -> bytes:
        raw=self.shared('surface','moGeneralSurfRef_w',lambda:surface(self.version,False)) if self.style not in (0,4) else bytes(2)
        count=1 if edited else BODY_COUNTS[self.style]
        return raw+struct.pack('<i',count)+frame()*count

    def edge_pointer(self) -> bytes:
        return self.shared('edge',CLASSES[2],self.edge)

    def string_map(self, integer_key: bool) -> bytes:
        count=0 if self.style in (0,4) else 2;raw=struct.pack('<i',count)
        for index in range(count):
            word=struct.pack('<i',WORDS[(self.style+index)%6]);text=short_wide_name('Ω duplicate\0')
            raw+=word+text if integer_key else text+word
        return raw

    def maps_and_lists(self) -> bytes:
        raw=self.string_map(True)+self.string_map(False)
        count=0 if self.style in (0,4) else 2;raw+=struct.pack('<i',count)
        for _ in range(count):raw+=struct.pack('<i',-1)+self.marker()
        references=(0,1,2,1,0,2)[self.style]
        return raw+self.array()+struct.pack('<i',references)+bytes(2*references)+struct.pack('<i',references)+bytes(4*references)

    def section(self, edited: bool) -> bytes:
        keeper=lambda:self.shared('keeper',CLASSES[3],lambda:b'')
        raw=geometry_header(self.mapping,self.version,self.style in (3,5),keeper=keeper if self.style in (3,5) else None)
        raw+=self.shared('plane-data','moRefPlnData_c',lambda:plane_data(self.version,False)) if self.style not in (0,4) else bytes(2)
        raw+=self.section_geometry()+struct.pack('<i',12345 if edited else WORDS[self.style])+self.signed_pointers(self.edge_pointer)
        raw+=self.shared('line','moLineRef_w',lambda:strip_tag(geometric('moLineRef_w',self.version,False,0))) if self.style not in (0,4) else bytes(2)
        return raw+struct.pack('<2i',-2147483648,2147483647)+self.vector(1)+struct.pack('<i',-1)+self.maps_and_lists()


def mate_plane_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose all four documented tagged bodies while keeping section data directly owned."""
    graph=MatePlaneGraph(version,style,first_slot)
    build=(lambda:graph.mate(edited)) if kind==0 else (lambda:graph.section(edited)) if kind==1 else (lambda:graph.edge(edited)) if kind==2 else (lambda:b'')
    raw=graph.shared('root',CLASSES[kind],build)
    return raw,graph.mapping.slot+1
