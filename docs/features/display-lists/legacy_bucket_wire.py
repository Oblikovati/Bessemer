"""Independent buckets before persistent polygon IDs and packed flag words."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles,compact
from generate_item_vectors import record
from generate_shaded_vectors import ObjectBuilder
from version_range_wire import floats
from bucket_vector_wire import poly_list,line_property,LIST_ORDER


def poly(version: int, dimensions: int, count: int = 1, kind: int = 1, line: bool = True, smooth: int = -17) -> bytes:
    raw = integers(-1,2,smooth,count)
    if count<=0:
        return raw
    raw += floats(count*dimensions)+bytes([255 if line else 0])+(line_property() if line else b"")+integers(kind)
    if kind in (1,2,3):
        raw += integers(-2 if kind==2 else 2)+(b"" if kind==2 else doubles(2))
    raw += integers(-3,4,0,-5,6)
    raw += integers(-7) if version>=0x23b4 else b""
    return raw+(integers(-8,9,-10,0,-11) if version>0x2789 else b"")


def edge() -> bytes:
    raw = doubles(6)+integers(-1,2)+cstring("old component 鋼",True)
    return raw+integers(-3,4,-5,6,-7)+doubles(3)


def component_map(version: int, empty: bool) -> bytes:
    if empty:
        return struct.pack("<H",0)
    key = struct.pack("<H",0xffff) if version<=0x23d7 else integers(-7)
    return struct.pack("<HI",0xffff,2)+key+cstring("first 鋼",True)+key+cstring("second\0")


def thread_maps(builder: ObjectBuilder, version: int, identity: int, empty: bool) -> bytes:
    if empty:
        return integers(0,-1,0,0)
    raw = integers(1)+builder.object("gcBucketEdgeData_c",lambda _:edge())
    raw += poly_list([poly(version,3,kind=3,line=False,smooth=0)])
    raw += integers(1)+struct.pack("<H",identity)+poly_list([poly(version,3,count=-7)])
    return raw+integers(-2,0)


def main_list(label: str, version: int, dimensions: int, empty: bool, edited: bool) -> bytes:
    if empty:
        return poly_list([],True)
    values = {"silh":[poly(version,dimensions,2,smooth=0 if edited else -17),poly(version,dimensions,0),poly(version,dimensions,-3),poly(version,dimensions,kind=9,line=False,smooth=0)],
              "wire":[poly(version,dimensions,kind=2)],"hatch":[poly(version,dimensions,kind=3,smooth=0)]}
    return poly_list(values.get(label,[]),label=="hidden")


def bucket(builder: ObjectBuilder, version: int, identity: int, two_dimensional: bool, empty: bool, edited: bool) -> bytes:
    raw = doubles(3)+integers(-1,2)+doubles(1)+integers(-3,-4 if two_dimensional else 0)
    raw += compact(2,0x20) if two_dimensional else b""
    raw += b"".join(main_list(label,version,2 if two_dimensional else 3,empty,edited) for label in LIST_ORDER)
    item = record(1,doubles(3)+integers(-1,2))
    raw += struct.pack("<H",0 if empty else 1)+(b"" if empty else item)+struct.pack("<H",0)
    raw += integers(3)+component_map(version,empty)+thread_maps(builder,version,identity,empty)+integers(-5)
    raw += poly_list([] if empty else [poly(version,3,kind=9,line=False,smooth=0)])
    raw += integers(-6) if version>0x1c3d else b""
    if version>0x2789:
        raw += integers(0 if empty else 2)+(b"" if empty else integers(*range(-7,7)))
    return raw


def view_bucket(builder: ObjectBuilder, version: int, identity: int, two_dimensional: bool, empty: bool, edited: bool) -> bytes:
    raw = bucket(builder,version,identity,two_dimensional,empty,edited)
    raw += integers(-3 if two_dimensional else 0)+(compact(0,0x40) if two_dimensional else b"")
    raw += integers(4 if two_dimensional else 0)+(compact(2,0x10) if two_dimensional else b"")
    raw += integers(-1,2,-3,4,-5,6,-7,-2 if empty else 2)
    if not empty:
        raw += struct.pack("<H",identity)+builder.object("uiViewBucket_c",lambda state:view_bucket(state,version,state.next_index-1,False,True,False))
    return raw+struct.pack("<H",0)
