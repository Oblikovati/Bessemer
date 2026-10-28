"""Complete native gusset, weld-bead and side-data graphs (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap,body
from body_feature_fixture_bytes import suffix as extrusion_suffix
from sweep_fixture_bytes import shared_object,dimension
from freeze_model_references import wrapper
from freeze_environment import feature
from freeze_views import transform
from display_dimension_fixture_bytes import CLASSES as DISPLAY_CLASSES
from display_item_fixture_bytes import object_base,item
from display_weld_bytes import suffix as display_suffix,STRINGS
from weld_corner_bytes import counted

CLASSES=('moGusset_c','moWeldBeadFeat_c','weldBeadSideData_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2b9a,0x2b9b,0x2faf,0x2fb0,0x3371,0x3372,0x3373,0x3374,
          0x371d,0x371e,0x3747,0x3748,0x3b00,0x3b01,0x3b02,0x3b21,0x3b22,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)
PACKED=(0,0xffffffff,0x80000000,3,2,1)
FLAGS=(0,0x0b,0x1a,0x2b,0x40,0x4c)


class FinishingGraph:
    """Allocate actual inherited and shared objects in native wire order."""
    def __init__(self, version: int, style: int, first_slot: int) -> None:
        self.version=version;self.style=style
        self.mapping=WireMap();self.mapping.slot=first_slot
        self.ids: dict[str,int]={}

    def shared(self, key: str, name: str, contents: Callable[[],bytes]) -> bytes:
        """Preserve the first object identity before reading its children; e.g. graph.shared(key,name,body)."""
        return shared_object(self.mapping,self.ids,key,name,contents)

    def dimension(self, angular: bool=False) -> bytes:
        """Share complete distance/angular bodies across all occurrences; e.g. graph.dimension(True)."""
        if self.style in (0,4):return bytes(2)
        name='moDisplayAngularDim_c' if angular else 'moDisplayDistanceDim_c'
        return dimension(self.mapping,self.version,self.ids,name)

    def list_contents(self, owner: int) -> bytes:
        """Emit only selected entries so skipped children reserve no slots; e.g. graph.list_contents(2)."""
        entries=[]
        for index in range((self.style+1)%5):
            if index==0:entry=self.shared('marker','moPMarkRecord_c',lambda:struct.pack('<i',-2147483648))
            elif index==1:entry=bytes(2)
            elif index==2:entry=struct.pack('<H',owner)
            else:entry=self.dimension()
            entries.append(entry)
        return counted(entries,self.style==5)

    def listing(self, owner: int) -> bytes:
        """Share a tagged native list between typed members; e.g. graph.listing(2)."""
        if self.style in (0,4):return bytes(2)
        return self.shared('list','suObList',lambda:self.list_contents(owner))

    def reference(self) -> bytes:
        """Reuse the existing complete generic wrapper body; e.g. graph.reference()."""
        if self.style in (0,4):return bytes(2)
        return self.shared('wrapper','moRefWrapper_c',lambda:wrapper(self.version,False))

    def gusset_prefix(self, owner: int, edited: bool) -> bytes:
        """Retain the first packed field after the distance dimension; e.g. graph.gusset_prefix(2,False)."""
        raw=self.listing(owner)+self.listing(owner)
        raw+=struct.pack('<3i',12345 if edited else WORDS[self.style],WORDS[(self.style+1)%6],WORDS[(self.style+2)%6])
        raw+=self.dimension()+struct.pack('<QiI',BITS[self.style],WORDS[(self.style+3)%6],PACKED[self.style])
        return raw+b''.join(self.dimension() for _ in range(3))+self.dimension(True)

    def gusset_fields(self, owner: int, edited: bool) -> bytes:
        """Keep the complete seventy-six-byte null-pointer suffix; e.g. graph.gusset_fields(2,False)."""
        raw=self.gusset_prefix(owner,edited)+struct.pack('<I',PACKED[(self.style+1)%6])
        raw+=self.dimension()+struct.pack('<i',WORDS[(self.style+4)%6])
        raw+=self.dimension()+self.dimension()+self.dimension(True)
        return raw+struct.pack('<4i',*WORDS[:4])+self.reference()

    def cabinet(self) -> bytes:
        """Use the actual model-feature cabinet prefix and direct empty arrays; e.g. graph.cabinet()."""
        if self.style in (0,4):return bytes(2)
        def contents() -> bytes:
            self.mapping.slot+=2
            raw=feature(self.version,False)+struct.pack('<i',-2147483648)
            raw+=bytes(2)+struct.pack('<2Q',BITS[self.style],BITS[(self.style+1)%6])+bytes(6)
            return raw+(struct.pack('<i',-32769) if self.version>0x2b9a else b'')
        return self.shared('cabinet','moDetailCabinet_c',contents)

    def display(self) -> bytes:
        """Use complete existing display-item implementations; e.g. graph.display()."""
        if self.style in (0,4):return bytes(2)
        if self.style in (3,5):return dimension(self.mapping,self.version,self.ids,DISPLAY_CLASSES[VERSIONS.index(self.version)%5])
        def contents() -> bytes:
            return object_base(self.mapping,False,self.ids['display'])+item(self.mapping,self.version,False)+display_suffix(self.style%3,False,bytes(2))
        return self.shared('display','moDisplayWeld_c',contents)

    def side_fields(self, owner: int, edited: bool) -> bytes:
        """Three tagged list pointers precede exactly sixteen scalar bytes; e.g. graph.side_fields(2,False)."""
        raw=b''.join(self.listing(owner) for _ in range(3))
        return raw+struct.pack('<2iQ',12345 if edited else WORDS[self.style],WORDS[(self.style+1)%6],BITS[self.style])

    def side(self, owner: int) -> bytes:
        """Share complete side-data objects between both bead sides; e.g. graph.side(2)."""
        if self.style in (0,4):return bytes(2)
        return self.shared('side','weldBeadSideData_c',lambda:self.side_fields(owner,False))

    def bead_fields(self, owner: int, edited: bool) -> bytes:
        """Always serialize the cabinet in the declared helper-clear mode; e.g. graph.bead_fields(2,False)."""
        raw=(bytes(1) if self.style==0 else STRINGS[self.style%3])+self.display()+self.cabinet()
        raw+=transform(self.style%2==1,FLAGS[self.style])+struct.pack('<I',PACKED[self.style])
        raw+=self.side(owner)+self.side(owner)+self.listing(owner)
        raw+=b''.join(self.dimension() for _ in range(5))
        return raw+struct.pack('<2i',12345 if edited else WORDS[self.style],WORDS[(self.style+1)%6])

    def contents(self, kind: int, key: str, edited: bool) -> bytes:
        """Include the extrusion suffix only for the actual gusset; e.g. graph.contents(0,'first',False)."""
        owner=self.ids[key]
        if kind==2:return self.side_fields(owner,edited)
        raw=body(self.mapping,self.version,self.style in (3,5),owner)
        if kind==0:return raw+extrusion_suffix(self.mapping,self.version,'moExtrusion_c',self.style in (3,5))+self.gusset_fields(owner,edited)
        return raw+self.bead_fields(owner,edited)


def finishing_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep real distinct/repeated roots and seeded cached dependencies; e.g. finishing_graph(1,19000,5)."""
    graph=FinishingGraph(version,style,first_slot)
    def record(key: str) -> bytes:
        return graph.shared(key,CLASSES[kind],lambda:graph.contents(kind,key,edited and key=='first'))
    def array() -> bytes:
        entries=([graph.dimension()] if style in (3,5) else [])+[record('first')]
        if style!=3:entries+=[record('second'),record('first')]
        return struct.pack('<H',len(entries))+b''.join(entries)
    raw=graph.mapping.new('suObArray',array) if style in (2,3,5) else record('first')
    return raw,graph.mapping.slot+1
