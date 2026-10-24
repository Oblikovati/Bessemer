"""Independent version-19000 reference-geometry bodies embedded in an owned complete part root."""
from pathlib import Path
import hashlib
import json
import struct
from generate_named_vectors import cstring
from generate_transform_vectors import doubles, fixed_transform
from generate_blank_scalars import flatten
from generate_shaded_vectors import ShadedArchive, displaylists, swcontainer, dl_tess
import reference_corpus


def plane() -> bytes:
    raw = doubles(3)+doubles(3,3)+fixed_transform()
    return raw+doubles(4,4)+struct.pack("<ii", -110, 114)


def coordinate_system() -> bytes:
    return doubles(9,2)+struct.pack("<iiiii", -11,12,-13,14,-15)+doubles(3,5)


def cpoint_extra(first_float: int) -> bytes:
    raw = struct.pack("<Q",first_float)+doubles(1,3)+doubles(3,4)+doubles(3,7)+struct.pack("<ii", -40,44)
    raw += doubles(1,5)+struct.pack("<iii", -68,108,-120)+doubles(1,1)
    raw += struct.pack("<ii", 160,-164)+cstring("extra 鋼😀",True)+cstring("é\0")
    return raw+struct.pack("<i", -184)


def cpoint(first_float: int) -> bytes:
    raw = cstring("cpoint 鋼",True)+struct.pack("<i",-176)
    return raw+doubles(9,1)+doubles(3,4)+doubles(2,7)+cpoint_extra(first_float)


def reference(kind: int, first_float: int) -> bytes:
    base = struct.pack("<i",kind)+cstring(f"kind {kind} 鋼",True)+struct.pack("<iii",-10,14,-18)
    bodies = {0:b"", 1:doubles(3,7), 2:doubles(9)+doubles(3,3)+doubles(2,6),
              3:plane(), 4:coordinate_system(), 5:cpoint(first_float), 6:doubles(3,2)+doubles(3,5)}
    return base+bodies[kind]


def collector_slot(document: swcontainer.Document, source: Path, raw: bytes) -> tuple[int,int]:
    original = dl_tess.read_moRefGeomFeatureDataCollector
    spans: list[tuple[int,int]] = []
    def capture(archive: ShadedArchive) -> object:
        start = archive.p
        result = original(archive)
        spans.append((start,archive.p))
        return result
    dl_tess.read_moRefGeomFeatureDataCollector = capture
    try:
        archive = displaylists.archive_for(document,str(source),raw)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p == len(raw)
    finally:
        dl_tess.read_moRefGeomFeatureDataCollector = original
    assert len(spans) == 1
    return spans[0]


def oracle(document: swcontainer.Document, source: Path, raw: bytes, kinds: list[int], destination: Path) -> tuple[int,int]:
    archive = displaylists.archive_for(document,str(source),raw)
    root = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(raw)
    items = root["tessellation"]["ref_geom_feature_data_collector"]
    assert len(items) == len(kinds)
    rows: dict[str,str] = {}
    for index,(kind,item) in enumerate(zip(kinds,items,strict=True)):
        prefix = f"root.tessellation.ref_geom_feature_data_collector.entries[{index}]"
        item = dict(item)
        if kind == 3:
            del item["unknown_68"]
        flatten(prefix,item,rows)
        rows[f"{prefix}.type"] = f"i:{kind}"
    destination.write_text("".join(f"{key}={value}\n" for key,value in sorted(rows.items())),encoding="utf-8",newline="\n")
    trace = reference_corpus.trace(root)
    destination.with_name(destination.stem+"-trace.txt").write_bytes(trace)
    return len(rows),len(trace.splitlines())


def generate(kinds: list[int], name: str, first_float: int = 0x7ff0000000000000) -> dict[str,object]:
    source = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(source))
    raw = document.read("Contents/DisplayLists")
    start,end = collector_slot(document,source,raw)
    collector = struct.pack("<II",0,len(kinds))+b"".join(reference(kind,first_float) for kind in kinds)
    derived = raw[:start]+collector+raw[end:]
    destination = Path(f"tests/fixtures/display-lists/{name}.bin")
    destination.write_bytes(derived)
    fields,trace_fields = oracle(document,source,derived,kinds,destination.with_suffix(".txt"))
    return {"fixture":name,"kinds":kinds,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"scalar_fields":fields,"trace_fields":trace_fields,"aggregate_entries":49+len(kinds),"collector_start":start,"collector_end":start+len(collector),"source_sha256":hashlib.sha256(raw).hexdigest(),"fixed_transform_sha256":hashlib.sha256(fixed_transform()).hexdigest(),"reference_reader_sha256":hashlib.sha256(Path(dl_tess.__file__).read_bytes()).hexdigest()}


if __name__ == "__main__":
    rows = [generate(list(range(7)),"reference-kinds"),generate([6,5,4,3,2,1,0,2],"reference-permuted"),generate(list(range(7)),"reference-edited",0x4037000000000000)]
    Path(__file__).with_name("reference-vectors.json").write_text(json.dumps(rows,indent=2)+"\n",encoding="utf-8",newline="\n")
