"""Complete old bucket roots independently assembled for each supported snapshot."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_shaded_vectors import ObjectBuilder
from generate_early_shaded import generate as early_root
from generate_old_shaded import generate as old_root
from legacy_bucket_wire import view_bucket
import bucket_corpus


def source_root(version: int) -> bytes:
    framed,_ = early_root(version,"part",2) if version<10000 else old_root(version,"part",False,True)
    stored,kind,scenario,length = struct.unpack_from("<IBBI",framed)
    assert (stored,kind,scenario)==(version,0,2)
    return framed[10:10+length]


def bucket_slot(document: swcontainer.Document, path: Path, raw: bytes, version: int) -> tuple[int,int,int]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    original = archive.object
    slots: list[tuple[int,int,int]] = []
    def capture(expect: object = None) -> object:
        start,index = archive.p,len(archive.map)
        result = original(expect)
        if result is None:
            slots.append((start,archive.p,index))
        return result
    archive.object = capture
    root = displaylists.display_lists(archive,"part")
    assert archive.p==len(raw) and root["has_shaded_data"]==0 and len(slots)==1
    return slots[0]


def generate(version: int, two_dimensional: bool, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    raw = source_root(version)
    start,end,index = bucket_slot(document,path,raw,version)
    builder = ObjectBuilder(index)
    replacement = builder.object("uiViewBucket_c",lambda state:view_bucket(state,version,index+1,two_dimensional,scenario==2,scenario==1))
    derived = raw[:start]+replacement+raw[end:]
    archive = displaylists.archive_for(document,str(path),derived)
    archive.version = version
    root = displaylists.display_lists(archive,"part")
    assert archive.p==len(derived)
    trace = bucket_corpus.trace(root)
    framed = struct.pack("<IBBI",version,two_dimensional,scenario,len(derived))+derived+struct.pack("<I",len(trace))+trace
    prefix = 52 if version==7000 else 64 if version==9000 else 51
    entries = prefix+(0 if scenario==2 else 28+5*(2 if two_dimensional else 3)+(2 if version==11000 else 0))
    return framed,{"version":version,"two_dimensional":two_dimensional,"scenario":scenario,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"trace_fields":len(trace.splitlines()),"objects":sum(archive.classes.values()),"aggregate_entries":entries}


def main() -> None:
    cases = [generate(version,two) for version in [7000,9000,10000,11000] for two in [False,True]]+[generate(10000,True,1),generate(7000,False,2)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/legacy-buckets.bin").write_bytes(framed)
    evidence = {"scope":"complete unshaded roots at selected contexts, file version 19000; no model/application validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_ui.py","dl_doc.py","mfcobjects.py"]}
    Path(__file__).with_name("legacy-buckets.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
