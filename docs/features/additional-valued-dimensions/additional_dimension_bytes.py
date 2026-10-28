"""Complete native-derived valued dimensions using their exact shared base bodies (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from sketch_dimension_fixture_bytes import valued_body, base
from sketch_tail_fixture_bytes import ordinary
from legacy_dimension_baselines import VERSIONS

CLASSES=('sgSplineTanMagnitudeDim','sgArcLenDist','sg3DPlane3DPlaneDist')
BASES=('sgValG1Dim','sgValG3Dim','sgValG2Dim')
GROUPS=(0,2,1)


def edit_common_word(raw: bytes, version: int) -> bytes:
    """Change only the independently observed dcm_type word; e.g. edit_common_word(raw,19000)."""
    offset=len(base(version))-10
    if raw[offset:offset+4]!=struct.pack('<i',-1):
        raise ValueError(f'dimension word {raw[offset:offset+4].hex()} at {offset}; expected common i32 -1')
    return raw[:offset]+struct.pack('<i',12345)+raw[offset+4:]


def dimension(mapping: WireMap, ids: dict[str,int], kind: int, version: int, rich: bool, key: str, edited: bool) -> bytes:
    """Reserve the concrete class before its shared entity handles; e.g. dimension(mapping,ids,1,19000,True,'first',False)."""
    def contents() -> bytes:
        raw=valued_body(BASES[kind],version,rich,mapping.slot)
        if kind==1:raw=raw[:-2]+struct.pack('<H',65535 if rich else 0)
        # The existing body emits one new entity handle and, for G2, one new plane handle.
        if rich:mapping.slot+=4 if kind==2 else 2
        return edit_common_word(raw,version) if edited and key=='first' else raw
    return shared_object(mapping,ids,key,CLASSES[kind],contents)


def sketch(mapping: WireMap, ids: dict[str,int], kind: int, version: int, rich: bool, key: str, edited: bool, seeded: bool=False) -> bytes:
    """Replace only the existing four empty valued-collection counts; e.g. sketch(mapping,ids,0,19000,True,'sketch',False)."""
    def contents() -> bytes:
        before=mapping.slot;empty=ordinary(mapping,version,False)
        if mapping.slot!=before or empty[52:60]!=bytes(8):
            raise ValueError(f'empty sketch map/counts {(mapping.slot-before,empty[52:60].hex())}; expected no allocations and four zero u16 counts')
        first=dimension(mapping,ids,kind,version,rich,'first',edited)
        second=first if seeded else dimension(mapping,ids,kind,version,rich,'second',False)
        if seeded:first=struct.pack('<H',ids['first']);second=first
        items=first+second+struct.pack('<H',ids['first'])
        counts=b''.join(struct.pack('<H',3)+items if group==GROUPS[kind] else bytes(2) for group in range(4))
        return empty[:52]+counts+empty[60:]
    return shared_object(mapping,ids,key,'sgSketch',contents)


def additional_dimension_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep standalone, shared and complete collection callers in one real map; e.g. additional_dimension_object(2,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={};rich=style in (1,2,4,5)
    def record(key: str) -> bytes:
        return dimension(mapping,ids,kind,version,rich,key,edited)
    def array() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    def seeded_array() -> bytes:
        seed=record('first');owner=sketch(mapping,ids,kind,version,rich,'sketch',False,True)
        return struct.pack('<H',3)+seed+owner+struct.pack('<H',ids['sketch'])
    if style==2:raw=mapping.new('suObArray',array)
    elif style==5:raw=mapping.new('suObArray',seeded_array)
    elif style in (3,4):raw=sketch(mapping,ids,kind,version,rich,'sketch',edited)
    else:raw=record('first')
    return raw,mapping.slot+1
