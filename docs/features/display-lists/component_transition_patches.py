"""Versioned component state and the transient discarded viewer pointer."""
from lower_range_patches import string_capture
from precollector_patches import convert as convert_precollector
from mfcobjects import ObjectArchive
import dl_tess
import dl_light


def state_patches(version: int, spans: list[tuple[int,int]], patches: list[tuple[int,int,bytes]]) -> None:
    start,end = spans[-7]
    if version<=0x279c:patches.append((start,end,b""))
    fixed = end+32
    assert spans[-6][0]==fixed+18
    if version<=0x279d:patches.append((fixed,fixed+6,b""))
    if version<=0x27b1:patches.append((fixed+6,fixed+10,b""))
    if version<=0x27c1:patches.append((fixed+10,fixed+14,b""))
    if 0x27b6<version<0x27e3:patches.append((fixed+10,fixed+10,bytes(2)))


def viewer_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:
            result = original(archive)
        finally:
            archive.cstring = saved
        assert result["depended_comp_names"]==[] and result["equ_dep_comp_names"]==[]
        state_patches(version,spans,patches)
        return result
    return read


def body_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        if version<=0x2791:patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def appearance_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        if version<=0x2794:patches.append((archive.p-8,archive.p-4,b""))
        return original(archive)
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original_appearance = dl_light.read_uoDSPBRProperties_c
    dl_light.read_uoDSPBRProperties_c = appearance_reader(original_appearance,version,patches)
    original_viewer = dl_tess.read_moViewerComponentData
    original_body = ObjectArchive.readers["uoTempBodyTessData_c"]
    dl_tess.read_moViewerComponentData = viewer_reader(original_viewer,version,patches)
    ObjectArchive.readers["uoTempBodyTessData_c"] = body_reader(original_body,version,patches)
    try:
        return convert_precollector(document,path,raw,version,edited,patches)
    finally:
        dl_light.read_uoDSPBRProperties_c = original_appearance
        dl_tess.read_moViewerComponentData = original_viewer
        ObjectArchive.readers["uoTempBodyTessData_c"] = original_body
