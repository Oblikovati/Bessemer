"""Complete sketch owners with native-bound intersection wrapper targets (#46)."""
import struct
from dataclasses import dataclass
from fillet_fixture_bytes import WireMap
from sketch_tail_fixture_bytes import ordinary, intersection
from sweep_fixture_bytes import shared_object
from end_wrapper_bytes import CLASSES, wrapper_pointer
from legacy_intersection_baselines import VERSIONS


@dataclass
class IntersectionWrapperGraph:
    mapping: WireMap
    identities: dict[str,int]
    version: int
    kind: int
    style: int
    edited: bool
    foreign_field: int | None

    def reference(self, key: str, position: int) -> bytes:
        """Keep nulls, exact concrete classes and one independent edit; e.g. graph.reference('first',0)."""
        if (self.style==3 and position%2==1) or (self.style==4 and position%2==0):return bytes(2)
        changed=self.edited and key in ('first','seed') and position==0
        return wrapper_pointer(self.mapping,self.identities,self.version,self.kind,f'{key}-target-{position}',changed,position==self.foreign_field)

    def references(self, key: str) -> bytes:
        """Reuse actual seeded objects at each of the four fields; e.g. graph.references('first')."""
        selected='seed' if self.style in (1,5) else key
        return b''.join(self.reference(selected,position) for position in range(4))

    def sketch(self, key: str) -> bytes:
        """Embed the complete intersection at its ordinary direct sketch collection; e.g. graph.sketch('first')."""
        def complete_intersection() -> bytes:
            return intersection(self.mapping,self.version,self.style in (3,5),wrapper_references=lambda:self.references(key))
        return shared_object(self.mapping,self.identities,key,'sgSketch',lambda:ordinary(
            self.mapping,self.version,self.style in (3,5),intersection_body=complete_intersection))

    def outer(self) -> bytes:
        """Keep seeded objects, distinct sketches and repeated first sketches in wire order; e.g. graph.outer()."""
        seeds=[self.reference('seed',position) for position in range(4)] if self.style in (1,5) else []
        sketches=[self.sketch('first')]
        if self.style in (2,5):sketches.extend((self.sketch('second'),self.sketch('first')))
        items=seeds+sketches
        return struct.pack('<H',len(items))+b''.join(items)


def intersection_wrapper_object(kind: int, version: int, style: int, edited: bool=False,
                                first_slot: int=0, foreign_field: int | None=None) -> tuple[bytes,int]:
    """Reserve sketch identities before their complete mapped children; e.g. intersection_wrapper_object(3,19000,2)."""
    mapping=WireMap();mapping.slot=first_slot
    graph=IntersectionWrapperGraph(mapping,{},version,kind,style,edited,foreign_field)
    raw=mapping.new('suObArray',graph.outer) if style in (1,2,5) else graph.sketch('first')
    return raw,mapping.slot+1
