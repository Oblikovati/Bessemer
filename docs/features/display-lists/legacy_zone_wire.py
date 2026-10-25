"""Complete roots across zone, exploded-bucket and textured-plane list gates."""
import struct
import hashlib
import json
from pathlib import Path
from generate_blank_scalars import flatten
from legacy_font_wire import font_root,list_reader
from legacy_edge_roots import span_reader
from legacy_edge_reference import edge_reference
from generate_transform_vectors import compact
from generate_version_range import apply_patches
import dl_ui
import dl_doc
import dl_tess

VERSIONS = [1875,1876,1896,1897,1899,1900,2011]


def exploded_bucket_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, obj_keys: int) -> object:
        saved,spans = dl_ui._read_poly_list,[]
        dl_ui._read_poly_list = list_reader(saved,spans)
        try:result = original(archive,flag,obj_keys)
        finally:dl_ui._read_poly_list = saved
        omitted = {id(result[name]) for name in ("explode_v","explode_h")}
        matches = [(first,last,b"") for first,last,value in spans if id(value) in omitted]
        assert len(matches)==2,matches
        patches.extend(matches)
        return result
    return read


def zone_component_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        assert archive.b[archive.p-4:archive.p]==struct.pack("<i",result["is_a_zone"])
        patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def zone_planes(edited: bool) -> bytes:
    first = compact(2,0x40)
    if edited:first = first[:73]+struct.pack("<d",7.0)+first[81:]
    return struct.pack("<i",2)+bytes(2)+first+bytes(2)+compact(0,0x10)


def zone_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = font_root(2011,kind,scenario)
    originals = dl_ui.read_gcBucket_c,dl_tess.read_moViewerComponentData,dl_doc.read_TexturedPlanes
    patches,planes = [],[]
    if version<=0x753:dl_ui.read_gcBucket_c = exploded_bucket_reader(originals[0],patches)
    if version<=0x768:dl_tess.read_moViewerComponentData = zone_component_reader(originals[1],patches)
    dl_doc.read_TexturedPlanes = span_reader(originals[2],planes)
    try:edge_reference(raw,2011,kind,identities)
    finally:dl_ui.read_gcBucket_c,dl_tess.read_moViewerComponentData,dl_doc.read_TexturedPlanes = originals
    for first,last in planes:
        assert raw[first:last]==bytes(4)
        payload = zone_planes(scenario==4 or scenario%16==1) if version>0x76b else b""
        patches.append((first,last,payload))
    return apply_patches(raw,patches),identities


def zone_plane_trace(root: dict[str,object]) -> bytes:
    rows = {}
    for index,plane in enumerate(root.get("textured_planes",[])):
        assert plane["sketch_bitmap"] is None
        flatten(f"root.textured_planes.items[{index}].xform",plane["xform"],rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def write_zone_plane_traces(cases: list[tuple[bytes,dict[str,object]]]) -> None:
    framed,fields = b"",0
    for case,row in cases:
        length, = struct.unpack_from("<I",case,6)
        raw = case[10:10+length]
        root = edge_reference(raw,row["version"],row["kind"],tuple(row.get("external_ids",[])))
        trace = zone_plane_trace(root)
        fields += trace.count(b"\n")
        framed += struct.pack("<I",len(trace))+trace
    Path("tests/fixtures/display-lists/legacy-zone-plane-traces.bin").write_bytes(framed)
    report = {"fixtures":len(cases),"independent_plane_fields":fields,"sha256":hashlib.sha256(framed).hexdigest()}
    Path(__file__).with_name("legacy-zone-plane-traces.json").write_text(json.dumps(report,indent=2)+"\n")
