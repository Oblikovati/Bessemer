"""Complete end-specification callers using the reviewed finite wrapper family (#46)."""
import struct
from collections.abc import Callable
from dataclasses import dataclass
from fillet_fixture_bytes import WireMap
from body_feature_fixture_bytes import end_spec, revolution_end
from sketch_wrapper_bytes import CLASSES as ORIGINAL_WRAPPERS, target
from projected_line_targets import surface_pointer
from sweep_fixture_bytes import shared_object
from owned_name_edit import short_wide_name
from legacy_builder_baselines import VERSIONS

CLASSES=ORIGINAL_WRAPPERS+('moSurfaceRef_w',)
OWNERS=('moEndSpec_c','moFromEndSpec_c','moRevEndSpec_c')


def wrapper_pointer(mapping: WireMap, ids: dict[str,int], version: int, kind: int, key: str, edited: bool, foreign: bool=False) -> bytes:
    """Retain complete exact class bodies and their own saved identities; e.g. wrapper_pointer(mapping,ids,19000,13,'first',False)."""
    if foreign:return shared_object(mapping,ids,key,'moNodeName_c',lambda:short_wide_name('complete unrelated object'))
    if kind==18:return surface_pointer(mapping,ids,version,key,edited)
    return target(mapping,ids,version,kind,2,key,edited)


@dataclass
class EndWrapperGraph:
    mapping: WireMap
    identities: dict[str,int]
    version: int
    kind: int
    caller: int
    cached: bool
    edited: bool
    null: bool
    foreign: bool
    owned: bool

    def reference(self, key: str) -> bytes:
        """Reserve targets before nested bodies; e.g. graph.reference('first')."""
        if self.null:return bytes(2)
        return wrapper_pointer(self.mapping,self.identities,self.version,self.kind,'wrapper-'+key,self.edited and key=='first',self.foreign)

    def contents(self, key: str) -> bytes:
        """Write the complete concrete specification suffix; e.g. graph.contents('first')."""
        callback=lambda:self.reference(key)
        if self.caller==0:return end_spec(self.mapping,self.version,False,7,7,wrapper_reference=callback)
        if self.caller==1:return struct.pack('<i',2 if self.cached else 1)+callback()
        return revolution_end(self.mapping,self.version,False,wrapper_reference=callback)

    def specification(self, key: str) -> bytes:
        """Keep repeated specification identity distinct from the reference identity; e.g. graph.specification('first')."""
        return shared_object(self.mapping,self.identities,'spec-'+key,OWNERS[self.caller],lambda:self.contents(key))

    def owner(self, key: str) -> bytes:
        """Reach end/revolution specifications through complete shipped parents; e.g. graph.owner('first')."""
        if self.caller==1 and not self.owned:return self.specification(key)
        name='moRevolution_c' if self.caller==2 else 'moBoss_c'
        return shared_object(self.mapping,self.identities,'owner-'+key,name,
            lambda:feature_contents(self.mapping,self.version,self.caller,lambda:self.specification(key)))

    def outer(self) -> bytes:
        """Retain seeded cached references, distinct parents and repeated first parent; e.g. graph.outer()."""
        first=self.reference('first')+self.owner('first')
        second=self.reference('second')+self.owner('second')
        key='spec-first' if self.caller==1 and not self.owned else 'owner-first'
        return struct.pack('<H',5)+first+second+struct.pack('<H',self.identities[key])


def end_wrapper_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0, null: bool=False, foreign: bool=False) -> tuple[bytes,int]:
    """Use complete callers and actual cached targets; e.g. end_wrapper_object(13,19000,1)."""
    mapping=WireMap();mapping.slot=first_slot
    graph=EndWrapperGraph(mapping,{},version,kind,style//2,bool(style%2),edited,null,foreign,first_slot!=0)
    raw=mapping.new('suObArray',graph.outer) if style%2 else graph.owner('first') if first_slot else graph.specification('first')
    return raw,mapping.slot+1


def feature_contents(mapping: WireMap, version: int, caller: int, specification: Callable[[],bytes]) -> bytes:
    """Reuse complete existing body/suffix builders without another class frame; e.g. feature_contents(mapping,19000,0,read_spec)."""
    from body_feature_fixture_bytes import header, atom_fields, body_suffix, suffix
    owner=mapping.slot;raw=header(mapping,version,False)+atom_fields(mapping,False,owner)+body_suffix(False)
    if caller==1:
        original=specification
        specification=lambda:mapping.new('moEndSpec_c',lambda:end_spec(mapping,version,False,0,0,from_reference=original))
    return raw+suffix(mapping,version,'moRevolution_c' if caller==2 else 'moBoss_c',False,specification)
