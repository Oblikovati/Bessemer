"""Cut an independently parsed full root at its declared shaded-data flag."""
from generate_alias_fixture import displaylists
from generate_component_vectors import integers
from early_prefix import light_slot
import dl_light


def unshaded_prefix(document: object, path: object, raw: bytes, version: int, kind: str) -> bytes:
    positions: list[tuple[int,int]] = []
    original = dl_light.read_old_lights
    dl_light.read_old_lights = light_slot(original,positions)
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
        displaylists.display_lists(archive,kind)
        assert archive.p==len(raw) and len(positions)==1
    finally:
        dl_light.read_old_lights = original
    return raw[:positions[0][0]]+integers(0)
