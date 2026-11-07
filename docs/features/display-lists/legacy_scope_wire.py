"""Complete three-dimensional buckets before stored 2D coordinate scopes."""
import struct
from pathlib import Path
from itertools import product
from legacy_ghost_wire import early_ghost_root
from legacy_text_reference import early_text_reference
from legacy_boundary_wire import facet_reader,boundary_facet
from legacy_edge_roots import span_reader
from generate_version_range import apply_patches
import dl_ui
import dl_tess

VERSIONS = [1180,1181,1182,1215]


def scope_polygon_reader(original: object, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int = 0) -> object:
        first,two_dimensional = archive.p,bool(getattr(archive,"gc_2d",0))
        result = original(archive,flag)
        if not two_dimensional or result["count"]<=0:return result
        points = result["points_f32"]
        assert len(points)==8*result["count"] and archive.b[first+16:first+16+len(points)]==points
        z = struct.pack("<f",2.0 if edited else 1.0)
        expanded = b"".join(points[index:index+8]+z for index in range(0,len(points),8))
        patches.append((first+16,first+16+len(points),expanded))
        return result
    return read


def scope_bucket_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, obj_keys: int) -> object:
        first,saved,transforms = archive.p,dl_ui.read_mgXform_c,[]
        dl_ui.read_mgXform_c = span_reader(saved,transforms)
        try:result = original(archive,flag,obj_keys)
        finally:dl_ui.read_mgXform_c = saved
        position = first+44
        present, = struct.unpack_from("<i",archive.b,position)
        assert present==result["has_2d_info"]
        end = next(last for start,last in transforms if start==position+4) if present else position+4
        patches.append((position,end,b""))
        return result
    return read


def early_scope_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = early_ghost_root(1215,kind,scenario)
    saved = dl_ui.read_gcBucket_c,dl_ui.read_gcBucketPoly_c,dl_tess.facet_container
    patches,edited = [],scenario==4 or scenario%16==1
    if version<=0x49c:
        dl_ui.read_gcBucket_c = scope_bucket_reader(saved[0],patches)
        dl_ui.read_gcBucketPoly_c = scope_polygon_reader(saved[1],edited,patches)
    dl_tess.facet_container = facet_reader(saved[2],version,edited,patches)
    try:early_text_reference(raw,1215,kind,identities)
    finally:dl_ui.read_gcBucket_c,dl_ui.read_gcBucketPoly_c,dl_tess.facet_container = saved
    return apply_patches(raw,patches),identities


def write_early_scope_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version,enabled,edited in product(VERSIONS,(False,True),(False,True)):
        name = f"legacy-scope-facet-{version}-{int(enabled)}-{int(edited)}.bin"
        (root/name).write_bytes(boundary_facet(version,enabled,edited))
