"""Complete filled surfaces and their direct array-bearing keeper grammar (#54)."""
import struct
from fillet_fixture_bytes import WireMap, body
from sweep_fixture_bytes import shared_object
from filled_surface_contacts import WORDS, contact, contact_data

CLASSES=('moFillRefSurface_c','moFillRefSurfIdKeeper_c','moCurveContact_c','moCurveContactData_c')
VERSIONS=(0x27cc,0x27df,0x27e0,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b01,0x3b02,19000,0xffffffff)
FLAGS=(0,1,127,128,254,255)


def keeper_body(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Serialize the pointed-to keeper directly without a tag or presence flag; e.g. keeper_body(mapping, ids, 19000, 4)."""
    count=(0,1,3,2,3,4)[style]
    prefix=struct.pack('<HI',65535,count) if style>=4 else struct.pack('<H',count)
    if style<2:return prefix+bytes(2)*count
    return prefix+b''.join(contact_data(mapping,ids,version,style,'keeper-data') if index%2==0 else bytes(2) for index in range(count))


def payload(mapping: WireMap, ids: dict[str,int], version: int, style: int, edited: bool) -> bytes:
    """Keep raw +74 unchanged and +10C exactly one byte; e.g. payload(mapping, ids, 19000, 4, False)."""
    raw=b''
    for column in range(3):
        count=0 if style==0 else (style+column)%4
        items=[]
        for index in range(count):
            item=contact(mapping,ids,version,style,'contact') if style>=2 and index%2==0 else bytes(2)
            items.append(item)
        prefix=struct.pack('<HI',65535,count) if style>=4 else struct.pack('<H',count)
        raw+=prefix+b''.join(items)
    raw+=struct.pack('<i',12345 if edited else WORDS[style])+keeper_body(mapping,ids,version,style)
    raw+=struct.pack('<3iB',-2147483648,2,2147483647,FLAGS[style])
    return raw+struct.pack('<5i',WORDS[style],-2147483648,2147483647,-1,2)


def filled_surface_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Preserve real object identities separately from direct keeper/data bodies; e.g. filled_surface_object(0, 19000, 5)."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str) -> bytes:
        change=edited and key=='first'
        if kind==1:return shared_object(mapping,ids,key,CLASSES[1],lambda:keeper_body(mapping,ids,version,style))
        if kind==2:return contact(mapping,ids,version,style,key,change)
        if kind==3:return contact_data(mapping,ids,version,style,key,change)
        def contents() -> bytes:
            owner=mapping.slot
            return body(mapping,version,style in (3,5),owner)+payload(mapping,ids,version,style,change)
        return shared_object(mapping,ids,key,CLASSES[0],contents)
    def outer() -> bytes:
        ids['outer']=mapping.slot
        first,second=record('first'),record('second')
        return struct.pack('<H',3)+first+second+record('first')
    raw=mapping.new('suObArray',outer) if style in (2,4,5) else record('first')
    return raw,mapping.slot+1
