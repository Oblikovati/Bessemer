"""Reference-observed old appearance, face and null wall-material pointer layouts."""
from earliest_light_patches import convert as convert_lights
from lower_range_patches import string_capture
from mfcobjects import ObjectArchive
import dl_light


def appearance_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        assert all(key in result for key in ["axis_direction1","appearance_u_dir","lxp_filename"])
        if version<0x1165:patches.append((spans[1][0]-8,spans[2][1]+16,b""))
        if version<=0x1179:patches.append((spans[-1][0]-60,spans[-1][0]-20,b""))
        return result
    return read


def face_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:archive.cstring = saved
        start = spans[-1][0]-8-4-8*len(result["unknown_58"])
        if version<0x117d:patches.append((start-4,start,b""))
        return result
    return read


def wall_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        assert archive.p==start+14 and int.from_bytes(archive.b[start+8:start+10],"little")<0x7fff
        if version<0x117f:
            patches.extend([(start,start,b"\0\0"),(start+8,start+14,b"")])
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    bindings = [(dl_light,"read_uoRVAppearanceProperties",appearance_reader),(dl_light,"read_swWall_c",wall_reader)]
    originals = [getattr(module,name) for module,name,_ in bindings]
    original_face = ObjectArchive.readers["uoTempFaceTessData_c"]
    for (module,name,wrapper),original in zip(bindings,originals,strict=True):setattr(module,name,wrapper(original,version,patches))
    ObjectArchive.readers["uoTempFaceTessData_c"] = face_reader(original_face,version,patches)
    try:return convert_lights(document,path,raw,version,edited,patches)
    finally:
        for (module,name,_),original in zip(bindings,originals,strict=True):setattr(module,name,original)
        ObjectArchive.readers["uoTempFaceTessData_c"] = original_face
