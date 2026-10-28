"""Reuse complete existing caller builders with exact cone pointers (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper
from end_wrapper_bytes import EndWrapperGraph
from intersection_wrapper_bytes import IntersectionWrapperGraph
from owned_name_edit import short_wide_name


def cone_pointer(mapping: WireMap, identities: dict[str,int], version: int, key: str) -> bytes:
    """Reserve exact cone identity before its ordinary surface body; e.g. cone_pointer(mapping,ids,19000,'first')."""
    return shared_object(mapping,identities,key,'moConeRef_w',lambda:wrapper(version,False)+struct.pack('<12d',*range(12)))


class ConeEndGraph(EndWrapperGraph):
    def reference(self, key: str) -> bytes:
        """Keep complete end owners while replacing only their typed target; e.g. graph.reference('first')."""
        return cone_pointer(self.mapping,self.identities,self.version,'cone-'+key)


class ConeIntersectionGraph(IntersectionWrapperGraph):
    def reference(self, key: str, position: int) -> bytes:
        """Retain original null and shared identity branches; e.g. graph.reference('seed',0)."""
        if (self.style==3 and position%2==1) or (self.style==4 and position%2==0):return bytes(2)
        return cone_pointer(self.mapping,self.identities,self.version,f'{key}-cone-{position}')


def cone_wrapper_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reach every newly eligible generic wrapper caller; e.g. cone_wrapper_object(2,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot
    if kind<3:
        graph=ConeEndGraph(mapping,{},version,0,kind,bool(style%2),False,False,False,first_slot!=0)
        raw=mapping.new('suObArray',graph.outer) if style%2 else graph.owner('first') if first_slot else graph.specification('first')
    elif kind==3:
        graph=ConeIntersectionGraph(mapping,{},version,0,style,False,None)
        raw=mapping.new('suObArray',graph.outer) if style in (1,2,5) else graph.sketch('first')
    else:
        ids: dict[str,int]={}
        def entity() -> bytes:
            return mapping.new('sgExtEnt_c',lambda:cone_pointer(mapping,ids,version,'cone')+bytes(6)+short_wide_name('Ω cone axis'))
        def outer() -> bytes:
            return struct.pack('<H',2)+cone_pointer(mapping,ids,version,'cone')+entity()
        raw=mapping.new('suObArray',outer) if style%2 else entity()
    return raw,mapping.slot+1
