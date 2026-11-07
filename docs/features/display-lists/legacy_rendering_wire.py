"""Complete pre-rendering material records and independent versioned inline facets."""
import struct
from pathlib import Path
from legacy_ordinary_text import ordinary_root
from legacy_edge_reference import edge_reference
from legacy_edge_roots import span_reader
from generate_version_range import apply_patches
from old_shaded_wire import facet_wire
from swarchive import Archive
import dl_light
import facettess
import dl_tess

VERSIONS = [2388,2389,2406,2407,2417,2418,2563]
FLAGS = ("use_advanced_graphics","use_defined_texture","use_defined_shader","use_defined_color","use_defined_hatch")


def rendering_facet(version: int, enabled: bool, edited: bool) -> bytes:
    raw = facet_wire(version,-17,False,enabled)
    stamp = struct.pack("<I",0xdeadbeef)
    assert raw.count(stamp)==1,(version,enabled)
    if edited:raw = raw.replace(stamp,struct.pack("<I",0xdeadbeee))
    archive = Archive(raw,version)
    result = facettess.facet_container(archive)
    assert archive.p==len(raw) and result["stamp"]==0xdeadbeef-int(edited)
    return raw


def rendering_material_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,spans = dl_light.read_moUserProperty_c,[]
        dl_light.read_moUserProperty_c = span_reader(saved,spans)
        try:result = original(archive)
        finally:dl_light.read_moUserProperty_c = saved
        position = spans[0][0]
        expected = struct.pack("<iiiii",*(result[name] for name in FLAGS))
        assert archive.b[position-20:position]==expected
        patches.append((position-20,position,b""))
        return result
    return read


def rendering_facet_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        enabled = "texture_coords" in result
        patches.append((first,archive.p,rendering_facet(version,enabled,edited)))
        return result
    return read


def rendering_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = ordinary_root(2563,kind,scenario)
    material,facet = dl_light.read_uoMaterialProperties,dl_tess.facet_container
    patches = []
    if version<=0x954:dl_light.read_uoMaterialProperties = rendering_material_reader(material,patches)
    dl_tess.facet_container = rendering_facet_reader(facet,version,scenario%16==1,patches)
    try:edge_reference(raw,2563,kind,identities)
    finally:dl_light.read_uoMaterialProperties,dl_tess.facet_container = material,facet
    return apply_patches(raw,patches),identities


def write_rendering_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version in VERSIONS:
        for enabled in (False,True):
            for edited in (False,True):
                name = f"legacy-rendering-facet-{version}-{int(enabled)}-{int(edited)}.bin"
                (root/name).write_bytes(rendering_facet(version,enabled,edited))
