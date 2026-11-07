"""First-declaration decal scalar fields from the independent complete graph reader."""
from generate_blank_scalars import flatten


def walk(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        for key,child in value.items():
            if key.startswith("_"):
                continue
            member(path,key,child,rows,seen)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):
            walk(f"{path}[{index}]",child,rows,seen)


def member(path: str, key: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if key=="decals":
        for index,decal in enumerate(value):
            flatten(f"{path}.decals.items[{index}]",decal,rows)
        return
    if key=="decals_map":
        for index,(number,decals) in enumerate(value):
            flatten(f"{path}.decals_map[{index}].key",number,rows)
            for item,decal in enumerate(decals):
                flatten(f"{path}.decals_map[{index}].value.items[{item}]",decal,rows)
        return
    suffix = ".components" if key=="tessellation" and isinstance(value,list) else ""
    walk(f"{path}.{key}{suffix}",value,rows,seen)


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    walk("root",root,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
