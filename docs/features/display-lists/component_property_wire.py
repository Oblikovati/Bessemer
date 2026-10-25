"""Independent embedded fonts, visibility, and shared component-property CObjects."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from ui_object_wire import line_style


def font_count(version: int) -> int:
    if version<0xf32:return 15
    if version<0xf5c:return 23
    if version<0x12a3:return 24
    if version<0x12cf:return 25
    if version<0x13ef:return 26
    return 30 if version<0x23b3 else 31


def font(index: int, version: int = 19000) -> bytes:
    bits = [0x80000000,0x7f800042,0x7f800000,0x7fc00123,1][index%5]
    width = struct.pack("<h",-17)+(struct.pack("<I",bits) if version>=0xf41 else b"")
    return cstring(f"font {index} 鋼",True)+width+(struct.pack("<h",-32768) if version>=0xc9f else b"")


def property_body(state: object, version: int, variant: int) -> bytes:
    raw = b"".join(font(index,version) for index in range(font_count(version)))
    raw += struct.pack("<HHBIHh",0xffff,0x8000,254,0xfedcba98,0xffff,-32768)
    raw += struct.pack("<I",0x7f800042) if version>=0xf41 else b""
    raw += bytes([23 if variant==2 else 255])
    raw += integers(-17) if version>=0x147c else b""
    return raw+(integers(-19) if version>0x3738 else b"")


def pointer(identity: int) -> bytes:
    return struct.pack("<H",identity)


def new_property(state: object, version: int, variant: int) -> tuple[bytes,int]:
    identity = state.next_index+int("moComponentProperty" not in state.classes)
    raw = state.object("moComponentProperty",lambda builder:property_body(builder,version,variant))
    return raw,identity


def occurrences(variant: int, nested: bool = False) -> bytes:
    values = [] if variant in (1,4) else [0xffffffff] if nested else [0,0xffffffff,0x80000000]
    count = struct.pack("<HI",0xffff,len(values)) if variant in (3,4) else struct.pack("<H",len(values))
    return count+b"".join(struct.pack("<I",value) for value in values)


def nested_holder(state: object, version: int, owner: int, shared: list[int]) -> bytes:
    raw = pointer(owner)+cstring("nested 鋼",True)
    prop,identity = new_property(state,version,0)
    shared.append(identity)
    return raw+prop+occurrences(0,True)+holder_state(version)


def unknown_pointer(state: object, version: int, variant: int, owner: int, shared: list[int]) -> bytes:
    if variant==1:return pointer(0)
    if variant==3:return state.object("moLineStyle_c",lambda _:line_style(0))
    if variant==5:return state.object("moComponentPropertyHolder",lambda builder:nested_holder(builder,version,owner,shared))
    return pointer(owner)


def holder(state: object, version: int, variant: int) -> bytes:
    owner,shared = state.next_index-1,[]
    raw = unknown_pointer(state,version,variant,owner,shared)+cstring("" if variant==1 else "component 鋼",True)
    if variant==1:prop = pointer(0)
    elif shared:prop = pointer(shared[0])
    else:prop,_ = new_property(state,version,variant)
    return raw+prop+occurrences(variant)+holder_state(version)


def holder_state(version: int) -> bytes:
    return integers(-2147483648,-44) if version>=0x106a else b""


def body(state: object, version: int, class_name: str, variant: int) -> bytes:
    if class_name=="moComponentProperty":return property_body(state,version,variant)
    return holder(state,version,variant)


def entries(version: int, class_name: str, variant: int) -> int:
    if class_name=="moComponentProperty":return font_count(version)
    if variant==1:return 0
    return font_count(version)+(8 if variant==3 else 0)+(0 if variant==4 else 4 if variant==5 else 3)
