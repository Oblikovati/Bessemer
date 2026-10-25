"""Complete roots down to unsigned version zero, with explicit omitted-field spans."""
from legacy_fontless_wire import early_fontless_root
from legacy_component_reference import component_reference
from legacy_zero_ui import zero_pane_reader,zero_edge_reader,zero_bucket_reader
from legacy_zero_material import zero_light_reader,zero_material_reader,zero_visual_reader
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_ui
import dl_light

VERSIONS = [0,1,144,145,172,173,211,212,370,371,380,381,437,438,561,562,595,596]


def install_zero_ui(version: int, scenario: int, patches: list[tuple[int,int,bytes]]) -> tuple[object,...]:
    saved = dl_ui.read_uiSplitterInfo_c,dl_ui.read_gcEdgeVisData_c,dl_ui.read_gcBucket_c
    if version<=437:dl_ui.read_uiSplitterInfo_c = zero_pane_reader(saved[0],patches)
    if version<=561 and scenario not in (188,189):dl_ui.read_gcEdgeVisData_c = zero_edge_reader(saved[1],patches)
    dl_ui.read_gcBucket_c = zero_bucket_reader(saved[2],version,scenario,patches)
    return saved


def install_zero_appearance(version: int, patches: list[tuple[int,int,bytes]]) -> tuple[object,...]:
    saved = dl_light.read_moLightFeature_c,dl_light.read_uoMaterialProperties,ObjectArchive.readers["moVisualProperties_c"]
    if version<=380:dl_light.read_moLightFeature_c = zero_light_reader(saved[0],patches)
    if version<=370:dl_light.read_uoMaterialProperties = zero_material_reader(saved[1],patches)
    if version<=211:ObjectArchive.readers["moVisualProperties_c"] = zero_visual_reader(saved[2],version,patches)
    return saved


def early_zero_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = early_fontless_root(596,kind,32 if scenario in (188,189) else scenario)
    patches = []
    ui = install_zero_ui(version,scenario,patches)
    appearance = install_zero_appearance(version,patches)
    try:component_reference(raw,596,kind,identities)
    finally:
        dl_ui.read_uiSplitterInfo_c,dl_ui.read_gcEdgeVisData_c,dl_ui.read_gcBucket_c = ui
        dl_light.read_moLightFeature_c,dl_light.read_uoMaterialProperties,ObjectArchive.readers["moVisualProperties_c"] = appearance
    return apply_patches(raw,patches),identities
