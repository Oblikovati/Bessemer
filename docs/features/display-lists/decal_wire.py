"""Independent decal primitive fields and existing reviewed material payloads."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from version_range_wire import floats
from old_shaded_wire import texture


def decal(version: int, material: bytes, empty: bool, edited: bool) -> bytes:
    raw = b"\xff"+floats(10)+cstring("mask 鋼😀\0",True)+b"\xfe"+integers(23 if edited else -17)
    raw += struct.pack("<I",0 if empty else 2)+(b"" if empty else struct.pack("<II",0xffffffff,0x12345678))
    raw += texture()+(integers(-9) if version>0x372a else b"")+material
    raw += integers(0 if empty else 2)
    for index in range(0 if empty else 2):
        raw += struct.pack("<I",0xffffffff if index==0 else 0)+floats(3)
        if version>0x12db:
            raw += floats(3)+integers(-11)
    raw += integers(-3) if version>0x1197 else b""
    raw += integers(-9) if version>0x11bb else b""
    return raw+(floats(2) if version>0x1bbe else b"")


def payload(version: int, material: bytes, scenario: int, mapped: bool, first: bool) -> bytes:
    if scenario==3:
        return struct.pack("<I",0)
    first_decal = decal(version,material,scenario==2,scenario==1 and first)
    collection = struct.pack("<I",2)+first_decal+decal(version,material,scenario==2,False)
    if not mapped:
        return collection
    return struct.pack("<Ii",2,-7)+collection+integers(-7)+struct.pack("<I",0)
