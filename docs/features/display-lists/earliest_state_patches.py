"""Observe the old appearance word before it became part of the stored layout."""
from earliest_width_patches import convert as convert_widths
from lower_range_patches import string_capture
from mfcobjects import ObjectArchive
import struct
import dl_light


def visual_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans,ends = archive.cstring,[],[]
        material = dl_light.read_uoMaterialProperties
        dl_light.read_uoMaterialProperties = material_end_reader(material,ends)
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:
            archive.cstring = saved
            dl_light.read_uoMaterialProperties = material
        appearance_omissions(archive,result,spans,ends,version,patches)
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original = ObjectArchive.readers["moVisualProperties_c"]
    ObjectArchive.readers["moVisualProperties_c"] = visual_reader(original,version,patches)
    try:return convert_widths(document,path,raw,version,edited,patches)
    finally:ObjectArchive.readers["moVisualProperties_c"] = original


def material_end_reader(original: object, ends: list[int]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        ends.append(archive.p)
        return result
    return read



def appearance_omissions(archive: object, result: dict[str,object], spans: list[tuple[int,int]], ends: list[int], version: int, patches: list[tuple[int,int,bytes]]) -> None:
    assert len(ends)==1
    outer_strings = [span for span in spans if span[0]>=ends[0]]
    start = outer_strings[1][1]+4
    assert archive.b[start:start+4]==struct.pack("<i",result["default_appearance"])
    if version<=0xdb1:
        assert archive.b[start-4:start]==struct.pack("<i",result["hardcoded_facet_display"])
        patches.append((start-4,start,b""))
    if version<0xe55:patches.append((start,start+4,b""))
