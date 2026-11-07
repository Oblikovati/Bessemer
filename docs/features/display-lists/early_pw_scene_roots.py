"""Insert complete bucket/PW graphs and relocate every later observed MFC tag."""
from functools import lru_cache
from early_pw_scene_wire import pw_bucket_body
from generate_earliest_visuals import generate as visual_root
from legacy_width_wire import width_root
from legacy_component_reference import component_reference
from legacy_thread_wire import relocated_tags,tag_reader
from generate_shaded_vectors import ObjectBuilder
from generate_version_range import apply_patches
from reference_corpus import frame_at
from mfcobjects import ObjectArchive
import dl_doc


@lru_cache(maxsize=None)
def pw_source(version: int, kind: str) -> bytes:
    if version==2771:
        raw,identities = width_root(version,kind,0)
        assert not identities
        return raw
    framed,_ = visual_root(version,kind,0)
    raw,_ = frame_at(framed,6)
    return raw


def pw_slot_reader(original: object, slots: list[tuple[int,int,int]]) -> object:
    def read(archive: object) -> object:
        slots.append((archive.p-2,archive.p,len(archive.map)))
        return original(archive)
    return read


def early_pw_root(version: int, kind: str, scenario: int) -> tuple[bytes,bytes]:
    source,slots,tags = pw_source(version,kind),[],[]
    original,items = ObjectArchive.object,dl_doc.read_display_item_list
    ObjectArchive.object = tag_reader(original,tags)
    dl_doc.read_display_item_list = pw_slot_reader(items,slots)
    try:component_reference(source,version,kind,())
    finally:ObjectArchive.object,dl_doc.read_display_item_list = original,items
    assert len(slots)==1,slots
    first,last,index = slots[0]
    assert source[first:last]==bytes(2)
    builder = ObjectBuilder(index)
    replacement = builder.object("uiViewBucket_c",lambda state:pw_bucket_body(state,version,scenario))
    patches = [(first,last,replacement)]+relocated_tags(last,index,builder.next_index-index,tags)
    return source,apply_patches(source,patches)
