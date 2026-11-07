"""Keep the native logical node reset and capture the physically consumed i32 separately."""
import struct
from legacy_edge_reference import edge_reference
from legacy_region_reference import region_scalars as node_scalars
import dl_ui


def retained_node_reader(original: object) -> object:
    def read(archive: object, flag: int = 0) -> object:
        raw, = struct.unpack_from("<i",archive.b,archive.p)
        result = original(archive,flag)
        assert result["node_id"]==(-1 if archive.version<0x66e else raw)
        result["_stored_node_id"] = raw
        return result
    return read


def early_text_reference(raw: bytes, version: int, kind: str, identities: tuple[str,...]) -> dict[str,object]:
    original = dl_ui.read_gcBucketPoly_c
    dl_ui.read_gcBucketPoly_c = retained_node_reader(original)
    try:return edge_reference(raw,version,kind,identities)
    finally:dl_ui.read_gcBucketPoly_c = original
