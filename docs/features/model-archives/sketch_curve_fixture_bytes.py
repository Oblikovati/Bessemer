"""Independent direct hatch, arc, ellipse, conic and surface wire assembly."""
import struct
from freeze_views import object_tag
from sketch_reference_fixture_bytes import entity
from sketch_geometry_fixture_bytes import segment
from owned_name_edit import short_wide_name


def escaped(index: int, wide: bool) -> bytes:
    return struct.pack('<Hi', 65535, index) if wide else struct.pack('<H', index)


def arc(rich: bool, endpoints: bool, center: bool) -> bytes:
    out = segment(rich, True, endpoints)+struct.pack('<i', -2**31)
    out += escaped(-2**31 if center else 65534, center)
    return out+struct.pack('<4i', 2**31-1, -1, -2**31, 0)


def ellipse(rich: bool, endpoints: bool, mask: int) -> bytes:
    out = segment(rich, True, endpoints)+struct.pack('<i', -17)
    for bit in range(5):
        wide = bool(mask & 1 << bit)
        out += escaped(-2**31+bit if wide else 65534-bit, wide)
    return out


def conic(rich: bool) -> bytes:
    out = segment(rich, False, True)+struct.pack('<4i', -2**31, 2**31-1, -1, 0)
    return out+struct.pack('<QQH', 0x8000000000000000, 0x7ff000000000002a, 65535)


def count(value: int, extended: bool) -> bytes:
    return struct.pack('<HI', 65535, value) if extended else struct.pack('<H', value)


def crosshatch() -> bytes:
    return object_tag('gcXhatch_c')+short_wide_name('Ω hatch')+struct.pack('<2d6i', -0.0, 1.25, -1, 2, -3, 4, -5, 6)


def hatch(rich: bool, flags: int, extended: bool, list_class: str) -> bytes:
    out = entity(rich, flags_54=flags)
    if flags & 1:
        return out
    for length in [0, 1, 2, 3]:
        out += struct.pack('<i', length if rich else 0)
        if rich: out += struct.pack('<'+'I'*length, *[0xffffffff, 0, 0x89abcdef][:length])
    out += crosshatch() if rich else b'\0\0'
    out += b'\0\0'+struct.pack('<i', 2 if rich else 0)
    if rich: out += struct.pack('<iHiH', -2**31, 65535, 2**31-1, 0)
    out += b'\0\0'
    out += object_tag(list_class)+count(3, extended)+struct.pack('<3H', 2, 0, 4) if rich else b'\0\0'
    out += object_tag('su_CDWordArray')+count(3, extended)+struct.pack('<3I', 0, 0xffffffff, 0x89abcdef) if rich else b'\0\0'
    return out


def surface(kind: int, rich: bool, plane_flags: int = 3) -> bytes:
    out = entity(rich)
    if kind == 0:
        out += struct.pack('<17dB', -0.0, *[index/4 for index in range(1, 17)], 255)
        out += bytes([255 if plane_flags & 1 else 0])
        if plane_flags & 1: out += short_wide_name('Ω plane')
        out += struct.pack('<dB', -0.0, 254 if plane_flags & 2 else 0)
        if plane_flags & 2: out += struct.pack('<3d', -0.0, 3.5, -4.75)
    if kind in (1, 3):
        out += struct.pack('<7d', -0.0, 1, 2, 3, 4, 5, -6)
    if kind == 2:
        out += struct.pack('<4d', -0.0, 1, 2, -3)
    if kind == 3:
        out += struct.pack('<2Q', 0x8000000000000000, 0x7ff000000000002a)
    return out


def surfaces(rich: bool, active: int | None, plane_flags: int = 3) -> bytes:
    return b''.join(struct.pack('<i', 1 if active is None or active == kind else 0)
                    +(surface(kind, rich, plane_flags) if active is None or active == kind else b'') for kind in range(5))


def composite(rich: bool) -> bytes:
    out = struct.pack('<i', 1)+hatch(rich, 0xfffe, True, 'su_CObList')
    out += struct.pack('<iH', 1, 7)+arc(rich, True, True)
    out += struct.pack('<iH', 1, 65535)+ellipse(rich, True, 31)
    out += struct.pack('<iH', 1, 4)+conic(rich)
    return out+surfaces(rich, None)
