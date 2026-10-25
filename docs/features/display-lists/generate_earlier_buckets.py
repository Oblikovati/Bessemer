"""Owned complete bucket roots at every earlier coordinate, pointer and tail boundary."""
from pathlib import Path
import hashlib
import json
import struct
from generate_earlier_range import VERSIONS
from generate_middle_buckets import complete_root,displaylists,swcontainer
from generate_transition_buckets import view_layout
from generate_version_range import capture_plain,apply_patches
from earlier_bucket_wire import polygon_encoder,bucket_encoder
from transition_bucket_wire import edge
import bucket_vector_wire as wire
import bucket_corpus
import dl_doc


def encoded(document: object, path: Path, raw: bytes, version: int, scenario: int) -> bytes:
    originals = wire.bucket,wire.view_bucket,wire.poly,wire.edge,dl_doc.read_OrigFontScale
    patches: list[tuple[int,int,bytes]] = []
    def encode(builder: object) -> bytes:
        wire.poly = polygon_encoder(builder,version)
        return builder.object("uiViewBucket_c",lambda state:wire.view_bucket(state,state.next_index-1,scenario>=16,scenario%16==2,23 if scenario%16==1 else -1))
    wire.bucket,wire.view_bucket = bucket_encoder(originals[0],version),view_layout(originals[1],version)
    wire.edge = lambda:edge(version)
    dl_doc.read_OrigFontScale = capture_plain(originals[4],b"",patches)
    try:
        result = complete_root(document,path,raw,version,scenario,encode)
    finally:
        wire.bucket,wire.view_bucket,wire.poly,wire.edge,dl_doc.read_OrigFontScale = originals
    return apply_patches(result,patches)


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    assert kind=="part"
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    raw = encoded(document,path,document.read("Contents/DisplayLists"),version,scenario)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    root = displaylists.display_lists(archive,"part")
    assert archive.p==len(raw),(version,scenario,archive.p,len(raw))
    trace = bucket_corpus.trace(root)
    framed = struct.pack("<IBBI",version,0,scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    entries = 45 if scenario%16==2 else (86 if scenario>=16 else 91)-(version<=0x2faa)
    row = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values())}
    return framed,row


def write_scope(two_dimensional: bool) -> tuple[int,int]:
    cases = [generate(version,"part",16 if two_dimensional else 0) for version in VERSIONS]+[generate(12000,"part",17 if two_dimensional else 2)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    scope = "2d" if two_dimensional else "3d"
    Path(f"tests/fixtures/display-lists/earlier-buckets-{scope}.bin").write_bytes(framed)
    evidence = {"scope":"complete owned unshaded roots; discarded objects retained; CAD validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","mfcobjects.py"]}
    Path(__file__).with_name(f"earlier-buckets-{scope}.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    return len(cases),sum(row["trace_fields"] for _,row in cases)


if __name__=="__main__":
    totals = [write_scope(value) for value in [False,True]]
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))
