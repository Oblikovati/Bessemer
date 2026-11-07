"""Replay every frozen 11000-range root and bucket in two independent Rust processes."""
import sys
from verify_earlier_processes import verify_groups
from generate_precollector_range import generate as full_root
from generate_earlier_buckets import generate as bucket_root


def main(executable: str) -> None:
    groups = [("precollector-range-part",full_root),("precollector-range-assembly",full_root),("precollector-configurations",full_root),("precollector-buckets-3d",bucket_root),("precollector-buckets-2d",bucket_root)]
    modules = ["modern_root","modern_part_tess","modern_body_tess","modern_component","configurations","configuration_views"]
    verify_groups(executable,groups,"precollector-verification.json",modules)


if __name__=="__main__":
    main(sys.argv[1])
