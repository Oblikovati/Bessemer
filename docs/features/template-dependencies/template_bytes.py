"""Compose complete template flat patterns and gusset IDs through established bodies (#54)."""
import struct
from fillet_fixture_bytes import WireMap,body
from sweep_fixture_bytes import shared_object
from flange_fixture_bytes import FlangeFixtureOptions,group_wire
from flat_fixture_bytes import FlatFixtureOptions,flat_prefix,flat_tail

CLASSES=('moTemplateFlatPattern_c','moSMGussetEndCornerFilletSurfIdRep_c','moSMGussetLateralCornerFilletSurfIdRep_c')
VERSIONS=(0x27cc,0x2baf,0x2bc4,0x2bc5,0x3364,0x3365,0x372b,0x372c,0x3761,0x3762,0x46b9,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)


def options(kind: int, style: int, edited: bool=False) -> FlatFixtureOptions:
    """Choose saved helper state independently from rich references and raw kernel presence."""
    return FlatFixtureOptions(extended=kind==1,rich=style in (1,3,5),present=style in (2,5),edited=edited)


class TemplateDependencyGraph:
    """Own one native-order MFC map for complete roots and shared references."""
    def __init__(self, kind: int, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}
        self.kind,self.version,self.style,self.edited=kind,version,style,edited

    def flat_body(self) -> bytes:
        selected=options(self.kind,self.style,self.edited);owner=self.ids['root']
        raw=body(self.mapping,self.version,selected.rich,owner)
        raw+=group_wire(self.mapping,self.version,self.ids,FlangeFixtureOptions(extended=selected.extended,rich=selected.rich))
        return raw+flat_prefix(self.mapping,self.version,self.ids,selected)+flat_tail(self.mapping,self.version,self.ids,selected)

    def representation(self) -> bytes:
        owner=self.ids['root'] if self.style not in (0,4) else 0
        return struct.pack('<2H3i',owner,0,12345 if self.edited else WORDS[self.style],WORDS[(self.style+1)%6],WORDS[(self.style+2)%6])

    def root(self) -> bytes:
        name=CLASSES[0] if self.kind<2 else CLASSES[self.kind-1]
        return shared_object(self.mapping,self.ids,'root',name,self.flat_body if self.kind<2 else self.representation)


def template_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep complete inherited flat bodies and direct three-word representation suffixes."""
    graph=TemplateDependencyGraph(kind,version,style,edited,first_slot)
    raw=graph.root()
    return raw,graph.mapping.slot+1
