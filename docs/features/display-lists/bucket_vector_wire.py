"""Wire primitives for owned modern buckets; dimensions follow each serializer's scope."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import compact, doubles
from generate_item_vectors import record
from generate_shaded_vectors import ObjectBuilder

LIST_ORDER = ["silh","silh_h","wire","hidden","hatch","aux","tan_v","tan_h","explode_v","explode_h","weldbead_v","weldbead_h","solid_fill"]


def poly(count: int, dimensions: int, kind: int = 1, arc: bool = True, line: bool = True) -> bytes:
    raw = integers(-1,2)+struct.pack("<I",0xfedcba98 | (0x20 if arc else 0))+integers(count)
    if count <= 0:
        return raw
    raw += doubles(count*dimensions)+bytes([255 if line else 0])
    if line:
        raw += line_property()
    raw += integers(kind)
    if kind in (1,2,3):
        raw += integers(-2 if kind == 2 else 2)+(b"" if kind == 2 else doubles(2))
    raw += integers(-6,7,-8,9,-10)
    if arc:
        raw += doubles(9)+struct.pack("<BBiBi",255,2,-11,3,-12)
    return raw+struct.pack("<IIIi",0xffffffff,0xfffffffe,0xfffffffd,-13)+doubles(2)


def line_property() -> bytes:
    raw = cstring("line 鋼",True)
    return raw+struct.pack("<hIii",-3,0x80000000,-4,5)


def poly_list(values: list[bytes], negative: bool = False) -> bytes:
    return integers(-3 if negative else len(values))+b"".join(values)


def edge() -> bytes:
    raw = doubles(6)+integers(-1)+cstring("component 鋼",True)+struct.pack("<II",0xffffffff,0xfffffffe)
    raw += integers(-2,3,-4,5)+doubles(3)+struct.pack("<BBIiBi",2,255,0xfffffffd,-6,3,-7)
    return raw+doubles(2)+integers(-8)


def component_map(empty: bool, wide: bool = True) -> bytes:
    if empty:
        return struct.pack("<H",0)
    raw = struct.pack("<HI",0xffff,2)
    key = integers(-7) if wide else struct.pack("<H",65529)
    return raw+key+cstring("first 鋼",True)+key+cstring("second\0")


def thread_maps(builder: ObjectBuilder, identity: int, empty: bool) -> bytes:
    if empty:
        return integers(0,-1,0,0)
    raw = integers(1)+builder.object("gcBucketEdgeData_c",lambda _:edge())
    raw += poly_list([poly(1,3,3,False,False)])
    raw += integers(1)+struct.pack("<H",identity)+poly_list([poly(-7,3)])
    return raw+integers(-2,0)


def bucket(builder: ObjectBuilder, identity: int, two_dimensional: bool, empty: bool) -> bytes:
    raw = doubles(3)+integers(-1,2)+doubles(1)+integers(-3,-4 if two_dimensional else 0)
    raw += compact(2,0x20) if two_dimensional else b""
    for name in LIST_ORDER:
        raw += main_list(name,2 if two_dimensional else 3,empty)
    item = record(1,doubles(3)+integers(-1,2))
    raw += struct.pack("<H",0 if empty else 1)+(b"" if empty else item)+struct.pack("<H",0)
    raw += integers(3)+component_map(empty)+thread_maps(builder,identity,empty)+integers(-5)
    raw += poly_list([] if empty else [poly(1,3,9,False,False)])+integers(-6,0 if empty else 2)
    raw += b"" if empty else integers(*range(-7,7))
    return raw+integers(-8)+struct.pack("<H",0 if empty else 1)+(b"" if empty else item)+integers(-9)


def main_list(name: str, dimensions: int, empty: bool) -> bytes:
    if empty:
        return poly_list([],True)
    values = {"silh":[poly(2,dimensions),poly(0,dimensions),poly(-3,dimensions),poly(1,dimensions,9,False,False)],"wire":[poly(1,dimensions,2,False)],"hatch":[poly(1,dimensions,3)]}
    return poly_list(values.get(name,[]),name == "hidden")


def view_bucket(builder: ObjectBuilder, identity: int, two_dimensional: bool, empty: bool, mode: int = -1) -> bytes:
    raw = bucket(builder,identity,two_dimensional,empty)
    raw += integers(-3 if two_dimensional else 0)+(compact(0,0x40) if two_dimensional else b"")
    raw += integers(4 if two_dimensional else 0)+(compact(2,0x10) if two_dimensional else b"")
    raw += integers(mode,2,-3,4,-5,6,-7,-2 if empty else 2)
    if not empty:
        raw += struct.pack("<H",identity)+builder.object("uiViewBucket_c",lambda state:view_bucket(state,state.next_index-1,False,True))
    raw += struct.pack("<H",0)+integers(0 if empty else -8)
    raw += b"" if empty else bucket(builder,identity,False,True)
    return raw+integers(-9)
