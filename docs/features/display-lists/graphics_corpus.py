"""Graphics-reference scalar traces from first declarations in the independent graph."""
from generate_blank_scalars import flatten

CLASSES = {"moGraphicsConnectRefMgrData_c","moGraphicsMagneticConnectRefData_c","moGraphicsRefWrapData_c"}


def walk(path: str, value: object, rows: dict[str,str], seen: set[int], active: bool) -> None:
    if isinstance(value,dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        active = active or value.get("_class") in CLASSES
        for key,child in value.items():
            if key.startswith("_"):
                continue
            suffix = ".components" if key=="tessellation" and isinstance(child,list) else ""
            walk(f"{path}.{key}{suffix}",child,rows,seen,active)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):
            walk(f"{path}[{index}]",child,rows,seen,active)
    elif value is not None and active:
        flatten(path,value,rows)


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    walk("root",root,rows,set(),False)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
