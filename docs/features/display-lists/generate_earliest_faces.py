"""Complete part/assembly roots with populated old face pairs and polyline points."""
import json
import struct
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_early_appearance_roots import item_bytes,face_identity_trace
from generate_early_view_roots import entry_budget as early_entries
from generate_item_vectors import record,payloads
from generate_earliest_scene_roots import raw_nan_reader
from early_appearance_patches import rv_source
from earliest_face_patches import convert
from swarchive import Archive
from generate_earliest_components import write_documents

VERSIONS = [4600,0x1202,0x1203,0x1211,0x1212,0x12a2,0x12a3,0x12b0,0x12b1,4800]


def items(version: int) -> bytes:
    original = item_bytes(version)
    polyline = payloads(False)[5]
    if version<=0x12b0:polyline = polyline[:-4]
    return struct.pack("<H",6)+original[2:]+record(5,polyline)


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    return convert_root(document,path,rv_source(document,path,raw),version,kind,variant,convert,b"",items(version))


def entry_count(version: int, kind: str, variant: int) -> int:
    return early_entries(version,kind,variant)+2+3-4*int(version<0x1203)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_root(version,kind,scenario%16,converted,entry_count,face_identity_trace)
    finally:Archive.f32 = original
    row["scenario"] = 192+scenario%16
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


if __name__=="__main__":
    totals = [write_group("earliest-face-"+k,[generate(v,k) for v in VERSIONS]+[generate(4600,k,1)]) for k in ["part","assembly"]]
    write_documents(generate,4600,"earliest-face")
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
