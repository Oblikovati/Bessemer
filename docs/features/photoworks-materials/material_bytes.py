"""Independent complete PhotoWorks property and direct-manager wire layouts (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from scalar_bytes import name_bytes,WORDS,BITS

VERSIONS=(0x27cc,18000,19000,0xffffffff)
PREFIX=(('i','20'),('I','28 2c 30'),('Q','38'),('i','40 48 4c'),('Q','68 70 78 80 88 90'),('i','1c0 1c4'),('Q','50 58'),('i','60 98 9c'),('Q','b0 b8'),('i','a0 a4 a8'),('s','e0 e8'),('i','f8'),('Q','100 108 110 118 120 128'),('I','130'),('Q','138 140 150 158 168 170 178 180 188 190 198 1a0'),('i','1a8'))
SUFFIX=(('Q','c0 c8'),('i','1ac 220'),('Q','228 230 238 240'),('i','248'),('Q','250 258 260 268 270 278'),('s','280'),('i','d8 1b0 1b4 1b8 1bc 288 2ac'),('Q','2a0'),('i','2a8 298'),('Q','1c8'),('i','2c0'),('Q','2c8'),('i','2d0'),('s','290'),('i','2d4 2e0'),('s','2f8'),('Q','300 308'),('i','310 314'),('Q','318'))


def material_recipe_bytes(recipe: tuple[tuple[str,str],...], style: int, edited: bool=False) -> bytes:
    """Keep material and decal scalar encodings independent of their Rust readers."""
    raw=b''
    for kind,offsets in recipe:
        for index,offset in enumerate(offsets.split()):
            if kind=='s':raw+=name_bytes(style);continue
            value=0 if style==0 else (WORDS[(style+index)%6] if kind=='i' else BITS[(style+index)%6] if kind=='Q' else (0xffffffff,0x80000000,0)[index%3])
            if offset=='20' and kind=='i':value=12345 if edited else 0
            raw+=struct.pack('<'+kind,value)
    return raw


class MaterialGraph:
    """Reserve real property/unit slots before encoding recursive references."""
    def __init__(self, style: int, edited: bool, first_slot: int) -> None:
        self.style=style;self.edited=edited;self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}

    def count(self, count: int) -> bytes:
        return struct.pack('<HI',65535,count) if self.style in (3,4,5) else struct.pack('<H',count)

    def unit(self, key: str) -> bytes:
        return shared_object(self.mapping,self.ids,key,'moUnitComponent_c',lambda:bytes(3))

    def mapped_list(self, key: str) -> bytes:
        if self.style in (0,1,4):return self.count(0)
        if self.style==2:
            return self.count(3)+self.unit('unit-first')+self.unit('unit-second')+self.unit('unit-first')
        if self.style==3:return self.count(2)+bytes(2)+struct.pack('<H',self.ids[key])
        return self.count(3)+self.unit('unit-first')+struct.pack('<H',self.ids[key])+self.unit('unit-first')

    def property_body(self, key: str, edited: bool) -> bytes:
        raw=material_recipe_bytes(PREFIX,self.style,edited)
        vector=struct.pack('<3Q',*(BITS[(self.style+i)%6] if self.style else 0 for i in range(3)))
        return raw+vector*2+self.mapped_list(key)+vector+self.mapped_list(key)+material_recipe_bytes(SUFFIX,self.style)

    def property(self, key: str='first', edited: bool=False) -> bytes:
        return shared_object(self.mapping,self.ids,key,'moPWMaterialProperties_c',lambda:self.property_body(key,edited))

    def manager(self) -> bytes:
        raw=struct.pack('<2i',WORDS[self.style],WORDS[(self.style+1)%6])
        count=(0,1,3,1,0,3)[self.style];raw+=self.count(count)
        if count==0:return raw
        raw+=self.property('first',self.edited)
        if count==3:raw+=self.property('second')+self.property('first')
        return raw


def material_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep the zero-body oracle at 601 bytes and all property identities distinct."""
    graph=MaterialGraph(style,edited,first_slot);raw=graph.property(edited=edited)
    return raw,graph.mapping.slot+1


def manager_bytes(style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Serialize the direct manager without an outer MFC header; e.g. manager_bytes(2)."""
    graph=MaterialGraph(style,edited,first_slot);raw=graph.manager()
    return raw,graph.mapping.slot+1
