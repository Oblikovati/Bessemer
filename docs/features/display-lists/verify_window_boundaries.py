"""Replay complete reference-readable roots around the uncounted-window interval."""
import sys
from verify_earlier_processes import verify_groups
from generate_window_boundaries import generate as full_root
from generate_window_buckets import generate as bucket_root


def main(executable: str) -> None:
    groups = [("window-boundary-part",full_root),("window-boundary-assembly",full_root),("early-minimal-annotations",full_root),("window-buckets-3d",bucket_root),("window-buckets-2d",bucket_root)]
    modules = ["modern_root","modern_item_annotation","modern_ui","modern_body_tess","annotation_read"]
    verify_groups(executable,groups,"window-boundary-verification.json",modules)


if __name__=="__main__":
    main(sys.argv[1])
