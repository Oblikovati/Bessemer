"""Early view/item and shaded-root bytes for display versions 7000 and 9000."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles,compact
from generate_item_vectors import record
from generate_light_vectors import base
from generate_scene_vectors import concept,pw_scene,wall,floor
from old_shaded_wire import visual,part,assembly,item_parts
from generate_shaded_vectors import ObjectBuilder


def simple_text_payload(version: int, float_bits: int = 0x7f800042) -> bytes:
    raw = doubles(3)+integers(-1)+compact(0,0x40)+doubles(1)+integers(-2)+cstring("early text 鋼",True)+struct.pack("<h",0)
    raw += struct.pack("<I",float_bits) if 0x1c04<version<0x1fb5 else b""
    return raw+(integers(-3) if version>0x2000 else b"")


def early_items(version: int) -> bytes:
    text = simple_text_payload(version)
    spline = integers(1)+doubles(1)+(integers(-4) if version>=0x2016 else b"")+doubles(6)
    spline += cstring("spline 鋼",True) if version>=0x1ff8 else b""
    extra = [record(18,text),record(31,spline),record(30,doubles(3)+integers(-5))]
    parts = item_parts(version)
    return struct.pack("<H",len(parts)+len(extra))+b"".join(parts[:2]+extra+parts[2:])


def early_scene(builder: ObjectBuilder, version: int, viz: int) -> bytes:
    raw,identity = concept(builder,viz,False,version)
    raw += builder.object("pwSceneData_c",lambda state:pw_scene(state,False))+integers(3)
    raw += b"".join(wall(kind,viz) for kind in [1,2,3])
    return raw+floor(identity,False,0x7ff0000000000000)[:-20]


def early_shaded(builder: ObjectBuilder, version: int, kind: str, edited: bool) -> bytes:
    raw = integers(1,1)+struct.pack("<H",2)
    raw += builder.object("moAmbientLight_c",lambda _:base(True))
    raw += builder.object("moDirectionLight_c",lambda _:base(True)+doubles(4))
    viz = builder.next_index+1
    raw += builder.object("moVisualProperties_c",lambda _:visual(version,edited))
    if kind=="part":
        raw += part(builder,version,viz)
    else:
        raw += struct.pack("<H",1)+builder.object("uoTempAssemblySHDData_c",lambda state:assembly(state,version,viz,state.next_index-1))
    raw += integers(0,-12,1)+early_scene(builder,version,viz)
    return raw+(integers(-1,2,-3) if version>0x1fb2 else b"")
