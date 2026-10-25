"""Complete old shaded roots assembled independently; no CAD model validity is implied."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import displaylists,swcontainer
from generate_component_vectors import integers
from generate_transform_vectors import doubles
from generate_shaded_vectors import ObjectBuilder
from generate_light_vectors import base
from generate_scene_vectors import concept,pw_scene,wall,floor
from generate_version_range import apply_patches,capture_plain
from generate_blank_scalars import flatten
from old_shaded_wire import visual,part,assembly,items
import dl_doc
import dl_light
import material_corpus
import scene_corpus
import light_corpus
import item_corpus


def prefix(document: swcontainer.Document, path: Path, raw: bytes) -> tuple[bytes,int]:
    patches: list[tuple[int,int,bytes]] = []
    original_font,original_lights = dl_doc.read_OrigFontScale,dl_light.read_old_lights
    original_items = dl_doc.read_display_item_list
    slots: list[tuple[int,int]] = []
    def capture(archive: object) -> object:
        slots.append((archive.p-4,len(archive.map)))
        return original_lights(archive)
    try:
        dl_doc.read_OrigFontScale = capture_plain(original_font,b"",patches)
        dl_light.read_old_lights = capture
        dl_doc.read_display_item_list = capture_plain(original_items,items(),patches)
        archive = displaylists.archive_for(document,str(path),raw)
        displaylists.display_lists(archive,archive.doc_kind)
    finally:
        dl_doc.read_OrigFontScale,dl_light.read_old_lights = original_font,original_lights
        dl_doc.read_display_item_list = original_items
    assert archive.p==len(raw) and len(slots)==1
    end,index = slots[0]
    assert raw[end:end+4]==integers(1)
    return apply_patches(raw[:end],patches),index


def old_scene(builder: ObjectBuilder, viz: int) -> bytes:
    raw,identity = concept(builder,viz,False)
    raw += builder.object("pwSceneData_c",lambda state:pw_scene(state,False))+integers(3)
    raw += b"".join(wall(kind,viz) for kind in [1,2,3])
    return raw+floor(identity,False,0x7ff0000000000000)[:-20]


def shaded(builder: ObjectBuilder, version: int, kind: str, edited: bool) -> bytes:
    raw = integers(1,1)+struct.pack("<H",2)
    raw += builder.object("moAmbientLight_c",lambda _:base(True))
    raw += builder.object("moDirectionLight_c",lambda _:base(True)+doubles(4))
    viz = builder.next_index+1
    raw += builder.object("moVisualProperties_c",lambda _:visual(version,edited))
    if kind=="part":
        raw += part(builder,version,viz)
    else:
        raw += struct.pack("<H",1)+builder.object("uoTempAssemblySHDData_c",lambda state:assembly(state,version,viz,state.next_index-1))
    raw += integers(0,-12,1)+old_scene(builder,viz)+integers(-1,2,-3)
    return raw+(integers(0) if kind=="assembly" else b"")


def component_trace(root: dict[str,object], kind: str) -> bytes:
    if kind!="assembly":
        return b""
    viewer = dict(root["tessellation"][0]["viewer_comp_data"])
    for key in ["comp_xform","comp_bbox","comp_viz_prop"]:
        viewer.pop(key)
    rows: dict[str,str] = {}
    dependencies = viewer.pop("equ_dep_comp_names")
    base_path = "root.tessellation.components[0].viewer_comp_data"
    for index,(key,values) in enumerate(dependencies):
        flatten(f"{base_path}.equ_dep_comp_names[{index}].key",key,rows)
        flatten(f"{base_path}.equ_dep_comp_names[{index}].values",values,rows)
    flatten(base_path,viewer,rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def generate(version: int, kind: str, edited: bool = False, unshaded: bool = False) -> tuple[bytes,dict[str,object]]:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(str(path))
    start,index = prefix(document,path,document.read("Contents/DisplayLists"))
    derived = start+(integers(0) if unshaded else shaded(ObjectBuilder(index),version,kind,edited))
    archive = displaylists.archive_for(document,str(path),derived)
    archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
    root = displaylists.display_lists(archive,kind)
    assert archive.p==len(derived)
    trace = material_corpus.trace(root)+scene_corpus.trace(root)+light_corpus.trace(root)+item_corpus.trace(root)
    scalar = component_trace(root,kind)
    scenario = 2 if unshaded else int(edited)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(derived))+derived+struct.pack("<I",len(trace))+trace+struct.pack("<I",len(scalar))+scalar
    entries = 51 if unshaded else 87+(4 if version==11000 else 0)+(8 if kind=="assembly" else 0)
    return framed,{"version":version,"kind":kind,"scenario":scenario,"bytes":len(derived),"sha256":hashlib.sha256(derived).hexdigest(),"trace_fields":len(trace.splitlines()),"component_scalar_fields":len(scalar.splitlines()),"first_map_index":index,"new_objects":sum(archive.classes.values()),"aggregate_entries":entries}


def main() -> None:
    cases = [generate(version,kind) for version in [10000,11000] for kind in ["part","assembly"]]+[generate(10000,"part",True),generate(10000,"part",False,True)]
    framed = struct.pack("<I",len(cases))+b"".join(raw for raw,_ in cases)
    Path("tests/fixtures/display-lists/old-shaded.bin").write_bytes(framed)
    evidence = {"scope":"owned complete wire streams, explicit file context 19000, inline faces; no model/application validity claim","fixtures":[row for _,row in cases],"sha256":hashlib.sha256(framed).hexdigest()}
    evidence["source_hashes"] = {name:hashlib.sha256(Path(f"../inventor-interface/solidworks/{name}").read_bytes()).hexdigest() for name in ["dl_doc.py","dl_tess.py","dl_light.py","dl_ui.py","facettess.py"]}
    Path(__file__).with_name("old-shaded.json").write_text(json.dumps(evidence,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps(evidence))


if __name__=="__main__":
    main()
