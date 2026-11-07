"""Independent configuration header, matrix, visibility and tail semantic fields."""
from generate_blank_scalars import flatten
from generate_item_vectors import trace_items


def field(rows: dict[str,str], path: str, value: object) -> None:
    flatten("root.configurations."+path,value,rows)


def matrices(config: dict[str,object], rows: dict[str,str]) -> None:
    seen: set[int] = set()
    for index,(identity,objects) in enumerate(config.get("view_objects",[])):
        field(rows,f"view_objects.entries[{index}].config_id",identity)
        for view,obj in enumerate(objects):
            if obj is None or id(obj) in seen:
                continue
            seen.add(id(obj))
            field(rows,f"view_objects.entries[{index}].views[{view}].name",obj["name"])
    for index,(identity,lists) in enumerate(config.get("view_display_lists",[])):
        field(rows,f"view_display_lists.entries[{index}].config_id",identity)
        for view,items in enumerate(lists):
            trace_items(items,f"root.configurations.view_display_lists.entries[{index}].views[{view}]",rows)


def trace(root: dict[str,object], kind: str) -> bytes:
    config = root["configurations"]
    rows: dict[str,str] = {}
    for key in ["config_count","active_config_name","active_config_id","config_names"]:
        if key in config:
            field(rows,key,config[key])
    for index,entry in enumerate(config.get("configs",[])):
        field(rows,f"configs[{index}].config_id",entry[0] if kind=="part" else entry["config_id"])
    matrices(config,rows)
    for index,(view,name) in enumerate(config.get("annotation_view_visibility",[])):
        field(rows,f"annotation_view_visibility.entries[{index}].view_name",view)
        field(rows,f"annotation_view_visibility.entries[{index}].config_name",name)
    for index,(key,values) in enumerate(config.get("unknown_tail",[])):
        field(rows,f"unknown_tail.entries[{index}].key",key)
        field(rows,f"unknown_tail.entries[{index}].values",values)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()
