"""Independent direct configuration visual-map compositions (#54)."""
import struct
from split_item_bytes import visual_body

VERSIONS=(0x27cc,0x3368,0x3369,0x372a,0x372b,0x3f08,0x3f09,18000,19000)
COUNTS=(-1,0,1,2,-2147483648,3)


def visual_map_bytes(version: int, style: int, edited: bool=False) -> bytes:
    """Preserve signed count/key extremes and duplicate-key order; e.g. visual_map_bytes(19000,5)."""
    count=COUNTS[style];raw=struct.pack('<i',count)
    for index in range(max(0,count)):
        key=12345 if edited and index==0 else (-2147483648 if style==3 else 2147483647)
        body=visual_body(version)
        colors=struct.pack('<3I',0xffffffff,index,0x80000000)
        raw+=struct.pack('<i',key)+colors+body[12:]
    return raw
