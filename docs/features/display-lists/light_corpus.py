"""Independent DisplayLists old-light scalar traces with first-declaration identity."""
from generate_blank_scalars import flatten


def walk(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        for key,child in value.items():
            if key.startswith("_"):
                continue
            suffix = ".items" if key == "old_lights" else ".entries" if key == "user_property" else ""
            walk(f"{path}.{key}{suffix}",child,rows,seen)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):
            walk(f"{path}[{index}]",child,rows,seen)
    elif value is not None and path.startswith("root.old_lights.items["):
        flatten(path,value,rows)


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    walk("root",root,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
