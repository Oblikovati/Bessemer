"""Early grouped-component, scene tooltip and appearance state members."""
from lower_range_patches import string_capture
from component_transition_patches import convert as convert_component
import dl_tess
import dl_light


def viewer_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        if version<=0x23e4:
            end = spans[-7][1]
            patches.append((end+24,end+32,b""))
        return result
    return read


def node_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        if version<=0x23cd:patches.append((*spans[-1],b""))
        return result
    return read


def appearance_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        if version<=0x2419:patches.append((archive.p-12,archive.p-8,b""))
        return original(archive)
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    bindings = [(dl_tess,"read_moViewerComponentData"),(dl_light,"read_moNode_LWData"),(dl_light,"read_uoDSPBRProperties_c")]
    originals = [getattr(module,name) for module,name in bindings]
    readers = [viewer_reader(originals[0],version,patches),node_reader(originals[1],version,patches),appearance_reader(originals[2],version,patches)]
    try:
        for (module,name),reader in zip(bindings,readers,strict=True):setattr(module,name,reader)
        return convert_component(document,path,raw,version,edited,patches)
    finally:
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)
