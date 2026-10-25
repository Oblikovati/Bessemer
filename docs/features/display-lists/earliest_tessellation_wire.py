"""Remove independently observed pre-FD8 face words and empty decal-list headers."""
import struct
from generate_alias_fixture import collect
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_tess


def face_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert archive.b[archive.p-4:archive.p]==struct.pack("<i",result["unknown_34"])
        patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def decal_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        assert result==[] and archive.p==start+4
        patches.append((start,archive.p,b""))
        return result
    return read


def earlier_tails(document: object, path: object, raw: bytes, version: int, kind: str) -> bytes:
    if version>=0xfd8:return raw
    patches = []
    original_version,original_decals = dl_tess._v,dl_tess._decal_list
    original_face = ObjectArchive.readers["uoTempFaceTessData_c"]
    dl_tess._v = lambda archive:max(original_version(archive),0xfd8)
    dl_tess._decal_list = decal_reader(original_decals,patches)
    ObjectArchive.readers["uoTempFaceTessData_c"] = face_reader(original_face,patches)
    try:collect(document,path,raw,version,set())
    finally:
        dl_tess._v,dl_tess._decal_list = original_version,original_decals
        ObjectArchive.readers["uoTempFaceTessData_c"] = original_face
    assert len(patches)==(4 if kind=="part" else 5)
    return apply_patches(raw,patches)


def header_layout(raw: bytes, version: int) -> bytes:
    # The two root words and six bound doubles precede the optional size double.
    assert raw[:4]==struct.pack("<i",1)
    return raw[:56]+raw[64:] if version<=0xfca else raw
