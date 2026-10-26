"""Independent direct spline records with explicit point-list and optional-body branches."""
import struct
from freeze_views import object_tag, transform
from sketch_geometry_fixture_bytes import segment
from sketch_reference_fixture_bytes import handle
from spline_array_fixture_bytes import curve, doubles, interpolation, secondary, vectors
from kernel_geometry_fixture import frame
from owned_name_edit import short_wide_name

LISTS = [None, 'sgSplineList_c', 'modifSplineList_c', 'cptsSplineList_c']


def point_list(name: str | None, rich: bool, alias: int | None) -> bytes:
    if alias is not None:
        return struct.pack('<H', alias)
    if name is None:
        return b'\0\0'
    count = struct.pack('<HI', 65535, 2) if rich else struct.pack('<H', 0)
    return object_tag(name)+count+(handle('sgPointHandle', -1)+struct.pack('<H', 4) if rich else b'')


def prefix(rich: bool, flags: int, name: str | None, alias: int | None, inactive: bool) -> bytes:
    out = segment(rich, rich, True, 0xffff if inactive else 0xfffe)+struct.pack('<IH', flags, 65535)
    out += point_list(name, rich, alias)
    return out+struct.pack('<HHi3di6dH', 65534, 65533, -17, -0.0, 1.25, -2.5, 19, -0.0, 1, 2, 3, 4, 5, 65532)


def arrays(rich: bool, flags: int) -> bytes:
    count = 2 if rich else 0
    out = struct.pack('<i', count)+doubles(count)
    if not flags & 1:
        out += struct.pack('<i', 1 if rich else 0)+(vectors(1) if rich else b'')
    out += struct.pack('<I', 3 if rich else 0)
    if rich:
        for index, mode in enumerate([0, 2, 65535]):
            out += struct.pack('<iH', -index-1, mode)+vectors(1)
            if mode == 2: out += struct.pack('<d', -0.0)
    return out


def suffix(name: str | None, rich: bool, flags: int, secondary_flag: bool, curve_header: tuple[int, int, int, int] = (3, 2, 1, 0)) -> bytes:
    out = struct.pack('<iHH', -1 if secondary_flag else 0, 65531, 65530)
    if secondary_flag and rich and not flags & 1: out += secondary(2)
    out += struct.pack('<i', -23)
    if flags & 4:
        out += b''.join(short_wide_name(f'Ω equation {index}') for index in range(6))
        out += struct.pack('<ii', -2**31, 2**31-1)+transform(True, 0x1a)
    out += struct.pack('<i', -29)
    if name == 'cptsSplineList_c':
        out += curve(*curve_header, 255 if rich else 0) if flags & 1 else struct.pack('<H', 255 if rich else 0)+(struct.pack('<d', -0.0) if rich else b'')
    return out


def final_fields(rich: bool, kernel: int, final_word: int) -> bytes:
    out = struct.pack('<dI', -0.0, 2 if rich else 0)
    if rich: out += struct.pack('<iHiH', -2**31, 0, 2**31-1, 0)
    out += struct.pack('<HH', 65529, 65535 if kernel else 0)
    if kernel: out += b'\xc3' if kernel == 1 else frame()
    return out+struct.pack('<i', final_word)


def spline(name: str | None, rich: bool, flags: int, interpolation_mode: int, secondary_flag: bool,
           kernel: int = 0, alias: int | None = None, inactive: bool | None = None, final_word: int = -31) -> bytes:
    inactive = name is None if inactive is None else inactive
    out = prefix(rich, flags, name, alias, inactive)+arrays(rich, flags)+interpolation(interpolation_mode)
    return out+suffix(name, rich, flags, secondary_flag)+final_fields(rich, kernel, final_word)


def collection(name: str | None, rich: bool, flags: int, interpolation_mode: int, secondary_flag: bool,
               mask: int = 1, kernel: int = 0, final_word: int = -31) -> bytes:
    return struct.pack('<iH', 1, mask)+spline(name, rich, flags, interpolation_mode, secondary_flag, kernel, final_word=final_word)


def alias_collection(name: str) -> bytes:
    first = struct.pack('<H', 1)+spline(name, True, 5, 3, True)
    second = struct.pack('<H', 29)+spline(name, False, 0, 0, False, alias=2)
    return struct.pack('<i', 2)+first+second
