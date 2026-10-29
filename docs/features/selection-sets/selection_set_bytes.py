"""Complete selection sets and node-based selection items (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper
from owned_name_edit import short_wide_name
from comment_fixture_bytes import comment

VERSIONS=(0x27cc,0x3720,0x3721,0x3b0f,0x3b10,19000,0xffffffff)
CLASSES=('moSelectionSetData_c','moSelectionItemData_c')
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)


class SelectionGraph:
    """Allocate real node names and shared selection targets before following bodies."""
    def __init__(self, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.version,self.style,self.edited=version,style,edited
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}

    def obj(self, key: str, name: str, contents: Callable[[],bytes]) -> bytes:
        return shared_object(self.mapping,self.ids,key,name,contents)

    def node(self) -> bytes:
        name=self.obj('name','moNodeName_c',lambda:short_wide_name('Ω selection node'))
        return name+struct.pack('<iIiI',-3,0x80000001,WORDS[self.style],0xfffffffe)+short_wide_name('stored node tail')

    def selection_item(self) -> bytes:
        raw=self.node()
        if self.style in (0,4):return raw+bytes(2)
        return raw+self.obj('wrapper','moRefWrapper_c',lambda:wrapper(self.version,False))

    def selection_set(self) -> bytes:
        raw=self.node()+struct.pack('<i',12345 if self.edited else WORDS[self.style])+short_wide_name('Ω selection set')
        if self.version>0x3720:
            raw+=self.obj('comment','moNewFeatComment_c',lambda:comment(self.mapping,self.version,True,False,False)) if self.style not in (0,4) else bytes(2)
        if self.style in (0,4):return raw+bytes(2)
        item=self.obj('item',CLASSES[1],self.selection_item)
        return raw+struct.pack('<H',3)+item+struct.pack('<2H',self.ids['item'],self.ids['root'])

    def root(self, kind: int) -> bytes:
        return self.obj('root',CLASSES[kind],self.selection_set if kind==0 else self.selection_item)


def selection_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Build complete tagged selection records; e.g. selection_object(0,19000,2)."""
    graph=SelectionGraph(version,style,edited,first_slot)
    raw=graph.root(kind)
    return raw,graph.mapping.slot+1
