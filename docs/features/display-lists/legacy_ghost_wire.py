"""Complete ghost items, hidden words and part timestamps with old facet layouts."""
import struct
from pathlib import Path
from itertools import product
from legacy_ui_wire import early_ui_root
from legacy_text_reference import early_text_reference
from legacy_font_wire import list_reader
from legacy_boundary_wire import facet_reader,boundary_facet
from generate_transform_vectors import doubles
from generate_component_vectors import integers
from generate_item_vectors import record
from generate_version_range import apply_patches
import dl_ui
import dl_tess

VERSIONS = [1215,1216,1235,1236,1253,1254,1270,1271,1275,1276,1286]


def ghost_item_list(edited: bool, empty: bool) -> bytes:
    if empty:return bytes(2)
    payload = doubles(3)+integers(-1+int(edited),2)
    return struct.pack("<H",1)+record(1,payload)


def ghost_bucket_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, obj_keys: int) -> object:
        saved,spans = dl_ui.read_suDynList_uoDisplayItem_c,[]
        dl_ui.read_suDynList_uoDisplayItem_c = list_reader(saved,spans)
        try:result = original(archive,flag,obj_keys)
        finally:dl_ui.read_suDynList_uoDisplayItem_c = saved
        matches = [(first,last) for first,last,value in spans if value is result["dl_ghost_bucket"]]
        assert len(matches)==1 and result["dl_ghost_bucket"]==[],matches
        first,last = matches[0]
        payload = ghost_item_list(edited,not result["silh"]) if version>0x4bf else b""
        patches.append((first,last,payload))
        if version<0x4d4:
            assert archive.b[archive.p-4:archive.p]==struct.pack("<i",result["show_hidden_count"])
            patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def ghost_part_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        assert archive.b[archive.p-4:archive.p]==struct.pack("<I",result["ddata_time_stamp"]&0xffffffff)
        patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def early_ghost_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = early_ui_root(1286,kind,scenario)
    saved = dl_ui.read_gcBucket_c,dl_tess.read_uoTempPartTessData,dl_tess.facet_container
    patches,edited = [],scenario==4 or scenario%16==1
    dl_ui.read_gcBucket_c = ghost_bucket_reader(saved[0],version,edited,patches)
    if version<0x4f7:dl_tess.read_uoTempPartTessData = ghost_part_reader(saved[1],patches)
    dl_tess.facet_container = facet_reader(saved[2],version,edited,patches)
    try:early_text_reference(raw,1286,kind,identities)
    finally:dl_ui.read_gcBucket_c,dl_tess.read_uoTempPartTessData,dl_tess.facet_container = saved
    return apply_patches(raw,patches),identities


def write_early_ghost_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version,enabled,edited in product(VERSIONS,(False,True),(False,True)):
        name = f"legacy-ghost-facet-{version}-{int(enabled)}-{int(edited)}.bin"
        (root/name).write_bytes(boundary_facet(version,enabled,edited))
