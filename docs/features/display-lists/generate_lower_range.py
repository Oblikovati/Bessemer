"""Complete owned roots at every 14000..15999 component, texture and facet boundary."""
from pathlib import Path
import hashlib
import json
import struct
from middle_range_source import source,component_trace,displaylists
from lower_range_patches import texture_source,convert
import material_corpus
import scene_corpus

VERSIONS = [14000,0x3729,0x372a,0x372b,0x3742,0x3743,0x374a,0x374b,0x3751,0x3752,0x3757,0x3758,0x3b0d,0x3b0e,0x3b1b,0x3b1c,0x3b1d,0x3b22,0x3b23,0x3b28,0x3b29,15999,16000]


def entry_budget(version: int, kind: str) -> int:
    entries = 141 if kind=="part" else 146
    entries -= 2 if version<=0x3751 else 0
    entries -= 16 if version<=0x3b28 else 0
    if kind=="assembly":
        entries += 0 if version<=0x3b1b else 1 if version<0x3b23 else 2
    return entries


def scalar_trace(root: dict[str,object], kind: str, version: int) -> bytes:
    scalar = component_trace(root,kind)
    if 0x3b1b<version<0x3b23:
        scalar = b"".join(line+b"\n" for line in scalar.splitlines() if b".viewer_comp_tess_numbers[0][0]=" not in line)
    return scalar


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    path,document,raw = source(kind)
    raw = convert(document,path,texture_source(document,path,raw),version,scenario%32==1)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = material_corpus.trace(root)+scene_corpus.trace(root)
    scalar = scalar_trace(root,kind,version)
    framed = struct.pack("<IBBI",version,kind=="assembly",32+scenario%32,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",len(scalar))+scalar
    evidence = {"version":version,"kind":kind,"scenario":32+scenario%32,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(trace.splitlines())+len(scalar.splitlines()),"aggregate_entries":entry_budget(version,kind),"objects":sum(archive.classes.values())}
    return framed,evidence


def main() -> None:
    cases = [generate(version,kind) for version in VERSIONS for kind in ["part","assembly"]]+[generate(14000,kind,1) for kind in ["part","assembly"]]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/lower-range.bin").write_bytes(framed)
    evidence = {"scope":"complete wire roots with populated textures and inline facets; file context 19000; no CAD validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_tess.py","dl_doc.py","dl_light.py","dl_ui.py","facettess.py","mfcobjects.py"]}
    Path(__file__).with_name("lower-range.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(cases),"fields":sum(row["oracle_fields"] for _,row in cases)}))


if __name__=="__main__":
    main()
