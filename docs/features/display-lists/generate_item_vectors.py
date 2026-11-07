"""Independently encode modern display-item payloads in an owned complete root."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists, swcontainer
from generate_named_vectors import cstring
from generate_transform_vectors import compact, doubles
from generate_component_vectors import integers
from generate_blank_scalars import flatten
import dl_doc


def record(kind: int, payload: bytes = b"") -> bytes:
    return struct.pack("<Ii",0xfedcba98,kind)+payload


def width() -> bytes:
    return struct.pack("<hI",-17,0x80000000)


def text_payload(enabled: bool) -> bytes:
    raw = doubles(5)+integers(-1,2)+cstring("é😀",True)+integers(-3)+doubles(2)
    return raw+struct.pack("<h",-2 if enabled else 0)+(integers(-1,2) if enabled else b"")


def simple_payload(enabled: bool) -> bytes:
    raw = doubles(3)+integers(-3 if enabled else 0)+(compact(2,0x20) if enabled else b"")
    raw += doubles(1)+integers(-1)+cstring("λ",True)+struct.pack("<h",-2 if enabled else 0)
    raw += integers(3,-1,2,3) if enabled else b""
    return raw+integers(-4)+doubles(1)


def symbol_payload(enabled: bool) -> bytes:
    raw = doubles(3)+integers(2 if enabled else 0)+(compact(0,0x40) if enabled else b"")
    raw += doubles(2)+cstring("symbol 鋼",True)+cstring("second")
    nested = doubles(3)+integers(0)+doubles(2)+cstring("")+cstring("")+integers(0)
    return raw+integers(2)+record(1,doubles(3)+integers(-3,4))+record(19,nested)


def annotation_payload() -> bytes:
    raw = integers(-1,2,1,-3,2,-4,5,1)+doubles(3)
    raw += integers(2)+cstring("one")+cstring("two 鋼",True)+integers(-6,1)+cstring("three")
    return raw+b"".join(cstring(value,True) for value in ["four","five","six","ID 鋼"])


def payloads(enabled: bool) -> dict[int,bytes]:
    pair, triple = doubles(6), doubles(9)
    font = doubles(1)+integers(-2)+doubles(2)+struct.pack("<I",0xfedcba98)+cstring("font 鋼",True)+doubles(2)
    prop = integers(-1)+doubles(3)+integers(2,-3)
    table = integers(-1,2,0 if enabled else -1)+(struct.pack("<III",2,0xffffffff,3) if enabled else b"")
    table += cstring("table 鋼",True)+integers(-4,5)+cstring("view names")
    return {
        1:doubles(3)+integers(-1,2),2:doubles(12)+integers(-3),3:doubles(17),
        4:pair+bytes([255]),5:integers(2)+pair+integers(-5),6:integers(-6,2)+pair,
        7:text_payload(enabled),8:doubles(3)+compact(2,0x10)+doubles(1)+cstring("sketch",True),
        9:integers(-1,2)+struct.pack("<I",0xffffffff)+integers(-3)+width()+integers(4,-5,6),
        10:font,11:cstring("https://example.invalid/鋼",True),
        12:struct.pack("<I",0xffffffff)+width()+integers(-3,4)+cstring("style"),
        13:doubles(15)+integers(-1)+doubles(2),14:font,15:text_payload(not enabled),
        18:simple_payload(enabled),19:symbol_payload(enabled),23:table,24:prop,
        25:struct.pack("<I",0xffffffff),26:struct.pack("<I",0xfffffffe),28:triple+integers(-8),
        29:annotation_payload(),30:doubles(3)+integers(-9),
        31:integers(2)+doubles(1)+integers(-10)+doubles(12)+cstring("pipe 鋼",True),
        32:prop,33:doubles(3)+integers(-11)+doubles(1)+integers(-12)+doubles(1),
        100:pair+integers(-13),101:doubles(12)+integers(-14),108:struct.pack("<I",0x80000000),
        113:integers(-1,2)+struct.pack("<I",0x7f800000),116:prop,
    }


def trace_items(items: list[dict[str,object]], prefix: str, rows: dict[str,str]) -> None:
    for index,item in enumerate(items):
        path = f"{prefix}.items[{index}]"
        flatten(f"{path}.size_of_element",item["size_of_element"],rows)
        flatten(f"{path}.type",item["type"],rows)
        body = dict(item.get("data",{}))
        nested = body.pop("items",None)
        for key,value in body.items():
            if isinstance(value,bytes):
                value = list(struct.unpack("<"+"I"*(len(value)//4),value))
            flatten(f"{path}.data.{key}",value,rows)
        if nested is not None:
            trace_items(nested,f"{path}.data",rows)


def view_slot(document: swcontainer.Document, path: Path, source: bytes) -> tuple[int,int]:
    original = dl_doc.read_AnnotationViewTransform
    slots: list[int] = []
    def capture(archive: object) -> object:
        result = original(archive)
        slots.append(archive.p)
        return result
    dl_doc.read_AnnotationViewTransform = capture
    try:
        archive = displaylists.archive_for(document,str(path),source)
        root = displaylists.display_lists(archive,archive.doc_kind)
    finally:
        dl_doc.read_AnnotationViewTransform = original
    assert archive.p == len(source) and root["view_count"] == 1 and len(slots) == 1
    view_archive = displaylists.archive_for(document,str(path),source)
    view_archive.p = slots[0]
    old_view = dl_doc.read_view_record(view_archive)
    assert old_view["view_bucket"] is None and old_view["display_items"] == []
    return slots[0],view_archive.p


def view_bytes(enabled: bool, extended: bool, edited: bool) -> tuple[bytes,int]:
    bodies = payloads(enabled)
    if edited:
        bodies[1] = doubles(3)+integers(23,2)
    parts = [record(kind,payload) for kind,payload in bodies.items()]
    parts += [record(kind) for kind in [0x15,0x16,0x1b,*range(0x66,0x6c),0x6d,0x6e,0x6f,0x70,0x72,-1,0x7fffffff]]
    count = struct.pack("<HI",0xffff,len(parts)) if extended else struct.pack("<H",len(parts))
    view = integers(-1)+compact(0,0)+compact(0,0)+cstring("camera 鋼",True)+integers(-2,3,-4)+bytes(2)+count+b"".join(parts)
    return view,len(parts)


def write_fixture(name: str, derived: bytes, document: swcontainer.Document, path: Path) -> int:
    archive = displaylists.archive_for(document,str(path),derived)
    root = displaylists.display_lists(archive,archive.doc_kind)
    assert archive.p == len(derived)
    rows: dict[str,str] = {}
    trace_items(root["views"][0]["display_items"],"root.views[0].display_items",rows)
    destination = Path(f"tests/fixtures/display-lists/{name}.bin")
    destination.write_bytes(derived)
    destination.with_suffix(".txt").write_text("".join(f"{key}={value}\n" for key,value in sorted(rows.items())),encoding="utf-8",newline="\n")
    return len(rows)


def generate(enabled: bool, extended: bool, edited: bool = False) -> dict[str,object]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    source = document.read("Contents/DisplayLists")
    start,old_end = view_slot(document,path,source)
    view,count = view_bytes(enabled,extended,edited)
    derived = source[:start]+view+source[old_end:]
    name = "items-edited" if edited else "items-present" if enabled else "items-absent"
    fields = write_fixture(name,derived,document,path)
    return {"fixture":name,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"view_start":start,"view_end":start+len(view),"items":count,"scalar_fields":fields,"extended_count":extended}


def main() -> None:
    evidence = {"context":{"display_version":19000,"file_version":19000,"kind":"part"},"scope":"wire interoperability; no model validity or application acceptance claim","fixtures":[generate(True,True),generate(False,False),generate(True,True,True)]}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_ui.py","displaylists.py"]}
    Path(__file__).with_name("item-vectors.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__ == "__main__":
    main()
