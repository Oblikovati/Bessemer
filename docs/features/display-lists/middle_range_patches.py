"""Reference-guided byte replacements at documented middle-range version gates."""
import struct
from generate_version_range import capture_plain,capture_rv,apply_patches
from generate_spatial_vectors import collection
from version_range_wire import dspbr
from old_shaded_wire import facet_wire
import dl_doc
import dl_light
import dl_tess


def dspbr_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        patches.append((start-4 if version<=0x3f08 else start,archive.p,b"" if version<=0x3f08 else dspbr(version)))
        return result
    return read


def scene_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, kind: int) -> object:
        result = original(archive,kind)
        patches.append((archive.p-(20 if version<=0x42ec else 8),archive.p,b""))
        return result
    return read


def viewer_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        missing = 5 if version<=0x42e7 else int(version<0x42f3)
        if missing:
            patches.append((archive.p-missing,archive.p,b""))
        return result
    return read


def ambient_capture(original: object, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    first = [True]
    def read(archive: object) -> object:
        start = archive.p
        archive.cstring()
        archive.cstring()
        position = archive.p
        archive.p = start
        result = original(archive)
        if first[0] and edited:
            patches.append((position,position+8,struct.pack("<d",23.0)))
        first[0] = False
        return result
    return read


def envelope_capture(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        if version<=0x42da:
            patches.append((start,archive.p,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    bindings = [(dl_light,"read_uoDSPBRProperties_c"),(dl_light,"read_uoRVAppearanceProperties"),(dl_light,"read_scene"),(dl_doc,"read_3DViewTransform"),(dl_tess,"read_moViewerComponentData"),(dl_doc,"read_EnvelopeSystemOptions"),(dl_tess,"facet_container"),(dl_light,"read_uoMaterialProperties")]
    originals = [getattr(module,name) for module,name in bindings]
    replacements = [dspbr_capture(originals[0],version,patches),capture_rv(originals[1],version,patches),scene_capture(originals[2],version,patches),capture_plain(originals[3],collection(version,2),patches),viewer_capture(originals[4],version,patches),envelope_capture(originals[5],version,patches),capture_plain(originals[6],facet_wire(version,-17,False,True),patches),ambient_capture(originals[7],edited,patches)]
    from generate_alias_fixture import displaylists
    try:
        for (module,name),reader in zip(bindings,replacements,strict=True):setattr(module,name,reader)
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p==len(raw)
    finally:
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)
    return apply_patches(raw,patches)
