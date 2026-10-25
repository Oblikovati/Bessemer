"""Remove old appearance scalars and component display-state words from observed spans."""
from earliest_body_patches import convert as convert_bodies
from lower_range_patches import string_capture
import dl_light
import dl_tess


def appearance_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        start = spans[0][1]
        if version<=0x10a1:patches.append((start+53,start+57,b""))
        if version<=0x10bf:patches.append((start+57,start+73,b""))
        return result
    return read


def component_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        if version<=0x10ae:patches.append((spans[4][0]-8,spans[4][0]-4,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    bindings = [(dl_light,"read_uoRVAppearanceProperties",appearance_reader),(dl_tess,"read_moViewerComponentData",component_reader)]
    originals = [getattr(module,name) for module,name,_ in bindings]
    for (module,name,wrapper),original in zip(bindings,originals,strict=True):setattr(module,name,wrapper(original,version,patches))
    try:return convert_bodies(document,path,raw,version,edited,patches)
    finally:
        for (module,name,_),original in zip(bindings,originals,strict=True):setattr(module,name,original)
