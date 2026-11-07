"""Complete mitered-flange bodies using existing bend and handle-array fixtures (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from flange_fixture_bytes import FlangeFixtureOptions,group_wire,handle_collection
from member_reference_bytes import MemberFixtureOptions,member_profile
from reference_curve_bytes import dwords

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)


class MiteredFlangeGraph(EdgeGraph):
    """Keep three direct arrays, two DWORD arrays and four handle groups distinct."""
    def leading(self) -> bytes:
        raw=member_profile(self.mapping,self.version,self.ids,MemberFixtureOptions(rich=self.style not in (0,4)))
        raw+=struct.pack('<4Q',*(BITS[(self.style+i)%6] for i in range(4)))
        raw+=struct.pack('<8i',*(WORDS[(self.style+i)%6] for i in range(8)))
        raw+=struct.pack('<2Q2i',BITS[self.style],BITS[(self.style+1)%6],-1,2147483647)
        return raw+self.display()+self.display()+struct.pack('<i',-2147483648)

    def collections(self) -> bytes:
        raw=self.array()+dwords(self.style)+dwords((self.style+1)%6)+self.array()
        counts=((0,0,0,0),(0,1,2,3),(1,0,3,2),(3,2,1,0),(0,0,1,0),(2,3,2,3))[self.style]
        for count in counts:
            raw+=handle_collection(self.mapping,self.version,self.ids,count)
        return raw+self.array()

    def mitered(self, edited: bool) -> bytes:
        raw=body(self.mapping,self.version,self.style in (3,5),self.ids['root'])
        raw+=group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions(rich=self.style%2==1))
        return raw+self.leading()+self.collections()+struct.pack('<i',12345 if edited else WORDS[self.style])


def mitered_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete records with independent direct-collection framing."""
    if kind!=0:raise ValueError(f'mitered-flange fixture kind {kind}; expected 0')
    graph=MiteredFlangeGraph(version,style,first_slot)
    raw=graph.shared('root','moSMMiteredFlange_c',lambda:graph.mitered(edited))
    return raw,graph.mapping.slot+1
