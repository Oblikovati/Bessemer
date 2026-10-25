"""Early configuration bodies and the transient annotation visibility prefix."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring


def configurations(version: int, kind: str, edited: bool = False) -> bytes:
    raw = integers(1)+cstring("active 鋼",True)+integers(-1)+cstring("cfg\0",True)+integers(-2)
    if kind=="part":
        raw += struct.pack("<H",0)+integers(0,0,0,0,-168,23 if edited else -1) if version<0x2b67 else bytes(2)
    else:
        raw += bytes(4)
    minimum,visibility_minimum = (0x2b61,0x2b7f) if kind=="part" else (0x2b6f,0x2b77)
    if version<=minimum:
        return raw
    raw += integers(-3)+bytes(2)+integers(-4)+bytes(2)
    if version>visibility_minimum:
        raw += integers(-17) if version<0x2b7a else b""
        raw += integers(1)+cstring("view 鋼",True)+cstring("cfg\0",True)
    return raw


def configuration_entries(version: int, kind: str) -> int:
    minimum,visibility_minimum = (0x2b61,0x2b7f) if kind=="part" else (0x2b6f,0x2b77)
    return 2+4*(version>minimum)+(version>visibility_minimum)
