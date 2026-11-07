"""Complete curve-contact dependencies with exact tagged-pointer order (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object, dimension, synchronization
from sketch_wrapper_bytes import target

WORDS=(-2147483648,-1,0,1,2,2147483647)
BITS=(0x8000000000000000,0x7ff800000000002a,0x7ff0000000000000,0xfff0000000000000,1,0x7ff000000000002b)


def contact_data(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool=False) -> bytes:
    """Retain distinct wrappers, arbitrary references and full unsigned trailing bits; e.g. contact_data(mapping, ids, 19000, 5, 'first')."""
    def contents() -> bytes:
        wrapped=target(mapping,ids,version,3 if style in (2,4) else 0,0,'wrapper-'+key,edited) if style>=2 else bytes(2)
        if style==5:arbitrary=struct.pack('<H',ids['outer'])
        elif style>=2:arbitrary=target(mapping,ids,version,3 if style in (2,4) else 0,0,'wrapper-'+key,False)
        else:arbitrary=bytes(2)
        return wrapped+arbitrary+struct.pack('<I',(0,0xffffffff,0x89abcdef,0x80000000,1,0xfffffffe)[style])
    return shared_object(mapping,ids,key,'moCurveContactData_c',contents)


def contact_array(mapping: WireMap, ids: dict[str,int], version: int, style: int) -> bytes:
    """Use a direct array with repeated complete data pointers; e.g. contact_array(mapping, ids, 19000, 4)."""
    if style==0:return bytes(2)
    if style==1:return struct.pack('<H',2)+bytes(4)
    first=contact_data(mapping,ids,version,style,'contact-data')
    prefix=struct.pack('<HI',65535,3) if style>=4 else struct.pack('<H',3)
    return prefix+first+bytes(2)+contact_data(mapping,ids,version,style,'contact-data')


def contact(mapping: WireMap, ids: dict[str,int], version: int, style: int, key: str, edited: bool=False) -> bytes:
    """Keep angular/synchronization pointers and all raw contact words complete; e.g. contact(mapping, ids, 19000, 3, 'first')."""
    def contents() -> bytes:
        raw=contact_array(mapping,ids,version,style)+struct.pack('<Qi',BITS[style],12345 if edited else WORDS[style])
        raw+=dimension(mapping,version,ids,'moDisplayAngularDim_c') if style>=2 else bytes(2)
        raw+=struct.pack('<Q4i',BITS[(style+1)%6],WORDS[style],WORDS[(style+1)%6],-2147483648,2147483647)
        raw+=synchronization(mapping,version,ids,style>=3) if style>=2 else bytes(2)
        return raw+struct.pack('<3iQ',-2147483648,-1,2147483647,BITS[(style+2)%6])
    return shared_object(mapping,ids,key,'moCurveContact_c',contents)
