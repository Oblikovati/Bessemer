"""Independently encode markup-ink grids inside complete owned display-list roots."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists, swcontainer
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_blank_scalars import flatten
from generate_version_range import generate as version_root, capture_plain, apply_patches
import dl_doc


def dimensions(scenario: int) -> tuple[int,int]:
    return {2:(0,7),3:(2,0),4:(0,0),5:(1,1),6:(1,0)}.get(scenario,(2,2))


def text(value: str, minimal: bool) -> bytes:
    return b"\0" if minimal else cstring(value,True)


def ink_view(version: int, configs: int, scenario: int, index: int) -> bytes:
    minimal = scenario in (5,6)
    raw = text(f"ink {index} 鋼😀\0",minimal)+integers(-17 if not minimal else 0)
    for config in range(configs):
        shown = 23 if scenario==1 and index==0 and config==0 else -3
        raw += text(f"cfg {config} é\0",minimal)+integers(-config-7,(-2147483648,2147483647)[config%2],shown)
    if version>0x332b:
        raw += text(f"tail {index} λ\0",minimal)
    return raw


def ink_body(version: int, scenario: int) -> bytes:
    views,configs = dimensions(scenario)
    return integers(views,configs)+b"".join(ink_view(version,configs,scenario,index) for index in range(views))


def source(version: int, kind: str) -> tuple[Path,object,bytes,int]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    if version==18000:
        framed,_ = version_root(version)
        length, = struct.unpack_from("<I",framed,5)
        return path,document,framed[9:9+length],67
    return path,document,document.read("Contents/DisplayLists"),52


def replace_ink(path: Path, document: object, raw: bytes, version: int, kind: str, scenario: int) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    original = dl_doc.read_MarkUpInkViews
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    try:
        dl_doc.read_MarkUpInkViews = capture_plain(original,ink_body(version,scenario),patches)
        displaylists.display_lists(archive,kind)
    finally:
        dl_doc.read_MarkUpInkViews = original
    assert archive.p==len(raw) and len(patches)==1
    return apply_patches(raw,patches)


def semantic_trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    flatten("root.markup_ink_views.views",root["markup_ink_views"],rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    path,document,raw,base_entries = source(version,kind)
    raw = replace_ink(path,document,raw,version,kind,scenario)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version = version
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = semantic_trace(root)
    views,configs = dimensions(scenario)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<II",0,len(trace))+trace
    evidence = {"version":version,"kind":kind,"scenario":scenario,"views":views,"configs":configs,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"trace_fields":len(trace.splitlines()),"new_objects":sum(archive.classes.values()),"aggregate_entries":base_entries+views+configs+views*configs}
    return framed,evidence


def main() -> None:
    cases = [generate(19000,"part",scenario) for scenario in range(7)]+[generate(19000,"assembly",0),generate(18000,"part",0)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/markup-ink.bin").write_bytes(framed)
    evidence = {"scope":"complete wire roots with explicit file context 19000; application validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","displaylists.py","swarchive.py"]}
    Path(__file__).with_name("markup-ink.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
