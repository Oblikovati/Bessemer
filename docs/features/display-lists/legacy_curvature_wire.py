"""Complete roots with independently populated legacy facet arrays and calculated colours."""
import struct
from pathlib import Path
from itertools import product
from legacy_component_wire import early_component_root
from legacy_component_reference import component_reference
from legacy_boundary_wire import boundary_facet
from generate_version_range import apply_patches
import dl_tess

VERSIONS = [773,774,816,817,903]


def legacy_curvature_array(enabled: bool, edited: bool) -> bytes:
    count = 1+int(enabled)+int(edited)
    payload = bytearray(bytes(range(24))*count)
    if edited:payload[-1] = 255
    return struct.pack("<IIII",24,17+int(edited),23+int(edited),count)+payload


def curvature_facet(version: int, enabled: bool, edited: bool) -> bytes:
    raw = boundary_facet(version,enabled,edited)
    if version>816:return raw
    original = struct.pack("<IIII",24,17,23,1)+bytes(range(24))
    assert raw.count(original)==1
    return raw.replace(original,legacy_curvature_array(enabled,edited))


def curvature_facet_reader(original: object, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        enabled = "curvature_colors" in result
        patches.append((first,archive.p,curvature_facet(version,enabled,edited)))
        return result
    return read


def early_curvature_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = early_component_root(903,kind,scenario)
    original,patches = dl_tess.facet_container,[]
    edited = scenario%2==1 if scenario>=144 else scenario==4 or scenario%16==1
    dl_tess.facet_container = curvature_facet_reader(original,version,edited,patches)
    try:component_reference(raw,903,kind,identities)
    finally:dl_tess.facet_container = original
    return apply_patches(raw,patches),identities


def write_curvature_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version,enabled,edited in product(VERSIONS,(False,True),(False,True)):
        name = f"legacy-curvature-facet-{version}-{int(enabled)}-{int(edited)}.bin"
        (root/name).write_bytes(curvature_facet(version,enabled,edited))
