"""Independent old inline facets, with a non-geometric stamp edit located by the reader."""
import struct
from pathlib import Path
from old_shaded_wire import facet_wire
from swarchive import Archive
import facettess

VERSION = 0xa24


def facet_stamp_offset(raw: bytes) -> int:
    archive,words = Archive(raw,VERSION),[]
    original = archive.u32
    def read() -> int:
        first = archive.p
        value = original()
        words.append((first,value))
        return value
    archive.u32 = read
    result = facettess.facet_container(archive)
    assert archive.p==len(raw)
    positions = [position for position,value in words if value==result["stamp"]]
    assert len(positions)==1,(result["stamp"],positions)
    return positions[0]


def facet_bytes(enabled: bool, edited: bool) -> bytes:
    raw = facet_wire(VERSION,-17,False,enabled)
    position = facet_stamp_offset(raw)
    if edited:raw = raw[:position]+struct.pack("<I",0xdeadbeee)+raw[position+4:]
    archive = Archive(raw,VERSION)
    result = facettess.facet_container(archive)
    assert archive.p==len(raw) and result["stamp"]==0xdeadbeef-int(edited)
    return raw


def write_facets() -> None:
    for enabled in (False,True):
        for edited in (False,True):
            name = f"resumed-inline-facet-{int(enabled)}-{int(edited)}.bin"
            Path("tests/fixtures/display-lists",name).write_bytes(facet_bytes(enabled,edited))
