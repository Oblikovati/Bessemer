"""Replay early roots and buckets in independent reconstruction processes."""
import sys
from verify_earlier_processes import verify_groups
from generate_early_view_roots import generate as full_root
from generate_early_view_buckets import generate as bucket_root


def main(executable: str) -> None:
    groups = [(name, full_root) for name in ("early-view-roots-part", "early-view-roots-assembly", "earliest-minimal-annotations")]
    groups += [(name, bucket_root) for name in ("early-view-buckets-3d", "early-view-buckets-2d")]
    verify_groups(executable, groups, "early-view-verification.json", ["modern_root", "modern_item_annotation", "modern_item_text", "modern_bucket"])


if __name__ == "__main__":
    main(sys.argv[1])
