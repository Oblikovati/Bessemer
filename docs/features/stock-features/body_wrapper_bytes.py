"""Exercise body-reference ancestry through every existing generic-wrapper caller (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_model_references import wrapper
from end_wrapper_bytes import EndWrapperGraph
from intersection_wrapper_bytes import IntersectionWrapperGraph
from owned_name_edit import short_wide_name


def body_pointer(mapping: WireMap, identities: dict[str,int], version: int, key: str) -> bytes:
    """Use the actual body-reference grammar with zero control; e.g. body_pointer(w,ids,19000,'first')."""
    return shared_object(mapping,identities,key,'moBodyRef_w',lambda:wrapper(version,False)+struct.pack('<i',0))


class BodyEndGraph(EndWrapperGraph):
    def reference(self, key: str) -> bytes:
        """Keep complete end owners and typed body-reference targets; e.g. graph.reference('first')."""
        return body_pointer(self.mapping,self.identities,self.version,'body-'+key)


class BodyIntersectionGraph(IntersectionWrapperGraph):
    def reference(self, key: str, position: int) -> bytes:
        """Retain the caller's null and shared branches; e.g. graph.reference('seed',0)."""
        if (self.style==3 and position%2==1) or (self.style==4 and position%2==0):return bytes(2)
        return body_pointer(self.mapping,self.identities,self.version,f'{key}-body-{position}')


def sketch_entity(mapping: WireMap, ids: dict[str,int], version: int) -> bytes:
    """Keep the sketch's four separate reference fields; e.g. sketch_entity(w,ids,19000)."""
    return mapping.new('sgExtEnt_c',lambda:body_pointer(mapping,ids,version,'body')+bytes(6)+short_wide_name('Ω body axis'))


def body_wrapper_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reach every affected complete caller without modifying its bytes; e.g. body_wrapper_object(2,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot
    if kind<3:
        graph=BodyEndGraph(mapping,{},version,0,kind,bool(style%2),False,False,False,first_slot!=0)
        raw=mapping.new('suObArray',graph.outer) if style%2 else graph.owner('first') if first_slot else graph.specification('first')
    elif kind==3:
        graph=BodyIntersectionGraph(mapping,{},version,0,style,False,None)
        raw=mapping.new('suObArray',graph.outer) if style in (1,2,5) else graph.sketch('first')
    else:
        ids: dict[str,int]={}
        def outer() -> bytes:
            return struct.pack('<H',2)+body_pointer(mapping,ids,version,'body')+sketch_entity(mapping,ids,version)
        raw=mapping.new('suObArray',outer) if style%2 else sketch_entity(mapping,ids,version)
    return raw,mapping.slot+1
