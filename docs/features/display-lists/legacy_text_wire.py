"""Complete early ordinary-text tails and physically retained polygon-node words."""
import struct
from pathlib import Path
from legacy_discard_wire import discard_root
from legacy_boundary_wire import facet_reader,boundary_facet
from legacy_edge_reference import edge_reference
import dl_doc
import dl_ui
import dl_tess
from generate_version_range import apply_patches

VERSIONS = [1645,1646,1673,1674,1834,1835,1857]


def text_tail_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        expected = struct.pack("<idd",result["unknown_34"],result["unknown_38"],result["unknown_40"])
        assert archive.b[archive.p-20:archive.p]==expected
        patches.append((archive.p-20,archive.p,b""))
        return result
    return read


def node_patch_reader(original: object, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int = 0) -> object:
        first = archive.p
        result = original(archive,flag)
        assert struct.unpack_from("<i",archive.b,first)[0]==result["node_id"]
        value = 2147483647 if edited else -2147483648
        patches.append((first,first+4,struct.pack("<i",value)))
        return result
    return read


def early_text_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = discard_root(1857,kind,scenario)
    text,alias,poly,facet = dl_doc.ITEM_TYPES[7],dl_doc.ITEM_TYPES[15],dl_ui.read_gcBucketPoly_c,dl_tess.facet_container
    patches,edited = [],scenario==4 or scenario%16==1
    if version<=0x72a:
        dl_doc.ITEM_TYPES[7] = (text[0],text_tail_reader(text[1],patches))
        dl_doc.ITEM_TYPES[15] = (alias[0],text_tail_reader(alias[1],patches))
    dl_ui.read_gcBucketPoly_c = node_patch_reader(poly,edited,patches)
    dl_tess.facet_container = facet_reader(facet,version,edited,patches)
    try:edge_reference(raw,1857,kind,identities)
    finally:dl_doc.ITEM_TYPES[7],dl_doc.ITEM_TYPES[15],dl_ui.read_gcBucketPoly_c,dl_tess.facet_container = text,alias,poly,facet
    return apply_patches(raw,patches),identities


def write_early_text_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version in VERSIONS:
        for enabled in (False,True):
            for edited in (False,True):
                name = f"legacy-text-facet-{version}-{int(enabled)}-{int(edited)}.bin"
                (root/name).write_bytes(boundary_facet(version,enabled,edited))
