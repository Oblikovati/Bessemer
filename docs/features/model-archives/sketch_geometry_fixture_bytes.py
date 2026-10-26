"""Direct sketch point, segment, line and text wire records without MFC entity tags."""
import struct
from sketch_reference_fixture_bytes import entity, handle
from freeze_sketch_text import wire as dimension_text
from owned_name_edit import short_wide_name


def coordinates(mask: int) -> bytes:
    return struct.pack('<2d', -0.0, 1.25) if mask & 0x10 else struct.pack('<3d', -0.0, 1.25, -2.5)


def point(mask: int, rich: bool, handles: bool) -> bytes:
    out = entity(rich)+struct.pack('<H', mask)+coordinates(mask)+struct.pack('<HH', 65535, 65534)
    out += handle('sgPointHandle', -1)+struct.pack('<H', 2) if handles else bytes(4)
    out += struct.pack('<Hi', 65533, -2**31)+coordinates(mask)*2
    return out+struct.pack('<HH', 0, 65532)


def segment(rich: bool, vector: bool, escaped: bool, flags_54: int = 0xffff) -> bytes:
    out = entity(rich, 0x8200 if vector else 0x8000, flags_54)
    out += struct.pack('<HiHi', 65535, -2**31, 65535, 2**31-1) if escaped else struct.pack('<HH', 65534, 9)
    return out+struct.pack('<HHd', 65535, 65534, -0.0)


def line(rich: bool, vector: bool, escaped: bool, final_word: int = 65535) -> bytes:
    out = segment(rich, vector, escaped)
    if vector:
        out += struct.pack('<3d', -0.0, 3.5, -4.75)
    return out+struct.pack('<H', final_word)


def text(version: int, rich: bool, alias: int | None = None, curve: bool = False) -> bytes:
    out = segment(rich, False, True)
    out += dimension_text(version, 0, 1 if curve else 0, False) if alias is None else struct.pack('<H', alias)
    if curve:
        return out+b'\0\0'
    return out+short_wide_name('<RTL>Ω shape')+struct.pack('<ii', -2**31, 2**31-1)


def texts(version: int, rich: bool, count: int, alias: int = 2) -> bytes:
    out = struct.pack('<i', count)
    if count:
        out += text(version, rich)
    return out+(text(version, rich, alias) if count == 2 else b'')


def composite(version: int, rich: bool) -> bytes:
    out = struct.pack('<iH', 1, 65535)+point(0x8010, rich, True)
    out += struct.pack('<iH', 1, 7)+line(rich, True, True)
    return out+texts(version, rich, 2, 4)
