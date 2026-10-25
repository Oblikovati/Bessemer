"""Independent root-view bucket traces, preserving first object declarations and list roles."""
import struct
from generate_blank_scalars import flatten
from generate_item_vectors import trace_items

POLY_LISTS = {"silh","silh_h","wire","hidden","hatch","aux","tan_v","tan_h","explode_v","explode_h","weldbead_v","weldbead_h","solid_fill","halo_repair"}
ITEM_LISTS = {"dl_bucket","dl_ghost_bucket","dl_model_bucket"}


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    walk("root",root,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def walk(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        for key,child in value.items():
            if key.startswith("_") or key == "count":
                continue
            member(path,key,child,rows,seen)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):
            walk(f"{path}[{index}]",child,rows,seen)
    elif value is not None and ".view_bucket." in path:
        if isinstance(value,bytes):
            value = list(struct.unpack("<"+"d"*(len(value)//8),value))
        flatten(path,value,rows)


def member(path: str, key: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    target = f"{path}.{key}"
    if key in ITEM_LISTS and ".view_bucket." in target:
        trace_items(value,target,rows)
    elif key == "component_str_map":
        string_map(target,value,rows)
    elif key == "cthread_maps":
        thread_maps(target,value,rows,seen)
    else:
        suffix = ".items" if key in POLY_LISTS else ".entries" if key == "user_property" else ""
        walk(target+suffix,value,rows,seen)


def string_map(path: str, values: list[tuple[int,str]], rows: dict[str,str]) -> None:
    if ".view_bucket." not in path:
        return
    for index,(key,value) in enumerate(values):
        flatten(f"{path}[{index}].key",key,rows)
        flatten(f"{path}[{index}].value",value,rows)


def thread_maps(path: str, values: list[list[tuple[object,object]]], rows: dict[str,str], seen: set[int]) -> None:
    for index,entries in enumerate(values):
        for offset,(key,value) in enumerate(entries):
            prefix = f"{path}[{index}].entries[{offset}]"
            walk(f"{prefix}.key",key,rows,seen)
            walk(f"{prefix}.value.items",value,rows,seen)
