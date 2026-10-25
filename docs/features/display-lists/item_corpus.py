"""Display-item semantic traces from the independent complete root reader."""
from generate_item_vectors import trace_items
import reference_corpus


def trace(root: dict[str,object]) -> bytes:
    rows: dict[str,str] = {}
    for index,view in enumerate(root["views"]):
        trace_items(view.get("display_items",[]),f"root.views[{index}].display_items",rows)
    return "".join(f"{key}={value}\n" for key,value in sorted(rows.items())).encode()


def field_coverage(actual: bytes, expected: bytes, label: str) -> dict[str,int]:
    total = reference_corpus.compare_trace(actual,expected,label)
    scenes = sum(line.startswith(b"root.scene.") for line in actual.splitlines())
    items = sum(b".display_items.items[" in line for line in actual.splitlines())
    buckets = sum(b".view_bucket." in line for line in actual.splitlines())
    return {"reference":total-scenes-items-buckets,"scene":scenes,"items":items,"buckets":buckets}
