"""Independently encode complete graphics-reference roots with explicit plane-layout evidence."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer,collect
from generate_component_vectors import integers
from generate_shaded_vectors import ObjectBuilder
from generate_old_shaded import prefix as old_prefix
import generate_old_shaded as old_shaded
from generate_scene_vectors import node_list,pw_scene,wall,floor
from generate_named_vectors import cstring
from graphics_wire import manager,ref
import graphics_corpus
import dl_tess


def plane_reader(archive: object) -> dict[str,object]:
    return {"root_point":archive.point(),"normal":archive.point()}


def fixture_slot(document: swcontainer.Document, path: Path, raw: bytes, kind: str) -> tuple[int,int]:
    archive = displaylists.archive_for(document,str(path),raw)
    strings: list[int] = []
    slots: list[tuple[int,int]] = []
    original_string,original_collector = archive.cstring,dl_tess.read_moRefGeomFeatureDataCollector
    def string() -> str:
        strings.append(archive.p)
        return original_string()
    def collector(current: object) -> object:
        slots.append((current.p-84 if kind=="part" else strings[-1]-4,len(current.map)))
        return original_collector(current)
    archive.cstring = string
    try:
        dl_tess.read_moRefGeomFeatureDataCollector = collector
        displaylists.display_lists(archive,kind)
    finally:
        dl_tess.read_moRefGeomFeatureDataCollector = original_collector
    assert archive.p==len(raw) and len(slots)==1 and raw[slots[0][0]:slots[0][0]+4]==integers(0)
    return slots[0]


def require_stable_tags(document: swcontainer.Document, path: Path, raw: bytes, start: int, index: int) -> None:
    for offset,_,_,_ in collect(document,path,raw):
        if offset<start:
            continue
        tag, = struct.unpack_from("<H",raw,offset)
        assert tag<0x7fff and tag<index, (offset,tag,index)


def modern_root(kind: str, empty: bool, edited: bool) -> bytes:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    raw = document.read("Contents/DisplayLists") if kind=="part" else Path("tests/fixtures/display-lists/owner-cycle-assembly.bin").read_bytes()
    start,index = fixture_slot(document,path,raw,kind)
    require_stable_tags(document,path,raw,start,index)
    builder = ObjectBuilder(index)
    payload = builder.object("moGraphicsConnectRefMgrData_c",lambda state:manager(state,19000,state.next_index-1,empty,edited))
    return raw[:start]+integers(-19)+payload+raw[start+4:]


def old_scene(builder: ObjectBuilder, viz: int) -> bytes:
    identity = builder.next_index+1
    raw = builder.object("moNodeName_c",lambda _:cstring("graphics scene 鋼",True))
    raw += integers(-1,2,-3,4)+cstring("tip",True)+integers(0)+ref(viz)
    node = builder.object("moGraphicsConnectRefMgrData_c",lambda state:manager(state,11000,state.next_index-1,False,False))
    raw += node_list([node],-72,True)+builder.object("pwSceneData_c",lambda state:pw_scene(state,False))+integers(3)
    return raw+b"".join(wall(kind,viz) for kind in [1,2,3])+floor(identity,False,0x7ff0000000000000)[:-20]


def old_root() -> bytes:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    start,index = old_prefix(document,path,document.read("Contents/DisplayLists"))
    original = old_shaded.old_scene
    try:
        old_shaded.old_scene = old_scene
        return start+old_shaded.shaded(ObjectBuilder(index),11000,"part",False)
    finally:
        old_shaded.old_scene = original


def reference_trace(raw: bytes, version: int, kind: str) -> bytes:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    previous = dl_tess.EXTERN.get("mgPlane")
    try:
        dl_tess.EXTERN["mgPlane"] = plane_reader
        root = displaylists.display_lists(archive,kind)
    finally:
        if previous is None:
            dl_tess.EXTERN.pop("mgPlane",None)
        else:
            dl_tess.EXTERN["mgPlane"] = previous
    assert archive.p==len(raw)
    return graphics_corpus.trace(root)


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw = old_root() if version==11000 else modern_root(kind,scenario==2,scenario==1)
    trace = reference_trace(raw,version,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace
    return framed,{"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines())}


def main() -> None:
    cases = [generate(19000,"part",0),generate(19000,"assembly",0),generate(19000,"part",2),generate(19000,"part",1),generate(11000,"part",0)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/graphics-references.bin").write_bytes(framed)
    evidence_path = Path("../inventor-interface/solidworks/research/named-plane-native-evidence.json")
    plane = json.loads(evidence_path.read_text())
    evidence = {"scope":"wire graph identity only; no CAD/application validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest(),"plane":{"layout":"root point then normal, six f64, 48 bytes","evidence_sha256":hashlib.sha256(evidence_path.read_bytes()).hexdigest(),"module_sha256":plane["sha256"],"function":plane["function"],"address":plane["start"]}}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_tess.py","mfcobjects.py","swarchive.py"]}
    Path(__file__).with_name("graphics-references.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
