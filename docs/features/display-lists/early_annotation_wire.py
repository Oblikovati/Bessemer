"""Minimal complete EDW records before the early optional state/string fields."""
import struct
from generate_component_vectors import integers
from generate_item_vectors import record


def minimal_record(version: int) -> bytes:
    raw = integers(-7,0,0)
    raw += integers(0) if version>0x1c4b else b""
    raw += integers(-9) if version>0x1fa8 else b""
    raw += integers(0) if version>0x1fd4 else b""
    raw += b"\0" if version>0x23fe else b""
    return raw+(bytes(2) if version>0x332b else b"")


def minimal_items(version: int) -> bytes:
    payload = integers(-1,2,100)+minimal_record(version)*100
    return struct.pack("<H",1)+record(29,payload)
