"""Complete old part models without a model-visual pointer or facet edge arrays."""
import struct
from pathlib import Path
from itertools import product
from legacy_scope_wire import early_scope_root
from legacy_text_reference import early_text_reference
from legacy_boundary_wire import facet_reader,boundary_facet
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_tess
import dl_light
from legacy_light_base_wire import old_light_base_reader,old_light_derived_reader
import dl_ui
from legacy_edge_roots import span_reader

VERSIONS = [989,990,997,998,1030,1031,1063,1064,1086,1087,1105,1106,1107,1108,1116,1117,1180]


def model_viz_object_reader(original: object, spans: list[tuple[int,int,int,int,object]]) -> object:
    def read(archive: object, *args: object) -> object:
        first,before = archive.p,len(archive.map)
        result = original(archive,*args)
        spans.append((first,archive.p,before,len(archive.map),result))
        return result
    return read


def model_viz_part_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = ObjectArchive.object,[]
        ObjectArchive.object = model_viz_object_reader(saved,spans)
        try:result = original(archive)
        finally:ObjectArchive.object = saved
        first,last,before,after,value = spans[-1]
        assert last==archive.p and before==after,(first,last,before,after)
        assert value is result["model_viz_props"][0][1]
        patches.append((first,last,b""))
        return result
    return read


def early_model_viz_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    profile = 16+(scenario-144)//2 if scenario>=144 else scenario
    raw,identities = early_scope_root(1180,kind,profile)
    patches = []
    saved = install_model_viz_readers(version,scenario,patches)
    try:early_text_reference(raw,1180,kind,identities)
    finally:restore_model_viz_readers(saved)
    return apply_patches(raw,patches),identities


def write_early_model_viz_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version,enabled,edited in product(VERSIONS,(False,True),(False,True)):
        name = f"legacy-model-viz-facet-{version}-{int(enabled)}-{int(edited)}.bin"
        (root/name).write_bytes(boundary_facet(version,enabled,edited))


def model_viz_splitter_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,transforms = dl_ui.read_mgXform_c,[]
        dl_ui.read_mgXform_c = span_reader(saved,transforms)
        try:result = original(archive)
        finally:dl_ui.read_mgXform_c = saved
        assert len(transforms)==2*len(result["panes"])
        for index,pane in enumerate(result["panes"]):
            position = transforms[2*index+1][1]+2
            raw, = struct.unpack_from("<H",archive.b,position)
            assert pane["hlr_display_quality"]==(0 if raw==0xffff else raw)
            patches.append((position,position+2,b""))
        return result
    return read


def install_model_viz_readers(version: int, scenario: int, patches: list[tuple[int,int,bytes]]) -> tuple[object,...]:
    classes = ("moDirectionLight_c","moSpotLight_c","moPointLight_c")
    saved = (dl_tess.read_uoTempPartTessData,dl_tess.facet_container,dl_ui.read_uiSplitterInfo_c,dl_light.read_moLightFeature_c,*(ObjectArchive.readers[name] for name in classes))
    edited,ends = (scenario%2==1 if scenario>=144 else scenario==4 or scenario%16==1),{}
    if version<0x3e6:dl_tess.read_uoTempPartTessData = model_viz_part_reader(saved[0],patches)
    if version<=0x45c:dl_ui.read_uiSplitterInfo_c = model_viz_splitter_reader(saved[2],patches)
    dl_tess.facet_container = facet_reader(saved[1],version,edited,patches)
    dl_light.read_moLightFeature_c = old_light_base_reader(saved[3],version,edited,ends,patches)
    for index,name in enumerate(classes):ObjectArchive.readers[name] = old_light_derived_reader(saved[4+index],name,version,edited,ends,patches)
    return saved


def restore_model_viz_readers(saved: tuple[object,...]) -> None:
    dl_tess.read_uoTempPartTessData,dl_tess.facet_container,dl_ui.read_uiSplitterInfo_c,dl_light.read_moLightFeature_c = saved[:4]
    for index,name in enumerate(("moDirectionLight_c","moSpotLight_c","moPointLight_c")):
        ObjectArchive.readers[name] = saved[4+index]
