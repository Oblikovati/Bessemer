"""Complete bucket roots across arc widths, edge IDs and the detailing-view introduction."""
from pathlib import Path
import hashlib
import json
import struct
from generate_middle_buckets import complete_root,displaylists,swcontainer
import generate_middle_buckets as middle
from generate_lower_buckets import omit_final_word
from generate_transition_range import VERSIONS
from transition_bucket_wire import polygon_factory,edge
from generate_version_range import capture_plain,apply_patches
import bucket_vector_wire as wire
import bucket_corpus
import dl_doc


def view_layout(original: object, version: int) -> object:
    def encode(builder: object, identity: int, two_dimensional: bool, empty: bool, mode: int = -1) -> bytes:
        raw = original(builder,identity,two_dimensional,empty,mode)
        if version<=0x3341:
            removed = 4+(0 if empty else len(wire.bucket(builder,identity,False,True)))
            raw = raw[:-(removed+4)]+raw[-4:]
        return raw[:-4] if version<0x374b else raw
    return encode


def root_bytes(document: object, path: Path, raw: bytes, version: int, scenario: int) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    original = dl_doc.read_OrigFontScale
    if version<=0x3331:
        dl_doc.read_OrigFontScale = capture_plain(original,b"",patches)
    try:
        output = complete_root(document,path,raw,version,scenario)
    finally:
        dl_doc.read_OrigFontScale = original
    return apply_patches(output,patches)


def encoded(document: object, path: Path, raw: bytes, version: int, scenario: int) -> bytes:
    original_bucket,original_view,original_poly,original_edge = wire.bucket,wire.view_bucket,middle.legacy_poly,wire.edge
    wire.bucket = omit_final_word(original_bucket) if version<0x372a else original_bucket
    wire.view_bucket = view_layout(original_view,version)
    middle.legacy_poly = polygon_factory(original_poly,version)
    wire.edge = lambda:edge(version)
    try:
        return root_bytes(document,path,raw,version,scenario)
    finally:
        wire.bucket,wire.view_bucket,middle.legacy_poly,wire.edge = original_bucket,original_view,original_poly,original_edge


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    raw = encoded(document,path,document.read("Contents/DisplayLists"),version,scenario)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    root = displaylists.display_lists(archive,"part")
    assert archive.p==len(raw)
    trace = bucket_corpus.trace(root)
    framed = struct.pack("<IBBI",version,0,scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    entries = 45 if scenario%16==2 else 86 if scenario>=16 else 91
    row = {"version":version,"kind":"part","two_dimensional":scenario>=16,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values())}
    return framed,row


def write_scope(two_dimensional: bool) -> tuple[int,int]:
    variant = 16 if two_dimensional else 0
    cases = [generate(version,"part",variant) for version in VERSIONS]+[generate(13000,"part",17 if two_dimensional else 2)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    scope = "2d" if two_dimensional else "3d"
    Path(f"tests/fixtures/display-lists/transition-buckets-{scope}.bin").write_bytes(framed)
    evidence = {"scope":"complete unshaded part roots; raw words and overwritten IDs retained; CAD validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","mfcobjects.py"]}
    Path(__file__).with_name(f"transition-buckets-{scope}.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    return len(cases),sum(row["trace_fields"] for _,row in cases)


def main() -> None:
    totals = [write_scope(value) for value in [False,True]]
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
