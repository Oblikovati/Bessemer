"""Owned shared holder graphs across the first admitted holder-state boundary."""
import json
from functools import partial
import generate_component_properties as properties
from generate_earlier_range import write_group
from generate_early_appearance_roots import item_bytes as appearance_items
from generate_earliest_visuals import entry_count,table_layout
from earliest_visual_patches import convert
from implicit_visual_maps import single_properties

ORIGINAL_CONVERTED = properties.converted


def items(version: int) -> bytes:
    raw = appearance_items(version)
    if version<=0x1115:raw = raw[:18]+raw[22:]
    return table_layout(raw,version)


def converted(document: object, path: object, raw: bytes, version: int, kind: str, scenario: int, class_name: str, variant: int) -> bytes:
    raw = ORIGINAL_CONVERTED(document,path,raw,version,kind,scenario,class_name,variant)
    return single_properties(document,path,raw,version,False)


def base_entries(version: int, kind: str) -> int:
    return entry_count(version,kind,0)-3


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    saved = properties.convert,properties.converted,properties.item_bytes,properties.base_entries
    properties.convert = partial(saved[0],base_converter=convert)
    properties.converted,properties.item_bytes,properties.base_entries = converted,items,base_entries
    try:return properties.generate(version,kind,scenario)
    finally:properties.convert,properties.converted,properties.item_bytes,properties.base_entries = saved


if __name__ == "__main__":
    cases = [generate(v,k,s) for v in [4100,0x1051,0x1052,4200,4201,4202,0x10ae,0x10af] for k in ["part","assembly"] for s in [0,16]]
    print(write_group("earliest-visual-properties",cases,("displaylists.py",)))
