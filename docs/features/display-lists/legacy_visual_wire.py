"""Observe appearance tail fields before their two independent historical gates."""
import struct
from legacy_camera_wire import camera_root
from generate_legacy_panes import physical_reference
from lower_range_patches import string_capture
from generate_version_range import apply_patches
from generate_named_vectors import cstring
from mfcobjects import ObjectArchive


def visual_tail(archive: object, result: dict[str,object], spans: list[tuple[int,int]], version: int, edited: bool) -> list[tuple[int,int,bytes]]:
    first,last = spans[-1]
    assert last==archive.p and result["appearance_name"]==""
    assert archive.b[first-4:first]==struct.pack("<i",result["use_in_sw_lw_rendering"])
    word = struct.pack("<i",-2147483648+int(edited)) if version>0xb53 else b""
    name = cstring("appearance 鋼"+(" edited" if edited else ""),True) if version>0xb55 else b""
    return [(first-4,first,word),(first,last,name)]


def visual_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:archive.cstring = saved
        patches.extend(visual_tail(archive,result,spans,version,edited))
        return result
    return read


def visual_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = camera_root(version,kind,scenario)
    original,patches = ObjectArchive.readers["moVisualProperties_c"],[]
    ObjectArchive.readers["moVisualProperties_c"] = visual_reader(original,version,scenario in (1,4),patches)
    try:physical_reference(raw,2902,kind,identities)
    finally:ObjectArchive.readers["moVisualProperties_c"] = original
    assert patches or scenario==2,"visual spans were empty; expected shaded root appearance declarations"
    return apply_patches(raw,patches),identities
