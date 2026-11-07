"""Independent populated dependency lists and duplicate equation keys in complete old components."""
from lower_range_patches import string_capture
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_version_range import apply_patches
from middle_range_source import displaylists
import dl_tess


def dependencies() -> bytes:
    return integers(2)+cstring("first",True)+cstring("second 鋼",True)


def equations() -> bytes:
    return integers(2)+cstring("key",True)+integers(2)+cstring("x",True)+cstring("y",True)+cstring("key",True)+integers(0)


def component_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        assert result.get("depended_comp_names",[])==[] and result.get("equ_dep_comp_names",[])==[]
        start = spans[6][1]
        if version>0x12df:patches.append((start,start+4,dependencies()))
        if version>0x12e2:patches.append((start+4,start+8,equations()))
        return result
    return read


def populate(document: object, path: object, raw: bytes, version: int) -> bytes:
    patches = []
    original = dl_tess.read_moViewerComponentData
    dl_tess.read_moViewerComponentData = component_reader(original,version,patches)
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        archive.version = version
        displaylists.display_lists(archive,"assembly")
        assert archive.p==len(raw)
    finally:dl_tess.read_moViewerComponentData = original
    return apply_patches(raw,patches)
