"""Complete independently converted 4800-range roots across every component metadata gate."""
import json
import hashlib
from pathlib import Path
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_early_appearance_roots import item_bytes,face_identity_trace
from generate_early_view_roots import entry_budget
from early_appearance_patches import rv_source
from generate_earliest_scene_roots import raw_nan_reader
from earliest_component_patches import convert
from swarchive import Archive
from document_context_wire import document
from reference_corpus import frame_at
from earliest_dependency_wire import populate

VERSIONS = [4800,0x12c3,0x12c4,0x12ce,0x12cf,0x12d0,0x12d1,0x12d3,0x12d4,
            0x12db,0x12dc,0x12dd,0x12de,0x12df,0x12e0,0x12e2,0x12e3,0x12e8,0x12e9,4850]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    raw = convert_root(document,path,rv_source(document,path,raw),version,kind,variant%2,convert,b"",item_bytes(version))
    return populate(document,path,raw,version) if variant in (2,3) else raw


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%16
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_root(version,kind,variant,converted,entry_count,face_identity_trace)
    finally:Archive.f32 = original
    row["scenario"] = 208+variant
    return framed[:5]+bytes([208+variant])+framed[6:],row


def entry_count(version: int, kind: str, variant: int) -> int:
    populated = (2*int(version>0x12df)+4*int(version>0x12e2)) if variant in (2,3) else 0
    return entry_budget(version,kind,variant%2)+2+populated


def write_documents(root_generator: object = generate, version: int = 4800, prefix: str = "earliest-component") -> None:
    rows = []
    for kind in ["part","assembly"]:
        framed,_ = root_generator(version,kind)
        raw,_ = frame_at(framed,6)
        owned = document([("Contents/DisplayLists",raw),(f"_DL_VERSION_{version}/DLUpdateStamp",b""),("Unrelated/Payload",b"retained\0payload")])
        Path(f"tests/fixtures/display-lists/{prefix}-document-{kind}.bin").write_bytes(owned)
        rows.append({"kind":kind,"display_version":version,"file_version":19000,"directory_version":19000,"sha256":hashlib.sha256(owned).hexdigest()})
    Path(__file__).with_name(prefix+"-documents.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")


if __name__=="__main__":
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(v,kind) for v in VERSIONS]+[generate(4800,kind,1)]
        if kind=="assembly":cases += [generate(v,kind,2) for v in [0x12df,0x12e0,0x12e2,0x12e3,4850]]+[generate(0x12e3,kind,3)]
        totals.append(write_group("earliest-component-"+kind,cases))
    write_documents()
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
