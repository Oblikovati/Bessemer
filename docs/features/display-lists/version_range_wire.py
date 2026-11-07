"""Independent wire bodies for the material gates between display versions 18000 and 19000."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring


def floats(count: int) -> bytes:
    bits = [0,0x80000000,0x7f800000,0xff800000,0x7fc00123,0x7f800042,1,0x7f7fffff]
    return b"".join(struct.pack("<I",bits[index%len(bits)]) for index in range(count))


def symbol() -> bytes:
    raw = b"".join(cstring(value,True) for value in ["symbol 鋼","two\0","😀"])
    return raw+floats(17)+struct.pack("<II",0xffffffff,0xfedcba98)+integers(-1,2,-3,4)


def dspbr(version: int) -> bytes:
    raw = floats(26)+integers(-1)+floats(18)+integers(-2)+floats(2)
    raw += b"".join(cstring(value,True) for value in ["map 鋼","two","three\0","four","five 😀"])+floats(1)
    if version > 0x46b4:
        raw += cstring("extension 鋼",True)+cstring("b0")+integers(-3)+floats(6)+cstring("118")+cstring("120")
    if version > 0x4971:
        raw += floats(2)+b"".join(cstring(value,True) for value in ["140 鋼","148","150"])
    return raw
