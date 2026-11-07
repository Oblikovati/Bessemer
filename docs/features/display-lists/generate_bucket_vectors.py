"""Construct complete owned unshaded bucket streams, with separate semantic oracles."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import compact
from generate_shaded_vectors import ObjectBuilder
from bucket_vector_wire import view_bucket
import bucket_corpus
import dl_doc


def prefix_slot(document: swcontainer.Document, path: Path, raw: bytes) -> tuple[int,int]:
    original = dl_doc.read_AnnotationViewTransform
    slots: list[tuple[int,int]] = []
    def capture(archive: object) -> object:
        result = original(archive)
        slots.append((archive.p,len(archive.map)))
        return result
    dl_doc.read_AnnotationViewTransform = capture
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        root = displaylists.display_lists(archive,archive.doc_kind)
    finally:
        dl_doc.read_AnnotationViewTransform = original
    assert archive.p == len(raw) and root["view_count"] == 1 and len(slots) == 1
    return slots[0]


def write_trace(document: swcontainer.Document, path: Path, raw: bytes, destination: Path) -> int:
    archive = displaylists.archive_for(document,str(path),raw)
    root = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(raw) and root["has_shaded_data"] == 0
    trace = bucket_corpus.trace(root)
    destination.with_suffix(".txt").write_bytes(trace)
    return len(trace.splitlines())


def generate(two_dimensional: bool, empty: bool, edited: bool = False) -> dict[str,object]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    source = document.read("Contents/DisplayLists")
    start,first_index = prefix_slot(document,path,source)
    builder = ObjectBuilder(first_index)
    view = integers(-1)+compact(0,0)+compact(0,0)+cstring("bucket camera 鋼",True)+integers(-2,3,-4)
    view += builder.object("uiViewBucket_c",lambda state:view_bucket(state,first_index+1,two_dimensional,empty,23 if edited else -1))
    derived = source[:start]+view+struct.pack("<H",0)+integers(0)
    name = "bucket-edited" if edited else "bucket-empty" if empty else "bucket-2d" if two_dimensional else "bucket-3d"
    destination = Path(f"tests/fixtures/display-lists/{name}.bin")
    destination.write_bytes(derived)
    fields = write_trace(document,path,derived,destination)
    return {"fixture":name,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"view_start":start,"first_map_index":first_index,"semantic_fields":fields,"aggregate_entries":45 if empty else 86 if two_dimensional else 91,"objects":43 if empty else 45}


def main() -> None:
    evidence = {"scope":"version-19000 wire interoperability; model and application validity not established","fixtures":[generate(True,False),generate(False,False),generate(False,True),generate(True,False,True)]}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","mfcobjects.py"]}
    Path(__file__).with_name("bucket-vectors.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__ == "__main__":
    main()
