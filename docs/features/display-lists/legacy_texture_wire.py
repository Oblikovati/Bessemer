"""Observe and remove primitive texture and shaded-edge fields in complete roots."""
import struct
from legacy_rendering_wire import rendering_root
from legacy_edge_reference import edge_reference
from legacy_edge_roots import span_reader
from generate_version_range import apply_patches
import dl_light
import dl_doc
import dl_ui

VERSIONS = [2359,2360,2374,2375,2388]


def texture_material_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        suffix = archive.p
        assert archive.b[suffix-4:suffix]==bytes(4),result
        # At the source version only a zero user-property count follows the texture.
        assert result["user_property"]==[],result
        prefix = string_pair_end(archive,first)+48
        flag, = struct.unpack_from("<i",archive.b,prefix)
        assert bool(flag)==("texture" in result)
        patches.append((prefix,suffix-4,b""))
        return result
    return read


def string_pair_end(archive: object, position: int) -> int:
    saved = archive.p
    archive.p = position
    try:
        archive.cstring()
        archive.cstring()
        return archive.p
    finally:archive.p = saved


def observed_texture_root(version: int, raw: bytes, kind: str, identities: tuple[str,...]) -> bytes:
    originals = dl_light.read_uoMaterialProperties,dl_doc.read_AnnotationViewTransform,dl_ui.read_mgXform_c
    patches,annotation,transforms = [],[],[]
    if version<=0x937:dl_light.read_uoMaterialProperties = texture_material_reader(originals[0],patches)
    dl_doc.read_AnnotationViewTransform = span_reader(originals[1],annotation)
    dl_ui.read_mgXform_c = span_reader(originals[2],transforms)
    try:root = edge_reference(raw,2388,kind,identities)
    finally:dl_light.read_uoMaterialProperties,dl_doc.read_AnnotationViewTransform,dl_ui.read_mgXform_c = originals
    if version<=0x946:
        assert len(annotation)==1 and root["view_count"]==1
        view = [span for span in transforms if span[0]>=annotation[0][1]][:2]
        position = view[1][1]+8
        assert struct.unpack_from("<i",raw,position)[0]==root["views"][0]["edges_in_shaded_mode"]
        patches.append((position,position+4,b""))
    return apply_patches(raw,patches)


def texture_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = rendering_root(2388,kind,scenario)
    return observed_texture_root(version,raw,kind,identities),identities
