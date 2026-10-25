"""Keep older assembly geometry as explicit external face-cache references."""
import struct
from pre_facet_wire import prototype,observed,items
from generate_unsigned_versions import reference
from generate_version_range import apply_patches
import dl_tess


def observe_facets(original: object, spans: list[tuple[int,int]]) -> object:
    def capture(archive: object) -> object:
        start = archive.p
        result = original(archive)
        spans.append((start,archive.p))
        return result
    return capture


def external_root(version: int, edited: bool) -> tuple[bytes,tuple[str,...]]:
    raw = prototype("assembly")
    spans,facets = observed(raw,"assembly"),[]
    original = dl_tess.facet_container
    dl_tess.facet_container = observe_facets(original,facets)
    try:root = reference(raw,3196,"assembly",())
    finally:dl_tess.facet_container = original
    assert len(facets)==2,f"facet spans {facets}; expected both owned faces"
    component = root["tessellation"][0]
    identities = (component["viewer_comp_data"]["comp_id_string"],)
    patches = [(first,last,b"") for first,last in facets]
    patches.append((*spans["items"],items(version,edited)))
    if version<=0xc6e:
        end = spans["tessellation"][1]
        assert raw[end-4:end]==struct.pack("<i",component["unknown_320"])
        patches.append((end-4,end,b""))
    return apply_patches(raw,patches),identities
