"""Complete roots with single visual pointers, not synthetic counted maps."""
import json
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_earliest_bodies import items as body_items,extra_trace as body_trace
from generate_earliest_faces import entry_count as face_entries
from generate_earliest_lights import extra_trace as light_trace
from generate_earliest_scene_roots import raw_nan_reader
from generate_earliest_components import write_documents
from early_appearance_patches import rv_source
from implicit_visual_maps import single_properties
from earliest_tessellation_wire import earlier_tails,header_layout
from earliest_visual_patches import convert
from swarchive import Archive

VERSIONS = [4000,0xfca,0xfcb,0xfd7,0xfd8,4100,0x1051,0x1052,4200,4201,4202,0x1098,0x1099,0x10a1,0x10a2,0x10ae,0x10af,0x10b2,0x10b3,0x10bf,0x10c0,4300]


def table_layout(raw: bytes, version: int) -> bytes:
    return raw[:10]+raw[18:] if version<=0x10b2 else raw


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    raw = convert_root(document,path,rv_source(document,path,raw),version,kind,variant%2,convert,b"",table_layout(body_items(version),version))
    return earlier_tails(document,path,single_properties(document,path,header_layout(raw,version),version,variant==2),version,kind)


def entry_count(version: int, kind: str, variant: int) -> int:
    maps = 3 if version<=0x10ae else int(version<0x10f6 and variant!=2)
    # Each owned face has two surface-parameter elements, absent through 0x1098.
    return face_entries(version,kind,variant%2)+maps-4*int(version<=0x1098)


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    extra = light_trace if version<=0x10ae else body_trace
    try:framed,row = generate_root(version,kind,scenario%16,converted,entry_count,extra)
    finally:Archive.f32 = original
    row["scenario"] = 208+scenario%16
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


if __name__=="__main__":
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(v,kind) for v in VERSIONS]+[generate(4000,kind,1),generate(4000,kind,2),generate(4100,kind,1),generate(4100,kind,2),generate(4200,kind,1),generate(4200,kind,2),generate(0x10ae,kind,2)]
        totals.append(write_group("earliest-visual-"+kind,cases,("displaylists.py",)))
    write_documents(generate,4200,"earliest-visual")
    write_documents(generate,4100,"earliest-appearance")
    write_documents(generate,4000,"earliest-tessellation")
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
