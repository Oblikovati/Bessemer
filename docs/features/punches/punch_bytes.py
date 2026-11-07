"""Complete punch records with independent internal revision and file-version gates (#54)."""
import struct
from edge_fixture_bytes import EdgeGraph,BITS,WORDS
from fillet_fixture_bytes import body
from draft_reference_bytes import DraftOptions,keeper

VERSIONS=(0x27cc,0x3746,0x3747,19000,0xffffffff)
REVISIONS=(-1,0,1,2,3,4,5,6,7,8)


def dynamic_array(style: int, points: bool) -> bytes:
    """Keep element widths independent of metadata and zero-count payloads."""
    unit=24 if points else 4
    width=(0,unit,unit,2*unit,(0xffffffff//unit)*unit,unit)[style]
    if not width:return bytes(4)
    count=(0,0,2,1,0,1)[style]
    raw=struct.pack('<4I',width,0xffffffff,0x80000000,count)
    for index in range(count*(width//unit)):
        raw+=struct.pack('<3Q',*(BITS[(style+index+axis)%6] for axis in range(3))) if points else struct.pack('<i',WORDS[(style+index)%6])
    return raw


class PunchGraph(EdgeGraph):
    """Reserve actual targets before direct arrays and repeated extension tags."""
    revision: int

    def nullable_array(self) -> bytes:
        presence=(0,1,-2147483648,-1,2147483647,0)[self.style]
        return struct.pack('<i',presence)+(self.array() if presence else b'')

    def leading(self) -> bytes:
        raw=struct.pack('<i',self.revision)+self.display()+self.display()
        if self.revision>=1:raw+=struct.pack('<i',WORDS[self.style])
        raw+=self.nullable_array()
        if self.revision<3:raw+=self.nullable_array()
        return raw+struct.pack('<2i',-2147483648,2147483647)+self.nullable_array()

    def values(self) -> bytes:
        if self.revision<2:return struct.pack('<3Qi',*BITS[:3],WORDS[self.style])
        return dynamic_array(self.style,True)+dynamic_array(self.style,False)

    def extensions(self, edited: bool) -> bytes:
        raw=b''
        if self.style not in (0,4):
            raw+=struct.pack('<2iiQ',0x3d2,WORDS[self.style],0x13e,BITS[self.style])
            raw+=struct.pack('<i',0x1466)+self.marker()
            if self.style in (2,5):raw+=struct.pack('<iH',0x1466,self.ids['root'])+struct.pack('<2i',0x3d2,-1)
        return raw+struct.pack('<i',12345 if edited else (0,-1,-2147483648,2147483647,42,0)[self.style])

    def payload(self, edited: bool) -> bytes:
        raw=self.leading()+self.values()
        for minimum in (4,5,6,6):
            if self.revision>=minimum:raw+=struct.pack('<i',WORDS[self.style])
        if self.revision>=7:raw+=self.shared('tagged-array','suObArray',self.array) if self.style not in (0,4) else bytes(2)
        raw+=self.extensions(edited)
        if self.revision<2:raw+=keeper(self.mapping,self.version,self.ids,DraftOptions(style=self.style))
        return raw


def punch_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Compose the complete inherited feature and one independently selected revision."""
    graph=PunchGraph(version,style,first_slot);graph.revision=REVISIONS[kind]
    raw=graph.shared('root','moPunch_c',lambda:body(graph.mapping,version,style in (3,5),graph.ids['root'])+graph.payload(edited))
    return raw,graph.mapping.slot+1
