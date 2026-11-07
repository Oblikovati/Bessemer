"""Reference-observed omission of early part, assembly and component state words."""
from earliest_wall_patches import convert as convert_walls
from lower_range_patches import string_capture
from mfcobjects import ObjectArchive
import dl_tess


def part_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        if version<0x10f0:patches.append((spans[-1][0]-12,spans[-1][0]-8,b""))
        return result
    return read


def component_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        if version<=0x110d:patches.append((spans[4][0]-4,spans[4][0],b""))
        return result
    return read


def assembly_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        width = 4+4*len(result["hidden_body_atoms"])
        if version<0x10f0:patches.append((archive.p-width-4,archive.p-width,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    bindings = [("read_uoTempPartTessData",part_reader),("read_moViewerComponentData",component_reader)]
    originals = [getattr(dl_tess,name) for name,_ in bindings]
    names = ["uoTempAssemblySHDData_c","uoTempSubAssemblySHDData_c"]
    classes = {name:ObjectArchive.readers[name] for name in names}
    for (name,wrapper),original in zip(bindings,originals,strict=True):setattr(dl_tess,name,wrapper(original,version,patches))
    for name in names:ObjectArchive.readers[name] = assembly_reader(classes[name],version,patches)
    try:return convert_walls(document,path,raw,version,edited,patches)
    finally:
        for (name,_),original in zip(bindings,originals,strict=True):setattr(dl_tess,name,original)
        ObjectArchive.readers.update(classes)
