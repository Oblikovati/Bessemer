"""Independent complete split parents, raw integer maps and empty tagged records (#54)."""
import struct
from fillet_fixture_bytes import WireMap, header, atom_fields, body_suffix
from sweep_fixture_bytes import shared_object
from surface_operation_bytes import array, chooser, profile
from body_feature_fixture_bytes import direction
from weld_corner_bytes import counted
from split_item_bytes import split_item

CLASSES=('moSplit_c','moSplitItem_c','moSplitData_c','moSplitBodyIdKeeper_c')
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2b91,0x2b92,0x2bcc,0x2bcd,0x2f85,0x2f86,0x332c,0x332d,19000,0xffffffff)
PAIRS=((),(),((3,10),(3,20),(-1,5),(4,-1),(0,0),(3,-1),(2147483647,-2147483648),(2147483647,2147483647)),(),((7,8),(7,9),(0,-2147483648),(-1,-1),(0,17)),((0,0),(5,-2147483648),(5,77),(5,88)))
COUNTS=(0,-1,8,-2147483648,5,4)


def split_base(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int) -> bytes:
    """Exercise the native-proven keeper through the inherited nonnull body role; e.g. split_base(mapping,ids,19000,3,4)."""
    rich=style in (3,5);raw=header(mapping,version,rich)
    if rich:
        if raw[-2:]!=bytes(2):raise ValueError(f'split header tail {raw[-2:]}; expected absent keeper word')
        raw=raw[:-2]+struct.pack('<H',65535)+shared_object(mapping,ids,'split-keeper',CLASSES[3],lambda:b'')
    return raw+atom_fields(mapping,rich,owner)+body_suffix(rich)


def prefix(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Retain parent/owned-data interleaving and direct chooser identity; e.g. prefix(mapping,ids,19000,2,4,False)."""
    raw=array(mapping,ids,version,style,(0,1,2,3,0,2)[style])+array(mapping,ids,version,style,(0,0,3,1,2,1)[style])
    raw+=profile(mapping,ids,version) if style else bytes(2)
    raw+=shared_object(mapping,ids,'direction','moDirectionSpec_c',lambda:direction(mapping,True)) if style else bytes(2)
    raw+=struct.pack('<I',0xffffffff if style else 0)+chooser(mapping,ids,version,style,owner,2 if style>=2 else 0)
    items=[split_item(mapping,ids,version,style,'nested-item'),bytes(2),split_item(mapping,ids,version,style,'nested-item')] if style>=2 else []
    raw+=counted(items,style in (3,5))+struct.pack('<i',12345 if edited else -2147483648)
    words=(0xffffffff,0,0x80000000) if style else ()
    return raw+counted([struct.pack('<I',word) for word in words],style in (2,4))+struct.pack('<i',2147483647)


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, owner: int, edited: bool) -> bytes:
    """Retain map duplicates, negative pairs and direct entity keeper without tags; e.g. payload(mapping,ids,19000,2,4,True)."""
    raw=prefix(mapping,ids,version,style,owner,edited)+array(mapping,ids,version,style,2 if style else 0)
    raw+=struct.pack('<i',COUNTS[style])+b''.join(struct.pack('<2i',*pair) for pair in PAIRS[style])
    raw+=struct.pack('<i',-1)+array(mapping,ids,version,style,3 if style>=2 else 0)
    return raw+struct.pack('<8i',-2147483648,2147483647,-1,0,1,-2147483648,2147483647,-1)


def split_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Keep tagged empty records distinct from direct parent-owned data; e.g. split_object(0,19000,5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def contents(key: str) -> bytes:
        if kind:return b''
        owner=mapping.slot;raw=split_base(mapping,ids,version,style,owner)
        return raw+payload(mapping,ids,version,style,owner,edited and key=='first')
    def record(key: str) -> bytes:
        if kind==1:return split_item(mapping,ids,version,style,key,edited and key=='first')
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:contents(key))
    def outer() -> bytes:
        return struct.pack('<H',3)+record('first')+record('second')+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,5) else record('first')
    return raw,mapping.slot+1
