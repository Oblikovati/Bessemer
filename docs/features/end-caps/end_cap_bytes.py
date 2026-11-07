"""Complete end-cap bodies with native pointer order and shared dimensions (#54)."""
import struct
from fillet_fixture_bytes import WireMap,body
from sweep_fixture_bytes import shared_object,dimension
from weld_corner_bytes import counted

VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3371,0x3372,0x3373,0x3374,0x371d,0x371e,0x3747,0x3748,0x3b01,0x3b02,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)


class EndCapFixture:
    """Allocate each real dependency before emitting its first pointer."""
    def __init__(self, version: int, style: int, first_slot: int) -> None:
        self.version=version;self.style=style
        self.mapping=WireMap();self.mapping.slot=first_slot
        self.ids: dict[str,int]={}

    def display(self) -> bytes:
        """Share one complete distance dimension across all four occurrences; e.g. fixture.display()."""
        if self.style in (0,4):return bytes(2)
        return dimension(self.mapping,self.version,self.ids,'moDisplayDistanceDim_c')

    def list_contents(self, owner: int) -> bytes:
        """Build only serialized entries, preserving owner and dimension identities; e.g. fixture.list_contents(2)."""
        entries=[]
        for index in range((self.style+1)%5):
            if index==0:entry=shared_object(self.mapping,self.ids,'marker','moPMarkRecord_c',lambda:struct.pack('<i',-2147483648))
            elif index==1:entry=bytes(2)
            elif index==2:entry=struct.pack('<H',owner)
            else:entry=self.display()
            entries.append(entry)
        return counted(entries,self.style==5)

    def payload(self, kind: int, owner: int, edited: bool) -> bytes:
        """Keep +840 before +838 and the unsupported arc-length body null; e.g. fixture.payload(1,2,False)."""
        raw=bytes(2) if self.style in (0,4) else shared_object(self.mapping,self.ids,'list','suObList',lambda:self.list_contents(owner))
        words=[WORDS[(self.style+kind+i)%6] for i in range(5)]
        raw+=struct.pack('<4i',12345 if edited else words[0],*words[1:4])
        raw+=b''.join(self.display() for _ in range(4))
        return raw+struct.pack('<Qi',BITS[(self.style+kind)%6],words[4])+bytes(2)


def end_cap_graph(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Retain full inherited bodies and true repeated-root identity; e.g. end_cap_graph(1,19000,5)."""
    fixture=EndCapFixture(version,style,first_slot)
    def record(key: str) -> bytes:
        def contents() -> bytes:
            owner=fixture.mapping.slot
            return body(fixture.mapping,version,style in (3,5),owner)+fixture.payload(kind,owner,edited and key=='first')
        return shared_object(fixture.mapping,fixture.ids,key,'moEndCap_c',contents)
    def array() -> bytes:
        entries=([fixture.display()] if style in (3,5) else [])+[record('first')]
        if style!=3:entries+=[record('second'),record('first')]
        return struct.pack('<H',len(entries))+b''.join(entries)
    raw=fixture.mapping.new('suObArray',array) if style in (2,3,5) else record('first')
    return raw,fixture.mapping.slot+1
