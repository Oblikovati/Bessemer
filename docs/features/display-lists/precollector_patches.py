"""Remove caller-specific members absent in the complete 11000-range root layouts."""
from lower_range_patches import string_capture
from earlier_range_patches import convert as convert_earlier
from mfcobjects import ObjectArchive
import dl_tess


def part_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:
            result = original(archive)
        finally:
            archive.cstring = saved
        start,end = spans[-1]
        assert result["config_name"] and "graphics_connect_ref_mgr_data" not in result
        if version<=0x2b96:patches.append((start-4,end,b""))
        if version<=0x2b9d:patches.append((end,end+4,b""))
        if version<=0x2ba3:patches.append((end+4,end+84,b""))
        return result
    return read


def viewer_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:
            result = original(archive)
        finally:
            archive.cstring = saved
        assert len(result["ref_geom_data_collector"])==4
        start,end = spans[-6]
        assert not result["seed_reference_id"] and "graphics_connect_ref_mgr_data" not in result
        if version<=0x2b9b:patches.append((start,end,b""))
        if version<=0x2b88:patches.append((start-4,start,b""))
        return result
    return read


def face_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:
            result = original(archive,schema)
        finally:
            archive.cstring = saved
        start,end = spans[-1]
        assert result["face_string_empty"]==0
        if version<=0x2bcf:patches.append((start-4,end,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original_part,original_viewer = dl_tess.read_uoTempPartTessData,dl_tess.read_moViewerComponentData
    original_face = ObjectArchive.readers["uoTempFaceTessData_c"]
    dl_tess.read_uoTempPartTessData = part_reader(original_part,version,patches)
    dl_tess.read_moViewerComponentData = viewer_reader(original_viewer,version,patches)
    ObjectArchive.readers["uoTempFaceTessData_c"] = face_reader(original_face,version,patches)
    try:
        return convert_earlier(document,path,raw,version,edited,patches)
    finally:
        dl_tess.read_uoTempPartTessData,dl_tess.read_moViewerComponentData = original_part,original_viewer
        ObjectArchive.readers["uoTempFaceTessData_c"] = original_face
