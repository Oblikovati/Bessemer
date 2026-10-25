"""Independent modern scene records in owned roots; graph fixtures do not claim model validity."""
from pathlib import Path
import hashlib
import json
import struct
from generate_shaded_vectors import ObjectBuilder, ShadedArchive, displaylists, swcontainer
from generate_named_vectors import cstring
from generate_transform_vectors import compact, doubles
from generate_component_vectors import integers
import dl_doc
import scene_corpus


def object_ref(index: int) -> bytes:
    return struct.pack("<H",index)


def floats(count: int) -> bytes:
    bits = [0,0x80000000,0x3f800000,0xbf800000,0x3e800000,0x3fc00000]
    return b"".join(struct.pack("<I",bits[index%len(bits)]) for index in range(count))


def node_list(payloads: list[bytes], tail: int, extended: bool) -> bytes:
    count = struct.pack("<HI",0xffff,len(payloads)) if extended else struct.pack("<H",len(payloads))
    return count+b"".join(payloads)+integers(tail)


def concept(builder: ObjectBuilder, viz: int, empty: bool) -> tuple[bytes,int]:
    name_id = builder.next_index+1
    raw = builder.object("moNodeName_c",lambda _:cstring("scene name 鋼",True))
    raw += integers(-1,2,-3,4)+cstring("tip λ\0",True)+integers(0 if empty else -17)
    if not empty:
        raw += node_list([object_ref(name_id)],-71,True)
    raw += object_ref(viz)
    nodes = [] if empty else [object_ref(viz),builder.object("moNodeName_c",lambda _:cstring("child name 鋼",True))]
    return raw+node_list(nodes,-72,True),name_id


def pw_wall(index: int) -> bytes:
    raw = integers(-7 if index<2 else 9)+cstring(f"wall {index} 鋼",True)+cstring("info é\0")
    return raw+integers(-18,28,-32,44)


def pw_scene(builder: ObjectBuilder, empty: bool) -> bytes:
    raw = b"".join(cstring(value,True) for value in ["C:\\example\\scene","display 鋼","lights"])
    raw += integers(-1,2,-3,4,-3 if empty else 3)
    raw += b"" if empty else b"".join(pw_wall(index) for index in range(3))
    raw += doubles(4,1)+integers(-5,6,-7,8)
    raw += doubles(3)+doubles(3,3)+cstring("background 鋼",True)+doubles(1,6)+integers(9,-10)
    raw += doubles(3,1)+doubles(3,4)+cstring("environment",True)+doubles(2,7)+integers(-11,12)
    raw += doubles(3,2)+doubles(2,5)+integers(-13,14,-15,16,-17,18)
    return raw+doubles(3,3)+integers(-19,20)


def wall(kind: int, viz: int) -> bytes:
    base = integers(-7 if kind<3 else 9,kind,-1,2)+object_ref(viz if kind!=2 else 0)+integers(-16)
    payload = {1:floats(18),2:floats(3)+doubles(2,4),3:doubles(3,1)+doubles(1,5)}
    return base+payload[kind]


def floor(name_id: int, empty: bool, exposure: int) -> bytes:
    raw = doubles(10)+integers(-8)+compact(2,0x20)+object_ref(0 if empty else name_id)
    raw += integers(-9)+doubles(1,6)+integers(10)+doubles(2,2)+integers(-11)+floats(1)
    raw += integers(12,-13,14,-15,16)+doubles(2,4)+integers(-17)+compact(0,0x40)
    return raw+struct.pack("<Q",exposure)+integers(18)+doubles(1,7)


def scene(builder: ObjectBuilder, viz: int, empty: bool, exposure: int) -> bytes:
    raw,name_id = concept(builder,viz,empty)
    raw += builder.object("pwSceneData_c",lambda state:pw_scene(state,empty))
    raw += integers(-2 if empty else 3)
    raw += b"" if empty else b"".join(wall(kind,viz) for kind in [1,2,3])
    return raw+floor(name_id,empty,exposure)


def scene_slot(document: swcontainer.Document, source: Path, raw: bytes) -> tuple[int,int,int]:
    original = dl_doc.read_UnitsofMeasurement
    slots: list[tuple[int,int]] = []
    def capture(archive: ShadedArchive) -> object:
        slots.append((archive.p-4,len(archive.map)))
        return original(archive)
    dl_doc.read_UnitsofMeasurement = capture
    try:
        archive = displaylists.archive_for(document,str(source),raw)
        root = displaylists.display_lists(archive,archive.doc_kind)
    finally:
        dl_doc.read_UnitsofMeasurement = original
    assert archive.p == len(raw) and len(slots) == 1 and root["scene_type"] == 0
    viz = next(index for index,value in enumerate(archive.map) if value is root["visual_properties"])
    return *slots[0],viz


def generate(kind: int, name: str, empty: bool = False, exposure: int = 0x7ff0000000000000) -> dict[str,object]:
    source = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(source))
    raw = document.read("Contents/DisplayLists")
    start,first_index,viz = scene_slot(document,source,raw)
    builder = ObjectBuilder(first_index)
    body = scene(builder,viz,empty,exposure) if kind in [1,2] else b""
    derived = raw[:start]+integers(kind)+body+raw[start+4:]
    archive = displaylists.archive_for(document,str(source),derived)
    root = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(derived)
    if kind in [1,2]:
        assert len(root["scene"]["walls"]) == (0 if empty else 3)
    Path(f"tests/fixtures/display-lists/{name}.bin").write_bytes(derived)
    trace = scene_corpus.trace(root)
    Path(f"tests/fixtures/display-lists/{name}-trace.txt").write_bytes(trace)
    return {"fixture":name,"scene_type":kind,"empty":empty,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"source_sha256":hashlib.sha256(raw).hexdigest(),"scene_start":start,"scene_end":start+4+len(body),"map_slots":len(archive.map),"new_objects":sum(archive.classes.values()),"aggregate_entries":53 if empty or kind not in [1,2] else 62,"trace_fields":len(trace.splitlines())}


if __name__ == "__main__":
    rows = [generate(1,"scene-cubical"),generate(2,"scene-spherical"),generate(1,"scene-empty",True),generate(3,"scene-omitted"),generate(1,"scene-edited",False,0x4037000000000000)]
    Path(__file__).with_name("scene-vectors.json").write_text(json.dumps(rows,indent=2)+"\n",encoding="utf-8",newline="\n")
