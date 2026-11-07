"""Complete old font and solid-fill layouts with independently built legacy facets."""
import struct
from pathlib import Path
from legacy_boundary_wire import boundary_root,boundary_facet,facet_reader
from legacy_edge_reference import edge_reference
from legacy_edge_roots import span_reader
from generate_version_range import apply_patches
from generate_transform_vectors import doubles
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_item_vectors import record
import dl_ui
import dl_doc
import dl_tess

VERSIONS = [2011,2012,2015,2016,2032,2033,2078]


def font_item(version: int, kind: int, wide: bool, edited: bool) -> bytes:
    name = ("B" if edited else "A")+("字😀" if wide else "font")
    raw = doubles(1)+integers(-2)+doubles(2)+struct.pack("<I",0xfedcba98)+cstring(name,wide)
    raw += struct.pack("<d",2.0 if edited else 1.0)
    if version>0x7df:raw += struct.pack("<Q",0x7ff8000000000042 if edited else 0x8000000000000000)
    return record(kind,raw)


def list_reader(original: object, spans: list[tuple[int,int,object]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        spans.append((first,archive.p,result))
        return result
    return read


def bucket_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, obj_keys: int) -> object:
        saved,spans = dl_ui._read_poly_list,[]
        dl_ui._read_poly_list = list_reader(saved,spans)
        try:result = original(archive,flag,obj_keys)
        finally:dl_ui._read_poly_list = saved
        matches = [(first,last,b"") for first,last,value in spans if value is result["solid_fill"]]
        assert len(matches)==1,matches
        patches.extend(matches)
        return result
    return read


def observed_font_root(raw: bytes, identities: tuple[str,...], version: int, kind: str, scenario: int) -> bytes:
    originals = dl_ui.read_gcBucket_c,dl_tess.facet_container,dl_doc.read_display_item_list
    patches,items = [],[]
    if version<=0x7f0:dl_ui.read_gcBucket_c = bucket_reader(originals[0],patches)
    dl_tess.facet_container = facet_reader(originals[1],version,scenario%16==1,patches)
    dl_doc.read_display_item_list = span_reader(originals[2],items)
    try:edge_reference(raw,2078,kind,identities)
    finally:dl_ui.read_gcBucket_c,dl_tess.facet_container,dl_doc.read_display_item_list = originals
    first,last = items[-1]
    count, = struct.unpack_from("<H",raw,first)
    edited = scenario==4 or scenario%16==1
    extra = b"".join(font_item(version,t,w,edited) for t in (10,14) for w in (False,True))
    patches.append((first,last,struct.pack("<H",count+4)+raw[first+2:last]+extra))
    return apply_patches(raw,patches)


def font_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = boundary_root(2078,kind,scenario)
    return observed_font_root(raw,identities,version,kind,scenario),identities


def write_font_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version in VERSIONS:
        for enabled in (False,True):
            for edited in (False,True):
                name = f"legacy-font-facet-{version}-{int(enabled)}-{int(edited)}.bin"
                (root/name).write_bytes(boundary_facet(version,enabled,edited))
