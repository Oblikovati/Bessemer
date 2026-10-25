"""Owned complete streams for the version-7000/9000 layout gates."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_component_vectors import integers
from generate_shaded_vectors import ObjectBuilder
from generate_old_shaded import component_trace
from early_prefix import prefix
from early_shaded_wire import early_shaded
import material_corpus
import scene_corpus
import light_corpus
import item_corpus


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    start,index = prefix(document,path,document.read("Contents/DisplayLists"),version)
    derived = start+(integers(0) if scenario==2 else early_shaded(ObjectBuilder(index),version,kind,scenario==1))
    archive = displaylists.archive_for(document,str(path),derived)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(derived)
    trace = material_corpus.trace(root)+scene_corpus.trace(root)+light_corpus.trace(root)+item_corpus.trace(root)
    scalar = component_trace(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(derived))+derived+struct.pack("<I",len(trace))+trace+struct.pack("<I",len(scalar))+scalar
    entries = (52 if version==7000 else 64)+(0 if scenario==2 else 36+(8 if kind=="assembly" else 0))
    return framed,{"version":version,"kind":kind,"scenario":scenario,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"trace_fields":len(trace.splitlines()),"component_scalar_fields":len(scalar.splitlines()),"first_map_index":index,"new_objects":sum(archive.classes.values()),"aggregate_entries":entries}


def main() -> None:
    cases = [generate(version,kind) for version in [7000,9000] for kind in ["part","assembly"]]+[generate(9000,"part",1),generate(9000,"part",2),generate(7000,"part",2)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/early-shaded.bin").write_bytes(framed)
    evidence = {"scope":"complete wire roots at selected versions, file context 19000; no model/application validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_light.py","dl_ui.py","dl_tess.py","facettess.py"]}
    Path(__file__).with_name("early-shaded.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
