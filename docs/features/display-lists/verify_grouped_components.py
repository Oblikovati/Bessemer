"""Replay the 9000 interval and populated decal boundary roots through isolated processes."""
import sys
from verify_earlier_processes import verify_groups
from generate_grouped_components import generate as full_root
from generate_earlier_buckets import generate as bucket_root
from generate_early_decal_transition import generate as decal_root


def main(executable: str) -> None:
    groups = [("grouped-component-part",full_root),("grouped-component-assembly",full_root),("early-assembly-configurations",full_root),("grouped-buckets-3d",bucket_root),("grouped-buckets-2d",bucket_root),("early-decal-transition",decal_root)]
    modules = ["modern_root","modern_bucket","modern_item_annotation","modern_concept","modern_decals","modern_component","configurations","configuration_views"]
    verify_groups(executable,groups,"grouped-component-verification.json",modules)


if __name__=="__main__":
    main(sys.argv[1])
