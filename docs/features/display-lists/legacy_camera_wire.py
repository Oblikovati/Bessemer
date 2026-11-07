"""Remove observed camera and viewer fields before their documented version gates."""
import struct
from legacy_pane_wire import root_bytes,display_items
from generate_legacy_panes import physical_reference
from generate_version_range import apply_patches
from generate_transform_vectors import doubles,compact
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_item_vectors import record
import dl_doc
import dl_tess


def capture_reader(original: object, spans: list[tuple[int,int]], tail: int = 0) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        spans.append((archive.p-tail if tail else start,archive.p))
        return result
    return read


def camera_reader(original: object, spans: list[tuple[int,int]]) -> object:
    def view(archive: object) -> object:
        result = original(archive)
        string_reader = archive.cstring
        def camera() -> str:
            start = archive.p
            result = string_reader()
            spans.append((start,archive.p))
            archive.cstring = string_reader
            return result
        archive.cstring = camera
        return result
    return view


def observed_fields(raw: bytes, kind: str, identities: tuple[str,...]) -> dict[str,list[tuple[int,int]]]:
    spans = {name:[] for name in ("camera","viewer","items")}
    originals = dl_doc.read_AnnotationViewTransform,dl_tess.read_moViewerComponentData,dl_doc.read_display_item_list
    dl_doc.read_AnnotationViewTransform = camera_reader(originals[0],spans["camera"])
    dl_tess.read_moViewerComponentData = capture_reader(originals[1],spans["viewer"],8)
    dl_doc.read_display_item_list = capture_reader(originals[2],spans["items"])
    try:physical_reference(raw,2999,kind,identities)
    finally:dl_doc.read_AnnotationViewTransform,dl_tess.read_moViewerComponentData,dl_doc.read_display_item_list = originals
    return spans


def text_items(version: int, edited: bool) -> bytes:
    raw = doubles(3)
    if version>0xb74:raw += integers(-7)+compact(2,0x40)
    raw += doubles(1)+integers(-1)+cstring("text 鋼",True)
    raw += struct.pack("<h",-2)+integers(3,-1,2,3)
    return struct.pack("<H",5)+display_items(version,edited)[2:]+record(18,raw)


def camera_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = root_bytes(version,kind,scenario)
    spans = observed_fields(raw,kind,identities)
    assert len(spans["items"])==1 and len(spans["camera"])>=1,spans
    first,last = spans["items"][0]
    patches = [(first,last,text_items(version,scenario in (1,4)))]
    if version<=0xbb6:patches.append((*spans["camera"][0],b""))
    if version<0xb9b:patches += [(first,last,b"") for first,last in spans["viewer"]]
    return apply_patches(raw,patches),identities
