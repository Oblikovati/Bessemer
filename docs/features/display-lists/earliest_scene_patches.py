"""Observe native reader spans and omit the two 4850-range gated words."""
from early_appearance_patches import convert as convert_appearance
from lower_range_patches import string_capture
from mfcobjects import ObjectArchive
import dl_tess


def part_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        start,_ = spans[-1]
        assert result["config_name"] and "graphics_connect_ref_mgr_data" not in result
        if version<=0x12fc:patches.append((start-8,start-4,b""))
        return result
    return read


def scene_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert "use_sky_shader" in result and "bg_fit_to_sw_graphics_area" in result
        if version<=0x1310:patches.append((archive.p-8,archive.p-4,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original_part = dl_tess.read_uoTempPartTessData
    original_scene = ObjectArchive.readers["pwSceneData_c"]
    dl_tess.read_uoTempPartTessData = part_reader(original_part,version,patches)
    ObjectArchive.readers["pwSceneData_c"] = scene_reader(original_scene,version,patches)
    try:return convert_appearance(document,path,raw,version,edited,patches)
    finally:
        dl_tess.read_uoTempPartTessData = original_part
        ObjectArchive.readers["pwSceneData_c"] = original_scene
