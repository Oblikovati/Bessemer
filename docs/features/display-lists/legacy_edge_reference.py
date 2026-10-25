"""Retain the temporary edge word discarded by the independent native-reader model."""
from legacy_region_reference import region_reference
import dl_ui


def integer_reader(original: object, words: list[int]) -> object:
    def read() -> int:
        value = original()
        words.append(value)
        return value
    return read


def discarded_reader(original: object) -> object:
    def read(archive: object) -> dict[str,object]:
        saved,words = archive.i32,[]
        archive.i32 = integer_reader(saved,words)
        try:result = original(archive)
        finally:archive.i32 = saved
        assert words[:2]==[result["unknown_0"],result["unknown_4"]],words
        if archive.version<0xa4b:result["discarded_old_8"] = words[2]
        if 0x744<=archive.version<=0x74a:result["discarded_v0744"] = words[5]
        return result
    return read


def edge_reference(raw: bytes, version: int, kind: str, identities: tuple[str,...]) -> dict[str,object]:
    original = dl_ui.read_gcEdgeVisData_c
    dl_ui.read_gcEdgeVisData_c = discarded_reader(original)
    try:return region_reference(raw,version,kind,identities)
    finally:dl_ui.read_gcEdgeVisData_c = original
