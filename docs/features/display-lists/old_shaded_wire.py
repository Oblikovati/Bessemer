"""Independent version-10000/11000 shaded records, with inline facets and shared identities."""
from pathlib import Path
import struct
import sys
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import doubles,compact
from version_range_wire import floats
from generate_shaded_vectors import ObjectBuilder
from generate_item_vectors import record
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/"face-tessellations"))
from generate_vectors import facet as facet_wire


def ref(identity: int) -> bytes:
    return struct.pack("<H",identity)


def items() -> bytes:
    table = integers(-1,2,-3)+cstring("table 鋼",True)+integers(-4,5)
    symbol = doubles(3)+integers(-1)+compact(2,0x20)+doubles(2)+cstring("symbol",True)+cstring("other")
    annotation = integers(-1,2,3)+(integers(-7,0,0,0,-8,0)+cstring(""))*3
    return struct.pack("<H",3)+record(23,table)+record(19,symbol)+record(29,annotation)


def texture() -> bytes:
    return doubles(13)+cstring("old texture 鋼",True)+struct.pack("<h",-7)+integers(-8)+doubles(2,3)


def appearance() -> bytes:
    raw = struct.pack("<II",0xffffffff,0xfedcba98)+cstring("rv 鋼",True)+bytes([255])+struct.pack("<I",0xffffffff)
    raw += floats(10)+integers(-1)+floats(5)+integers(-2)+floats(2)+cstring("shader",True)
    raw += struct.pack("<I",0xfffffffe)+floats(2)+cstring("bump",True)+floats(3)+integers(-3)
    return raw+floats(1)+integers(-4)+floats(13)+cstring("lxp 鋼",True)+floats(1)


def visual(version: int, edited: bool) -> bytes:
    raw = struct.pack("<III",0xffffffff,0xfffffffe,0xfffffffd)+cstring("material 鋼",True)+cstring("ID")
    raw += (struct.pack("<d",23.0) if edited else doubles(1))+doubles(5,1)+integers(-1)+texture()
    raw += integers(-2,3,-4,5,-6)+cstring("classification",True)+integers(-7,-8)+appearance()+struct.pack("<I",0)
    raw += cstring("addin",True)+integers(-9,10)+cstring("appearance",True)+integers(-11,12,-13,14)
    return raw+(integers(-15) if version>0x2794 else b"")


def face(version: int) -> bytes:
    raw = facet_wire(version,0,False,False)+integers(0,-34,-68,2,-1,2,3,-4,-38)
    return raw


def body(builder: ObjectBuilder, version: int, viz: int) -> bytes:
    raw = struct.pack("<H",2)+b"".join(builder.object("uoTempFaceTessData_c",lambda _:face(version)) for _ in range(2))
    raw += integers(-20,-24,-44,1,-1)+builder.object("uoBodyPropInfo_c",lambda _:ref(viz)+integers(-8,6,-16,3))
    raw += integers(0)+cstring("body 鋼",True)
    return raw+(integers(-104) if version>0x2791 else b"")


def part(builder: ObjectBuilder, version: int, viz: int) -> bytes:
    raw = struct.pack("<H",1)+builder.object("uoTempBodyTessData_c",lambda state:body(state,version,viz))
    raw += integers(1,-1)+ref(viz)+struct.pack("<I",0xffffffff)+integers(2,-1,2,-3,4)
    return raw+struct.pack("<I",0)+integers(-168,2)


def component(version: int, viz: int) -> bytes:
    raw = cstring("old component 鋼",True)+compact(2,0x20)+doubles(10)+ref(viz)+cstring("config",True)
    raw += integers(-1)+struct.pack("<II",0xffffffff,0xfffffffe)+cstring("model")+cstring("path",True)
    raw += integers(-2,2,-3,4,6)+doubles(1)+integers(-5,6,-7,8)+struct.pack("<I",0xfffffffd)+integers(-9,10,-11,12,-13)
    raw += b"".join(cstring(value,True) for value in ["ref","spool","description"])
    raw += cstring("config description",True) if version>0x279c else b""
    raw += integers(1)+cstring("dependency",True)+integers(1)+cstring("key")+integers(1)+cstring("value",True)
    raw += integers(-14,15,-16,17,-18,19)
    return raw+(integers(-20)+struct.pack("<H",0xffff)+integers(-21,22) if version>0x279d else b"")


def assembly(builder: ObjectBuilder, version: int, viz: int, identity: int) -> bytes:
    raw = component(version,viz)+builder.object("uoTempPartTessData_c",lambda state:part(state,version,viz))
    return raw+integers(-32)+struct.pack("<H",0)+ref(identity)+integers(-33,0,-34,2,-35,36)
