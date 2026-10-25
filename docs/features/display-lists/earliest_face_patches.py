"""Remove version-absent face pairs and scene flags from observed complete reference spans."""
from earliest_component_patches import convert as convert_component
from lower_range_patches import string_capture
from generate_transform_vectors import compact
from generate_component_vectors import integers
from mfcobjects import ObjectArchive
import dl_light


def face_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:archive.cstring = saved
        pairs = result["unknown_58"]
        end = spans[-1][0]-8
        start = end-4-8*len(pairs)
        assert archive.b[start:end]==integers(len(pairs),*(value for pair in pairs for value in pair))
        if version<0x1203:patches.append((start,end,b""))
        return result
    return read


def scene_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, kind: int) -> object:
        result = original(archive,kind)
        end = archive.p-20-len(compact(0,0x40))-4-8-12
        assert archive.b[end-4:end]==integers(result["flags"])
        if version<=0x1211:patches.append((end-4,end,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original_face = ObjectArchive.readers["uoTempFaceTessData_c"]
    original_scene = dl_light.read_scene
    ObjectArchive.readers["uoTempFaceTessData_c"] = face_reader(original_face,version,patches)
    dl_light.read_scene = scene_reader(original_scene,version,patches)
    try:return convert_component(document,path,raw,version,edited,patches)
    finally:
        ObjectArchive.readers["uoTempFaceTessData_c"] = original_face
        dl_light.read_scene = original_scene
