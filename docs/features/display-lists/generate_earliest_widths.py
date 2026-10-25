"""Complete roots with short font widths and full-width legacy display-item pens."""
import json
import struct
import generate_earliest_visuals as roots
from generate_earlier_range import write_group
from generate_item_vectors import record
from generate_component_vectors import integers
from generate_named_vectors import cstring

ORIGINAL_ITEMS = roots.body_items
ORIGINAL_ENTRIES = roots.entry_count


def width(version: int, positive: bool, edited: bool) -> bytes:
    if version<0xf41:
        value = 2147483647-int(edited) if positive else -2147483648+int(edited)
        return struct.pack("<i",value)
    value = 32767-int(edited) if positive else -32768+int(edited)
    return struct.pack("<hI",value,1 if positive else 0x7f800042)


def items(version: int, edited: bool) -> bytes:
    raw = ORIGINAL_ITEMS(version)
    style = integers(-1,2)+struct.pack("<I",0xffffffff)+integers(-3)+width(version,False,edited)+integers(4)+(integers(-5) if version>0xca5 else b"")
    pen = struct.pack("<I",0xffffffff)+width(version,True,edited)+integers(-3)+(integers(4) if version>=0xca6 else b"")+cstring("legacy pen")
    count, = struct.unpack_from("<H",raw)
    return struct.pack("<H",count+2)+raw[2:]+record(9,style)+record(12,pen)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%16
    roots.body_items = lambda current:items(current,variant==1)
    roots.entry_count = lambda v,k,s:ORIGINAL_ENTRIES(v,k,s)+2
    try:framed,row = roots.generate(version,kind,0)
    finally:roots.body_items,roots.entry_count = ORIGINAL_ITEMS,ORIGINAL_ENTRIES
    row["scenario"] = 240+variant
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


if __name__ == "__main__":
    cases = [generate(v,k,s) for v in [3196,0xca5,0xca6,3890,0xf40,0xf41] for k in ["part","assembly"] for s in [0,1]]
    print(write_group("earliest-width-items",cases,("displaylists.py",)))
