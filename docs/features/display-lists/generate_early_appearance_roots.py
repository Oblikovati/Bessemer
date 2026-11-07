"""Independent complete 5000-range roots with transient text and geometry items."""
import json
import struct
from generate_earlier_range import generate as generate_root,converted as convert_root,write_group
from generate_early_view_roots import display_items,entry_budget
from generate_item_vectors import record,payloads
from early_appearance_patches import convert,rv_source
from generate_component_vectors import integers

VERSIONS = [5000,0x13ec,0x13ed,0x13ef,0x1400,0x1401,0x1415,0x1416,0x1421,0x1422,0x143a,0x143b,0x1447,0x1448,0x1449,0x144a,0x144b,0x144c,0x1462,0x1463,0x146e,0x146f,0x1472,0x1473,0x147d,0x147e,5999,6000]


def item_bytes(version: int) -> bytes:
    original = display_items(version)
    original += integers(-1701) if 0x1447<version<0x144a else b""
    parabola = payloads(False)[13]
    if version<=0x13ec:parabola = parabola[:120]+parabola[124:]
    center = payloads(False)[33]
    if version<0x1473:center = center[:-8]
    if version<=0x146e:center = center[:28]
    return struct.pack("<H",5)+original[2:]+record(13,parabola)+record(33,center)


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    return convert_root(document,path,rv_source(document,path,raw),version,kind,variant,convert,b"",item_bytes(version))


def face_identity_trace(root: dict[str,object]) -> bytes:
    tessellation = root["tessellation"]
    path = "root.tessellation"
    if isinstance(tessellation,list):
        tessellation = tessellation[0]["comp_part_tess_data"]
        path += ".components[0].comp_part_tess_data"
    rows = []
    for body_index,body in enumerate(tessellation["bodies"]):
        for face_index,face in enumerate(body["faces"]):
            if "unknown_38" in face:
                rows.append(f"{path}.bodies[{body_index}].faces[{face_index}].unknown_38=i:{face['unknown_38']}\n")
    return "".join(rows).encode()


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%32
    framed,row = generate_root(version,kind,variant,converted,lambda v,k,s:entry_budget(v,k,s)+2,face_identity_trace)
    row["scenario"] = 224+variant
    return framed[:5]+bytes([224+variant])+framed[6:],row


if __name__=="__main__":
    totals = []
    for kind in ["part","assembly"]:
        totals.append(write_group("early-appearance-roots-"+kind,[generate(v,kind) for v in VERSIONS]+[generate(5000,kind,1)]))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
