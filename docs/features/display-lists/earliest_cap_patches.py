"""Observe caller-specific front-view and font end-cap omissions."""
from earliest_state_patches import convert as convert_states
from generate_version_range import capture_plain
from mfcobjects import ObjectArchive
import struct
import dl_doc


def font_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert archive.b[archive.p-2:archive.p]==struct.pack("<h",result["end_cap"])
        if version<0xc9f:patches.append((archive.p-2,archive.p,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    font = ObjectArchive.readers["uiLFConfig_c"]
    front = dl_doc.read_FrontViewTransform
    ObjectArchive.readers["uiLFConfig_c"] = font_reader(font,version,patches)
    if version<=0xc9c:dl_doc.read_FrontViewTransform = capture_plain(front,b"",patches)
    try:return convert_states(document,path,raw,version,edited,patches)
    finally:
        ObjectArchive.readers["uiLFConfig_c"] = font
        dl_doc.read_FrontViewTransform = front
