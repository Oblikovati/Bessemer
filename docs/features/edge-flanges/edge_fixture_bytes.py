"""Complete edge-flange families assembled from existing independent native bodies (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap,body
from flange_fixture_bytes import FlangeFixtureOptions,group_wire,flange_profile
from sheet_fixture_bytes import SheetFixtureOptions,common_bend,component_reference
from sweep_fixture_bytes import shared_object,dimension
from freeze_model_references import wrapper
from freeze_geometric_references import wire as geometric
from display_dimension_fixture_bytes import body as strip_tag
from owned_name_edit import short_wide_name
from weld_corner_bytes import counted
from one_bend_bytes import prefix,ordinary_handle,continuation,raw_tail

CLASSES=('moEdgeFlange_c','moEdgeFlangeData_c','moOneBend_c','moMultiEdgeFlangeDataPerEdge_c','moEdgeGeom_c','moLinearEdgeGeom_c')
VERSIONS=(0x27cc,0x2f5f,0x2f60,0x334b,0x334c,0x334e,0x334f,0x3356,0x3357,0x335a,0x335b,0x372e,0x372f,0x3734,0x3735,0x42cb,0x42cc,19000,0xffffffff)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)


class EdgeGraph:
    """Allocate actual native targets and preserve direct versus tagged ownership."""
    def __init__(self, version: int, style: int, first_slot: int, extended: bool=False) -> None:
        self.version=version;self.style=style;self.extended=extended
        self.mapping=WireMap();self.mapping.slot=first_slot
        self.ids: dict[str,int]={}

    def shared(self, key: str, name: str, contents: Callable[[],bytes]) -> bytes:
        """Reserve real identities before reading nested objects; e.g. graph.shared(key,name,body)."""
        return shared_object(self.mapping,self.ids,key,name,contents)

    def bits(self) -> int:
        """Choose exact IEEE payload bits without arithmetic; e.g. graph.bits()."""
        return BITS[self.style]

    def marker(self) -> bytes:
        """Use one actual shared node-name object for generic references; e.g. graph.marker()."""
        if self.style==0:return bytes(2)
        return self.shared('marker','moNodeName_c',lambda:short_wide_name('Ω bend marker'))

    def component(self) -> bytes:
        """Reuse the complete native component-feature prefix; e.g. graph.component()."""
        return component_reference(self.mapping,self.version,self.ids,'moCompFeature_c') if self.style in (1,3,5) else bytes(2)

    def display(self, angular: bool=False) -> bytes:
        """Share full typed dimension bodies; e.g. graph.display(True)."""
        if self.style in (0,4):return bytes(2)
        return dimension(self.mapping,self.version,self.ids,'moDisplayAngularDim_c' if angular else 'moDisplayDistanceDim_c')

    def reference(self) -> bytes:
        """Keep the canonical wrapper body and its cached occurrences; e.g. graph.reference()."""
        if self.style in (0,4):return bytes(2)
        return self.shared('wrapper','moRefWrapper_c',lambda:wrapper(self.version,False))

    def point(self) -> bytes:
        """Use a complete typed point reference; e.g. graph.point()."""
        if self.style in (0,4):return bytes(2)
        return self.shared('point','moPointRef_w',lambda:strip_tag(geometric('moPointRef_w',self.version,False,0)))

    def array(self) -> bytes:
        """Emit only selected tagged entries inside a direct counted array; e.g. graph.array()."""
        entries=[] if self.style in (0,4) else [self.marker(),bytes(2),self.marker()]
        return counted(entries,self.style==5)

    def listing(self) -> bytes:
        """Share one tagged list without adding a second direct body; e.g. graph.listing()."""
        if self.style in (0,4):return bytes(2)
        return self.shared('list','suObList',self.array)

    def data_prefix(self, owner: bytes, edited: bool) -> bytes:
        """Keep nonmonotonic fields and the required owner before the direct data arrays; e.g. graph.data_prefix(owner,False)."""
        raw=owner+(b'' if self.extended else bytes(2))+self.display()
        raw+=struct.pack('<i4Q',12345 if edited else WORDS[self.style],*(BITS[(self.style+i)%6] for i in range(4)))
        raw+=struct.pack('<5i',*WORDS[:5])+self.reference()+self.point()+struct.pack('<i',-1)+self.reference()+self.point()
        return raw+struct.pack('<5i',*WORDS[1:6])+self.display()+self.reference()+struct.pack('<3i',-1,0,2147483647)

    def data_fields(self, owner: bytes, edited: bool) -> bytes:
        """Keep all three late words and the helper-selected sheet pointer; e.g. graph.data_fields(owner,False)."""
        raw=self.data_prefix(owner,edited)
        if self.version>0x2f5f:raw+=struct.pack('<i',-32769)
        raw+=self.array()+self.display()+struct.pack('<4i',-2147483648,2147483647,-1,0)
        if self.version>0x3356:raw+=struct.pack('<i',-1)
        raw+=self.reference()+struct.pack('<2iQ2i',-1,0,self.bits(),2147483647,-2147483648)
        return raw+(struct.pack('<i',-1) if self.version>0x42cb else b'')

    def edge(self, key: str, edited: bool=False) -> bytes:
        """Write feature data directly after the full bend group; e.g. graph.edge('first')."""
        def contents() -> bytes:
            owner=self.ids[key];raw=body(self.mapping,self.version,self.style in (3,5),owner)
            raw+=group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions(extended=self.extended))
            raw+=self.data_fields(struct.pack('<H',owner),edited)+self.array()
            return raw+(struct.pack('<i',-2147483648) if self.version>=0x372f else b'')
        return self.shared(key,CLASSES[0],contents)

    def one_bend(self, edited: bool) -> bytes:
        """Compose the full inherited body before the two conditional direct handles; e.g. graph.one_bend(False)."""
        raw=body(self.mapping,self.version,self.style in (3,5),self.mapping.slot)
        raw+=common_bend(self.mapping,self.version,self.ids,SheetFixtureOptions(self.extended,False,0))+struct.pack('<3i',*WORDS[:3])
        return raw+prefix(self)+ordinary_handle(self)+continuation(self)+raw_tail(self.version,self.style,edited)

    def geometry(self, edited: bool) -> bytes:
        """Keep both exact native geometry bindings at 124 bytes; e.g. graph.geometry(False)."""
        return struct.pack('<i15Q',12345 if edited else WORDS[self.style],*(BITS[(self.style+i)%6] for i in range(15)))

    def per_edge(self, edited: bool) -> bytes:
        """A nonzero signed late flag adds a distance-dimension tag; e.g. graph.per_edge(False)."""
        raw=flange_profile(self.mapping,self.version,self.ids,self.style in (1,3,5))+bytes(2)+struct.pack('<i',12345 if edited else WORDS[self.style])
        raw+=bytes(2) if self.style==0 else self.shared('geometry',CLASSES[5],lambda:self.geometry(False))
        if self.version>=0x42cc:
            flag=0 if self.style in (0,4) else WORDS[self.style];raw+=struct.pack('<i',flag)
            if flag:raw+=self.display()
        return raw

    def record(self, kind: int, key: str, edited: bool) -> bytes:
        """Stop after each concrete native body, with no invented inherited fields; e.g. graph.record(1,'first',False)."""
        if kind==0:return self.edge(key,edited)
        def contents() -> bytes:
            if kind==1:return self.data_fields(self.edge('owner'),edited)
            if kind==2:return self.one_bend(edited)
            return self.per_edge(edited) if kind==3 else self.geometry(edited)
        return self.shared(key,CLASSES[kind],contents)


def edge_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, extended: bool=False) -> tuple[bytes,int]:
    """Keep fresh, distinct and repeated roots across all native field gates; e.g. edge_graph(2,19000,5)."""
    graph=EdgeGraph(version,style,first_slot,extended)
    def record(key: str) -> bytes:
        return graph.record(kind,key,edited and key=='first')
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=graph.mapping.new('suObArray',array) if style in (2,5) else record('first')
    return raw,graph.mapping.slot+1
