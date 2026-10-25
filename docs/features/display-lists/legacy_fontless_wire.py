"""Complete roots with absent font managers and omitted polygon line-property records."""
from legacy_root_item_wire import early_root_item_root
from legacy_component_reference import component_reference
from legacy_model_viz_wire import model_viz_object_reader
from legacy_fontless_tags import remove_font_manager,rewrite_font_tags
from legacy_fontless_facets import fontless_facet_reader,write_fontless_facets,VERSIONS
from legacy_edge_roots import span_reader
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_ui
import dl_tess


def fontless_polygon_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int = 0) -> object:
        first,saved,styles = archive.p,dl_ui.read_moLineViz_c,[]
        dl_ui.read_moLineViz_c = span_reader(saved,styles)
        try:result = original(archive,flag)
        finally:dl_ui.read_moLineViz_c = saved
        if result["count"]<=0:return result
        position = first+16+12*result["count"]
        present = archive.b[position]
        assert bool(present)==("user_line_property" in result)
        assert len(styles)==int(bool(present))
        end = styles[0][1] if present else position+1
        if present:assert styles[0][0]==position+1
        patches.append((position,end,b""))
        return result
    return read


def install_fontless_readers(version: int, scenario: int, patches: list[tuple[int,int,bytes]], objects: list[tuple[int,int,int,int,object]], faces: list[int]) -> tuple[object,...]:
    saved = ObjectArchive.object,dl_ui.read_gcBucketPoly_c,dl_tess.facet_container
    ObjectArchive.object = model_viz_object_reader(saved[0],objects)
    if version<=760:dl_ui.read_gcBucketPoly_c = fontless_polygon_reader(saved[1],patches)
    dl_tess.facet_container = fontless_facet_reader(saved[2],version,scenario%2==1,patches,faces)
    return saved


def early_fontless_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    profile = scenario-18 if scenario in (180,181) else scenario
    raw,identities = early_root_item_root(767,kind,profile)
    patches,objects,faces = [],[],[]
    saved = install_fontless_readers(version,scenario,patches,objects,faces)
    try:root = component_reference(raw,767,kind,identities)
    finally:ObjectArchive.object,dl_ui.read_gcBucketPoly_c,dl_tess.facet_container = saved
    assert len(faces) in (0,2)
    if version<=766:remove_font_manager(raw,root,objects,patches,scenario in (180,181))
    elif scenario in (180,181):rewrite_font_tags(raw,objects,(0,0,0,0),patches,True)
    return apply_patches(raw,patches),identities
