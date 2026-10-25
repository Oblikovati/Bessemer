"""Independent legacy facet arrays in complete roots on both sides of the old width split."""
import struct
from pathlib import Path
from itertools import product
from legacy_boundary_wire import boundary_facet
from legacy_curvature_wire import curvature_facet
from swarchive import Archive
import facettess

VERSIONS = [596,597,716,717,760,761,766,767]


def facet_array_reader(original: object, archive: object, spans: list[tuple[int,int]]) -> object:
    def read() -> object:
        first = archive.p
        result = original()
        spans.append((first,archive.p))
        return result
    return read


def curvature_array_span(raw: bytes, version: int) -> tuple[int,int]:
    archive,spans = Archive(raw,version),[]
    archive.dyn_array = facet_array_reader(archive.dyn_array,archive,spans)
    facettess.facet_container(archive)
    assert archive.p==len(raw) and len(spans)==4
    return spans[-1]


def fontless_facet(version: int, enabled: bool, edited: bool) -> bytes:
    raw = boundary_facet(version,enabled,edited)
    width = 4 if version<717 else 24
    original = struct.pack("<IIII",width,17,23,1)+bytes(range(width))
    first,last = curvature_array_span(raw,version)
    assert raw[first:last]==original
    count = 1+int(enabled)+int(edited)
    payload = bytearray(bytes(range(width))*count)
    if edited:payload[-1] = 255
    replacement = struct.pack("<IIII",width,17+int(edited),23+int(edited),count)+payload
    return raw[:first]+replacement+raw[last:]


def fontless_facet_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]], faces: list[int]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        index = len(faces)
        assert index<2
        assert archive.b[first:archive.p]==curvature_facet(773,bool(index),edited)
        patches.append((first,archive.p,fontless_facet(version,bool(index),edited)))
        faces.append(index)
        return result
    return read


def write_fontless_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version,enabled,edited in product(VERSIONS,(False,True),(False,True)):
        name = f"legacy-fontless-facet-{version}-{int(enabled)}-{int(edited)}.bin"
        (root/name).write_bytes(fontless_facet(version,enabled,edited))
