"""Complete roots across every 4500-range appearance, light and assembly boundary."""
import json
from functools import partial
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_earliest_faces import items,entry_count as face_entries
from generate_earliest_scene_roots import raw_nan_reader
from generate_early_appearance_roots import face_identity_trace
from generate_earliest_components import write_documents
from early_appearance_patches import rv_source
from earliest_light_patches import convert
from earliest_light_wire import replace_light,populate_hidden
from swarchive import Archive
import light_corpus
from generate_blank_scalars import flatten

VERSIONS = [4500,4501,0x1197,0x1198,0x119e,0x119f,0x11a3,0x11a4,0x11a5,0x11a6,
            0x11ac,0x11ad,0x11af,0x11b0,0x11bb,0x11bc,0x11ed,0x11ee,0x11f5,0x11f6,4600]


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int, light: int) -> bytes:
    raw = replace_light(document,path,rv_source(document,path,raw),light)
    if kind=="assembly":raw = populate_hidden(document,path,raw)
    return convert_root(document,path,raw,version,kind,variant,convert,b"",items(version))


def entry_count(version: int, kind: str, variant: int) -> int:
    return face_entries(version,kind,variant)+4+(3*int(version>0x11af) if kind=="assembly" else 0)


def extra_trace(root: dict[str,object]) -> bytes:
    rows = {}
    components = root["tessellation"]
    if isinstance(components,list):
        for index,component in enumerate(components):
            if "hidden_body_atoms" in component:flatten(f"root.tessellation.components[{index}].hidden_body_atoms",component["hidden_body_atoms"],rows)
    return face_identity_trace(root)+light_corpus.trace(root)+"".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    light,edited = scenario%32//2,scenario%2
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:framed,row = generate_root(version,kind,edited,partial(converted,light=light),entry_count,extra_trace)
    finally:Archive.f32 = original
    row["scenario"] = 128+scenario%32
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


if __name__=="__main__":
    roots = [generate(v,k) for v in VERSIONS for k in ["part","assembly"]]+[generate(4500,k,1) for k in ["part","assembly"]]
    kinds = [generate(v,"part",2*k) for v in [4500,0x11a3,0x11a4,0x11f5,0x11f6,4600] for k in range(1,5)]
    totals = [write_group("earliest-light-roots",roots,("displaylists.py",)),write_group("earliest-light-kinds",kinds,("displaylists.py",))]
    write_documents(generate,4500,"earliest-light")
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
