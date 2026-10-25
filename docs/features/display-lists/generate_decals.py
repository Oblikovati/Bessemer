"""Freeze complete roots with body/component lists and versioned part decal maps."""
from pathlib import Path
import hashlib
import json
import struct
from decal_sources import source,locate,displaylists
from decal_wire import payload
from decal_corpus import trace
from generate_version_range import apply_patches


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    path,document,original,base_entries = source(version,kind)
    slots,material = locate(document,path,original,version,kind)
    patches = [(start,end,payload(version,material,scenario,mapped,index==0)) for index,(start,end,mapped) in enumerate(slots)]
    raw = apply_patches(original,patches)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    scalar = trace(root)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<II",0,len(scalar))+scalar
    maps = sum(mapped for _,_,mapped in slots)
    per_decal = 0 if scenario==2 else 4
    entries = base_entries if scenario==3 else base_entries+2*len(slots)*(1+per_decal)+2*maps
    evidence = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"scalar_fields":len(scalar.splitlines()),"objects":sum(archive.classes.values()),"lists":len(slots)-maps,"maps":maps,"aggregate_entries":entries,"material_sha256":hashlib.sha256(material).hexdigest()}
    return framed,evidence


def main() -> None:
    cases = [generate(version,kind,0) for version in [7000,9000,10000,11000,19000] for kind in ["part","assembly"]]
    cases += [generate(18000,"part",0)]+[generate(19000,"part",scenario) for scenario in [1,2,3]]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/decals.bin").write_bytes(framed)
    evidence = {"scope":"complete archive wire roots, explicit file context 19000; no CAD validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_tess.py","dl_light.py","swarchive.py"]}
    Path(__file__).with_name("decals.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
