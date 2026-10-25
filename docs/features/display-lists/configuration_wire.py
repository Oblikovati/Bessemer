"""Independent multi-configuration records; reuse reviewed facet/component wire builders."""
import struct
from generate_component_vectors import integers, viewer, shaded
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
from generate_shaded_vectors import ObjectBuilder, part
from generate_item_vectors import record
from old_shaded_wire import assembly as old_assembly
from graphics_wire import next_identity, ref


def assembly(builder: ObjectBuilder, identity: int) -> bytes:
    original = shaded(6)
    start = len(viewer(6))
    assert original[start:start+2] == ref(0) and original[start+12:start+14] == ref(0)
    payload = builder.object("uoTempPartTessData_c",lambda state:part(state,False))
    return original[:start]+payload+original[start+2:start+12]+ref(identity)+original[start+14:]


def records(builder: ObjectBuilder, version: int, kind: str, viz: int, minimal: bool) -> bytes:
    if minimal:
        return integers(-10)+ref(0) if kind=="part" else integers(-10)+ref(0)+ref(0)
    name = "uoTempPartTessData_c" if kind=="part" else "uoTempAssemblySHDData_c"
    identity = next_identity(builder,name)
    body = (lambda state:part(state,False)) if kind=="part" else (lambda state:old_assembly(state,version,viz,identity) if version<18000 else assembly(state,identity))
    payload = builder.object(name,body)
    first = integers(-10)+(payload if kind=="part" else struct.pack("<H",1)+payload+ref(viz))
    second = integers(-11)+(ref(identity) if kind=="part" else struct.pack("<H",1)+ref(identity)+ref(viz))
    return first+second


def view_tables(builder: ObjectBuilder, count: int, views: int, minimal: bool) -> bytes:
    raw = b""
    identity = next_identity(builder,"moNodeName_c")
    for config in range(count):
        raw += integers(-20-config)
        for view in range(views):
            raw += ref(0) if minimal else builder.object("moNodeName_c",lambda _:cstring("view object 鋼😀\0",True)) if config==0 and view==0 else ref(identity)
    for config in range(count):
        raw += integers(-30-config)
        for view in range(views):
            items = record(1,doubles(3)+integers(-17,3))+record(-77)
            raw += struct.pack("<H",0) if minimal or config else struct.pack("<HI",0xffff,2)+items
    return raw


def suffix(version: int, kind: str, edited: bool, minimal: bool) -> bytes:
    visibility = b""
    if version>0x2b7f if kind=="part" else version>0x2b77:
        visibility = integers(0) if minimal else integers(2)+b"".join(cstring(value,True) for value in ["view 鋼","cfg A\0","view 鋼","cfg B😀"])
    if version<=0x2fa5:
        return visibility
    if minimal:
        return visibility+struct.pack("<II",0,0)
    tail = struct.pack("<II",0,2)+cstring("tail 鋼\0",True)+struct.pack("<II",0,3)+integers(23 if edited else -2147483648,0,2147483647)
    return visibility+tail+cstring("tail 鋼\0",True)+struct.pack("<II",0,0)


def configurations(builder: ObjectBuilder, version: int, kind: str, views: int, viz: int, scenario: int) -> bytes:
    if scenario in (2,3):
        return integers(-7 if scenario==2 else 0)
    minimal = scenario==4
    count = 1 if minimal else 2
    raw = integers(count)+cstring("active 鋼😀\0",True)+integers(-9)
    raw += b"".join(cstring(f"cfg {index} 鋼\0",True) for index in range(count))
    raw += records(builder,version,kind,viz,minimal)
    if version>(0x2b61 if kind=="part" else 0x2b6f):
        raw += view_tables(builder,count,views,minimal)
    return raw+suffix(version,kind,scenario==1,minimal)
