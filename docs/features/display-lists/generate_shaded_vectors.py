"""Embed independently built shaded objects in owned roots, with inline and external facets."""
from collections.abc import Callable
from pathlib import Path
import hashlib
import json
import struct
from typing import Protocol
from generate_alias_fixture import swcontainer, displaylists
from generate_named_vectors import cstring
from generate_transform_vectors import doubles
import dl_tess


class ShadedArchive(Protocol):
    p: int
    map: list[object]


class ObjectBuilder:
    def __init__(self, first_index: int) -> None:
        self.next_index = first_index
        self.classes: dict[str, int] = {}

    def object(self, name: str, body: Callable[["ObjectBuilder"], bytes]) -> bytes:
        if name in self.classes:
            prefix = struct.pack("<H", 0x8000 | self.classes[name])
        else:
            self.classes[name] = self.next_index
            self.next_index += 1
            prefix = struct.pack("<HHH", 0xffff, 1, len(name))+name.encode("ascii")
        self.next_index += 1
        return prefix+body(self)


def face(external: bool, index: int) -> bytes:
    raw = b"" if external else Path("tests/fixtures/tessellation/facet-4a38-1.bin").read_bytes()
    raw += struct.pack("<i", 0)+struct.pack("<iii", -34, -68, 2)
    raw += struct.pack("<iiiii", -1, 2, -3, 4, -38)
    raw += struct.pack("<i", 0)+cstring(f"face {index} 鋼", True)
    return raw+struct.pack("<ii", index, -78)


def body(builder: ObjectBuilder, external: bool) -> bytes:
    raw = struct.pack("<H", 2)
    for index in range(2):
        raw += builder.object("uoTempFaceTessData_c", lambda _: face(external, index))
    raw += struct.pack("<iiiii", -20, -24, -44, 1, -1)
    raw += builder.object("uoBodyPropInfo_c", lambda _: struct.pack("<Hiiii", 0, -8, 6, -16, 3))
    return raw+struct.pack("<I", 0)+cstring("body 鋼", True)+struct.pack("<i", -104)


def part(builder: ObjectBuilder, external: bool) -> bytes:
    raw = struct.pack("<H", 1)+builder.object("uoTempBodyTessData_c", lambda state: body(state, external))
    raw += struct.pack("<iIiiiiiIiii", 0, 0xffffffff, 2, -1, 2, -3, 4, 0, -168, -2, -3)
    raw += cstring("configuration 鋼", True)+struct.pack("<i", 0)+doubles(10)
    return raw+struct.pack("<IIB", 0, 0, 255)


def part_slot(document: swcontainer.Document, path: Path, raw: bytes) -> tuple[int, int, int]:
    original = dl_tess.read_uoTempPartTessData
    spans: list[tuple[int, int, int]] = []
    def capture(archive: ShadedArchive) -> object:
        start, first_index = archive.p, len(archive.map)
        result = original(archive)
        spans.append((start, archive.p, first_index))
        return result
    dl_tess.read_uoTempPartTessData = capture
    try:
        archive = displaylists.archive_for(document, str(path), raw)
        displaylists.display_lists(archive, archive.doc_kind)
        assert archive.p == len(raw)
    finally:
        dl_tess.read_uoTempPartTessData = original
    assert len(spans) == 1
    return spans[0]


def assembly_slot(document: swcontainer.Document, path: Path, raw: bytes) -> tuple[int, int, int]:
    archive = displaylists.archive_for(document, str(path), raw)
    original = archive.object
    slots: list[tuple[int, int, int]] = []
    def capture(expect: object = None) -> object:
        start, first_index = archive.p, len(archive.map)
        result = original(expect)
        if result is None:
            slots.append((start, archive.p, first_index))
        return result
    archive.object = capture
    root = displaylists.display_lists(archive, archive.doc_kind)
    assert root["tessellation"][0]["comp_part_tess_data"] is None
    assert archive.p == len(raw)
    return slots[-2]


def generate(external: bool) -> dict[str, object]:
    source = Path("tests/fixtures/blank-assembly.SLDASM" if external else "tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(source))
    raw = document.read("Contents/DisplayLists")
    start, end, first_index = (assembly_slot if external else part_slot)(document, source, raw)
    builder = ObjectBuilder(first_index)
    replacement = builder.object("uoTempPartTessData_c", lambda state: part(state, True)) if external else part(builder, False)
    derived = raw[:start]+replacement+raw[end:]
    archive = displaylists.archive_for(document, str(source), derived)
    archive.face_tess_comp_ids = {"Assem1"} if external else set()
    root = displaylists.display_lists(archive, archive.doc_kind)
    assert archive.p == len(derived)
    tessellation = root["tessellation"][0]["comp_part_tess_data"] if external else root["tessellation"]
    faces = tessellation["bodies"][0]["faces"]
    assert len(faces) == 2
    assert all(("facet_container_from" in item) == external for item in faces)
    name = "external-shaded-assembly" if external else "inline-shaded-part"
    Path(f"tests/fixtures/display-lists/{name}.bin").write_bytes(derived)
    return {"fixture": name, "bytes": len(derived), "sha256": hashlib.sha256(derived).hexdigest(), "source_sha256": hashlib.sha256(raw).hexdigest(), "map_slots": len(archive.map), "first_inserted_index": first_index, "face_names": [item["face_string"] for item in faces], "external_components": sorted(archive.face_tess_comp_ids)}


if __name__ == "__main__":
    evidence = [generate(False), generate(True)]
    Path(__file__).with_name("shaded-vectors.json").write_text(json.dumps(evidence, indent=2)+"\n", encoding="utf-8", newline="\n")
