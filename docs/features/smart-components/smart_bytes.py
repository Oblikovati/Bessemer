"""Independent full smart-component graphs with direct containers and owner cycles (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,WORDS
from freeze_environment import feature
from owned_name_edit import short_wide_name
from freeze_external_references import wire as external_wire
from display_dimension_fixture_bytes import body as strip_tag
from freeze_model_references import wrapper
from freeze_views import transform
from weld_corner_bytes import counted

CLASSES=('moSmartComponentFeature_c','moSmartComponentFeatureData_c','moSmartComponentFeatFolder_c','moSmartComponentCompFolder_c','moSmartComponentRefFolder_c','moSCItemData_c','moSCFeatureItemData_c','moSCCompItemData_c','moSCRefItemData_c','moSensorDiaMapList_c')
VERSIONS=(0x27cc,0x2f82,0x2f83,0x332f,0x3330,0x3f12,0x3f13,19000,0xffffffff)


class SmartGraph(EdgeGraph):
    """Reserve actual identities before traversing shared and cyclic members."""
    def __init__(self, kind: int, version: int, style: int, edited: bool, first_slot: int) -> None:
        super().__init__(version,style,first_slot)
        self.kind=kind;self.edited=edited

    def target(self, kind: int) -> bytes:
        """Reuse a matching containing root rather than create a substitute owner."""
        if self.kind==kind:return struct.pack('<H',self.ids['root'])
        return self.shared('target-'+str(kind),CLASSES[kind],lambda:self.contents(kind,False))

    def node(self, edited: bool) -> bytes:
        """Use a real shared node-name followed by the complete modern node body."""
        name=self.shared('node-name','moNodeName_c',lambda:short_wide_name('Ω smart node'))
        return name+struct.pack('<iIiI',-3,0x80000001,12345 if edited else -7,0xfffffffe)+short_wide_name('tail')

    def feature_prefix(self) -> bytes:
        """Count the actual fresh node-name embedded in the full feature helper."""
        self.mapping.slot+=2
        return feature(self.version,False)

    def external(self) -> bytes:
        """Preserve a complete saved external object without loading a file."""
        if self.style in (0,4):return bytes(2)
        return self.shared('external','moExtObject_c',lambda:strip_tag(external_wire(self.version,0)))

    def context(self) -> bytes:
        """Use the complete existing context-chain component grammar."""
        if self.style not in (3,5):return bytes(2)
        return self.shared('context','moContextChainComponent_c',lambda:bytes(4)+struct.pack('<I',0xfedcba98)+self.external()+struct.pack('<i',-1))

    def xform(self) -> bytes:
        """Share the complete canonical transform wrapper and both operators."""
        if self.style in (0,4):return bytes(2)
        return self.shared('xform','moCompXFormWrapper_c',lambda:wrapper(self.version,False)+transform(True,0x1a)+transform(False,7))

    def container(self, kinds: tuple[int,...]) -> bytes:
        """Encode direct ReadCount arrays/lists with nulls and cached entries."""
        if self.style in (0,4):return counted([],self.style==4)
        values=[self.target(kind) for kind in kinds]
        return counted(values+[bytes(2),self.target(kinds[0])],self.style==5)

    def sensor_map(self, edited: bool) -> bytes:
        """Retain duplicate keys and shared/null values in wire order."""
        if self.style in (0,4):return counted([],self.style==4)
        first=self.shared('map-item',CLASSES[5],lambda:self.node(edited))
        entries=[short_wide_name('Ω sensor')+first,short_wide_name('Ω sensor')+bytes(2),b'\x00'+struct.pack('<H',self.ids['map-item'])]
        return counted(entries,self.style==5)

    def feature_data(self, edited: bool) -> bytes:
        """Keep every direct container and typed folder in native order."""
        raw=self.container((6,7))+self.container((8,))+self.context()+self.container((5,))
        raw+=self.target(2)+self.target(3) if self.style not in (0,4) else bytes(4)
        raw+=struct.pack('<2i',WORDS[self.style],-1)+self.sensor_map(False)
        raw+=self.target(4) if self.style not in (0,4) else bytes(2)
        return raw+struct.pack('<i',12345 if edited else WORDS[self.style])

    def folder(self, kind: int, edited: bool) -> bytes:
        """Keep complete concept hooks before the folder's direct array."""
        raw=self.feature_prefix()+struct.pack('<2i',-1,12345 if edited else WORDS[self.style])
        return raw+self.container(((6,7,8)[kind-2],))

    def item(self, kind: int, edited: bool) -> bytes:
        """Preserve each distinct item suffix, including references to its owner."""
        raw=self.node(edited and kind in (5,8))
        if kind==5:return raw
        if kind==6:return raw+self.component()+short_wide_name('Ω feature')+struct.pack('<i',12345 if edited else WORDS[self.style])+self.owner()+b'\x03abc'
        if kind==7:return raw+self.xform()+self.owner()+short_wide_name('Ω component')+struct.pack('<i',12345 if edited else WORDS[self.style])+self.xform()
        return raw+short_wide_name('Ω reference')+b'\x03abc'+self.owner()+short_wide_name('Ω final')+self.container((5,))

    def owner(self) -> bytes:
        """Permit null only in the nullable owner field, retaining real cycles otherwise."""
        return bytes(2) if self.style in (0,4) else self.target(0)

    def contents(self, kind: int, edited: bool) -> bytes:
        """Compose complete class bodies without constructor-only placeholder objects."""
        if kind==0:return self.feature_prefix()+struct.pack('<i',12345 if edited else -2147483648)+self.target(1)+self.external()
        if kind==1:return self.feature_data(edited)
        if kind in (2,3,4):return self.folder(kind,edited)
        if kind==9:return self.sensor_map(edited)
        return self.item(kind,edited)


def smart_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Allocate real roots before inherited and cyclic members; e.g. smart_object(0,19000,2)."""
    graph=SmartGraph(kind,version,style,edited,first_slot)
    raw=graph.shared('root',CLASSES[kind],lambda:graph.contents(kind,edited))
    return raw,graph.mapping.slot+1
