"""Caller-scoped collector removal and complete earlier configuration suffixes."""
import struct
from generate_component_vectors import integers
from generate_named_vectors import cstring
from lower_range_patches import convert as convert_lower
import dl_tess


def role_reader(original: object, name: str, active: list[str]) -> object:
    def read(archive: object) -> object:
        saved = active[0]
        active[0] = name
        try:
            return original(archive)
        finally:
            active[0] = saved
    return read


def collector_reader(original: object, version: int, patches: list[tuple[int,int,bytes]], active: list[str]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        assert active[0] in ("part","viewer"), active[0]
        minimum = 0x2f5d if active[0]=="part" else 0x2f44
        if version<=minimum:
            patches.append((start,archive.p,b""))
        return result
    return read


def plane_reader(original: object, version: int, patches: list[tuple[int,int,bytes]], active: list[str]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        minimum = 0x2f5d if active[0]=="part" else 0x2f44
        if minimum<version<=0x2f66:
            patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    active = [""]
    bindings = [(dl_tess,"read_uoTempPartTessData"),(dl_tess,"read_moViewerComponentData"),(dl_tess,"read_moRefGeomFeatureDataCollector"),(dl_tess,"_rg_plane")]
    originals = [getattr(module,name) for module,name in bindings]
    readers = [role_reader(originals[0],"part",active),role_reader(originals[1],"viewer",active),collector_reader(originals[2],version,patches,active),plane_reader(originals[3],version,patches,active)]
    original_plane = dl_tess.REF_GEOM_TYPES[3]
    dl_tess.REF_GEOM_TYPES[3] = (original_plane[0],readers[3])
    try:
        for (module,name),reader in zip(bindings,readers,strict=True):setattr(module,name,reader)
        return convert_lower(document,path,raw,version,edited,patches)
    finally:
        dl_tess.REF_GEOM_TYPES[3] = original_plane
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)


def configurations(version: int, kind: str) -> bytes:
    raw = integers(1)+cstring("active 鋼",True)+integers(-1)+cstring("cfg\0",True)
    raw += integers(-2)+bytes(2)+(bytes(2) if kind=="assembly" else b"")
    raw += integers(-3)+bytes(2)+integers(-4)+bytes(2)
    raw += integers(1)+cstring("view 鋼",True)+cstring("cfg\0",True)
    if version>0x2fa5:
        raw += struct.pack("<II",0,1)+cstring("tail",True)+struct.pack("<II",0,2)+integers(-7,8)
    return raw
