"""Named-view field and byte oracles across the complete native display-list corpus."""
from pathlib import Path
import json
import sys
from view_corpus import verify


def count_views(value: object) -> int:
    if value is None:
        return 0
    if isinstance(value,list):
        return len(value)
    raise ValueError(f"named-view result {value!r}; expected None or list")


if __name__ == "__main__":
    evidence = verify(sys.argv[1],"read_NamedViewTransform",Path("tests/fixtures/display-lists/named-views.bin"),count_views)
    evidence["named_view_bytes"] = evidence.pop("view_bytes")
    evidence["named_views"] = evidence.pop("views")
    print(json.dumps(evidence))
