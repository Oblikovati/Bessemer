"""Independent complete documents with explicit display and tessellation context expectations."""
from pathlib import Path
import hashlib
import json
import struct
import tempfile
from document_context_wire import document
from generate_named_vectors import cstring
from generate_component_vectors import integers
from generate_alias_fixture import swcontainer,displaylists
import facettess
from mfcobjects import ObjectArchive
import dl_light
from middle_range_patches import ambient_capture
from generate_version_range import apply_patches

ROOT = Path("tests/fixtures/display-lists")


def directory_entry(version: int, identity: str, index: int) -> bytes:
    raw = integers(index)+cstring(identity,True)+struct.pack("<I",0xfedcba98)
    raw += struct.pack("<IIIIII",0x11223344,0xaabbccdd,0xdeadbeef,0x11223344,0x88776655,0x44332211)
    raw += cstring("000-000-007")+integers(index)
    if version>=0x3b23:return raw+integers(1,-1,23,7)
    return raw+integers(-3,4) if version>=0x3b1c else raw


def directory(version: int, empty: bool = False) -> bytes:
    names = [] if empty else ["Assem1","Assem1","assem1","cache 鋼"]
    return integers(2,7,len(names))+b"".join(directory_entry(version,name,index) for index,name in enumerate(names))


def source(kind: str, external: bool) -> bytes:
    if external:return (ROOT/"external-shaded-assembly.bin").read_bytes()
    path = "tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM"
    return swcontainer.Document(path).read("Contents/DisplayLists")


def context_archive(payload: bytes, version: int, kind: str, file_version: int, identities: list[str]) -> ObjectArchive:
    archive = ObjectArchive(payload,version)
    archive.doc_kind,archive.file_version,archive.face_tess_comp_ids = kind,file_version,set(identities)
    return archive


def independent_edit(payload: bytes, version: int, kind: str, file_version: int, identities: list[str]) -> bytes:
    patches: list[tuple[int,int,bytes]] = []
    original = dl_light.read_uoMaterialProperties
    dl_light.read_uoMaterialProperties = ambient_capture(original,True,patches)
    try:
        archive = context_archive(payload,version,kind,file_version,identities)
        root = displaylists.display_lists(archive,kind)
        assert archive.p==len(payload) and len(patches)==1
    finally:dl_light.read_uoMaterialProperties = original
    edited = apply_patches(payload,patches)
    archive = context_archive(edited,version,kind,file_version,identities)
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(edited) and root["visual_properties"]["material"]["ambient"]==23.0
    return edited


def verified_context(raw: bytes, version: int, kind: str, file_version: int, directory_version: int) -> list[str]:
    with tempfile.TemporaryDirectory(prefix="bessemer-context-") as folder:
        path = Path(folder)/"owned.SLDASM"
        path.write_bytes(raw)
        parsed = swcontainer.Document(str(path))
        ids = sorted({entry["comp_id"] for entry in facettess.directory(parsed.read("FaceTessellations/Directory"),directory_version)["entries"]}) if "FaceTessellations/Directory" in parsed.names() else []
        archive = context_archive(parsed.read("Contents/DisplayLists"),version,kind,file_version,ids)
        displaylists.display_lists(archive,kind)
        assert archive.p==len(archive.b)
        return ids


def write_case(name: str, kind: str, file_version: int, directory_version: int, external: bool = False, empty: bool = False) -> dict[str,object]:
    payload = source(kind,external)
    streams = [("_DL_VERSION_19000/DLUpdateStamp",b""),("Contents/DisplayLists",payload),("unrelated",b"retained")]
    if external or empty:streams.append(("FaceTessellations/Directory",directory(directory_version,empty)))
    raw = document(streams)
    ids = verified_context(raw,19000,kind,file_version,directory_version)
    (ROOT/(name+".bin")).write_bytes(raw)
    edited = independent_edit(payload,19000,kind,file_version,ids)
    (ROOT/(name+"-edited-display.bin")).write_bytes(edited)
    return {"edited_display_sha256":hashlib.sha256(edited).hexdigest(),"fixture":name,"kind":kind,"display_version":19000,"file_version":file_version,"directory_version":directory_version,"component_ids":ids,"external_faces":2 if external else 0,"sha256":hashlib.sha256(raw).hexdigest()}


def write_marker_case(name: str, markers: list[str]) -> dict[str,object]:
    streams = [(marker,b"") for marker in markers]+[("Contents/DisplayLists",b"")]
    raw = document(streams)
    (ROOT/(name+".bin")).write_bytes(raw)
    return {"fixture":name,"markers":markers,"sha256":hashlib.sha256(raw).hexdigest()}


def write_crc_case() -> dict[str,object]:
    raw = bytearray((ROOT/"context-external-modern.bin").read_bytes())
    parsed = swcontainer.Document(raw)
    entry = parsed.entry("FaceTessellations/Directory")
    bad_crc = entry.crc^0xffffffff
    struct.pack_into("<I",raw,entry.local_at+14,bad_crc)
    struct.pack_into("<I",raw,entry.central[0]+16,bad_crc)
    try:swcontainer.Document(raw).read("FaceTessellations/Directory")
    except ValueError as error:assert "CRC mismatch" in str(error)
    else:raise AssertionError("corrupt directory CRC was accepted")
    (ROOT/"context-directory-bad-crc.bin").write_bytes(raw)
    return {"fixture":"context-directory-bad-crc","sha256":hashlib.sha256(raw).hexdigest(),"expected":"Integrity"}


if __name__=="__main__":
    rows = [write_case("context-inline-part","part",0x223,19000),write_case("context-inline-assembly","assembly",0x224,19000),write_case("context-external-modern","assembly",19000,19000,True),write_case("context-external-pair","assembly",0xffffffff,0x3b1c,True),write_case("context-empty-directory","assembly",0,19000,empty=True)]
    markers = [("context-no-marker",[]),("context-conflict",["_DL_VERSION_19000/DLUpdateStamp","_DL_VERSION_18999/DLUpdateStamp"]),("context-invalid-marker",["_DL_VERSION_nope/DLUpdateStamp"]),("context-signed-aliases",["_DL_VERSION_-1/DLUpdateStamp","_DL_VERSION_4294967295/DLUpdateStamp"]),("context-lookalike",["nested/_DL_VERSION_19000/DLUpdateStamp","_DL_VERSION_19000/notDLUpdateStamp"])]
    report = {"scope":"document context declarations; external payload and geometry checks excluded","fixtures":rows,"marker_cases":[write_marker_case(name,names) for name,names in markers],"integrity_case":write_crc_case()}
    Path(__file__).with_name("document-contexts.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({"fixtures":len(rows)+len(markers)+1}))
