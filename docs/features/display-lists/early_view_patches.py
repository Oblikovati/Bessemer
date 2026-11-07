"""Populated, independently versioned named and annotation view collections."""
from window_boundary_patches import convert as convert_window
from generate_version_range import capture_plain
from generate_named_vectors import collection as named_collection
from generate_annotation_vectors import collection as annotation_collection
import dl_doc


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original = dl_doc.read_NamedViewTransform
    dl_doc.read_NamedViewTransform = capture_plain(original,named_collection(version,3),patches)
    try:
        return convert_window(document,path,raw,version,edited,patches,annotation_collection(version,2))
    finally:
        dl_doc.read_NamedViewTransform = original


def named_entries(version: int) -> int:
    return 0 if version<=0x1be7 else 2 if version<=0x1bf4 else 6


def annotation_entries(version: int) -> int:
    return 0 if version<=0x1c39 else 6


def item_entries(version: int) -> int:
    if version<=0x1c15:return 3
    if version<=0x1c49:return 5
    return 9+2*(version>0x1c4b)
