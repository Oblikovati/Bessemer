"""Complete independently assembled net-blend dependencies and their shared cycles (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from freeze_component_references import base, wire as component
from display_dimension_fixture_bytes import body
from freeze_model_references import wrapper

WORDS=(0,2147483647,-2147483648,-1,1,-2147483648)


def edge(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Share a complete component edge with both native option paths; e.g. edge(mapping, ids, 19000, 3)."""
    return shared_object(mapping,ids,'edge','moCompEdge_c',lambda:body(component('moCompEdge_c',version,False,8 if style in (3,5) else 0)))


def reference(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Exercise the typed wrapper boundary with a constructor-proven group; e.g. reference(mapping, ids, 19000, 2)."""
    return shared_object(mapping,ids,'wrapper','moRefWrapper_c',lambda:group(mapping,ids,version,style,'group',False)+wrapper(version,False)[2:])


def contact(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str='contact') -> bytes:
    """Retain the unqualified reference and full unsigned word; e.g. contact(mapping, ids, 19000, 2)."""
    target=lambda:reference(mapping,ids,version,style) if style else bytes(2)
    return shared_object(mapping,ids,key,'moContactData_c',lambda:target()+struct.pack('<I',(0,0xffffffff,0x89abcdef,0x80000000,1,0xfffffffe)[style]))


def group(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool) -> bytes:
    """Keep the two direct lists out of the map and preserve cached contact cycles; e.g. group(mapping, ids, 19000, 2, 'group', False)."""
    def contents() -> bytes:
        prefix=base(version,False)+struct.pack('<i',12345 if edited else WORDS[style])
        if not style:return prefix+bytes(4)
        first=contact(mapping,ids,version,style)
        second=multi_edge(mapping,ids,version,style,'multi',False)
        return prefix+struct.pack('<H',3)+first+bytes(2)+contact(mapping,ids,version,style)+struct.pack('<H',2)+second+contact(mapping,ids,version,style)
    return shared_object(mapping,ids,key,'moCompEntGroup_c',contents)


def multi_edge(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool) -> bytes:
    """Use a full u32 edge count, complete edge targets and direct owned implementation; e.g. multi_edge(mapping, ids, 19000, 2, 'multi', False)."""
    def contents() -> bytes:
        count=(0,1,3,2,3,2)[style]
        targets=b''.join(edge(mapping,ids,version,style) for _ in range(count))
        references=contact(mapping,ids,version,style)+group(mapping,ids,version,style,'group',False) if style else bytes(4)
        words=struct.pack('<ii',12345 if edited else WORDS[style],WORDS[5-style])
        numeric=struct.pack('<8QI',0x8000000000000000,0x7ff800000000002a,0,1,0x7ff0000000000000,0xfff0000000000000,0x7ff000000000002a,0x8000000000000000,0xffffffff)
        return base(version,False)+struct.pack('<I',count)+targets+references+words+numeric
    return shared_object(mapping,ids,key,'moCompMultiEdge_c',contents)
