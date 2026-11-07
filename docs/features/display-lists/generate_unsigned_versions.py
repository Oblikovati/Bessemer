"""Complete rich archives under the specification's normalized unsigned version rules."""
from pathlib import Path
from functools import lru_cache
import json
import struct
import hashlib
from middle_range_source import source,component_trace,displaylists
from verify_configuration_processes import records as configurations
from generate_earlier_range import write_group
from generate_earliest_scene_roots import raw_nan_reader
from swarchive import Archive
from mfcobjects import ObjectArchive
import reference_corpus
import scene_corpus
import item_corpus
import bucket_corpus
import light_corpus
import material_corpus
import configuration_corpus
from unsigned_version_cases import VERSIONS
from document_context_wire import document
from generate_bucket_vectors import prefix_slot


@lru_cache(maxsize=1)
def profiles() -> tuple[tuple[str,bytes,tuple[str,...],str],...]:
    values = [(kind,source(kind)[2],(),"scene-"+kind) for kind in ["part","assembly"]]
    for name in ["bucket-3d","bucket-2d","shared-font-part"]:
        values.append(("part",Path(f"tests/fixtures/display-lists/{name}.bin").read_bytes(),(),name))
    for row,raw in configurations():
        if row["version"]==19000:
            values.append((row["kind"],raw,tuple(row["external_ids"]),f"configuration-{row['kind']}-{row['scenario']}"))
    assert len(values)==17
    return tuple(values)


def reference(raw: bytes, version: int, kind: str, identities: tuple[str,...]) -> dict[str,object]:
    archive = ObjectArchive(raw,version)
    archive.doc_kind,archive.file_version,archive.face_tess_comp_ids = kind,19000,set(identities)
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:root = displaylists.display_lists(archive,kind)
    finally:Archive.f32 = original
    assert archive.p==len(raw)
    return root


def traces(root: dict[str,object], kind: str) -> bytes:
    modules = [reference_corpus,scene_corpus,item_corpus,bucket_corpus,light_corpus,material_corpus]
    return b"".join(module.trace(named_configurations(root,kind)) for module in modules)


def named_configurations(root: dict[str,object], kind: str) -> dict[str,object]:
    if kind!="part" or "configurations" not in root:return root
    result,config = dict(root),dict(root["configurations"])
    # The reference returns part config records as (id, object), not named dictionaries.
    config["configs"] = [{"config_id":entry[0],"tessellation":entry[1]} for entry in config.get("configs",[])]
    result["configurations"] = config
    return result


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    actual_kind,raw,identities,name = profiles()[scenario]
    assert kind==actual_kind
    root = reference(raw,version,kind,identities)
    expected = traces(root,kind)
    scalar = component_trace(root,kind)
    if "configurations" in root:scalar += configuration_corpus.trace(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw
    framed += struct.pack("<I",len(expected))+expected+struct.pack("<I",len(scalar))+scalar
    row = {"version":version,"kind":kind,"scenario":scenario,"profile":name,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(expected.splitlines())+len(scalar.splitlines()),"external_ids":list(identities),"file_version":19000}
    return framed,row


def write_documents() -> None:
    rows = []
    for kind in ["part","assembly"]:
        path,original,raw = source(kind)
        start,_ = prefix_slot(original,path,raw)
        edited = raw[:start]+struct.pack("<i",23)+raw[start+4:]
        assert reference(edited,0xffffffff,kind,())["views"][0]["view_index"]==23
        Path(f"tests/fixtures/display-lists/unsigned-edited-{kind}.bin").write_bytes(edited)
        streams = [("Contents/DisplayLists",raw),("_DL_VERSION_-1/DLUpdateStamp",b""),("_DL_VERSION_4294967295/DLUpdateStamp",b""),("_MO_VERSION_19000/History",b""),("Unrelated/Payload",b"retained\0payload")]
        owned = document(streams)
        Path(f"tests/fixtures/display-lists/unsigned-document-{kind}.bin").write_bytes(owned)
        rows.append({"kind":kind,"display_version":0xffffffff,"file_version":19000,"directory_version":19000,"sha256":hashlib.sha256(owned).hexdigest()})
    Path(__file__).with_name("unsigned-documents.json").write_text(json.dumps({"fixtures":rows},indent=2)+"\n",encoding="utf-8",newline="\n")


def compact_manifest(path: Path) -> None:
    manifest = json.loads(path.read_text())
    fixtures = manifest.pop("fixtures")
    rows = ",\n".join("    "+json.dumps(row) for row in fixtures)
    text = json.dumps(manifest,indent=2)[:-2]+",\n  \"fixtures\": [\n"+rows+"\n  ]\n}\n"
    path.write_text(text,encoding="utf-8",newline="\n")


if __name__=="__main__":
    cases = [generate(version,kind,index) for version in VERSIONS for index,(kind,_,_,_) in enumerate(profiles())]
    write_documents()
    print(write_group("unsigned-versions",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("unsigned-versions.json"))
