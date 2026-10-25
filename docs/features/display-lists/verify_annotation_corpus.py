"""Annotation-view field and byte oracles across complete native display-list parsing."""
from pathlib import Path
import json
import sys
from view_corpus import verify


def count_views(value: object) -> int:
    if value is None:
        return 0
    if isinstance(value,dict):
        views = value.get("views")
        if isinstance(views,list):
            return len(views)
    raise ValueError(f"annotation result {value!r}; expected None or dictionary with views list")


if __name__ == "__main__":
    evidence = verify(sys.argv[1],"read_AnnotationViewTransform",Path("tests/fixtures/display-lists/annotation-views.bin"),count_views)
    evidence["annotation_view_bytes"] = evidence.pop("view_bytes")
    evidence["annotation_views"] = evidence.pop("views")
    print(json.dumps(evidence))
