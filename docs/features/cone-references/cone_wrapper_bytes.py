"""Reuse complete existing caller builders with exact cone pointers (#54)."""
import struct
from collections.abc import Callable
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper
from end_wrapper_bytes import EndWrapperGraph
from intersection_wrapper_bytes import IntersectionWrapperGraph
from owned_name_edit import short_wide_name


def cone_pointer(mapping: WireMap, identities: dict[str,int], version: int, key: str) -> bytes:
    """Reserve exact cone identity before its ordinary surface body; e.g. cone_pointer(mapping,ids,19000,'first')."""
    return shared_object(mapping,identities,key,'moConeRef_w',lambda:wrapper(version,False)+struct.pack('<12d',*range(12)))


WrapperPointer = Callable[[WireMap,dict[str,int],int,str],bytes]


class WrapperEndGraph(EndWrapperGraph):
    pointer: WrapperPointer
    def reference(self, key: str) -> bytes:
        """Keep complete end owners while replacing only their typed target; e.g. graph.reference('first')."""
        return self.pointer(self.mapping,self.identities,self.version,'cone-'+key)


class WrapperIntersectionGraph(IntersectionWrapperGraph):
    pointer: WrapperPointer
    def reference(self, key: str, position: int) -> bytes:
        """Retain original null and shared identity branches; e.g. graph.reference('seed',0)."""
        if (self.style==3 and position%2==1) or (self.style==4 and position%2==0):return bytes(2)
        return self.pointer(self.mapping,self.identities,self.version,f'{key}-cone-{position}')


def wrapper_caller_object(kind: int, version: int, style: int, first_slot: int, pointer: WrapperPointer) -> tuple[bytes,int]:
    """Reach every newly eligible generic wrapper caller; e.g. cone_wrapper_object(2,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot
    if kind<3:
        graph=WrapperEndGraph(mapping,{},version,0,kind,bool(style%2),False,False,False,first_slot!=0);graph.pointer=pointer
        raw=mapping.new('suObArray',graph.outer) if style%2 else graph.owner('first') if first_slot else graph.specification('first')
    elif kind==3:
        graph=WrapperIntersectionGraph(mapping,{},version,0,style,False,None);graph.pointer=pointer
        raw=mapping.new('suObArray',graph.outer) if style in (1,2,5) else graph.sketch('first')
    else:
        ids: dict[str,int]={}
        def entity() -> bytes:
            return mapping.new('sgExtEnt_c',lambda:pointer(mapping,ids,version,'cone')+bytes(6)+short_wide_name('Ω cone axis'))
        def outer() -> bytes:
            return struct.pack('<H',2)+pointer(mapping,ids,version,'cone')+entity()
        raw=mapping.new('suObArray',outer) if style%2 else entity()
    return raw,mapping.slot+1


def cone_wrapper_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep all existing cone caller bytes unchanged through the shared builder."""
    return wrapper_caller_object(kind,version,style,first_slot,cone_pointer)
