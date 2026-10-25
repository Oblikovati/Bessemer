"""Independent material/DSPBR fields outside already traced scenes and view buckets."""
from generate_blank_scalars import flatten

MAPS = {"model_viz_props","face_viz_props","body_prop_infos"}


def visible(path: str) -> bool:
    return (".material." in path or ".dspbr." in path) and not path.startswith("root.scene.") and ".view_bucket." not in path


def walk(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        for key,child in value.items():
            if not key.startswith("_"):
                member(path,key,child,rows,seen)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):
            walk(f"{path}[{index}]",child,rows,seen)
    elif value is not None and visible(path):
        flatten(path,value,rows)


def member(path: str, key: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
    target = f"{path}.{key}"
    if key in MAPS:
        for index,(number,child) in enumerate(value):
            walk(f"{target}[{index}].value",child,rows,seen)
    elif key != "facet_container":
        suffix = ".components" if key == "tessellation" and isinstance(value,list) else ".entries" if key == "user_property" else ""
        walk(target+suffix,value,rows,seen)


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    walk("root",root,rows,set())
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
