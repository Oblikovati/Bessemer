"""Complete owned roots around old appearance, face and wall layout transitions."""
import json
import struct
import hashlib
from pathlib import Path
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_earliest_faces import items,entry_count as face_entries
from generate_earliest_components import write_documents
from generate_earliest_lights import extra_trace
from generate_earliest_scene_roots import raw_nan_reader
from early_appearance_patches import rv_source
from earliest_wall_patches import convert
from swarchive import Archive
from swarchive import ArchiveError
from reference_corpus import frame_at
from generate_alias_fixture import displaylists,swcontainer
import dl_light

VERSIONS = [4400,0x1164,0x1165,0x1179,0x117a,0x117c,0x117d,0x117e,0x117f,4500]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    return convert_root(document,path,rv_source(document,path,raw),version,kind,variant,convert,b"",items(version))


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_root(version,kind,scenario%16,converted,face_entries,extra_trace)
    finally:Archive.f32 = original
    row["scenario"] = 192+scenario%16
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


def write_unresolved() -> None:
    framed,_ = generate(4400,"part")
    raw,_ = frame_at(framed,6)
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    positions = wall_positions(document,path,raw)
    name = "moPWMaterialProperties_c"
    prefix = struct.pack("<HHH",0xffff,1,len(name))+name.encode()
    raw = raw[:positions[0]]+prefix+raw[positions[0]+2:]
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = 4400
    try:displaylists.display_lists(archive,"part")
    except ArchiveError as error:assert "no reader for class "+name in str(error)
    else:raise AssertionError("expected the unresolved wall body to be rejected")
    Path("tests/fixtures/display-lists/earliest-wall-unresolved.bin").write_bytes(raw)
    report = {"sha256":hashlib.sha256(raw).hexdigest(),"version":4400,"runtime_class":name,"expected_failure":"Unsupported; body layout unresolved"}
    Path(__file__).with_name("earliest-wall-unresolved.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")


def wall_positions(document: object, path: object, raw: bytes) -> list[int]:
    original,positions = dl_light.read_swWall_c,[]
    def read(archive: object) -> object:
        positions.append(archive.p)
        return original(archive)
    dl_light.read_swWall_c = read
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        archive.version = 4400
        displaylists.display_lists(archive,"part")
        assert archive.p==len(raw) and len(positions)==3
    finally:dl_light.read_swWall_c = original
    return positions


if __name__=="__main__":
    totals = [write_group("earliest-wall-"+k,[generate(v,k) for v in VERSIONS]+[generate(4400,k,1)],("displaylists.py",)) for k in ["part","assembly"]]
    write_documents(generate,4400,"earliest-wall")
    write_unresolved()
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
