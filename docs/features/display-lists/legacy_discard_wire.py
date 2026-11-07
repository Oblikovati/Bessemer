"""Complete old buckets around the seven-version discarded edge-word interval."""
import struct
from legacy_zone_wire import zone_root
from legacy_font_wire import list_reader
from legacy_edge_reference import edge_reference
from generate_version_range import apply_patches
import dl_ui

VERSIONS = [1857,1858,1859,1860,1866,1867,1875]


def silhouette_bucket_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, obj_keys: int) -> object:
        saved,spans = dl_ui._read_poly_list,[]
        dl_ui._read_poly_list = list_reader(saved,spans)
        try:result = original(archive,flag,obj_keys)
        finally:dl_ui._read_poly_list = saved
        matches = [(first,last,b"") for first,last,value in spans if value is result["silh_h"]]
        assert len(matches)==1,matches
        patches.extend(matches)
        return result
    return read


def interval_edge_reader(original: object, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        assert struct.unpack_from("<i",archive.b,first+16)[0]==result["unknown_c"]
        value = 2147483647-int(edited)
        patches.append((first+20,first+20,struct.pack("<i",value)))
        return result
    return read


def discard_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = zone_root(1875,kind,scenario)
    originals = dl_ui.read_gcBucket_c,dl_ui.read_gcEdgeVisData_c
    patches = []
    if version<=0x741:dl_ui.read_gcBucket_c = silhouette_bucket_reader(originals[0],patches)
    if 0x744<=version<=0x74a:
        edited = scenario==4 or scenario%16==1
        dl_ui.read_gcEdgeVisData_c = interval_edge_reader(originals[1],edited,patches)
    try:edge_reference(raw,1875,kind,identities)
    finally:dl_ui.read_gcBucket_c,dl_ui.read_gcEdgeVisData_c = originals
    return apply_patches(raw,patches),identities
