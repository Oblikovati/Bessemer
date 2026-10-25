"""Complete unshaded roots crossing the separate-to-packed bucket polygon flag transition."""
from pathlib import Path
import hashlib
import json
import struct
from generate_bucket_vectors import prefix_slot,displaylists,swcontainer
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import compact
from generate_shaded_vectors import ObjectBuilder
import bucket_vector_wire as wire
import bucket_corpus


def legacy_poly(original: object) -> object:
    def encode(count: int, dimensions: int, kind: int = 1, arc: bool = True, line: bool = True) -> bytes:
        raw = original(count,dimensions,kind,arc,line)
        raw = raw[:8]+integers(-17)+raw[12:]
        if count<=0:
            return raw
        tail = 52+(83 if arc else 0)
        attributes = integers(-3,-6,0,7,6,-7,-8,9,-10,0,-11)
        arc_bytes = raw[-115:-32] if arc else b""
        return raw[:-tail]+attributes+integers(-3 if arc else 0)+arc_bytes+integers(-2)+raw[-32:]
    return encode


def bucket_bytes(builder: ObjectBuilder, version: int, two_dimensional: bool, scenario: int) -> bytes:
    original = wire.poly
    try:
        if version<=0x41a0:
            wire.poly = legacy_poly(original)
        return builder.object("uiViewBucket_c",lambda state:wire.view_bucket(state,state.next_index-1,two_dimensional,scenario==2,23 if scenario==1 else -1))
    finally:
        wire.poly = original


def complete_root(document: object, path: Path, raw: bytes, version: int, scenario: int) -> bytes:
    start,index = prefix_slot(document,path,raw)
    view = integers(-1)+compact(0,0)+compact(0,0)+cstring("bucket camera 鋼",True)+integers(-2,3,-4)
    return raw[:start]+view+bucket_bytes(ObjectBuilder(index),version,scenario>=16,scenario%16)+struct.pack("<H",0)+integers(0)


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    assert kind=="part", f"middle bucket kind {kind!r}; expected part"
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    raw = complete_root(document,path,document.read("Contents/DisplayLists"),version,scenario)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    root = displaylists.display_lists(archive,"part")
    assert archive.p==len(raw)
    trace = bucket_corpus.trace(root)
    framed = struct.pack("<IBBI",version,0,scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    entries = 45 if scenario%16==2 else 86 if scenario>=16 else 91
    evidence = {"version":version,"kind":"part","two_dimensional":scenario>=16,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values()),"external_ids":[]}
    return framed,evidence


def main() -> None:
    cases = [generate(version,"part",scenario) for version in [16000,0x41a0,0x41a1,17999] for scenario in [0,16]]+[generate(16000,"part",17),generate(16000,"part",2)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/middle-buckets.bin").write_bytes(framed)
    evidence = {"scope":"part roots in both coordinate scopes; version 16000..17999; CAD validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","mfcobjects.py"]}
    Path(__file__).with_name("middle-buckets.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(cases),"trace_fields":sum(row["trace_fields"] for _,row in cases)}))


if __name__=="__main__":
    main()
