"""Capture physical region words before the independent reader turns them into flags."""
from generate_legacy_panes import physical_reference
from generate_blank_scalars import flatten
import bucket_corpus
import dl_ui


def region_bit_reader(original: object, words: list[int]) -> object:
    def read(flags: int, bit: int, value: int) -> int:
        if bit==dl_ui._NEW_REGION:words.append(value)
        return original(flags,bit,value)
    return read


def region_polygon_reader(original: object) -> object:
    def read(archive: object, flag: int = 0) -> dict[str,object]:
        saved,words = dl_ui._setbit,[]
        dl_ui._setbit = region_bit_reader(saved,words)
        try:result = original(archive,flag)
        finally:dl_ui._setbit = saved
        expected = int(archive.version>0xabc and result["count"]>0)
        assert len(words)==expected,(archive.version,result["count"],words)
        if words:result["_stored_region"] = words[0]
        return result
    return read


def region_reference(raw: bytes, version: int, kind: str, identities: tuple[str,...]) -> dict[str,object]:
    original = dl_ui.read_gcBucketPoly_c
    dl_ui.read_gcBucketPoly_c = region_polygon_reader(original)
    try:return physical_reference(raw,version,kind,identities)
    finally:dl_ui.read_gcBucketPoly_c = original


def region_member(path: str, key: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if key.startswith("_"):return
    suffix = ".items" if key in bucket_corpus.POLY_LISTS else ""
    region_rows(f"{path}.{key}{suffix}",value,rows,seen)


def region_rows(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:return
        seen.add(id(value))
        if "_stored_region" in value:flatten(f"{path}.new_region",value["_stored_region"],rows)
        for key,child in value.items():region_member(path,key,child,rows,seen)
        return
    if isinstance(value,(list,tuple)):
        for index,child in enumerate(value):region_rows(f"{path}[{index}]",child,rows,seen)


def region_scalars(root: dict[str,object]) -> bytes:
    rows = {}
    region_rows("root",root,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
