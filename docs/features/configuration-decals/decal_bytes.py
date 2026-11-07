"""Complete decal property bodies and distinct nested owned-map framing (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from material_bytes import material_recipe_bytes
from scalar_bytes import name_bytes,WORDS,BITS
from decal_dependency_bytes import DecalDependencies,NESTED_CLASSES

VERSIONS=(0x27cc,0x332c,0x332d,18000,19000,0xffffffff)
PREFIX=(('i','48 4c'),('Q','68 70 78 80 88'),('i','98 9c'),('Q','b0 b8'),('i','a0 a4 a8'),('s','e0 e8'))
MIDDLE=(('i','f8'),('Q','100 108 110 118 120 128'),('I','130'),('Q','138 140 150 158 168 170 178 180 188 190 198 1a0'),('i','1a8'))
SUFFIX=(('Q','c0 c8'),('i','1ac 1b0 1b4 1b8 1bc 2ac'),('Q','1c8'))
OBJECT_SUFFIX=(('Q','70 78'),('i','80 84 88 8c 90 94'))


class DecalGraph:
    """Reserve complete decal and nested-object identities before recursive reads."""
    def __init__(self, kind: int, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.kind=kind;self.version=version;self.style=style;self.edited=edited
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}
        self.dependencies=DecalDependencies(self.mapping,self.ids,version,style)

    def packed(self, count: int) -> bytes:
        return struct.pack('<HI',65535,count) if self.style in (3,4,5) else struct.pack('<H',count)

    def indices(self) -> bytes:
        count=(0,-1,3,-2147483648,0,2)[self.style]
        values=(0xffffffff,0x80000000,0)
        return struct.pack('<i',count)+b''.join(struct.pack('<I',values[i%3]) for i in range(max(0,count)))

    def nested_list(self, key: str) -> bytes:
        if self.style in (0,1,4):return self.packed(0)
        pointer=self.dependencies.pointer(NESTED_CLASSES[self.kind],'nested-first')
        return self.packed(3)+pointer+struct.pack('<H',self.ids[key])+self.dependencies.pointer(NESTED_CLASSES[self.kind],'nested-first')

    def property_body(self, key: str, edited: bool) -> bytes:
        prefix=material_recipe_bytes(PREFIX,self.style)
        if edited:prefix=struct.pack('<i',12345)+prefix[4:]
        object_fields=struct.pack('<i',WORDS[self.style] if self.style else 0)+name_bytes(self.style)+struct.pack('<i',WORDS[(self.style+1)%6] if self.style else 0)
        middle=material_recipe_bytes(MIDDLE,self.style)
        vector=struct.pack('<3Q',*(BITS[(self.style+i)%6] if self.style else 0 for i in range(3)))
        lists=vector*2+struct.pack('<i',WORDS[self.style] if self.style else 0)+self.nested_list(key)+vector+self.nested_list(key)
        return prefix+object_fields+self.indices()+middle+lists+material_recipe_bytes(SUFFIX,self.style)+material_recipe_bytes(OBJECT_SUFFIX,self.style)

    def property(self, key: str='first', edited: bool=False) -> bytes:
        return shared_object(self.mapping,self.ids,key,'moDecalProperties_c',lambda:self.property_body(key,edited))

    def owned_map(self) -> bytes:
        count=(-1,1,3,1,-2147483648,3)[self.style];raw=struct.pack('<i',count)
        for index in range(max(0,count)):
            pointer=bytes(2) if self.style==1 else self.property('second' if index==1 else 'first',self.edited and index==0)
            raw+=pointer+struct.pack('<i',WORDS[(self.style+index)%6])
        associations=(-2147483648,1,2,2,-1,2)[self.style];raw+=struct.pack('<i',associations)
        for index in range(max(0,associations)):raw+=self.association(index)
        return raw

    def association(self, index: int) -> bytes:
        count=0 if self.style==1 else -1 if self.style==3 and index==0 else 3
        raw=struct.pack('<2i',-1,count)
        if count<=0:return raw
        return raw+self.property('first')+bytes(2)+self.property('first')


def decal_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Cross each complete scoped dependency and preserve independent edit goldens."""
    graph=DecalGraph(kind,version,style,edited,first_slot);raw=graph.property(edited=edited)
    return raw,graph.mapping.slot+1


def decal_map(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Retain signed counts, ordered duplicate keys and nested pointer arrays."""
    graph=DecalGraph(kind,version,style,edited,first_slot);raw=graph.owned_map()
    return raw,graph.mapping.slot+1
