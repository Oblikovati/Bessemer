"""Scene semantic traces from the independent object graph, honoring prior declarations."""
from generate_blank_scalars import flatten


def walk(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        for key,child in value.items():
            if key == "wall_info":
                wall_info(f"{path}.{key}",child,rows,seen)
            elif not key.startswith("_"):
                suffix = ".entries" if key == "user_property" else ""
                walk(f"{path}.{key}{suffix}",child,rows,seen)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):
            walk(f"{path}[{index}]",child,rows,seen)
    elif value is not None and path.startswith("root.scene."):
        flatten(path,value,rows)


def wall_info(path: str, entries: list[tuple[int,dict[str,object]]], rows: dict[str,str], seen: set[int]) -> None:
    for index,(key,value) in enumerate(entries):
        if path.startswith("root.scene."):
            flatten(f"{path}[{index}].key",key,rows)
        walk(f"{path}[{index}].value",value,rows,seen)


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    walk("root",root,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
