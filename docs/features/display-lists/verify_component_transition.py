"""Replay frozen 10000-range roots, buckets and self-referencing viewer records."""
import sys
from verify_earlier_processes import verify_groups
from generate_component_transition import generate as full_root
from generate_earlier_buckets import generate as bucket_root


def main(executable: str) -> None:
    groups = [("component-transition-part",full_root),("component-transition-assembly",full_root),("transient-viewers",full_root),("component-buckets-3d",bucket_root),("component-buckets-2d",bucket_root)]
    modules = ["modern_root","modern_component","archive_objects","modern_body_tess","legacy_bucket_poly","modern_bucket"]
    verify_groups(executable,groups,"component-transition-verification.json",modules)


if __name__=="__main__":
    main(sys.argv[1])
