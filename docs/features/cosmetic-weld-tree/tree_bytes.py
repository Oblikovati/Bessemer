"""Complete cosmetic tree graphs using existing independent inherited builders (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object,dimension
from freeze_environment import feature
from weldment_fixture_bytes import custom_list
from weld_symbol_bytes import symbol_fields
from display_item_fixture_bytes import object_base,item
from display_weld_bytes import suffix as display_suffix,VERSIONS
from kernel_geometry_fixture import frame
from owned_name_edit import short_wide_name
from weld_corner_bytes import counted
from parameter_bytes import BITS,WORDS,parameters,endpoint,wire_map

CLASSES=('moCosmeticWeldSubFolder_c','moCosmeticWeldBead_c','moCosmeticWeldBeadData_c',
         'moCWeldBeadFromToParams_c','moIntermittentCWeldBeadParams_c','moWeldWireEdgeToFaceMap_c')
DISPLAY_CLASSES=('moDisplayDim_c','moDisplayDistanceDim_c','moDisplayRadialDim_c','moDisplayAngularDim_c','moDisplayRevolveDim_c')


class WeldFixtureGraph:
    """Reserve every object in wire order, including real shared dependencies."""
    def __init__(self, version: int, style: int, first_slot: int) -> None:
        self.version=version;self.style=style
        self.mapping=WireMap();self.mapping.slot=first_slot
        self.ids: dict[str,int]={}

    def shared(self, key: str, name: str, contents: Callable[[],bytes]) -> bytes:
        """Reserve an actual class/object pair only for its first occurrence; e.g. graph.shared(key,name,body)."""
        return shared_object(self.mapping,self.ids,key,name,contents)

    def marker(self) -> bytes:
        """Use a real named object as a generic shared array entry; e.g. graph.marker()."""
        return self.shared('marker','moNodeName_c',lambda:short_wide_name('Ω bead marker'))

    def direct_array(self, owner: int, ordinal: int) -> bytes:
        """Keep array bodies direct, with null/shared/self entries; e.g. graph.direct_array(2,0)."""
        if self.style==0:return bytes(2)
        if (self.style+ordinal)%5==0:return counted([],self.style in (4,5))
        entries=[self.marker(),bytes(2),struct.pack('<H',owner),self.marker()]
        return counted(entries[:(self.style+ordinal)%5],self.style in (4,5))

    def property_pointer(self) -> bytes:
        """Share an actual tagged property list between subfolders; e.g. graph.property_pointer()."""
        if self.style in (0,4):return bytes(2)
        return self.shared('properties','moCustomPropertyList',lambda:custom_list(self.style%4))

    def parameter_pointer(self, intermittent: bool) -> bytes:
        """Share exact typed parameter objects; e.g. graph.parameter_pointer(False)."""
        if self.style in (0,4):return bytes(2)
        kind=4 if intermittent else 3
        return self.shared(CLASSES[kind],CLASSES[kind],lambda:parameters(intermittent,self.style))

    def symbol_pointer(self) -> bytes:
        """Use the existing complete cosmetic-symbol constructor; e.g. graph.symbol_pointer()."""
        if self.style in (0,4):return bytes(2)
        return self.shared('symbol','moCosmeticWeldBeadSymbol_c',lambda:
            symbol_fields(self.mapping,self.ids,self.version,1,0,self.ids['symbol'],False))

    def display_pointer(self) -> bytes:
        """Use complete existing display-item implementations; e.g. graph.display_pointer()."""
        if self.style in (0,4):return bytes(2)
        if self.style in (3,5):return dimension(self.mapping,self.version,self.ids,DISPLAY_CLASSES[VERSIONS.index(self.version)%5])
        def contents() -> bytes:
            raw=object_base(self.mapping,False,self.ids['display'])+item(self.mapping,self.version,False)
            return raw+display_suffix(self.style%3,False,bytes(2))
        return self.shared('display','moDisplayWeld_c',contents)

    def feature_prefix(self) -> bytes:
        """Account for the existing feature builder's real tagged node name; e.g. graph.feature_prefix()."""
        self.mapping.slot+=2
        return feature(self.version,self.style%2==1)

    def data_prefix(self, owner: int, edited: bool) -> bytes:
        """Place the two constructor-owned endpoints between direct arrays and tagged parameters; e.g. graph.data_prefix(2,False)."""
        raw=self.direct_array(owner,0)+self.direct_array(owner,1)
        raw+=endpoint(self.style)+endpoint(0 if self.style==0 else (self.style+1)%6)
        raw+=struct.pack('<i',12345 if edited else WORDS[self.style])
        return raw+self.parameter_pointer(False)+self.parameter_pointer(True)+self.symbol_pointer()

    def data_fields(self, owner: int, edited: bool) -> bytes:
        """Preserve the nine direct arrays and strict kernel-body boundary; e.g. graph.data_fields(2,False)."""
        raw=self.data_prefix(owner,edited)+struct.pack('<Q',BITS[self.style])
        raw+=frame() if self.style in (1,3,5) else b'\xff'
        raw+=self.direct_array(owner,2)+struct.pack('<Q',BITS[(self.style+1)%6])
        raw+=b''.join(self.direct_array(owner,i) for i in range(3,6))
        raw+=struct.pack('<i',WORDS[(self.style+1)%6])
        return raw+b''.join(self.direct_array(owner,i) for i in range(6,9))

    def record_fields(self, kind: int, key: str, edited: bool) -> bytes:
        """Select one complete native body without adding a fictional base; e.g. graph.record_fields(2,'first',False)."""
        if kind>=3:return wire_map(self.style,edited) if kind==5 else parameters(kind==4,self.style,edited)
        if kind==2:return self.data_fields(self.ids[key],edited)
        raw=self.feature_prefix()
        if kind==0:return raw+self.property_pointer()+struct.pack('<i',12345 if edited else WORDS[self.style])
        raw+=self.shared('data','moCosmeticWeldBeadData_c',lambda:self.data_fields(self.ids['data'],False)) if self.style not in (0,4) else bytes(2)
        return raw+self.display_pointer()+struct.pack('<2i',12345 if edited else WORDS[self.style],WORDS[(self.style+1)%6])


def tree_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep distinct equal roots, true cached references and seeded arrays; e.g. tree_graph(2,19000,5)."""
    graph=WeldFixtureGraph(version,style,first_slot)
    def record(key: str) -> bytes:
        return graph.shared(key,CLASSES[kind],lambda:graph.record_fields(kind,key,edited and key=='first'))
    def array() -> bytes:
        entries=([graph.marker()] if style in (3,5) else [])+[record('first')]
        if style!=3:entries+=[record('second'),record('first')]
        return struct.pack('<H',len(entries))+b''.join(entries)
    raw=graph.mapping.new('suObArray',array) if style in (2,3,5) else record('first')
    return raw,graph.mapping.slot+1
