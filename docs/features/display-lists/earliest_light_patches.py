"""Reference-observed old-light, appearance and assembly omissions in 4500-range roots."""
from earliest_face_patches import convert as convert_faces
from lower_range_patches import string_capture
from mfcobjects import ObjectArchive
from generate_component_vectors import integers
import dl_light
import dl_doc


def base_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, *args: object, **kwargs: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,*args,**kwargs)
        finally:archive.cstring = saved
        end = spans[1][1]
        if version<=0x11a3:patches.append((end+8,end+32,b""))
        if version<=0x11f5:patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def derived_reader(original: object, version: int, width: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        if version<0x11a4:patches.append((archive.p-width,archive.p,b""))
        return result
    return read


def appearance_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        start = spans[-1][0]
        if version<=0x119e:patches.append((start-20,start-12,b""))
        if version<=0x11ac:patches.append((start-12,start-8,b""))
        return result
    return read


def scene_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        if version<=0x1194:patches.append((archive.p-32,archive.p-8,b""))
        return result
    return read


def assembly_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        width = 4+4*len(result["hidden_body_atoms"])
        assert archive.b[archive.p-width:archive.p]==integers(len(result["hidden_body_atoms"]),*result["hidden_body_atoms"])
        if version<=0x11af:patches.append((archive.p-width,archive.p,b""))
        return result
    return read


def plane_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, drawing: bool) -> object:
        result = original(archive,drawing)
        if version<=0x11ed:patches.append((archive.p,archive.p+4,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    functions = [(dl_light,"read_moLightFeature_c",base_reader),(dl_light,"read_uoRVAppearanceProperties",appearance_reader),(dl_doc,"read_TexturedPlanes",plane_reader)]
    originals = [getattr(module,name) for module,name,_ in functions]
    widths = {"moDirectionLight_c":8,"moSpotLight_c":36,"moPointLight_c":28}
    classes = {name:ObjectArchive.readers[name] for name in [*widths,"pwSceneData_c","uoTempAssemblySHDData_c","uoTempSubAssemblySHDData_c"]}
    for (module,name,wrapper),original in zip(functions,originals,strict=True):setattr(module,name,wrapper(original,version,patches))
    for name,width in widths.items():ObjectArchive.readers[name] = derived_reader(classes[name],version,width,patches)
    ObjectArchive.readers["pwSceneData_c"] = scene_reader(classes["pwSceneData_c"],version,patches)
    for name in ["uoTempAssemblySHDData_c","uoTempSubAssemblySHDData_c"]:ObjectArchive.readers[name] = assembly_reader(classes[name],version,patches)
    try:return convert_faces(document,path,raw,version,edited,patches)
    finally:
        for (module,name,_),original in zip(functions,originals,strict=True):setattr(module,name,original)
        ObjectArchive.readers.update(classes)
