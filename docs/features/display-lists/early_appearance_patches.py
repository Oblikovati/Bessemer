"""Reference-guided field omissions in 5000-range appearance and face records."""
from historical_scene_patches import convert as convert_historical
from lower_range_patches import string_capture
from mfcobjects import ObjectArchive
import dl_light
from generate_version_range import apply_patches
from middle_range_source import displaylists
from old_shaded_wire import appearance
from generate_component_vectors import integers


def string_reader(original: object, version: int, role: str, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, *args: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,*args)
        finally:archive.cstring = saved
        start,end = spans[-1]
        if role=="rv":
            if version<=0x1415:patches.append((start-8,start,b""))
            if version<=0x1421:patches.append((start,end,b""))
            if version<=0x1462:patches.append((end,end+4,b""))
        if role=="node" and version<=0x143a:patches.append((start-4,start,b""))
        if role=="face" and version<0x1401:patches.append((start-8,start-4,b""))
        return result
    return read


def visual_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        if version<=0x144b:patches.append((archive.p-16,archive.p-12,b""))
        return original(archive)
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    bindings = [(dl_light,"read_uoRVAppearanceProperties"),(dl_light,"read_moNode_LWData"),(dl_light,"read_uoDSPBRProperties_c")]
    originals = [getattr(module,name) for module,name in bindings]
    readers = [string_reader(originals[0],version,"rv",patches),string_reader(originals[1],version,"node",patches),visual_reader(originals[2],version,patches)]
    original_face = ObjectArchive.readers["uoTempFaceTessData_c"]
    ObjectArchive.readers["uoTempFaceTessData_c"] = string_reader(original_face,version,"face",patches)
    try:
        for (module,name),reader in zip(bindings,readers,strict=True):setattr(module,name,reader)
        return convert_historical(document,path,raw,version,edited,patches)
    finally:
        ObjectArchive.readers["uoTempFaceTessData_c"] = original_face
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)


def rv_source_reader(original: object, raw: bytes, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        if "rv_app_props" in result:return result
        position = spans[-1][0]-4
        assert result["user_property"]==[] and raw[position:position+4]==integers(0)
        patches.append((position,position+4,integers(-1)+appearance()))
        return result
    return read


def rv_source(document: object, path: object, raw: bytes) -> bytes:
    original = dl_light.read_uoMaterialProperties
    patches: list[tuple[int,int,bytes]] = []
    dl_light.read_uoMaterialProperties = rv_source_reader(original,raw,patches)
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p==len(raw)
    finally:dl_light.read_uoMaterialProperties = original
    return apply_patches(raw,patches)
