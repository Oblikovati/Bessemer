"""Complete old environments and facets without unstored texture-coordinate flags."""
import struct
from pathlib import Path
from legacy_texture_wire import texture_root
from legacy_rendering_wire import rendering_facet_reader,rendering_facet
from legacy_edge_reference import edge_reference
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_tess

VERSIONS = [2277,2278,2304,2305,2359]


def environment_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        expected = struct.pack("<ii",result["unknown_1b8"],result["unknown_1bc"])
        assert archive.b[archive.p-8:archive.p]==expected
        patches.append((archive.p-8,archive.p,b""))
        return result
    return read


def environment_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = texture_root(2359,kind,scenario)
    environment,facet = ObjectArchive.readers["uiUserModelEnv_c"],dl_tess.facet_container
    patches = []
    if version<=0x8e5:ObjectArchive.readers["uiUserModelEnv_c"] = environment_reader(environment,patches)
    dl_tess.facet_container = rendering_facet_reader(facet,version,scenario%16==1,patches)
    try:edge_reference(raw,2359,kind,identities)
    finally:ObjectArchive.readers["uiUserModelEnv_c"],dl_tess.facet_container = environment,facet
    return apply_patches(raw,patches),identities


def write_environment_facets() -> None:
    root = Path("tests/fixtures/display-lists")
    for version in VERSIONS:
        for enabled in (False,True):
            for edited in (False,True):
                name = f"legacy-environment-facet-{version}-{int(enabled)}-{int(edited)}.bin"
                (root/name).write_bytes(rendering_facet(version,enabled,edited))
