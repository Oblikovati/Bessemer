"""Saved external unit components with complete fresh and cached file records (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_external_references import wire as external_wire
from display_dimension_fixture_bytes import body as strip_tag

PRESENCE=(0,0,1,255,128,254)

class UnitGraph:
    """Reserve actual archive identities without inventing a model body."""
    def __init__(self, version: int, style: int, edited: bool, first_slot: int) -> None:
        self.version=version;self.style=style;self.edited=edited;self.mapping=WireMap();self.mapping.slot=first_slot;self.ids: dict[str,int]={}

    def external(self, key: str='external') -> bytes:
        def contents() -> bytes:
            raw=strip_tag(external_wire(self.version,0))
            if self.edited and key=='external':
                suffix=16 if self.version>0x3f12 else 12
                raw=raw[:-suffix]+struct.pack('<i',12345)+raw[-suffix+4:]
            return raw
        return shared_object(self.mapping,self.ids,key,'moExtObject_c',contents)

    def unit(self) -> bytes:
        def contents() -> bytes:
            owner=bytes(2) if self.style in (0,3) else self.external()
            raw=owner+bytes([PRESENCE[self.style]])
            return raw+(self.external('second' if self.style==4 else 'external') if PRESENCE[self.style] else b'')
        return shared_object(self.mapping,self.ids,'unit','moUnitComponent_c',contents)

    def graph(self) -> bytes:
        if self.style!=5:return self.unit()
        def entries() -> bytes:
            return struct.pack('<H',4)+self.external()+self.unit()+self.unit()+self.external()
        return self.mapping.new('suObArray',entries)


def unit_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep the external-model pointer tagged and shared; e.g. unit_object(0,19000,2)."""
    graph=UnitGraph(version,style,edited,first_slot);raw=graph.graph()
    return raw,graph.mapping.slot+1
