"""Independent table and EDW annotation bodies at their 0x333x string boundaries."""
import struct
from generate_item_vectors import record
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles


def annotation(version: int) -> bytes:
    raw = integers(-7,1,-8,1)+doubles(3)
    raw += integers(1)+cstring("edw one 鋼",True) if version>0x1c4b else b""
    raw += integers(-9) if version>0x1fa8 else b""
    raw += integers(1)+cstring("edw two\0",True) if version>0x1fd4 else b""
    raw += cstring("tail",True) if version>0x23fe else b""
    return raw+(cstring("four",True)+cstring("five😀",True) if version>0x332b else b"")


def items(version: int) -> bytes:
    table = integers(-1,2,-3)+cstring("table 鋼",True)+(integers(-4) if version>0x1c56 else b"")+(integers(5) if version>0x238c else b"")
    table += cstring("scaled views",True) if version>0x3332 else b""
    start = integers(-1,2)
    if 0x1c15<version<0x1c4a:start += integers(2,-17,42)
    if version>0x1c49:start += integers(2)+annotation(version)*2
    start += cstring("ID 鋼",True) if version>=0x35e9 else b""
    return struct.pack("<H",2)+record(23,table)+record(29,start)
