"""Complete roots for the 12000-range collectors, ink and configuration-tail gates."""
from pathlib import Path
import hashlib
import json
import struct
from middle_range_source import source,displaylists
from lower_range_patches import texture_source
from earlier_range_patches import convert,configurations
from generate_version_range import capture_plain
from transition_item_wire import items
from generate_transition_range import markup,ink_trace
from generate_lower_range import scalar_trace
from configuration_corpus import trace as configuration_trace
import dl_doc
import dl_tess
import material_corpus
import scene_corpus
import item_corpus

VERSIONS = sorted({12000,0x2f44,0x2f45,0x2f5d,0x2f5e,0x2f62,0x2f63,0x2f7a,0x2f7b,0x2f80,0x2f81,0x2f8b,0x2f8c,0x2f95,0x2f96,0x2f99,0x2f9a,0x2fa5,0x2fa6,0x2fa9,0x2faa,0x2fab,0x2fac,0x2fad,12999,13000})


def converted(document: object, path: Path, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    bindings = [(dl_doc,"read_display_item_list"),(dl_doc,"read_MarkUpInkViews"),(dl_tess,"multiple_configs")]
    originals = [getattr(module,name) for module,name in bindings]
    ink = b"" if version<=0x2f80 else markup(version,variant) if variant in (2,3) else struct.pack("<ii",0,1)
    def config_reader(archive: object, doc_kind: str, restore: int = 1, n_views: int = 0) -> object:
        return capture_plain(lambda current:originals[2](current,doc_kind,restore,n_views),configurations(version,kind) if variant==4 else struct.pack("<i",0),patches)(archive)
    readers = [capture_plain(originals[0],items(version),patches),capture_plain(originals[1],ink,patches),config_reader]
    try:
        for (module,name),reader in zip(bindings,readers,strict=True):setattr(module,name,reader)
        return convert(document,path,raw,version,variant==1,patches)
    finally:
        for (module,name),reader in zip(bindings,originals,strict=True):setattr(module,name,reader)


def entry_budget(version: int, kind: str, variant: int) -> int:
    result = 135 if kind=="part" else 140
    result -= 1 if version<=0x2f80 else 0
    result -= 4 if kind=="assembly" and version<=0x2f44 else 0
    result += 99 if variant==2 else 7 if variant==3 else 0
    result += (7+(3 if version>0x2fa5 else 0)) if variant==4 else 0
    return result


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%128
    path,document,raw = source(kind)
    raw = converted(document,path,texture_source(document,path,raw),version,kind,variant)
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    trace = material_corpus.trace(root)+scene_corpus.trace(root)+item_corpus.trace(root)
    scalar = scalar_trace(root,kind,version)+configuration_trace(root,kind)+(ink_trace(root) if root["markup_ink_views"] is not None else b"")
    framed = struct.pack("<IBBI",version,kind=="assembly",128+variant,len(raw))+raw+struct.pack("<I",len(trace))+trace+struct.pack("<I",len(scalar))+scalar
    row = {"version":version,"kind":kind,"scenario":128+variant,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(trace.splitlines())+len(scalar.splitlines()),"aggregate_entries":entry_budget(version,kind,variant),"objects":sum(archive.classes.values())}
    return framed,row


def write_kind(kind: str) -> tuple[int,int]:
    cases = [generate(version,kind) for version in VERSIONS]+[generate(12000,kind,1),generate(0x2f81,kind,2)]+[generate(version,kind,4) for version in [0x2fa5,0x2fa6]]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path(f"tests/fixtures/display-lists/earlier-range-{kind}.bin").write_bytes(framed)
    evidence = {"scope":"complete owned wire roots with explicit file context 19000; CAD validity unclaimed","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_tess.py","dl_light.py","dl_ui.py","facettess.py","mfcobjects.py"]}
    Path(__file__).with_name(f"earlier-range-{kind}.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    return len(cases),sum(row["oracle_fields"] for _,row in cases)


def main() -> None:
    totals = [write_kind(kind) for kind in ["part","assembly"]]
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
