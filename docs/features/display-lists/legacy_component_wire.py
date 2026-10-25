"""Construct complete early roots while preserving all MFC class and object slots."""
import struct
from legacy_model_viz_wire import early_model_viz_root,model_viz_object_reader
from legacy_text_reference import early_text_reference
from generate_version_range import apply_patches
from legacy_edge_roots import span_reader
from mfcobjects import ObjectArchive
import dl_tess
import dl_ui

VERSIONS = [903,904,915,916,930,931,989]


def early_polygon_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int = 0) -> object:
        result = original(archive,flag)
        if result["count"]<=0:return result
        length = 4+(4+8*len(result["type_data"]) if result["type"] in (1,2,3) else 0)
        first = archive.p-length
        assert struct.unpack_from("<i",archive.b,first)[0]==result["type"]
        patches.append((first,archive.p,b""))
        return result
    return read


def early_component_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = ObjectArchive.object,[]
        ObjectArchive.object = model_viz_object_reader(saved,spans)
        try:result = original(archive)
        finally:ObjectArchive.object = saved
        assert len(spans)==1 and spans[0][4] is result["comp_viz_prop"]
        assert len(archive.map)==spans[0][3]
        patches.append((spans[0][1],archive.p,b""))
        return result
    return read


def early_environment_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = dl_ui.read_uiSplitterInfo_c,[]
        dl_ui.read_uiSplitterInfo_c = span_reader(saved,spans)
        try:result = original(archive,schema)
        finally:dl_ui.read_uiSplitterInfo_c = saved
        assert len(spans)==1 and archive.p==spans[0][1]+8
        first = spans[0][1]
        assert struct.unpack_from("<ii",archive.b,first)==(result["unknown_1b0"],result["browser_view_width"])
        patches.append((first,archive.p,b""))
        return result
    return read


def early_component_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = early_model_viz_root(989,kind,scenario)
    saved = dl_tess.read_moViewerComponentData,ObjectArchive.readers["uiUserModelEnv_c"],dl_ui.read_gcBucketPoly_c
    patches = []
    if version<0x3a3:dl_tess.read_moViewerComponentData = early_component_reader(saved[0],patches)
    if version<=0x387:ObjectArchive.readers["uiUserModelEnv_c"] = early_environment_reader(saved[1],patches)
    if version<=0x393:dl_ui.read_gcBucketPoly_c = early_polygon_reader(saved[2],patches)
    try:early_text_reference(raw,989,kind,identities)
    finally:dl_tess.read_moViewerComponentData,ObjectArchive.readers["uiUserModelEnv_c"],dl_ui.read_gcBucketPoly_c = saved
    return apply_patches(raw,patches),identities
