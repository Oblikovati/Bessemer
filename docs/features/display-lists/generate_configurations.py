"""Freeze independent complete multi-configuration display roots, graphs and scalar oracles."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_shaded_vectors import ObjectBuilder
from generate_version_range import generate as version_root
from generate_old_shaded import generate as old_root
from generate_item_vectors import view_slot
from generate_component_vectors import integers
from configuration_wire import configurations
from configuration_corpus import trace


def source(version: int, kind: str) -> tuple[Path,object,bytes]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    if version in (10000,11000):
        framed,_ = old_root(version,kind)
        length, = struct.unpack_from("<I",framed,6)
        return path,document,framed[10:10+length]
    if version==18000:
        framed,_ = version_root(version)
        length, = struct.unpack_from("<I",framed,5)
        return path,document,framed[9:9+length]
    return path,document,document.read("Contents/DisplayLists")


def without_views(path: Path, document: object, raw: bytes) -> bytes:
    start,end = view_slot(document,path,raw)
    assert raw[4:8]==integers(1)
    return raw[:4]+integers(0)+raw[8:start]+raw[end:]


def reference(path: Path, document: object, raw: bytes, version: int, kind: str) -> tuple[object,dict[str,object]]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    archive.face_tess_comp_ids = {"component 鋼"} if version>=18000 and kind=="assembly" else set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    return archive,root


def derive(path: Path, document: object, raw: bytes, version: int, kind: str, scenario: int) -> bytes:
    archive,root = reference(path,document,raw,version,kind)
    assert root["configurations"]["config_count"]==0
    assert raw[-4:]==integers(0)
    viz = next(index for index,value in enumerate(archive.map) if value is root["visual_properties"])
    body = configurations(ObjectBuilder(len(archive.map)),version,kind,root["view_count"],viz,scenario)
    return raw[:-4]+body


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    path,document,raw = source(version,kind)
    if scenario==5:
        raw = without_views(path,document,raw)
    raw = derive(path,document,raw,version,kind,scenario)
    archive,root = reference(path,document,raw,version,kind)
    scalar = trace(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<II",0,len(scalar))+scalar
    evidence = {"version":version,"kind":kind,"scenario":scenario,"views":root["view_count"],"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"scalar_fields":len(scalar.splitlines()),"objects":sum(archive.classes.values()),"external_ids":sorted(archive.face_tess_comp_ids),"aggregate_entries":entry_budget(version,kind,scenario)}
    return framed,evidence


def entry_budget(version: int, kind: str, scenario: int) -> int:
    if version in (10000,11000):
        return 133 if version==10000 else 141
    if version==18000:
        return 175
    if scenario in (2,3):
        return 53
    if scenario==4:
        return 59
    if scenario==5:
        return 153 if kind=="part" else 169
    return 160 if kind=="part" else 176


def main() -> None:
    cases = [generate(19000,kind,scenario) for kind in ["part","assembly"] for scenario in [0,1,2,3,4,5]]
    cases += [generate(version,"assembly",0) for version in [10000,11000]]+[generate(18000,"part",0)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/configurations.bin").write_bytes(framed)
    evidence = {"scope":"complete wire graph reconstruction; explicit file context 19000; CAD application validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_tess.py","dl_doc.py","displaylists.py","mfcobjects.py"]}
    Path(__file__).with_name("configurations.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
