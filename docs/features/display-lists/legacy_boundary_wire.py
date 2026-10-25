"""Full roots around body, polygon, facet-stamp and cone-culling gates."""
import struct
from pathlib import Path
from legacy_model_wire import model_root
from legacy_edge_reference import edge_reference
from generate_version_range import apply_patches
from old_shaded_wire import facet_wire
from mfcobjects import ObjectArchive
from swarchive import Archive
import dl_tess
import dl_ui
import facettess

VERSIONS = [2078,2079,2089,2090,2091,2101,2102,2106,2107,2130]


def boundary_facet(version: int, enabled: bool, edited: bool) -> bytes:
    raw = facet_wire(version,-17,False,enabled)
    if edited:
        assert struct.unpack_from("<i",raw)[0]==1
        raw = struct.pack("<i",2)+raw[4:]
        if version>=0x82a:
            stamp = struct.pack("<I",0xdeadbeef)
            assert raw.count(stamp)==1
            raw = raw.replace(stamp,struct.pack("<I",0xdeadbeee))
    archive = Archive(raw,version)
    result = facettess.facet_container(archive)
    assert archive.p==len(raw) and result["num_facets"]==1+int(edited)
    assert result["stamp"]==(0xdeadbeef-int(edited) if version>=0x82a else 0)
    return raw


def facet_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        enabled = "curvature_colors" in result
        patches.append((first,archive.p,boundary_facet(version,enabled,edited)))
        return result
    return read


def body_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert archive.b[archive.p-4:archive.p]==struct.pack("<i",result["unknown_24"])
        patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def polygon_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int = 0) -> object:
        result = original(archive,flag)
        if result["count"]<=0:return result
        boundary,atom = struct.unpack_from("<ii",archive.b,archive.p-8)
        assert atom==result["atom_id"] and bool(boundary)==bool(result["flags"]&4)
        if version<=0x835:patches.append((archive.p-4,archive.p,b""))
        if version<=0x81e:patches.append((archive.p-8,archive.p-4,b""))
        return result
    return read


def boundary_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = model_root(2130,kind,scenario)
    originals = ObjectArchive.readers["uoTempBodyTessData_c"],dl_ui.read_gcBucketPoly_c,dl_tess.facet_container
    patches = []
    if version<0x82b:ObjectArchive.readers["uoTempBodyTessData_c"] = body_reader(originals[0],patches)
    dl_ui.read_gcBucketPoly_c = polygon_reader(originals[1],version,patches)
    dl_tess.facet_container = facet_reader(originals[2],version,scenario%16==1,patches)
    try:edge_reference(raw,2130,kind,identities)
    finally:ObjectArchive.readers["uoTempBodyTessData_c"],dl_ui.read_gcBucketPoly_c,dl_tess.facet_container = originals
    return apply_patches(raw,patches),identities


def write_boundary_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version in VERSIONS:
        for enabled in (False,True):
            for edited in (False,True):
                name = f"legacy-boundary-facet-{version}-{int(enabled)}-{int(edited)}.bin"
                (root/name).write_bytes(boundary_facet(version,enabled,edited))
