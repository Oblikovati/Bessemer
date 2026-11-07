"""Complete roots with implicit character-width counts and independently counted successors."""
import struct
from legacy_addin_wire import addin_root
from legacy_pane_wire import display_items
from generate_legacy_panes import physical_reference
from generate_version_range import apply_patches
from pre_facet_wire import capture
from generate_transform_vectors import doubles
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_item_vectors import record
import dl_doc


def simple_text(version: int, text: str, flag: int, widths: list[int], wide: bool) -> bytes:
    raw = doubles(3)+doubles(1)+integers(-1)+cstring(text,wide)+struct.pack("<h",flag)
    if flag==0:return record(18,raw)
    assert version>0xaeb or len(widths)==len(text),(version,text,widths)
    if version>0xaeb:raw += integers(len(widths))
    return record(18,raw+integers(*widths))


def width_items(version: int, edited: bool) -> bytes:
    text = ("B" if edited else "A")+"😀e\u0301鋼"
    widths = [-1+int(edited),-2147483648,2147483647,0,1073741824]
    if version>0xaeb:widths = widths[:3]
    samples = [simple_text(version,text,-2,widths,True),simple_text(version,"",-1,[],True),
        simple_text(version,"widths off 鋼",0,[],True),simple_text(version,"ascii",1,[1,2,3,4,5],False),
        simple_text(version,"😀",-32768,[] if version>0xaeb else [-1],True)]
    return struct.pack("<H",9)+display_items(version,edited)[2:]+b"".join(samples)


def width_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = addin_root(version,kind,scenario)
    original,spans = dl_doc.read_display_item_list,{}
    dl_doc.read_display_item_list = capture(original,"items",spans)
    try:physical_reference(raw,2796,kind,identities)
    finally:dl_doc.read_display_item_list = original
    first,last = spans["items"]
    replacement = width_items(version,scenario in (1,4))
    return apply_patches(raw,[(first,last,replacement)]),identities
