"""Version-11000 direct suffixes with no later-version padding or synthetic defaults (#54)."""
import struct
from scalar_bytes import WORDS,name_bytes


def tail_bytes(style: int) -> bytes:
    """Exercise ordered nullable maps, nonpositive counts and original u32 values."""
    counts=(-1,0,1,2,-2147483648,1);count=counts[style];raw=struct.pack('<i',count)
    raw+=b''.join(bytes(2)+struct.pack('<i',WORDS[style]) for _ in range(max(0,count)))
    raw+=struct.pack('<i',count)
    for _ in range(max(0,count)):raw+=struct.pack('<2i',-1,2)+bytes(4)
    raw+=struct.pack('<i',count)+b''.join(bytes(2)+struct.pack('<i',WORDS[style]) for _ in range(max(0,count)))
    raw+=struct.pack('<i',count)+b''.join(struct.pack('<i',-1)+bytes(2) for _ in range(max(0,count)))
    raw+=struct.pack('<i',count)+bytes(2*max(0,count))
    strings=2 if style in (2,5) else 0
    raw+=(struct.pack('<HI',65535,strings) if style>=3 else struct.pack('<H',strings))
    raw+=name_bytes(style)*strings
    return raw+struct.pack('<iI',WORDS[style],(0,2,0xffffffff,0x80000000,255,0xffffffff)[style])


def rejected_tail(name: str, conversion: bool) -> bytes:
    """Present a real documented fresh class where this exact factory has no reader."""
    encoded=name.encode('ascii');pointer=struct.pack('<3H',65535,1,len(encoded))+encoded
    return (bytes(16) if conversion else b'')+struct.pack('<i',1)+pointer+bytes(32)
