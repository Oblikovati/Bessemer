"""Freeze complete roots at each version boundary from 16000 through 17999."""
from pathlib import Path
import hashlib
import json
import struct
from middle_range_source import source,component_trace,displaylists
from middle_range_patches import convert
import material_corpus
import scene_corpus

VERSIONS = [16000,0x3ee7,0x3ee8,0x3f08,0x3f09,0x3f0e,0x3f0f,0x3f10,0x41a0,0x41a1,0x42da,0x42db,0x42e7,0x42e8,0x42ec,0x42ed,0x42f2,0x42f3,17999,18000]


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    path,document,raw = source(kind)
    raw = convert(document,path,raw,version,scenario==1)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = material_corpus.trace(root)+scene_corpus.trace(root)
    scalar = component_trace(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",len(scalar))+scalar
    entries = (149 if kind=="part" else 154)-(8 if version<=0x3ee7 else 0)
    evidence = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"component_fields":len(scalar.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values())}
    return framed,evidence


def main() -> None:
    cases = [generate(version,kind) for version in VERSIONS for kind in ["part","assembly"]]+[generate(16000,kind,1) for kind in ["part","assembly"]]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/middle-range.bin").write_bytes(framed)
    evidence = {"scope":"complete owned wire roots with inline facets, explicit file context 19000; CAD validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_tess.py","dl_doc.py","dl_light.py","dl_ui.py","facettess.py"]}
    Path(__file__).with_name("middle-range.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(cases),"trace_fields":sum(row["trace_fields"]+row["component_fields"] for _,row in cases)}))


if __name__=="__main__":
    main()
