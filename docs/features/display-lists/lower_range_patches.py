"""Additional reference-guided layouts below display version 16000."""
from generate_version_range import apply_patches
from generate_component_vectors import integers
from old_shaded_wire import texture
from middle_range_patches import convert as convert_middle
from middle_range_source import displaylists
import dl_light
import dl_doc
import dl_tess
from mfcobjects import ObjectArchive


def texture_source(document: object, path: object, raw: bytes) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    original = dl_light.read_uoMaterialProperties
    def read(archive: object) -> object:
        start = archive.p
        archive.cstring()
        archive.cstring()
        position = archive.p+48
        archive.p = start
        result = original(archive)
        assert "texture" not in result and raw[position:position+4]==integers(0)
        patches.append((position,position+4,integers(-17)+texture()+integers(-9)))
        return result
    try:
        dl_light.read_uoMaterialProperties = read
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p==len(raw)
    finally:
        dl_light.read_uoMaterialProperties = original
    return apply_patches(raw,patches)


def tail_capture(original: object, absent: bool, width: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        if absent:
            patches.append((archive.p-width,archive.p,b""))
        return result
    return read


def material_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        original_string = archive.cstring
        spans: list[tuple[int,int]] = []
        archive.cstring = string_capture(original_string,archive,spans)
        try:
            result = original(archive)
        finally:
            archive.cstring = original_string
        assert result["user_property"]==[] and "texture2" not in result
        if version<=0x372a:
            patches.append((archive.p-8,archive.p-4,b""))
        if version<=0x3368:
            start,end = spans[-1]
            patches.append((start,end,b""))
        return result
    return read


def string_capture(original: object, archive: object, spans: list[tuple[int,int]]) -> object:
    def read() -> str:
        start = archive.p
        result = original()
        spans.append((start,archive.p))
        return result
    return read


def viewer_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        original_string = archive.cstring
        spans: list[tuple[int,int]] = []
        archive.cstring = string_capture(original_string,archive,spans)
        try:
            result = original(archive)
        finally:
            archive.cstring = original_string
        assert result["viewer_comp_tess_numbers"]==[]
        start,end = spans[-1]
        replacement = b"" if version<=0x3b1b else integers(-91,92) if version<0x3b23 else integers(2,-1,2,3,-4,5,6)
        patches.append((start-4,start,replacement))
        if version<=0x3b1c:
            patches.append((start,end,b""))
        return result
    return read


def face_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        missing = 8 if version<=0x336b else 4 if version<0x3743 else 0
        if missing:
            patches.append((archive.p-missing,archive.p,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]] | None = None) -> bytes:
    patches = [] if patches is None else patches
    bindings = [(dl_light,"read_uoMaterialTextureProperties"),(dl_tess,"read_uoTempPartTessData"),(dl_tess,"read_moViewerComponentData"),(dl_light,"read_uoMaterialProperties"),(dl_doc,"read_OrigFontScale")]
    originals = [getattr(module,name) for module,name in bindings]
    replacements = [tail_capture(originals[0],version<=0x372a,4,patches),tail_capture(originals[1],version<=0x3751,1,patches),viewer_capture(originals[2],version,patches),material_capture(originals[3],version,patches),tail_capture(originals[4],version<=0x3331,8,patches)]
    original_face = ObjectArchive.readers["uoTempFaceTessData_c"]
    ObjectArchive.readers["uoTempFaceTessData_c"] = face_capture(original_face,version,patches)
    try:
        for (module,name),reader in zip(bindings,replacements,strict=True):setattr(module,name,reader)
        return convert_middle(document,path,raw,version,edited,patches)
    finally:
        ObjectArchive.readers["uoTempFaceTessData_c"] = original_face
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)
