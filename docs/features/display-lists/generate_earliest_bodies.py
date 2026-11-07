"""Complete owned roots around direct-body-property and early state-word transitions."""
import json
import struct
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_earliest_faces import items as parent_items,entry_count as face_entries
from generate_earliest_components import write_documents
from generate_earliest_lights import extra_trace as light_trace
from generate_earliest_scene_roots import raw_nan_reader
from generate_component_vectors import integers
from early_appearance_patches import rv_source
from earliest_body_patches import convert
from earliest_body_wire import direct_properties
from swarchive import Archive

VERSIONS = [4300,0x10ef,0x10f0,0x10f5,0x10f6,0x110d,0x110e,0x1115,0x1116,4400]


def items(version: int) -> bytes:
    raw = parent_items(version)
    assert raw[18:22]==integers(-3)
    ole = integers(0)+struct.pack("<III",2,0,0xffffffff) if version>0x1115 else b""
    return raw[:18]+ole+raw[22:]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    raw = convert_root(document,path,rv_source(document,path,raw),version,kind,variant%2,convert,b"",items(version))
    return direct_properties(document,path,raw,version,variant==2)


def entry_count(version: int, kind: str, variant: int) -> int:
    return face_entries(version,kind,variant%2)+int(version<0x10f6 and variant!=2)+2*int(version>0x1115)


def extra_trace(root: dict[str,object]) -> bytes:
    part = root["tessellation"]
    prefix = "root.tessellation"
    if isinstance(part,list):
        part = part[0]["comp_part_tess_data"]
        prefix += ".components[0].comp_part_tess_data"
    rows = [f"{prefix}.bodies[0].body_prop_infos[{index}].key=i:{key}\n" for index,(key,_) in enumerate(part["bodies"][0]["body_prop_infos"])]
    return light_trace(root)+"".join(rows).encode()


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_root(version,kind,scenario%16,converted,entry_count,extra_trace)
    finally:Archive.f32 = original
    row["scenario"] = 224+scenario%16
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


if __name__=="__main__":
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(v,kind) for v in VERSIONS]+[generate(4300,kind,1),generate(4300,kind,2),generate(0x10f5,kind,2)]
        totals.append(write_group("earliest-body-"+kind,cases,("displaylists.py",)))
    write_documents(generate,4300,"earliest-body")
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
