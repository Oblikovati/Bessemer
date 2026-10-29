"""Complete scalar bodies with signed extrema, exact IEEE bits and CString encodings (#54)."""
import struct
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object
from owned_name_edit import short_wide_name

CLASSES=('moSMAndFPFeatIdPair','moCustomDefinedMaterialProperty_c')
VERSIONS=(0x27cc,0x2bcc,0x2bcd,18000,19000,0xffffffff)
WORDS=(-2147483648,2147483647,-1,0,1,-32769)
BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)


def name_bytes(style: int) -> bytes:
    """Cover empty, embedded-NUL, Unicode and both extended-length CString encodings."""
    if style==0:return b'\0'
    if style==1:return b'\x05Steel'
    if style==2:return short_wide_name('Ω\0 alloy 🜁')
    if style==3:return b'\xff'+struct.pack('<H',255)+bytes(range(255))
    if style==4:return b'\xff\xfe\xff\xff'+struct.pack('<H',300)+('Ω'*300).encode('utf-16-le')
    return b'\xff\xff\xff'+struct.pack('<I',7)+b'A\0A\xffxyz'


def scalar_map(kind: int, version: int, style: int, edited: bool, first_slot: int) -> tuple[bytes,int]:
    """Compose actual fixed-u32 manager maps with raw keys and shared scalar values."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str, changed: bool) -> bytes:
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:scalar_body(kind,style,changed))
    raw=struct.pack('<Ii',3,-1)+record('first',edited)
    raw+=struct.pack('<i',7)+record('second',False)+struct.pack('<i',-1)+record('first',False)
    return raw,mapping.slot+1


def scalar_body(kind: int, style: int, edited: bool) -> bytes:
    """Use native word order rather than floating-point arithmetic or value normalization."""
    word=12345 if edited else WORDS[style]
    if kind==0:return struct.pack('<3i',word,WORDS[(style+1)%6],WORDS[(style+2)%6])
    return name_bytes(style)+struct.pack('<2QiQ',BITS[style],BITS[(style+1)%6],word,BITS[(style+2)%6])


def scalar_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reserve actual identities before repeated roots and duplicate-but-distinct values."""
    mapping=WireMap();mapping.slot=first_slot;ids: dict[str,int]={}
    def record(key: str, changed: bool) -> bytes:
        return shared_object(mapping,ids,key,CLASSES[kind],lambda:scalar_body(kind,style,changed))
    def roots() -> bytes:
        return struct.pack('<H',3)+record('first',edited)+record('second',False)+record('first',False)
    raw=mapping.new('suObArray',roots) if style in (2,5) else record('first',edited)
    return raw,mapping.slot+1
