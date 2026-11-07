"""Observe component metadata spans before removing fields absent in 4800-range roots."""
from earliest_scene_patches import convert as convert_scene
from lower_range_patches import string_capture
from generate_component_vectors import integers
import dl_tess


def component_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive)
        finally:archive.cstring = saved
        assert result["depended_comp_names"]==[] and result["equ_dep_comp_names"]==[]
        start,end = spans[4][0],spans[7][1]
        assert archive.b[end:end+12]==integers(0,0,0)
        omit_words(version,start,end,patches)
        if version<=0x12c3:patches.append((spans[4][0],spans[6][1],b""))
        return result
    return read


def omit_words(version: int, reference_start: int, dependency_start: int, patches: list[tuple[int,int,bytes]]) -> None:
    for threshold,start in [(0x12d0,reference_start-36),(0x12d3,reference_start-32),
                            (0x12df,dependency_start),(0x12e2,dependency_start+4),
                            (0x12e8,dependency_start+8)]:
        if version<=threshold:patches.append((start,start+4,b""))


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original = dl_tess.read_moViewerComponentData
    dl_tess.read_moViewerComponentData = component_reader(original,version,patches)
    try:return convert_scene(document,path,raw,version,edited,patches)
    finally:dl_tess.read_moViewerComponentData = original
