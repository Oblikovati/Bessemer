"""Independent separate-flag polygon and edge records at the 0x333x scalar-width gates."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles


def flag(version: int, wide: int, narrow: int) -> bytes:
    return bytes([narrow]) if version>0x3350 else integers(wide)


def arc_payload(version: int) -> bytes:
    raw = doubles(9)+(doubles(3,3) if version<=0x3350 else b"")
    raw += flag(version,-17,255) if version>0x2fa9 else b""
    raw += bytes([2]) if version>=0x3724 else b""
    raw += integers(-11) if version>0x3336 else b""
    raw += flag(version,-3,3) if version>0x3344 else b""
    return raw+(integers(-12) if version>=0x3364 else b"")


def polygon_factory(original: object, version: int) -> object:
    def factory(polygon: object) -> object:
        encode_legacy = original(polygon)
        def encode(count: int, dimensions: int, kind: int = 1, arc: bool = True, line: bool = True) -> bytes:
            raw = encode_legacy(count,dimensions,kind,arc,line)
            if count<=0:
                return raw
            before = raw[:-(32+4+(83 if arc else 0)+4)]
            face_node = integers(-21) if version<0x3331 else b""
            silhouette = integers(-2)+(integers(-17,42) if version<0x333c else b"")
            return before+face_node+integers(-3 if arc else 0)+(arc_payload(version) if arc else b"")+silhouette+identifiers(raw[-32:],version)
        return encode
    return factory


def identifiers(raw: bytes, version: int) -> bytes:
    result = raw[:8] if version>0x3330 else b""
    result += raw[8:12] if version>0x3333 else b""
    return result+(raw[12:] if version>0x3350 else b"")


def edge(version: int) -> bytes:
    raw = doubles(6)+integers(-1)+(integers(2) if version<0x3331 else b"")+cstring("component 鋼",True)
    raw += struct.pack("<I",0xffffffff) if version>0x3330 else b""
    raw += struct.pack("<I",0xfffffffe) if version>0x3333 else b""
    raw += integers(-2)+(integers(-21) if version<0x3331 else b"")+integers(3,-4,5)+doubles(3)
    raw += integers(-73,74) if 0x2f7a<version<0x333c else b""
    raw += flag(version,-17,2)+flag(version,-255,255) if version>0x2fac else b""
    raw += struct.pack("<I",0xfffffffd) if version>0x3330 else b""
    raw += integers(-6) if version>0x3336 else b""
    raw += flag(version,-3,3) if version>0x3344 else b""
    raw += integers(-7)+doubles(2) if version>0x3350 else b""
    return raw+(integers(-8) if version>0x3363 else b"")
