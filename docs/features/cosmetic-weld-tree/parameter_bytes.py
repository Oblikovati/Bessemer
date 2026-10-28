"""Fixed native parameter/endpoint records and signed wire-map lists (#54)."""
import struct

BITS=(0,0x8000000000000000,0x7ff0000000000042,0x7ff8000000001234,0x7ff0000000000000,0xfff0000000000000)
WORDS=(-2147483648,2147483647,-1,0,-32769,65535)
COUNTS=(-2147483648,-1,0,1,3,2)


def parameters(intermittent: bool, style: int, edited: bool=False) -> bytes:
    """Keep fixed scalar widths and payload bits; e.g. parameters(False,2)."""
    raw=struct.pack('<i',12345 if edited else WORDS[style])
    if intermittent:raw+=struct.pack('<i',WORDS[(style+1)%6])
    return raw+struct.pack('<2Q',BITS[style],BITS[(style+1)%6])


def endpoint(style: int) -> bytes:
    """Construct the complete direct thirty-two-byte endpoint; e.g. endpoint(2)."""
    if style==0:return bytes(32)
    return struct.pack('<2Q2iQ',BITS[style],BITS[(style+1)%6],WORDS[style],WORDS[(style+1)%6],BITS[(style+2)%6])


def wire_map(style: int, edited: bool=False) -> bytes:
    """Keep negative empty counts without synthesizing entries; e.g. wire_map(0)."""
    raw=struct.pack('<I',12345 if edited else 0xffffffff-style)
    for column in range(2):
        count=COUNTS[(style+column)%6]
        raw+=struct.pack('<i',count)
        raw+=b''.join(struct.pack('<i',WORDS[(style+index)%6]) for index in range(max(0,count)))
    return raw
