"""Complete part/assembly roots through the 13000..13999 appearance and annotation gates."""
from pathlib import Path
import hashlib
import json
import struct
from middle_range_source import source,displaylists
from lower_range_patches import texture_source,convert
from generate_lower_range import entry_budget as base_budget,scalar_trace
from generate_version_range import capture_plain
from generate_component_vectors import integers
from generate_markup_ink import ink_view,ink_body
from generate_blank_scalars import flatten
from transition_item_wire import items
import dl_doc
import material_corpus
import scene_corpus
import item_corpus

VERSIONS = [13000,0x332b,0x332c,0x3330,0x3331,0x3332,0x3333,0x3334,0x3336,0x3337,0x333b,0x333c,0x3340,0x3341,0x3342,0x3344,0x3345,0x3347,0x3348,0x3350,0x3351,0x3363,0x3364,0x3368,0x3369,0x336b,0x336c,0x35e8,0x35e9,13999,14000]


def markup(version: int, variant: int) -> bytes:
    if variant==2:
        return integers(100,0)+b"".join(ink_view(version,0,6,index) for index in range(100))
    return ink_body(version,0)


def converted(document: object, path: Path, raw: bytes, version: int, variant: int) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    original_items,original_ink = dl_doc.read_display_item_list,dl_doc.read_MarkUpInkViews
    dl_doc.read_display_item_list = capture_plain(original_items,items(version),patches)
    if variant in (2,3):
        dl_doc.read_MarkUpInkViews = capture_plain(original_ink,markup(version,variant),patches)
    try:
        return convert(document,path,raw,version,variant==1,patches)
    finally:
        dl_doc.read_display_item_list,dl_doc.read_MarkUpInkViews = original_items,original_ink


def ink_trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    flatten("root.markup_ink_views.views",root["markup_ink_views"],rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%64
    path,document,raw = source(kind)
    raw = converted(document,path,texture_source(document,path,raw),version,variant)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = material_corpus.trace(root)+scene_corpus.trace(root)+item_corpus.trace(root)
    scalar = scalar_trace(root,kind,version)+ink_trace(root)
    framed = struct.pack("<IBBI",version,kind=="assembly",64+variant,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",len(scalar))+scalar
    entries = base_budget(version,kind)+12+(99 if variant==2 else 7 if variant==3 else 0)
    row = {"version":version,"kind":kind,"scenario":64+variant,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(trace.splitlines())+len(scalar.splitlines()),"aggregate_entries":entries,"objects":sum(archive.classes.values())}
    return framed,row


def write_kind(kind: str) -> tuple[int,int]:
    cases = [generate(version,kind) for version in VERSIONS]+[generate(13000,kind,1),generate(13000,kind,2)]+[generate(version,kind,3) for version in [0x332b,0x332c]]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path(f"tests/fixtures/display-lists/transition-range-{kind}.bin").write_bytes(framed)
    evidence = {"scope":"complete wire roots with populated items and versioned ink views; file context 19000; CAD validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_tess.py","dl_light.py","dl_ui.py","facettess.py","mfcobjects.py"]}
    Path(__file__).with_name(f"transition-range-{kind}.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    return len(cases),sum(row["oracle_fields"] for _,row in cases)


def main() -> None:
    totals = [write_kind(kind) for kind in ["part","assembly"]]
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
