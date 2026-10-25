"""Independent null-bitmap textured-plane records; unresolved bitmap bodies are excluded."""
from pathlib import Path
import hashlib
import json
import struct
from decal_sources import source,displaylists
from generate_transform_vectors import compact
from generate_component_vectors import integers
from generate_version_range import capture_plain,apply_patches
from generate_blank_scalars import flatten
import dl_doc


def variants(scenario: int) -> list[tuple[int,int]]:
    if scenario==2:
        return [(0,0)]
    if scenario==3:
        return []
    if scenario==4:
        return [(presence,flags) for presence in [0,1,2,255] for flags in range(256)]
    return [(0,0),(2,0x1f),(255,0x2f),(1,0x4f)]


def body(scenario: int) -> bytes:
    records: list[bytes] = []
    for index,(presence,flags) in enumerate(variants(scenario)):
        transform = compact(presence,flags)
        if scenario==1 and index==0:
            transform = transform[:1]+struct.pack("<d",23.0)+transform[9:]
        records.append(b"\0\0"+transform)
    return integers(len(records))+b"".join(records)


def derive(document: object, path: Path, raw: bytes, version: int, kind: str, scenario: int) -> tuple[bytes,int]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    patches: list[tuple[int,int,bytes]] = []
    original = dl_doc.read_TexturedPlanes
    def capture(current: object, is_drawing: bool = False) -> object:
        return capture_plain(lambda current:original(current,is_drawing),body(scenario),patches)(current)
    try:
        dl_doc.read_TexturedPlanes = capture
        displaylists.display_lists(archive,kind)
    finally:
        dl_doc.read_TexturedPlanes = original
    assert archive.p==len(raw) and len(patches)==1
    return apply_patches(raw,patches),patches[0][0]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    path,document,raw,base = source(version,kind)
    raw,start = derive(document,path,raw,version,kind,scenario)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw) and all(item["sketch_bitmap"] is None for item in root["textured_planes"])
    rows: dict[str,str] = {}
    for index,plane in enumerate(root["textured_planes"]):
        flatten(f"root.textured_planes.items[{index}].xform",plane["xform"],rows)
    trace = "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",0)
    evidence = {"version":version,"kind":kind,"scenario":scenario,"planes":len(variants(scenario)),"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"transform_fields":len(rows),"aggregate_entries":base+len(variants(scenario)),"objects":sum(archive.classes.values()),"count_offset":start,"external_ids":[]}
    return framed,evidence


def main() -> None:
    cases = [generate(version,kind,0) for version in [7000,9000,10000,11000,19000] for kind in ["part","assembly"]]
    cases += [generate(18000,"part",0)]+[generate(19000,"part",scenario) for scenario in [1,2,3,4]]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/textured-planes.bin").write_bytes(framed)
    evidence = {"scope":"null bitmap pointers with complete transforms only; new bitmap bodies remain unsupported; no CAD validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","displaylists.py","mfcobjects.py"]}
    Path(__file__).with_name("textured-planes.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
