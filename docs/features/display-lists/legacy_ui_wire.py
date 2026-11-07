"""Complete early UI, font, sketch-text and body layouts with versioned facets."""
import struct
from itertools import product
from pathlib import Path
from legacy_text_wire import early_text_root
from legacy_text_reference import early_text_reference
from legacy_font_wire import list_reader
from legacy_boundary_wire import facet_reader,boundary_facet
from legacy_edge_roots import span_reader
from generate_transform_vectors import doubles,compact
from generate_named_vectors import cstring
from generate_item_vectors import record
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_doc
import dl_ui
import dl_tess

VERSIONS = [1286,1287,1289,1290,1300,1301,1325,1326,1327,1331,1332,1645]


def ui_font_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        assert archive.b[archive.p-8:archive.p]==struct.pack("<d",result["unknown_28"])
        patches.append((archive.p-8,archive.p,b""))
        return result
    return read


def ui_environment_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = dl_ui.read_uiSplitterInfo_c,[]
        dl_ui.read_uiSplitterInfo_c = list_reader(saved,spans)
        try:result = original(archive,schema)
        finally:dl_ui.read_uiSplitterInfo_c = saved
        matches = [(first,last,b"") for first,last,value in spans if value is result["splitter_e0"]]
        assert len(matches)==1,matches
        patches.extend(matches)
        return result
    return read


def ui_body_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert archive.b[archive.p-4:archive.p]==struct.pack("<i",result["unknown_20"])
        patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def skip_ui_transform(raw: bytes, position: int, transforms: list[tuple[int,int]]) -> int:
    present, = struct.unpack_from("<i",raw,position)
    first = position+4
    if not present:return first
    return next(last for start,last in transforms if start==first)


def ui_view_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved = dl_ui.read_gcBucket_c,dl_ui.read_mgXform_c
        buckets,transforms = [],[]
        dl_ui.read_gcBucket_c = span_reader(saved[0],buckets)
        dl_ui.read_mgXform_c = span_reader(saved[1],transforms)
        try:result = original(archive,schema)
        finally:dl_ui.read_gcBucket_c,dl_ui.read_mgXform_c = saved
        first = skip_ui_transform(archive.b,buckets[0][1],transforms)
        last = skip_ui_transform(archive.b,first,transforms)
        present, = struct.unpack_from("<i",archive.b,first)
        assert bool(present)==("ghost_dl_xform" in result)
        patches.append((first,last,b""))
        return result
    return read


def sketch_ui_item(version: int, enabled: bool, edited: bool) -> bytes:
    transform = compact(2 if enabled else 0,0x10)
    text = ("B" if edited else "A")+("図😀" if enabled else "sketch")
    raw = doubles(3)+transform
    if version>0x52d:raw += struct.pack("<d",2.0 if edited else 1.0)
    return record(8,raw+cstring(text,enabled))


def snapshot_ui_readers() -> dict[str,object]:
    return {"font":dl_doc.ITEM_TYPES[10],"alias":dl_doc.ITEM_TYPES[14],
        "environment":ObjectArchive.readers["uiUserModelEnv_c"],
        "body":ObjectArchive.readers["uoTempBodyTessData_c"],
        "view":ObjectArchive.readers["uiViewBucket_c"],
        "facet":dl_tess.facet_container,"items":dl_doc.read_display_item_list}


def install_ui_readers(saved: dict[str,object], version: int, edited: bool, patches: list[tuple[int,int,bytes]], items: list[tuple[int,int]]) -> None:
    if version<=0x52e:
        dl_doc.ITEM_TYPES[10] = (saved["font"][0],ui_font_reader(saved["font"][1],patches))
        dl_doc.ITEM_TYPES[14] = (saved["alias"][0],ui_font_reader(saved["alias"][1],patches))
    if version<0x515:ObjectArchive.readers["uiUserModelEnv_c"] = ui_environment_reader(saved["environment"],patches)
    if version<=0x506:ObjectArchive.readers["uoTempBodyTessData_c"] = ui_body_reader(saved["body"],patches)
    if version<=0x509:ObjectArchive.readers["uiViewBucket_c"] = ui_view_reader(saved["view"],patches)
    dl_tess.facet_container = facet_reader(saved["facet"],version,edited,patches)
    dl_doc.read_display_item_list = span_reader(saved["items"],items)


def restore_ui_readers(saved: dict[str,object]) -> None:
    dl_doc.ITEM_TYPES[10],dl_doc.ITEM_TYPES[14] = saved["font"],saved["alias"]
    ObjectArchive.readers["uiUserModelEnv_c"],ObjectArchive.readers["uoTempBodyTessData_c"],ObjectArchive.readers["uiViewBucket_c"] = saved["environment"],saved["body"],saved["view"]
    dl_tess.facet_container,dl_doc.read_display_item_list = saved["facet"],saved["items"]


def early_ui_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = early_text_root(1645,kind,scenario)
    saved,patches,items = snapshot_ui_readers(),[],[]
    edited = scenario==4 or scenario%16==1
    install_ui_readers(saved,version,edited,patches,items)
    try:early_text_reference(raw,1645,kind,identities)
    finally:restore_ui_readers(saved)
    first,last = items[-1]
    count, = struct.unpack_from("<H",raw,first)
    extra = b"".join(sketch_ui_item(version,enabled,edited) for enabled in (False,True))
    nested = [(start-first,end-first,payload) for start,end,payload in patches if first<=start<=end<=last]
    patches = [(start,end,payload) for start,end,payload in patches if not first<=start<=end<=last]
    replaced = apply_patches(raw[first:last],nested)
    patches.append((first,last,struct.pack("<H",count+2)+replaced[2:]+extra))
    return apply_patches(raw,patches),identities


def write_early_ui_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version,enabled,edited in product(VERSIONS,(False,True),(False,True)):
        name = f"legacy-ui-facet-{version}-{int(enabled)}-{int(edited)}.bin"
        (root/name).write_bytes(boundary_facet(version,enabled,edited))
