"""Owned complete drawing streams, populated direct lists and shared MFC aliases."""
from verify_corpus import reference,frames,verify
from pathlib import Path
import struct
import json
import hashlib
from generate_shaded_vectors import ObjectBuilder
import generate_shaded_vectors as shaded
from bucket_vector_wire import view_bucket
from generate_named_vectors import cstring
from swarchive import Archive
import facettess
from document_context_wire import document


def drawing_facet() -> bytes:
    raw = Path("tests/fixtures/tessellation/facet-4a38-1.bin").read_bytes()
    archive = Archive(raw,19000)
    original,spans = archive.i64,[]
    def capture() -> int:
        start = archive.p
        value = original()
        spans.append((start,value))
        return value
    archive.i64 = capture
    parsed = facettess.facet_container(archive)
    assert archive.p==len(raw) and len(spans)==1 and spans[0][1]==len(parsed["edge_points"])
    start,count = spans[0]
    return raw[:start]+raw[start+8+count*28:]


def part(builder: ObjectBuilder) -> bytes:
    original = shaded.face
    facet = drawing_facet()
    full = Path("tests/fixtures/tessellation/facet-4a38-1.bin").read_bytes()
    shaded.face = lambda external,index:facet+original(False,index)[len(full):]
    try:return shaded.part(builder,False)
    finally:shaded.face = original


def direct_strings(values: list[str], wide: bool) -> bytes:
    return struct.pack("<i",len(values))+b"".join(cstring(value,wide) for value in values)


def model(builder: ObjectBuilder, is_part: bool, edited: bool) -> bytes:
    raw = struct.pack("<i",is_part)+cstring("edited 鋼" if edited else "model 鋼",True)
    raw += struct.pack("<HiH",65535,-7,0)+bytes(2)
    return raw+(part(builder) if is_part else bytes(2))


def generate(scenario: int) -> bytes:
    if scenario==0:return bytes(8)
    builder = ObjectBuilder(1)
    bucket = builder.object("uiViewBucket_c",lambda state:view_bucket(state,2,scenario==4,scenario==1))
    if scenario==1:return struct.pack("<i",1)+bucket+bytes(4)
    raw = struct.pack("<i",3)+bucket+struct.pack("<HH",2,0)+struct.pack("<i",-2147483648)
    raw += struct.pack("<IIIIII",0xffffffff,0,1,0,0xffffffff,0x80000000)
    for wide in [False,True]:
        raw += direct_strings(["", "list\0"],wide)+direct_strings([],wide)+direct_strings(["鋼" if wide else "narrow"],wide)
    raw += struct.pack("<i",2)+model(builder,True,scenario==3)+model(builder,False,False)
    for group in range(3):
        raw += struct.pack("<iIIiiI",2,0xffffffff,group+int(scenario==3),0,1,0x80000000)
    return raw


def documents(destination: Path) -> None:
    contexts = {"document":[("_MO_VERSION_19000/History",b""),("_DL_VERSION_7000/DLUpdateStamp",b"")],
        "missing":[("_DL_VERSION_19000/DLUpdateStamp",b"")],
        "conflicting":[("_MO_VERSION_19000/History",b""),("_MO_VERSION_18000/History",b"")],
        "alias":[("_MO_VERSION_19000/History",b""),("_MO_VERSION_019000/Other",b"")]}
    for name,markers in contexts.items():
        raw = document([("Contents/VBLists",generate(2)),("Unrelated/Payload",b"retained\0payload")]+markers)
        (destination/(name+".bin")).write_bytes(raw)


if __name__=="__main__":
    cases = [generate(index) for index in range(5)]
    destination = Path("tests/fixtures/drawing-buckets")
    destination.mkdir(exist_ok=True)
    documents(destination)
    (destination/"vectors.bin").write_bytes(frames(cases))
    (destination/"fields.bin").write_bytes(frames([reference(raw) for raw in cases]))
    report = {"fixtures":[{"scenario":index,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"fields":len(reference(raw).splitlines())} for index,raw in enumerate(cases)]}
    Path(__file__).with_name("vectors.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(report))
