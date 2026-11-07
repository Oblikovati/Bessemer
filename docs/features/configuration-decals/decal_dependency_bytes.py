"""Complete ordinary decal dependencies with owned MFC identities (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_component_references import base as component_base
from scalar_bytes import WORDS,BITS,name_bytes

NESTED_CLASSES=('moUnitComponent_c','moCompFace_c','moFaceRef_c','moVertexRef_c','moEndFaceSurfIdRep_c','moNetSideSurfIdRep_c','moFR_c','moFromSktEntSurfIdRep_c','moSimpleSurfIdRep_c','moFromSktEnt3IntSurfIdRep_c')
REP_WIDTHS={'moEndFaceSurfIdRep_c':2,'moNetSideSurfIdRep_c':2,'moFromSktEntSurfIdRep_c':1,'moSimpleSurfIdRep_c':1,'moFromSktEnt3IntSurfIdRep_c':3}


class DecalDependencies:
    """Share complete nested references without reusing unrelated tag identities."""
    def __init__(self, mapping: WireMap, ids: dict[str,int], version: int, style: int) -> None:
        self.mapping=mapping;self.ids=ids;self.version=version;self.style=style

    def pointer(self, name: str, key: str) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,lambda:self.body(name,key))

    def body(self, name: str, key: str) -> bytes:
        if name=='moUnitComponent_c':return bytes(3)
        if name=='moCompFace_c':return self.component_face(key)
        if name=='moFaceRef_c':return self.boundary(key,True)+struct.pack('<i',2)+struct.pack('<iIIi',-2147483648,0xffffffff,0x80000000,2147483647)*2
        if name=='moVertexRef_c':return self.boundary(key,False)+struct.pack('<i3Q',-1,*(BITS[self.style] for _ in range(3)))
        if name=='moFR_c':return bytes(2)+struct.pack('<iI',WORDS[self.style],0xffffffff)
        suffix=struct.pack('<'+'i'*REP_WIDTHS[name],*(WORDS[(self.style+i)%6] for i in range(REP_WIDTHS[name])))
        return bytes(2)+self.pointer('moFR_c','shared-feature-reference')+suffix

    def component_face(self, key: str) -> bytes:
        prefix=component_base(self.version,False)
        alias=(b'\xff'+name_bytes(self.style)) if self.style in (3,5) else bytes(1)
        return prefix+alias+self.pointer('moFaceRef_c',key+'-face')+bytes(2)

    def boundary(self, key: str, populated: bool) -> bytes:
        raw=struct.pack('<5I2Q',0xffffffff,0x80000000,2 if populated else 0,3,5,BITS[self.style],BITS[(self.style+1)%6])
        raw+=self.pointer('moVertexRef_c','shared-vertex') if populated else bytes(2)
        if populated:raw+=self.pointer('moEndFaceSurfIdRep_c',key+'-end')+self.pointer('moFromSktEnt3IntSurfIdRep_c',key+'-three')
        raw+=struct.pack('<Ii',0xfedcba98,0)
        return raw+(struct.pack('<2i',WORDS[self.style],0) if self.version>0x332c else b'')
