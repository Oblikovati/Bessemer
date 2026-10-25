"""Remove independently observed pre-FD8 face words and empty decal-list headers."""
import struct
from contextlib import contextmanager
from collections.abc import Iterator
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


def assembly_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        if version<=0xf76:
            assert archive.b[archive.p-8:archive.p-4]==struct.pack("<i",result["has_shrink_wrap_config"])
            patches.append((archive.p-8,archive.p-4,b""))
        return result
    return read


@contextmanager
def tail_readers(patches: list[tuple[int,int,bytes]], version: int) -> Iterator[None]:
    original_version,original_decals = dl_tess._v,dl_tess._decal_list
    names = ["uoTempFaceTessData_c","uoTempAssemblySHDData_c","uoTempSubAssemblySHDData_c"]
    originals = {name:ObjectArchive.readers[name] for name in names}
    dl_tess._v = lambda archive:max(original_version(archive),0xfd8)
    dl_tess._decal_list = decal_reader(original_decals,patches)
    for name,original in originals.items():
        ObjectArchive.readers[name] = face_reader(original,patches) if name==names[0] else assembly_reader(original,version,patches)
    try:yield
    finally:
        dl_tess._v,dl_tess._decal_list = original_version,original_decals
        ObjectArchive.readers.update(originals)


def earlier_tails(document: object, path: object, raw: bytes, version: int, kind: str) -> bytes:
    if version>=0xfd8:return raw
    patches = []
    with tail_readers(patches,version):collect(document,path,raw,version,set())
    assert len(patches)==(4 if kind=="part" else 5+int(version<=0xf76))
    return apply_patches(raw,patches)

def header_layout(raw: bytes, version: int) -> bytes:
    # The two root words and six bound doubles precede the optional size double.
    assert raw[:4]==struct.pack("<i",1)
    return raw[:56]+raw[64:] if version<=0xfca else raw
