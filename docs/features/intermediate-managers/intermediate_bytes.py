"""Independent directly owned configuration records at exact reviewed versions (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from scalar_bytes import WORDS,name_bytes

VERSIONS=(11000,18000,19000)


class IntermediateConfigGraph:
    """Keep the direct record tagless while reserving actual list-member identities."""
    def __init__(self, version: int, style: int, edited: bool=False, first_slot: int=0) -> None:
        self.version=version;self.style=style;self.edited=edited
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}

    def packed(self, count: int) -> bytes:
        return struct.pack('<HI',65535,count) if self.style in (3,4,5) else struct.pack('<H',count)

    def name(self) -> bytes:
        return shared_object(self.mapping,self.ids,'name','moNodeName_c',lambda:name_bytes(self.style))

    def listing(self, populated: bool=True) -> bytes:
        if not populated or self.style not in (2,5):return self.packed(0)
        return self.packed(3)+self.name()+bytes(2)+self.name()

    def body(self) -> bytes:
        word=12345 if self.edited else WORDS[self.style]
        raw=struct.pack('<2i',word,WORDS[(self.style+1)%6])+bytes(4)
        raw+=b''.join(self.listing(offset not in ('58','78')) for offset in ('10','60','100','80','68','58','78'))
        raw+=bytes(10)+self.listing()+struct.pack('<2i',WORDS[self.style],WORDS[(self.style+2)%6])
        presence=(0,0,1,65535,0,32768)[self.style];raw+=struct.pack('<H',presence)
        if presence:raw+=self.listing()
        raw+=bytes(2)
        if self.version>0x2b8a:raw+=shared_object(self.mapping,self.ids,'list-88','suObList',self.listing) if self.style in (2,5) else bytes(2)
        if self.version>0x4972:raw+=self.name() if self.style in (2,5) else bytes(2)
        return raw


def config_object_bytes(version: int, style: int, edited: bool=False) -> tuple[bytes,int]:
    """Keep the +88 and +A8 pointers absent at 11000; e.g. config_object_bytes(11000,0)."""
    graph=IntermediateConfigGraph(version,style,edited);raw=graph.body()
    return raw,graph.mapping.slot+1
