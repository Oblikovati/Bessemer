"""3D-view field, bitmap and byte oracles across complete native display-list parsing."""
from pathlib import Path
import json
import sys
from view_corpus import verify,list_view_count

if __name__ == "__main__":
    evidence = verify(sys.argv[1],"read_3DViewTransform",Path("tests/fixtures/display-lists/spatial-views.bin"),list_view_count)
    evidence["spatial_view_bytes"] = evidence.pop("view_bytes")
    evidence["spatial_views"] = evidence.pop("views")
    print(json.dumps(evidence))
