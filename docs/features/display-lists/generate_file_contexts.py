"""Complete roots with native file context independent of display-list version."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from file_context_wire import bucket_root,entry_budget
import reference_corpus
import scene_corpus
import item_corpus
import bucket_corpus
import light_corpus
import material_corpus

FILE_VERSIONS = [0,0x223,0x224,18000,18999,19000,0xffffffff]


def reference(document: object, path: Path, raw: bytes, version: int, file_version: int, kind: str) -> tuple[object,bytes]:
    archive = displaylists.archive_for(document,str(path),raw)
    archive.version,archive.file_version,archive.doc_kind,archive.face_tess_comp_ids = version,file_version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(raw)
    expected = b"".join(module.trace(root) for module in [reference_corpus,scene_corpus,item_corpus,bucket_corpus,light_corpus,material_corpus])
    return archive,expected


def generate(version: int, kind: str, scenario: int, file_version: int) -> tuple[bytes,dict[str,object]]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    raw = document.read("Contents/DisplayLists")
    if scenario!=4:
        raw = bucket_root(document,path,raw,version,file_version,scenario)
    else:
        assert version==19000
    archive,expected = reference(document,path,raw,version,file_version,kind)
    entries = 53 if scenario==4 else entry_budget(version,file_version,scenario)
    objects = sum(archive.classes.values())
    framed = struct.pack("<IIBBII",version,file_version,kind=="assembly",scenario,entries,objects)
    framed += struct.pack("<I",len(raw))+raw+struct.pack("<I",len(expected))+expected
    row = {"version":version,"file_version":file_version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(expected.splitlines()),"aggregate_entries":entries,"objects":objects}
    return framed,row


def cases(version: int) -> list[tuple[bytes,dict[str,object]]]:
    result = [generate(version,"part",scenario,file_version) for file_version in FILE_VERSIONS for scenario in [0,16]]
    result += [generate(version,"assembly",scenario,file_version) for file_version in [0,0x224] for scenario in [0,16]]
    result += [generate(version,"part",scenario,file_version) for file_version in [0,0x224] for scenario in [1,17,2]]
    return result


def write_group(name: str, records: list[tuple[bytes,dict[str,object]]]) -> tuple[int,int]:
    framed = struct.pack("<I",len(records))+b"".join(raw for raw,_ in records)
    Path(f"tests/fixtures/display-lists/{name}.bin").write_bytes(framed)
    report = {"scope":"complete wire roots with explicit unsigned file context; CAD validity unclaimed","fixtures":[row for _,row in records],"sha256":hashlib.sha256(framed).hexdigest()}
    report["reader_sha256"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}.py").read_bytes()).hexdigest() for name in ["dl_ui","displaylists","dl_doc","dl_tess","dl_light"]}
    Path(__file__).with_name(name+".json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    return len(records),sum(row["oracle_fields"] for _,row in records)


def main() -> None:
    totals = [write_group(f"file-context-{version}",cases(version)) for version in [9000,16000,19000]]
    records = [generate(19000,kind,4,file_version) for file_version in FILE_VERSIONS for kind in ["part","assembly"]]
    totals.append(write_group("file-context-shaded",records))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
