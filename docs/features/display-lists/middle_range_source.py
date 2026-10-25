"""Complete owned roots with inline facets and populated shared scenes for both kinds."""
from pathlib import Path
from generate_shaded_vectors import ObjectBuilder,assembly_slot,part,displaylists,swcontainer
from generate_scene_vectors import scene,scene_slot
from generate_component_vectors import integers
from generate_blank_scalars import flatten


def source(kind: str) -> tuple[Path,object,bytes]:
    path = Path("tests/fixtures/blank-part.SLDPRT" if kind=="part" else "tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    raw = document.read("Contents/DisplayLists")
    if kind=="part":
        raw = Path("tests/fixtures/display-lists/inline-shaded-part.bin").read_bytes()
    else:
        start,end,index = assembly_slot(document,path,raw)
        raw = raw[:start]+ObjectBuilder(index).object("uoTempPartTessData_c",lambda state:part(state,False))+raw[end:]
    start,index,viz = scene_slot(document,path,raw)
    replacement = scene(ObjectBuilder(index),viz,False,0x7ff0000000000000)
    return path,document,raw[:start]+integers(1)+replacement+raw[start+4:]


def component_trace(root: dict[str,object], kind: str) -> bytes:
    if kind!="assembly":
        return b""
    viewer = dict(root["tessellation"][0]["viewer_comp_data"])
    for key in ["comp_xform","comp_bbox","comp_viz_prop","ref_geom_data_collector","unknown_obj"]:
        viewer.pop(key,None)
    rows: dict[str,str] = {}
    dependencies = viewer.pop("equ_dep_comp_names")
    path = "root.tessellation.components[0].viewer_comp_data"
    for index,(key,values) in enumerate(dependencies):
        flatten(f"{path}.equ_dep_comp_names[{index}].key",key,rows)
        flatten(f"{path}.equ_dep_comp_names[{index}].values",values,rows)
    flatten(path,viewer,rows)
    if root.get("envelope_system_options") is not None:
        flatten("root.envelope_system_options",root["envelope_system_options"],rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
