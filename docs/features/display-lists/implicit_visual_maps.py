"""Encode single visual pointers without changing the embedded facet archive's real version."""
import struct
from contextlib import contextmanager
from collections.abc import Iterator
from generate_alias_fixture import collect
from generate_component_vectors import integers
from generate_version_range import apply_patches
from earliest_body_wire import direct_properties,relocated_after
from mfcobjects import ObjectArchive
import dl_tess


def observing_reader(original: object, floor: int) -> object:
    def read(archive: object, *args: object) -> object:
        previous = dl_tess._v
        def layout_version(current: object) -> int:
            return max(previous(current),floor)
        dl_tess._v = layout_version
        try:return original(archive,*args)
        finally:dl_tess._v = previous
    return read


def map_reader(original: object, spans: list[tuple[int,int]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        assert result==[] and archive.p==start+4
        spans.append((start,archive.p))
        return result
    return read


@contextmanager
def observed_layout(spans: list[tuple[int,int]]) -> Iterator[None]:
    names = {"uoTempFaceTessData_c":0x10af,"uoTempBodyTessData_c":0x10f6,
             "uoTempAssemblySHDData_c":0xfd8,"uoTempSubAssemblySHDData_c":0xfd8}
    classes = {name:ObjectArchive.readers[name] for name in names}
    original_part,original_map = dl_tess.read_uoTempPartTessData,dl_tess._viz_prop_map
    for name,floor in names.items():ObjectArchive.readers[name] = observing_reader(classes[name],floor)
    dl_tess.read_uoTempPartTessData = observing_reader(original_part,0x10af)
    dl_tess._viz_prop_map = map_reader(original_map,spans)
    try:yield
    finally:
        ObjectArchive.readers.update(classes)
        dl_tess.read_uoTempPartTessData,dl_tess._viz_prop_map = original_part,original_map


def single_properties(document: object, path: object, raw: bytes, version: int, null_only: bool) -> bytes:
    if version>0x10ae:return direct_properties(document,path,raw,version,null_only)
    spans = []
    with observed_layout(spans):records = collect(document,path,raw,version,set())
    assert len(spans)==3
    targets = [record for record in records if record[3]=="uoBodyPropInfo_c"]
    assert len(targets)==1
    start,end,identity,_ = targets[0]
    visuals = {record[2] for record in records if record[3]=="moVisualProperties_c"}
    assert len(visuals)==1 and raw[start:start+2]==b"\xff\xff" and raw[start-8:start]==integers(1,-1)
    visual = next(iter(visuals))
    assert visual<identity-1
    pointer = struct.pack("<H",0 if null_only else visual)
    patches = [(first,last,pointer) for first,last in spans]+[(start-8,end,pointer)]
    patches.extend(relocated_after(raw,records,end,identity))
    return apply_patches(raw,patches)
