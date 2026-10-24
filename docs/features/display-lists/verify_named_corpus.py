"""Named-view field and byte oracles across the complete native display-list corpus."""
from pathlib import Path
import json
import sys
from view_corpus import verify,list_view_count


if __name__ == "__main__":
    evidence = verify(sys.argv[1],"read_NamedViewTransform",Path("tests/fixtures/display-lists/named-views.bin"),list_view_count)
    evidence["named_view_bytes"] = evidence.pop("view_bytes")
    evidence["named_views"] = evidence.pop("views")
    print(json.dumps(evidence))
