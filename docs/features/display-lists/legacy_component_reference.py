"""Separate stored old view activity words from their native logical booleans."""
import struct
from legacy_text_reference import early_text_reference,node_scalars
from legacy_edge_roots import span_reader
from generate_blank_scalars import flatten
from mfcobjects import ObjectArchive
import dl_ui


def activity_reader(original: object) -> object:
    def read(archive: object, schema: int) -> object:
        saved = dl_ui.read_gcBucket_c,dl_ui.read_mgXform_c
        buckets,transforms = [],[]
        dl_ui.read_gcBucket_c = span_reader(saved[0],buckets)
        dl_ui.read_mgXform_c = span_reader(saved[1],transforms)
        try:result = original(archive,schema)
        finally:dl_ui.read_gcBucket_c,dl_ui.read_mgXform_c = saved
        # #42: newer ghost-transform framing changes offsets; its activity is already raw.
        if archive.version>=0x394:
            result["_stored_activity"] = result["flags"]
            return result
        result["_stored_activity"] = old_activity_word(archive,buckets,transforms,result)
        return result
    return read


def old_activity_word(archive: object, buckets: list[tuple[int,int]], transforms: list[tuple[int,int]], result: dict[str,object]) -> int:
    end = buckets[0][1]+4
    if "xform" in result:end = next(last for first,last in transforms if first==end)
    raw, = struct.unpack_from("<i",archive.b,end+24)
    assert result["flags"]==int(raw!=0)
    return raw


def component_reference(raw: bytes, version: int, kind: str, identities: tuple[str,...]) -> dict[str,object]:
    saved = ObjectArchive.readers["uiViewBucket_c"]
    ObjectArchive.readers["uiViewBucket_c"] = activity_reader(saved)
    try:return early_text_reference(raw,version,kind,identities)
    finally:ObjectArchive.readers["uiViewBucket_c"] = saved


def activity_rows(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:return
        seen.add(id(value))
        if "_stored_activity" in value:flatten(path+".flags",value["_stored_activity"],rows)
        for key,child in value.items():
            if not key.startswith("_"):activity_rows(path+"."+key,child,rows,seen)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):activity_rows(f"{path}[{index}]",child,rows,seen)


def activity_scalars(root: dict[str,object]) -> bytes:
    rows = {}
    activity_rows("root",root,rows,set())
    return node_scalars(root)+"".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
