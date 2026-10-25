"""Insert observed legacy arrays without changing MFC class or object slots."""
from legacy_visibility_wire import visibility_root
from legacy_region_reference import region_reference
from legacy_edge_wire import edge_array,view_triples
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_ui


def span_reader(original: object, spans: list[tuple[int,int]]) -> object:
    def read(archive: object, *args: object) -> object:
        first = archive.p
        result = original(archive,*args)
        spans.append((first,archive.p))
        return result
    return read


def bucket_patches(result: dict[str,object], spans: dict[str,list[tuple[int,int]]], version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> None:
    assert len(spans["items"])==2 and len(spans["maps"])==1,spans
    if version>=0xa5a:return
    position = spans["items"][0][0]
    dimensions = 2 if result["has_2d_info"] else 3
    raw = edge_array(version,dimensions,not result["silh"],edited)
    patches.append((position,position,raw))
    first,last = spans["maps"][0]
    patches.append((first,last,b""))


def bucket_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, obj_keys: int) -> dict[str,object]:
        saved = dl_ui.read_suDynList_uoDisplayItem_c,dl_ui._mfc_map
        spans = {"items":[],"maps":[]}
        dl_ui.read_suDynList_uoDisplayItem_c = span_reader(saved[0],spans["items"])
        dl_ui._mfc_map = span_reader(saved[1],spans["maps"])
        try:result = original(archive,flag,obj_keys)
        finally:dl_ui.read_suDynList_uoDisplayItem_c,dl_ui._mfc_map = saved
        bucket_patches(result,spans,version,edited,patches)
        return result
    return read


def view_reader(original: object, version: int, edited: bool, insertions: dict[int,bytes]) -> object:
    def read(archive: object, schema: int) -> dict[str,object]:
        result = original(archive,schema)
        if 0xa42<version<0xa5a:
            raw = view_triples(not result["bucket"]["silh"],edited)
            insertions[archive.p] = insertions.get(archive.p,b"")+raw
        return result
    return read


def edge_root(version: int, kind: str, scenario: int, cached: bool = False) -> tuple[bytes,tuple[str,...]]:
    assert 2606<=version<=2650,f"edge fixture version {version}; expected 2606..2650"
    raw,identities = visibility_root(2650,kind,scenario,cached)
    saved = dl_ui.read_gcBucket_c,ObjectArchive.readers["uiViewBucket_c"]
    patches,insertions = [],{}
    edited = scenario==4 or scenario%16==1
    dl_ui.read_gcBucket_c = bucket_reader(saved[0],version,edited,patches)
    ObjectArchive.readers["uiViewBucket_c"] = view_reader(saved[1],version,edited,insertions)
    try:region_reference(raw,2650,kind,identities)
    finally:dl_ui.read_gcBucket_c,ObjectArchive.readers["uiViewBucket_c"] = saved
    patches += [(position,position,payload) for position,payload in insertions.items()]
    return apply_patches(raw,patches),identities
