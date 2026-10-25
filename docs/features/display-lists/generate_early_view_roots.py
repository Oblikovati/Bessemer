"""Complete 7000-range roots with populated view collections and historical annotation IDs."""
import json
import struct
from generate_earlier_range import converted as convert_root,generate as generate_root,write_group
from generate_window_boundaries import entry_budget as previous_entries
from early_annotation_wire import minimal_items
from early_view_patches import convert,named_entries,annotation_entries,item_entries
from transition_item_wire import items
from early_shaded_wire import simple_text_payload
from generate_item_vectors import record
from minimal_root_wire import unshaded_prefix

VERSIONS = [7000,7001,0x1be7,0x1be8,0x1bf4,0x1bf5,0x1c04,0x1c05,0x1c15,0x1c16,0x1c18,0x1c19,0x1c1e,0x1c1f,0x1c23,0x1c24,0x1c39,0x1c3a,0x1c3b,0x1c3d,0x1c3e,0x1c40,0x1c41,0x1c42,0x1c43,0x1c49,0x1c4a,0x1c4b,0x1c4c,0x1c56,0x1c57,7999,8000,0x1fcf]
MINIMAL_VERSIONS = [0x1c4a,0x1c4b,0x1c4c,7999]


def display_items(version: int, edited_float: bool = False) -> bytes:
    original = items(version)
    return struct.pack("<H",3)+original[2:]+record(18,simple_text_payload(version,0x41b80000 if edited_float else 0x7f800042))


def converted(document: object, path: object, raw: bytes, version: int, kind: str, variant: int) -> bytes:
    payload = minimal_items(version) if variant==8 else display_items(version,variant==2)
    raw = convert_root(document,path,raw,version,kind,variant,convert,b"",payload)
    return unshaded_prefix(document,path,raw,version,kind) if variant==8 else raw


def entry_budget(version: int, kind: str, variant: int) -> int:
    if variant==8:return 44+named_entries(version)+(6 if version>0x1c18 else 0)+annotation_entries(version)+101
    result = previous_entries(version,kind,0)+named_entries(version)
    result += annotation_entries(version)-1
    result -= 6*(version<=0x1c18)+2*(version<=0x1c23)
    return result+item_entries(version)-10


def generate(version: int, kind: str, scenario: int = 0) -> tuple[bytes,dict[str,object]]:
    variant = scenario%32
    framed,row = generate_root(version,kind,variant,converted,entry_budget)
    row["scenario"] = 160+variant
    return framed[:5]+bytes([160+variant])+framed[6:],row


def main() -> None:
    totals = []
    for kind in ["part","assembly"]:
        cases = [generate(version,kind) for version in VERSIONS]+[generate(7000,kind,1),generate(8000,kind,2)]
        totals.append(write_group(f"early-view-roots-{kind}",cases))
    cases = [generate(version,"part",8) for version in MINIMAL_VERSIONS]
    totals.append(write_group("earliest-minimal-annotations",cases))
    print(json.dumps({"fixtures":sum(count for count,_ in totals),"fields":sum(fields for _,fields in totals)}))


if __name__=="__main__":
    main()
