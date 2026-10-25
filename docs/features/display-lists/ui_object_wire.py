"""Independent line-style and layer-manager wire bodies, retaining native count spellings."""
import struct
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
from generate_component_vectors import integers


def line_style(variant: int) -> bytes:
    count = 0 if variant==1 else 65535 if variant==3 else 8
    segments = doubles(count)
    if variant==2:segments = segments[:40]+struct.pack("<d",23.0)+segments[48:]
    return cstring("internal 鋼",True)+cstring("external é\0")+struct.pack("<H",count)+segments+bytes([255])


def read_count(version: int, count: int, variant: int) -> bytes:
    if version>=0x1be5:return integers(count)
    if variant==3:return struct.pack("<HI",0xffff,count)
    if variant in (4,5):return struct.pack("<HIQ",0xffff,0xffffffff,count)
    return struct.pack("<H",count)


def layers(version: int, variant: int) -> bytes:
    names = [] if variant==1 else [(-7,""),(-7,"layer 鋼"),(42,"unused é")]
    attributes = [] if variant==1 else [(-7,-12),(-7,23 if variant==2 else 1),(99,-2147483648)]
    raw = read_count(version,len(names),variant)
    raw += b"".join(integers(key)+cstring(value,True) for key,value in names)
    raw += read_count(version,len(attributes),0 if variant==5 else variant)
    return raw+b"".join(integers(key,value) for key,value in attributes)


def body(version: int, class_name: str, variant: int) -> bytes:
    return line_style(variant) if class_name=="moLineStyle_c" else layers(version,variant)


def entries(class_name: str, variant: int) -> int:
    if variant==1:return 0
    if class_name=="uiLayerMgr_c":return 6
    return 65535 if variant==3 else 8
